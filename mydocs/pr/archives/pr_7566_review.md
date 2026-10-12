---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-13
---

# PR #7566 리뷰 — 수정: 펼친 1×1 래퍼 안 누름틀이 바깥 셀 주소를 잃지 않게 한다

## 최종 판정

**머지 보류 — 초기18개 누적 통합 head의 최종 검증 진행 중.** 사용자 지시에 따라 원 PR 번호별로 작성한 검토 기록이며, 필요한 최종 검증·시각 증적 확인 뒤 판정을 갱신한다. 현재 원 PR 승인 또는 통합 PR 제출을 의미하지 않는다. 모든 필수 테스트·전쪽 Native/fresh 비교·직접 판독이 완료되고 실패/미달이 해결되면 사용자 지시에 따라 **메인터너 보정으로 수용 가능**으로 갱신한다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR / 작성자 / base | [#7566](https://github.com/edwardkim/rhwp/pull/7566) / semanticist21 / devel |
| 초기 수용 source head | `abf6f9259ac45ee25e1a2ccd4fd5b7c2073c8dcf` |
| 2026-10-12 원격 조회 source head / 상태 | `abf6f9259ac45ee25e1a2ccd4fd5b7c2073c8dcf` / OPEN / draft=False |
| 검증 생산 소스 / 고정 base | `940ea1322617ccff5a7b47d6920ae96e94cfffad` / `bfa28a77a2714482f647ee6a7b9aa7f7876d8404` |
| 이번 검토 기록 기준 통합 head | `75bd77e09443188701397bb7298eb5d27e10a692` |
| 관련 이슈 | 원 PR 본문 참조; 자동 종료 범위는 최종 검증 뒤 확정 |

## 적용과 검토 범위

원 기여는 초기18개 scope에 이미 체리픽된 기능 commit에 한정한다. 초기 수용 뒤 새 원격 source가 있더라도 추가 체리픽하지 않는다. 메인터너 보정은 같은 review 브랜치에서 분리해 보존하며 최종 code head의 검증으로 판정한다.

| 원 commit | 최초 검토 적용 commit(역사) | 현재 rebase 적용 commit |
| --- | --- | --- |
| `d3cf3161c5077e96ab1745be710b1102613a4ae3` | `b4f958c42` | `918299e2bf01f84cef0835e27479bf6020357178` |
| `00efed2547e2d625c74ad82f056c2a6d57bf294a` | `51aa04372` | `6b246098230e97af5d736517b70677983e65d9a7` |
| `3f8e6251ceb36eac1aa1a9e7f4787525fbf15c51` | `0b5d0af07` | `6475cbd86c83460cebdf08201e0ef2134fbbf2dc` |

최초 검토의 역사 관측: Native 셀 주소/활성화 계약 통과. fresh WASM UI 활성화는 미검증.

위 관측은 최초 후보의 결과이며 현재 code head의 통과/실패로 재사용하지 않는다. 이후 원인별 보정·실제 생산/측정/배치 소비 경로·반례와 증거는 [누적 보정 기록](semanticist21_20261008_integrated_review.md)에 연결한다. 이 PR의 적용/비적용 경로와 필수 개별 검증 결과는 최종 실행 자료를 확인한 뒤 아래에 확정한다.

### 이 PR의 코드·계약 검사

초기 적용 commit의 diff에서 분리한 변경 파일은 다음과 같다. 원 commit SHA가 Git 객체로 남지 않은 경우에도 backup에 보존한 최초 cherry-pick commit의 patch를 사용하며, 최신 원격 diff를 초기 수용 범위로 대체하지 않는다.

- `src/renderer/layout/table_layout.rs`
- `src/renderer/layout/table_partial.rs`
- `tests/cases/clickhere_wrapper_cell_path.rs`

| 현재 검사 소스 | 이 PR에서 추가한 계약 함수 | 현재 상태 |
| --- | --- | --- |
| `tests/cases/clickhere_wrapper_cell_path.rs` | `nontransparent_nested_table_preserves_full_cell_path`, `transparent_wrapper_preserves_full_cell_path`, `transparent_wrapper_activation_hides_its_guide`, `split_transparent_wrapper_preserves_full_cell_path` | v345 실제 4 PASS; 최종 통합/시각 gate 대기 |

이 목록은 현재 source와 최초 적용 patch를 대조한 결과이며 실행 통과를 뜻하지 않는다. 실제 검사 결과는 이 함수가 실행된 전체 nextest/focused 로그에서 PR별로 추출하고, 최종 입력·Print·Native/fresh 및 확대 판독을 해당 변경 의미에 연결한다. 공통 빌드·회귀 성공만으로 이 PR의 적용 경계나 편집 후 저장 출력이 검증됐다고 판정하지 않는다.

### 현재 원 PR 계약의 실제 실행 확인

2026-10-12 초기 수용 범위와 현재 검사 소스를 다시 대조했다. 위 계약 4건은
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
원격의 후속 source 변경은 조회만 했으며 추가 체리픽하지 않았다. PR 생성은 사용자 승인 뒤 진행한다.

## 검증 입력·결과와 조판 원칙

현재 Native 빌드와 KPS77/가맹사업142/CGMP48 등5원본 쪽수 진단은 통과했다. 가맹사업42쪽 새 Native review는94.27957%이며 표·각주 비겹침을 직접 확인했다. 이 선택 증거는 각 PR의 편집 저장본 Print 계약 또는 전쪽/fresh WASM 통과를 대신하지 않는다.

fmt·Native/WASM32 Clippy와 workspace build는 v172 source에서 통과했다. all-target Clippy는 봉투 계약 검사의 제외 모듈 선언 누락으로 실패하여 `2d6f82fbb`에서 시험지 제외 helper만 연결했다. renderer source는 동일하다. v173 fmt·workspace/all-target Clippy·고정 base manifest/unit 정책은 통과했다. Native/WASM32 Clippy·workspace build의 v172 성공은 동일한 production source의 증거이며, 테스트 모듈 복구 뒤 all-target 경로를 다시 실행했다. fresh WASM 최적화 빌드는 exit0으로 완료됐고 JS/WASM의 pkg·Studio public 동일성을 확인했다. 공개 API runner6종과 #7566의 Studio 실제 클릭도 통과했다. 개별 범위와 byte SHA는 누적 기록140절에 연결했으며 전체 release-test nextest·Native Skia3종·대형 복합 검증의 최종 결과는 대기 중이다. KPS Native 전쪽77쪽은 최저42쪽90.91935%·90% 미만0·누락0·77/77쪽으로 확인했으며, 이 결과를 해당 PR의 편집 저장본 또는 fresh WASM의 시각 증거로 대신하지 않는다. 실제 끝 exit·최저값·누락·쪽수·직접 확대 판독과 입력/기준 PDF SHA를 최종 기록에 연결한다. 조판/회귀/입력 커밋 검토의 미완료 범위는 **미검증**이다. 절대px 기대값·baseline을 통과 목적으로 완화하지 않는다.

## 시각 증적과 남은 범위

새 현재 wrapper 저장본의 독립 Print와 Native 비교는1쪽35.95134%로 미달이다. 실제 이미지에서
뒤 두 중첩 표의 여분 외곽선·세로 점유와 뒤 문단 위치 차이를 확인했다. 실제 Studio 클릭 계약은
통과했지만 저장본의 Print 일치는 별도 미충족이다. 누적 기록141절과
`current-print-unexecuted-native-v183/current-05-17a174162fd3/`에 연결한 원본·Print·실패 증거를
보존하고 보정·최종 Native/fresh 재검증 뒤 판정한다.

최종 Native/fresh TSV와 대표 review/overlay를 같은 입력·쪽·source로 연결한다. 현재 source의 증적은 ignored `output/pr-review/semanticist21-20261008/pr-preparation-native-whole-v172-result.json`, `pr-preparation-wasm-whole-v174-result.json` 및 v171 retry2 기록에서 실제 종료 결과를 확인한다. 미달·구조 결함이 있으면 보정 후 재실행하며 최종 이미지와 URL은 새 head로 확정한다.

## 통합 뒤 원 PR 후속 처리 계획

사용자 승인 후 통합 PR과 정확한 head CI를 확인한다. 통합 merge SHA·이 PR의 해결 범위·보정 이유와 원 기여를 구분한 한국어 존댓말 문안을 준비하고, 실제 시각 증적 사용 시 merge SHA 고정 PNG를 첨부한다. 원 PR 종료/댓글은 승인 범위와 현재 상태를 재확인한 뒤 수행한다. 최초18개 밖의 후속 PR은 이번 통합 대상이 아니다.


## 커밋 후 현재 통합 head의 계약 재검증 — 2026-10-13

`review/semanticist21-20261008`의 커밋 `3cdcb2e20a47dc23dfdd6d1b399ad4d247430b44`에서
이 PR의 위 계약 4개가 모두 실제 전체 nextest 실행에서 PASS했다.
고정 base는 `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`이며 fetch 후 SHA가 같아 rebase하지 않았다.
원 PR 최신 head도 기존 접수 기록과 같았다. 원격 조회는 참고값이며 merge 직전 다시 확인한다.
전체 nextest는10258 PASS/12 FAIL/45 skipped·exit100이고, 원 계약 전체78/78 PASS를
통합 PR 전체 성공으로 확대하지 않는다. 실패별 보정과 필수 시각/backend 검증 뒤 최종 판정을 갱신한다.
원문 로그와 함수별 연결은 ignored `pr-review-resumed-v447/full-nextest.log`,
`original-pr-contract-results.json` 및 [누적 기록](semanticist21_20261008_integrated_review.md)에 보존한다.
