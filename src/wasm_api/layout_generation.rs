//! Expose the same session layout generation that the Native CLI selects.

use wasm_bindgen::prelude::*;

use super::HwpDocument;

#[wasm_bindgen]
impl HwpDocument {
    /// Select the Hancom 2024 layout emulation; false selects the default
    /// generation. DocumentCore owns invalidation and repagination.
    #[wasm_bindgen(js_name = setHangul2024Compat)]
    pub fn set_hangul2024_compat(&mut self, enabled: bool) {
        self.core.set_hangul2024_compat(enabled);
    }
}
