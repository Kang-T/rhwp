---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7828 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **합성 LineSeg의 fit 높이**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `a2ff0709a00282c0bd60dd36ca336f274de57eb5`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `1255cab11fe74afb2779101219d36a7cdb8de199` | `92cb16e3a28d63efa5ebae17e65018d934b895b8` |
| `90a608beb06aa41544a53beaeefeee5a32fddb54` | `322e37faf2f4bcb6df37607c38bf9428eca88b2c` |
| `a2ff0709a00282c0bd60dd36ca336f274de57eb5` | `26b024dc6be46f830661ede1c68238f218236c33` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

줄 합성 → height_for_fit·before/after spacing·drift budget → 문단 분할·배치를 추적한다. 실제 저장본과 lineseg 없는 합성 입력을 구분한다.

## 독립 기대값과 검증 계획

합성 원본/Print 전2쪽의 줄 소유·내용 완전 보존·뒤 본문을 확인한다. odd_spacing_prev와 HWP5-nopassword/HWP3 변환본의 영향을 비교하며 source의 87/84%로 완료를 선언하지 않는다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.
