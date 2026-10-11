---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7825 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **문단의 글자 단위 여백**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `b368487aa2bac4234c1dd1755b1a768485d65212`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `d3cc21e2f1927fdc32cf3e8d4ccea5541c10ff5c` | `542b7622f5f5dc51d6dd2f9c5585d1e513566f6a` |
| `b368487aa2bac4234c1dd1755b1a768485d65212` | `2d6bb006abd7f35f3f3350fca5049840a879c9cb` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

stored unit decode·기준 스타일 크기 → composer/layout의 유효 여백·indent → 실제 줄 구성과 문단 원점을 확인한다. 측정 뒤 별도 원점 덮어쓰기도 대조한다.

## 독립 기대값과 검증 계획

합성 9개와 같은 입력 Print 전체를 비교한다. 들여쓰기 경계·정렬·base char style·위아래 간격을 확인하고 절대 원점 pins는 시각 통과 뒤 독립 관계형 계약으로 보정한다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.
