---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7701 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **이어받기 조각의 바깥 위 여백**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
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


## Canvas·Skia 따옴표 추가 보정 — 2026-10-12

production `467bfd1d8`에서 동일 입력 `curly_quote_latin_slot.hwpx`의 교정된 실제 글꼴을
공급해 Native/fresh WASM 전쪽 TSV를 재산출했다. 17개 소형 문서 양쪽 최저는
99.56697%이고, 이 따옴표 문서 Native/fresh WASM은 99.65006%다. 실제 Canvas는
Studio 등록 글꼴 목록에도 같은 프로그램을 등록해야 정책상 HY신명조로 치환되지 않는다.
등록 후 전체 쪽은 99.09501%, Skia Print/명시적 font-path 출력은 98.98308%이나,
4개 probe 문단 확대 판독에서 원 글리프가 다시 좁아지는 차이를 검출했다. 점수만으로
수용하지 않았다.

원인은 `text_measurement.rs`의 `metricTrueTypeGlyph` 생산 → 문자 전진폭 →
`forces_halfwidth_punct_paint` → Canvas/Skia의 0.5배 paint 분기다. 판정이
`embeddedMetric`이면서 1em 이상인 경우만 원 글리프로 인정하여, 검증된 휴먼명조
프로그램과 원래 좁은 Times 글리프 모두 측정 결과와 달리 다시 축소됐다.
판정이 두 원 프로그램 폭 경로를 그대로 소비하도록 보정한다. 저장 IR·문자 원점·
기존 HFT 호환 overlay와 다른 구두점의 적용 조건은 바꾸지 않는다.

before actual 출력과 font program 해시는 ignored
`correct-face-curly-browser-canvas-registered/`, `correct-face-curly-skia-print/`,
`correct-face-curly-registered-owned-lines-enlarged-review.png`에 보존했다.
정식 관계형 검사 후보는 ignored `issue_7702_curly_quote_paint_advance.pending.rs`에
보관하며, 수정 후 Native/fresh WASM 전쪽 TSV 및 실제 Canvas/Skia 확대 판독이
충족되면 `tests/cases/`에 등록한다. before/after 함수 검출은 unchanged predicate의
공개 범위만 넓힌 고정 e72 worktree와 보정 source를 대조할 계획이며 아직 미실행이다.
