---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-13
---

# PR #7701 체리픽·메인터너 보정 검토 계획

## 변경 전 C20 통합 검증 — 2026-10-13

변경 전 C20 검증 source HEAD는 `e2a525d36f2923918185f62610869e7c3068b9d0`, 검증 base는
`552f5228d2dbfd6046c838f14241704f1bcb8a05`다. 오늘 `upstream/devel` fetch와 rebase가 모두 성공했으며
SHA 변경·충돌은 없었다. 아래 과거 단계의 SHA·관측값은 당시 기록으로 유지한다.

이 C20 source에서 Rust 전체 회귀 **10,241건 통과·45건 제외**, Studio **1,836건 통과·실패 0건**,
Native Skia library/workspace **4,046건 통과·12건 ignored**를 확인했다.
Studio typecheck/build, Native·WASM·workspace all-target Clippy와 두 정책의 base 비교,
최적화 fresh WASM 및 Native 빌드도 exit 0이다. 모든 Cargo 빌드는 공용
`/home/tsjang/rhwp/target/pr-review`를 사용했다. 실행 명령·시간·SHA·exit code는
ignored `output/pr-review/planet6897-followup-20261011/c20-validation-sequence-results.json`,
상세 lint 결과는 `c20-lint-results.json`에 보존한다.

9개 PR의 전체 Native/fresh WASM 시각 검증은 **진행 중**이다. 이 테스트 통과를
시각 수용·PR 승인으로 판정하지 않는다. 문서별 실제 Print 세대, 전체 TSV, 대표 review와
직접 확대 판독을 대조한 뒤 이 PR의 적용 범위를 최종 판정한다. 현재 head의 결과는
`c20-native-whole-corpus-results.json`과 `c20-wasm-whole-corpus-results.json`에 기록한다.
완료되지 않은 결과 파일의 부재·일부 레코드는 통과가 아니다. 원격 통합 PR·CI·merge와
후속 처리는 아직 진행하지 않았다.


### C21 Canvas 점 페인트 보정 후보

현재 소스에는 C20 이후의 Canvas 가운뎃점 보정이 있다. 일반/효과 페인트가
SVG·Skia와 같은 전진 상자 중앙 및 점 기하 상수를 소비한다. 근거·실제 호출
차이는 [#7815 구현 검토](pr_7815_review_impl.md)의 최신 문장부호 항목을 따른다.
C20 Native 주요24문서·325쪽은 완료했지만 미달 문서가 남았다. C20 전체 WASM과
뒤 추가 corpus는 새 소스와 섞이지 않도록 중단됐다. C20 완료 증거는 그대로 보존한다.

보정 후보의 prepare·fmt·Native/WASM/all-target Clippy·workspace build와
고정 base 대비 manifest/unit-tier **8단계 preflight는 모두exit0**다.
새 code commit의 정식 회귀·fresh WASM 실제 재생·관련 전쪽 시각 검증은 아직
필요하며, 이전 C20 결과를 보정 후보의 최종 통과로 옮겨 적지 않는다.
원격 PR·CI·merge와 후속 처리는 미실행이다. 상세 후보와 명령은 ignored
`c21-canvas-dot-source-candidate.json`, `c21-canvas-lint-preflight-results.json`,
`c21-c20-source-supersession.json`에 보존한다.


## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **이어받기 조각의 바깥 위 여백**다.

- branch: `review/planet6897-followup-20261011`
- 초기 검토 base(과거 단계): `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `80879e9350b0cc8232d7c37cee62bdf192baeae4`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `1f43a042232a9d22d70195609932c240eed3beca` | `6077a02496a5470dc4d7627ccec7f9f472179e8e` |
| `80879e9350b0cc8232d7c37cee62bdf192baeae4` | `f22cdb0359cd0c0c400e1b0348c7f8c63f53be0c` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

row-boundary continuation → 요구/예약 높이 → 표 조각 top과 후속 본문을 대조한다. 기존 관계형 종료 검사는 유지했다. 새 incoming `rowbreak_continuation_reopens_outer_top`의 415쪽 assertion은 기존 #7445/#7743 이관 범위를 다시 등록한 것이었으며, 아래 동일 입력 base/current 근거로 신규 등록을 철회했다. 기존 정상 회귀와 공유 IR/소유 계약 검사는 유지한다.

## 독립 기대값과 검증 계획

행 경계와 행 내부 컷, 첫/끝 조각, 여러 rowspan 종료, 마지막 내용 이후 뒤 문단·불필요한 빈 쪽을 확인한다. 1342000의 415쪽 기준과 실제 영향을 함께 확인한다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.


## 415쪽 기대값 실패의 구역별 진단

통합 전체 회귀의 `continuation_outer_top_matches_hangul_page_counts`는 원본
`1342000_edu_curriculum_map.hwp`에서 418≠정본 415로 실패했다. 정확한 쪽수 기대값을
유지한다. 실제 source `ab5112`의 전체 `dump-pages`·쪽별 본문과 독립 Print 415쪽의
본문을 대조해 추가 쪽을 구역1(65↔64), 구역5(49↔48), 구역16(8↔7)로 좁혔다.
구역 시작 제목의 독립 본문 대응은 `1342000-print-native-page-alignment.json`이며,
5글자 집합의 이웃 쪽 유사도는 진단 보조값일 뿐 쪽 소속의 검증으로 사용하지 않는다.

같은 구역의 첫쪽을 서로 대응해 직접 PNG로 비교했다. Native25/Print25는 99.355%,
Native121/Print120은 88.71813%, Native271/Print269는 96.3668%다. 쪽수가 서로 다른
문서에서 선택한 대응 페이지의 점수는 전체 90% 게이트나 415쪽 일치의 증거가 아니다.
구역5 첫쪽에서는 위 요약 표의 내용·괘선 범위가 독립 Print와 다르고, 다음 본문 표의
일부 세로 괘선도 달랐다. 마지막 행 이월의 실제 컷과 소비 높이 원인은 계속 추적한다.
기대값·회귀 허용치·baseline은 바꾸지 않았고, 조판 수용 판정도 아직 하지 않았다.

자료는 ignored output의 `1342000-extra-page-localization.json`,
`1342000-section-opening-diagnostic-results.json`과 구역별 `review/review_001.png`다.
추가한 1342000용 유효 Unicode 글꼴 subset은 사용 cmap·glyph ID·outline·hmtx를 원본
프로그램과 대조한 비공개 검증 산출물이며 저장소에 포함하지 않는다.

## 고정 upstream base 재검증과 신규 검사 재등록 철회

`bfa28a77a2714482f647ee6a7b9aa7f7876d8404`의 실제 Native CLI를 별도로 빌드·동결한 뒤
동일 입력·독립 Print·글꼴로 `c9323fd56` production과 비교했다. 양쪽 모두 418≠415이며,
선택한 같은 물리121·124·271쪽의 내용·소유 순서, 표와 TextLine 좌표는 동일했다.
TextRun의 가로 원점/폭 차이만 작게 남았고 세로 원점/높이는 동일했다. 전415쪽 또는 fresh WASM
교육과정 검증으로 보고하지 않는다. 124쪽에서 Print의 표 내용·세로 괘선 차이가 양쪽에 보인다.

18쪽 rowbreak 반례는 Native/fresh WASM 전쪽 최저55.00512%, 미달4·6·7·8·15·16쪽이다.
7쪽 저장 frame 대조군은 두 backend 최저96.93933%지만 7쪽 체크박스 모양 차이가 남았다.
따라서 신규 mixed 함수와 절대 px 검사 두 함수를 활성 suite에서 철회했다. 정확한415쪽 기대값을
418쪽으로 바꾸거나 `ignore` 처리하지 않는다. #7743/#7741의 실제 결함은 해결되지 않았다.

원 검사·동일124쪽 대표 PNG·상세 범위는
[보존 기록](../assets/issue7445/rowbreak_continuation_20261012/README.md)에 있다.
기존 원문/PDF는 공유 계약 검사 사용 때문에 사용자 지시에 따라 유지한다.
파생 manifest 재준비 결과1456sources/6177test attributes이며 검사 함수가2개 줄었다.
`c9323fd56` 전체 회귀는10243건 중10242PASS/1FAIL(교육과정 쪽수),45SKIP였고,
철회 후 실행은 이 이전 결과와 구분해 이어서 기록한다.

## 원 head 추가 보정 접수 — 2026-10-13

최신 원 head는 `8a3e192d81fbdf925a825c09c50dd77809dae9be`다. 기존9PR/18고유 커밋에
새 원 고유 커밋1개를 추가해9PR/19고유 커밋으로 검토한다. 고정 upstream base는
`552f5228d2dbfd6046c838f14241704f1bcb8a05`, 적용 전 C15는
`1c7fb035f9b4f382e855f76af84a3745619e68ce`이며 필수 lint 묶음은 전부exit0이다.
새 보정은 이어받는 rowspan 시작 행의 요구 높이를 남은 내용과 물리 꼬리의 공통 결과로
예약하여 앞 조각이 그린 띠를 다시 세지 않는다. 실제 호출은
`RowScanQuery::whole_row_height → required_height/straddle_continuation_demand → consumed → fragment emit`이다.

원 신규 검사는 수정 전 동결 C15에서 compile0/test101로 같은30쪽 내용 경계를 재현했다.
시각 선행 조건 미충족의1342000 원 검사는 기존[#7743](https://github.com/edwardkim/rhwp/issues/7743)에
연결하고 활성 suite에 재등록하지 않는다. 원문 두 개와 이전 원문은
[보존 기록](../assets/issue7445/rowbreak_continuation_20261012/README.md)에 함께 남겼다.
현재 보정 후 경계·회귀·Native/fresh WASM 검증은 아직 완료 전이며 수용 판정은 유지 보류한다.
C15 동결 출력은 추가 보정 전 대조군이고 이후 source head의 통과 증거로 재사용하지 않는다.

## 원 head 추가 보정 접수 — 저장 0→0 경계

2026-10-13 최신 원 head는 `fd2001f801d45f191e90c99b41c14c5f7a6828dc`다.
추가 코드 `0012228b0eb9548836c96bc3dde06a1ae4691d80`을
`789febd75`로 충돌 없이 적용했다. 검토 이력은9개 원 PR·20개 고유 source commit이며,
기여자가 이미 닫은 #7669의 이력과 기존 보정도 유지한다. 문서·PNG 추가와 이미 반영된
devel merge는 코드로 중복 체리픽하지 않았다.

`stored_zero_restart_is_confirmed`를 호출하는 분할 후보와 재시작 높이 소비 위치에 같은
profile 인자를 전달한다. 실제 인자는 `hwp5_stored_pagination_layout()`이며
`native_hwp5_layout || hwp5_origin_hwpx`다. 순수 HWPX는 제외되지만 HWP5 출처 마커를
보존한 HWPX는 적용되므로 이 경로도 확인해야 한다.

실제 원본1342000의 구역1·문단3·두 번째 표(47×8) 셀35/38에서 저장vpos `0,0,1300`,
높이1000·줄간격300을 확인했다. raw HWP 레코드도 파서 dump와 대조했으며 뒤 줄의
tag는 `0x60000`이다. 독립415쪽 Print의52쪽에는 첫 서울 조례 제목,53쪽에는 뒤 `제10조`
본문이 있다. 같은 원본의 다른 세 셀에서도 동일 저장 사다리가 관측되어 추가 경계로 선정했다.
본문 검색만으로 새 구현의 배치·소유나 전쪽90%를 충족으로 판정하지 않는다.

원본 저장값·비트·독립 Print 본문과 실제 적용/비적용 경로는 ignored
`c21-pr7701-original-zero-ladder-observations.json`, `c21-pr7701-original-zero-ladder-raw-tags.json`,
`c21-pr7701-independent-print-p052-p053-text.json`, `c21-pr7701-incoming-review-plan.json`에 있다.
새 통합 head의 경계·쪽 소속·누락/중복·후속 내용, Native/fresh WASM 전쪽 TSV 및 직접 판독은
아직 필요하다. 기존 #7743 이관과415쪽 신규 검사 철회는 유지하며 수용 판정은 보류한다.
