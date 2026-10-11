---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7672 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **미주 제목 소유 경계**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `b387d1358256c0ae02f68b8c2a497026c3f0d1e9`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `d9976901da859be26367b32f5220efa01618de5c` | `72fb611e6cfbbac2526447639d0c8396f0030d54` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

미주 등록 범위 → 첫 문단 소유 조회 → 미주 측정·배치의 제목 여부를 추적한다. #7832의 미주 범위 검사와 새 첫 문단 검사를 함께 보존했다.

## 독립 기대값과 검증 계획

같은 미주 본문의 문 접두사·다른 접두사 편집 대조군을 저장하고 같은 편집본의 독립 Print를 확보한다. 원본 출력 무변화로 편집 경로를 비해당 처리하지 않는다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.


## 편집본 출처 재확인과 시각 실패 — 2026-10-12

첫 편집본을 current Native CLI `467bfd1d8`로 같은 원본·같은 명령으로 다시 생성했다.
`2023-note348-mun.hwp`는 이전 `f083228cd` 생성본과 바이트 및 SHA-256
`29317054c045cdb4d4fbbd92a9a86ac1cd8c1bf81dda8873166c752de8b1516b`가 동일하다.
따라서 이전 CLI가 만든 잘못된 편집본이라는 가정을 시각 실패의 원인으로 사용하지 않는다.
기존 편집본에 대응하는 독립 Print는 계속 동일 입력의 기준이다.

AB production의 기존 전쪽 raster에서 1·15·20쪽 review를 만들어 직접 판독했다.
15쪽(37.26871%)에서 같은 미주 문항의 세로 간격 및 뒤 칼럼 시작 내용이 독립 Print와
다르다. 글꼴만의 차이·낮은 평균·과거 입력이라는 분류로 면제하지 않았다.
같은 입력의 current production Native 전쪽 재출력은 진행 중이다.

ignored output의 `pr7672-current-edit-input-byte-comparison.json`,
`pr7672-edited-native-scores-valid-subsets/pr7672-2023-note348-mun/pr7672-2023-note348-mun/review/review_015.png`,
`pr7672-current-467-native-sweep.log`에 연결한다. 기존 raster 판독과 current 재출력을
구분하며 미주 제목 소유 보정의 전체 시각 수용은 아직 미충족이다.
