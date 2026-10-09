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

처음 기록의 “대응 Print PDF 없음”은 접수 당시의 상태다. 원본 5개를 지정된 다운로드 자료에서 찾았으며 모두 실제 마지막 저장 product가 Hancom2020/11.0.0.8227이었다. 해당 버전 계약대로 HWP2020 MCP의 비동기 Print method0/one-up으로 PDF를 새로 만들었고, 실제 runtime11.0.0.9136, 완료 상태·byte 수·SHA·PDF signature/EOF·쪽수를 확인했다. 기존3개 sample은 그대로 보존했다. 아래의 바이트 대조로 이 sample들이 대응 원본과 동일함을 확인했으며, 파일명만으로 수동 수정본으로 분류하지 않는다. 서비스 주소·token은 공개하지 않는다.

| 원본 | 독립 Print PDF | 쪽수 | Native 전쪽 최저 | 판정 |
| --- | --- | --- | --- | --- |
| [36308670](../../../samples/issue7344/original/36308670_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36308670-2020.pdf) | 2 | 87.25354% | 미충족 — 2쪽 표 높이/뒤 내용 차이 |
| [36312980](../../../samples/issue7344/original/36312980_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36312980-2020.pdf) | 1 | 99.91520% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |
| [36414761](../../../samples/issue7344/original/36414761_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36414761-2020.pdf) | 2 | 99.70523% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |
| [36455985](../../../samples/issue7344/original/36455985_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36455985-2020.pdf) | 1 | 99.73514% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |
| [36494836](../../../samples/issue7344/original/36494836_seoul_approval.hwpx) | [Print PDF](../../../pdf/issue7344/36494836-2020.pdf) | 1 | 99.99166% | Native 점수 충족; fresh WASM·최종 직접 판독 필요 |

실행 source는 `f9714465180585f3ed6eb56d9b3df4e17793f4c6`이며 전7쪽을 비교했다. 36308670의2쪽 review/overlay를 직접 판독해 첫 이어받은 행이 짧아 사진 행과 뒤 `끝.`이 위로 당겨진 것을 확인했다. pypdf로 독립 PDF의 실제 괘선 좌표를 읽었으며 사진 행 시작은195.46667px, Native cell 시작은180.5px였다. 단순 글꼴 농도 차이로 분류하지 않는다. Native 전체 score는 이 표와 같고, fresh WASM 원본7쪽 검증은 아직 남는다. `candidate-f97144651-pr7628-original-native.json`, `candidate-f97144651-pr7628-original-panels/`, `pr7628-36308670-pdf-p2-paths.json`을 ignored output에 보존한다.

base74의 같은 원본·새 Print PDF Native 전2쪽 최저42.02958%를 별도로 얻었지만, 임베딩 face와 직접 판독/fresh WASM 및 실제 실패 검사와의 연결을 확인하기 전에는 새로운 #7445 이관을 확정하지 않는다. base74→5168의 원본 read/export 소스 동등성 추론은 새5168 실행과 구별한다. 현재 브랜치87.25354%만 보고 이관하지 않으며, 정상 테스트·원본·PDF는 보존한다. base에서90% 이상인 실패는 현재 branch 개선 대상이고 정확히90%도 포함한다. 다른4개 원본의 Native 개선은 첫 원본의 남은 배치 차이나 실제로 편집한 입력의 독립 Print 검증을 대신하지 않는다. 새 회귀·golden·baseline은 이 자료 보존 commit에서 추가/완화하지 않았다.

원본/PDF의 정확한 SHA-256과 보존 경로는 아래 표이며, 실제 검증 파일과 바이트 동일함을 확인했다.

| 문서 | 원본 SHA-256 | Print PDF SHA-256 |
| --- | --- | --- |
| 36308670 | `d65b785ece5319287c6fdd4b78742b25a856a86b40b0e6b8b14371dd3cbfc134` | `94ff472c75bc2f5e4a211ae743c841f13da8df4b49296da861c5e14ed9603e5c` |
| 36312980 | `57e3acec05d3c856e931b5c4146fae6ecd3f96c6356fc7d6d99c501a83a34e21` | `ef35e530d654dcbfc9c4f008e89b2280d7a297b0e7bbcb2f39efef344ea4532b` |
| 36414761 | `b6dd9fef43c34a099384cb2e0a0795642dc268a3a50f54affc713f4814717541` | `77b776755b080f5f8621f53910636e2922fc2d407cbf37afdc8e5750d575bbc3` |
| 36455985 | `9518b5ee4ca2c625b9b6ac8504def1915472bd7be60023eec9b79a4f38f045a2` | `6b6dc687b810ab6a1f6def894334fcf9e2bde1084d1a7ad6bf38b1416fc2b8a7` |
| 36494836 | `4103c9e87b43bd783043c9d757d83179363e9ca5e50cc6c50bcea7220ac86358` | `485e04ee4d309235cdbb0bcb345432c879391d850e1b4831a3d22136a2dbabde` |

## HWPX 저장 행의 물리 높이 보정 후보

리베이스 뒤 `2288ccb2cdbe78c2781d73025932a9277738bdff`에서 남은 차이를 추적했다. 실제 Hancom 저장 원본 36308670의 XML `pageBreak="CELL"` 행은 두 Center 정렬 소유 셀이 같은 선언 높이 35618HU를 갖고, 본문 셀의 저장 문단 vpos가 쪽 경계에서 다시 시작한다. 독립 Print PDF의 앞/뒤 행 조각 높이 합은 약 474.36px로 선언 높이 474.91px에 대응한다. 기존 Native 조각 합은 약 459.8px이며 뒤 사진 행이 약 15px 위로 당겨졌다. 이 수치는 진단 관측값이며 회귀 기대값으로 고정하지 않는다.

후보는 저장본·비편집·비재조판 HWPX CELL 경로에서 연속 열을 완전히 덮는 비병합 소유 셀, 동일 선언 높이, 비Top 정렬, 유효한 저장 문단 프레임 재시작과 비투영 내용 조건을 확인한다. XML CELL은 `parser/hwpx/section.rs`에서 IR `RowBreak`로 변환되므로 이 내부 속성으로 분기한다. 기존 HWP5 RowBreak 수용 조건과 편집/재조판 경로는 유지한다. 특정 문서 식별자나 위치 상수를 추가하지 않았다.

실제 소비 경로는 `layout/table_layout.rs::stored_full_width_row_declared_height` → `typeset/table/continuation/fragment/emit.rs`의 수용 예산 및 `stored_row_box_sum` 누적 → 같은 emit의 실제 수용 높이를 뺀 `start_row_height_override` → `scan/runner.rs`·행 스캔의 잔여 요구 높이 → `layout/table_partial.rs`의 행 높이와 저장 프레임 정렬 원점이다. 내용 컷과 이미 배치한 물리 공간을 구별하며, 측정 이후 paint가 다른 높이를 추정하는 처리를 추가하지 않는다.

**미검증:** 이 후보의 컴파일·작은 경계 실행·새 Native/fresh WASM 출력은 아직 실행 전이다. 앞/뒤 조각의 내용 소속·누락·중복, 마지막 유닛 뒤 빈 밴드와 뒤 행/문단, 정상 대조군을 새 source head에서 확인한다. 이전 f971의 시각·집중 검사 통과를 이 후보의 통과로 재사용하지 않으며 새 회귀·baseline 변경도 아직 추가하지 않는다.

기존 `36312980_broken_cell_ladder_center.hwpx`, `36455985_no_lineseg_cell_spacing_before.hwpx`, `36494836_field_marker_tac_lines.hwpx`의 SHA-256은 각각 위 대응 원본과 정확히 같다. 새로 확보한 이 세 경로의 Print PDF는 동일 입력의 반복 출력이며, 별도의 편집·합성 입력 coverage로 세지 않는다. 과거의 변형본 분류는 이 바이트 대조로 정정했다.

후보 commit `46de329e53084e5c50b62555e256cf7e2f8f7e8a` 뒤 `upstream/devel`을 다시 fetch했다. base는 `6f66932a73ee6fd05a74a83bdc438c9499a89a12`로 유지되어 rebase는 up-to-date로 끝났고 원 기여/보정 순서는 바뀌지 않았다. 동일 후보를 검토 worktree에 checkout해 파생 suite 준비와 `cargo fmt --all -- --check`를 exit0으로 확인했다. 주 작업공간의 오래된 ignored generated suite를 대상으로 실패했던 앞선 포맷 실행은 통과 근거에 포함하지 않는다. Native 빌드와 시각 확인은 이 기록 작성 시 진행 중이며 이전 종료값 없는 workspace 검사는 중단 기록으로 보존했다.

이후 후보46de의 Native 빌드는447.50초/exit0으로 완료했고 원본5개 전7쪽과 동일 바이트 샘플3개의 반복 Print 전3쪽을 비교했다. 정상4개는 위 점수를 유지했으나36308670의2쪽은87.25354%로 바뀌지 않았다. 실제 scan은 `end_cut=[1,16]`, `frame=None`, `override=None`였다. 후보의 enum 조건이 XML 이름을 내부 enum 이름으로 잘못 해석해 경로가 비적용이었던 것을 파서의 CELL→RowBreak 매핑으로 확인하고 수정했다. 새 후보는 아직 재빌드·시각 실행 전이며, 이 관측을 개선 통과나 정식 회귀의 수정 전 FAIL로 세지 않는다.
