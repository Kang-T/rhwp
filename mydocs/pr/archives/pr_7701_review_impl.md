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

## C22 통합 head 경계 실행 — 2026-10-13

실제 source `c9f0f521cbd2bdc9391e3e5931068a3315e372b5`에서 고정
`target/pr-review`로 Native를 빌드하고, 같은 원본·독립 Print·compat2022·full font·96dpi의
49–53·62–63쪽을 산출했다. 7쪽 모두 산출됐으나 50·51·53·62·63쪽은 각각
55.33040·51.81060·67.39812·47.27359·44.11633%로 미달이다. 49·52쪽은
91.93033·92.20262%다. 전체 쪽수도 Native417/Print415로 불일치하여
`re_review_required`이며, 종료1을 실행 실패나 통과로 바꾸지 않는다.

전체 `dump-pages` 실행exit0에서 구역1은64쪽으로 독립 Print64쪽과 맞고,
구역5는49/48, 구역16은8/7로 각각1쪽이 남는다. 과거ab5112의418쪽보다1쪽 줄었으나, 새0→0 수정 전 동결C20(e2a525d)도 실제
재출력에서417쪽이었다. 쪽수 감소를001222의 효과로 귀속하지 않는다.
전체 개선 완료가 아니다. 원본 입력 SHA는
`623b00d56beffc45d27c5bf23911bdc49d3a541ded8aecbb323d0716a2bc9f4e`,
독립 PDF SHA는 `bc16ee92640644307fd1ce92ab8ffe663088210caa7740bef620daaa0187e197`다.

실제 raster로 만든50·53쪽 compare/overlay/review를 직접 열었다. 50쪽은 표 상단
내용의 시작과 행 높이가 다르고,53쪽은 앞쪽 행 경계·후속 본문 위치가 다르다.
쪽 소속의 부분 개선과 기하 배치의 일치를 구분한다. Native 경계 실패 뒤 중단된
자동 orchestration은 원 결과를 보존하고 같은 head의 fresh WASM 진단 빌드만 이어갔다.
큰 전쪽 검증과 회귀 기대값 변경은 이 경계의 차이 확인 뒤 진행한다.

원 명령·TSV·manifest는 ignored `c22-native-pr7701-zero-boundary.log`와
`c22-native-pr7701-zero-boundary/pr7701-zero-boundary/`에, 실제 구역별 수는
`c22-pr7701-section-page-localization.json`에 있다. 대표 직접 판독 이미지는 같은
경계 폴더의 `direct-review/review_050.png`와 `review_053.png`다.
Native/fresh WASM 전쪽·직접 확대 판독·관련 반례를 통과하기 전에는 수용하지 않는다.

### 새 0→0 수정 전후 직접 비교와 별도 실제 회귀

동결 C20 `e2a525d36f2923918185f62610869e7c3068b9d0`의 같은 원본·Print·글꼴로
50·52·53쪽을 새로 산출했다. 이 동결 source는001222 전의 대조군이며 upstream base가 아니다.
50쪽은55.33040%로 현재와 PNG까지 동일하고,52쪽은91.68712→92.20262%,
53쪽은55.41235→67.39812%다. 두 source 모두417/415쪽이다.53쪽의 부분 개선을
전쪽90% 통과나 쪽수 감소로 보고하지 않는다.

최신 원 PR 본문은 외곽 위 여백 변경 `1f43a0422` 때문에148·149쪽 행27이 분할되어
기존94.9·94.2%에서83.3·75.5%로 하락했다고 작성자가 명시한다. 현재 통합 source의
실제 Native148·149쪽도83.07534·75.81202%이며, dump-pages에서148쪽 `17..28`,
끝컷 `[1,1,1,2,2,1]`과149쪽 `27..40`, 같은 시작컷을 확인했다.148쪽 review를
직접 열어 마지막「사이버 중독」내용의 앞/뒤 조각 소유와 표 하단 차이를 확인했다.
이는 기존 피델리티 이슈 등록만으로 이관·승인할 수 있는 범위가 아니다.

같은 source의 `RHWP_TABLE_DRIFT=1 RHWP_DIAG_SCAN=1 dump-pages ... --compat 2022 --json`
진단은exit0이다. 구역5 문단11에서 이어받기17행의 host_before1.9, header57.1,
행 가용580.4와 끝 행 부분 컷의 소비565.3/limit37.3이 기록된다. 계산값을 기대값으로
재사용하지 않으며, 독립 Print의 완전한27행을 수용하도록 요구/예약/paint 경로를 검토한다.
수정 전 동결 비교는 `c22-pr7701-before-current-zero-boundary-comparison.json`,
별도 회귀는 `c22-native-pr7701-row27-regression.log`와 같은 경계 폴더의
`direct-review/review_148.png`·`review_149.png`, 예약 진단은
`c22-pr7701-section5-budget.log`에 보존했다. 기대값·baseline은 변경하지 않았다.

### 같은 C22 fresh WASM 경계 완료

고정 `target/pr-review`·root wrapper의 optimized fresh web 빌드는18분40초 후exit0이다.
원본 source는c9f0f521이며 `pkg`와Studio public의JS/WASM SHA-256 일치를 확인했다.
같은 원본·Print·compat2022·96dpi·Native의 실제 full font CSS를 공급한49–53·62–63쪽
fresh WASM TSV가7쪽 누락 없이 완료됐다. 두 backend의7개 지표는 모두 같으며
전체417/415쪽 불일치와50·51·53·62·63쪽 미달도 동일하다. 완료된 음성 검증의exit1을
통과로 취급하지 않는다. 전415쪽 범위는 아직 통과하지 못했다.

첫 진단 시도는 CSS fallback 목록의 모든 대체 face를 필수 공급 face로 잘못 검사해
raster 전에 중단됐다. 실제 우선 face의 full program과 기존 순서의 fallback 목록을
보존하도록 ignored 진단 wrapper를 수정하고 새r2 경로에 재실행했다. source·출력 좌표·
metric·90% 기준은 변경하지 않았다. 첫 실패 경로도 남겼다. primary face/cmap 검사,
Native CSS 원본과hash, 실제 WASM raw SVG와render tree, wrapper SHA는r2 기록에 있다.

최종 경계 로그는 `c22-wasm-pr7701-zero-boundary-r2.log`, TSV/manifest는
`c22-wasm-pr7701-zero-boundary-r2/pr7701-zero-boundary/`, Native/WASM 비교는
`c22-native-wasm-zero-boundary-comparison.json`이다. 대표 fresh WASM 직접 판독 이미지는
같은 경계의 `direct-review/review_052.png`·`review_053.png`다.

### 소비 높이와 실제 앞 조각의 경계 추적

현재147쪽 NativeRenderTree에서 문단11·표0의상자는y77.5/h620.5이며,
걸침셀(8,1)의상자는y134.5/h563.2다. 앞 조각의실제 상자와 scanner가소비한
565.8을동일한값으로취급하지 않는다. 같은Print147쪽raster의긴괘선마지막은
716번째행이고,Native표상자/셀상자의끝은약698이다. raster 줄번호는진단관측값이며
회귀기대px핀을만들지않았다.

생산·소비경로는 `FragmentStart`의 `straddle_page_band_relief` → `RowScanQuery`의
요구/수용과 `emit_fragment`의 `partial_height`/`straddle_page_bands_after_fragment`
→ `PageItem::PartialTable` → `layout_partial_table`이다. 실제paint는MeasuredTable을
바탕으로행높이를 다시 구성하며,온전한 plain행은 `whole_fragment_row_uses_measured_height`
분기에서cut높이재계산을건너뛴다. 새 `straddle_row_relief`는 전달되지만같은helper를
썼다는사실만으로이후의행상자/쪽끝밴드가공유됐다고판정하지않는다.

자료는 `c22-native-pr7701-physical-ledger/`, `c22-pr7701-section5-budget.log`,
독립 `c22-independent-print-p147.png`이다. Native앞쪽상자의차이와정본앞쪽빈밴드를
확인하고,소비공간과내용컷을분리한공통물리결과를검토한다. 원인이정확히확정되기전에는
장식상자만늘리거나배치값을clamp하여수용한것으로보고하지않는다.

### C24 저장 첫 줄 점유 보정 후보 — Native 선행 검증

2026-10-13 검증 후보는 `b78eb69604067ae7cff7600e48af03ad982e0592` 위의
미커밋 Rust 변경이다. HEAD만으로 후보 코드를 식별하지 않는다.
`c24-cell-leading-native-candidate/provenance.json`에 네 소스 파일 SHA-256과
패치 SHA-256, 실제 실행 파일 SHA-256을 고정했다. 아직 최종 승인 대상이 아니다.

원본 구역5 문단11의 셀 `(11,8)`은 첫 줄 원점200HU와 다음 문단 원점1700HU,
줄높이1000HU 및 위·아래 패딩141HU를 저장한다. 독립 저장 끝점은2982HU인데,
기존 측정은첫 원점200HU를 제외한2782HU였다. 기존 컷은 그 선행 공간을 이미
소비하고 있어 측정과 컷이 다른 공간을 예약했다. 후보는 기존 bounded stored lead를
측정과 컷에서 공통 helper로 소비한다. 배치 원점·정렬에서 별도로 소비하는 lead는
다시 더하지 않는다. 저장 줄이 없는 재조판은 기존 문단 앞 간격 계약을 유지한다.

Native Clippy와 Native/Skia CLI 빌드는 exit0이다. 같은 원본·독립 Print·compat2022·
96dpi·full font 공급으로147–149쪽을 재출력했다.

| 페이지 | 이전 C22 Native | C24 Native | 직접 판독 |
| --- | ---: | ---: | --- |
| 147 | 별도 앞 조각 진단 | 94.82317% | 행11의 저장 선행 점유는 반영됐으나 쪽 끝 표 외곽이 Print보다 짧다. |
| 148 | 83.07534% | 88.71359% | 전체 행 경계로 이어지지만 첫 본문 행 이후의 높이 차이가 남는다. 90% 미달이다. |
| 149 | 75.81202% | 96.75070% | Print의 마지막 ‘재해·재난의 원인’ 행이 해당 쪽에 없다. 높은 점수로 소속 차이를 면제하지 않는다. |

후보의 전 문서 쪽수는 여전히417/415다. `dump-pages`는148쪽에서17–28행을
내용 컷 없이 소비하고,149쪽은28–40행,150쪽은40행부터 소비한다.
원본40행 셀 `(40,8)`의 저장 줄 원점200→0은 실제 원본의 이어받기 경계이며,
단순히 문서 전체에서 텍스트가 존재한다는 검사로 해당 쪽의 보존을 대신하지 않는다.
앞 조각의 빈 물리 공간, 일반 수용 예산의 pagination tolerance, 다음 조각의
걸침 셀 잔여 높이와 실제 paint 상자를 계속 대조한다. 허용치·기대값은 변경하지 않았다.

Native 대조군은 그림 셀4쪽 최저99.85950%, 그림 띠2쪽92.75255%,
중첩 표 정상 입력 두 문서 각각8쪽96.81300%, K-water27쪽94.12523%로,
각 문서의 전체 쪽수도 일치한다. K-water의27개 Native PNG와 TSV는 C22와 모두 같다.
이 결과는 Native 선행 증거이며 fresh WASM 및 직접 판독을 자동 대체하지 않는다.

증거는 ignored `output/pr-review/planet6897-followup-20261011/` 아래
`c24-native-leading-summary.json`, `c24-original-cell-line-probe.log`,
`c24-pr7701-dump-pages.json`, `c24-pr7701-budget-diagnostic.log`에 있다.
실제 원본 줄·스타일을 읽은 sidecar probe는 진단이며 정식 회귀 검사로 등록하지 않았다.
대표 이미지는 `c24-native-pr7701-leading-147-149/pr7701-leading-147-149/`
아래 `direct-review/review/review_147.png`, `review_148.png`, `review_149.png`다.

후속 fresh WASM 빌드는 root wrapper, 공용 `/home/tsjang/rhwp/target/pr-review`,
`CARGO_BUILD_JOBS=6`으로17분46초 후 exit0을 확인했다.
`c24-live-build-jobs6-audit.json`은 실제 Cargo 프로세스 환경을 확인한 기록이다.
동결 package와 루트 `pkg`·Studio public의 JS/WASM SHA-256도 일치한다.

fresh WASM은 교육과정 경계3쪽과 Native 대조군49쪽을 모두 산출했다. 교육과정3쪽과
대조군49쪽의 페이지별 지표·PNG는 Native와 동일하다. 이는 해당 후보의 측정 경로
일치 증거이며 전415쪽 승인 또는 직접 판독 완료를 뜻하지 않는다.

K-water의 첫 진단 시도는 서로 다른 페이지의 Native 폰트 CSS를 하나로 합치려다
descriptor 충돌 검사에서 중단됐다. 각 페이지에 해당 Native SVG의 실제 full-font CSS만
그대로 적용하는 r2 adapter로27쪽 전체를 재실행했다. 모델·좌표·폰트 프로그램·90% 기준은
변경하지 않았다. 최초 실패 로그와 원 adapter를 보존했다. 최종 결과는
`c24-wasm-k-water-middle-dot-control-r2.log` 및 같은 r2 경로의 TSV/manifest에 있다.
전체 paired 완료 기록은 `c24-wasm-leading-diagnostic-results.json`,
`c24-wasm-leading-diagnostic-completion.json`이다.

최종 source의 전체 회귀·WASM/workspace/Skia lint·전체 시각 검증은 아직 남아 있다.
148쪽 미달, 쪽수와149쪽 내용 소속 문제 때문에 PR 생성·승인·통합은 계속 보류한다.

### C25 쪽 끝 빈 밴드의 물리 공간 보정 — Native 선행 검증

원본 저장 셀의 선언 높이는 앞 조각이 차지한 빈 공간도 포함한다.
`rowbreak_fragment_frame_bottom` → fragment budget의 실제 본문·각주 경계 →
`page_end_band` → 이어받기 행 높이 감산 → `PageItem::PartialTable`의 소비 경로를
대조했다. 일반 여러 행 표에 단일 셀용100HU 안쪽 경계를 적용하던 가정과
이미 예약한 아래 바깥여백을 다시 차감하던 처리를 바로잡았다. 내용 컷·허용치·
회귀 기대값은 이 단계에서 변경하지 않았다.

동결 Native 후보와 같은 원본·Print로147–149쪽을 산출했다.
148쪽은88.71359%에서97.47476%로 개선됐으며147쪽94.82317%,149쪽96.75070%는
이전과 같다. 전체 출력은416쪽으로 줄었지만 Print415쪽과 불일치한다.
직접 판독에서도147쪽 끝 외곽과149쪽 마지막 ‘재해·재난의 원인’ 행의 쪽 소속 차이가
남는다. 이 단계는 PR 승인 근거가 아니다.

대조군49쪽은 모두90% 이상이며 쪽수도 일치한다. 그림 셀4쪽의 최저값은99.27611%,
그림 띠2쪽92.75255%, 정상 중첩 입력 두 문서 각각8쪽96.81300%,
K-water27쪽94.12523%다. 기존 반례17544911의3쪽과1376496의5쪽은
이전과 지표·PNG가 동일하며 기존 미달은 해결하지 않았다.

Native Clippy·빌드는 각각exit0, jobs6·공용target을 사용했다.
`emit.rs` 영문 주석 번역 전후의 비주석 줄은 동일하다. 번역 때문에 소스 해시가 달라진
경우의 캡처 재개 기록과 동결 바이너리 provenance를 분리해 보존했다.
증거는 ignored output의 `c25-native-page-band-summary.json`,
`c25-emit-korean-comments-audit.json`, `c25-pr7701-budget-diagnostic.log` 및
`c25-native-pr7701-leading-147-149/pr7701-leading-147-149/direct-review/review/`다.
최종 fresh WASM·전체 회귀·lint 및 전쪽 검증은 보정 완료 뒤 수행한다.

### C26 병합 행의 저장 쪽 경계에서 끝 간격을 공통 소비

`stored_page_reset_cut_trailing_trim`의 병합 셀 주변 행 일괄 제외를 제거했다.
기존 주석과 달리 `row_block_content_height`와 `cell_cut_visible_height`도
`native_saved_reset_cut_trailing_trim`을 소비하고, per-row paint는 같은
`row_cut_content_height`와 수용한 끝 행 높이를 사용한다. 중첩 표의 국소 되감김,
편집·재조판 및 저장 정보 유효성 제외는 유지했다. 문서별 수치 분기는 추가하지 않았다.

원본40행 셀의 원점200→0HU, 줄높이1000HU, 줄간격300HU와 패딩141HU×2를
직접 읽었다. 첫 프레임의 내용16px와 패딩3.76px는 C25의 내용 예산16.827px 안에
들어간다. 실제 출력에서149쪽의 ‘재해·재난의 원인’과 첫 줄,150쪽의 남은 ‘기’를
확인했으며40행 제목은150쪽에 중복 출력되지 않는다.149쪽99.33573%,
150쪽93.20865%이고147쪽94.82317%,148쪽97.47476%다.

Native 대조군49쪽은 C25와 PNG가 모두 동일하다. 기존 반례17544911은3쪽 모두
동일하며1376496은4쪽42.88337%→43.82035%,3·5쪽은 동일하다.
Native Clippy·빌드는 jobs6·공용target에서 각각exit0이다.
증거는 `c26-native-stored-reset-summary.json`, `c26-original-row40-output-ownership.json`,
원본147–150쪽 `direct-review/review/`와 `c26-page-band-native-candidate/provenance.json`이다.

전체는 여전히416/415쪽이다. 원본 구역5의 중학교 표(문단11)가146–156쪽으로
이어지지만 Print의 고등학교 표는156쪽에서 시작한다. 마지막 구간166–168쪽은
72.05152%,64.35064%,23.43152%로 미달한다. 쪽수·후반부 소속 차이와147쪽 물리
상자 차이는 계속 보정해야 한다. 정식 회귀 추가·기대값 변경·PR 승인은 하지 않았다.
