---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7648 리뷰 — 수정: 주석을 지운 뒤 각주와 미주 번호를 따로 매긴다

## 최종 판정

**머지 보류 — 초기18개 누적 통합 head의 최종 검증 진행 중.** 사용자 지시에 따라 원 PR 번호별로 작성한 검토 기록이며, 필요한 최종 검증·시각 증적 확인 뒤 판정을 갱신한다. 현재 원 PR 승인 또는 통합 PR 제출을 의미하지 않는다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR / 작성자 / base | [#7648](https://github.com/edwardkim/rhwp/pull/7648) / semanticist21 / devel |
| 초기 수용 source head | `9703c5fc91b38ca0e4d80dece8d68c9014b006e2` |
| 현재 원격 source head / 상태 | `9703c5fc91b38ca0e4d80dece8d68c9014b006e2` / OPEN / draft=False |
| 현재 통합 code head / base | `2d6f82fbba427035eccfddaa818b5de4bb878e31` / `bfa28a77a2714482f647ee6a7b9aa7f7876d8404` |
| 관련 이슈 | [#3047](https://github.com/edwardkim/rhwp/issues/3047) |

## 적용과 검토 범위

원 기여는 초기18개 scope에 이미 체리픽된 기능 commit에 한정한다. 초기 수용 뒤 새 원격 source가 있더라도 추가 체리픽하지 않는다. 메인터너 보정은 같은 review 브랜치에서 분리해 보존하며 최종 code head의 검증으로 판정한다.

| 원 commit | 최초 검토 적용 commit(역사) | 현재 rebase 적용 commit |
| --- | --- | --- |
| `43fd0e509bcc021ec4dc3fd6f896270885159916` | `5394ca686` | `bfcd9614e087501b016d07b6c7b828c6c4c46e10` |
| `ddf8c8c0b6b68d4bdf82dc5cc98435572a80c3b5` | `2e33dd6ca` | `798f1680aa0db6f8da0be167a40d8d475cdce7ed` |

최초 검토의 역사 관측: 현재 Native3계약 및 공개WASM5번호상태×3저장단계15PASS. 실제 같은 입력 번호 Print 대조는 미검증.

위 관측은 최초 후보의 결과이며 현재 code head의 통과/실패로 재사용하지 않는다. 이후 원인별 보정·실제 생산/측정/배치 소비 경로·반례와 증거는 [누적 보정 기록](semanticist21_20261008_integrated_review.md)에 연결한다. 이 PR의 적용/비적용 경로와 필수 개별 검증 결과는 최종 실행 자료를 확인한 뒤 아래에 확정한다.

### 이 PR의 코드·계약 검사

초기 적용 commit의 diff에서 분리한 변경 파일은 다음과 같다. 원 commit SHA가 Git 객체로 남지 않은 경우에도 backup에 보존한 최초 cherry-pick commit의 patch를 사용하며, 최신 원격 diff를 초기 수용 범위로 대체하지 않는다.

- `src/document_core/commands/footnote_ops.rs`
- `tests/cases/delete_note_numbers_each_kind.rs`

| 현재 검사 소스 | 이 PR에서 추가한 계약 함수 | 현재 상태 |
| --- | --- | --- |
| `tests/cases/delete_note_numbers_each_kind.rs` | `deleting_the_first_footnote_makes_the_next_footnote_1`, `deleting_one_kind_leaves_the_other_kind_numbers`, `deleting_a_footnote_keeps_the_endnote_start_number` | 현재 suite에 존재, 최종 실행 결과 대기 |

이 목록은 현재 source와 최초 적용 patch를 대조한 결과이며 실행 통과를 뜻하지 않는다. 실제 검사 결과는 이 함수가 실행된 전체 nextest/focused 로그에서 PR별로 추출하고, 최종 입력·Print·Native/fresh 및 확대 판독을 해당 변경 의미에 연결한다. 공통 빌드·회귀 성공만으로 이 PR의 적용 경계나 편집 후 저장 출력이 검증됐다고 판정하지 않는다.

## 검증 입력·결과와 조판 원칙

현재 Native 빌드와 KPS77/가맹사업142/CGMP48 등5원본 쪽수 진단은 통과했다. 가맹사업42쪽 새 Native review는94.27957%이며 표·각주 비겹침을 직접 확인했다. 이 선택 증거는 각 PR의 편집 저장본 Print 계약 또는 전쪽/fresh WASM 통과를 대신하지 않는다.

fmt·Native/WASM32 Clippy와 workspace build는 v172 source에서 통과했다. all-target Clippy는 봉투 계약 검사의 제외 모듈 선언 누락으로 실패하여 `2d6f82fbb`에서 시험지 제외 helper만 연결했다. renderer source는 동일하다. v173 fmt/workspace Clippy·정책부터 fresh WASM·전체 release-test nextest·Native Skia3종을 순차로 재검증하며, 전쪽 시각과 실제 Studio/대형 복합 검증은 현재 실행 중 또는 대기 중이다. 실제 끝 exit·최저값·누락·쪽수·직접 확대 판독과 입력/기준 PDF SHA를 최종 기록에 연결한다. 조판/회귀/입력 커밋 검토의 미완료 범위는 **미검증**이다. 절대px 기대값·baseline을 통과 목적으로 완화하지 않는다.

## 시각 증적과 남은 범위

최종 Native/fresh TSV와 대표 review/overlay를 같은 입력·쪽·source로 연결한다. 현재 source의 증적은 ignored `output/pr-review/semanticist21-20261008/pr-preparation-native-whole-v172-result.json`, `pr-preparation-wasm-whole-v174-result.json` 및 v171 retry2 기록에서 실제 종료 결과를 확인한다. 미달·구조 결함이 있으면 보정 후 재실행하며 최종 이미지와 URL은 새 head로 확정한다.

## 통합 뒤 원 PR 후속 처리 계획

사용자 승인 후 통합 PR과 정확한 head CI를 확인한다. 통합 merge SHA·이 PR의 해결 범위·보정 이유와 원 기여를 구분한 한국어 존댓말 문안을 준비하고, 실제 시각 증적 사용 시 merge SHA 고정 PNG를 첨부한다. 원 PR 종료/댓글은 승인 범위와 현재 상태를 재확인한 뒤 수행한다. 최초18개 밖의 후속 PR은 이번 통합 대상이 아니다.
