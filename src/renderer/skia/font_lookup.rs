use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use skia_safe::{FontMgr, FontStyle, Typeface};

use crate::renderer::base_family_without_weight_suffix;

pub(super) type SystemFontFamilies = HashSet<String>;

const TEXT_FALLBACK_FAMILIES: &[&str] = &[
    "Noto Sans KR",
    "Noto Serif KR",
    "Noto Sans CJK KR",
    "Noto Serif CJK KR",
    "Nanum Gothic",
    "Nanum Myeongjo",
    "Malgun Gothic",
    "맑은 고딕",
    "Batang",
    "바탕",
    "Apple SD Gothic Neo",
    "AppleMyungjo",
    "DejaVu Sans",
    "Arial",
    "sans-serif",
];

#[derive(Clone)]
pub(super) struct TypefaceCandidate {
    pub(super) typeface: Typeface,
    pub(super) source: &'static str,
}

pub(super) fn text_family_candidates(requested: &str, bold: bool) -> Vec<String> {
    let mut families = Vec::new();
    // 원 face와 설치 별칭을 먼저 찾고 문서의 선언 대체를 같은 순서로 찾는다.
    // 체인 전체를 한 개의 시스템 글꼴 이름으로 조회하지 않는다.
    for face in crate::renderer::internal_font_family_members(requested) {
        if !families.iter().any(|family| family == face) {
            families.push(face.to_string());
        }
        for alias in crate::renderer::installed_render_font_aliases(face) {
            if !families.iter().any(|family| family == alias) {
                families.push((*alias).to_string());
            }
        }
        if let Some(base) = base_family_without_weight_suffix(face) {
            if !families.iter().any(|family| family == &base) {
                families.push(base);
            }
        }
    }
    // SVG·Canvas와 같은 계열/굵기별 순서로 없는 글리프를 찾는다.
    // 선언 face와 문서의 명시 대체는 위에서 이미 우선 수집했다.
    let primary = crate::renderer::internal_font_family_members(requested)
        .first()
        .copied()
        .unwrap_or("");
    let fallback = crate::renderer::generic_fallback_for_weight(primary, bold);
    for family in fallback.split(',').map(|family| family.trim_matches('\'')) {
        if !families.iter().any(|candidate| candidate == family) {
            families.push(family.to_string());
        }
    }
    // 공통 후보가 없는 환경에서는 기존 시스템 후보도 계속 사용할 수 있다.
    for family in TEXT_FALLBACK_FAMILIES {
        if !families.iter().any(|candidate| candidate == family) {
            families.push((*family).to_string());
        }
    }
    families
}

pub(super) fn text_typeface_candidates(
    font_mgr: &FontMgr,
    system_families: &SystemFontFamilies,
    custom_typefaces: &HashMap<String, Typeface>,
    bundled_typefaces: &HashMap<String, Typeface>,
    requested: &str,
    style: FontStyle,
) -> (Vec<String>, Vec<TypefaceCandidate>) {
    let families =
        text_family_candidates(requested, *style.weight() >= *FontStyle::bold().weight());
    let chain = typeface_candidates_for_families(
        font_mgr,
        system_families,
        custom_typefaces,
        bundled_typefaces,
        &families,
        style,
    );
    (families, chain)
}

/// 본문과 수식이 같은 글꼴 계열 순서와 공급 경로로 후보를 조회한다.
pub(super) fn typeface_candidates_for_families(
    font_mgr: &FontMgr,
    system_families: &SystemFontFamilies,
    custom_typefaces: &HashMap<String, Typeface>,
    bundled_typefaces: &HashMap<String, Typeface>,
    families: &[String],
    style: FontStyle,
) -> Vec<TypefaceCandidate> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut push = |typeface: Typeface, source: &'static str| {
        if is_placeholder_typeface(&typeface) {
            return;
        }
        let key = typeface.family_name();
        if seen.insert(key) {
            chain.push(TypefaceCandidate { typeface, source });
        }
    };
    for family in families {
        if let Some(typeface) = custom_typefaces.get(family).cloned() {
            push(typeface, "custom");
        }
        // 원 face의 system 프로그램보다 뒤 face의 custom 프로그램을 먼저
        // 고르면 선언 순서를 뒤집는다. 공급 출처는 각 face 내부에서 우선한다.
        if let Some(typeface) = match_system_family_style(font_mgr, system_families, family, style)
        {
            push(typeface, "system");
        }
        if let Some(typeface) = bundled_typefaces.get(family).cloned() {
            push(typeface, "bundled");
        }
    }
    if let Some(typeface) = legacy_typeface_for_style(font_mgr, style) {
        push(typeface, "legacy");
    }
    chain
}

pub(super) fn select_typeface_for_character<'a>(
    chain: &'a [TypefaceCandidate],
    character: char,
) -> Option<&'a TypefaceCandidate> {
    if character.is_whitespace() {
        return chain
            .iter()
            .find(|candidate| !is_placeholder_typeface(&candidate.typeface));
    }
    let codepoint = character as i32;
    chain.iter().find(|candidate| {
        !is_placeholder_typeface(&candidate.typeface)
            && candidate.typeface.unichar_to_glyph(codepoint) != 0
    })
}

/// CoreText는 요청한 family 이름을 유지한 채 LastResort를 반환할 수 있다.
/// 그 프로그램의 비영 gid는 문자 대신 Unicode 블록 대용 기호이므로,
/// family_name이나 unichar_to_glyph만으로 실제 문자 공급이라고 판단하지 않는다.
fn is_placeholder_typeface(typeface: &Typeface) -> bool {
    let id = typeface.unique_id();
    PLACEHOLDER_TYPEFACE_CACHE.with(|cache| {
        if let Some(value) = cache.borrow().get(&id).copied() {
            return value;
        }
        let reserved = |name: &str| matches!(name, ".LastResort" | "LastResort");
        let placeholder = reserved(&typeface.family_name())
            || typeface
                .copy_table_data(u32::from_be_bytes(*b"name"))
                .is_some_and(|data| {
                    ttf_parser::name::Table::parse(data.as_bytes()).is_some_and(|table| {
                        table.names.into_iter().any(|name| {
                            matches!(name.name_id, 1 | 4 | 6 | 16)
                                && name.to_string().is_some_and(|value| reserved(&value))
                        })
                    })
                });
        cache.borrow_mut().insert(id, placeholder);
        placeholder
    })
}

/// OS 조회가 실제 프로그램을 공급하지 못할 때 같은 설치 family를 파일로 찾는다.
/// 정상 OS 스타일 매칭은 유지하며, family당 regular 한 개로 덮어쓰지 않는다.
fn recover_installed_system_typeface(
    font_mgr: &FontMgr,
    family: &str,
    style: FontStyle,
) -> Option<Typeface> {
    use usvg::fontdb::{Family, Query, Stretch, Style, Weight};

    SYSTEM_PROGRAM_DATABASE.with(|database| {
        let mut database = database.borrow_mut();
        let database = database.get_or_insert_with(|| {
            let mut database = usvg::fontdb::Database::new();
            for directory in crate::renderer::font_paths::system_font_dirs() {
                if directory.exists() {
                    database.load_fonts_dir(directory);
                }
            }
            database
        });
        let families = [Family::Name(family)];
        let stretch = match *style.width() {
            1 => Stretch::UltraCondensed,
            2 => Stretch::ExtraCondensed,
            3 => Stretch::Condensed,
            4 => Stretch::SemiCondensed,
            6 => Stretch::SemiExpanded,
            7 => Stretch::Expanded,
            8 => Stretch::ExtraExpanded,
            9 => Stretch::UltraExpanded,
            _ => Stretch::Normal,
        };
        let font_style = match style.slant() {
            skia_safe::font_style::Slant::Italic => Style::Italic,
            skia_safe::font_style::Slant::Oblique => Style::Oblique,
            _ => Style::Normal,
        };
        let id = database.query(&Query {
            families: &families,
            weight: Weight((*style.weight()).clamp(1, 1000) as u16),
            stretch,
            style: font_style,
        })?;
        database
            .with_face_data(id, |bytes, index| {
                font_mgr.new_from_data(skia_safe::Data::new_copy(bytes), Some(index))
            })
            .flatten()
            .filter(|typeface| !is_placeholder_typeface(typeface))
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FontStyleCacheKey {
    weight: i32,
    width: i32,
    slant: i32,
}

impl FontStyleCacheKey {
    fn new(style: FontStyle) -> Self {
        Self {
            weight: *style.weight(),
            width: *style.width(),
            slant: style.slant() as i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FontLookupKey {
    family: String,
    style: FontStyleCacheKey,
}

thread_local! {
    static PLACEHOLDER_TYPEFACE_CACHE: RefCell<HashMap<u32, bool>> =
        RefCell::new(HashMap::new());
    static SYSTEM_PROGRAM_DATABASE: RefCell<Option<usvg::fontdb::Database>> =
        const { RefCell::new(None) };
    static SYSTEM_TYPEFACE_CACHE: RefCell<HashMap<FontLookupKey, Option<Typeface>>> =
        RefCell::new(HashMap::new());
    static LEGACY_TYPEFACE_CACHE: RefCell<HashMap<FontStyleCacheKey, Option<Typeface>>> =
        RefCell::new(HashMap::new());
}

pub(super) fn collect_system_families(font_mgr: &FontMgr) -> SystemFontFamilies {
    font_mgr.family_names().collect()
}

pub(super) fn has_system_family(system_families: &SystemFontFamilies, family: &str) -> bool {
    system_families.contains(family)
}

pub(super) fn match_system_family_style(
    font_mgr: &FontMgr,
    system_families: &SystemFontFamilies,
    family: &str,
    style: FontStyle,
) -> Option<Typeface> {
    if !has_system_family(system_families, family) {
        return None;
    }

    let key = FontLookupKey {
        family: family.to_string(),
        style: FontStyleCacheKey::new(style),
    };
    SYSTEM_TYPEFACE_CACHE.with(|cache| {
        if let Some(cached) = { cache.borrow().get(&key).cloned() } {
            return cached;
        }

        let matched = font_mgr
            .match_family_style(family, style)
            .filter(|typeface| !is_placeholder_typeface(typeface))
            .or_else(|| recover_installed_system_typeface(font_mgr, family, style));
        cache.borrow_mut().insert(key, matched.clone());
        matched
    })
}

pub(super) fn legacy_typeface_for_style(font_mgr: &FontMgr, style: FontStyle) -> Option<Typeface> {
    let key = FontStyleCacheKey::new(style);
    LEGACY_TYPEFACE_CACHE.with(|cache| {
        if let Some(cached) = { cache.borrow().get(&key).cloned() } {
            return cached;
        }

        let matched = font_mgr.legacy_make_typeface(None::<&str>, style);
        cache.borrow_mut().insert(key, matched.clone());
        matched
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_family_is_filtered_before_system_lookup() {
        let font_mgr = FontMgr::default();
        let system_families = SystemFontFamilies::new();

        assert!(match_system_family_style(
            &font_mgr,
            &system_families,
            "Definitely Missing RHWP Test Font",
            FontStyle::normal(),
        )
        .is_none());

        // macOS의 LastResort는 비영(非零) gid를 반환하지만 문자의 실제
        // 글리프가 아니다. 네이티브 글꼴 공급에서 이를 coverage로 세지 않는다.
        #[cfg(target_os = "macos")]
        {
            let placeholder = font_mgr
                .match_family_style(".LastResort", FontStyle::normal())
                .expect("macOS의 내장 LastResort 프로그램");
            assert_ne!(placeholder.unichar_to_glyph('나' as i32), 0);
            let chain = [TypefaceCandidate {
                typeface: placeholder,
                source: "system",
            }];
            assert!(
                select_typeface_for_character(&chain, '나').is_none(),
                "블록 대용 글리프를 실제 한글 coverage로 사용하지 않음"
            );
        }
    }

    #[test]
    fn system_family_membership_uses_exact_family_name() {
        let mut system_families = SystemFontFamilies::new();
        system_families.insert("AppleGothic".to_string());

        assert!(has_system_family(&system_families, "AppleGothic"));
        assert!(!has_system_family(&system_families, "applegothic"));
        assert!(!has_system_family(&system_families, "Missing Family"));
    }
}
