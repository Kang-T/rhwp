---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7622_review.md
last_verified: 2026-10-10
---

# PR #7622 리뷰 — 굽은 따옴표의 영문 글꼴 슬롯

## 최종 판정

**머지 보류.** 공개 대조군의 충족, 실행으로 확인한 시각 미충족, 코드 검토상 우려와 필수 증거 부족을 아래에 구별한다. 사용자가 승인한 conflict·CI 보정은 진행했으나 90% 시각 gate는 면제되지 않는다. 통합 후보의 로컬 회귀 PASS를 원 contributor head의 독립 검증 완료나 merge 승인으로 보고하지 않는다.

## 접수·적용 범위

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#7622](https://github.com/edwardkim/rhwp/pull/7622) / planet6897 / devel |
| 원 head | `cca0d1b671d62b9111774f53c6948d85c0fb5d4d` |
| 최초 적용 commit | `9229d63b3868bc2b44a839e240c7ee2e5c3a25af` |
| 이 PR의 적용 뒤 head | `f6b95026683f1762231cc0b2db94c906c195d3a2`; 기능 commit 3개, 원 저자·cherry-pick -x 보존 |
| 누적 검증 source head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563` |
| 최신 base 정렬 뒤 code head | `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`; 제품/검증 입력 tree 동일 확인 |
| 검토 branch·reviewer | `review/planet6897-20261008` / jangster77; 원 PR reviewer assign 선행 |
| 원 PR 참고 상태 | draft=False, MERGEABLE / CLEAN; 작성 시점 snapshot이며 merge 직전 판정을 대신하지 않음 |
| 경로 | collaborator 외부 PR의 주 작업공간 누적 cherry-pick 9.1.1; intake/template/local_validation/visual_fixture_evidence 및 대형 PR 보조 절차 |

관련 이슈와 해결 표현은 원 PR 본문의 범위를 유지한다. 이번 보류 기록에 새로운 closes 표현을 추가하지 않는다. 원 PR 제공 before/after와 corpus 수치는 참고이며 통합 head의 독립 실행과 구별한다.

## 구현 주장·소비 경로·검증 판정

`style_resolver.rs:824`의 `detect_lang_category`는 U+2018/2019/201C/201D만 영문 슬롯으로 분류한다. 독립 공개 논문 PDF p3의 ‘/’ face는 TimesNewRomanPSMT, p4의 U+2024는 Haansoft Batang임을 PDFium으로 조회하여 punctuation 블록 전체를 바꾸지 않는 근거와 대조했다. 동일 원본 7쪽 Native/fresh WASM 최저 94.23341%, 대표 p3의 2단·줄바꿈과 인용부호를 직접 비교했다. 글꼴 굵기·AA 및 인용 참조 모양 차이는 남는다.

**실행한 진단의 제한:** legacy 인라인 표 경로 `layout/paragraph_layout.rs:3616`은 `cs_changed`에 언어 변경을 포함하지 않고 누적 run을 첫 문자의 `first_lang`으로 칠한다. 공개 TAC 입력에서 한글과 ‘/’를 섞고 모든 영문 face를 Times New Roman으로 지정한 합성 진단에서 따옴표 SVG는 GulimChe 체인이었다. 이는 현재 소비 경로의 미해결 결과이며 새로 발생한 회귀나 정상 한컴 저장본과의 차이로 승격하지 않는다. 진단 입력의 저장 LineSeg를 실제 생성본의 증거로 사용하지 않는다.

**미검증:** 이 소비 경로의 독립 편집 Print PDF·수정 전 FAIL/수정 후 PASS, 원 PR의 전체 175문서 영향 범위·쪽수 차이. 새 회귀 fixture/golden을 추가하거나 기존 기준값을 통과 목적으로 완화하지 않았다.

공통 빌드·회귀·글꼴 등록·383쪽 시각 결과와 입력/기준 해시·증적 보존·merge 후 댓글 계획은 [#7611 공통 통합 검증](pr_7611_review.md#공통-통합-검증과-증거-보존)에 연결한다. 이 PR의 조건부 수용이 아니라 미충족·미검증 해소 후 재검토 계획이다.

![native quotes-retry p3 비교](../assets/planet6897_20261008/native_quotes-retry_p003_review.png)

![wasm quotes-retry p3 비교](../assets/planet6897_20261008/wasm_quotes-retry_p003_review.png)

## 한컴 저장본·Print로 확인한 legacy 인라인 소비 오류 — 2026-10-10

앞선 수동 LineSeg 진단을 그대로 정상 저장본 증거로 사용하지 않았다. 같은 공개 원본의 명시적 진단 derivative를 Windows 한컴에서 열고, 실제 본문 선택에 인용부호가 포함된464문자를 확인한 뒤 Latin face를 Arial→Times New Roman으로 두 번 바꾸고 매번 parameter readback을 확인하여 HWPX로 저장했다. [공식 Automation action 목록](https://raw.githubusercontent.com/hancom-io/devcenter-archive/main/hwp-automation/ActionObject.pdf)의 `SelectAll`과 HAction 경로를 사용했다. 초기 선택 호출 실패와 PowerShell5의 UTF-8 regex 해석 실패는 수정 전 제품 FAIL로 세지 않으며 별도 실행 실패로 보존한다. `quotes-native-save-v3.json`에 실제 edit/readback·입력 hash를 기록했다.

저장 제품은 hancom-office-2020이며 대응 Print는 명시적 engine2020, 한컴11.0.0.9136, `pdf_print_method:0`, `hancom2020_pdf_driver_one_up`으로 산출했다. PDF signature/EOF·byte 수·다운로드 hash를 확인했고 전체1쪽이다(`quotes-native-print-v3.json`). 실제 source의 `가‘나’` host는 Latin face Times New Roman과 새 저장 LineSeg를 가지며 독립 PDF의 두 인용부호는 TimesNewRomanPSMT다(`quotes-native-source-print-check.json`). 원래 공개 입력의 개선·일치나 source 생성 출처의 동일성을 이 derivative의 결과로 바꾸어 주장하지 않는다.

source `711f7f92ee63279754d80e1be9529ae24905f5ad`의 실제 새 Native와 fresh WASM에서 이1쪽은 각각98.93360%지만, Native SVG의 두 부호는 굴림체/GulimChe 체인으로 나왔다. 이미 확보한 Print와 실제 family를 대조해 legacy TAC run의 측정/paint 슬롯 불일치를 실행으로 확인했다. 사용한 WASM 조판 generation은2022이며 Print service engine2020과 구별한다. 잘못 지정한 compat2020은 exporter가 거부했고 output 없는 orchestration 오류로 보존·교정했다. source/입력/PDF/package hash·전체1쪽 TSV·원시 WASM 출력은 `quotes-native-v3-before-711f7f92e/` 및 `quotes-wasm-v3-compat2022-before-711f7f92e/`에 있다. 90% 점수만으로 잘못된 family를 면제하지 않는다.

남은 보완은 언어 슬롯으로 측정한 문자들이 같은 char-shape ID라는 이유로 첫 문자의 family로 함께 paint되지 않게 하는 것이다. 같은 face·장평·자간을 공유하는 슬롯은 기존 run을 유지하며 실제 글꼴/메트릭이 달라지는 경계를 분리해야 한다. 정상 저장본의 before/after·정식 의미 검사·최신 원본 대조와 필수 검증이 아직 미완료이므로 머지 보류를 유지한다.

### 실제 측정 style과 run 소유의 보완 후보

legacy TAC 텍스트 경로에서 `ParagraphMetricScope::style`이 문자별로 생산한 family/크기/장평/자간과 현재 누적 run 첫 문자의 같은 생산 결과를 비교한다. 값이 달라지면 기존 flush와 마지막 run paint가 같은 시작 문자·style을 소유하도록 run을 끊는다. Unicode category만 다른 경우나 char-shape ID만 같은 경우를 글꼴 동일성으로 대신하지 않는다. `with_stored_tabs`의 소비 위치별 tab extension 변화는 font/메트릭 차이와 구별하며, 기존 char-shape·supplemental metric scope·저장 줄 바꿈·제어문자·실제 빈 run의 경계는 유지한다.

실제 연결은 문자 폭의 `metric_scope.style(styles, cs_id, lang, ch_idx)` → 축적 run 경계의 같은 style 비교 → 중간 flush/마지막 flush의 `metric_scope.style(..., first_lang, line_run_start)` → TextRun backend 소비다. 첫 문자의 속성을 모든 뒤 문자에 적용하던 가정을 제거하고 특정 인용부호·입력 ID 예외를 추가하지 않는다. 같은 글꼴/크기/장평/자간을 공유하는 다른 슬롯은 기존 run을 유지한다. 후보 작성 시점에는 새 Native/fresh WASM·직접 판독·정식 before FAIL/after PASS·원본 대조·회귀/lint가 미검증이었다. 후속 실제 실행 결과는 다음 절에 구별한다. 원래 source의 전쪽 및 실제 backend 검증 없이 이 후보를 승인·통합하지 않는다.

### 2026-10-10 실제 저장 대조군의 보완 실행

제품 source `5538efd3832ab2137e86de5302b4dee7caf392b7`, 고정 base
`21ed64071f81f35f37618717693bd30f727392c9`의 새 Native·fresh WASM을 실행했다.
Native 빌드366.8초, WASM 빌드59.19초이며 모두 exit0이다. Native 실행 파일 SHA-256은
`27b18b1744264305f21617d90e63b0d9e1e2882bcb62a3bcf4c022c281323063`이다.
같은 실제 한컴 저장 대조군·대응 2020 1-up Print 전1쪽에서 두 경로 모두99.01152%다.
저장 글자모양의 Latin face, 독립 Print의 TimesNewRomanPSMT와 실제 두 인용부호의
Times New Roman paint 요청이 일치했다. 앞서711의 실제 Native/fresh WASM은 둘 다 굴림체였고,
같은 독립 기대값 검사에서 before 두 실행 exit1 / after 두 실행 exit0을 확인했다.

입력과 독립 PDF는 [대조군 생성·해시 기록](../../../samples/issue7622/README.md)에 연결하고
커밋 `bea45e807be47d460899a6c62f55e200a0fcd423`에 포함했다.
대표 [Native review](../assets/issue7622_inline_table_font_slots/curly-quote-native-review.png)·
[overlay](../assets/issue7622_inline_table_font_slots/curly-quote-native-overlay.png),
[fresh WASM review](../assets/issue7622_inline_table_font_slots/curly-quote-wasm-review.png)·
[overlay](../assets/issue7622_inline_table_font_slots/curly-quote-wasm-overlay.png)를 직접 읽었다.
표 외곽·뒤 본문·인용부호를 확인했으며 작은 획 농도·AA·괘선 두께 차이는 남는다.
이 대조군은 공개 원본의 한컴 편집 derivative이며 원 contributor 원본 전체의 일치 증거로 바꾸어 보고하지 않는다.

![Native 실제 저장 대조군](../assets/issue7622_inline_table_font_slots/curly-quote-native-review.png)
![fresh WASM 실제 저장 대조군](../assets/issue7622_inline_table_font_slots/curly-quote-wasm-review.png)

정식 `issue_7622_inline_table_font_slots`의2개 검사는 실제 최종 render tree에서 Latin face와
한글 face, 순서·각 문자1개 보존을 확인한다. 소스의 같은 char-shape ID와 정상 저장 LineSeg를
확인하며 절대 출력 좌표·SVG hash를 기대값으로 쓰지 않는다. source 보강 후 파생 suite를 다시
준비한 실행에서2 PASS/208 skipped, 실행0.086초, build45.32초, exit0이었다.
suite를 재준비하기 전0건 실행 exit4는 orchestration 오류이며 통과·제품 회귀로 집계하지 않는다.
현재 정확한 Rust harness의 수정 전 실행은 미검증이다. 위 실제 SVG producer/paint 요청의
before/after 실행과 정식 Rust harness 실행을 구별한다.

같은 대조군의 수정 전후 layout-anomaly는 off-canvas·overflow·text-overlap 모두0이다.
새 sample의 쪽수1/1 행만 추가했으며 기존 baseline을 완화하지 않았다. 이번 branch의 추가·교체
HWP/HWPX11개를 같은 frozen Native의3개 보안 CLI로 실제 검사하여33건 모두 clean이었다.
이 진단은 정식 보안 suite나 다섯 코퍼스 래칫을 대신하지 않는다. `bea45e807` 전체 nextest는
build-jobs8/test-threads10으로 시작했으며, 최종 요약·세 Clippy·다른 원본과 소비 backend는
아직 미검증이다. 전체 통합 후보와 이 원 PR의 머지 보류는 유지한다.

실행 로그·TSV·원시 출력·package/source hash·인용부호 before/after는 ignored
`output/pr-review/planet6897-20261008/`의 `candidate-5538efd38-{native,wasm}-check.json`,
`quotes-native-v3-after-5538efd38/`, `quotes-wasm-v3-compat2022-after-5538efd38/`,
`quotes-native-slot-runtime-control.json`, `quotes-formal-strengthened-result.json`,
`new-sample-detectors-5538efd38.json`, `bea45e807-full-regression.json`에 보존했다.

## 2026-10-11 전체 영향 자료 확보 감사

기여자 원 TSV의175문서/1,566영향쪽을 실제 읽고 각 원본과 repository 기준 PDF의 존재·SHA 및 PDF 전체 쪽수를 확인했다. 입력 누락0, PDF 누락0, PDF 읽기 실패0이며 대응 PDF 쪽수 합은6,078쪽이다. ignored `batch208-font-impact-inventory/provenance.json`에 전 입력을 기록했다. 이 감사는 현재 base/head의 시각·원문 대응 검증이나 이전 낮은 점수의 #7445 이관 근거가 아니다.

자산관리규정 `samples/issue6031/3249937_asset_management_rules.hwpx`도 이 목록에 있다. 기준 `pdf/3249937_asset_management_rules-2020.pdf`는 전체60쪽이고 기여자의 영향쪽은37/38/39다. b886 frozen Native의37쪽을 실제 재출력하여99.35586%를 측정했다. 다음38쪽 브라우저 raster는 `ENOSPC`로 실패했으므로39쪽·전체60쪽·fresh WASM과 직접 판독은 미검증이다. 원본·Print·검사를 삭제하거나 이 문서를 이관하지 않는다. 실패한 실행은 `batch209-asset-boundary-diagnosis/`에 보존했다.
