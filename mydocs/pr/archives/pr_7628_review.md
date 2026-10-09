---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7628_review.md
last_verified: 2026-10-09
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


## 원본·독립 Print PDF 재확보와 최신 Native 비교

처음 기록의 “대응 Print PDF 없음”은 접수 당시의 상태다. 원본 5개를 지정된 다운로드 자료에서 찾았으며 모두 실제 마지막 저장 product가 Hancom2020/11.0.0.8227이었다. 해당 버전 계약대로 HWP2020 MCP의 비동기 Print method0/one-up으로 PDF를 새로 만들었고, 실제 runtime11.0.0.9136, 완료 상태·byte 수·SHA·PDF signature/EOF·쪽수를 확인했다. 변형된 기존3개 sample은 그대로 보존하며 아래 원본과 동일한 입력으로 취급하지 않는다. 서비스 주소·token은 공개하지 않는다.

| 원본 | 독립 Print PDF | 쪽수 | Native 전쪽 최저 | 판정 |
| --- | --- | --- | --- | --- |
| [36308670](../../../samples/issue7344/original/36308670_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36308670-2020.pdf) | 2 | 87.25354% | 미충족 — 2쪽 표 높이/뒤 내용 차이 |
| [36312980](../../../samples/issue7344/original/36312980_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36312980-2020.pdf) | 1 | 99.91520% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |
| [36414761](../../../samples/issue7344/original/36414761_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36414761-2020.pdf) | 2 | 99.70523% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |
| [36455985](../../../samples/issue7344/original/36455985_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36455985-2020.pdf) | 1 | 99.73514% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |
| [36494836](../../../samples/issue7344/original/36494836_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36494836-2020.pdf) | 1 | 99.99166% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |

실행 source는 `f9714465180585f3ed6eb56d9b3df4e17793f4c6`이며 전7쪽을 비교했다. 36308670의2쪽 review/overlay를 직접 판독해 첫 이어받은 행이 짧아 사진 행과 뒤 `끝.`이 위로 당겨진 것을 확인했다. pypdf로 독립 PDF의 실제 괘선 좌표를 읽었으며 사진 행 시작은195.46667px, Native cell 시작은180.5px였다. 단순 글꼴 농도 차이로 분류하지 않는다. Native 전체 score는 이 표와 같고, fresh WASM 원본7쪽 검증은 아직 남는다. `candidate-f97144651-pr7628-original-native.json`, `candidate-f97144651-pr7628-original-panels/`, `pr7628-36308670-pdf-p2-paths.json`을 ignored output에 보존한다.

base74의 같은 원본·새 Print PDF Native 전2쪽 최저42.02958%를 별도로 얻었지만, 임베딩 face와 직접 판독/fresh WASM 및 실제 실패 검사와의 연결을 확인하기 전에는 새로운 #7445 이관을 확정하지 않는다. base74→5168의 원본 read/export 소스 동등성 추론은 새5168 실행과 구별한다. 현재 브랜치87.25354%만 보고 이관하지 않으며, 정상 테스트·원본·PDF는 보존한다. base에서90% 이상인 실패는 현재 branch 개선 대상이고 정확히90%도 포함한다. 다른4개 원본의 Native 개선은 첫 원본의 남은 배치 차이나 편집한 변형 fixture의 독립 Print 검증을 대신하지 않는다. 새 회귀·golden·baseline은 이 자료 보존 commit에서 추가/완화하지 않았다.

원본/PDF의 정확한 SHA-256과 보존 경로는 아래 표이며, 실제 검증 파일과 바이트 동일함을 확인했다.

| 문서 | 원본 SHA-256 | Print PDF SHA-256 |
| --- | --- | --- |
| 36308670 | `d65b785ece5319287c6fdd4b78742b25a856a86b40b0e6b8b14371dd3cbfc134` | `94ff472c75bc2f5e4a211ae743c841f13da8df4b49296da861c5e14ed9603e5c` |
| 36312980 | `57e3acec05d3c856e931b5c4146fae6ecd3f96c6356fc7d6d99c501a83a34e21` | `ef35e530d654dcbfc9c4f008e89b2280d7a297b0e7bbcb2f39efef344ea4532b` |
| 36414761 | `b6dd9fef43c34a099384cb2e0a0795642dc268a3a50f54affc713f4814717541` | `77b776755b080f5f8621f53910636e2922fc2d407cbf37afdc8e5750d575bbc3` |
| 36455985 | `9518b5ee4ca2c625b9b6ac8504def1915472bd7be60023eec9b79a4f38f045a2` | `6b6dc687b810ab6a1f6def894334fcf9e2bde1084d1a7ad6bf38b1416fc2b8a7` |
| 36494836 | `4103c9e87b43bd783043c9d757d83179363e9ca5e50cc6c50bcea7220ac86358` | `485e04ee4d309235cdbb0bcb345432c879391d850e1b4831a3d22136a2dbabde` |
