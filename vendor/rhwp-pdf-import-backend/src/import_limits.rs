//! Request-scoped limits for the rhwp import path. Not a process-wide heap limit.

use crate::error::{Error, Result};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

/// Stream bytes retain the originating request's budget through object clones.
#[derive(Clone, Debug, Default)]
pub struct StreamData {
    bytes: bytes::Bytes,
    budget: Option<Arc<ImportBudget>>,
}

impl StreamData {
    /// Create empty, unbound stream bytes for legacy writers.
    pub fn new() -> Self {
        Self::default()
    }
    /// Wrap static bytes without a request context.
    pub fn from_static(bytes: &'static [u8]) -> Self {
        bytes::Bytes::from_static(bytes).into()
    }
    /// Copy bytes for legacy object construction.
    pub fn copy_from_slice(bytes: &[u8]) -> Self {
        bytes::Bytes::copy_from_slice(bytes).into()
    }
    pub(crate) fn bounded(bytes: Vec<u8>, budget: Option<&Arc<ImportBudget>>) -> Self {
        Self {
            bytes: bytes.into(),
            budget: budget.cloned(),
        }
    }
    /// Original request context, shared by cloned stream objects.
    pub fn budget(&self) -> Option<&Arc<ImportBudget>> {
        self.budget.as_ref()
    }
}

impl std::ops::Deref for StreamData {
    type Target = bytes::Bytes;
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
impl AsRef<[u8]> for StreamData {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}
impl PartialEq for StreamData {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}
impl From<bytes::Bytes> for StreamData {
    fn from(bytes: bytes::Bytes) -> Self {
        Self {
            bytes,
            budget: None,
        }
    }
}
impl From<Vec<u8>> for StreamData {
    fn from(bytes: Vec<u8>) -> Self {
        bytes::Bytes::from(bytes).into()
    }
}
impl From<String> for StreamData {
    fn from(bytes: String) -> Self {
        bytes::Bytes::from(bytes).into()
    }
}

/// Explicit limits for one PDF import; zero never means unlimited.
#[derive(Clone, Copy, Debug)]
pub struct PdfImportLimits {
    /// Input byte limit checked before the constructor's copy.
    pub input_bytes: usize,
    /// Maximum size of any individual decoded output buffer.
    pub stream_bytes: usize,
    /// Cumulative decoded/copy/predictor bytes, including failed attempts.
    pub decoded_bytes: usize,
    /// Maximum number of filters in one pipeline.
    pub filters: usize,
    /// Maximum lexical nesting and xref-chain depth.
    pub nesting: usize,
    /// Maximum count of parsed object values.
    pub objects: usize,
    /// Maximum cumulative collection and xref entry count.
    pub entries: usize,
    /// Maximum operators across all parses in this request.
    pub operators: usize,
    /// Maximum graphics state stack depth in a stream.
    pub graphics_stack: usize,
    /// Maximum literal or hex string size before materializing its value.
    pub string_bytes: usize,
    /// Maximum text-showing bytes across the document.
    pub text_bytes: usize,
    /// Maximum emitted spans before postprocessing.
    pub spans: usize,
    /// Maximum decoded source bytes for one ToUnicode CMap.
    pub cmap_bytes: usize,
    /// Maximum expanded CMap entries, including overwritten/temporary entries.
    pub cmap_entries: usize,
    /// Maximum conservative byte-scan work across CMap parses.
    pub cmap_scan_bytes: usize,
    /// Maximum CID width/vertical metric expansion work across fonts.
    pub font_entries: usize,
}

impl Default for PdfImportLimits {
    fn default() -> Self {
        Self {
            input_bytes: 16 * 1024 * 1024,
            stream_bytes: 32 * 1024 * 1024,
            decoded_bytes: 256 * 1024 * 1024,
            filters: 8,
            nesting: 16,
            objects: 50_000,
            entries: 100_000,
            operators: 250_000,
            graphics_stack: 128,
            string_bytes: 64 * 1024,
            text_bytes: 2 * 1024 * 1024,
            spans: 40_000,
            cmap_bytes: 1024 * 1024,
            cmap_entries: 100_000,
            cmap_scan_bytes: 64 * 1024 * 1024,
            font_entries: 100_000,
        }
    }
}

/// Shared accounting with a sticky fatal state; not a global allocation tracker.
#[derive(Debug)]
pub struct ImportBudget {
    /// Immutable request policy.
    pub limits: PdfImportLimits,
    decoded: AtomicUsize,
    objects: AtomicUsize,
    entries: AtomicUsize,
    operators: AtomicUsize,
    text_bytes: AtomicUsize,
    spans: AtomicUsize,
    cmap_entries: AtomicUsize,
    cmap_scanned: AtomicUsize,
    font_entries: AtomicUsize,
    failed: AtomicBool,
    failure_reason: std::sync::Mutex<Option<&'static str>>,
}

impl ImportBudget {
    /// Create independent accounting; reject disabled limits.
    pub fn new(limits: PdfImportLimits) -> Result<Self> {
        if [
            limits.input_bytes,
            limits.stream_bytes,
            limits.decoded_bytes,
            limits.filters,
            limits.nesting,
            limits.objects,
            limits.entries,
            limits.operators,
            limits.graphics_stack,
            limits.string_bytes,
            limits.text_bytes,
            limits.spans,
            limits.cmap_bytes,
            limits.cmap_entries,
            limits.cmap_scan_bytes,
            limits.font_entries,
        ]
        .contains(&0)
        {
            return Err(Error::ImportResourceLimit("zero import budget"));
        }
        Ok(Self {
            limits,
            decoded: AtomicUsize::new(0),
            objects: AtomicUsize::new(0),
            entries: AtomicUsize::new(0),
            operators: AtomicUsize::new(0),
            text_bytes: AtomicUsize::new(0),
            spans: AtomicUsize::new(0),
            cmap_entries: AtomicUsize::new(0),
            cmap_scanned: AtomicUsize::new(0),
            font_entries: AtomicUsize::new(0),
            failed: AtomicBool::new(false),
            failure_reason: std::sync::Mutex::new(None),
        })
    }

    /// Fail once any previous operation exhausted this request.
    pub fn check(&self) -> Result<()> {
        if self.failed.load(Ordering::Acquire) {
            let reason = self
                .failure_reason
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            Err(Error::ImportResourceLimit(reason.unwrap_or("request budget exhausted")))
        } else {
            Ok(())
        }
    }

    /// Mark a resource failure as fatal for the whole import.
    pub fn reject(&self, reason: &'static str) -> Error {
        let mut first = self
            .failure_reason
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let reason = *first.get_or_insert(reason);
        self.failed.store(true, Ordering::Release);
        Error::ImportResourceLimit(reason)
    }

    fn charge(
        &self,
        counter: &AtomicUsize,
        amount: usize,
        cap: usize,
        reason: &'static str,
    ) -> Result<()> {
        self.check()?;
        counter
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |used| {
                used.checked_add(amount).filter(|&next| next <= cap)
            })
            .map(|_| ())
            .map_err(|_| self.reject(reason))
    }

    /// Successfully charged decoded/copy bytes so far, including failed attempts.
    pub fn decoded_bytes(&self) -> usize {
        self.decoded.load(Ordering::Relaxed)
    }

    /// Charge before allocating or extending an output buffer.
    pub fn charge_decoded(&self, amount: usize) -> Result<()> {
        self.charge(&self.decoded, amount, self.limits.decoded_bytes, "document decoded bytes")
    }

    pub(crate) fn charge_object(&self) -> Result<()> {
        self.charge(&self.objects, 1, self.limits.objects, "parsed object count")
    }

    pub(crate) fn charge_entries(&self, count: usize) -> Result<()> {
        self.charge(&self.entries, count, self.limits.entries, "collection entries")
    }

    pub(crate) fn charge_operator(&self) -> Result<()> {
        self.charge(&self.operators, 1, self.limits.operators, "content operators")
    }
    pub(crate) fn charge_text(&self, amount: usize) -> Result<()> {
        self.charge(&self.text_bytes, amount, self.limits.text_bytes, "shown text bytes")
    }
    pub(crate) fn charge_span(&self) -> Result<()> {
        self.charge(&self.spans, 1, self.limits.spans, "emitted spans")
    }

    pub(crate) fn cmap_source(&self, bytes: usize) -> Result<()> {
        self.check()?;
        if bytes > self.limits.cmap_bytes {
            return Err(self.reject("CMap source bytes"));
        }
        self.charge_cmap_scan(
            bytes
                .checked_mul(4)
                .ok_or_else(|| self.reject("CMap scan bytes"))?,
        )?;
        // Account for bounded source normalization and section/line scratch.
        self.charge_decoded(
            bytes
                .checked_mul(24)
                .ok_or_else(|| self.reject("CMap scratch"))?,
        )
    }

    pub(crate) fn cmap_line(&self, bytes: usize) -> Result<()> {
        self.check()?;
        if bytes > self.limits.string_bytes {
            return Err(self.reject("CMap line bytes"));
        }
        Ok(())
    }

    pub(crate) fn charge_cmap_scan(&self, bytes: usize) -> Result<()> {
        self.charge(&self.cmap_scanned, bytes, self.limits.cmap_scan_bytes, "CMap scan bytes")
    }

    pub(crate) fn charge_cmap(&self, entries: usize, text_bytes: usize) -> Result<()> {
        self.charge(&self.cmap_entries, entries, self.limits.cmap_entries, "CMap entries")?;
        let work = entries
            .checked_mul(128)
            .and_then(|n| n.checked_add(text_bytes.checked_mul(4)?))
            .ok_or_else(|| self.reject("CMap expansion bytes"))?;
        self.charge_decoded(work)
    }

    pub(crate) fn charge_font_entries(&self, entries: usize) -> Result<()> {
        self.charge(&self.font_entries, entries, self.limits.font_entries, "font metric entries")?;
        self.charge_decoded(
            entries
                .checked_mul(128)
                .ok_or_else(|| self.reject("font expansion bytes"))?,
        )
    }

    pub(crate) fn depth(&self, depth: usize) -> Result<()> {
        self.check()?;
        if depth > self.limits.nesting {
            return Err(self.reject("object nesting"));
        }
        Ok(())
    }

    pub(crate) fn output(&self) -> BoundedOutput<'_> {
        BoundedOutput {
            budget: self,
            bytes: Vec::new(),
        }
    }

    pub(crate) fn copy_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        let mut output = self.output();
        output.append(bytes)?;
        Ok(output.finish())
    }

    pub(crate) fn append_bytes(&self, bytes: &mut Vec<u8>, more: &[u8]) -> Result<()> {
        let mut output = BoundedOutput {
            budget: self,
            bytes: std::mem::take(bytes),
        };
        output.append(more)?;
        *bytes = output.finish();
        Ok(())
    }

    pub(crate) fn read(&self, mut reader: impl Read) -> Result<Vec<u8>> {
        let mut output = self.output();
        let mut chunk = [0u8; 4096];
        loop {
            self.check()?;
            let count = reader.read(&mut chunk).map_err(Error::Io)?;
            if count == 0 {
                return Ok(output.finish());
            }
            output.append(&chunk[..count])?;
        }
    }
}

pub(crate) struct BoundedOutput<'a> {
    budget: &'a ImportBudget,
    bytes: Vec<u8>,
}

impl BoundedOutput<'_> {
    pub(crate) fn append(&mut self, bytes: &[u8]) -> Result<()> {
        let next = self
            .bytes
            .len()
            .checked_add(bytes.len())
            .filter(|&next| next <= self.budget.limits.stream_bytes)
            .ok_or_else(|| self.budget.reject("stream output bytes"))?;
        self.budget.charge_decoded(bytes.len())?;
        if next > self.bytes.capacity() {
            let capacity = self
                .bytes
                .capacity()
                .saturating_mul(2)
                .max(4096)
                .max(next)
                .min(self.budget.limits.stream_bytes);
            self.bytes
                .try_reserve_exact(capacity - self.bytes.len())
                .map_err(|_| self.budget.reject("output allocation"))?;
        }
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    pub(crate) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BoundedOutput<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.append(bytes)
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
