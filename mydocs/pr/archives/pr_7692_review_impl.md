---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7692 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **병합 칸 높이와 줄 상자**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `135e2b075a8bee4aec792b003e9fa89618c32b5c`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `fa6cc7eab488a5b800f97b4df7c3df0f094616c9` | `62888424d8a2663af40dabea4ba3c83181017243` |
| `6b11130813d4c9e9ee8cdd4f5e9dc2e215a788d5` | `fc0ede395feedcb45b4834077f14e9aedc62ae6c` |
| `135e2b075a8bee4aec792b003e9fa89618c32b5c` | `338856f43010d0e2d0f08d67b700397949e5e927` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

선언 높이 constraint 생산 → 공용 rowspan solver → 측정 row heights와 실제 layout row heights 소비를 확인한다. 양쪽 constraint에서 내부 행 간격을 같은 방식으로 제외해 #7832의 행 간격 계약을 보존했다.

## 독립 기대값과 검증 계획

내부 행 간격이 있는 span, 이미 채운 행의 초과 높이와 빈 행, 일반 그림 칸·강제 줄바꿈을 대조한다. 80168 HWP/HWPX의 표 뒤 문단 악화와 1342000 전쪽을 확인한다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.
