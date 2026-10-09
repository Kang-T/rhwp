---
kind: investigation
status: historical
canonical: mydocs/pr/archives/pr_7617_review.md
last_verified: 2026-10-09
---

# PR #7617 리뷰 — 표가 민 첫 글줄의 문단 앞 간격

## 최종 판정

**머지 보류.** 공개 대조군의 충족, 실행으로 확인한 시각 미충족, 코드 검토상 우려와 필수 증거 부족을 아래에 구별한다. 사용자가 승인한 conflict·CI 보정은 진행했으나 90% 시각 gate는 면제되지 않는다. 통합 후보의 로컬 회귀 PASS를 원 contributor head의 독립 검증 완료나 merge 승인으로 보고하지 않는다.

## 접수·적용 범위

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자·base | [#7617](https://github.com/edwardkim/rhwp/pull/7617) / planet6897 / devel |
| 원 head | `c19f5b805e4be92d9b51f561b2e47e0c587e3145` |
| 최초 적용 commit | `64f0a293212070210550021e64ecd5a1b7c5876c` |
| 이 PR의 적용 뒤 head | `3e7bacd2c9204d5bc3048b32628b8fc70a753bec`; 기능 commit 6개, 원 저자·cherry-pick -x 보존 |
| 누적 검증 source head | `90e2c6fd2c614ba0ea7667355c4ed2f3617f6563` |
| 최신 base 정렬 뒤 code head | `ae786f75b2cee72bd5576f6473bd51d9a8216f7c`; 제품/검증 입력 tree 동일 확인 |
| 검토 branch·reviewer | `review/planet6897-20261008` / jangster77; 원 PR reviewer assign 선행 |
| 원 PR 참고 상태 | draft=False, MERGEABLE / CLEAN; 작성 시점 snapshot이며 merge 직전 판정을 대신하지 않음 |
| 경로 | collaborator 외부 PR의 주 작업공간 누적 cherry-pick 9.1.1; intake/template/local_validation/visual_fixture_evidence 및 대형 PR 보조 절차 |

관련 이슈와 해결 표현은 원 PR 본문의 범위를 유지한다. 이번 보류 기록에 새로운 closes 표현을 추가하지 않는다. 원 PR 제공 before/after와 corpus 수치는 참고이며 통합 head의 독립 실행과 구별한다.

## 구현 주장·소비 경로·검증 판정

`typeset/paragraph.rs:223`의 float 표 원점과 `fmt.spacing_before` 차감 → `typeset/paragraph/metrics.rs:flow_trimmed_spacing_before / flow_advance_height` → `paragraph/flow.rs:place_fitted_paragraph`의 실제 배치를 대조했다. 표가 밀어낸 첫 글줄의 앞 간격 중복과 표 없는 정상 간격은 서로 다른 상태다.

공개 `float-push` 1쪽과 `column-top` 4쪽의 Native/fresh WASM 전쪽 최저는 각각 99.89324%, 99.09774%다. 대표 p1/p2에서 표 아래 첫 글줄·후속 문단 순서와 정상 대조군 간격을 직접 확인했다. **미검증:** 원 PR의 전체 영향 corpus에 있는 90% 미만 쪽, 대응 Print PDF가 없는 비공개 사례의 실제 저장본/재조판 경로. 공개 대조군의 충족만으로 원 PR 전체를 승인하지 않는다.

공통 빌드·회귀·글꼴 등록·383쪽 시각 결과와 입력/기준 해시·증적 보존·merge 후 댓글 계획은 [#7611 공통 통합 검증](pr_7611_review.md#공통-통합-검증과-증거-보존)에 연결한다. 이 PR의 조건부 수용이 아니라 미충족·미검증 해소 후 재검토 계획이다.

![native float-push p1 비교](../assets/planet6897_20261008/native_float-push_p001_review.png)

![wasm float-push p1 비교](../assets/planet6897_20261008/wasm_float-push_p001_review.png)

![native column-top p2 비교](../assets/planet6897_20261008/native_column-top_p002_review.png)

![wasm column-top p2 비교](../assets/planet6897_20261008/wasm_column-top_p002_review.png)

## 최신 보완의 경계 입력 검증

원 PR의 후속 구현 `7802047fb2682a2c97b35a3045dc76ddfac343fe`는 통합 후보에서 원 contributor 구현 뒤·메인터너 보정 앞의 `6230b09b6486e07cf3775580e9d9ff8f796f890d`로 반영했다. 아래 결과의 source head는 `59e1756d59b8d261b7d525772f991fe92e7edf9a`다. 최신 원 PR의 공개 경계 입력·기준 PDF·생성기·7개 회귀 source는 `1ed091daf421f92a9e2536645f9178877d54be40`의 정확한 Git blob을 ignored scratch로 추출해 선행 검증했으며, 아직 통합 commit에 추가하지 않았다.

공통 결과의 소비 경로는 `float_placement.rs:float_pushed_line_spacing_before` → `typeset.rs`의 첫 post-table 글줄 조건 → `state/commands.rs:consume_float_pushed_line_spacing`의 누적 높이·문단별 map → 단 전환의 map 전달 → `layout.rs`의 `start_line == 0` map 조회 → `paragraph_layout.rs:layout_partial_paragraph_with_spacing_before`의 확정 앞 간격이다. 저장 host 원점이 있는 경우는 생산 조건에서 제외한다. 앞 간격은 문단 상단 기준 거리이며, 표 점유 끝 위에 남는 몫만 한 번 예약한다. endnote 계측의 새 `EndnoteColumnPlacements`에도 global→local 문단 remap으로 map을 전달하는 호환 보정을 포함했다. 이 경계 입력은 endnote를 포함하지 않으므로 해당 endnote 실행 경계는 미검증이다.

공개 합성 HWPX(저장 줄 없는 생성본)와 이를 한/글 2020에서 저장한 HWP를 같은 HWP Print PDF 4쪽에 대조했다. Native/fresh WASM 각각 HWP 최저 99.43997%, HWPX 최저 96.45793%이며 총 16쪽 모두 `pr_review_gate=passed`, 누락·90% 미만은 0쪽이다. 16개 review PNG를 직접 열어 앞 간격이 표보다 큰/작은 host, 표 없는 대조 문단, 쪽 맨 위, 마지막 host 세 줄과 뒤 문단을 확인했다. 글꼴 획·농도와 HWPX 재조판의 일부 글자 간격 차이는 남으며 완전한 시각 일치로 보고하지 않는다.

독립 Print PDF의 텍스트에서 얻은 2쪽 채움 1~39·3쪽 채움 40~42를 네 출력의 실제 render tree와 대조해 순서·누락·중복 및 마지막 host 세 줄을 확인했다. 결과·입력/build hash·명령·manifest·PNG는 ignored `output/pr-review/planet6897-20261008/pr7617-boundaries-59e175/`의 `verification.json`·`page-ownership.json`과 네 하위 sweep에 보존한다. 이 검증은 공개 경계 입력의 신규 회귀 추가 선행 증거이며, 아직 실행하지 않은 해당 7개 정식 회귀의 PASS나 전체 영향 corpus·통합 PR 승인으로 기록하지 않는다.

## 편람 HWPX의 384쪽 불일치와 별도 메인터너 보정

후속 구현 `7802047` 적용 전 `1a1625ebdbbaf9369f142991fed0b4086f5db4d6`의 보존 WASM과 적용 후 `59e1756d59b8d261b7d525772f991fe92e7edf9a`의 보존 WASM을 같은 Chrome·원문·옵션으로 실행했다. 두 출력 모두 384쪽이고, 279~283쪽 SVG와 render tree 해시도 같았다. 따라서 이 마지막 구현이 해당 추가 쪽을 만들었다는 가설은 재현 결과와 맞지 않는다. 이 A/B는 이전 패키지를 사용한 원인 분리이며 최종 fresh WASM 승인 증거가 아니다. 기록은 `output/pr-review/planet6897-20261008/pr7617-source-ab-wasm/comparison.json`에 있다.

원본 HWPX 구역 10 문단 3의 제목 묶음 도형은 문단 기준·오프셋 0·자리차지이고, 빈 host의 유효 저장 줄은 폭 0, 높이 1000HU, 줄간격 600HU다. 도형 높이 6315HU와 아래 여백 1417HU의 합은 다음 목차 host의 저장 시작점 7732HU와 같다. 독립 Print PDF의 목차는 279~281쪽이며 뒤 질의·답변은 282쪽에서 시작한다. 도형은 보이지만 빈 host 줄은 도형 띠 안에 포함되므로 별도 공간으로 더하지 않는 계약이다.

기존 공통 판별 `renderer/mod.rs:empty_host_line_absorbed_by_topbottom_float` → `typeset/section/controls.rs`의 추가 예약 높이 → 가로 컬럼별 누적 예약과 `advance_flow_by` → `layout.rs:layout_shape_item`의 실제 도형 하단 전진을 추적했다. 그림 분기는 문단 측정에서 이미 소비한 host 줄을 차감했지만 도형 분기는 차감하지 않아 목차 예산에 1600HU(96dpi에서 21.33px)를 중복 계상했다. 도형 분기에도 같은 공통 판별을 적용했다. 문서 ID나 목차 항목 수로 분기하지 않으며, 글자·유효 줄 폭·세로 오프셋·구현 생성 줄 등 기존 판별의 반례 조건을 유지한다.

`59e1756d59` 위의 해당 source 수정 후보를 공유 `target/pr-review`에서 `cargo build --locked --jobs 8 --bin rhwp`로 새로 빌드했고 종료 코드 0을 확인했다. 같은 바이너리의 `dump-pages --compat 2024 --json`에서 HWPX 384→383쪽, HWP 대조군 383→383쪽이다. HWPX 목차 첫 예약 원점은 124.4→103.1px, 첫 두 조각의 끝 컷은 27/60→29/62로 바뀌어 원본 저장 줄의 쪽 경계와 일치한다. 마지막 조각 뒤 질의·답변도 283→282쪽이다. HWP는 전쪽 항목 소속이 같고 5쪽의 예약 높이만 변했다. 바이너리·source patch·원문 hash와 실행 로그는 `output/pr-review/planet6897-20261008/shape-host-absorption-v1/verification.json` 및 같은 디렉터리의 두 형식 페이지 계획·drift 로그에 보존했다.

첫 도형 보정 후보의 Native 279~283쪽 Visual Sweep 최저는 96.97055%, 자동 gate는 `passed`다. 5개 review PNG를 직접 열어 목차 항목 1~18/19~37/38~55의 소속과 뒤 본문 시작을 대조했고, render tree에서도 항목 1~55의 순서·누락·중복을 검사했다. 최초 실행은 Windows의 manifest 원자 교체 시 파일 접근 오류로 종료됐고, 동일 provenance의 `--resume` 실행 종료 코드 0을 확인했다. 원 실패 로그와 재개 로그를 모두 보존했다. **직접 판독으로 남은 배치 결함:** 명시적 우측 탭 쪽번호의 가로 이동과 281쪽 마지막 표 테두리의 부족한 높이. 높은 점수로 이 차이를 면제하지 않는다.

사용자가 지적한 281쪽 목차 쪽번호 18개의 시작은 Native x=596.27px, PDF advance box x=439.92pt(96dpi에서 586.56px)로 약 9.7px 오른쪽이다. 본문 `38.`/`39.`의 시작은 Native 120.5px, PDF 90.36pt(120.48px)로 맞는다. 원본 paraPr 742의 RIGHT 탭 정지는 38272HU이고, 단 시작에 더한 우단은 약 608.59px다. 기존 `find_next_tab_stop`이 이 정지를 생산했지만 교차 run 배치/폭 추정과 글자 위치 측정이 리더를 이유로 셀 콘텐츠 우단으로 덮어썼다. 후속 보정 후보는 명시적 탭 정지를 유지하고 자동 탭만 본문 우단을 생산하도록 생산자의 좌표계를 통일했다. `text_measurement.rs:find_next_tab_stop` → 문자 위치/폭 측정 → `paragraph_layout.rs:resolve_last_tab_pending` → 교차 run 폭 추정/방출의 실제 소비 경로를 함께 바꿨다. inline 저장 확장 탭의 별도 경로는 아직 이 보정의 검증 완료로 세지 않는다.

후속 탭 보정 후보도 `--jobs 8` Native 빌드 종료 코드 0과 HWP/HWPX 각각 383쪽을 확인했다. 새 281쪽 Native에서 쪽번호 시작점은 586.59px로 이동해 PDF 586.56px와 맞고, Visual Sweep 종료 코드 0·실루엣 99.12348%를 확인했다. 새 review PNG를 직접 열어 쪽번호·줄바꿈과 남은 표 아래 테두리 차이를 확인했다. 기록과 이미지는 `output/pr-review/planet6897-20261008/shape-host-and-right-tab-v2/`에 보존했다.

후속 탭 보정 후보의 fresh WASM 빌드와 실제 Chrome 281쪽 비교도 종료 코드 0을 확인했다. Native와 같은 쪽번호 위치·실루엣 99.12348%였고 새 review PNG를 직접 확인했다. 당시 표 아래 테두리 차이는 남아 있었으며 이 결과는 다음 종료 프레임 보정이나 전쪽 WASM 검증에 승계하지 않는다. 기록은 `shape-host-and-right-tab-v2/wasm-toc-panel/verification.json`에 보존했다.

종료 프레임 보정은 위 도형·탭 변경과 함께 `e469c4486e5e5d7b6de5a6e4a74d938e89dab188`에 커밋했다. 원본 단일 셀 선언 높이는 159642HU로 세 쪽을 포함하지만 기존 종료 helper는 첫 object 높이 47726HU 하나만 빼서 후속 빈 문단의 vpos와 비교했다. 실제 뒤 빈 문단의 저장 원점은 55458HU다. 새 `stored_single_cell_closing_frame_bottom`은 유효·비편집·비합성 저장 host/후속 줄을 확인하고 그 하단을 생산한다. 종료 `emit`은 실제 재개 컷이 원본 쪽 프레임에서 시작한 경우에만 현재 조각 원점을 빼고, 내용 요구 높이 이상·가용 예산 이하인지 확인한 같은 상자를 `partial_height`와 행 높이 override로 전달한다. 반복 머리행 높이는 override에서 한 번 제외한다. 내용 컷을 더 소비하거나 paint에서 별도로 상자를 늘리지 않는다. 편집·reflow·caption·각주·일반 셀 분기는 이 근거의 대용으로 쓰지 않는다.

이 커밋의 새 Native 279~282쪽 compare·standalone overlay·review를 산출했다(exit 0). 최저 98.49306%, 90% 미만·측정 불가 0쪽, focused gate `passed`다. 281쪽은 99.92654%이며 마지막 셀 높이는 709.3867→739.44px로 변했다. 네 review와 281쪽 standalone overlay를 직접 열어 쪽번호·표 아래 테두리·뒤 282쪽 본문을 대조했다. 실제 TextLine을 결합한 render tree 검사에서도 목차 1~18/19~37/38~55가 원래 279~281쪽에 한 번씩 순서대로 소속됐다. 글꼴 획·농도·AA, 제목 괘선 두께와 일부 본문 장식 경계는 남는다. source·binary·원본/PDF hash, 명령·PNG hash·직접 판독은 `committed-e469c4486-native-toc-provenance.json`, 소속은 `committed-e469c4486-toc-ownership.json`, PNG는 `committed-e469c4486-native-toc/handbook-hwpx-e469c4486/`에 보존한다. 이 자료는 Native 선택 쪽 증거이며 전쪽·fresh WASM·정식 새 회귀의 완료를 뜻하지 않는다.

**현재 미검증:** 종료 프레임 보정의 fresh WASM·전쪽 시각 검증·반례와 필수 lint 묶음. 전체 회귀는 [최신 커밋의 공통 결과](pr_7611_review.md#최신-커밋의-전체-회귀와-base-판정)처럼 28개가 실패했으며 최신 base의 90% 기준으로 이관/현재 보정을 구분 중이다. 첫 후보의 파생 suite를 준비한 review worktree의 `cargo fmt --all -- --check`와 fresh WASM 빌드는 통과했다. 주 작업공간에서 먼저 실행한 fmt 확인은 오래된 파생 목록의 누락 module 때문에 실패했으며 제품 결함 재현으로 세지 않는다. 후속 source 변경에는 첫 후보의 검증을 그대로 적용하지 않는다. 최신 원 PR head `85ea9a90bd0cd348b820db0eb9cea8d95356a15d`의 7개 경계 검사·자산은 선행 진단 scratch에만 있으며 아직 통합 commit에 추가하지 않았다.

## HWPX 전쪽 비교와 저장 경계 후보의 소유 보정

f971 제품 source의 HWP 편람 전383쪽은 Native/fresh WASM 최저90.08363%·누락0을 확인했다. 같은 source의 HWPX 전383쪽 Native는 최저27.09624%이며16쪽(116,126,135,140,152~156,223,287,289,290,370,373,375)이90% 미만이다. 같은 쪽수로 내용 소유의 차이를 면제하지 않는다.116/140/290쪽 compare·overlay·review를 직접 판독해 인접 표/뒤 본문 간격, 이어받기 문단 및 중첩 표의 실제 배치 차이를 확인했다. `candidate-f97144651-native-handbook-hwpx-full/provenance.json`과 `candidate-f97144651-hwpx-failure-panels/`에 보존하며, HWPX fresh WASM 전쪽 결과나 후속 코드의 통과로 승격하지 않는다.

140쪽의 원본(section3, host 문단64,1×1 RowBreak 셀)은 저장 줄이 문단16에서 새 원점으로 돌아간다. 독립 Print는 다음 문단17을140쪽에 두지만 Native는 그 문단들을 앞쪽으로 가져와140쪽을 문단20부터 시작한다. 실제 `CUT_BASE`는21유닛/714.507px, 기존 `stored_frame_cut_for_row`가 인증한 source 후보는16유닛/639.627px이며 동일 행 예산735.160px에 들어간다. `source_frame::resolve → source_tail → scan/runner/row_step`은 이 후보를 계산하고도 height extension이 양수가 아니라는 이유로 버렸다. 음의74.880px는 화면 위치를 맞출 보정 상수가 아니라 관측한 두 컷의 내용 높이 차이다. 원본과 소스 좌표, 진단 로그는 `pr7617-hwpx140-source-frame-closure.json`, `candidate-46de329e5-hwpx140-scan/`에 보존한다.

후속 후보는 기존 인증·profile·행 소유 gate 안에서, 실제 hard-break source 컷이 모든 셀의 capacity 컷보다 같거나 앞서며 적어도 하나가 앞설 때 그 source 컷을 선택한다. 해당 컷의 중첩 점유 예약까지 현재 행 예산에 들어가야 한다는 기존 판정은 유지한다. 예산이 source 경계에 못 미친 경우의 기존 양수 확장 계약·상한은 유지한다. 선택 결과는 같은 `res.end_cut/consumed_height`로 fit/retry와 emit을 거쳐 다음 continuation 시작 컷 및 실제 배치가 소비한다. source 인증을 완화하거나 임의로 컷 인덱스/좌표를 고정하지 않는다. 빈 물리 밴드·누적 예약과 최종 괘선/뒤 본문의 소비 경로도 새 실행에서 대조한다. 원본의 관련 출력 범위90%를 확보하기 전 새 회귀나 기준값을 추가하지 않는다.

제품 head `02de253c1bd84ca9607694694abfd7658a1bdb01`의 공유 target Native 빌드는 jobs8·exit0(452.17초)이다. 원본 결재문서5개 전7쪽은 최신 Native에서도 최저99.61781%다. 편람 HWPX의 기존 미달16쪽을 새로 출력한 결과,370쪽96.40865%와373쪽98.59779%는90% 이상이 됐으나 나머지14쪽은 미달이다. 전체 문서는383쪽이며 선택 쪽 비교를 전쪽 통과로 보고하지 않는다.140쪽은 문단17부터 이어받아 원본 소유 경계를 회복했지만26.89414%이고, 셀 안 문단 간격·그림·최종 외곽선의 높이 차이는 남는다. `candidate-02de253c1-handbook-boundaries.json`, `candidate-02de253c1-handbook-hwpx-boundaries/handbook-hwpx/silhouette.tsv`와 `candidate-02de253c1-handbook-boundary-panels/`에 실행·직접 비교 자료를 보존한다. fresh WASM 및 최종 전체 회귀는 아직 미검증이다.

116쪽 원본(section2,문단884/885)의 TAC 표 앞에는 흐름을 점유하지 않는 `IN_FRONT_OF_TEXT` 장식 도형이 있다. 두 host의 저장 높이7041HU는 표6761HU와 바깥여백140+140HU의 합이고, 다음 저장 원점12921−4440=8481HU에는 첫 줄 후행 간격1440HU도 포함된다. 기존 Native의 표 원점 차이는93.9px로 줄 높이만 소비하고19.2px의 간격을 놓쳤다. 장식이 없다는 단일 컨트롤 가정 때문에 `stored_tac::prepare_computed`가 닫힌 원본 저장 줄을 거절하고, 일반 paint의 가시 장식 전용 억제 경로가 후행 간격을 버린 것이 원인이다. 장식은 비가시 metadata가 아니며, 가시성과 흐름 점유를 구별한다.

후속 후보는 편집되지 않은 HWPX의 기존 저장 원점·후속 줄 연결·실측 높이 인증을 유지하면서, 유일한 표와 비TAC 글 앞/뒤 장식만 있는 경우 실제 표 컨트롤 번호로 계획을 만든다. 합성 줄은 기존 단일 컨트롤 조건을 유지하고, 자리차지·어울림·TAC 장식과 다른 표는 이 단축을 쓰지 않는다. 생산 `prepare_computed`의 `origin/end` → `commit_stored_tac_control`의 fit/흐름/inline placement → `layout_table_control`의 같은 pen와 `advance_end` 소비를 연결한다. 장식은 기존 `place_paragraph_controls`에서 계속 발행한다. 새 후보의 직접 출력과 원본 대조군은 아직 미검증이며, 이 단계에 새 시험·fixture·golden을 추가하지 않는다.

140쪽에서 남은 간격 차이는 빈 문단16/18/22의 실제 저장 슬롯1760/1872/1700HU(합71.09333px)와 연결된다. 모두 다음 저장 줄 원점에 정확히 닿지만 현재 tree에는 세 문단의 줄이 없고, 문단17이 셀 위 패딩 직후 시작한다. `cell_units_uncached::preserve_forward_stored_empty_spacer`는 직접 HWPX에 선언 높이800px 이상과 셀 전체의 선형 등식을 요구한다. 해당 표는735.70667px이고 그림 host11/14 뒤에는 바깥 간격300HU가 있어 두 전제를 통과하지 못한다. 반면 분할 컷은 이미 `direct_hwpx_cell_has_declared_stored_frame`으로 같은 원본의 두 프레임을 인증한다. 독립 PDF 텍스트 연산자·원본 저장 줄·현재 tree/hash는 `candidate-02de253c1-book140-blank-trace.json`에 보존한다.

`ca98ce5e8b15d50886fca35a99bd24580eeaf8ab`의 Native 빌드429.72초·exit0, 결재문서5개 전7쪽 최저99.61781%, 기존 의미 검사2PASS를 확인했다. 의미 검사는 검증한 기존 harness와 변하지 않은 모델·parser·검사 source를 재사용하되 출력 CLI를 새 바이너리로 고정했으며 최종 전체 suite 컴파일로 보고하지 않는다.116쪽은39.39997→97.35333%이고 표 원점·뒤 본문 간격이 맞지만, 직접 review에서 화살표 장식의 누락을 검출했다. 앞 기록의 `place_paragraph_controls`가 장식을 발행한다는 설명은 잘못됐다. 표 host의 장식은 일반 `controls/paragraph_flow::place_table_host_shape`가 발행하며, 저장 줄 단축의 조기 반환이 그 경로를 건너뛰었다. 높은 보조값으로 이 누락을 면제하지 않는다. 실행 기록은 `candidate-ca98ce5e8-native-check.json`, `candidate-ca98ce5e8-approval-existing-harness.json`, `candidate-ca98ce5e8-handbook-boundaries.json`과 같은 head의 대표 패널에 있다.

다음 후보는 확정된 단일 저장 TAC 표 앞뒤의 흐름 비점유 장식을 원래 컨트롤 순서대로 같은 `place_table_host_shape`에서 발행한다. 별도 줄 예약을 추가하지 않고 표 계획의 pen을 host 앵커로 전달한다.140쪽의 빈 줄은 기존 source frame 인증과 편집·dirty 제외를 공유하고, 빈 줄의 저장 높이·후행 간격·다음 원점이 정확히 닫힐 때만 물리 전진량을 보존한다. 측정은 그 값을 같은 `CellUnit`에 기록하고 실제 부분 표 배치는 현재 컷이 소유한 그 유닛의 높이를 소비한다. 글자가 없는 compositor 결과를 공간이 없다는 뜻으로 쓰지 않으며, 다른 조각의 빈 줄은 현재 쪽에 다시 소비하지 않는다. 일반 합성·로컬 리셋·중첩 표의 기존 계약은 유지한다. 새 후보의 전체 쪽수·빈 줄 앞뒤 내용·장식·외곽선·정상 대조군 및 Native/fresh WASM은 아직 미검증이며, 새 회귀나 기준값을 추가하지 않는다.

빈 줄 후보 `0ddbb2b420d73390eef068473840e2090be86d68`의 첫 컴파일은 CellUnit 내부 소유 필드를 sibling 모듈에서 직접 읽은 E0616 세 건으로 exit101(100.08초)이었다. 조판 결함의 수정 전 FAIL이나 회귀 실패로 세지 않는다. 후속 `8f5cf20efe4439ed477259873ad7f02ebdf6bf50`은 소유·컷 조회를 생산자 모듈의 `stored_empty_line_advances_for_cut`로 캡슐화했고, 해당 저장 프레임의 가로 글줄만 같은 측정·배치 계약으로 소비한다. 새 컴파일·실제 출력은 진행 중이며 결과를 선행 PASS로 표시하지 않는다. 실패 로그와 새 실행 기록은 `candidate-0ddbb2b42-cli-build.log` 및 `candidate-8f5cf20ef-native-check.json`에 보존한다.

8f의 Native 빌드는567.33초 exit0으로 끝났다. 편람 HWPX는 실제383쪽이며, 기존 미달 경계16쪽을 새로 출력해 비교했다.116쪽98.54465%,140쪽96.66997%,370쪽96.40865%,373쪽98.59779%이고, 나머지12쪽(126/135/152–156/223/287/289/290/375)은90% 미만이다. 선택16쪽의 결과를 전체383쪽 통과나 편람 fresh WASM 통과로 보고하지 않는다. 직접 읽은116쪽에서 화살표 장식은 복원됐으나 선·화살촉 차이가 남고,140쪽에서는 빈 줄과 본문 순서가 맞아졌지만 표의 마지막 하단이 여전히 짧다. 높은 점수로 실제 외곽선 차이를 면제하지 않는다. 정확한 실행·TSV·대표 패널은 ignored `candidate-8f5cf20ef-handbook-boundaries.json` 및 같은 head의 경계 폴더에 보존한다.

사용자가 개별 수정마다 빌드를 반복하지 않도록 요청했으므로, 다음 제품 빌드 전에 남은 원인을 묶어 분석한다. 현재 원문/실행 대조는 다음과 같으며 아직 구현·수용 완료 판정이 아니다.

| 경계 | 독립 원문·출력과 실제 경로 | 현재 판정 |
| --- | --- | --- |
| 135쪽 | section3/문단47의 전체 표다. 앞 안내 줄은vpos4000, textheight1000, spacing600이고 다음 그림 줄은vpos5600이다. 두 줄의 저장 line_height는21126HU로 같다. 문단 전체 그림 높이로 줄 소유를 추측하는 renderer/mod.rs와 table_layout.rs corrected_h, paragraph/format.rs의 같은 높이 두 줄 처리 및 실제 paragraph_layout.rs 배치를 함께 추적한다. | continuation으로 분류하지 않는다. 안내 줄의 실제 슬롯과 그림 소유를 공통 결과로 만들 경계이며 새 코드 검증은 미검증. |
| 152–156쪽 | section4/문단47의 같은 표다. 실제 Print보다 시작이 앞서 제목과 겹치고, 다음 쪽들도 내용 소속이 앞당겨진다. CENTER 셀의 저장 프레임이 여러 문단에 걸쳐 진행하다 되감기지만 direct_hwpx_cell_has_declared_stored_frame의 CENTER 단일-reset 조건이 일부 프레임을 제외한다. | 로컬 vpos=0 문단인 #6035 대조군과 실제 문단 간 좌표 진행을 구별해야 한다. 원점을 숫자로 당기거나 행 높이만 줄이지 않는다. |
| 223쪽 | section7/문단239의 실제 본문 셀은vpos1000에서 시작하고 두 후속 페이지 프레임은vpos200으로 돌아간다. 이 셀의 세 프레임 원문 끝과 선언 높이를 기록했다. 현재 인증의vpos<=0 조건은 양수의 실제 쪽 되감김을 세지 못한다. | 양수 원점의 저장 경계와 문단 로컬 좌표를 구별하는 인증·컷·paint 연결이 필요하다. 임의 reset 허용은 보정 근거가 아니다. |
| 289–290쪽 | section10/문단26 제목 도형은 원문 저장 줄부터2줄이며 curSz.height4066HU다. Print에서는 같은 원문이1줄로 출력된다. 기존 normalized_single_line_inline_textbox_host는 폭 fit과 호스트 높이를 함께 투영하지만 queries/rendering.rs에서 HWP5 프로필만 호출한다. | HWPX의 해당 경로 적용과 같은 폭·높이 인증을 검토한다. 단순 paint collapse로 뒤 표 원점을 남겨두지 않는다. 문단28 바깥 표는6×5이며4×4는 내부 표다. |
| 126쪽 | section2/문단991·994의 TAC 표 앞 장식은 Shape가 아니라 비TAC InFrontOfText 표다. 저장 host 높이는 본체와 위아래 여백의 합이고 첫 host의 다음 저장 원점은 후행880HU까지 닫힌다. | 유일한 흐름 표와 장식 표를 구분하는 경로와 실제 장식 발행을 함께 확인한다.116쪽에서 발견한 장식 누락을 반복하지 않는다. |
| 375쪽 | section12/문단18의 왼쪽 양식은 비TAC InFrontOfText 표이며 메모는 별도 본문41·42다. 표 내부 메모로 분류하지 않는다. 원문에는 그 사이 빈 문단의 양수 저장 슬롯과 간격이 있다. | 본문 빈 줄 점유·실제 후행 원점의 소비를 추가 확인한다. 가시 글자가 없다는 이유로 원문 공간을 없애지 않는다. |

ignored `batch-original-source-native-audit.json`, `batch-original-stored-frame-rewinds.json` 및 `batch-8f-source-diagnostic/provenance.json`에 원문 입력/PDF/실행 파일 해시와 현재 컷 로그를 보존했다. 진단은 기존8f 실행 파일을 사용하며 새 빌드를 하지 않았다. render tree의 기본 진단 프로필을 Print 시각 증거와 구별한다. 같은 미달12쪽의 새 Native raster가 ca98의 raster와 바이트 단위로 같은지도 `batch-8f-ca98-native-raster-identity.json`에서 확인했다. 이전 라벨의 패널을 새로 생성한8f 패널이라고 바꾸어 부르지 않는다. source/회귀/golden 변경이나 새로운 #7445 이관은 이 분석 단계에서 수행하지 않았다.

### 묶음 구현 후보 — 실행 검증 전

135쪽 안내 줄의 공통 생산자는 `renderer::stored_picture_guide_line_height_px`다. 원본/구성 줄 수가 같고, 현재·다음 원본 줄이 비합성·미편집이며, 현재 textheight와 spacing이 다음 원점을 정확히 닫고, 실제 control-to-line 소유로 다음 줄의 TAC 그림이 같은 저장 높이를 소유할 때만 안내 줄의 textheight를 반환한다. 현재 줄의 TAC 그림·도형·표, 같은 text_start, dirty/reflow 줄은 이 단축을 쓰지 않는다. HeightMeasurer의 문단/셀 경로4개와 table_layout의 corrected_h, paragraph/format의 예약, paragraph_layout의 실제 줄 높이가 같은 값을 소비한다. 문단 전체 개체 높이를 앞 줄 소유의 대용으로 쓰지 않으며 원문을 수정하지 않는다. 실제 입력의 적용 여부와 수정 전후 출력은 아직 실행으로 확인하지 않았다.

HWPX CENTER/양수 원점 프레임은 선언 span과 문단 사이 좌표 진행을 추가 인증한다. 진행 후 실제 원점이 작아질 때만 physical reset을 만들고, 큰 그림 줄 상자와 다음 줄의 겹침을 페이지 reset으로 보지 않는다. 단순 문단 로컬 vpos=0에는 진행 증거가 없어 새 인증을 쓰지 않으며 #6035 기존 대조군의 기대값은 바꾸지 않았다. 기존 HWP5 분기는 유지했다.

도형의 기존 폭 fit·자동 높이·호스트 투영은 HWPX에도 같은 인증을 거쳐 적용한다. 본문 빈 원본 글줄은 format 결과의 전량 전진을 사용해 다단의 후행 간격 손실을 막는다.152쪽 제목처럼 다음 비TAC 흐름 표의 원점이 원본 줄 끝·줄간격·현재 뒤 간격·후속 앞 간격을 정확히 닫는 경우에도 같은 전량 전진을 소비한다. `FormattedParagraph`에 흐름 전진 보존 상태를 분리했으며 fit 높이나 paint 원점을 위장해 바꾸지 않았다. 기존 unit test 생성 helper에는 새 상태의 기본값만 넣었고 새 회귀/fixture/golden은 추가하지 않았다.

이 후보는 수정 파일의 rustfmt와 `git diff --check`만 통과했다. 제품 빌드·집중 회귀·Native/fresh WASM·최종 lint는 아직 수행하지 않았다.140쪽의 내용 컷과 별개인 선언 물리 상자 잔여,126쪽 장식 표의 실제 발행 경로를 더 연결한 뒤 동일 묶음의 Native/WASM 산출물로 경계와 대조군을 함께 검증한다. 같은 작업의 수정마다 빌드를 다시 시작하지 않는다.

140쪽의 추가 연결은 기존 `saved_single_cell_opening_frame_height`를 사용한다. 원본 전체 셀의 진행·되감김·선언 span이 인증되고, 실제 첫 컷의 끝과 다음 프레임이 일치할 때 선언 첫 상자의 빈 하단도 예약한다. 마지막 슬롯이 빈 원본 줄이어도 글자 소유와 물리 공간을 구별한다. 기존 정확한 잉크 끝+안 여백 계약은 유지한다. 같은 생산 결과를 `fragment/budget`의 선행 예산, `row_step`의 실제 수용 높이와 row override, `emit`의 기존 `stored_row_box_sum` 및 다음 `start_row_height_override`, `row`의 후속 요구 높이와 부분 표 paint가 소비한다. 이미 소비한 내용 컷으로 과거의 물리 높이를 재추정하지 않는다. 단일 셀에 새 그림·줄·baseline을 추가하지 않았다. 첫 상자보다 원문 잉크가 크거나 dirty/재조판/로컬 리셋·중첩 셀이면 새 인증을 쓰지 않는다.

126쪽의 비TAC 장식 표는 flowWithText=false·글 앞/뒤·caption/각주 없음일 때 유일한 TAC 표와 구분한다. `prepare_computed`의 저장 원점·실측 높이·후속 줄 닫힘 인증을 유지하며, 장식은 일반 `place_decoration_table`에서 원래 순서로 발행한다. 그 경로의 포맷·continuation 조회도 확정된 host pen을 쓴다. 실제 deferred shape paint는 원본 한 줄의 유일한 inline 소유자가 전달한 같은 pen을 소비하므로, 장식이 뒤에 있다고 흐름 끝점으로 앵커를 바꾸지 않는다. 재조판·다줄·복수 inline 소유자는 기존 경로를 쓴다. 남은 경계와 결재 원본 대조군은 이 묶음의 고정된 실행 파일과 fresh WASM으로 함께 비교한다. 현재 기록은 구현 연결이며 실행 통과나 전체383쪽 승인으로 세지 않는다.

묶음 head `89e3fb88dffaaf6185b4971924aff8805f807e3e`의 manifest 준비·전체 fmt는 exit0이다. Native 컴파일은 셀 측정 세 번째 경로의 실제 줄 인덱스li를 i로 연결한 E0425 한 건으로 exit101(77.72초) 종료됐다. 실제 출력·회귀·WASM은 아직 실행되지 않았고 이 실패를 조판 회귀 또는 음성 대조로 세지 않는다. 각 경로의 iterator 인덱스를 대조해 연결 오류를 수정했으며 동일 묶음 전체를 다시 검증한다. 실패 원본과 명령/exit는 ignored `candidate-89e3fb88d-batch-check.json`, `candidate-89e3fb88d-native-check.json`, `candidate-89e3fb88d-cli-build.log`에 보존한다. 공유 target을 지우거나 새로운 target을 만들지 않았다.

묶음 제품 `bc254b8a5`의 Native/fresh WASM 경계 실행은 완료됐다. 두 backend 모두383쪽, 비교한 이전 미달16쪽 중135/154/155/156/223/287/375쪽7개만90% 미만이며 최저66.67882%다. 결재 원본5개 전7쪽은 두 backend 모두최저99.61781%를 유지했다. Native140쪽의 실제 마지막 외곽선은 Print와 맞아졌으나 위7쪽과 최종 전쪽/회귀/lint는 아직 보류다. 상세 실행·직접 판독과 #7611 새 보완의 source 우선/신규 검사 보류 및 순서 지정 rebase 결과는 [#7611 기록](pr_7611_review.md)의 마지막 절에 연결한다. ignored 실행 자료의 source는bc이며 이후 rebase head의 검증으로 승격하지 않는다.

### 남은 경계의 두 번째 묶음 — 2026-10-09, 실행 중

최신 base `79abd9e49675f4286eaed38197f7e837afb744e7` 위의 제품 후보는 `5e31d6db0bc73394d1d05f7e05a2e18b63d512dc`다. #7611 새 source가 기존 #7611 끝과 #7617 시작 사이에 들어가고, 원 053 커밋이 메인터너 보정에 앞서는 순서는 유지했다. 아래는 구현 연결과 이전 후보의 실패 관측이며 새 후보의 통과 판정이 아니다.

| 경계 | 원본·독립 출력·이전 관측 | 공통 생산 결과와 실제 소비 경로 |
| --- | --- | --- |
| 135쪽 | 그림 앞 문단의 before 500HU와 가로 여백이 실제 TextLine 상자에 반영됐지만 폴백 그림은 vpos에서 원점을 다시 만들었다. 그림이 Print보다 약6.67px 위·왼쪽이었다. 최종 원본 빈 줄 끝34626HU+안 여백282HU는 개체 높이34908HU를 정확히 닫는다. | 기존 `stored_inline_content_frame`의 원본·컨트롤 소유·정수 닫힘 인증을 HWPX의 HeightMeasurer와 실제 셀 배치가 함께 소비한다. 완전 셀 인증이 없는 경우도 `stored_picture_guide_line_height_px`가 다음 그림 줄의 소유를 증명한 경로는 이미 발행한 해당 TextLine 상자를 사용한다. 임의 좌표 보정이나 모든 그림의 강제 배치는 하지 않는다. |
| 154–156쪽 | 이전 HWP/HWPX는 같은 내용 컷을 선택했지만 HWPX의 비끝 물리 조각이 더 길어 마지막 잔여가 누적해 짧아졌다. source/PDF 괘선·내용은 `batch-bc254-native-pdf-baseline-audit.json`, `batch-bc254-native-pdf-rule-audit.json`과 `batch-bc254-source-diagnostic/{hwp,hwpx}-row-drift.log`에 있다. | `stored_table_has_physical_text_frames`는 기존 Native 또는 깨끗한 HWPX의 공유 축·진행·실제 되감김·선언 span을 인증한다. `fragment/budget`과 `table_partial`의 그리기는 기존 `rowbreak_fragment_frame_bottom`이 정한 같은 물리 하단을 사용한다. 스캔의 실제 수용 높이와 `stored_row_box_sum`이 다음 잔여를 전달한다. 내용 컷과 빈 물리 밴드를 섞지 않는다. 새 수치 허용치를 넣지 않았다. |
| 154쪽 셀 내부 ColumnDef | row5 본문은 단 설정이1단으로 바뀌며 vpos가0으로 돌아간다. 실제 Print에서는 같은 물리 행 안에 있는 로컬 축 전환이다. 뒤쪽 텍스트의 약8.5px 간격 차이는 별도 관측이며 물리 쪽으로 분류하지 않는다. | 진행 프레임 인증에서 ColumnDef가 동반한 문단 간 로컬 되감김을 페이지 인증으로 세지 않는다. 이 반례의 간격까지 해결됐다고 주장하지 않으며 새 출력에서 다시 확인한다. |
| 223쪽 | body 셀의 원점1000HU와 후속 두 프레임200HU는 선언 높이를 닫아 인증됐지만 local-origin gate가 후속 reset/cut 소비를 껐다. 앞 프레임의 마지막 내용이 다음 쪽으로 넘어가고 본문·뒤 문단이 밀렸다. | 인증된 `direct_hwpx_stored_frame_cell`을 원점 및 실제 reset-before·tail·hard-frame 계산이 공유한다. 양수 origin 수용의 기존 숫자 상한을 높이지 않았다. 원본 소유·누락·중복은 새 출력의 해당 프레임들과 함께 확인한다. |
| 287쪽 | section10의 첫 표 원점3900HU+개체19883HU+바깥 아래566HU=다음 표 원점24349HU다. `dump-pages`는 같은 원점·개체 높이·후속 표 소유를 확인한다. 이전 후보는 닫힌 바깥 상자 예약 뒤 `finish_paragraph_float_flow`에서 host after를 다시 더해 다음 표 글줄을 Print보다 약13.4px 아래에 배치했다. | `empty_float::prepare`는 복구 여부와 관계없이 인증된 높이를 실제 수용 원점에서 투영한 후속 줄 원점을 전달한다. `ParagraphFloatPlacement::paragraph_end`는 그 원점이 점유 끝을 닫을 때 host after를 중복 가산하지 않는다. state의 흐름 마무리와 layout의 마지막 원점 소비가 같은 결과를 쓴다. 인증이 없거나 점유 끝과 다른 successor는 기존 간격을 유지한다. |
| 375쪽 | 원본 overlay 표 host 뒤의 빈 글줄은 LH+LS가 다음 저장 원점을 연속해서 닫는다. HWPX의 section entry가 HWP5 전용 gate 때문에 이 글줄들을 제외해 별도 본문 메모가 약257px 위에 놓였다. | 기존 `stored_overlay_guide_owns_flow`의 깨끗한 연속 원본 인증을 HWPX에도 적용한다. section entry → paragraph의 전체 advance → layout의 overlay host page-base 유지가 같은 사다리를 소비한다. 비연속·합성·편집·dirty 사다리는 원본 공간 인증에서 제외한다. |

수정 파일 rustfmt와 `git diff --check`는 통과했다. 공유 `target/pr-review`·jobs8의 Native 1회와 fresh WASM 1회로 결재 원본5개 전7쪽 및 편람의 이전 미달16쪽을 함께 실행했다. 이 경계 실행은 전383쪽 승인이나 최종 회귀/lint의 대체가 아니다. source SHA·명령·실행 파일·입력/PDF/글꼴 해시는 ignored `candidate-5e31d6db0-{batch,native,wasm}-check.json`과 각 로그에 기록한다. 아직 새 렌더링 회귀·fixture·golden·기준값 변경을 추가하지 않았으며 공개 PR·push·승인은 수행하지 않았다.

#### 두 번째 묶음의 실제 결과와 후속 원인

제품 source `5e31d6db0`의 Native 빌드는 exit0(568.69초), fresh WASM 빌드는 exit0(296.51초)다. 결재 원본5개·전체7쪽 Native는 최저99.61781%로 모두90% 이상이다. 편람 집중16쪽은135쪽99.48977%,155쪽99.30465%,156쪽98.60223%로 개선됐지만154쪽89.76118%는 미달이다. **전체384쪽으로 기준383쪽과 달라 수용 불가**다. 최신135/154/223쪽 review PNG를 직접 열어 그림 원점 개선,154쪽 간격 차이와223쪽의 한 줄만 남은 조각을 확인했다. `candidate-5e31d6db0-handbook-remaining-panels/`의 나머지 패널은 직접 판독 완료로 기록하지 않는다.

`batch2-page-count-audit/{before,after,provenance}.json`의 같은 입력·compat2022 비교에서 구역7만52→53쪽이다. 표 host239의 첫 컷은두후보 모두 `[1,10,1]`, 두 번째는 `[1,43,1]`이다. 새 후보는 다음에 `[1,44,1]`까지 한 줄만 따로 넘겼다. 첫 reset은 unit10(문단8), 다음은 unit44(문단29)이며 유닛·문단·저장 줄 순번을 혼동하지 않는다. `batch2-frame-budget-diagnostic/`의 실제 CLI 로그는 두 번째 조각 budget739.453px, 마지막 줄 전 누적718.880px, 그 줄 높이22.027px를 기록한다. 마지막 spacing452HU(6.02667px)를 제외하면 가시 끝734.880px가 들어가지만, 기존 trim은 원점0만 받아 양수200HU의 인증된 프레임을 놓친다. 한 쪽이 늘어난 뒤의 같은 쪽 번호 비교 점수에는 내용 소유가 밀린 영향이 포함된다.

후속 구현은 인증된 양수 source frame의 끝 spacing을 기존 `stored_page_reset_cut_trailing_trim → row_cut_with_start의 capacity/최종 높이 → cell_cut_visible_height/row_cut_content_height → table_partial` 경로에 전달한다. 일반0원점·중첩·rowspan 제외 조건은 유지한다. 같은 physical-frame 인증을 `native_text_frame_first_fragment_saved_top → block/prepare.source_text_first_frame_height → fragment/budget와 layout partial paint`, `native_text_frame_terminal_successor_gap → fragment/emit와 layout 최종 cursor`에도 연결한다. 이 보완은 실행 검증 전이며 새 head의 쪽수·원본 보존·독립 PDF 비교가 필요하다. 154쪽 ColumnDef를 포함한12문단의 원본 XML/style은 `batch2-column-axis-source.json`에 보존했다. 원문은 문단 앞/뒤 간격0·sameGap0이며 관측8.5px를 구현 상수로 사용하지 않는다.

WASM 비교는 첫 원본 export에서 `setHangul2024Compat` 부재로 중단됐다. 새 package와 `wasm_api.rs`에 export가 없고 Native의 `DocumentCore::set_hangul2024_compat`만 있다. 같은 core에 위임하는 공개 setter를 추가하고 rebuild 뒤 실제 Chrome export로 검증한다. 오래된 cache 또는 문서 회귀로 분류하지 않는다. Windows pdfinfo의 비UTF8 Title은 decoding만 교정했으며 수정 전실패/후통과 및 원본2/2쪽 확인을 `batch2-pdfinfo-{before,after}-fix.log`에 남겼다. Visual Sweep 도구105개는 두 OS 경로 assertion을 native `Path`로 교정한 뒤 모두통과했다(`batch2-visual-sweep-tool-tests-after-path-fix.log`). 원본 렌더링 검사·baseline·90% 기준을 완화하지 않았다. 최신source 전체 회귀·세 Clippy·전체 Native/fresh WASM은 아직 미완료다.

#### 후속 묶음 `3483c17f4`의 실행 결과

제품 source는 `3483c17f43b06516b590c495e4d96bb77c523330`, base는 `79abd9e49675f4286eaed38197f7e837afb744e7`다. 준비된 소유 review worktree의 전체 fmt가 exit0이고, jobs8·공유 target의 Native 빌드는570.17초 exit0, fresh WASM도 exit0이다. Native 실행 파일 SHA256은 `a0af71c560ade8c620f5e1f9cb85326b184c0021a35d6e3acdfbae6a06519b72`다. 실제 Chrome export에서 새 compat setter를 사용했고 결재 원본5개·전체7쪽 Native/fresh WASM 모두 최저99.61781%를 유지했다. 명령·source/입력/PDF/font/package 해시와 결과는 `candidate-3483c17f4-{native,wasm,batch}-check.json`에 보존한다. controller의 exit0은 단계 실행 완료이며 시각 gate 통과를 뜻하지 않는다.

편람은 Native/fresh WASM 모두 전체383쪽으로 복구됐다. 기준 Print에 맞는 `--compat 2024`의 집중16쪽에서 두 backend의 점수는 같고,223쪽99.66604%,287쪽99.03899%,370쪽97.60690%,373쪽99.57694%다. 미달은154쪽89.76118%와375쪽83.30378% 두 곳이다. `candidate-3483c17f4-handbook-compat2024-boundaries.json` 및 `candidate-3483c17f4-wasm-handbook-hwpx-compat2024-boundaries/provenance.json`의383쪽·generation2024·누락0을 확인했다. 앞서 default2022로 실행한16쪽은 별도 진단 기록으로 유지하며 같은 세대의 최종 검증으로 대체하지 않는다. `batch2-layout-generation-count-audit/`의bc254/5e31 두 세대 쪽수 대조에서도 기존383→384 실패가 같았다. 최신 전쪽 계획은 `batch3-pages-compat2024.json`에 있다. 이 집중 실행은 전383쪽 TSV·전체 영향 corpus·최종 회귀/lint의 통과가 아니다.

현재 원인 추적은 두 미달을 함께 다룬다. `batch3-3483-native-pdf-baseline-audit.json`의 독립 Print text operator 대조에서154쪽 row5의 ColumnDef 앞4줄은+4.3px, 그 뒤는−4.2px이며 다른 행은0.0~0.2px다. 임의의 전체 행 이동으로 고칠 수 없는 내부8.5px 전진 누락이다. 375쪽은 section12/host43의TAC3×1 서식이며 row1 선언33754HU(450.053px)가 실제454.7px로 성장해 뒤 문단의 차이4.6px와 연결된다. 원시 행합37927HU는 common.height를 닫는다. 저장33201HU 글줄과 TAC 자식 표 및 그 안의 비TAC 손자 표의 높이 소비를 따라가며, 부모 common.height로 출력을 clamp하지 않는다. 해당375쪽 최신 review PNG도 직접 판독했다. 새 rendering 회귀·baseline은 아직 추가하지 않으며 두 미달의 원본 출력 개선이 먼저다.

#### 남은 두 경계의 독립 대조와 구현 후보 (2026-10-09, 실행 검증 전)

154쪽의 단 설정 원인은 원본 section4/host47의 11×2 표를 한 구역으로 추출한 수동 XML 대조군으로 분리했다. 원본 자체는 변경하지 않았으며, 추출본의 상속된 한컴 메타데이터를 새 한컴 저장으로 주장하지 않는다. 실제 2024 한컴 Print(`13.0.0.3901`, `frame_print_to_pdf_ex_one_up`, `pdf_print_method=0`)에서 단 제어가 있으면 이전 마지막 줄→새 첫 줄의 기준선 전진은28.48px, 없으면20px다. 최초 제어 제거본은 저장 줄의 raw UTF-16 위치를 유지해 첫 줄의 줄바꿈도 달랐다. 후속 제거본은 제어8단위를 줄 시작 위치에서도 제거했고, 첫 줄 텍스트가 원본과 같아도20px였다. 숨은 단 제어 run만20pt로 바꾼 대조군은28.48px를 유지했다. 따라서 셀 전체 이동이나 숨은 제어 글자 크기 보정으로 해결하지 않는다. 기록은 `column-axis-independent-print-comparison.json`, `column-axis-independent-print-comparison-v6.json`이며, 실제 계산 규칙은 아직 미검증이다. 진단 입력의 XML encoding을 `utf8`로 잘못 선언한 두 v5 작업은 변환 전에 실패했고, `utf-8`로 고친 v6의 두 작업은4쪽 Print에 성공했다. 이 실패를 원본 결함이나 #7445 이관 근거로 세지 않는다.

375쪽은 최신 불변 실행 파일 `3483c17f4`의 `dump-pages --compat 2024`와 원시 XML을 대조했다(`batch4-parent-height-diagnostic/`, `batch4-child-measure-diagnostic/`). 외곽 표의 가운데 행은 선언450.1px→측정454.7px이며, 그 안의 11×14 표 마지막 셀은 선언14360HU다. 이 셀의 마지막 빈 호스트 줄은 vpos11432HU·LH900HU이고, 문단 기준 비TAC 손자 표의 몸통2364HU와 바깥 여백282HU는 같은 호스트 원점부터2646HU를 점유한다. 실제 점유 끝은11432+2646=14078HU, 셀 안 여백282HU까지14360HU로 정확히 닫힌다. 기존 압축 사다리 분류의 가산 경로는 앵커 글줄900HU를 먼저 더해12332+2646=14978HU로 계산한다. 이 이중 계상 때문에 자식 표를618HU 더 크게 측정하고, 부모의 선언33754HU를347HU(4.62667px) 초과한다.

구현 후보는 `float_placement::stored_block_cell_flow`가 후속 문단 없는 마지막 block host도 저장 앵커 줄과 비교하도록 하고, HeightMeasurer의 행 요구 높이·MeasuredCell 내용 높이가 그 흐름 계획을 먼저 소비하도록 연결한다. 실제 셀 배치는 이미 `sequential_nested_cell_layout`을 통해 같은 helper의 문단·개체 원점을 소비한다. 정상 저장 사다리, 한 줄의 빈 block host, 문단 기준 TopAndBottom 비TAC 표라는 기존 인증을 유지하고, TAC·Square·caption·dirty/합성 줄은 해당 계획에 포함하지 않는다. 부모 선언 높이에 맞춘 clamp나 픽셀 상수는 추가하지 않는다. 이 후보는 `351de6e06`에 저장했으며 수정 후 실제 출력·대조군·fresh WASM·전체 회귀는 아직 실행하지 않았다.

154쪽의 추가 대조는 `column-axis-independent-print-comparison-v7.json`과 `-v8.json`에 있다. 셀 높이를2000HU 늘리거나 전체 저장 줄을 제거해 한컴이 재조판하게 해도10pt 본문의 기준선 전진은28.48px였고,20pt 본문에서는41.76px였다. 20pt 대조군은 이전 문단의 마지막 `다.` 줄을 사용해야 하며 `0.5`가 들어간 앞줄의 기준선을 마지막 줄로 세지 않는다. 따라서8.48px를 고정 추가하는 보정은 반례를 통과하지 못한다.

[한컴의 단 너비 및 간격 도움말](https://help.hancom.com/hoffice100/ko-KR/Hwp/format/columns/columns(width).htm)은 다단 전환 간격을 구역 설정에서 지정한다고 설명한다. 원본 section4의 `secPr.spaceColumns`는1134HU다. v9 수동 XML 대조군의0/2268 변경은 Print에 반영되지 않았고, 이 PC의 한컴 COM `PageSetup.GetDefault` 및 import 후 HWPML 덤프에서는 모두1134HU로 관측됐다(`column-context-native-com-probe-v2.json`, `*-native-import.hml`). 입력 XML의 숫자만으로 실제 로드된 설정을 주장하지 않는다.

실제 한컴 `PageSetup` 액션의 `SpaceBetweenColumns`를0/2268로 변경·readback·새 파일 저장한 대조군은 `column-context-native-save-v10.json`에 보존했다. 해당 로컬 COM의 저장 제품은2020(`11.0.0.9136`)이므로2024로 추정하지 않고2020 Print로 보냈다. 두 입력 모두4쪽 정상 Print이며 기준선 전진은13.44/43.52px다(`column-axis-independent-print-comparison-v10.json`). 원본10pt 글줄 높이1000HU와 전환 간격0/2268HU의 합13.333/43.573px를 인쇄 좌표 정밀도 안에서 확인한다. 원본2024의1134HU 설정은28.453px로 앞선28.48px 관측과 연결된다. 수동 추출본과 네이티브 재저장 대조군은 원본383쪽의 시각 통과를 대신하지 않는다.

`7c9f55a6992ab0af0db353efceb7e625bb505f27`은 원본 저장 줄을 사용하는 텍스트 전용 셀에서 한 단 설정의 로컬 축을 공통 `StoredSingleColumnCellFlow`로 연결한다. 앞 밴드의 실제 줄 상자 끝에 source 구역 간격을 더하고, 다음 밴드의 저장 줄 위치를 그 원점에 더한다. 원본 row5의 전환 문단은5500+1134=6634HU, 전체 내용 끝은6634+17500=24134HU다. HeightMeasurer의 행/MeasuredCell, Layout의 세로 정렬/실제 문단 원점, CellUnit의 분할 advance가 동일한 줄 원점·점유 끝을 소비한다. DocumentCore의 일반/선택적/지연 측정과 Typeset의 scratch LayoutEngine에 같은 구역 값을 전달하고 값이 바뀌면 유닛 cache를 무효화한다. 구역 값이 없는 독립 helper 호출, 여러 단, 객체 혼재, 명시 쪽/단 나누기, dirty/합성/역행 줄은 기존 경로에 남는다. 새 source를 포함한 전체 재조판·영향 corpus·편집 저장본의 Print 검증과 회귀/lint는 아직 미완료이며, 이 두 보완을 한 번의 Native/fresh WASM 빌드로 먼저 확인한다.

#### 후보 `7c9f55a69`의 실행 결과와 실제 배치 경로 누락

공유 target·build jobs8의 Native 빌드는637.58초 exit0, 잠금 파일0.2.127의 fresh WASM 빌드는368.81초 exit0이었다. 준비 및 전체 fmt 검사도 exit0이다. 원본 결재문서5개·전체7쪽은 Native/fresh WASM 모두 최저99.61781%이며, 첫 원본의5개 그림은 실제 소유 셀 내부에 있다. 기록은 ignored `candidate-7c9f55a69-{native,wasm,batch}-check.json`과 각 명령 로그에 있다. controller의 exit0은 비교 실행의 종료이며 시각 gate 통과가 아니다.

편람 원본 HWPX는 Native/fresh WASM 모두383쪽이다. compat2024로 비교한 기존 미달16쪽 중15쪽은90% 이상이지만,154쪽은89.76118→84.67500%로 악화되어 gate가 미충족이다.375쪽은83.30378→92.52421%다. 두 backend의 점수는 같고 누락은0쪽이다. TSV와 package/source/입력/PDF/font 해시는 `candidate-7c9f55a69-handbook-compat2024-boundaries.json`, `candidate-7c9f55a69-wasm-handbook-hwpx-compat2024-boundaries/provenance.json`에 고정했다. 전383쪽 시각 비교·최종 회귀·세 Clippy·개별7개 PR의 최종 검토는 아직 미완료다. 새 렌더링 회귀·baseline 추가와 공개 PR 생성·push·승인은 수행하지 않았다.

`batch4-7c9-native-pdf-baseline-audit.json`에서154쪽 앞4줄은 Print와0.0px 차이로 맞지만 전환 후 줄은−8.5px다. `batch4-cell-flow-path-diagnostic/provenance.json`은 새 불변 CLI로 실제154쪽 `export-render-tree --compat 2024`를 실행한 exit0 기록이다. 해당 host47/셀(5,1)은 `table_partial.rs`의 paragraph paint를 사용하며 `cut=None`인 온전한 셀이다. cp0/1/2의 실제 문단 원점은306.6/346.6/386.6px로 순차 전진해 단 전환 간격이 없다. 일반 horizontal-cell path에 새 앵커를 연결한 것만으로 실제 소비가 끝났다는 앞 설명은 불충분했다. 공통 결과로 측정·정렬은 바뀌었으나 이 partial-table paint 소비가 빠졌다. 후속 보완은 같은 source 밴드 원점을 이 실제 paint에 연결하고, 컷이 있는 경우에는 현재 조각의 첫 소유 줄과 함께 원점을 해석해야 한다. 출력 전체를 이동하거나 관측8.5px를 상수로 추가하지 않는다.

375쪽은 최신 Native raster와 독립 PDF를 직접 열어 오른쪽 표 하단 및 뒤 본문의 높이 복구를 확인했다. `batch4-post-fix-height-diagnostic/provenance.json`의 same-source `dump-pages --compat 2024`도 exit0이다. 남은 좌측 양식의 약1.9px 위치 차이와 상단 쪽번호 추가 표시는 별도 미해결 관측이며, 높은 실루엣 점수로 누락·중복 계약을 면제하지 않는다. 두 경계의 compare/standalone overlay/review는 ignored `candidate-7c9f55a69-handbook-boundary-panels/`에 보존한다. 사용자가 이번 실행을 끝낸 뒤 최신 `upstream/devel` 동기화와 리베이스를 요청했으므로, 완료 결과를 먼저 커밋하고 그 다음 base 정렬 및 보완을 진행한다. 기존 기록·원본·독립 PDF·공유 cache를 보존한다.

#### 최신 base 위의 두 소비 경로 보완 — 실행 검증 전

최신 base `21ed64071`의 순서 보존 rebase를 끝낸 뒤 `b06e963f9`은 온전한 셀이 partial-table 안에 놓이는 실제 경로에도 `stored_single_column_cell_flow`의 문단/줄 원점을 전달한다. 부분 셀은 현재 컷의 첫 소유 줄을 같은 밴드 좌표계의 조각 원점으로 사용하며, 첫 조각의 leading band와 문단 앞 간격은 한 번만 소비한다. 기존 문단/개체 소유 컷을 바꾸거나 줄을 숨기지 않는다. 그림·중첩 표·dirty/합성 등 공통 생산자가 제외한 경로는 해당 계획을 사용하지 않는다. 실제154쪽의 `cut=None` 경계와 컷이 있는 대조군의 실행 확인은 아직 미검증이다.

375쪽의 상단 쪽번호는 원본 XML과 실제 Print로 별도 분리했다. 원문 전체에는 section11의 EVEN 머리말, ODD 꼬리말, 빈 EVEN 꼬리말만 있으며 ODD/BOTH 머리말은 없다. 홀수 표시 쪽367의 Print text operator에는 하단 쪽번호 하나만 있지만 이전 실제 render tree에는 Header/Footer 양쪽에 `－ 367 －`가 있다. `batch4-hf-parity-source-diagnostic.json`에 입력/PDF/tree hash·원본 applyPageType·Print 기준선681.92px·수정 전 노드를 보존했다. `0a8ac9f82e3123637b5f3741feb9f5d56dd735b7`은 구역 간 kind 상속의 반대 홀짝 fallback을 제거하고 같은 홀짝 슬롯만 소비한다. BOTH 정의는 기존 carry 갱신에서 두 슬롯에 각각 기록되며, 현재 구역의 슬롯별 재선택과 이전 구역의 같은 슬롯 상속은 유지한다. 임의로375쪽의 Header를 숨기는 처리가 아니다. 작성 당시 이 source의 Native/fresh WASM·홀짝 정상 대조군·전체 회귀와 최종 lint는 미검증이었다. 아래 실제 실행 결과에서 완료한 범위만 갱신한다.

#### 최신 후보의 영향 쪽 검증과 홀짝 대조 — 2026-10-10

제품 source `0a8ac9f82e3123637b5f3741feb9f5d56dd735b7`, base `21ed64071f81f35f37618717693bd30f727392c9`를 고정했다. 같은 공유 target과 build jobs8에서 Native CLI 빌드는481.66초 exit0, fresh WASM은129.53초 exit0이며 fmt check도 exit0이다. 이전 source의 산출물을 최신 source의 결과로 재사용하지 않았다. 원본 결재문서5개·전체7쪽은 Native/fresh WASM 모두 최저99.61781%, 미달·누락0쪽이다.

편람 원본 HWPX는 두 경로 모두383쪽이며 기존 미달16쪽의 Native/fresh WASM 최저92.77642%, 미달·누락0쪽이다. 154쪽은 수정 전84.67500%에서98.48451%,375쪽은92.52421%에서92.77642%다. `candidate-0a8ac9f82-handbook-compat2024-boundaries.json` 및 `candidate-0a8ac9f82-wasm-handbook-hwpx-compat2024-boundaries/provenance.json`에 입력·Print PDF·font·package/source hash와 전체16쪽 TSV를 연결했다. 이 결과는383쪽 전수 비교의 완료 판정이 아니다.

같은 후보의154·375쪽 Native/fresh WASM compare·standalone overlay·review4개를 새로 산출하고 모두 직접 열었다(`candidate-0a8ac9f82-handbook-boundary-panels/`). 154쪽은 셀 내부 단 전환 뒤 글줄과 후속 행의 큰 밀림이 해소됐다.375쪽은 오른쪽 서식의 외곽·뒤 본문이 복구되고 상단 중복 쪽번호가 제거됐다. 왼쪽 서식의 약1.9px 위치·획 농도 차이는 남는다. 이 차이를 점수로 면제하거나 전체 일치를 주장하지 않는다.

독립 원문 Print 번호와 실제 두 backend의 render tree를374·375·376쪽에서 대조했다. 표시366·368의 짝수 쪽은 Header 번호만, 표시367의 홀수 쪽은 Footer 번호만 소유한다. 수정 전375쪽의 Header/Footer 중복과 수정 후 Footer 단독을 기록했다(`batch5-hf-parity-check.json`). `displayText`의 실제 자동 번호를 검사했으며 원시`text`의 공백을 번호 부재로 오인하지 않았다. 이 진단은 정식 회귀 검사를 대신하지 않는다.

고정 base를 전달한 manifest48/48 target 검사와 source unit tier4149 tests 검사는 각각 exit0이다(`batch5-base-policy-checks.json`). 원 PR7개는 원격에서 모두 open이며 기록한 contributor head와 동일함을 재확인했다(`batch5-remote-pr-heads.json`). 현재 HWP/HWPX의 Native 전체383쪽 비교를 실행 중이다. 전체 fresh WASM 비교·분할 반례의 정식 회귀·전체 nextest·세 Clippy·실제 출력 backend·각 PR 최종 검토는 아직 미완료이므로 보류 판정을 유지한다. 새 rendering test/baseline과 원격 push·통합 PR은 아직 추가하지 않았다.

#### 242쪽 이월 오류의 공통 끝점 보완 후보 — 2026-10-10

`0a8ac9f82`의 HWP fresh WASM 전383쪽도 종료됐다. 미달은242쪽44.16538% 하나이며, 앞서 완료한 HWP Native44.16228%, HWPX Native/fresh WASM43.99893%와 같은 행 소유 오류다. 실제 전쪽 결과는 `candidate-0a8ac9f82-wasm-handbook-hwp-full/provenance.json`에 보존한다. 4개 조합의382쪽 통과를 전체 gate 통과로 바꾸지 않는다.

같은 원본을 고정된 Native 실행파일 `f97144651`과 `bc254b8a5`로 진단했다. section7 host408의 첫 컷은 원래0..11/이어받기11..13에서0..9/9..13으로 달라졌으며 셀별 줄 높이는 같다. 이전 표405의 컷·이어받기 점유도 같다. 차이는 제목407에서 전체49.3px를 누적한 흐름으로, 다음 표 진입이242.4→275.0px가 됐다. 실제 제목 원점과 표 원점은 두 실행의 paint에서 같지만 이 추가 예약 때문에 두 행이242쪽으로 이월된다. 기록은 `batch6-242-{cut,budget,flow}-history/`에 원본·binary·source hash와 명령·실제 JSON·진단 로그를 연결한다. 전역 내용 중복이 아니라 쪽 소유 오류다.

독립 원본 저장 줄은 제목407의17118HU와1250HU 줄높이·748HU 줄간격·1000HU 뒤 간격으로 다음 host408의20116HU를 정확히 닫는다.152쪽의 제목46도27323+1250+748+1000=30321HU로 후속 host47을 닫는다. 두 제목은 같은 문단 양식이다. 전체 문단 높이를 더하는 기존 boolean은152쪽에는 맞지만 앞 partial-table 뒤 커서의 차이가 있는241쪽에는 과다 예약한다. 이전처럼 무조건 fit 높이만 더하면152쪽의 제목·표 겹침을 다시 만든다.

후속 후보는 `float_placement.rs:stored_paragraph_successor_flow`에서 genuine·비편집·비dirty의 저장 줄과 후속 비TAC 문단 상대 흐름 표가 닫는 콘텐츠 하단/흐름 끝 HU를 함께 만든다. 문서 ID·행 번호·고정 픽셀 보정·clamp를 사용하지 않는다. 재조판 줄 수가 달라진 경로와 다단·합성·저장 줄 리셋은 이 전체 문단 계약 밖이다. page base는 column anchor, lazy base는 body origin, unbased partial-table 저장 사다리는 기존 배치처럼 body-relative 축을 쓴다.

측정 소비는 전체 fitted/atomic-overflow 문단의 공통 끝점 사영 → 실제 cursor 차이 전진 → 다음 표의 budget/cut이며, 콘텐츠 하단도 같은 결과를 기록한다. 절대 끝점과 전체 문단 높이의 차이를 spacing trim 또는 underrun으로 재해석하지 않는다. 실제 배치는 `layout.rs`의 FullParagraph 최종 항목 완료에서 같은 끝점을 같은 축으로 소비하고, 그 뒤 다음 항목 커서를 갱신한다. 빈/partial 문단은 이 전체 문단 계약으로 덮어쓰지 않는다. 이 source 변경의 fresh Native/WASM·전체 페이지 gate·정상 대조군·필수 회귀/lint는 아직 미검증이다. 새 회귀나 baseline은 아직 추가하지 않으며 보류 판정을 유지한다.
