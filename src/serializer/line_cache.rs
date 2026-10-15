//! 외부 소비자에게 전달할 문단 줄 캐시의 유효성.

use crate::model::paragraph::{tab_ext_is_placeholder, Paragraph};

/// 미계산 탭을 포함한 줄은 소비자가 다시 조판하도록 저장 캐시를 생략한다.
///
/// 한컴 Print는 줄 캐시와 폭 0 자리표를 함께 받으면 탭을 전진시키지 않는다.
/// 같은 본문·탭 정의에서 줄 캐시만 생략하면 실제 탭 정지를 계산한다.
/// 실제 전진량이 있는 탭 확장은 그대로 유지하고, 메모리의 줄 구성도 바꾸지 않는다.
pub(super) fn can_serialize_line_partition(para: &Paragraph) -> bool {
    !para.stored_text_partition_is_dirty()
        && !para
            .text
            .chars()
            .filter(|&ch| ch == '\t')
            .enumerate()
            .any(|(index, _)| {
                para.tab_extended
                    .get(index)
                    .is_none_or(tab_ext_is_placeholder)
            })
}
