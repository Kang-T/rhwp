---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7611_review.md
last_verified: 2026-10-08
---

# PR #7611 리뷰 — 큰 표·저장 컷과 편람 쪽 배치

## 최종 판정

**머지 보류.** 공개 대조군의 충족, 실행으로 확인한 시각 미충족, 코드 검토상 우려와 필수 증거 부족을 아래에 구별한다. 사용자가 승인한 conflict·CI 보정은 진행했으나 90% 시각 gate는 면제되지 않는다. 통합 후보의 로컬 회귀 PASS를 원 contributor head의 독립 검증 완료나 merge 승인으로 보고하지 않는다.

## 접수·적용 범위

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#7611](https://github.com/edwardkim/rhwp/pull/7611) / planet6897 / devel |
| 원 head | `700c0557df154e17255c8bde76881839f0033767` |
| 최초 적용 commit | `b14a0e4daf79ac36799e65aed71226a28826cea1` |
| 이 PR의 적용 뒤 head | `d7c6dae108d6acf72c2c2321e243d93bd116ab50`; 기능 commit 13개, 원 저자·cherry-pick -x 보존 |
| 누적 검증 source head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563` |
| 최신 base 정렬 뒤 code head | `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`; 제품/검증 입력 tree 동일 확인 |
| 검토 branch·reviewer | `review/planet6897-20261008` / jangster77; 원 PR reviewer assign 선행 |
| 원 PR 참고 상태 | draft=False, MERGEABLE / CLEAN; 작성 시점 snapshot이며 merge 직전 판정을 대신하지 않음 |
| 경로 | collaborator 외부 PR의 주 작업공간 누적 cherry-pick 9.1.1; intake/template/local_validation/visual_fixture_evidence 및 대형 PR 보조 절차 |

관련 이슈와 해결 표현은 원 PR 본문의 범위를 유지한다. 이번 보류 기록에 새로운 closes 표현을 추가하지 않는다. 원 PR 제공 before/after와 corpus 수치는 참고이며 통합 head의 독립 실행과 구별한다.

## 구현 주장·소비 경로·검증 판정

`typeset/table/continuation/fragment.rs:72`가 저장 컷·걸침 칸 밴드로 `straddle_row_relief`를 만들고, cut/whole-row fit 높이에 반영한다. 방출의 `fragment/emit.rs` 두 `PartialTable` 경로 → `pagination.rs:643` → `layout.rs:11413` → `layout/table_partial.rs`의 relief 소비를 확인했다. 미주 scratch remap도 같은 필드를 전달하도록 충돌을 보정했다. 이 연결의 존재와 실제 컷·예약 높이·빈 물리 밴드·종료 계약의 충족은 구별한다.

**실행으로 확인한 미충족:** 공개 편람 HWP는 383쪽으로 맞지만 Native/fresh WASM 전쪽에서 각각 50쪽이 90% 미만이며 156쪽 최저 47.54585%다. 기준 156쪽의 `나. 문장을 병기하는 경우…`가 rhwp 155쪽에 소속되고, 156쪽은 `다. …`부터 시작한다. 표 첫 조각의 높이가 줄어 뒤 본문이 위로 당겨진다. 2020·2024 독립 PDF 모두 이 문단을 156쪽에 두므로 단순 PDF 연도 차이·글꼴 예외로 해소되지 않는다. 통합 후보의 관측이며 개별 PR이 최초 원인이라는 base 대비 회귀 증명은 아직 하지 않았다.

**미검증:** HWPX 전쪽과 나머지 대형 문서의 전쪽 시각 비교, 원래 높이만 fit하는 예산·여러 rowspan 종료·빈 밴드/후속 캡션·각주 경계, 기존 baseline 증가의 독립 근거. 48개 입력의 쪽수 조회는 아래 공통 결과에 분리한다. 쪽수가 같거나 unit 컷이 단조라는 이유로 배치·누락/중복 검증을 충족 처리하지 않는다.


## 공통 통합 검증과 증거 보존

### 메인터너 보정 조사 — 원본 셀 좌표의 쪽 프레임

시스템에 등록한 원래 KoPub 6개 face와 원본 입력을 사용해 152~156쪽을 추가 조사했다. 두 독립 한컴 PDF의 156쪽은 `나. 문장을 병기하는 경우…`로 시작하고, 원본 section 4 / paragraph 47의 표 오른쪽 셀에도 해당 문단 앞에 양수 `vpos → 0`이 있다. 그 전후 다른 문단은 직전 저장 줄 높이·간격만큼 같은 셀 좌표를 전진한다. 각 문단이 독립적으로 0부터 시작하는 입력과 이 연속 좌표 원본을 구별했다. 원시 행 상자들의 전체 합은 저장 object의 첫 조각 높이보다 크다.

수정 전 ordinary-row scanner는 선언 행 높이가 예산에 들어가면 `CellUnit` 컷을 생략했고, 컷 경로에서도 셀 좌표가 본문 높이의 절반보다 작으면 실제 쪽 되감김을 relaxed reset으로 흡수했다. 원본 줄의 쪽 소속을 유지한다는 계약에 따라, 변경되지 않은 plain-text 셀의 저장 프레임 경계를 unit 생산·ordinary-row 수용 판단·컷의 같은 결과로 전달하는 보정 후보를 작성했다. 표 ID·쪽 번호·임의 픽셀값으로 분기하지 않는다. 편집·reflow·합성 줄·문단별 로컬 0·걸침 칸은 이 새 저장 원본 증거의 대용으로 사용하지 않는다.

저장 프레임 보정은 로컬 commit `e70954bb8`에 분리했다. Native Clippy와 진단 빌드를 통과했고 383쪽을 유지하며 155쪽 끝 컷이 `[3,8]`에서 `[3,4]`로, 156쪽 시작 컷도 `[3,4]`로 바뀌었다. 이는 목표 문단의 쪽 소속에 대한 중간 관측이다. 첫 후보의 152~156쪽 실제 비교·overlay·review를 산출해 직접 판독했으나 최저 45.29312%로 시각 gate는 여전히 미충족이다. 표 외곽·뒤 본문·빈 물리 띠·누락/중복의 확인과 대조군, 최종 source의 Native/fresh WASM 전쪽 및 필수 회귀·lint는 아직 완료 판정하지 않는다. 아래 PASS 표는 명시된 수정 전 source의 기록이며 보정 후보에 승계하지 않는다. 새 rendering 회귀는 전체 적용 범위의 시각 선행 gate를 통과한 뒤에만 추가한다.

추가로 간격 있는 셀의 geometry를 추적했다. 이 원본은 `cell_spacing=255HU`이고, 실제 셀 너비 `6454 + 32942HU`와 표 선언 너비 `40161HU`의 차이는 내부 한 띠·외부 두 띠의 합 `3 × 255HU`다. 독립 PDF 155쪽의 좌·우 셀 괘선도 간격을 두고 각각 그려지지만 Native는 폭 잔여를 마지막 열에 넣고 다음 셀 시작까지의 간격을 앞 셀 폭에 포함했으며 인접 괘선을 합쳤다. PDF path의 실제 변환 행렬과 SVG를 96dpi 좌표로 대조한 원본 수치는 ignored `candidate-native-boundary/pdf-p155-lines-96.json`에 보존했다. 기존 기술 문서의 “외곽 간격 없음” 설명은 이 저장본·정상 PDF와 충돌하므로 구현 증명으로 사용하지 않는다.

간격 보정 후보에서는 원본 셀 상자의 폭·높이와 빈 간격 띠를 분리한다. `Table::outer_cell_spacing_hu` 결과를 `HeightMeasurer`의 전체 높이 → fragment budget의 가용 높이 → fragment emit의 실제 예약에 전달하고, 같은 원본 행·컷 높이로 normal/nested/partial의 셀 원점과 상자를 구성한다. 간격 있는 괘선은 최종 셀 bbox를 소비하고, 간격 0의 기존 공유 엣지 경로와 구별한다. 이 후보의 실제 출력·대조군·필수 검증은 진행 중이며, 전체 외곽 fallback·cellzone·반복 헤더·걸침 칸·nested cut 이후 덮어쓰기까지 확인하기 전에는 충족으로 바꾸지 않는다.

추가 후보는 모두 미채택 상태로 보존한다. 후보 3은 384쪽·156쪽 63.97231%, 후보 4는 383쪽이지만 155쪽의 오른쪽 내용이 다음 행에 겹쳤다. 후보 5는 Native Clippy·진단 빌드 PASS 후 155·156쪽의 직접 판독에서 그 겹침은 없어졌으나 각각 74.07245%·69.69247%로 gate FAIL이고, 전체 쪽수는 380으로 줄었다. 383쪽이었던 후보 4도 구역별로는 이전 source와 달랐다. 각 후보의 정확한 dirty source patch·파일별 SHA-256·실행 binary SHA-256·전쪽 page items·최종 exit·compare/overlay/review는 ignored `candidate-*-runtime-provenance.json`, `candidate-*-source.patch`, `candidate-*-all-page-items.json`, `candidate-native-reviews/`에 구별 보존한다.

후보 5의 호출 경로 대조에서 두 원인을 확인했다. 첫 표의 source host `vpos=30321HU`와 본문 원점·바깥 위 여백·셀 간격은 독립 PDF의 첫 셀 위치를 설명하지만, 이전 flow cursor는 표를 약 32px 위에 놓았다. 저장 원점 조회를 block 준비·첫 조각 예산·partial paint에서 공유하도록 보정했다. 또 `scan/runner.rs`의 빠른 경로는 `start_row_height_override`가 있다는 이유로 내용 검사 없이 행 전체를 소비했다. 실제로 구역 11의 큰 행이 6개 조각에서 2개로 줄었으며 남은 내용과 잔여 물리 높이를 혼동한 경로였다. 후보 6은 유닛의 실제 소진 여부를 확인한 뒤에만 그 빠른 경로를 사용하고, 예약한 최종 행 높이를 이월에 반영하며, 입증된 첫 object frame 안에서 스캔하도록 수정 중이다. 아직 출력·회귀 통과를 주장하지 않는다.

후보 6의 Native Clippy·진단 빌드는 PASS이고 큰 행의 6개 조각이 복원됐다. 155·156쪽 compare/overlay/review를 직접 판독했으며 일치율은 81.63049%·77.80430%로 개선됐지만 전체 384쪽·gate FAIL이다. 그리기의 `single_cell_page_fragment_bottom`은 이름과 달리 일반 다행 표의 하단 접기에도 사용되는데, source-owned row frame의 예산은 같은 경계를 예약하지 않았다. 따라서 실제 수용한 높이와 배치 뒤 접힌 높이가 달라 이월 물리 공간도 어긋났다. 독립 PDF 152~156·316~318쪽 괘선의 실제 CTM 좌표는 `independent-pdf-cell-edges-selected-pages-96.json`에 보존했다. 후보 7은 해당 저장 원본 경로에서 paint의 기존 쪽 경계를 먼저 예산에 반영하며, 행 컷과 rowspan block 컷의 인덱스 공간도 구별한다. 아직 Native/fresh WASM 전쪽 gate·필수 회귀의 충족으로 보고하지 않는다.

후보 7은 Native Clippy·진단 빌드 PASS 후 155쪽 97.81975%·156쪽 90.35099%로 **선택한 두 쪽의 Native gate만 PASS**다. 같은 두 review PNG를 직접 열어 표 외곽·행 사이 간격·이월 문단과 표 뒤 본문을 대조했다. 글자 색·일부 AA와 머리말 모양은 남아 있다. 전체 쪽수는 386이며 기준 PDF 383과 다르다. 구역 7은 원 누적 source보다 1쪽, 구역 11·12는 각각 추가로 1쪽 늘었으므로 전체 쪽수·전쪽 gate의 미충족은 유지한다. 이 중간 원점·공간 보정은 로컬 checkpoint로만 보존하고, 추가 쪽의 원인·대조군·fresh WASM 및 최종 필수 검증을 계속한다. source/push/PR 생성·merge 승인으로 승격하지 않는다.

후보 8은 걸침 셀의 선언/내용 높이를 행 core 합뿐 아니라 내부 셀 간격을 포함한 물리 높이와 비교하도록 측정·배치·저장 높이 축소의 소비 지점을 함께 수정했다. 4행 span의 내부 3띠가 배치에서 다시 더해지던 약 10.2px 증가를 제거해 220쪽 끝 행 14, 221쪽 시작 행 14를 한컴과 맞췄다. 원본 Center/Bottom 정렬 프레임의 기존 적용 조건도 다중 owner에 유지해 Top 정렬의 선언 높이를 무조건 이월 최소 높이로 쓰지 않는다. Native Clippy·진단 빌드 PASS, 전체 385쪽, 155·156쪽은 97.81975%·90.35099%를 유지했다. 220~224쪽 재출력에서 220쪽은 90% 이상이나 221쪽 85.21024%, 222쪽 56.80893%, 223쪽 63.97008%, 224쪽 23.36573%로 gate FAIL이다. 221쪽 review를 직접 열어 첫 표의 행·외곽은 맞고 뒤 정책연구 윤리 점검 표가 다음 쪽으로 넘어간 실제 배치 차이를 확인했다. 런타임 source/binary 해시는 ignored `candidate-8-runtime-provenance.json`, patch·쪽 목록·실행 로그·PNG는 같은 증적 폴더에 보존했다. 새 회귀/fixture/golden은 추가하지 않았고 전쪽 Native/fresh WASM·최종 회귀는 미완료다.

후보 9는 저장 문단 사이의 연속성·새 프레임 원점을 resolved 앞/뒤 간격까지 포함해 판별한다. 다음 프레임의 첫 줄이 그 문단 앞 간격에서 시작하면 양수 vpos도 원점이며, host 줄의 앞 간격을 뺀 표 원점을 첫 조각 예약과 paint에 함께 전달한다. 원본 표 239의 새 프레임 첫 줄은 200HU다. 0만 인정하던 선언 이월 guard 대신 같은 원본 text-frame query가 스캐너 진입도 판단한다. Native Clippy·진단 빌드 PASS, 전체 384쪽이며 221쪽 표 239의 첫 컷 `[1,10,1]`, 222쪽 `[1,10,1]`→`[1,44,1]`, 223쪽 종결로 돌아왔다. 155·156쪽 재출력 및 직접 판독은 97.81975%·90.35099%를 유지했다. source/binary와 명령은 ignored `candidate-9-runtime-provenance.json`, `candidate-9-build.json`, `candidate-native-9-result.json`에 고정했다. 구역 12의 371쪽 행 8 이월 때문에 여전히 +1쪽이며 이 경계와 전쪽·fresh WASM 검증은 계속한다. 쪽 목록을 실제 시각 통과로 바꾸어 보고하지 않는다.

후보 10은 부록 371쪽 행 8의 실제 호출 경로를 대조했다. 기존 저장 쪽 컷의 비이월 빈 밴드 경로는 `end_row_height_override=rest`로 실제 상자를 그리면서, 수용 검사는 그보다 큰 선언 높이로 Center/Bottom 정렬 끝을 계산했다. 같은 경로의 실제 수용 상자를 정렬 검사에도 전달하며, 명시적인 원본 text-frame 경계가 있는 행과 일반 이월 밴드는 본래 공간 계약을 보존한다. Native Clippy·진단 빌드 PASS 후 전체 383쪽으로 돌아왔고 155→156쪽 컷 `[3,4]`와 표 239의 쪽 소속을 유지했다. 독립 PDF 371쪽의 두 단 괘선 CTM 좌표는 ignored `independent-pdf-table-boundaries-landscape-96.json`, 보정 전·후 9의 실제 셀 상자는 `landscape-371-before-cell-frames.json` 및 `landscape-371-candidate9-cell-frames.json`에 있다. 이 후보의 직접 시각 검증·대조군·전쪽 Native/fresh WASM·최종 필수 lint/회귀는 진행 중이며, 쪽수 관측만으로 승인하지 않는다.

후보 10의 추가 Native 검증은 220~224·369~372쪽을 실제 재출력했다. 223쪽 76.8772%·369쪽 84.79807%로 gate FAIL이며, 두 쪽의 위치 차이를 직접 판독했다. 223쪽은 표 종결 뒤 문단이 13.33px 아래로 밀리고, 369쪽은 중첩 표만 가진 문단의 앞 간격 9.33px이 누락됐다. 앞선 383쪽 관측과 선택 155·156쪽 PASS를 전쪽 통과로 승격하지 않는다.

후보 11은 `table_partial.rs`의 객체 전용 문단 경로가 건너뛰던 `cell_paragraph_spacing_before`를 측정과 동일하게 소비하고, 저장 원점이 이미 포함한 간격은 중복 적용하지 않는다. Native Clippy·진단 빌드 PASS, 전체 383쪽 및 155·156쪽 97.81975%·90.35099%를 유지했다. 369쪽은 99.37787%로 개선됐으며 같은 review PNG에서 중첩 표 외곽과 뒤쪽 회색 행을 직접 확인했다. 223쪽은 76.8772%로 미충족이다. `emit.rs`의 종결 흐름 보정만으로는 `layout.rs::layout_partial_table_item`이 별도로 더하는 뒤 간격이 바뀌지 않았음을 실제 render tree로 확인했다. 저장 후속 줄 상자의 간격을 계산하는 공통 결과를 두 소비 지점에 연결하는 후보 12를 검증한다. 해당 결과·source patch·binary SHA·명령은 ignored `candidate-11-runtime-provenance.json`, `candidate-11-source.patch`, `candidate-native-11-boundaries-result.json`과 대응 compare/overlay/review에 보존한다. 전쪽 Native/fresh WASM과 새 source의 필수 회귀는 아직 미검증이다.

후보 12는 원본 후속 문단의 저장 줄 상자를 독립적인 종결 흐름 기준으로 사용한다. `table_layout.rs::native_text_frame_terminal_successor_gap`이 프레임·dirty 상태·문단 줄 정보·세로 기준을 검증하고, `emit.rs`의 두 종결 예약과 `layout.rs::layout_partial_table_item`의 실제 뒤 흐름이 같은 결과를 소비한다. 첫 조각의 원점 수용 여부를 종결 조건의 대용으로 쓰지 않는다. 223쪽 표의 실제 끝과 다음 빈 줄 원점은 483.1px, 주석 줄 원점은 492.5px이며 원본 vpos 25180HU·25880HU와 독립 PDF에 대응한다. 저장 좌표보다 이미 큰 상자는 clamp하지 않고 이 경로를 수용하지 않는다. Native Clippy·진단 빌드 PASS, 전체 383쪽이며 새 재출력의 155·156·223·369쪽은 각각 97.81975%·90.35099%·99.70281%·99.37787%다. 네 review PNG를 직접 열어 표 종결·후속 본문·중첩 표 외곽을 확인했다. 선택 Native 경계의 PASS이며 전쪽 Native/fresh WASM·대조군·필수 회귀의 통과로 승격하지 않는다. 실행은 ignored `candidate-12-runtime-provenance.json`, `candidate-12-source.patch`, `candidate-12-build.json`, `candidate-native-12-result.json`, `candidate-native-12-boundaries-result.json`에 보존한다. 새 rendering 회귀 test/fixture/golden은 아직 추가하지 않았다.

### 수정 전 누적 source의 공통 검증

9개 원 PR을 #7591 → #7611 → #7617 → #7618 → #7619 → #7622 → #7624 → #7628 → #7629 순서로 기능 commit 63개 누적 cherry-pick(-x)했다. 원 저자와 출처를 보존했으며 contributor branch는 rewrite/push하지 않았다. 사용자 지시의 conflict·CI 보정 범위에서 `ba34731730dfa116eaa1b151ee95f4e71b094315` 충돌을 처리했다. #7591의 공통 endnote scratch helper를 보존하고 #7611의 `straddle_row_relief`를 실제 remap 경로에 clone했다. 당시 보정 SHA는 `4e3a97bdbee90c13740adb1ca09b805266887a21`이며 최신 base 재정렬 mapping은 ignored `cherry-pick-state.json` 및 `base-alignment-verification.json`에 있다.

검증 source는 `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563`, 당시 base는 `b3c3047db575d146dff9d39098e6de98c4630b3e`다. 검증 도중 #7651의 CI·문서만 변경된 devel `60efc4b7b380b2a183c20c3ee0ab24a60bc2d832`로 정렬했다. 당시 code head `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`와 source/Cargo/lock/toolchain·시각 실행 도구·입력/기준 PDF tree가 동일함을 Git object로 검사하여 고비용 Rust/WASM/시각 검증을 재사용했다. 새로운 base로 manifest·unit-tier 정책 비교를 다시 통과했다. 이후 조판 보정에는 이 결과를 승계하지 않는다. 원 PR의 CI와 통합 후보의 로컬 검증은 별개이며 원 PR CI 성공으로 새 후보 CI를 합성하지 않는다.

### 실행 결과

공유 `CARGO_TARGET_DIR`와 모든 Cargo `--target-dir`에 기본 clone의 절대 `target/pr-review`를 사용했다. Cargo는 순차 실행하고 전체 nextest는 사용자 지정 10 threads, build jobs 2였다. 기본 증분 정책을 유지하고 cache를 삭제·초기화하지 않았다.

| 검사 | 실제 결과 |
| --- | --- |
| fmt, Native Clippy, wasm32 lib Clippy, workspace build, workspace/all-targets Clippy | 모두 exit 0; 세 Clippy에 `-D warnings` |
| integration manifest prepare/check와 source unit tiers | pinned base 비교 PASS; generated suite는 stage하지 않음 |
| 집중 nextest | 47 PASS, 0 FAIL |
| 전체 nextest | 10,540 PASS, 50 skipped, 37 slow; summary 1851.098s, exit 0 |
| fresh WASM | root `wasm-pack-locked.ps1 --target web --out-dir pkg --mode no-install --no-opt` PASS; Rust 1.93.1/wasm-bindgen 0.2.125 |
| Native Skia lib | test result: ok. 3927 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out; finished in 138.67s; test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s; test result: ok. 165 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s |
| Native Skia missing picture placeholder | Summary [   0.975s] 2 tests run: 2 passed, 212 skipped |
| Native Skia direct PDF | Summary [   0.396s] 4 tests run: 4 passed, 214 skipped |

전체 회귀 재현 명령:

```powershell
$env:CARGO_TARGET_DIR='<기본 clone의 절대 경로>/target/pr-review'
cargo nextest run --locked --cargo-profile release-test --target-dir $env:CARGO_TARGET_DIR --tests --test-threads 10 --no-fail-fast
```

Native Skia lib는 `cargo test --locked --profile release-test --target-dir $env:CARGO_TARGET_DIR --features native-skia --lib -- --test-threads 10`; placeholder/direct PDF는 `node scripts/run-rust-test.mjs <case> -- --cargo-profile release-test --target-dir $env:CARGO_TARGET_DIR --features native-skia --test-threads 10 --no-fail-fast`. Windows Native로 실행했고 macOS/Linux 렌더링 검증은 미실행이다.

### 글꼴과 시각 결과

사용자 지정 교차 호스트에서 실제 legacy KoPub TTF 6개를 읽기 전용으로 확보했고 전송 전후 SHA-256·내부 face 이름을 대조했다. Windows all-users Fonts/HKLM 등록과 새 비상승 GDI 열거에서 돋움·바탕 Light/Medium/Bold 6개를 확인했다. 글꼴 파일은 공개 저장소에 커밋하지 않았다. KoPub World와 동일 face로 간주하지 않았으며 서명/비밀 환경·SSH 정보는 공개 기록에서 제외했다.

| 공개 대조군 | 쪽수 | Native 최저 | fresh WASM 최저 |
| --- | --- | --- | --- |
| float-push | 1 | 99.89324% | 99.89324% |
| column-top | 4 | 99.09774% | 99.09774% |
| masked-vpos | 1 | 99.16989% | 99.16989% |
| square-overlap | 1 | 99.73718% | 99.73718% |
| square-narrow | 1 | 99.65201% | 99.65201% |
| square-paragraph | 1 | 100.0% | 100.0% |
| quotes-retry | 7 | 94.23341% | 94.23341% |
| 편람 HWP 전체 | 383 | **47.54585% / 50쪽 미달** | **47.54585% / 50쪽 미달** |

표의 작은 대조군은 해당 입력 전쪽 점수다. 대표 compare·독립 overlay·review를 열어 표 외곽과 뒤 문단 순서·줄바꿈을 직접 확인했다. 7개 대조군의 Native/WASM 실제 raster bytes도 전쪽 동일했다. AA·색/괘선 굵기와 일부 글자 모양은 다르다. 실루엣 보조값만 생성한 작은 대조군 manifest는 `not_evaluated`이므로 이를 자동 승인 gate라고 기록하지 않는다.

편람은 HWP/PDF 383쪽 전쪽을 Native와 fresh WASM 각각 래스터했다. WASM p154의 Chrome 임시 profile 정리 EPERM은 host 오류였고 같은 해시의 SVG·WASM을 유지하여 p154–383을 정상 exit 0으로 다시 raster했다. p1–153은 원 완료 raster와 해시로 구별 보존했다. `--png-pair` 전383쪽 비교도 `re_review_required`로 exit 1이며 host 실패가 시각 실패의 근거가 아니다. Native p156의 full-font 재출력도 47.58269%로 같은 geometry 차이를 보였고, 전체 기본 system-font 출력 47.54585%와 별도 capture로 구별한다.

입력 [편람 HWP](<../../../samples/2025 행정업무운영 편람(최종).hwp>) SHA-256 `40d6d05eac4d55bdc4b0c62c42d93af104d5123b447581246f36fd15de7bd46f`, [독립 KoPub 2024 PDF](<../../../pdf/2025 행정업무운영 편람(최종)-hwp-kopub-2024.pdf>) SHA-256 `3cdf329004cffad0ebacd122586e5e1702aff36b3015cbcb401d038857019593`.

![Native 편람 155쪽 비교·overlay](../assets/planet6897_20261008/native_handbook_p155_review.png)

![Native 편람 156쪽 비교·overlay](../assets/planet6897_20261008/native_handbook_p156_review.png)

![fresh WASM 편람 155쪽 비교·overlay](../assets/planet6897_20261008/wasm_handbook_p155_review.png)

![fresh WASM 편람 156쪽 비교·overlay](../assets/planet6897_20261008/wasm_handbook_p156_review.png)

48개 공개 pagination 입력의 Native 쪽수는 42개가 기존 지정 PDF와 같고 6개가 달랐다. issue1891 73/70, issue1937 51/50, issue5877 32/31, task1718 47/48, task2287 416/415, issue2430 39/40 (Native/PDF). 마지막 것은 옛 비KoPub 40쪽 기준과 새 KoPub 39쪽 기준의 환경 차이다. 새 PDF와의 39쪽 일치도 전쪽 시각 PASS가 아니다. 쪽수 조회는 `debug/rhwp-q-page-items.exe --page 0 --json`의 계산이며 3631쪽 전부의 Native/WASM Visual Sweep이 아니다. base 비교를 실행하지 않아 나머지 차이도 새 회귀로 확정하지 않는다.

비공개 corpus는 지정 로컬 기준 위치에서 43개 문서 식별자를 조사하여 42개 원본을 찾았지만 대응 PDF는 0개였다. 파일명·개인 경로·원본은 공개하지 않고 그 범위는 미검증이다. 원 PR의 before/after·저자 TSV와 실제 이 통합 head의 검증을 혼동하지 않는다. 편집 후 저장본의 Print 출력과 원래 저장본은 별도 계약이며 한 경로의 결과로 다른 경로를 면제하지 않는다.

### 보존·재현·제출 조건

실행 명령·exit·검증 base/head·입력/PDF/WASM/binary 해시·전체 TSV·진단 JSON은 ignored `output/pr-review/planet6897-20261008/`의 `validation-current.json`, `post-lint-validation.json`, `fresh-wasm-package-hashes.json`, 각 `silhouette_manifest.json`, `wasm-handbook-raster-recovery.json`, `pagination-native-page-counts.json`에 보존했다. score-only capture의 해시와 실제 PNG 조립의 원본 해시를 재확인하고 canonical compare/overlay/review 함수로 아래 대표 PNG를 만들었다. helper 스크립트는 사용자 Downloads에 보존한다. 원 PR의 커밋된 HWP/HWPX·Print PDF·증적은 그대로 보존했다. 새 rendering 회귀 test/fixture/golden이나 ratchet 완화는 추가하지 않았다.

추가 제출 전 해당 경계의 독립 원본·Print PDF를 공급하여 본문/개체 소속과 geometry를 개선하고, 새 source head에서 영향 경계·필수 검증·Native/fresh WASM gate를 갱신해야 한다. 기준은 [시각 fixture 절차](../../manual/pr_review/visual_fixture_evidence.md)와 [Visual Sweep](../../manual/verification/visual_sweep_guide.md)의 **"이 상태에서는 새 PR을 만들지 않고 이미 열린 PR은 승인·통합하지 않는다"**다. 현재는 시각 미충족 및 증거 부족 때문에 통합 PR 생성·원 PR 승인/close·merge를 수행하지 않았다. 공통 머지 보류는 9개 PR 각각이 편람 결함을 새로 만들었다는 뜻이 아니다. 코드가 없는 문서 전용 기록 PR으로 통합 검토 기록을 따로 보내지 않는다.

머지 후 comment 계획은 gate 통과 시점의 최종 통합 PR·merge SHA·CI URL·같은 대표 이미지의 merge-SHA raw URL·남은 차이를 각 원 PR에 한국어로 남기고 통합 PR 링크와 함께 종료하는 것이다. 현재 머지되지 않았으므로 merge SHA·issue 종료·duration refresh를 완료로 기록하지 않는다. contributor branch와 공용 cache는 보존한다.

| 대표 PNG | SHA-256 |
| --- | --- |
| `native_float-push_p001_review.png` | `b8f2b6cdc3ec0f5df7df44b0f59a1d850096cb07a0ae55116acc79852cd3571c` |
| `native_column-top_p002_review.png` | `2daa52c2bbba6a1937723918f1c65704117656919f3dfb9d85aa9de7c0dcc220` |
| `native_masked-vpos_p001_review.png` | `5cabed1c92e15588a85447c8414628b50b4706db917c3f592b9ba7bf52c79c34` |
| `native_square-overlap_p001_review.png` | `bc8cfdc99977e621c433c2f32c5c2a287022e71b9a02f6c05a9d62a83ecfda42` |
| `native_square-narrow_p001_review.png` | `500fff50dc7b436e0be18498440bb22bdf0f8eff907137c6b1e1a8da74884610` |
| `native_square-paragraph_p001_review.png` | `86fc5409917664ab4011a9c3cfb8123b93bf856df0f17748fe463418023a8a10` |
| `native_quotes-retry_p003_review.png` | `04227b11a2dfdead39335d2618177fd85a81e3340b7dd35831cf4b3e6f00f453` |
| `native_handbook_p155_review.png` | `b320fe187b37a1260d1fe05a5209d8c9b805bb0300e8067baa3a3e5bce7c73ce` |
| `native_handbook_p156_review.png` | `298d909e91f348e73fd2cbf2c7b97a3ffef027977dc14b069001706358747607` |
| `native_handbook_p377_review.png` | `bfacb279d2f729c2eb571d6d9b03bceefa0549443309ba9853617c3b38ff6aa0` |
| `native_handbook_p378_review.png` | `12745cdfb9396127df57a9ab45939b881e07462eb7b8e78774aceb98a9793fb7` |
| `native_handbook_p156_overlay.png` | `c4f9717bfcf4fecea307e7a0ce995931dfd36407b332bae808130e37356eae87` |
| `wasm_float-push_p001_review.png` | `b8f2b6cdc3ec0f5df7df44b0f59a1d850096cb07a0ae55116acc79852cd3571c` |
| `wasm_column-top_p002_review.png` | `2daa52c2bbba6a1937723918f1c65704117656919f3dfb9d85aa9de7c0dcc220` |
| `wasm_masked-vpos_p001_review.png` | `5cabed1c92e15588a85447c8414628b50b4706db917c3f592b9ba7bf52c79c34` |
| `wasm_square-overlap_p001_review.png` | `bc8cfdc99977e621c433c2f32c5c2a287022e71b9a02f6c05a9d62a83ecfda42` |
| `wasm_square-narrow_p001_review.png` | `500fff50dc7b436e0be18498440bb22bdf0f8eff907137c6b1e1a8da74884610` |
| `wasm_square-paragraph_p001_review.png` | `86fc5409917664ab4011a9c3cfb8123b93bf856df0f17748fe463418023a8a10` |
| `wasm_quotes-retry_p003_review.png` | `04227b11a2dfdead39335d2618177fd85a81e3340b7dd35831cf4b3e6f00f453` |
| `wasm_handbook_p155_review.png` | `b320fe187b37a1260d1fe05a5209d8c9b805bb0300e8067baa3a3e5bce7c73ce` |
| `wasm_handbook_p156_review.png` | `298d909e91f348e73fd2cbf2c7b97a3ffef027977dc14b069001706358747607` |
| `wasm_handbook_p377_review.png` | `bfacb279d2f729c2eb571d6d9b03bceefa0549443309ba9853617c3b38ff6aa0` |
| `wasm_handbook_p378_review.png` | `12745cdfb9396127df57a9ab45939b881e07462eb7b8e78774aceb98a9793fb7` |
| `wasm_handbook_p156_overlay.png` | `c4f9717bfcf4fecea307e7a0ce995931dfd36407b332bae808130e37356eae87` |

초기 따옴표 캡처의 30초 Navigation timeout은 거대한 full-font SVG를 로드하는 host 오류였다. 같은 source에서 120초 raster timeout으로 Native/fresh WASM 각각 7쪽을 재실행하여 모두 exit 0, 최저 94.23341%를 확인했다. 원 실패 로그를 삭제하거나 렌더링 회귀로 분류하지 않았으며 `quotes-timeout-retry-results.json`에 구별한다.

### 공개 대조군 입력·기준 해시

| 입력 | 커밋된 경로 | SHA-256 |
| --- | --- | --- |
| float-push / hwp | [samples/float_push_spacing_before/float_push_spacing_before.hwp](<../../../samples/float_push_spacing_before/float_push_spacing_before.hwp>) | `f4404432d153d5cd2de073f90a1f35ba366e885da702331e5b7d39755064bafd` |
| float-push / pdf | [pdf/float_push_spacing_before/float_push_spacing_before-2020.pdf](<../../../pdf/float_push_spacing_before/float_push_spacing_before-2020.pdf>) | `8243af2cac393e00c2830241cf64116d2bff3166d093b7216e86cd4df69d4709` |
| column-top / hwp | [samples/float_push_spacing_before/float_push_spacing_before_column_top.hwp](<../../../samples/float_push_spacing_before/float_push_spacing_before_column_top.hwp>) | `c2d0d80f5a7fdf66e06fc0d473cb754260ee442414b6d56b97bf845a07987dad` |
| column-top / pdf | [pdf/float_push_spacing_before/float_push_spacing_before_column_top-2020.pdf](<../../../pdf/float_push_spacing_before/float_push_spacing_before_column_top-2020.pdf>) | `4695f998ebc4ce3ca5e7897ee085fc1f0fcc55d3d3c03b2863d89b1997f306f1` |
| masked-vpos / hwp | [samples/masked_stale_vpos/masked_stale_vpos.hwpx](<../../../samples/masked_stale_vpos/masked_stale_vpos.hwpx>) | `4b18c3ff777d8c2acd04d8049470026f5308e188a3f895fe4a9da6af218ac4ca` |
| masked-vpos / pdf | [pdf/masked_stale_vpos/masked_stale_vpos-2020.pdf](<../../../pdf/masked_stale_vpos/masked_stale_vpos-2020.pdf>) | `98f6781dc7e681c2b6b4887f950ea275acfe101f7fba68279ca6d2bf5e777a05` |
| square-overlap / hwp | [samples/page_anchored_square/page_anchored_square_reflow_allow_overlap.hwpx](<../../../samples/page_anchored_square/page_anchored_square_reflow_allow_overlap.hwpx>) | `fe8d2341e919f23978750d936129bb987a0e488f1392f5459578144144018791` |
| square-overlap / pdf | [pdf/page_anchored_square/page_anchored_square_reflow_allow_overlap-2020.pdf](<../../../pdf/page_anchored_square/page_anchored_square_reflow_allow_overlap-2020.pdf>) | `1967b0396b0fb69875ea4b2e21ac7b68030265646cbdb34cfc1b8db2655b9149` |
| square-narrow / hwp | [samples/page_anchored_square/page_anchored_square_reflow_narrow.hwpx](<../../../samples/page_anchored_square/page_anchored_square_reflow_narrow.hwpx>) | `2454813ec3a22323bc5a7e4a20fcea3670478e62a88b6318a13ff15aedbb3923` |
| square-narrow / pdf | [pdf/page_anchored_square/page_anchored_square_reflow_narrow-2020.pdf](<../../../pdf/page_anchored_square/page_anchored_square_reflow_narrow-2020.pdf>) | `5f99b5d6f1a31e06ad7d9d4c271fb7d907ebe11f13f909881fb046b3c504d2c5` |
| square-paragraph / hwp | [samples/para_square_lane/para_square_lane_reflow.hwpx](<../../../samples/para_square_lane/para_square_lane_reflow.hwpx>) | `ec69ea339b29243b5769c73f3df29b1a23f68c8c9f9b1bc180cd3cfa1bac1900` |
| square-paragraph / pdf | [pdf/para_square_lane/para_square_lane_reflow-2020.pdf](<../../../pdf/para_square_lane/para_square_lane_reflow-2020.pdf>) | `89f93b32457a0774e733ecc9f3eef578ab9269f73481cb0726a72959be504666` |
| quotes-retry / hwp | [samples/basic/treatise sample.hwp](<../../../samples/basic/treatise sample.hwp>) | `1aec0ca0a958d24199591b96916a2535d04f1c654f60dda1027ecbaf4839f828` |
| quotes-retry / pdf | [pdf/basic/treatise sample-2022.pdf](<../../../pdf/basic/treatise sample-2022.pdf>) | `2b5bfa4900f0f82adf880d68130f18c1d4055d600b0767f6c13e749139430e94` |

### 후속 전쪽·경계 확인

후보 12를 커밋한 `6ae28a8ea77f8a7d4426634a8dd89331893f2dd3`의 Native 전쪽 산출은 383쪽 누락 없이 완료했지만 22쪽이 90% 미만이며 최저는 307쪽 48.38365%다. 기존 후보의 50쪽 미달에서 줄어든 결과이고 전체 gate는 여전히 FAIL이다. 전체 목록·TSV·실행 결과는 ignored `source-6ae28-native-full-summary.json`, `candidate-native-12-full-scores-result.json` 및 `candidate-native-scores/handbook-native-candidate-12-full-scores/`에 보존한다. 선택 네 쪽 통과를 전쪽 통과로 보고하지 않는다.

같은 source의 fresh WASM 빌드는 성공했으며 root `pkg/`와 Studio `public/`의 WASM SHA-256은 `ec6cc7290742b57fe5e30047e08af7be4f8d5814535ed41a56c6441903af5bf8`이다. Windows wrapper에 자동 동기화가 없어 초기 확인 스크립트의 동기화 assertion이 실패했고, 빌드 성공본을 정확히 복사한 뒤 두 경로의 바이트 일치를 확인했다. 브라우저 WASM의 383쪽 raw SVG·tree를 실제 산출했다. 155·156·223쪽 tree는 Native와 동일하며 369쪽의 네 차이는 header 문단의 `usize::MAX`가 32/64bit에서 다르게 직렬화된 값뿐이고 좌표·내용은 동일하다. 이 raw 비교는 전쪽 WASM 래스터·시각 gate 통과가 아니다. `source-6ae28-fresh-wasm.json`, `source-6ae28-fresh-wasm-raw.json`, 두 tree 대조 JSON에 성공과 초기 확인 실패를 구별해 기록했다. 이 source의 Native·WASM lib Clippy 두 단계는 별도 review worktree에서 PASS했으며 workspace/all-targets·새 전체 회귀는 아직 미완료다.

후보 13은 저장 다음 줄이 완전한 Square 표 상자를 닫는 경우를 기존 공통 프레임에 연결했다. Native Clippy·진단 빌드 PASS, 383쪽을 유지했으며 48~51쪽은 각각 99.87052%·99.91768%·99.86297%·99.60882%다. 47쪽은 86.71117%로 FAIL이며 실제 PNG와 tree에서 뒤 빈 줄·두 번째 표의 세로 위치 차이를 확인했다. 후보 14는 완전한 표의 실제 배치 후에도 같은 후속 줄 원점을 소비하고, 글 앞/뒤 중첩 표를 같은 문단의 겹침 개체로 분류해 적층 가산을 제거했다. Native Clippy·진단 빌드 PASS, 383쪽 및 155·156쪽 97.81975%·90.35099%를 유지했다. 경계 비교의 47쪽 91.7358%·85쪽 98.8317%는 직접 판독했지만 47쪽 두 번째 표에는 앞 간격 9.33px의 위치 차이가 남는다. 86쪽 67.68623%·87쪽 83.50097%로 경계 전체 gate는 FAIL이다. 단계별 source/binary 해시·patch·명령·결과는 ignored `candidate-{13,14}-runtime-provenance.json`, 대응 build/result JSON·compare/overlay/review에 보존한다. 진단용 로그 코드는 최종 source에서 제거했다. 새 회귀/fixture/golden·통합 PR 제출·승인은 계속 보류하며 이후 후보의 결과를 이 기록에 소급하지 않는다.

후보 15는 직인 표의 문단 원점·부호 있는 오프셋·바깥여백을 `nested_overlay_outer_frame`으로 공유했다. 저장 첫 줄의 앞 간격은 `nested_overlay_stored_line_lead`로 측정/정렬에서 한 번 차감하며, 실제 중첩 표 배치도 같은 프레임의 시작점을 소비한다. Native Clippy·진단 빌드 PASS, 383쪽 및 155·156쪽 값을 유지했다. 47·85·86·87쪽은 각각 91.7358%·99.19562%·96.48817%·98.3841%이며 86·87쪽 PNG를 직접 판독했다. 본문·하단 정보 위치는 복원됐지만 기관장 표제의 작은 세로 차이와 47쪽 앞 간격 차이는 남아 있다. 경계 통과를 전쪽 완료로 승격하지 않는다. 해시/patch/build/visual 결과는 ignored `candidate-15-runtime-provenance.json` 및 대응 파일에 보존한다.

후보 16의 진단은 분할 표의 첫 호스트가 실제 글줄을 그리는 경로만 앞 간격을 소유하도록 측정된 커서와 paint의 판별을 공유한다. 또한 저장 표 프레임의 초과 허용으로 온전한 행을 소비하기 전에 실제 내용·정렬 끝이 행 예산에 들어가는지 확인한다. `SourceFrameQuery::resolve → row_complete_cut_content_height/row_aligned_content_bottom → row cut → layout_partial_table`을 대조했다. Native Clippy·진단 빌드 PASS, 383쪽을 유지했으며 306쪽 마지막 행의 내용 컷 `[1,1,4]`가 307쪽의 시작 컷으로 이어진다. 실제 시각 비교는 진행 중이며 이 구조 확인만으로 통과 판정하지 않는다.

135쪽의 다음 진단은 원 HWP5 저장본의 1×1 TAC 표를 대상으로 한다. 독립 한컴 PDF는 표 상자가 약 465px이고 본문이 상단에서 시작하지만 후보 12는 약 740px 상자를 가운데 정렬한다. 원본의 마지막 줄 끝 `34626 HU`와 실효 안 여백 `282 HU`가 개체 높이 `34908 HU`를 정확히 닫고, 호스트 줄 높이 `35188 HU`는 바깥여백 `280 HU`를 추가한 값이다. 그림이 소유한 두 번째 저장 줄은 `5600 HU`에서 시작해 `21126 HU` 그림을 담으며 앞 장식의 저장 줄과 물리적으로 겹친다. 각 줄 높이를 적층 합산하는 측정은 이 띠를 중복한다. 새 공통 프레임 후보는 단조·미편집 저장 줄, 실제 UTF-16 컨트롤 소유, 모든 개체의 프레임 내 포함, 마지막 줄과 개체 프레임의 일치로 유효성을 확인한다. `stored_inline_content_frame → HeightMeasurer content/required height → resolve_row_heights → 셀 정렬/저장 원점`에서 같은 결과를 소비하며, 편집·재조판·쪽 리셋·불완전 분할 프레임은 제외한다. 아직 빌드·시각·반례 검증 전이므로 미검증이며 새 회귀/fixture/golden을 추가하지 않았다.

후보 16의 Native 경계 비교는 47·85·86·87·306·307쪽 모두 gate PASS이며 각각 99.9051%·99.19562%·96.48817%·98.3841%·93.1853%·98.64204%다. 47·306·307쪽 PNG를 직접 판독했다. 47쪽 앞 간격의 중복은 제거됐고 307쪽에는 앞 쪽에서 누락되던 답변 세 줄이 표시된다. 306쪽 마지막 질문 상자의 시작 위치에는 작은 차이가 남아 있어 완전 일치를 주장하지 않는다. 기존 #5057 Native 13쪽·#6123 14쪽·#5593 1쪽을 실제 진단 runtime으로 확인했고, 이 count 진단은 기존 정식 검사·직접 시각 반례 검증을 대체하지 않는다. 결과는 ignored `candidate-native-16-boundaries-result.json`, `candidate-16-counter-pages.json` 및 대응 페이지별 overlay/review에 보존한다. 전쪽 Native/fresh WASM gate는 아직 미검증이다.

후보 17는 Native Clippy·진단 빌드 PASS, 383쪽을 유지했고 135쪽 표 높이를 독립 저장 프레임과 같은 465.4px로 복원했다. 직접 PNG에서 표 외곽·첫 문단·그림 뒤 문단은 기준 위치로 돌아왔지만 그림만 약 6.67px 위·왼쪽에 남았다. 실제 tree의 저장 소유 두 번째 줄과 TAC 그림 폴백 배치를 추적하니, 폴백이 문단 앞 간격을 뺀 원점을 다시 사용하고 줄의 가로 원점을 소비하지 않았다. 135쪽은 88.32656%로 여전히 FAIL이며 126쪽 71.56523%·130쪽 89.69246%도 그대로다. 이 결과는 ignored `candidate-17-runtime-provenance.json`, `candidate-native-17-result.json`과 대응 PNG/tree에 보존했다. 다음 후보는 같은 완전한 셀 프레임의 실제 줄 원점과 UTF-16 개체 소유를 글줄·그림이 함께 소비하도록 연결하며, 세로 사다리 적용을 가로 들여쓰기 규칙과 분리한다. 그림은 이미 방출한 소유 줄의 상자를 소비하고, 같은 줄의 앞 그림 폭만 추가한다. 아직 다음 후보의 실제 검증 전이다.

130쪽 원본 그림은 선 종류 1·검정·34 HU 테두리가 선언돼 있고 한컴 PDF에서도 바깥 개체 프레임의 선이 보이지만, 후보 17 tree에는 해당 테두리가 없다. TAC 직접 ImageNode 방출 세 경로와 본문 폴백 경로가 기존 `render_picture_border`를 거치지 않았음을 확인했다. 후보 18은 Native Clippy·진단 빌드 PASS이며 인라인·부동 그림이 같은 `make_picture_border_node`와 바깥 프레임/변환을 소비하도록 연결했다. crop/안쪽 여백은 bitmap 내부에만 적용하며 선을 그리기 위해 그림·본문 높이를 변경하지 않는다. 실제 비교는 진행 중이고 회전/대조군·최종 Native Skia/fresh WASM은 아직 미검증이다.

과거 source 90·후보 12의 전쪽 SVG 383개씩은 NTFS 무손실 압축으로 모두 보존했다. 삭제·이동 없이 약 2GB를 확보했고 두 묶음의 첫/중간/마지막 파일 SHA-256이 압축 전후 동일함을 확인했다. 기존 압축 archive와 삭제 거부 기록도 그대로 유지한다. ignored `historical-svg-ntfs-compression.json`에 명령 결과·파일 수·논리/저장 byte 수·대표 해시를 기록했으며 source/증적 내용을 바꾸지 않았다.

후보 18의 실제 비교에서 130쪽은 94.00168%로 PASS이고 PNG에서 그림 바깥 테두리가 독립 PDF와 같은 프레임에 복원됨을 확인했다. 작은 bitmap 명암 차이는 남으며 완전 일치를 주장하지 않는다. 126쪽 71.56523%·135쪽 88.32656%는 그대로 FAIL이다. 이후 후보 19는 완전한 셀 프레임에서 각 글줄의 저장 세로 원점을 명시적으로 전달하고, TAC 그림 폴백이 이미 방출한 소유 TextLine의 상자를 사용하도록 연결했다. Native Clippy·진단 빌드는 PASS했고 실제 비교는 진행 중이다. 이 경로는 문단 들여쓰기를 없애는 기존 `physical_frame_rows`와 분리했으며 같은 줄의 여러 그림은 UTF-16 소유와 앞 그림의 실제 점유 폭을 유지한다. 편집·재조판·불완전 프레임은 기존 경로를 유지한다.

126쪽에서는 원본 `2.991`의 저장 줄 끝 `2040 + 24430 + 880 = 27350 HU`가 바로 다음 빈 줄을 닫지만, TAC 표 앞 직인 표를 가시 컨트롤로 분류해 표 소유 줄 사영/후행 간격을 버린다. 후보 20은 공통 `nested_table_is_overlay`로 흐름 중립 표를 판별하며 실제 저장 다음 줄과 완전히 닫히는 경우에만 기존 `tac_next_line_full_spacing`을 소비한다. 그림·도형의 기존 흐름 참여 대조군은 유지하고 근거 없는 간격 전량 적용을 하지 않는다. `composer::tac_next_line_full_spacing → typeset/controls/stored_tac의 소유 줄 예약 → layout_table_item의 같은 소유 줄 후행 간격`을 대조했으며 이 후보는 아직 빌드·실행 전이다.

후속 실제 호출 경로 대조에서 위의 후보 20 생산 경로를 구체화했다. `stored_tac::prepare`의 단일 저장 개체 단축은 `controls=[Table]`만 수용하므로 직인과 본문 표가 함께 있는 `2.991`에는 적용되지 않는다. 이 사례의 일반 TAC 수용은 `typeset::format_table → controls::reconcile_tac_height → tac_reconcile::measure/effective_cap/capped_bottom`이며 물리 쪽나눔 예산에는 기존 후행 간격 절반 계약을 유지한다. 다음 저장 줄의 실제 원점에는 독립적으로 인증한 전량 간격을 `layout_table_item → tac_next_line_full_spacing`에서 소비한다. 분할 수용 높이와 다음 줄 원점은 서로 다른 계약으로 기록하고, 이름이 같은 helper가 있다는 이유로 이 입력이 저장 단축을 탔다고 주장하지 않는다. 실제 출력으로 원점·테두리·다음 내용과 대조군을 확인할 때까지 후보 20은 미검증이다.

후보 19의 실제 Native 비교에서 135쪽은 99.49018%, 130쪽은 94.00168%로 PASS했다. 135쪽 review PNG를 직접 판독하여 그림과 저장 소유 줄의 원점, 표 외곽, 뒤 문단의 위치가 복원됐음을 확인했다. 후보 20은 Native Clippy·진단 빌드를 통과하고 383쪽을 유지했지만, 126쪽은 71.56523%로 개선되지 않았다. 따라서 해당 보정을 성공으로 기록하지 않는다. 원본 parser 진단에서 직인 표 `2.991`·`2.994`의 `flow_with_text`는 false이며, 이 속성은 흐름 참여가 아니라 쪽 영역 제한이다. [한컴 표 위치 도움말](https://help.hancom.com/hoffice130/ko-KR/Hwp/table/tableattribute/table%28position%29.htm)과 저장소 사양 정오표 31절은 글 앞·뒤 배치 역할과 이 제한을 구분한다. 후보 21은 공통 overlay 역할 판정에서 이 잘못된 전제만 제거하고, 완전한 저장 셀 프레임은 절대 크기 단위일 때만 수용하며, 같은 줄 그림들의 가로 점유 합산을 부호 있는 정수의 덧셈 없이 수행한다. 후보 21의 최종 시각·회귀 판정은 실행 결과를 기다린다. 후보 19·20의 source/binary 해시·patch·383쪽 소속·페이지별 PNG·명령·결과는 ignored `candidate-{19,20}-runtime-provenance.json`, `candidate-native-{19,20}-result.json` 및 대응 visual 폴더에 보존한다.

후보 21의 Native fmt·Clippy·진단 빌드가 모두 PASS했고 실제 쪽수는 383쪽이다. 126·130·135쪽은 각각 99.97565%·94.00168%·99.49018%로 gate PASS다. 126쪽 review PNG를 직접 판독하여 두 TAC 표의 외곽과 직인, 표 뒤 빈 줄과 제목의 소속·간격이 독립 PDF 위치로 복원됐음을 확인했다. 작은 획·명암 차이는 남는다. 이 결과는 선택 세 쪽의 Native 검증이며, 최신 source의 전쪽 Native/fresh WASM gate와 기존 회귀 검사는 아직 완료하지 않았다. 명령·결과·source/binary 해시·patch·전체 쪽 소속 및 직접 판독한 PNG는 ignored `candidate-21-build.json`, `candidate-21-runtime-provenance.json`, `candidate-native-21-result.json`과 `candidate-native-reviews/handbook-native-candidate-21-content-frame/`에 보존한다. 이후 원 미달 페이지와 분할 경계의 최신 source 재출력을 계속한다.

최신 후보 21 SVG에서 140·373·375쪽의 선행 비교를 따로 산출했고 각각 68.34052%·79.00432%·74.48182%로 FAIL이다. 이 비교는 같은 입력과 독립 PDF의 변경 없는 기준 PNG를 재사용하고 Native만 최신 source로 재출력한 진단이며 전쪽 판정이 아니다. 375쪽의 실제 source trace는 `prev_pi=32 → pi=41`에서 지연 기준 `19280 HU`를 만들며, 원본 `36332 HU`의 본문을 약 257.07px 위로 옮긴다. 같은 쪽 왼쪽 단의 측정 점유는 97.2px, 독립 저장 점유는 554.82667px다. 원본 `12.18 → 12.19..31 → 12.32 → 12.33..40 → 12.41`은 빈 줄의 높이·후행 간격으로 닫히는 실제 저장 사다리다. 글 앞 표의 흐름 중립을 빈 호스트와 뒤 빈 줄의 공간 부재로 대신한 두 분기를 확인했다. `candidate-21-p375-vpos-debug.txt`, 최신 tree 및 `handbook-native-candidate-21-early-remaining/early-current-source-panels.json`에 source·SVG·기준 PNG 해시와 실제 관측값을 보존한다.

후보 22는 유효한 저장 줄의 독립적인 다음 줄 닫힘으로 `stored_overlay_host_flow_line`을 만들고, 같은 연속 사다리의 `stored_overlay_guide_owns_flow`를 조회한다. 실제 호출 대조에서 빈 호스트의 점유는 기존 `decoration_host::stored_decoration_host_line_advance_hu → ParagraphFragment → commit_decoration_host_text`가 이미 원본 줄로 예약함을 확인했다. 따라서 잠정적인 formatter·context 중복 계산을 제거했다. `section::entry`에서 증명된 뒤 빈 줄을 흐름에 보존하고, `place_fitted_paragraph → stored_overlay_guide_advance → apply_full_paragraph_flow`는 수용한 빈 줄의 전체 구성 높이를 소비한다. fit의 후행 간격 제외를 누적 흐름의 공간 부재로 대신하지 않는다. 이 조회는 한 개의 유효한 빈 저장 줄을 요구하므로 줄 수 0의 분기와 그림 재조판 분기에는 적용하지 않으며, 정상 분할의 줄별 수용은 기존 `line_advance` 결과를 유지한다. 실제 배치는 같은 원본 호스트 줄로 단 원점을 세우고, 흐름 중립 Shape 항목 때문에 그 원점을 삭제하지 않는다. 원본 HWP5·미편집 경로만 적용하며 합성·dirty 줄, 단/쪽 재시작, 불연속 사다리, 글자취급·자리차지 표는 기존 경로를 사용한다. 이 후보는 구현 중이며 빌드·시각·반례 검증 전이다. 진행 중인 기존 회귀는 별도 worktree의 커밋 `1a84aa4760e2a83a899aaa4724a041e0d738bfc0`에 고정했으며 후보 22의 결과로 재사용하지 않는다.

커밋 `1a84aa4760e2a83a899aaa4724a041e0d738bfc0`의 기존 경계 회귀는 Windows 별도 review worktree에서 공유 절대 `target/pr-review`, `--locked --cargo-profile release-test --test-threads 10 --no-fail-fast`로 10개 suite 중 27개를 선택해 전부 PASS했다. nextest summary는 27 PASS / 2233 선택 제외이며 전체 회귀 통과로 보고하지 않는다. 빌드 포함 1676.12초, 실제 검사 0.923초다. fmt·Native Clippy·진단 빌드와 같이 source 해시를 기록했고, 이 검사는 뒤 후보 22의 결과가 아니다. 같은 커밋의 Native 선택 27쪽은 누락 없이 산출했지만 140·154·241·262·277·287·289·290·373·375쪽이 90% 미만이다. 최저 140쪽 68.34052%로 FAIL이며 나머지 17쪽의 PASS는 이 보류를 해소하지 않는다. ignored `source-1a84aa-focused.json`, `source-1a84aa-runtime-mapping.json`, `candidate-native-21-remaining-boundaries-result.json` 및 대응 전쪽 소속·PNG·tree·overlay에 보존한다.

후보 22의 Windows Native fmt·Clippy·진단 빌드는 PASS했고 실제 쪽수 383쪽을 유지했다. 126·374·375·376쪽은 각각 99.97565%·99.41021%·92.50825%·97.77218%로 선택 범위 gate PASS다. 최신 374~376쪽 review PNG를 직접 판독하여 375쪽의 직인이 발신명의 줄로 돌아오고 서식 밖 비고가 표 아래에 표시됨을 확인했다. 인접 쪽의 표 외곽·내용 소속도 유지된다. 명암·일부 작은 선 위치 차이는 남는다. 양쪽 단의 사용 높이는 저장 끝보다 각각 14.9333px 짧으며, 기존 두 가시 본문 문단의 다단 fit 계약이 후행 간격을 제외하는 경로는 이번 빈 안내 줄 보정에서 바꾸지 않았다. 선택 네 쪽 통과를 전쪽 Native/fresh WASM 통과로 보고하지 않는다. source/binary 해시·patch·빌드·383쪽 소속·명령·PNG/tree/overlay는 ignored `candidate-22-build.json`, `candidate-22-runtime-provenance.json`, `candidate-native-22-result.json` 및 `candidate-native-reviews/handbook-native-candidate-22-overlay-guides/`에 보존한다.

140쪽은 원본 셀 문단의 네 공백과 문단 여백 뒤에 TAC Group이 이어진다. 독립 Print PDF의 두 bitmap은 동일한 가로 원점 127.422pt(96dpi에서 169.896px)를 사용한다. 후보 21 tree의 글줄 원점 143.8px과 앞 공백 폭 24.6px은 이 위치를 설명하지만 Group은 셀 왼쪽 117.1px에 그려진다. 실제 호출 대조는 `paragraph_layout의 run_tacs → set_inline_shape_position → table_partial의 TAC Shape → layout_cell_shape`이며, 분할 표의 마지막 소비가 등록 위치를 무시하고 순차 셀 원점을 사용한다. 후보 23은 온전한 셀과 동일한 cell context로 등록 위치를 소비하며 등록 없는 폴백만 기존 배치를 유지한다. 그룹 crop의 bitmap 원점과 보이는 clip 상자는 구분하고 세로 crop 차이를 임의 오프셋으로 보정하지 않는다. 이 후보는 빌드·시각 확인 전이며, 표 마지막의 남은 물리 높이 차이는 별도 미해결로 유지한다.

후속 폰트 바이트 대조에서 기존 KoPub 6 face의 시스템 등록과 SVG 공급을 구별해야 함을 확인했다. Windows Fonts의 `KoPub Dotum Bold.ttf` 등은 원본 SHA와 이름 테이블이 검증된 설치본이지만, SVG 파일 조회는 한글 face 이름에서 이 영문 배포 파일명을 만들지 못한다. 실제 후보 21의 `KoPub돋움체 Bold`·`Medium` data URI는 Noto Sans KR ExtraLight를 담고 있었다. 따라서 시스템 등록 확인을 최신 SVG의 정확한 폰트 공급 증거로 사용한 기존 판단을 바로잡는다. 후보 24는 원 name table의 한글/영문 기존 KoPub face 6종을 실제 배포 파일명에 연결하고, 명시적 Bold face의 weight 선언에도 같은 파일을 공급한다. 다른 OS에서도 동일 조회를 사용하며 KoPub World를 이 face의 별칭으로 넣지 않는다. glyph·advance·문서 스타일을 바꾸지 않고 원본 font bytes로 재출력하여 내장 이름/해시와 시각 결과를 검증한다. 앞의 Noto 공급 캡처는 그대로 보존하되 정확한 KoPub 출력 통과로 승격하지 않는다. 후보 24는 아직 빌드·실제 검증 전이다.

후보 23은 Windows Native fmt·Clippy·진단 빌드를 모두 통과하고 383쪽을 유지했다. 140쪽 Group 원점이 117.1px에서 글줄의 예약 위치 168.4px로 바뀌었고 선택 비교는 97.39256%로 PASS다. 최신 review PNG에서 앞 공백 뒤 이미지 배치와 뒤 본문 순서를 직접 확인했다. 표 하단의 물리 공간 차이는 그대로이며, 이 성공을 표 높이 완전 일치로 보고하지 않는다. 154쪽 87.53696%·373쪽 79.00432%는 그대로 FAIL, 375쪽은 92.50825%를 유지한다. 후보 23의 source/binary 해시·patch·빌드·383쪽 소속·명령·PNG/tree/overlay는 ignored `candidate-23-build.json`, `candidate-23-runtime-provenance.json`, `candidate-native-23-result.json` 및 `candidate-native-reviews/handbook-native-candidate-23-partial-inline-shape/`에 보존한다. 이 비교의 KoPub돋움체 data URI는 아직 대체 font를 포함하므로 정확한 KoPub byte 공급 후보 24로 다시 출력한다.

후보 24의 Windows Native fmt·Clippy·진단 빌드는 PASS했고 383쪽을 유지했다. 선택 10쪽 SVG의 KoPub 선언 45개를 직접 디코딩해 각각 요청 family/weight의 PostScript name과 legacy 원본 SHA-256이 일치함을 확인했다. 기존 KoPub 6종이 모두 실제로 공급됐으며 World·Noto byte로 대체되지 않는다. 이 검증은 글꼴 byte 공급의 성공이고, 진행 중인 10쪽 시각 비교와 최종 fresh WASM/전쪽 gate를 대신하지 않는다. ignored `candidate-24-embedded-kopub-bytes.json`, `candidate-24-build.json`, `candidate-24-runtime-provenance.json`에 공급 원본과 실행 source를 연결했다.

262쪽의 원본 사다리는 첫 제목의 vpos 300 HU가 앞 간격 600/2 HU와 같고, 다음 줄까지의 간격 `3366−300−1270−696=1100 HU`는 제목 뒤 간격 1800/2 HU와 다음 문단 앞 간격 400/2 HU의 합이다. 독립 PDF의 제목 baseline 165.76px은 기존 Native와 맞지만 다음 문단 baseline 205.92px부터 Native가 약 4px 위로 이동한다. 공통 `stored_first_margin_is_page_relative`는 다음 앞 간격만 대조해 이 유효한 사다리를 거절하고 첫 제목의 300 HU를 쪽 원점으로 차감한다. 후보 25는 두 문단 사이의 저장 간격에 이전 문단 뒤 간격도 포함한다. `typeset::section::vpos_snap_current_height → record_vpos_page_origin`과 `layout::build_single_column → vpos_page_base_init`가 같은 판정을 소비하며, 실제 다음 글줄·그림 원점과 쪽 소속으로 검증한다. 기존 개체 제목/overlay 제외와 저장 출처 조건은 유지하고 PDF의 픽셀 차이를 고정 상수로 더하지 않는다. 구현 전 독립 사다리/Print 근거이며 이 후보는 빌드·실제 비교 전이다.

커밋 `688fb3e03bfbc585bbbb0494b59d5edc623fa00a`와 source/binary 해시가 일치하는 후보 24의 Native 선택 10쪽 비교는 누락 없이 완료했다. 140·277·375쪽은 각각 97.38618%·99.93632%·92.51634%로 PASS다. 277·375쪽 최신 PNG를 직접 판독하여 큰 숫자의 실제 KoPub 윤곽과 표 밖 비고·직인 위치를 확인했다. 277쪽 연속 그림자의 중간 패스 부재에 따른 작은 가장자리 차이는 남으며 완전 일치를 주장하지 않는다. 154·241·262·287·289·290·373쪽은 각각 87.5229%·75.3933%·77.04581%·86.92829%·69.09145%·79.27553%·79.00377%로 FAIL이다. 새 폰트 공급으로 다른 배치 결함을 면제하지 않는다. 241·289·290쪽도 직접 판독하여 이어받기 뒤의 표/본문 위치 차이와 289쪽 제목 글상자의 줄바꿈 차이를 확인했다. 해시 고정은 ignored `source-688fb3-runtime-mapping.json`, 실제 명령/결과는 `candidate-native-24-result.json`, 최신 시각 증적은 `candidate-native-reviews/handbook-native-candidate-24-legacy-font-supply/`에 보존한다.

154쪽의 이어받기 시작 컷 30은 원본 셀 문단 14의 첫 줄이다. 앞 문단 마지막 줄의 vpos 42028 HU에서 새 문단의 0 HU로 되감기며, 기존 `cell_units`가 이 독립 저장 프레임 경계의 `page_frame_reset_before`를 생산한다. 끝까지 소유한 나머지 내용은 다섯 줄로, 마지막 후행 전진을 제외한 정렬 높이 7000 HU(93.3333px)다. 실제 셀 높이 107.9px에서 안 여백 141 HU씩을 빼고 이 내용을 가운데 맞춘 첫 baseline은 독립 PDF 203.04px과 맞지만, 현재 split 판정의 Top 강제로 약 5.4px 위에 그려진다. 후보 26은 Native 미편집 저장 프레임에서 이 경계로 시작하고 끝까지 내용을 소비하는 유한한 이어받기 상자에 원래 세로 정렬을 유지한다. 컷 소유 유닛/물리 예약 높이는 변경하지 않고 `cell_units → cell_unit_opens_stored_page_frame → cut_frame_owns_alignment → 구성 창/내용 높이 → text_y_start → 실제 글줄`을 대조한다. 중간 글줄 컷·미소유 후속 내용·재조판/편집 경로는 이 새 저장 경계 조건으로 수용하지 않는다. 원본 사다리와 Print 출력에 근거한 구현 후보이며 실제 빌드·관련 이어받기/대조군 검증 전이다.

후보 25의 Windows Native fmt·Clippy·진단 빌드는 PASS, 실제 쪽수는 383쪽이다. 261·262·263쪽은 각각 99.56224%·99.34646%·99.38329%로 PASS했고 최신 262쪽 PNG를 직접 판독하여 첫 제목을 유지하면서 다음 문단·두 그림·뒤 본문의 원점이 복원됨을 확인했다. bitmap 명암과 작은 획 차이는 남는다. 241·287·289·290쪽은 후보 24와 같은 FAIL 값으로 개선되지 않았다. 선택 범위 전체 gate는 FAIL이며 Native/fresh WASM 전쪽 완료를 주장하지 않는다. ignored `candidate-25-build.json`, `candidate-25-runtime-provenance.json`, `candidate-native-25-result.json` 및 `candidate-native-reviews/handbook-native-candidate-25-source-paragraph-margin/`에 source/명령/383쪽 소속/최신 PNG·tree·overlay를 연결했다.

후보 26의 Windows Native fmt·Clippy·진단 빌드와 이후 fmt check는 모두 PASS했고 쪽수 383쪽을 유지했다. 선택 140·153·154·155·156·241·369·373쪽은 각각 97.38618%·95.74603%·90.08363%·99.29825%·99.09504%·75.3933%·99.34319%·99.5902%다. 154·373쪽 최신 PNG를 직접 판독해 이어받은 첫 셀의 정렬과 뒤 행·다음 내용 소속을 확인했다. 154쪽은 90%를 근소하게 넘으며 온전한 다음 행의 원본 저장 되감김 위치에 Print와 약 8.48px의 내부 밴드 차이가 남는다. 첫 네 줄과 뒤 열두 줄의 baseline 차이가 반대 방향 약 4.24px로 나타남을 확인했으므로 완전 일치로 보고하지 않는다. 241쪽은 그대로 FAIL이며 선택 전체 gate도 FAIL이다. ignored `candidate-26-build.json`, `candidate-26-runtime-provenance.json`, `candidate-native-26-result.json` 및 `candidate-native-reviews/handbook-native-candidate-26-closed-continuation-frame/`에 정확한 source/383쪽 소속/명령/결과/최신 PNG·tree·overlay를 보존했다.

후보 27의 원본 `7.405` 블록 컷 `[1,3,3,4,2,0,0]`은 앞쪽에서 5·6행의 모든 유닛과 걸침 셀의 내용을 이미 소비했으며 7행의 두 본문 셀만 남긴다. 종전 paint는 높이 0인 두 행에도 셀 간격 255 HU씩을 넣었고, 측정 행 재사용 조건은 소비가 끝난 걸침 셀의 선언 부족분까지 남은 행에 다시 반영했다. `row_block_cut_uses_measured_height → RowBlockQuery::fragment_height → 수용/누적 예약`과 `layout_partial_table_resolved → 같은 컷 판정 → 양수 물리 행 목록/좌표`를 대조해 이미 소비한 내용과 예약된 빈 밴드를 구별했다. 양수 높이 override를 가진 실제 빈 밴드는 계속 보존한다. Native fmt·Clippy·진단 빌드 PASS, 쪽수 383쪽과 양쪽 컷 소속을 유지했다. 선택 140·154·240·241·242·306·307·369·373쪽은 각각 97.38618%·90.08363%·95.80279%·98.56607%·98.80264%·93.18027%·98.64256%·99.34319%·99.5902%로 모두 PASS다. 최신 241쪽 PNG를 직접 판독해 첫 표의 잔여 행, 표 외곽, 뒤 제목·표의 위치 복원을 확인했다. 작은 획·표선 차이는 남는다. ignored `candidate-27-runtime-provenance.json`, `candidate-27-build.json`, `candidate-native-27-result.json`, `candidate-native-reviews/handbook-native-candidate-27-retained-block-rows/`에 source·실제 명령·소속·PNG/tree/overlay를 연결했다. 이 선택 통과는 전쪽 Native/fresh WASM과 기존 정식 회귀 검증을 대체하지 않는다.

원본 HWP를 보존한 채 한컴 2024 13.0.0.3901로 별도 HWPX 저장과 Print PDF 대조 사본을 만들었다. HWPX 사본의 289쪽 제목은 원본과 같은 두 줄 사다리와 도형 현재 높이 4066 HU를 유지하므로 재저장만으로 캐시 갱신을 주장하지 않는다. Print 사본은 383쪽이며 SHA-256 `929ceb0a0eb05f1c6dfb27673f4688935b6b3ab1bd2c9e5b521efb7bf76ca7c1`이다. 기존 기준 PDF와 전쪽의 디코딩된 page content stream 383개가 바이트 단위로 동일하고, 154·241·287·289·290쪽의 텍스트·폰트·좌표 operand도 동일함을 확인했다. 154·289쪽 Print PNG를 직접 확인했으며 289쪽 제목은 여전히 한 줄이다. 기준 PDF를 교체하지 않고 캐시와 실제 인쇄의 다른 계약을 조사한다. ignored `hancom-cache-probe-job.json`, `hancom-cache-probe-pdf-job.json`, `hancom-cache-probe-heading-geometry.json`, `independent-print-control-comparison.json`, `independent-print-control-all-page-content.json`과 Downloads의 대조 사본에 출처·원본/산출 해시·인쇄 경로를 보존했다.

287쪽의 앞 표 `10.22`는 원본 호스트 3900 HU에 선언 높이 19883 HU와 아래 여백 566 HU를 더한 24349 HU를 다음 표의 저장 원점으로 잇는다. typeset의 확정된 `ParagraphFloatPlacement::occupied_bottom`은 이를 예약했지만 partial paint의 host-margin 회수는 앞 표의 괘선 끝만 비교하여 그 여백도 지웠다. 후보 28은 앞에서 수용한 `NextLine` 배치의 끝을 회수의 하한으로 공유한다. `record_paragraph_float_placement → preceding_float_flow_bottom_px → layout_partial_table_item/동일 cursor probe → layout_partial_table_resolved → prev_table_end/host-margin 회수`를 대조했다. 배제용 float는 이 흐름 하한에 포함하지 않고, 재귀 셀 내부는 외부 문단의 배치를 상속하지 않는다. 문서 ID나 픽셀 차이를 조건으로 쓰지 않는다.

후보 28의 Windows Native fmt·Clippy·진단 빌드는 PASS, 쪽수 383쪽을 유지했다. 선택 140·154·241·287·288·289·290·306·307·369·373쪽은 각각 97.38618%·90.08363%·98.56607%·99.03593%·98.86094%·69.09145%·79.27553%·99.63554%·98.64256%·99.34319%·99.5902%다. 최신 287쪽 review PNG를 직접 판독해 앞 표의 아래 여백과 뒤 표·본문 원점이 복원됨을 확인했다. 작은 획·선 차이와 제목 둥근 모서리 차이는 남는다. 289·290쪽은 계속 FAIL이며 선택 전체 gate도 FAIL이다. ignored `candidate-28-runtime-provenance.json`, `candidate-28-build.json`, `candidate-native-28-result.json`, `candidate-native-reviews/handbook-native-candidate-28-accepted-predecessor-flow/`에 정확한 source·명령·소속·PNG/tree/overlay를 보존했다. 같은 공개 Native API로 세 쪽을 한 번 로드하여 내보낸 SVG/tree 6개는 일반 CLI의 바이트와 모두 동일함을 확인했고, 실행 시간 단축의 근거로만 기록한다(`native-batch-equivalence-28-provenance.json`, `native-batch-equivalence-28-comparison.json`). 최종 Native/fresh WASM·정식 회귀 통과를 주장하지 않는다.

### 문서 검사 범위

287·289쪽 제목의 모서리는 원본 `round_rate=50`인데 일반 사각형 paint가 짧은 변에 50%를 적용한 뒤 다시 2로 나누어 반지름을 1/4로 만들었다. [한컴 공식 개체 속성 도움말](https://help.hancom.com/hoffice130/ko-KR/Hwp/insert/objectattribute/objectattribute%28line%29.htm)은 0% 직각·20% 둥근 모양·50% 반원을 정의한다. 후보 29는 이 백분율을 짧은 변에 한 번 적용하여 50%에서 반지름이 짧은 변의 절반이 되도록 한다. 사각형 원점·높이·흐름은 변경하지 않고 공유 RectangleNode가 모든 backend에 같은 반지름을 전달한다. 일반 곡률값과 직각 대조군을 포함해 실제 출력으로 검증하며, 아직 빌드·시각 통과 전이다. 이는 289쪽의 줄바꿈과 290쪽 후속 간격을 해결했다는 주장이 아니다.

새 review 9개는 canonical `check_document_metadata.validate_file`로 각각 PASS, review·오늘할일 10문서는 `check_markdown_links.py --changed-from HEAD <10개 경로>`로 상대 링크 PASS다. UTF-8 without BOM과 LF를 확인했다. 첫 staged 공백 검사는 제가 만든 CRLF 때문에 실패하여 새 Markdown 10개만 LF로 고치고 다시 PASS를 확인했다.

전체 `check_document_metadata.py`는 파일 인자를 받지 않고 canonical 문서 614개를 검사하여 기존 4개 문서의 필수 필드 16개 누락으로 FAIL했다. `benchmark_vs_alternatives.md`, issue-4964 README, issue-5511 README와 CLI surface inventory는 최신 base와 동일함을 확인했고 이번 변경에서 수정하지 않았다. 새 기록의 PASS와 저장소 전체의 기존 FAIL을 구별하며 전역 메타데이터 통과로 보고하지 않는다. 실제 Git merge tree와 기록 commit tree의 대조 결과는 ignored `review-submission-tree.json`에 보존한다.

### 원 PR의 CI 취소 보정

#7622의 [Adapter run 37637724623](https://github.com/edwardkim/rhwp/actions/runs/37637724623)과 #7617의 [Adapter run 37637759547](https://github.com/edwardkim/rhwp/actions/runs/37637759547)은 attempt 1에서 preflight가 취소됐다. #7622의 head object 오류는 checkout 취소 뒤 발생했으며, source 결함으로 단정하지 않았다. 같은 head의 해당 job만 재실행한 attempt 2는 두 workflow 모두 SUCCESS다. #7622 Adapter worker는 실제 SUCCESS, #7617 worker는 policy SKIPPED로 구별한다. 재조회한 9개 원 PR은 모두 MERGEABLE/CLEAN, 실패·취소·진행 중 check 없음이다. 통합 후보의 새 GitHub CI는 미실행이며 시각 gate 실패를 원 PR CI 성공으로 면제하지 않는다.
