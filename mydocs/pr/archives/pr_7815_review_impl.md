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
