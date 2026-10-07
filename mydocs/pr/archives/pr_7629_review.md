---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7629_review.md
last_verified: 2026-10-08
---

# PR #7629 리뷰 — 쪽 나누기 첫 문단 원점과 병합 칸 컷

## 최종 판정

**머지 보류.** 공개 대조군의 충족, 실행으로 확인한 시각 미충족, 코드 검토상 우려와 필수 증거 부족을 아래에 구별한다. 사용자가 승인한 conflict·CI 보정은 진행했으나 90% 시각 gate는 면제되지 않는다. 통합 후보의 로컬 회귀 PASS를 원 contributor head의 독립 검증 완료나 merge 승인으로 보고하지 않는다.

## 접수·적용 범위

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#7629](https://github.com/edwardkim/rhwp/pull/7629) / planet6897 / devel |
| 원 head | `7879869327ef975e66a1317145e0e0b25815c478` |
| 최초 적용 commit | `522803bceac5a905557d41b59108f07b384416ad` |
| 이 PR의 적용 뒤 head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563`; 기능 commit 2개, 원 저자·cherry-pick -x 보존 |
| 누적 검증 source head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563` |
| 최신 base 정렬 뒤 code head | `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`; 제품/검증 입력 tree 동일 확인 |
| 검토 branch·reviewer | `review/planet6897-20261008` / jangster77; 원 PR reviewer assign 선행 |
| 원 PR 참고 상태 | draft=False, MERGEABLE / CLEAN; 작성 시점 snapshot이며 merge 직전 판정을 대신하지 않음 |
| 경로 | collaborator 외부 PR의 주 작업공간 누적 cherry-pick 9.1.1; intake/template/local_validation/visual_fixture_evidence 및 대형 PR 보조 절차 |

관련 이슈와 해결 표현은 원 PR 본문의 범위를 유지한다. 이번 보류 기록에 새로운 closes 표현을 추가하지 않는다. 원 PR 제공 before/after와 corpus 수치는 참고이며 통합 head의 독립 실행과 구별한다.

## 구현 주장·소비 경로·검증 판정

`HeightCursor`의 쪽 시작 앞 간격 해석 → typeset fit/예약 → 실제 paragraph 원점과 빈 float host의 사전 차감 경로를 대조했다. 병합 칸은 `table_layout.rs:19083 snap_units_to_stored_cell_reset`가 유효 저장본·비편집 셀에서 높이 fit을 fit-1로 바꿀 수 있고, `table_partial.rs:1577`의 paint 끝 컷과 `table_layout.rs:20594/20602`의 다음 시작/끝 컷에서 사용된다.

**필수 증거 부족:** 두 호출의 `eu == 다음 su`만으로 요구 높이·누적 예약·예산 실패·바깥 괘선·뒤 본문 계약은 입증되지 않는다. 작은 원래 높이만 fit하는 예산, 시작/끝 컷과 rowspan 종료·물리 잔여 높이를 같은 실제 호출 경로로 비교해야 한다. 작성자가 보고한 변경 127쪽 중 78쪽의 90% 미만 결과와 #7095 관련 p3 악화는 저자 자료의 보류 사유이며, 이번 통합 head에서 개별 수정의 새 회귀로 독립 재현한 결과는 아니다. focused/full PASS를 시각 충족으로 승격하지 않는다.

공통 빌드·회귀·글꼴 등록·383쪽 시각 결과와 입력/기준 해시·증적 보존·merge 후 댓글 계획은 [#7611 공통 통합 검증](pr_7611_review.md#공통-통합-검증과-증거-보존)에 연결한다. 이 PR의 조건부 수용이 아니라 미충족·미검증 해소 후 재검토 계획이다.
