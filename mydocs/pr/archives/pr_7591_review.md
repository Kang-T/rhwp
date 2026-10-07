---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7591_review.md
last_verified: 2026-10-08
---

# PR #7591 리뷰 — 미주 단 수용과 잉크 하단

## 최종 판정

**머지 보류.** 공개 대조군의 충족, 실행으로 확인한 시각 미충족, 코드 검토상 우려와 필수 증거 부족을 아래에 구별한다. 사용자가 승인한 conflict·CI 보정은 진행했으나 90% 시각 gate는 면제되지 않는다. 통합 후보의 로컬 회귀 PASS를 원 contributor head의 독립 검증 완료나 merge 승인으로 보고하지 않는다.

## 접수·적용 범위

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#7591](https://github.com/edwardkim/rhwp/pull/7591) / planet6897 / devel |
| 원 head | `d5cc92dba07eb7c7db768d7a14fd72fb47e72417` |
| 최초 적용 commit | `c08ce6cf47b2aa6d07c0fb7d350f618b170ab848` |
| 이 PR의 적용 뒤 head | `cba840e0febbf392772b798a3b19832e9ecf4765`; 기능 commit 22개, 원 저자·cherry-pick -x 보존 |
| 누적 검증 source head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563` |
| 최신 base 정렬 뒤 code head | `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`; 제품/검증 입력 tree 동일 확인 |
| 검토 branch·reviewer | `review/planet6897-20261008` / jangster77; 원 PR reviewer assign 선행 |
| 원 PR 참고 상태 | draft=False, MERGEABLE / CLEAN; 작성 시점 snapshot이며 merge 직전 판정을 대신하지 않음 |
| 경로 | collaborator 외부 PR의 주 작업공간 누적 cherry-pick 9.1.1; intake/template/local_validation/visual_fixture_evidence 및 대형 PR 보조 절차 |

관련 이슈와 해결 표현은 원 PR 본문의 범위를 유지한다. 이번 보류 기록에 새로운 closes 표현을 추가하지 않는다. 원 PR 제공 before/after와 corpus 수치는 참고이며 통합 head의 독립 실행과 구별한다.

## 구현 주장·소비 경로·검증 판정

미주 수용 판정이 `typeset/notes/endnotes/measure.rs:194`의 `measure_endnote_render_ink_fit` → `simulate_endnote_column_extent:263` → scratch `LayoutEngine`의 실제 배치 결과를 소비한다. 전체 문단의 커서·잉크 끝과 부분 문단의 잉크 끝을 구별하고, 0.25px 허용치로 `Fits / SplitAt / NextColumn`을 결정한다. 뒤 제목 원점과 빈 줄 상자·구분선의 흐름 소비는 별도 분기이므로 helper 공유만으로 모두 검증됐다고 판정하지 않는다.

**미검증:** 원 PR이 보고한 전체 범위의 90% 미만 쪽과 미주 제목·본문 혼합 단, 각주/구분선·빈 줄의 직접 비교를 이 통합 head에서 전수 재확인하지 못했다. 전체 회귀의 endnote 검사 PASS는 독립 PDF 일치의 대용이 아니다. 원 PR의 overflow baseline 증가를 정상 변화로 승인하지 않으며 기존 테스트·증적은 보존한다. 대형 PR 경로를 적용하고 공통 통합 후보의 시각 gate 실패도 유지한다.

공통 빌드·회귀·글꼴 등록·383쪽 시각 결과와 입력/기준 해시·증적 보존·merge 후 댓글 계획은 [#7611 공통 통합 검증](pr_7611_review.md#공통-통합-검증과-증거-보존)에 연결한다. 이 PR의 조건부 수용이 아니라 미충족·미검증 해소 후 재검토 계획이다.
