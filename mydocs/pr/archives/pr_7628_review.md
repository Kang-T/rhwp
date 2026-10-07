---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7628_review.md
last_verified: 2026-10-08
---

# PR #7628 리뷰 — 인라인 표·탭·정렬과 첫 TAC 원점

## 최종 판정

**머지 보류.** 공개 대조군의 충족, 실행으로 확인한 시각 미충족, 코드 검토상 우려와 필수 증거 부족을 아래에 구별한다. 사용자가 승인한 conflict·CI 보정은 진행했으나 90% 시각 gate는 면제되지 않는다. 통합 후보의 로컬 회귀 PASS를 원 contributor head의 독립 검증 완료나 merge 승인으로 보고하지 않는다.

## 접수·적용 범위

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#7628](https://github.com/edwardkim/rhwp/pull/7628) / planet6897 / devel |
| 원 head | `9086277bc82e8f05e84d74eb4a0656a2f76a1e81` |
| 최초 적용 commit | `61b9248234514576b82b04e48d54660f4b62be64` |
| 이 PR의 적용 뒤 head | `5c5edd08459ac57aac9798122062071cd5b02a3e`; 기능 commit 5개, 원 저자·cherry-pick -x 보존 |
| 누적 검증 source head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563` |
| 최신 base 정렬 뒤 code head | `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`; 제품/검증 입력 tree 동일 확인 |
| 검토 branch·reviewer | `review/planet6897-20261008` / jangster77; 원 PR reviewer assign 선행 |
| 원 PR 참고 상태 | draft=False, MERGEABLE / CLEAN; 작성 시점 snapshot이며 merge 직전 판정을 대신하지 않음 |
| 경로 | collaborator 외부 PR의 주 작업공간 누적 cherry-pick 9.1.1; intake/template/local_validation/visual_fixture_evidence 및 대형 PR 보조 절차 |

관련 이슈와 해결 표현은 원 PR 본문의 범위를 유지한다. 이번 보류 기록에 새로운 closes 표현을 추가하지 않는다. 원 PR 제공 before/after와 corpus 수치는 참고이며 통합 head의 독립 실행과 구별한다.

## 구현 주장·소비 경로·검증 판정

`layout/paragraph_layout.rs`의 저장 탭 폭·탭 간격 → inline_x/줄 나눔 → 말미 공백을 제외한 정렬 폭과 실제 TextRun 원점을 대조했다. 같은 경로는 누름틀 마커를 건너뛴 저장 줄 소비도 바꾼다. `layout/table_layout.rs`의 끊긴 셀 사다리 정렬 높이와 저장 줄 없는 첫 문단 TAC 원점은 별도 소비 경로다. 공통 값 뒤 최종 앵커 선택·정렬 보정을 다시 확인해야 한다.

**미검증:** 비공개 결재란/양식 원본은 로컬에서 찾았지만 대응 Print PDF가 없었다. 공개 변환·합성 fixture의 계약 검사 PASS와 작성자 제공 이미지를 원본의 독립 출력 일치로 바꾸어 보고하지 않는다. 탭·후행 공백·Field marker·첫 문단 위 간격 각각의 정상 대조군/비적용 경계에 같은 편집을 한 저장본의 Print PDF가 필요하다. 검증 범위 최저 90%를 입증하지 못해 머지 보류한다.

공통 빌드·회귀·글꼴 등록·383쪽 시각 결과와 입력/기준 해시·증적 보존·merge 후 댓글 계획은 [#7611 공통 통합 검증](pr_7611_review.md#공통-통합-검증과-증거-보존)에 연결한다. 이 PR의 조건부 수용이 아니라 미충족·미검증 해소 후 재검토 계획이다.
