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

### 문서 검사 범위

새 review 9개는 canonical `check_document_metadata.validate_file`로 각각 PASS, review·오늘할일 10문서는 `check_markdown_links.py --changed-from HEAD <10개 경로>`로 상대 링크 PASS다. UTF-8 without BOM과 LF를 확인했다. 첫 staged 공백 검사는 제가 만든 CRLF 때문에 실패하여 새 Markdown 10개만 LF로 고치고 다시 PASS를 확인했다.

전체 `check_document_metadata.py`는 파일 인자를 받지 않고 canonical 문서 614개를 검사하여 기존 4개 문서의 필수 필드 16개 누락으로 FAIL했다. `benchmark_vs_alternatives.md`, issue-4964 README, issue-5511 README와 CLI surface inventory는 최신 base와 동일함을 확인했고 이번 변경에서 수정하지 않았다. 새 기록의 PASS와 저장소 전체의 기존 FAIL을 구별하며 전역 메타데이터 통과로 보고하지 않는다. 실제 Git merge tree와 기록 commit tree의 대조 결과는 ignored `review-submission-tree.json`에 보존한다.

### 원 PR의 CI 취소 보정

#7622의 [Adapter run 37637724623](https://github.com/edwardkim/rhwp/actions/runs/37637724623)과 #7617의 [Adapter run 37637759547](https://github.com/edwardkim/rhwp/actions/runs/37637759547)은 attempt 1에서 preflight가 취소됐다. #7622의 head object 오류는 checkout 취소 뒤 발생했으며, source 결함으로 단정하지 않았다. 같은 head의 해당 job만 재실행한 attempt 2는 두 workflow 모두 SUCCESS다. #7622 Adapter worker는 실제 SUCCESS, #7617 worker는 policy SKIPPED로 구별한다. 재조회한 9개 원 PR은 모두 MERGEABLE/CLEAN, 실패·취소·진행 중 check 없음이다. 통합 후보의 새 GitHub CI는 미실행이며 시각 gate 실패를 원 PR CI 성공으로 면제하지 않는다.
