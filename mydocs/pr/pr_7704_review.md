---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7704 검토 — 표 조각의 반복 셀 검사 성능

## 최종 판정

**메인테이너 16쪽 보정·시각 판정 통과 / PR 전체 시각 gate 보류.**
원 기여자의 두 순회 개선은 계산 의미를 보존한다. devel `920dc29284` 통합본에 메인테이너 보정을 적용한 Native/fresh Docker WASM 전11문서·393쪽 재출력을 완료했다. HWPX 16쪽은 98% 이상이며 작업지시자의 직접 판정을 통과했다. 다른 문서의 점수는 원 PR 출력과 같고, 이번 수정으로 새 미달이 생기지 않았다. 전체 회귀 10,570건과 Native Skia 필수 3종 검사는 통과했다. 그러나 기존 5문서·312쪽의 미달을 면제한 승인은 없으므로 원격 승인·통합 단계로 진행하지 않는다. 검증 종료 시 원격 devel `bfa28a77a2`의 추가 통합을 확인했으며 이 기준의 재검증은 아직 하지 않았다.

## 접수와 트리아지

- PR: [#7704](https://github.com/edwardkim/rhwp/pull/7704), 작성자 `oksure`, base `devel`, [#7639](https://github.com/edwardkim/rhwp/issues/7639)는 `Refs`이며 부분 개선이다.
- 검토 head: `896e64ff6d876e676d6ebf987d6669b47653912a`.
- code head: `3e746bf676611983e6e002e355fd2e25258d05ec`; devel 통합 head `0ebdfc42e5dc61d6ca0f9323aedec631cd7cc131` 이후 현재 head까지는 WASM PNG 두 개 추가뿐이다.
- 최신 비교 base: `f79f2e1d6831a2187657b9c60deabc5d7c8b8e0d`.
- 담당자 `oksure`, reviewer `edwardkim`, milestone `v1.0.0`, labels `layout`, `performance`, `rust`, `table` 반영·재조회 완료.
- maintainer 일반 + 접수/로컬 검증/시각 증적/다수 PR 경로. 최신 base 충돌 simulation 결과 `696dae99d74f14051a9faeb7f4279a94a9c96193`, 충돌·whitespace 오류 없음.

## 구현과 조판 계약

| 변경 | 의미 보존 근거 | 실제 소비 경로 |
| --- | --- | --- |
| `budget.rs`의 `stored_row_heights` | 범위 내 행, `row_span == 1`, 높이 `< 0x8000_0000` 셀만 행별 0.0에서 같은 순서로 max. 빈 행·범위 밖 셀의 취급 동일 | 첫 source frame의 행 끝 인증, source complete frame의 컷 인증 → 스캔 예산·이월 → 기존 조각 배치 |
| `typeset.rs`의 table-wide vpos reset | 기존 행별 존재 검사와 새 범위 내 셀별 존재 검사는 동치. 셀 안 문단을 가로지르는 양수→0 이하 전환과 synthetic LineSeg 제외 유지 | `scan/runner`, `row_step`, `block/prepare`, `whole_fit`, `block/entry`, `continuation/fragment/budget` |

새 helper는 셀 내부 판정을 원 코드에서 추출한 것이다. 셀 사이에 previous vpos를 공유하지 않는다. 순회 순서만 바뀌며 판정 함수에 상태 변경이 없다. row별 caller는 동일 helper를 소비한다. 새로운 문서 ID, 픽셀 여유, clamp, 분할 허용치, 저장 정보 신뢰 조건을 추가하지 않았다. 시작/끝 컷, 요구/예약 높이, 예산 실패 및 실제 paint 계약 자체를 바꾸지 않는다.

행×전체 셀 순회를 전체 셀 한 번 순회로 바꾼다. Table에 지속 캐시를 도입한 변경은 아니므로 편집 후 캐시 무효화 계약이 추가되지 않는다. 다른 행별 탐색과 편집 성능 전체 해결은 범위 밖이다.

## CI와 성능 증거

- 원 code head의 [Full CI 38024324551](https://github.com/edwardkim/rhwp/actions/runs/38024324551): Build & Test, 세 Clippy를 포함한 Lint, Native Skia 실제 실행·성공 확인.
- `eb2da208`은 원 code head에서 Native PNG 두 개만 더했다. 현재 [CI 38122343388](https://github.com/edwardkim/rhwp/actions/runs/38122343388)의 preflight는 이 성공 후보 및 `0ebdfc42`의 current-base merge-tree 일치를 확인해 fast pass했다. skipped worker를 새 Full CI 실행으로 표현하지 않는다.
- 기여자 측정: 동일 Linux/AMD EPYC, `RHWP_2424_PROFILE=1 rhwp info samples/issue2063_huge_cellbreak_table.hwp`. typeset 128,790→5,445ms, pagination 130,771→7,299ms, Node WASM open/pageCount 150.9→14.6초. 상대 개선 측정과 실행 명령이 있다. 검토자 재측정 수치가 아니며 다른 호스트의 절대 목표로 적용하지 않는다.
- 기여자의 1,225개 Native 동일 출력 및 17개 WASM 동일 출력 주장과 실제 재조회한 11개 문서 TSV를 구분한다. 전 코퍼스 실행 원본을 검토자가 독립 재실행한 것으로 쓰지 않는다.

## 검토자 focused 실행

정확한 head에서 `node scripts/rust-test-suite-manifest.mjs --prepare` 후 공용 `/home/edward/mygithub/rhwp/target/pr-review`를 사용했다.

- `node scripts/run-rust-test.mjs issue_6123_rowbreak_boundary_row_line_cut -- --cargo-profile release-test --target-dir /home/edward/mygithub/rhwp/target/pr-review --test-threads 2`: 1 passed. 저장 frame과 행 끝 불일치에서 줄 단위 컷과 앞뒤 조각 컷 연결을 확인한다.
- `node scripts/run-rust-test.mjs p122_stored_vpos_page_break -- --cargo-profile release-test --target-dir /home/edward/mygithub/rhwp/target/pr-review --test-threads 2`: 3 passed. 저장 vpos 되감김의 쪽 소속과 그림 배치를 확인한다.
- `pr7704-focused-height.log`, `pr7704-focused-vpos.log`에 원문 보존. 기존 검사를 실행했으며 새 baseline·fixture·테스트를 추가하거나 기대값을 변경하지 않았다. 이 검사 결과는 전체 시각 일치의 증거가 아니다.

## 검토자 성능 대표 문서 실행

정확한 PR head에서 `cargo build --locked --profile release-test --bin rhwp --target-dir /home/edward/mygithub/rhwp/target/pr-review` 성공 후 같은 fixture에 `RHWP_2424_PROFILE=1 /usr/bin/time ... rhwp info samples/issue2063_huge_cellbreak_table.hwp`를 실행했다. 총 9.88초, pagination 8,745.688ms, typeset 6,325.199ms, continuation 1,970.01ms, 161쪽이었다. `pr7704-cli-build.log`와 `pr7704-head-perf.log`에 보존했다. 검토자 base 실행은 없으므로 이 결과로 독립적인 전후 배수 개선을 주장하지 않는다. 한컴 PDF 쪽수 162는 제출자의 기록이며 실제 head의 161쪽과 구분한다.

## 제출 시각 증적 재검산

[원 TSV gist](https://gist.github.com/oksure/e4ad1f22e723bdf3dfb54001e9114369)의 current base/head Native·WASM 44개를 내려받아 재검산했다. 출력 경로별 11문서 393쪽. 11쌍씩 모든 base/head TSV가 바이트 단위로 동일하며 페이지별 수치에 누락을 숨기는 평균은 사용하지 않는다.

| 문서 | 쪽수 | Native/WASM 최저 | 각 경로 90% 미만 쪽 |
| --- | ---: | ---: | ---: |
| rowbreak HWP | 18 | 92.04216% | 0 |
| cell image HWP/HWPX | 각 8 | 96.74178% | 0 |
| table vpos HWP/HWPX | 각 5 | 98.86640% / 98.86666% | 0 |
| picture toggle HWP | 1 | 99.95151% | 0 |
| rowbreak HWPX | 18 | 31.59993% | 6 |
| giant cell overfill HWPX | 48 | 0.26231% | 36 |
| giant nested table HWP/HWPX | 각 115 | 23.48645% | 각 115 |
| scattered header HWP | 52 | 41.03657% | 41 |

각 경로 합계 313쪽 미달이다. 성능 대표 `issue2063`의 161쪽 대 PDF 162쪽은 기여자가 기록한 기존 차이이며, 11쌍 표에는 포함되지 않는다.

원본/PDF 22개 경로가 검토 head에 들어 있고 실제 파일과 commit 바이트가 같음을 확인했다. `pdfinfo`의 전체 쪽수와 제출 TSV 행 수가 일치한다. 한컴 Creator의 2020/2022/2024 표기는 기준 provenance 보조값이며 Print 생성 이력을 독립 재확인한 것으로 쓰지 않는다.

head에 고정된 Native/WASM rowbreak HWP 7쪽 review PNG를 실제로 열었다. 대표 쪽의 표 외곽, 행 경계 및 본문 순서에 큰 배치 차이는 보이지 않으며 글자 모양·굵기 차이가 있다. 원 제출 PNG 판독이며 검토자의 최신 통합 head 재출력 또는 작업지시자의 시각 통과 판정으로 쓰지 않는다. 대표 한 쪽의 92%로 다른 313쪽을 면제하지 않는다. 정식 font mismatch 예외 JSON도 제출되지 않았다.

증적은 `/home/edward/mygithub/rhwp/output/pr-review/oksure-20261011/`의 `pr7704-gist.json`, `pr7704-submitted-tsv/`, `pr7704-submitted-tsv-summary.json`, `pr7704-input-provenance.json`, `pr7704-preflight.log`, `pr7704-full-ci-jobs.json`에 보존했다.

## 해제 조건

기존 차이와 이번 의미 보존 성능 변경의 처리 범위를 메인터너가 결정한다. 현행 기준을 적용하는 동안은 기여자가 원본/정상 글꼴/독립 Print를 대조하고, 필요한 수정 또는 정식 글꼴 예외 근거와 새 head 증적을 제출해야 한다. 그 뒤 최신 통합 head의 focused·Native/fresh WASM·직접 확대 판독 및 CI를 확인한다. 자동 점수 기준을 낮추거나 기존 차이를 새 회귀로 단정하는 댓글은 게시하지 않는다.

관련 #7639는 다른 병목과 편집 지연이 남으므로 이 두 순회 개선만으로 전체 종료를 주장하지 않는다. 현재 GitHub approve·merge·comment·close는 수행하지 않았다.


## 메인테이너 직접 시각 판정 준비

사용자 요청에 따라 같은 PR head `896e64ff6d876e676d6ebf987d6669b47653912a`의 Native를 공용 target에서 다시 빌드하고, Docker 표준 wasm 서비스로 새 web package를 만들었다. 코드·테스트·기여자 branch는 변경하지 않는다. 새 준비 증적은 기본 작업공간의 ignored `output/pr-review/oksure-20261011/pr7704-visual-review/`에 보존한다. 기존 gist TSV와 이번 검토자 재출력을 구분한다.

- 빌드: `cargo build --locked --profile release-test --bin rhwp --target-dir /home/edward/mygithub/rhwp/target/pr-review`; 바이너리를 이번 증적의 `candidate/rhwp`로 복사하고 SHA-256을 고정했다.
- WASM: `docker compose --project-name rhwp --env-file .env.docker run --rm wasm`; 최적화를 포함해 8m29s 성공. 새 pkg와 Studio public의 JS/WASM SHA-256 동일성 확인.
- 준비 범위: 제출된 11개 실제 원본과 대응 PDF, Native/Docker WASM 전쪽 각각 393쪽 TSV·PNG 및 분할 경계·미달 대표 review/overlay PNG. 빌드와 실행 로그, 명령·입력 경로·DPI·compat·binary/package 해시는 `provenance.json`, `logs/`, 경로별 sweep manifest에 남긴다.
- 독립 PDF마다 2024 또는 2022 compat을 지정한다. 설치/Studio 웹폰트 정책을 적용하며, 자동 점수를 글꼴 예외나 한컴 일치 최종 판정으로 바꾸지 않는다. 잘못된 font 경로를 지정하지 않는다.
- 검토용 Studio: `http://localhost:7794/?review=pr7704-896e64ff`. 기존 서버를 교체하지 않고 별도 worktree에서 실행한다. CDP Chrome 새 탭에 `rowbreak-problem-pages.hwp`를 실제 file input 경로로 열어 18쪽·Canvas 표시를 확인했다. 실제 응답 WASM hash `3f03e6ed3085b5e3411ac875d38feff565d6b4ad856992d78b1c4ac956df99ce`가 Docker 산출물과 일치한다(`studio-browser-proof.json`).
- 첫 판정: rowbreak HWP 7쪽 → HWPX 4·6·7·8·15·16쪽 → overfill·nested·scattered의 미달과 정상 대조군. HTML은 상대 이미지·원본·PDF 링크를 사용해 전체 폴더 복사 뒤에도 볼 수 있게 구성한다. 판정·완료·merge 승인을 추정하지 않으며 사용자 확인을 기다린다.

### 준비 완료 및 메인테이너 판정 (2026-10-11)

- 동일 head에서 Native와 Docker WASM 각각 11개 문서 전체 393쪽 TSV 및 PNG 산출 완료. 6개 문서는 전쪽 90% 이상, 5개 문서는 각 경로 313쪽 미달로 자동 gate 보류를 유지한다.
- 메인테이너 직접 판정: **rowbreak HWP 16쪽 정상 / rowbreak HWPX 16쪽 하단 결함**. 형식별 판정을 구분하며 HWP 통과를 HWPX 통과 근거로 쓰지 않는다.
- HWPX 16쪽은 하단 두 번째 표·그 아래 내용 영역에서 기준 PDF와 배치 차이가 남는다. 해당 쪽 실루엣 Native 78.46732%, WASM 78.45578%. 이 성능 PR로 새로 발생한 회귀라고 단정하지 않는다.
- 비교 화면: http://localhost:7795/index.html#doc=rowbreak-hwpx&page=16 . 전체 폴더의 상대 경로 HTML과 원본/PDF/TSV/PNG 링크를 제공한다.
- 증적: `/home/edward/mygithub/rhwp/output/pr-review/oksure-20261011/pr7704-visual-review/manual-verdicts.json`, `README.md`, `provenance.json`. PR 승인·병합·원격 게시 없음.
- 추가 확인: 메인테이너가 동일 문서 16쪽 하단의 HWP/HWPX 조판 불일치를 직접 확인했다. 파서 속성·IR·실제 조판 소비 경로의 원인은 아직 확정하지 않았다. 비교 HTML을 CDP Chrome 새 탭의 HWPX 16쪽에 열고 세 이미지 로드·겹침/배율 제어와 상대 이미지 경로 1,335개를 확인했다(`gallery-browser-proof.json`).

### 메인테이너 보정 착수

- 사용자 승인: HWP/HWPX 16쪽 불일치를 원인 분석 후 직접 보정하고 #7704 처리에 포함한다. 로컬 branch `review/oksure-pr7704-20261011` (기존 검토 branch에서 보정 연속 진행); 최신 `upstream/devel` `920dc29284e8a6e624861bfccf68903e19ae309b` 통합 SHA `973abfd5c` (충돌 없음). contributor 원 SHA와 분리한다.
- 원본 생성: 저장된 실제 HWP/HWPX 쌍과 독립 한컴 Print PDF를 그대로 사용, 메타데이터 수동 변경 없음. compat 2022/2024 교차 실행에도 HWP cut=11 / HWPX cut=12로 차이가 유지된다.
- 원인: `table_layout.rs::cell_units_uncached` → 같은 높이·문단/줄 소속을 가진 47유닛 → `advance_row_cut_inner`의 `entering_non_inline_control_range_end` 전체 개체 수용 검사 → 컷 → `table_partial.rs`의 shape 소유 control range 배치. 같은 유닛 목록에도 HWP 전용 gate가 HWPX에서 전체 수용 검사를 끈다. HWPX는 p5 글상자 filler 첫 16px를 소비해 현재 조각에서도 글상자를 배치·clip한다. Shape는 다음 조각에서도 소유 유닛에 따라 내부 내용을 이어 그리므로 Picture의 entry-only 정책과 구분한다.
- 위반 계약: 현재 조각에서 앞 내용이 이미 소비된 후 새 비인라인 개체를 시작할 때는 소유 range의 전체 요구 높이를 확인한다. 새 쪽에서 수용 가능한 글상자의 첫 filler만 현재 쪽에 남기지 않는다. 입력 형식은 이 계약을 바꾸지 않는다. 첫 유닛부터 시작한 fresh fragment와 본문보다 큰 개체의 진행 보장은 유지한다. 태그 없는 텍스트·TAC·CellBreak 표는 이 검사 비대상이다.
- 수정: HWP 전용 조건을 제거하고 RowBreak 비-TAC 및 실제 tagged control range로 검사. 픽셀 clamp·화면 숨김·문서 ID 조건 없음. 기존 helper/높이 결과를 그대로 소비한다.
- 선행 관측: 같은 first-fragment avail 286.2px에서 원 HWP cut11, HWPX cut12. HWPX 하단 글상자 entry가 현재 쪽에 들어가 PDF와 다름. logs `pr7704-maintainer/diagnosis/*units-old.log` (두 목록 동일), `*scan-old.log`. 수정 후 시각 gate와 영향 경계 확인 뒤 정식 회귀 추가 및 필수 lint/회귀를 진행한다.

### 두 번째 원인: 부동 개체 위 호스트 줄의 간격 소유

- `layout.rs::layout_table_item`에서 `host_line_fits_above_offset_float`가 참이면, TAC 간격을 더한 뒤 문단 원점으로 되돌렸다. HWPX는 문단 위 간격(raw 1000HU / 2 = 6.6667px)과 outer top(283HU, 3.7733px)을 잃었다. 정상 HWP는 확정 `InlineBoxPlacement` 원점과 advance_end가 이 간격을 보존했다.
- 진단 probe의 실제 소비값: HWPX `y=615.52, anchor=615.52, placement=None`; HWP `placement.y=527.6933`(단 상대), `advance_end=642.9067`. 같은 앞 문단 끝점에도 최종 TAC 괘선은 HWP 625.94 / HWPX 615.52. 독립 Print PDF의 두 형식 출력은 같은 위치다. ignored `diagnosis/*inline-origin.log`에 기록했고 probe는 제품 코드에서 제거했다.
- 보정은 기존 원점 복원을 표 위 간격 적용 앞으로 옮긴다. 부동 개체가 이미 전진시킨 흐름 하한은 기존 restore로 유지하고, 글줄의 spacing_before와 outer top은 한 번 적용한다. 신규 형식·문서 조건이나 위치 상수는 추가하지 않는다. 이후 `layout_table_control_block`의 확정 placement가 있으면 기존 공통 결과를 우선한다.
- 새 Native/fresh Docker WASM 시각 판독과 정상 대조군은 진행 중이다. 이전 head 결과를 보정 head의 통과로 사용하지 않는다.

### 16쪽 보정 결과와 메인테이너 판정

- Native 16쪽: 78.46732% → 98.18272%; fresh Docker WASM: 78.45578% → 98.16647%. 독립 한컴 Print와 16·17쪽 비교 PNG를 직접 대조했고, 메인테이너도 새 Studio에서 **시각 판정 통과**를 승인했다. 18쪽 전체 쪽수 유지.
- 수정 전 글상자 제목은 16·17쪽 트리에 모두 있었고, 수정 후에는 17쪽에만 한 번 있다. TAC 표 원점도 정상 HWP와 일치한다. `diagnosis/behavior-before-after.json`은 실행 전후 구조 관측이며 정식 신규 회귀 테스트를 대신하지 않는다.
- 정상 HWP 대조군 전18쪽 Native/fresh WASM 최저 92.08273%; 16쪽 Native 98.13736% / WASM 98.12108%.
- 실제 Studio `http://localhost:7794/?review=pr7704-maintainer-p16`에서 18쪽, 16쪽 Canvas 표시 및 응답 WASM SHA-256 `a3c92ddb91393b33d692555d6d3cf35654a12658bed9158af75a8a4c786663f2`가 최종 Docker/pkg/public과 일치한다. 진단용 임시 probe는 제거했다.
- 기존 경계 검사 #6133·#4468·#6045·#6123 각각1건 통과. fmt, native/WASM/workspace-all-target Clippy, workspace build 및 manifest 정책(base `920dc29284`) 통과. 전체 release-test 10,570건 통과(50건 skipped). Native Skia 라이브러리 및 #2225 누락 그림·37쪽 직접 PDF export 필수 3종도 모두 통과했다. 정확한 명령과 종료 코드는 `regression-results.json`에 보존한다.
- HWPX 전쪽은 4·6·7·8·15쪽에서 여전히 90% 미달이다. 사용자 판정은 보정된 16쪽에 적용한다. 다른 문서의 전체 게이트·기존 미달을 면제한 승인으로 해석하지 않는다. 새 정식 렌더링 회귀의 추가는 관련 전쪽 Native/fresh WASM 선행 조건이 충족될 때 진행한다. baseline·기존 기대값·허용치 변경 없음.
- 증적: 기본 작업공간 `output/pr-review/oksure-20261011/pr7704-maintainer/`의 `provenance.json`, `manual-verdicts.json`, `lint-focused-results.json`, `native-spacing/`, `sweeps/*/wasm/`, `native-spacing-review/`, `wasm-spacing-review/` 및 실행 로그. 비교 HTML `http://localhost:7796/`.

### 최종 코드의 전체 시각 재검증

- source `973abfd5c421e7ccef58f4d8954a25ec6faee992` + `diagnosis/maintainer.patch` SHA-256 `0ba8ab738991aa77c06a0084fbbfee10bcd18c8dc6ef66bfaf26f7b9945fe412` (로컬 보정 미커밋 상태). 전11문서 Native/fresh Docker WASM 각393쪽.
- 6문서 전쪽90%이상. 기존 미달5문서·312쪽: rowbreak HWPX 5쪽, overfill36쪽, nested HWP/HWPX 각115쪽, scattered41쪽. 모든 결과의 페이지 수는 기존 출력과 동일하다. rowbreak HWPX 16·17쪽 외의 수치는 원 PR 검토자 출력과 동일하다.
- `native-visual-summary.json`, `wasm-visual-summary.json`, `sweeps/*/{native,wasm}/*/silhouette.tsv`에서 전쪽 최저·미달 쪽과 원 PR head 대비 변화 값을 확인할 수 있다. 시각 부족을 성능 코드의 새 회귀로 바꾸어 보고하지 않는다.

### 실제 분할·이어받기 경로 확인

- 일반 행: `scan/runner/row_step.rs:444`에서 안여백을 뺀 내용 예산 → `:456`의 mixed-nested 예약 컷 → `table_layout.rs:20887`의 `advance_row_cut` → `:17015`의 소유 control range 높이 검사 → `row_step.rs:799`의 같은 컷 표시 높이 및 `:1215`의 수용 누적. 시작 컷은 16쪽 `[]`, 종료 컷 `[11]`; 17쪽 시작 `[11]`에서 나머지 유닛을 소비한다. tagged range 없는 줄, TAC/CellBreak, fresh fragment의 진행 분기는 이번 guard 확장의 비대상이다. 여러 rowspan이 함께 끝나는 block-cut 경로는 이 사례에 적용되지 않으며 이 수정으로 공통화했다고 주장하지 않는다.
- 실행: `diagnosis/final-path-scan.log`에서 16쪽 예산 286.2px, 내용 컷 263.0px·안여백 포함 수용 266.7px, 실제 프레임 281.1px 모두 예산 안에 있다. `final-next-path-scan.log`에서 17쪽 시작 `[11]`, 종료 `[]`, 예약/paint 920.7px·예산 933.5px를 확인했다. 기존 paint의 저장 프레임 보정은 별도 경로이며, 모든 물리 프레임 높이와 예약을 이 수정에서 통합했다고 주장하지 않는다.
- 실제 paint: `table_partial.rs:3252`의 Shape 소유·이어받기 분기는 같은 유닛 컷을 소비한다. 제목의 16쪽 제거·17쪽 단일 표시, 표 윤곽·다음 내용 및 18쪽 전체 쪽수는 전후 트리와 독립 PDF/Native/fresh WASM으로 확인했다. 작은 원래 높이만 fit하는 예산 등 추가 합성 경계는 미검증이며 신규 정식 회귀의 시각 선행 조건을 우회하지 않는다.
- TAC 원점: `layout.rs:13397`의 호스트 줄 복원 → 문단 위 간격/표 outer top → `:13565`의 table control 배치 → `:13591`의 앞 부동 개체 흐름 하한 복원. 확정 InlineBoxPlacement가 있는 정상 HWP는 그대로 우선하고, HWPX의 spacing 소유를 지우던 실행 순서만 고쳤다.

### 최신 원격 기준 확인

- 로컬 검증 종료 후 원격 `devel`은 `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`였다. 검증 기준 `920dc29284`보다 266개 커밋 앞서며 조판 코드도 변경됐다. 기존 검증 결과를 최신 통합 코드의 통과로 바꾸어 보고하지 않는다.
- 최신 코드에도 HWP 전용 range gate와 spacing 적용 뒤 원점 복원이 남아 있는 것은 확인했다. 기존 검증 보정본을 먼저 보존하며, 최신 코드와 통합한 뒤 영향 경계·필수 검사·전쪽 시각 결과를 다시 확인해야 한다. 원격 push/PR 생성/approve/merge는 수행하지 않았다.

### 최신 기준 통합과 범위 확장 (2026-10-12)

- 16쪽 통과 보정은 `ce4189473`에 보존했다. 최신 devel `bfa28a77a2` 통합 head는 `1990ae5c1cf019b9d6cbf8553d87489243a2730a`. 충돌 한 곳은 새 셀별 예산·native 저장 프레임 변수를 유지하면서 HWPX에도 range 검사를 적용하도록 해결했다. 변경된 조판 코드를 포함하므로 이전 전체 회귀를 이 head의 결과로 재사용하지 않는다.
- 이 head의 관련 경계 4건, fmt, Native/WASM/all-target Clippy, workspace build, manifest 정책(base `bfa28a77a2`)은 통과했다. 전체 회귀는 추가 보정 범위가 승인되어 실행 중 중단했으며 최신 head의 완료 검사로 세지 않는다(`current-base/validation-stopped.json`). 기존 `ce4189473` 검증의 전회귀/Skia 통과 기록은 보존한다.
- Native 전11문서·393쪽 완료: 정상6문서 전쪽90%이상, 나머지5문서316쪽 미달. HWPX16쪽 98.30924% / 17쪽 99.83140%. 직접 review PNG에서 표 앞 간격과 17쪽 글상자 시작·윤곽·후속 내용을 다시 확인했다. 최신 devel의 다른 변경도 포함된 결과이므로 과거312쪽과의 차이를 이번 보정의 개선/회귀로 단정하지 않는다. 이 head의 고정 Docker WASM package 전쪽 실행은 계속한다.
- 사용자 범위 결정: **기존 시각 미달도 보정한 후 PR 처리**. 이번 PR에서 시각 게이트 예외는 적용하지 않는다. 순서: rowbreak HWPX 4·6·7·8·15쪽 → overfill HWPX → nested HWP/HWPX → scattered HWP. 원본/Print 및 전체 TSV를 보존하고, 원인 규칙별로 영향 경계·시각 출력을 먼저 고친 뒤 최종 source에서 필수 회귀를 다시 수행한다.
- 최신 증적은 `output/pr-review/oksure-20261011/pr7704-maintainer/current-base/`의 `provenance.json`, `native-summary.json`, `validation-results.json`, `native-review/`, `sweeps/` 및 실행 로그. 사용자 시각 판정은 `ce4189473`의16쪽 판정이며 최신 통합본 직접 판독과 구분한다.
