---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7622_review.md
last_verified: 2026-10-08
---

# PR #7622 리뷰 — 굽은 따옴표의 영문 글꼴 슬롯

## 최종 판정

**머지 보류.** 공개 대조군의 충족, 실행으로 확인한 시각 미충족, 코드 검토상 우려와 필수 증거 부족을 아래에 구별한다. 사용자가 승인한 conflict·CI 보정은 진행했으나 90% 시각 gate는 면제되지 않는다. 통합 후보의 로컬 회귀 PASS를 원 contributor head의 독립 검증 완료나 merge 승인으로 보고하지 않는다.

## 접수·적용 범위

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#7622](https://github.com/edwardkim/rhwp/pull/7622) / planet6897 / devel |
| 원 head | `cca0d1b671d62b9111774f53c6948d85c0fb5d4d` |
| 최초 적용 commit | `9229d63b3868bc2b44a839e240c7ee2e5c3a25af` |
| 이 PR의 적용 뒤 head | `f6b95026683f1762231cc0b2db94c906c195d3a2`; 기능 commit 3개, 원 저자·cherry-pick -x 보존 |
| 누적 검증 source head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563` |
| 최신 base 정렬 뒤 code head | `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`; 제품/검증 입력 tree 동일 확인 |
| 검토 branch·reviewer | `review/planet6897-20261008` / jangster77; 원 PR reviewer assign 선행 |
| 원 PR 참고 상태 | draft=False, MERGEABLE / CLEAN; 작성 시점 snapshot이며 merge 직전 판정을 대신하지 않음 |
| 경로 | collaborator 외부 PR의 주 작업공간 누적 cherry-pick 9.1.1; intake/template/local_validation/visual_fixture_evidence 및 대형 PR 보조 절차 |

관련 이슈와 해결 표현은 원 PR 본문의 범위를 유지한다. 이번 보류 기록에 새로운 closes 표현을 추가하지 않는다. 원 PR 제공 before/after와 corpus 수치는 참고이며 통합 head의 독립 실행과 구별한다.

## 구현 주장·소비 경로·검증 판정

`style_resolver.rs:824`의 `detect_lang_category`는 U+2018/2019/201C/201D만 영문 슬롯으로 분류한다. 독립 공개 논문 PDF p3의 ‘/’ face는 TimesNewRomanPSMT, p4의 U+2024는 Haansoft Batang임을 PDFium으로 조회하여 punctuation 블록 전체를 바꾸지 않는 근거와 대조했다. 동일 원본 7쪽 Native/fresh WASM 최저 94.23341%, 대표 p3의 2단·줄바꿈과 인용부호를 직접 비교했다. 글꼴 굵기·AA 및 인용 참조 모양 차이는 남는다.

**실행한 진단의 제한:** legacy 인라인 표 경로 `layout/paragraph_layout.rs:3616`은 `cs_changed`에 언어 변경을 포함하지 않고 누적 run을 첫 문자의 `first_lang`으로 칠한다. 공개 TAC 입력에서 한글과 ‘/’를 섞고 모든 영문 face를 Times New Roman으로 지정한 합성 진단에서 따옴표 SVG는 GulimChe 체인이었다. 이는 현재 소비 경로의 미해결 결과이며 새로 발생한 회귀나 정상 한컴 저장본과의 차이로 승격하지 않는다. 진단 입력의 저장 LineSeg를 실제 생성본의 증거로 사용하지 않는다.

**미검증:** 이 소비 경로의 독립 편집 Print PDF·수정 전 FAIL/수정 후 PASS, 원 PR의 전체 175문서 영향 범위·쪽수 차이. 새 회귀 fixture/golden을 추가하거나 기존 기준값을 통과 목적으로 완화하지 않았다.

공통 빌드·회귀·글꼴 등록·383쪽 시각 결과와 입력/기준 해시·증적 보존·merge 후 댓글 계획은 [#7611 공통 통합 검증](pr_7611_review.md#공통-통합-검증과-증거-보존)에 연결한다. 이 PR의 조건부 수용이 아니라 미충족·미검증 해소 후 재검토 계획이다.

![native quotes-retry p3 비교](../assets/planet6897_20261008/native_quotes-retry_p003_review.png)

![wasm quotes-retry p3 비교](../assets/planet6897_20261008/wasm_quotes-retry_p003_review.png)
