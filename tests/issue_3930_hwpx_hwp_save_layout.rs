//! Issue #3930/#3820 — HWPX 저장 모델과 별도 HWP 원본의 page owner를 보존한다.
//! HWPX 조판 회귀는 사용자 요청으로 다른 브랜치에서 처리한다.

use std::fs;
use std::path::Path;

use rhwp::model::control::Control;
use rhwp::model::header_footer::{HeaderFooterApply, MasterPage};
use rhwp::model::shape::ShapeObject;
use rhwp::model::style::BorderLineType;
use rhwp::wasm_api::HwpDocument;

const FIXTURE: &str = "samples/2025 행정업무운영 편람(최종).hwpx";
const HWP_FIXTURE: &str = "samples/2025 행정업무운영 편람(최종).hwp";
const PAGE_30: u32 = 29;
const PAGE_144: u32 = 143;
const PAGE_145: u32 = 144;
// 쪽 번호는 독립 정본 `pdf/2025 행정업무운영 편람(최종)-hwp(x)-kopub-2024.pdf`(한컴 2024 +
// KoPub 내장, 383쪽)의 물리 쪽이고 상수 값은 그 0-기반 색인이다. [#5923] 에서 한 쪽씩 당긴
// 값(281 …)은 rhwp 가 제5장 간지 뒤 PageHide 빈 쪽(정본 278쪽)을 만들지 않아 생긴 어긋남을
// 받아 준 것이었다. 그 빈 쪽을 되살린 뒤 Q5·Q8·Q9·Q10·Q16·Q27 은 정본과 같은 물리 쪽에 온다.
const PAGE_283: u32 = 282;
const PAGE_284: u32 = 283;
const PAGE_285: u32 = 284;
const PAGE_286: u32 = 285;
const PAGE_287: u32 = 286;
const PAGE_290: u32 = 289;
const PAGE_291: u32 = 290;
const PAGE_294: u32 = 293;
const PAGE_295: u32 = 294;
const PAGE_296: u32 = 295;
const Q5_RESPONSE_FIRST_LINE: &str = "문서는 결재권자의 결재가 완료된 시점에";
const Q9_TITLE: &str = "보조기관, 보좌기관, 합의제행정기관의 의미";
const Q10_TITLE: &str = "공문서 작성시 연·월·일의 정확한 표기방법";
const Q16_TITLE: &str = "문서의 결재과정에서 협조자는 문서 수정이나 반려가 가능한지요?";
const Q27_TITLE: &str = "소방서장이 지시한 업무에 대해서 소방파출소장이 문서를";
const Q30_TITLE: &str = "직속기관, 사업소, 출장소, 구청";
const ATTACHMENT_GUIDANCE: &str = "기안문에 작성한 붙임 문서를 첨부";

fn page_tree(document: &HwpDocument, page: u32) -> String {
    document
        .get_page_render_tree(page)
        .unwrap_or_else(|error| panic!("p{} render tree: {error:?}", page + 1))
}

fn master_page_text(master_page: &MasterPage) -> String {
    let mut text = String::new();
    for paragraph in &master_page.paragraphs {
        text.push_str(&paragraph.text);
        for control in &paragraph.controls {
            let Control::Shape(shape) = control else {
                continue;
            };
            let Some(text_box) = shape
                .drawing()
                .and_then(|drawing| drawing.text_box.as_ref())
            else {
                continue;
            };
            for text_box_paragraph in &text_box.paragraphs {
                text.push_str(&text_box_paragraph.text);
            }
        }
    }
    text
}

#[test]
fn issue_3930_preserves_serialized_master_page_and_picture_model() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE);
    let bytes = fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    // CLI가 사용하는 native HwpDocument 래퍼까지 동일하게 통과해야 한다.
    let mut source = HwpDocument::from_bytes(&bytes).expect("HWPX fixture parse");

    // 사용자 요청: 이 HWPX의 조판 검사는 다른 브랜치에서 처리한다.
    // 아래의 저장·재열기 모델 계약은 유지한다.
    let source_border_fill = &source.document().doc_info.border_fills[67];
    assert_eq!(
        source_border_fill.borders[0].line_type,
        BorderLineType::Dot,
        "HWPX DASH 테두리는 Hancom HWP5 code 3 점선으로 읽어야 한다"
    );
    // CLI/MCP 저장 경로도 배포용 해제 단계를 먼저 거치므로 같은 순서로 검증한다.
    source
        .convert_to_editable_native()
        .expect("편집 가능 문서 정규화");
    let saved = source.export_hwp_with_adapter().expect("HWP 저장");

    // HWPX 원본은 그대로이며 HWP 저장본에만 한컴2020 바탕쪽의
    // SECTION_DEF 19바이트 꼬리를 넣는다.
    let section_index = 10;
    let section = &source.document().sections[section_index];
    assert_eq!(
        section.section_def.raw_ctrl_extra.len(),
        0,
        "구역 {section_index} source root SectionDef는 HWP lowering에 오염되지 않는다"
    );
    let inline_section_def = section.paragraphs[0]
        .controls
        .iter()
        .find_map(|control| match control {
            Control::SectionDef(section_def) => Some(section_def.as_ref()),
            _ => None,
        })
        .expect("첫 문단 SectionDef");
    assert_eq!(
        inline_section_def.raw_ctrl_extra.len(),
        0,
        "구역 {section_index} source inline SectionDef는 HWP lowering에 오염되지 않는다"
    );
    let reloaded = HwpDocument::from_bytes(&saved).expect("저장 HWP 재로드");

    assert_eq!(
        reloaded.document().sections[10]
            .section_def
            .raw_ctrl_extra
            .len(),
        19,
        "직렬화된 구역 10 SectionDef도 HWP 2020 바탕쪽 tail을 보존해야 한다"
    );

    let section = &reloaded.document().sections[2].section_def;
    let base_master_pages: Vec<&MasterPage> = section
        .master_pages
        .iter()
        .filter(|master_page| !master_page.is_extension)
        .collect();
    assert_eq!(base_master_pages.len(), 1, "HWP 2020 단일 Odd 저장 슬롯");
    assert_eq!(base_master_pages[0].apply_to, HeaderFooterApply::Odd);
    // 한컴 2020은 아래 SECTION_DEF 0x80000000 플래그로 이전 구역의 짝수 바탕쪽을
    // 상속한다. HWP5 parser도 이 단일 Odd 계약을 그대로 복원해야 한다.
    assert!(
        master_page_text(base_master_pages[0]).contains("제2장. 공문서 관리"),
        "홀수 쪽은 현재 구역 장 제목 바탕쪽을 사용해야 한다"
    );
    assert_eq!(
        section.flags & 0xe000_0000,
        0x8000_0000,
        "단일 Odd 슬롯은 한컴 2020의 이전 짝수 쪽 상속 플래그여야 한다"
    );
    assert_eq!(
        reloaded.document().doc_info.border_fills[67].borders[0].line_type,
        BorderLineType::Dot,
        "저장 HWP도 날인 상자의 점선 BORDER_FILL을 유지해야 한다"
    );

    let first_picture = reloaded.document().sections[0]
        .paragraphs
        .iter()
        .flat_map(|paragraph| paragraph.controls.iter())
        .find_map(|control| match control {
            Control::Picture(picture) => Some(picture.as_ref()),
            _ => None,
        })
        .expect("첫 그림");
    let grouped_picture = reloaded.document().sections[0]
        .paragraphs
        .iter()
        .flat_map(|paragraph| paragraph.controls.iter())
        .find_map(|control| match control {
            Control::Shape(shape) => match shape.as_ref() {
                ShapeObject::Group(group) => group.children.iter().find_map(|child| match child {
                    ShapeObject::Picture(picture) => Some(picture.as_ref()),
                    _ => None,
                }),
                _ => None,
            },
            _ => None,
        })
        .expect("묶음 내부 그림");
    for picture in [first_picture, grouped_picture] {
        assert_eq!(
            picture.raw_picture_extra.len(),
            18,
            "HWPX 그림의 HWP5 SC_PICTURE extra 길이"
        );
        assert_eq!(
            &picture.raw_picture_extra[9..17],
            &[0; 8],
            "한컴 HWPX 저장본처럼 SC_PICTURE original image size는 0으로 쓴다"
        );
    }
    assert_eq!(grouped_picture.image_attr.brightness, 0);
    assert_eq!(grouped_picture.image_attr.contrast, 8);
}

/// native HWP 원본의 6x5 Q&A RowBreak 표에서 짧은 마지막 응답 tail은 저장
/// frame owner를 유지한다. Stage 131의 terminal spacer/guide 보정은 386쪽을 383쪽으로 낮추며,
/// HWPX fixture의 저장/roundtrip page-count 계약과는 별도로 고정한다 (#3820).
///
/// [#5751] 기대값을 383 → 385 로 갱신했다. `#501` 가드를 렌더와 같은 기준으로
/// 맞추면서 이 문서의 표 43개가 각 1행씩 내용에 맞게 늘었다. 383 은 한컴 2020
/// 기준이고 한글 2022 는 같은 파일을 384쪽으로 조판한다 — 갱신 전후 모두 오차 1.
///
/// [#5923] 비-TAC 다문단 셀 trailing 줄간격 제외로 385 → 384 — 한글 2022 조판과
/// 정확히 일치한다. Q8 표제 위치도 같은 이유로 한 쪽 당겨진다(283 → 282, 0-기반).
#[test]
fn issue_3820_hwp5_qa_rowbreak_tail_reduces_page_count() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(HWP_FIXTURE);
    let bytes = fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let source = HwpDocument::from_bytes(&bytes).expect("HWP fixture parse");

    assert!(
        page_tree(&source, PAGE_285).contains("홈페이지상의 질의에 대하여"),
        "Hancom PDF physical p285와 같이 Q8 표제는 Q7 tail 뒤 같은 쪽에서 시작해야 한다"
    );
}
