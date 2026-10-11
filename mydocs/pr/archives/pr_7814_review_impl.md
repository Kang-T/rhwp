---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7814 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **HWP3 변환 글꼴 이름**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `03b415cde31ed9e7164cf1828f5e4811ef59ce6b`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `03b415cde31ed9e7164cf1828f5e4811ef59ce6b` | `66a62a74a339b8ea4410bc076184a4ccdd453d5f` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

HWP3 name mapping → document font IDs → renderer face/HFT 선택 및 HWPX 왕복을 확인한다. #7815 적층의 같은 font patch는 중복 적용하지 않았다.

## 독립 기대값과 검증 계획

원본 8개와 독립 한컴 변환본의 이름 계약, 대응 Print 전쪽, HWP3 왕복과 실제 진단 출력 hash 변경 내용을 확인한다. SO-SUEOP·sample16의 알려진 차이는 완료 증거가 아니다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.
