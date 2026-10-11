---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7615 리뷰 — 수정: 누름틀 문단 편집이 다른 컨트롤 슬롯과 문단 앞 정의를 지킨다

## 최종 판정

**머지 보류 — 초기18개 누적 통합 head의 최종 검증 진행 중.** 사용자 지시에 따라 원 PR 번호별로 작성한 검토 기록이며, 필요한 최종 검증·시각 증적 확인 뒤 판정을 갱신한다. 현재 원 PR 승인 또는 통합 PR 제출을 의미하지 않는다. 모든 필수 테스트·전쪽 Native/fresh 비교·직접 판독이 완료되고 실패/미달이 해결되면 사용자 지시에 따라 **메인터너 보정으로 수용 가능**으로 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR / 작성자 / base | [#7615](https://github.com/edwardkim/rhwp/pull/7615) / semanticist21 / devel |
| 초기 수용 source head | `ed9c2847523c18f658344ec89cb7ed1a426807ae` |
| 현재 원격 source head / 상태 | `4a6c27c5ffa54e4988a7e0e7956115d933725aca` / OPEN / draft=False |
| 현재 통합 code head / base | `2d6f82fbba427035eccfddaa818b5de4bb878e31` / `bfa28a77a2714482f647ee6a7b9aa7f7876d8404` |
| 관련 이슈 | [#4680](https://github.com/edwardkim/rhwp/issues/4680), [#7444](https://github.com/edwardkim/rhwp/issues/7444), [#7596](https://github.com/edwardkim/rhwp/issues/7596), [#7598](https://github.com/edwardkim/rhwp/issues/7598), [#7612](https://github.com/edwardkim/rhwp/issues/7612), [#7614](https://github.com/edwardkim/rhwp/issues/7614), [#7662](https://github.com/edwardkim/rhwp/issues/7662) |

## 적용과 검토 범위

원 기여는 초기18개 scope에 이미 체리픽된 기능 commit에 한정한다. 초기 수용 뒤 새 원격 source가 있더라도 추가 체리픽하지 않는다. 메인터너 보정은 같은 review 브랜치에서 분리해 보존하며 최종 code head의 검증으로 판정한다.

| 원 commit | 최초 검토 적용 commit(역사) | 현재 rebase 적용 commit |
| --- | --- | --- |
| `f189e5dd59d38c6e4b3c31581a7b55d03c4d20a2` | `a2ef9eab3` | `45a636b33213472eeef9559f795e4db1fcc622ee` |
| `0a76f6fabbd313ddd908da4b3949cbfb0d4db68e` | `761499928` | `3e1c4ff8c3f0fc97fcc102c4871ab5870aa147b1` |
| `c09761d797a13f31c5c6a0ca687c2134a1535c47` | `ebbec90fb` | `bfb61a3b0bf4c0ec3dba5824ea02cf0299b76138` |
| `1dad94cf07f2be5ffba6e84dc42cc521975bb862` | `60385438c` | `08c19c770711291ea2cfa207152660090627443f` |
| `4091704afca53dadf067d85be36779c6abf246d5` | `3addfced3` | `3c7ec1f9e55beb50b593743e611783e016c43f4d` |
| `8a17e2e770d98215e0d5f2259868b58a2cf1ee75` | `6a8f77dab` | `a95e5b031e699f786d72b15f21be152c3d79dee1` |
| `ae2a058d63b8340d56549097af844697925eacb5` | `2be87053d` | `083c1e560661396ec813a9271fec9e1a916c43a2` |

최초 검토의 역사 관측: Native 누름틀·정의·슬롯 보존 검사 통과. 편집 저장본 Print 대조 범위 미검증.

위 관측은 최초 후보의 결과이며 현재 code head의 통과/실패로 재사용하지 않는다. 이후 원인별 보정·실제 생산/측정/배치 소비 경로·반례와 증거는 [누적 보정 기록](semanticist21_20261008_integrated_review.md)에 연결한다. 이 PR의 적용/비적용 경로와 필수 개별 검증 결과는 최종 실행 자료를 확인한 뒤 아래에 확정한다.

### 이 PR의 코드·계약 검사

초기 적용 commit의 diff에서 분리한 변경 파일은 다음과 같다. 원 commit SHA가 Git 객체로 남지 않은 경우에도 backup에 보존한 최초 cherry-pick commit의 patch를 사용하며, 최신 원격 diff를 초기 수용 범위로 대체하지 않는다.

- `src/document_core/commands/clipboard.rs`
- `src/document_core/commands/document.rs`
- `src/document_core/commands/text_editing.rs`
- `src/document_core/hyperlink.rs`
- `src/document_core/queries/field_query.rs`
- `src/model/paragraph.rs`
- `tests/cases/auto_number_trailing_placeholder.rs`
- `tests/cases/field_paragraph_control_slots.rs`
- `tests/cases/leading_definition_controls.rs`
- `tests/cases/split_caret_axis.rs`

| 현재 검사 소스 | 이 PR에서 추가한 계약 함수 | 현재 상태 |
| --- | --- | --- |
| `tests/cases/auto_number_trailing_placeholder.rs` | `typing_after_a_trailing_auto_number_in_a_click_here_paragraph_keeps_both` | 현재 suite에 존재, 최종 실행 결과 대기 |
| `tests/cases/field_paragraph_control_slots.rs` | `control_fields_only_paragraph_keeps_slots_when_typing`, `typing_outside_field_keeps_footnote_slot`, `typing_inside_active_field_keeps_footnote_slot`, `local_replace_before_field_keeps_field_and_footnote`, `split_keeps_footnote_shape_and_bookmark_slots`, `page_break_keeps_footnote_shape_and_bookmark_slots`, `pasted_footnote_keeps_slot_before_following_field`, `typing_into_first_empty_field_keeps_later_field`, `typing_into_later_field_keeps_first_empty_field_at_start`, `typing_into_first_empty_field_keeps_footnote_and_bookmark_slots`, `breaking_before_empty_field_keeps_both_fields_in_new_paragraph`, `setting_field_value_keeps_later_footnote_slot`, `splitting_inside_filled_field_keeps_later_footnote_slot`, `setting_field_value_keeps_adjacent_field_value`, `typing_after_field_keeps_header_table_and_picture_slots`, `typing_inside_field_keeps_header_table_and_picture_slots`, `breaking_before_last_char_keeps_header_table_and_picture_slots`, `pasting_field_copied_from_header_paragraph_leaves_header_behind`, `pasting_after_field_that_ends_paragraph_keeps_field_end_slot`, `typing_in_cell_keeps_bookmark_and_field_slots`, `merging_after_field_that_ends_paragraph_keeps_field_end_slot`, `merging_paragraph_that_starts_with_empty_field_counts_its_end_slot`, `typing_after_footnote_marker_by_logical_caret_goes_after_it` | 현재 suite에 존재, 최종 실행 결과 대기 |
| `tests/cases/leading_definition_controls.rs` | `typing_before_a_leading_object_keeps_definitions_first`, `composing_before_a_leading_object_keeps_definitions_first`, `splitting_before_a_leading_object_keeps_it_first_in_the_new_paragraph`, `pasting_before_a_leading_object_keeps_definitions_first`, `splitting_before_an_inline_object_mid_paragraph_keeps_it_first`, `splitting_right_before_a_click_here_field_keeps_it_first`, `title_marks_in_the_gap_keep_their_slots`, `typing_at_the_start_of_an_ordinary_first_paragraph_is_unchanged` | 현재 suite에 존재, 최종 실행 결과 대기 |
| `tests/cases/split_caret_axis.rs` | `undo_of_enter_right_after_auto_number_restores_the_paragraph` | 현재 suite에 존재, 최종 실행 결과 대기 |

이 목록은 현재 source와 최초 적용 patch를 대조한 결과이며 실행 통과를 뜻하지 않는다. 실제 검사 결과는 이 함수가 실행된 전체 nextest/focused 로그에서 PR별로 추출하고, 최종 입력·Print·Native/fresh 및 확대 판독을 해당 변경 의미에 연결한다. 공통 빌드·회귀 성공만으로 이 PR의 적용 경계나 편집 후 저장 출력이 검증됐다고 판정하지 않는다.

## 검증 입력·결과와 조판 원칙

현재 Native 빌드와 KPS77/가맹사업142/CGMP48 등5원본 쪽수 진단은 통과했다. 가맹사업42쪽 새 Native review는94.27957%이며 표·각주 비겹침을 직접 확인했다. 이 선택 증거는 각 PR의 편집 저장본 Print 계약 또는 전쪽/fresh WASM 통과를 대신하지 않는다.

fmt·Native/WASM32 Clippy와 workspace build는 v172 source에서 통과했다. all-target Clippy는 봉투 계약 검사의 제외 모듈 선언 누락으로 실패하여 `2d6f82fbb`에서 시험지 제외 helper만 연결했다. renderer source는 동일하다. v173 fmt·workspace/all-target Clippy·고정 base manifest/unit 정책은 통과했다. Native/WASM32 Clippy·workspace build의 v172 성공은 동일한 production source의 증거이며, 테스트 모듈 복구 뒤 all-target 경로를 다시 실행했다. fresh WASM 최적화 빌드는 exit0으로 완료됐고 JS/WASM의 pkg·Studio public 동일성을 확인했다. 공개 API runner6종과 #7566의 Studio 실제 클릭도 통과했다. 개별 범위와 byte SHA는 누적 기록140절에 연결했으며 전체 release-test nextest·Native Skia3종·대형 복합 검증의 최종 결과는 대기 중이다. KPS Native 전쪽77쪽은 최저42쪽90.91935%·90% 미만0·누락0·77/77쪽으로 확인했으며, 이 결과를 해당 PR의 편집 저장본 또는 fresh WASM의 시각 증거로 대신하지 않는다. 실제 끝 exit·최저값·누락·쪽수·직접 확대 판독과 입력/기준 PDF SHA를 최종 기록에 연결한다. 조판/회귀/입력 커밋 검토의 미완료 범위는 **미검증**이다. 절대px 기대값·baseline을 통과 목적으로 완화하지 않는다.

## 시각 증적과 남은 범위

최종 Native/fresh TSV와 대표 review/overlay를 같은 입력·쪽·source로 연결한다. 현재 source의 증적은 ignored `output/pr-review/semanticist21-20261008/pr-preparation-native-whole-v172-result.json`, `pr-preparation-wasm-whole-v174-result.json` 및 v171 retry2 기록에서 실제 종료 결과를 확인한다. 미달·구조 결함이 있으면 보정 후 재실행하며 최종 이미지와 URL은 새 head로 확정한다.

## 통합 뒤 원 PR 후속 처리 계획

사용자 승인 후 통합 PR과 정확한 head CI를 확인한다. 통합 merge SHA·이 PR의 해결 범위·보정 이유와 원 기여를 구분한 한국어 존댓말 문안을 준비하고, 실제 시각 증적 사용 시 merge SHA 고정 PNG를 첨부한다. 원 PR 종료/댓글은 승인 범위와 현재 상태를 재확인한 뒤 수행한다. 최초18개 밖의 후속 PR은 이번 통합 대상이 아니다.
