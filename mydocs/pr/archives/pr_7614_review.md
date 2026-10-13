---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-13
---

# PR #7614 리뷰 — 수정: HTML 붙이기가 논리 캐럿을 쓰고 문단 끝 자동 번호 자리를 지킨다

## 최종 판정

**머지 보류 — 118쪽 회귀 보정과 Native 확인 완료; 새 fresh·전쪽·최종 게이트 검증 진행 중.** 사용자 지시에 따라 원 PR 번호별로 작성한 검토 기록이며, 필요한 최종 검증·시각 증적 확인 뒤 판정을 갱신한다. 현재 원 PR 승인 또는 통합 PR 제출을 의미하지 않는다. 모든 필수 테스트·전쪽 Native/fresh 비교·직접 판독이 완료되고 실패/미달이 해결되면 사용자 지시에 따라 **메인터너 보정 후 승인**으로 갱신한다.

## 현재 개별 검토 상태 — 2026-10-13

| 항목 | 확인 결과 |
| --- | --- |
| 작업 브랜치 / 고정 base | `review/semanticist21-20261008` / `552f5228d2dbfd6046c838f14241704f1bcb8a05` |
| 현재 production 보정 | `ddf17f86476293b114b6236bfa6cf1ebc9d9d474` — 원본 전체 행/중첩 셀의 프레임 출처와 정렬 높이 계약 분리 |
| 현재 Native/fresh 재검증 code head | ddf17f864 Native 경계17·CGMP48·KPS77쪽 통과/대표 직접 판독. fresh 빌드exit0·pkg/public 해시 일치. 가맹사업 fresh17·전쪽·관련 회귀/lint·Canvas는 미완료 |
| 원 기여 적용 | 위 적용 표의 현재 SHA는 모두 HEAD의 조상이며 최초 적용 patch와 stable patch-id가 같다 |
| 이 PR의 실제 계약 실행 | 직전 b4ff8c 전체 nextest에서 8/8 PASS, FAIL/SKIP/미관측0; 현재 검사 source SHA 동일 확인 |
| 현재 통합 완료 판정 | **머지 보류** — 직전 b4ff8c의 전체10267개/원PR78계약·Skia/doc는 PASS였으나, 전142쪽 직접 대조에서118쪽 정렬 회귀 확인. 새 보정 소스의 검증 필요 |
| 원격 진행 권한 | 작업지시자의 자동 승인: 모든 필수 테스트·시각 검증 완료 후 통합 PR 생성 → 최신 head CI 확인 → 머지 → 후속 처리 |

이 PR의 입력·호출 경로·계약 함수와 미검증 범위는 아래 개별 항목을 따른다. v447 계약 PASS와
후속 #157 보정의 전2쪽 Native/fresh 통과는 각각 해당 SHA·입력 범위의 결과이며, 이 PR의
편집 저장본 전체나 최종 통합 검증의 통과로 바꾸지 않는다. 아래 이전 진행 문단은 당시 관측이다.
현재 적용 SHA·진행 권한·완료 여부는 이 절을 우선한다. 현재 공통 보정과 순차 검증의 실행 결과는 [최신 PR 준비](semanticist21_20261008_integrated_review.md#진행-갱신208--k-water-fresh-전27쪽-판독-및-최신-대표-png-2026-10-13)에 연결한다. 모든 필수 검증 완료 후 최상단 판정을
**메인터너 보정 후 승인**으로 갱신한다.

이 PR의 검증 함수별 원문 로그 연결은 ignored
`pr-review-resumed-v447/original-pr-contract-results.json`의 PR #7614 항목이고,
리베이스 적용 대조는 `regression-root-cause-batch-v448/individual-pr-current-application-audit.json`이다.
메인터너의 공통 표 프레임 보정·새 base 비교·양 backend 증거는
[누적 기록의 진행173](semanticist21_20261008_integrated_review.md#진행-갱신173--157-양-backend-선행-조건과-정식-회귀-검출-2026-10-13)에 연결한다.

### 이전 검증 기록 — b4ff8c

현재 production `b4ff8c81b5d986baf47bad9a3eee83cbd9d8367b`의 전체 nextest는 **10267 PASS·45 SKIP·exit0**이다. 이 PR 계약 **8/8 PASS**와 동일 검사 source를 v452 `original-pr-contract-results.json`에서 확인했다. Native Skia3종과 doc는 완료했으며, 최종 전쪽·Canvas와 새 정식 관계 회귀 검증은 아직 남아 **머지 보류**를 유지한다. 아래7f04c 결과는 수정 전 역사 기록이다.

수정 전 production `7f04c48cd395086ed59c08565f343d3f9e1b013d`의 전체 회귀에서 이 PR 계약 8/8개가 PASS했다. 현재 production `b4ff8c81b5d986baf47bad9a3eee83cbd9d8367b`는 72쪽 배치 차이를 보정한 새 소스이며, 이전 PASS를 현재 소스의 검증으로 대신하지 않는다. Native/fresh 영향 쪽·최종 회귀 및 필수 게이트 완료 전 판정은 **머지 보류**다. 근거와 소비 경로는 [누적 기록의 진행 갱신195](semanticist21_20261008_integrated_review.md#진행-갱신195--전-회귀-통과와-72쪽-정렬-높이-소비-경로-보정-2026-10-13)에 연결한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR / 작성자 / base | [#7614](https://github.com/edwardkim/rhwp/pull/7614) / semanticist21 / devel |
| 초기 수용 source head | `2d4148eb8bcb561b2eac2004c7a64741864e7431` |
| 2026-10-13 원격 조회 source head / 상태 | `2d4148eb8bcb561b2eac2004c7a64741864e7431` / OPEN / draft=False |
| 과거 검증 생산 소스 / 당시 base | `940ea1322617ccff5a7b47d6920ae96e94cfffad` / `bfa28a77a2714482f647ee6a7b9aa7f7876d8404` |
| 이번 검토 기록 기준 통합 head | `5cd155886dbcfb19d5f4fa17ae778f278940e08b` |
| 관련 이슈 | [#2740](https://github.com/edwardkim/rhwp/issues/2740), [#7444](https://github.com/edwardkim/rhwp/issues/7444), [#7612](https://github.com/edwardkim/rhwp/issues/7612) |

## 적용과 검토 범위

원 기여는 초기18개 scope에 이미 체리픽된 기능 commit에 한정한다. 초기 수용 뒤 새 원격 source가 있더라도 추가 체리픽하지 않는다. 메인터너 보정은 같은 review 브랜치에서 분리해 보존하며 최종 code head의 검증으로 판정한다.

| 원 commit | 최초 검토 적용 commit(역사) | 현재 rebase 적용 commit |
| --- | --- | --- |
| `cb8f6f48d0310ae5ebfe4c602c38b3008cc356c9` | `093ccccb7` | `490cdd07cd1ba4f161cd1b706d39dfc97d2412d1` |
| `b8a6f54e73d32b0fbd4a5b9b4b97d407c5a1240a` | `28c97f2a4` | `4055c3f74e9b15c4df7e1e77b8de1fce18d0409c` |

최초 검토의 역사 관측: 현재 Native49 묶음의 HTML/문단 끝 번호 계약 통과; 공개 WASM 본문 논리캐럿7상태/왕복28PASS. 현재 편집 출력 Print 미검증; source CodeQL 실패와 통합CI를 구분.

위 관측은 최초 후보의 결과이며 현재 code head의 통과/실패로 재사용하지 않는다. 이후 원인별 보정·실제 생산/측정/배치 소비 경로·반례와 증거는 [누적 보정 기록](semanticist21_20261008_integrated_review.md)에 연결한다. 이 PR의 적용/비적용 경로와 필수 개별 검증 결과는 최종 실행 자료를 확인한 뒤 아래에 확정한다.

### 이 PR의 코드·계약 검사

초기 적용 commit의 diff에서 분리한 변경 파일은 다음과 같다. 원 commit SHA가 Git 객체로 남지 않은 경우에도 backup에 보존한 최초 cherry-pick commit의 patch를 사용하며, 최신 원격 diff를 초기 수용 범위로 대체하지 않는다.

- `src/document_core/commands/clipboard.rs`
- `src/document_core/commands/html_import.rs`
- `src/model/paragraph.rs`
- `src/wasm_api.rs`
- `tests/cases/auto_number_trailing_placeholder.rs`
- `tests/cases/paste_html_caret_axis.rs`

| 현재 검사 소스 | 이 PR에서 추가한 계약 함수 | 현재 상태 |
| --- | --- | --- |
| `tests/cases/auto_number_trailing_placeholder.rs` | `typing_after_a_trailing_auto_number_keeps_the_number_in_place`, `enter_after_a_trailing_auto_number_leaves_the_number_in_its_paragraph`, `typing_after_an_auto_number_merged_with_the_next_paragraph_keeps_the_number` | v345 실제 3 PASS; 최종 통합/시각 gate 대기 |
| `tests/cases/paste_html_caret_axis.rs` | `single_paragraph_paste_lands_at_the_logical_caret_around_a_footnote`, `single_paragraph_paste_lands_after_an_inline_equation`, `multi_paragraph_paste_splits_at_the_logical_caret_after_a_footnote`, `cell_paste_of_paragraphs_splits_at_the_text_caret_after_an_inline_equation`, `cell_paste_of_paragraphs_counts_a_char_overlap_before_the_text_caret` | v345 실제 5 PASS; 최종 통합/시각 gate 대기 |

이 목록은 현재 source와 최초 적용 patch를 대조한 결과이며 실행 통과를 뜻하지 않는다. 실제 검사 결과는 이 함수가 실행된 전체 nextest/focused 로그에서 PR별로 추출하고, 최종 입력·Print·Native/fresh 및 확대 판독을 해당 변경 의미에 연결한다. 공통 빌드·회귀 성공만으로 이 PR의 적용 경계나 편집 후 저장 출력이 검증됐다고 판정하지 않는다.

### 현재 원 PR 계약의 실제 실행 확인

2026-10-12 초기 수용 범위와 현재 검사 소스를 다시 대조했다. 위 계약 8건은
v345 전체 회귀의 실제 실행 로그에서 모두 PASS다. 해당 검사 파일은 생산 소스 커밋
`940ea1322617ccff5a7b47d6920ae96e94cfffad`의 바이트와 동일하며, 최근 교육과정 검사 이관은 이 PR의 검사나 생산 코드를 바꾸지 않았다.
로그는 ignored `output/pr-review/semanticist21-20261008/logs/whole-regression-audit-v345-nextest.log`,
정확한 함수·소스 SHA·로그 행의 연결은 `original-pr-contract-readiness-v396.json`에 보존한다.
전체 v345는10254PASS/12FAIL였으므로 이 PR의 PASS를 전체 통합 성공으로 확대하지 않는다.

사용자 지시에 따라 원래 체리픽 문서의 보정과 증거 정리를 우선한다. 기존 초기 편집 입력과
같은 독립 Print48쌍/52쪽의 현재 Native/fresh WASM 비교는 v396에서 실행 중이다.
7개 논리 캐럿 입력은 현재 v175 저장본과 기존 v94 증적 입력의 바이트가 모두 동일함을 확인했다.
같은 생산 소스의 기존 각주·미주/누름틀8쌍(v322/v326) 증거는 입력·Print·소스 SHA와 전쪽 결과를
확인해 해당 범위에만 연결한다. 다른 PR 문서나 미실행 경로의 통과로 대신하지 않는다.
대표 review/overlay의 실제 판독과 이 PR의 적용·비적용 경로를 확정한 뒤 최종 판정을 갱신한다.
원격의 후속 source 변경은 조회만 했으며 추가 체리픽하지 않았다. 현재 원격 진행 권한은 아래 자동 승인 기록을 따른다.

## 검증 입력·결과와 조판 원칙

현재 Native 빌드와 KPS77/가맹사업142/CGMP48 등5원본 쪽수 진단은 통과했다. 가맹사업42쪽 새 Native review는94.27957%이며 표·각주 비겹침을 직접 확인했다. 이 선택 증거는 각 PR의 편집 저장본 Print 계약 또는 전쪽/fresh WASM 통과를 대신하지 않는다.

fmt·Native/WASM32 Clippy와 workspace build는 v172 source에서 통과했다. all-target Clippy는 봉투 계약 검사의 제외 모듈 선언 누락으로 실패하여 `2d6f82fbb`에서 시험지 제외 helper만 연결했다. renderer source는 동일하다. v173 fmt·workspace/all-target Clippy·고정 base manifest/unit 정책은 통과했다. Native/WASM32 Clippy·workspace build의 v172 성공은 동일한 production source의 증거이며, 테스트 모듈 복구 뒤 all-target 경로를 다시 실행했다. fresh WASM 최적화 빌드는 exit0으로 완료됐고 JS/WASM의 pkg·Studio public 동일성을 확인했다. 공개 API runner6종과 #7566의 Studio 실제 클릭도 통과했다. 개별 범위와 byte SHA는 누적 기록140절에 연결했으며 전체 release-test nextest·Native Skia3종·대형 복합 검증의 최종 결과는 대기 중이다. KPS Native 전쪽77쪽은 최저42쪽90.91935%·90% 미만0·누락0·77/77쪽으로 확인했으며, 이 결과를 해당 PR의 편집 저장본 또는 fresh WASM의 시각 증거로 대신하지 않는다. 실제 끝 exit·최저값·누락·쪽수·직접 확대 판독과 입력/기준 PDF SHA를 최종 기록에 연결한다. 조판/회귀/입력 커밋 검토의 미완료 범위는 **미검증**이다. 절대px 기대값·baseline을 통과 목적으로 완화하지 않는다.

## 시각 증적과 남은 범위

최종 Native/fresh TSV와 대표 review/overlay를 같은 입력·쪽·source로 연결한다. 현재 source의 증적은 ignored `output/pr-review/semanticist21-20261008/pr-preparation-native-whole-v172-result.json`, `pr-preparation-wasm-whole-v174-result.json` 및 v171 retry2 기록에서 실제 종료 결과를 확인한다. 미달·구조 결함이 있으면 보정 후 재실행하며 최종 이미지와 URL은 새 head로 확정한다.

## 통합 뒤 원 PR 후속 처리 계획

자동 승인된 범위에서 필수 검증 완료 후 통합 PR과 정확한 head CI를 확인한다. 통합 merge SHA·이 PR의 해결 범위·보정 이유와 원 기여를 구분한 한국어 존댓말 문안을 준비하고, 실제 시각 증적 사용 시 merge SHA 고정 PNG를 첨부한다. 원 PR 종료/댓글은 승인 범위와 현재 상태를 재확인한 뒤 수행한다. 최초18개 밖의 후속 PR은 이번 통합 대상이 아니다.


## 커밋 후 현재 통합 head의 계약 재검증 — 2026-10-13

`review/semanticist21-20261008`의 커밋 `3cdcb2e20a47dc23dfdd6d1b399ad4d247430b44`에서
이 PR의 위 계약 8개가 모두 실제 전체 nextest 실행에서 PASS했다.
고정 base는 `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`이며 fetch 후 SHA가 같아 rebase하지 않았다.
원 PR 최신 head도 기존 접수 기록과 같았다. 원격 조회는 참고값이며 merge 직전 다시 확인한다.
전체 nextest는10258 PASS/12 FAIL/45 skipped·exit100이고, 원 계약 전체78/78 PASS를
통합 PR 전체 성공으로 확대하지 않는다. 실패별 보정과 필수 시각/backend 검증 뒤 최종 판정을 갱신한다.
원문 로그와 함수별 연결은 ignored `pr-review-resumed-v447/full-nextest.log`,
`original-pr-contract-results.json` 및 [누적 기록](semanticist21_20261008_integrated_review.md)에 보존한다.
