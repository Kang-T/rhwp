---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7815 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **문장부호 언어 슬롯**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `f2a334bc4e90fbedd021e59f4cf2420cbcfbdb21`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `27fed2286618a993ca961692a119f914f22db8c1` | `800fcc433100f6772680b97da8870685e2cbaa2a` |
| `f2a334bc4e90fbedd021e59f4cf2420cbcfbdb21` | `5895c485bc470c61527ebc5df2447099338fa256` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

슬롯 감지·neutral carry → char advance → run boundary·bbox → Native/Canvas/Skia paint를 추적한다. 저장 탭 wrapper와 글꼴 환경의 명시적 대체/결측 glyph chain 구분을 보존했다.

## 독립 기대값과 검증 계획

curly quote 원본·Print 전쪽과 배치 순서를 확인한다. TAC 각주 flush의 이미 누적한 폭을 다시 inline_x에 더하던 경로를 제거했다. marker 앞뒤 텍스트와 저장 탭을 관계로 검사한다. coverage hash와 baseline 증가는 최종 head에서 재측정한다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.

## 전체 회귀에서 확인한 분류 원장 변경

`ab5112e29a2a23535525e4fe1dc96936defbe4e1` 전체 회귀에서 #4962의 공개 분류 원장
검사가 실패했다. 별도 aggregate 실행으로 3개 원본을 재측정했다. HWP/HWPX format parity
2개는 counts·categories·두 hash가 기존 원장과 같았다. 수출입 현황 원본은 문장부호의
언어 슬롯 경계가 바뀌어 source run 합이 4,573→4,585였고, HFT 원본 메트릭을 사용하는
글리프 대체 경로에서 metricFace 보고도 달라졌다. #7701까지의 source
`f22cdb0359cd0c0c400e1b0348c7f8c63f53be0c`와 비교한 원문 aggregate를 보존했다.

전후 coverage 16,685·layout 16,872·paragraph 2,565 및 모든 category 합·width source별
문자 수는 동일하다. 바뀐 분류 키·run 소유로 canonical aggregate/legacy hash가 달라져
원장의 run 합과 두 hash만 재측정값으로 갱신한다. 배치 좌표·실루엣 허용치·겹침 baseline은
이 작업으로 바꾸지 않는다. 자료는 ignored 검토 output의
`coverage-current-*.json`, `coverage-through-7701-overlay-and-policy-heuristic-hwp.json`,
`coverage-language-slot-run-boundary-differences.json`이다. 강제 재컴파일한 주 작업공간의
production 라이브러리에서 기존 정식 분류 검사 원본을 실행해 PASS했다
(`coverage-formal-test-after-verified.log`). 최종 source의 전체 회귀·lint와
curly 원본의 전쪽 Native/fresh WASM 시각 미달 보정은 여전히 필요하다.

기여자 commit이 갱신한 `tests/golden_svg/issue-147/aift-page3.svg`는 현재 어떤 활성
snapshot 검사에서도 사용하지 않는다. 전쪽 시각 증거 없이 과거의 절대 SVG 배치를 다시
고정하지 않도록 고정 base의 자료를 보존해 해당 갱신을 최종 diff에서 제거했다.


## 잉크 없는 자리차지 표 호스트의 독립 대조

현재 curly 원본은 전체 Native/fresh WASM 82.12343%로 미충족이다. 문장부호 폭과 별도로
원본 pi9의 양수 들여쓰기를 가진 빈 호스트 줄이 사라져 뒤 pi10 제목이 앞당겨진다.
동일 표를 쓴 정상 연속 paraPr의 한컴 Print 대조에서 양수 첫줄 들여쓰기와 명시적 공백은
호스트 줄을 유지했고, 정렬만 변경·내어쓰기·왼쪽 여백만 변경한 대조군은 유지하지 않았다.
글자의 잉크 유무로 줄 공간을 판정하던 가정을 재검토한다. 세로 offset 대조에서 줄 높이가
표 위에 들어가는 경우에는 호스트 줄이 표 뒤 꼬리가 되지 않는다.

공통 쿼리 `reflow_float_host_line::trailing_advance`의 입력은 원본 문단·종단 글꼴·
문단 줄간격·자리차지 밴드 시작이다. 표 `host_spacing`의 측정/fit과 실제 표 배치 후 간격이
같은 결과를 소비한다. 수용된 `ParagraphFloatPlacement`가 있으면 그 `occupied_bottom`이
최종 흐름 원점이므로 측정 after에 포함한 줄을 paint에서 다시 가산하지 않는다.
저장 줄·여러 개체·명시적 개행은 각자의 기존 소유 경로를 유지한다.

자료는 ignored output의 `curly-float-flow-independent-contract.json`,
`curly-float-flow-vertical-probe-print-observations.json`,
`curly-float-flow-native-before-observations.json`이다. 드문 style200으로 만든 최초 진단은
무효 입력이므로 제외했다. 구현 후보의 원본/대조군 재출력·전쪽 90%·확대 판독과
최종 통합 검증은 진행 중이다. 양수 offset의 큰 빈 공간에서 뒤 본문을 표 위에 배치하는
대조군은 기존 출력도 잘못되어 별도 실제 호출 경로를 계속 확인한다.


첫 후보의 전체 Native 원본은 82.12343→98.50387%이고 기존 quote 전진폭 검사는 PASS다.
`curly-host-line-candidate-native/curly_quote_latin_slot/review/`의 원본 전체와
수정 전/후/Print 동일 영역 확대에서 표 뒤 제목·본문의 줄 소속과 간격 복원을 확인했다.
15개 독립 대조군에서도 점수만으로 완료하지 않았다. 공백 대조군의 실제 문단 조각과 표
꼬리의 중복 계상을 직접 찾아 후보를 수정하고 있다. 공백 등 실제 텍스트가 있는 호스트는
기존 `PartialParagraph`가 줄을 소유하며, 새 쿼리는 텍스트가 없는 호스트의 들여쓰기 줄만
표 꼬리에 게시한다. 첫 후보의 공백 결과는 수용 근거에서 제외한다.

Native Clippy 첫 후보는 통과했지만 수정 후보의 다시 빌드·재검사와 fresh WASM은 아직
필요하다. 큰 양수 offset의 앞 빈 공간에 다음 문단을 넣어야 하는 대조군과 음수 offset
대조군의 작은 흐름 차이는 점수가 90%를 넘더라도 정확한 배치로 판정하지 않는다.
최종 통합 검증과 나머지 #7672·#7701 전쪽 미달은 여전히 남아 있다.


중복 소유를 제거한 두 번째 후보는 production 라이브러리 실제 재빌드(6분51초)와
Native Clippy(1분20초)를 통과했다. 16개 한쪽 입력 전체 Native TSV를 다시 산출했으며,
원본 98.50387%가 유지되고 공백 대조군은 기존 문단 조각의 흐름으로 복원됐다.
이 재실행의 원문은 `curly-host-line-candidate-2-native-results.json`이며,
정확한 dirty source 해시는 `curly-host-line-candidate-2/provenance.json`에 고정했다.
15개 대조군의 최저 94.50988%는 큰 양수 offset의 잘못된 뒤 본문 소속을 면제하지 않는다.
공백/음수 offset 대조군도 기존 작은 간격 차이가 남아 독립 Print와 완전 일치라고
기록하지 않는다. fresh WASM·정식 관계형 회귀 추가의 선행 조건은 아직 진행 중이다.


두 번째 후보의 기존 quote 정식 검사도 실제 production 라이브러리에서 다시 실행해
1건 PASS했다(`curly-host-line-formal-test-after-2.log`). 포맷·Native Clippy·WASM lib
Clippy와 고정 base `bfa28a77` 대비 unit tier 검사를 통과했다. 소스 변경 뒤 낡은 파생
harness로 수행한 첫 manifest 검사는 drift로 실패했고, `--prepare`로 파생 suite를 다시
준비한 뒤 manifest와 포맷 check 모두 통과했다. 파생 파일은 PR에 stage하지 않는다.
워크스페이스 build/all-targets Clippy·전체 회귀·Skia·Studio 브라우저와 fresh WASM은
최종 source의 추가 필수 게이트로 남아 있다. 최신 upstream fetch도 동일 base였다.


## 새 production source의 fresh WASM·브라우저와 관계형 회귀

`467bfd1d87a8801632a253c595065d58edb63138`의 루트 기본 release web package를 고정
`target/pr-review`에서 새로 빌드했다(18분01초, exit0). 루트 pkg와 Studio public의
JS/WASM SHA-256이 일치하며 새 package는 ignored `curly-host-line-fresh-wasm-candidate/`에
동결했다. WASM SHA-256은
`f9fd949a405793a9cf667821c264b9f9e4b2448f3d51577a831eb50393f26a02`다.
원본 전체 한쪽 fresh WASM TSV98.50387%가 Native98.50387%와 같았다.
WASM 실행의 font policy·note metadata 조회는 동결한 AB CLI(동일 font 코드)를 사용했고,
실제 WASM 기하는 새467 package에서 산출했다. 최신 Native CLI도467 production으로
별도 빌드·동결했다. 마지막 통합 증적에서는 이 최신 CLI와 새 package를 사용한다.

실제 Chrome·Vite의 HWPX 로드 완료를 확인한 뒤 WASM Canvas2D API로 출력했다.
Canvas 출력1쪽, 실제 잉크17,727픽셀과 독립 Print98.25768%를 확인했고 review에서
표 뒤 제목·후속 본문의 같은 소속과 간격을 직접 판독했다. 최초 진단은 초기 빈 HWP를
성급히 캡처해 실패했으므로 제외했으며 SourceFormat=hwpx 로드 완료를 기다리도록
진단을 고친 재실행은 exit0이다. 로그·해시·Canvas TSV·review는
`curly-host-line-browser-canvas/`와 `curly-host-line-browser-canvas-2.log`에 있다.

이 선행 증거 뒤 `tests/cases/issue_reflow_empty_float_host_line.rs`에 정식 회귀를 추가한다.
뒤 제목의 원본 소유를 찾아 그 앞 표와 제목 줄 사이에 선언 종단 글꼴의 줄 높이가 들어가는지
확인한다. 바깥 아래 여백은 원본에서 가져오고 고정 페이지 좌표·고정 px 영역·SVG 해시는
쓰지 않는다. 실제 수정 전 AB production library에서 gap1.88 < declared em13.333으로
의도한 assertion FAIL, 수정 후 PASS다. 같은 원본의 들여쓰기 없는 표 뒤 독립 빈 문단이
가짜 호스트 줄에 의해 밀리지 않는 정상 대조도 PASS했다. 실제 두 검사 로그는
`curly-host-line-relational-before-two.log`와 `curly-host-line-relational-after-two.log`다.
정식 suite 재준비·통합 전체 실행은 아직 남아 있다. 큰 offset 등 별도 대조의 남은 차이를
이 원본·정상 대조의 통과로 면제하지 않는다.


파생 suite 재준비 뒤 실제 `run-rust-test.mjs`가 새 모듈을
`regression_suite_010::issue_reflow_empty_float_host_line`로 해석했고,
nextest가 **2건 실행·2PASS·205SKIP, exit0**으로 끝났다. 0건 실행을 통과로 세지 않는다.
두 검사 최종 본문은 수정 전 library에서 1FAIL/정상대조1PASS, 수정 후2PASS다.
같은 production source의 Studio 재실행은1,834PASS/0FAIL/2SKIP이고 TypeScript exit0이다.
Node 전용 package가 필요한 기존2개 skip은 web WASM·브라우저 검증으로 대체 실행했다고
기록하지 않는다. 전체 Rust/Skia·나머지 미달 문서·통합 PR/CI/merge는 아직 남아 있다.
