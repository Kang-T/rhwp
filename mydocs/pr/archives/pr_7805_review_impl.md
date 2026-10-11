---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7805 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **장치 격자와 상대 크기 자간**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `b35f6eb56024033ce0ebf4e8488e6affef4a0067`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `b35f6eb56024033ce0ebf4e8488e6affef4a0067` | `af25ec3999a7bbacfef19969fefb5d68c9960b41` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

언어 슬롯의 실제 크기 → char_width_decision 장치 전진폭 → HU 줄 나눔 → 배치·paint를 추적한다. 보충 canvas 메트릭은 적용 여부를 별도로 확인한다.

## 독립 기대값과 검증 계획

검증용 합성 입력 4개와 대응 Print의 전쪽을 먼저 비교한다. 78494의 쪽수·누락 줄, 부동소수 HU 경계, ratio·relative size·space tie를 독립 PDF 간격과 대조한다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.

## 합성 검증 입력의 정상 저장과 검출력 확인

원래 4개 HWP는 한 `BodyText/Section0`의 처음/끝에 구역 정의를 중복 포함했다.
Native는 1쪽, 독립 Print는 2쪽이었다. 실제 한컴 Office 2020으로 정상 저장한 대조군은
두 BodyText 스트림 각각의 첫 문단에 구역 정의를 갖는다. 실패 원본과 양쪽 SHA를
`pdf/issue7702/input-evidence/`에 보존하고 활성 samples를 정상 저장본으로 바꿨다.
이 작업은 검증 입력의 생성 오류를 보정하며 원본 입력의 렌더링 문제 해결로 보고하지 않는다.

정상 저장 후 독립 Print(method0)한 8쪽은 기존 기준 PDF 8쪽과 RGB 픽셀이 전부 동일하다.
따라서 기존 Print와 회귀 기대값은 그대로 사용했다. 문단 수·공백 제외 본문도 보존했다.
source `ab5112`의 Native/fresh WASM 전체 페이지 최저는 99.56697% 이상이다.
fresh WASM의 상대 크기·자간 대표 PNG와 확대본에서 크기·간격·행 순서를 직접 확인했다.
획 raster 차이는 남지만 누락·중복·줄 이동은 관찰하지 않았다. 최종 통합 head에서는 시각
출력과 필수 게이트를 새로 고정한다.

같은 정식 회귀 원본과 정상 저장 입력을 #7805 전의 source
`f22cdb0359cd0c0c400e1b0348c7f8c63f53be0c`에 연결하면 4건 모두 기대한 전진폭·자간 원인으로
FAIL하고, 보정한 production source에서는 4건 모두 PASS했다. 검사를 삭제하거나 허용치를
바꾸지 않았다. 로그는 `pr7805-normalized-formal-tests-before.log`와
`pr7805-normalized-formal-tests.log`이다.

ignored 검토 output의 `pr7805-original-normalized-print-raster-equality.json`,
`pr7805-normalized-native-results.json`, `pr7805-normalized-fresh-wasm-results.json`,
`pr7805-normalized-fixture-review.png`, `pr7805-normalized-fixture-review-enlarged.png`에 연결한다.
