---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-09
---

# semanticist21 비 draft PR 통합 검토

## 현재 판정

**머지 보류 — 메인터너 보정과 검토 진행 중.** 초기18개 비 draft PR의 고유41개 contributor commit을 누적 적용했다. 최초 검토의 미충족과 이후 보정 결과는 각각의 source SHA에 연결해 아래에 기록한다. 현재 남은 일반 문서의 시각 미통과·내용 소속 차이·최종 head 검증이 해소되기 전에는 통합하지 않는다.

- 브랜치: `review/semanticist21-20261008`
- 현재 통합 base: `79abd9e49675f4286eaed38197f7e837afb744e7` (진행54의 fetch·devel 동기화·rebase 확인; 원격 최신 상태는 제출 직전 재조회). 이전 base `6f66932a73ee6fd05a74a83bdc438c9499a89a12`는 진행42–53의 역사 검증 기준이다. 이전base `5168e3256ad58eda425d84fdd4d67e9b47bb74b1`는진행39–42의검증기준이다. 이전base `f0e7228f6dd2ea1437724e53ad640c40c56d204b`는역사검증의기준이다. 최초 검토 base `60efc4b7b380b2a183c20c3ee0ab24a60bc2d832`는 역사 검증의 기준이다.
- 현재 메인터너 체크포인트는 이 문서와 함께 보존하는 v74–v77 묶음 보정이다. 진행58에 Native 검증·실패·미검증 범위를 기록한다. 이전 `3505cb8cd8d09f3c125fdb08c9ccec6efece78ff`는 진행55–56의 v73 검증·커밋·동기화 체크포인트다. 이전 `f3c8cd7d370668e36322db71c1c2c820260124ae`는 진행54의 v72 커밋·리베이스 체크포인트다. 이전 `fe201fa05f25ee744fb465a8b8a7348d38576d18`의 v64 retry1 원본 표제/첫 조각 보정과 검증은 진행44의 역사 증거다. 이전 `eb29a7a9ce171c2cc5dfd9ff4689eb7a0d64eaae` 및 v61 결과는 역사 증거다.
- rebase 전 contributor 누적 head: `cd0eb2d25380f6db60a4b8bb99ed90f7e1d5c84c` (#7659 포함). 메인터너 보정 v44 source는 `64cc5b5e805e2b05ab3fd64fa5aa4803e0d6357a`이며, 이후 후보와 검증은 아래 단계별 기록을 따른다.
- 최초 17 PR 검증 code head: `134a646e5a5325daf52d614e0395094175e78301` (아래 최초 검증 결과의 역사 head)
- 최초 검토 head: `8a9f87300a47293d644f8b862906c989adf34f84` (새 source 적용 전 역사 결과)
- 최초 contributor 누적 diff(역사 후보):47 source/test 파일, 5421 추가 / 823 삭제. contributor 원 commit은 `cherry-pick -x`로 보존. merge commit 제외, #7615 중복 의존 commit 4개는 재적용하지 않음.
- draft 제외: #7559, #7572, #7573, #7588, #7589, #7598.
- 통합 PR 생성/push/merge/원 PR close/후속 처리: 아직 미수행. 실패 gate를 통과한 것으로 보고하거나 일부 PR만으로 scope를 줄이지 않음.

## PR별 최초 검토 근거

아래 표는 최초 누적 후보의 판정이다. 이후 메인터너 보정의 개선/남은 차이와 Native/fresh WASM 증거는 아래 source별 기록을 따른다.

| PR | source head | 적용 commit | 현재 근거와 남은 범위 |
| --- | --- | --- | --- |
| [#7566](https://github.com/edwardkim/rhwp/pull/7566) | `abf6f9259ac45ee25e1a2ccd4fd5b7c2073c8dcf` | `b4f958c42` `51aa04372` `0b5d0af07` | Native 셀 주소/활성화 계약 통과. fresh WASM UI 활성화는 미검증. |
| [#7612](https://github.com/edwardkim/rhwp/pull/7612) | `34ceeb0d067549ee0a8689b3b24231bfb1f23428` | `b1d7ccf33` `dbfb72a17` | Native 논리 캐럿·개체 슬롯 계약 통과. 영향 편집 저장본 Print/시각 범위 미검증. |
| [#7613](https://github.com/edwardkim/rhwp/pull/7613) | `eab4fcd8307556c8e8650a35eac8fa881ffce38a` | `0d47bdfba` `64f9f3d82` | 미충족: 두 연속 표의 1쪽 Native/fresh WASM 48.61432% / 38.42346%. |
| [#7614](https://github.com/edwardkim/rhwp/pull/7614) | `2d4148eb8bcb561b2eac2004c7a64741864e7431` | `093ccccb7` `28c97f2a4` | Native HTML 붙이기 및 문단 끝 번호 계약 통과. 편집 출력 Print 미검증; source CodeQL 실패. |
| [#7615](https://github.com/edwardkim/rhwp/pull/7615) | `ed9c2847523c18f658344ec89cb7ed1a426807ae` | `a2ef9eab3` `761499928` `ebbec90fb` `60385438c` `3addfced3` `6a8f77dab` `2be87053d` | Native 누름틀·정의·슬롯 보존 검사 통과. 편집 저장본 Print 대조 범위 미검증. |
| [#7616](https://github.com/edwardkim/rhwp/pull/7616) | `ca43e499d30e90789d5e662f9cb562252fe95367` | `428a71a9b` `841ecf256` | 미충족: 일반/누름틀/책갈피 HWPX 각 1쪽 Native/fresh WASM 14.16561%. HWP 인쇄 세 건 900초 timeout. |
| [#7638](https://github.com/edwardkim/rhwp/pull/7638) | `aa0c88be40be3ca29aabc95bc724c2ade29c333a` | `c68d0b6dd` `51fbf680c` | Native 마당 1~10 머리말/꼬리말 글자 스타일 검사 통과. 전체 마당 Print/fresh WASM 시각 미검증. |
| [#7641](https://github.com/edwardkim/rhwp/pull/7641) | `9b6f6c51c2e873da0aaf339cb51e9e60be33a0f1` | `d746acf6e` | 미충족: 좁은 셀 1218HU에 중첩 표 점유 7766HU. Native/fresh WASM 모두 재현. |
| [#7647](https://github.com/edwardkim/rhwp/pull/7647) | `e33be129632b4fb9369fcc76c9b5af617aec5be4` | `f78d84aff` `327a4bf2a` | Native 잘못된 21식·정상 45식·오류 시 칸 보존 계약 통과. 공개 WASM 오류/정상 합계 통과. 전체 gate 대기. |
| [#7648](https://github.com/edwardkim/rhwp/pull/7648) | `9703c5fc91b38ca0e4d80dece8d68c9014b006e2` | `5394ca686` `2e33dd6ca` | Native 각주/미주 독립 번호·시작 번호·저장 왕복 통과. 실제 번호 인쇄 범위 미검증. |
| [#7652](https://github.com/edwardkim/rhwp/pull/7652) | `95e51ad453a7e083e842a9024353919e895affb1` | `6484cd815` `40ebd5be5` | Native 탭 뒤 쪽 번호 자리표·HF 저장 계약 통과. Print/fresh WASM 시각 미검증; source Build & Test 실패. |
| [#7653](https://github.com/edwardkim/rhwp/pull/7653) | `1c5f93a2309437f2b9770c8619c92c5dcc8fb84d` | `b89dfdc30` `e774138aa` | 원 head 단독의 뒤 문단 겹침은 #7656 누적 후 Native/fresh WASM에서 보정 확인. 독립 인쇄 범위 진행 중. |
| [#7654](https://github.com/edwardkim/rhwp/pull/7654) | `db92404bf4427fad3cac2e9a6fa300c6f94d7a65` | `338a825ab` `a5ddd6fa9` | Native/fresh WASM에서 세 문자열의 미주 캐럿 경계 대조 통과. paint 변경 없음(동일 공백/번호 판정 helper 추출). 전체 gate 대기. |
| [#7655](https://github.com/edwardkim/rhwp/pull/7655) | `51d44bffb68c9459cbf4eb470878cf0afb027ad1` | `0b211ca68` `8a9f87300` | Native 무늬 종류/색·전체 Fill 비교 검사 통과. 다섯 편집 상태 Print 성공; Native/fresh WASM 시각 실행 중. |
| [#7656](https://github.com/edwardkim/rhwp/pull/7656) | `268e40eb1daf448bf66541eebf570a69ef8ad6ef` | `f1cc14414` `cf43c3ca5` | 미충족: 다음 문단이 비면 100%, 글자가 있으면 두 경로 86.01714%. 겹침은 없어졌지만 글줄이 아래로 이동. |
| [#7657](https://github.com/edwardkim/rhwp/pull/7657) | `a64ab93cbce0f788e9812673e6add4ce0dd24dce` | `031d7ccee` `b7e559534` | Native 실제 한컴 sample 포함 스타일/원시 바탕쪽 보존 통과. 공개 WASM 셀 스타일/HWP/HWPX 보존 통과. 조판 비해당 범위 근거 추가 확인 필요. |
| [#7658](https://github.com/edwardkim/rhwp/pull/7658) | `8b2f95d6872fba94ff1bb96d40c2bdbe64d6b90d` | `a4b502e32` `134a646e5` | 미충족: 탭 100%, 테두리/배경 82.33902% (두 경로). 녹색 문단 배경 하단 9.37px 차이. |

| [#7659](https://github.com/edwardkim/rhwp/pull/7659) | `db6f70d9c523692080f5318e406a578c11e6c32e` | `47a304211` `cd0eb2d25` | 신규 각주 스타일 Native source assertions 3/3 PASS. 실제 Print/fresh WASM/Studio 미검증. |

## 실행한 검증

| 명령 / 범위 | 결과 | 증적 |
| --- | --- | --- |
| `git diff --check upstream/devel...HEAD` | PASS | 원 source diff |
| `cargo build --locked --profile release-test --target-dir target/pr-review` | exit 0, 4분 21초 | `logs/native-build-134a646e5.log` |
| `cargo fmt --all -- --check` | exit 0 | `logs/fmt-check-134a646e5.log` |
| `scripts/wasm-pack-locked.sh --target web --out-dir pkg --no-opt`, `CARGO_TARGET_DIR=target/pr-review` | exit 0, 9분 30초. Docker 없는 환경의 최적화 생략 진단 빌드; optimized 배포 artifact 검증 아님 | `logs/wasm-build-134a646e5.log` |
| Native 원 PR source assertions의 진단 복사/연결 실행 | 최초 86/86 PASS + 자동 번호 4/4 + #7265 대조군 2/2 = 92 source assertions PASS. 정식 nextest 실행 아님 | `logs/fixture-capture-all-134a646e5.log`, `logs/fixture-capture-auto_number_trailing_placeholder-134a646e5.log`, `logs/fixture-capture-issue_7265_hwp5_origin_hwpx_table_outer_top-134a646e5.log` |
| 진단 반례 2개 | 표 여백 뒤 문단 PASS; 좁은 중첩 표 FAIL. ignored 진단에만 존재하고 tests/cases에 신규 추가하지 않음 | Native 위 로그, `wasm-public-contracts-134a646e5.json` |
| 공개 WASM API 6개 범위 | 5 PASS / 1 FAIL(중첩 표). 셀 스타일, 탭, 오류 계산식의 대상 보존, 미주 캐럿, 여백 변경의 다음 문단 포함 관계 확인 | `logs/wasm-public-contracts-134a646e5.log` |
| manifest `--check --base-ref 60efc4b7...` | PASS: 1503 sources / 6477 static attrs / 48 integration targets | `logs/manifest-check-134a646e5.log` |
| unit-tier `--check --base-ref 60efc4b7...` | PASS: 4196 tests / 298 modules | `logs/unit-tier-check-134a646e5.log` |

Native/WASM 산출물 SHA-256은 `build-provenance-134a646e5.json`에 고정했다. WASM SHA-256은 `e097f1b2ed1328aefee359a7b32dfb680f12ceb9f34983069807c143e5698cb4`; pkg/public의 JS와 WASM 각각 해시 일치. Studio 브라우저 사용자 여정은 미실행이다.

세 Clippy, workspace build, 전체 nextest, Skia 추가 gate는 아직 실행하지 않았다. 작은 경계와 직접 시각 판독에서 실제 차이를 먼저 확인하는 순서를 따랐다. 현재 source head의 CI 성공을 통합 head CI로 재사용하지 않는다.

## 한컴 Print와 출력 대조

같은 저장본을 실제 한컴 2020 `11.0.0.9136` / `PrintMethod=0` / `hancom2020_pdf_driver_one_up`로 출력했다. 입력이 공개 API 또는 수동 모델 변형을 거쳐 생성된 경우 실제 한컴 원본 생성본과 구분한다. 5000HU column-top 사례는 source test의 모델 변형이고, 공개 WASM 여백 사례는 `setTableProperties`로 편집했다.

두 공급 font 디렉터리는 실제 존재하며 HCR Batang을 공급한다: `/home/tsjang/.cache/hnc/fonts`, `/home/tsjang/.cache/rhwp-review-hancom-fonts-20260811`. Sweep은 96dpi print profile / embedded fonts / 2px 이웃 관용 내용 실루엣으로 Native와 fresh WASM을 각각 실행했다. WASM은 실제 Chrome export helper를 사용했다.

| 입력 (전부 1쪽) | Native | fresh WASM | 직접 판독 / 판정 |
| --- | --- | --- | --- |
| `cell_background_pattern_change-0001-dd74ebaaa9ec` | 100.00000% | 100.00000% | 해당 review PNG / gate 확인 |
| `cell_background_pattern_change-0003-62aaf8b57f83` | 100.00000% | 100.00000% | 해당 review PNG / gate 확인 |
| `cell_background_pattern_change-0005-4ace892a7d1b` | 100.00000% | 100.00000% | 해당 review PNG / gate 확인 |
| `cell_background_pattern_change-0007-fd96b4836e2c` | 100.00000% | 100.00000% | 해당 review PNG / gate 확인 |
| `cell_background_pattern_change-0009-07084746450e` | 100.00000% | 100.00000% | 해당 review PNG / gate 확인 |
| `column_top_table_reserves_outer_margins-0015-0aad3cf3a347` | 100.00000% | 100.00000% | 해당 review PNG / gate 확인 |
| `column_top_table_reserves_outer_margins-0021-80d658ee147e` | 86.01714% | 86.01714% | 해당 review PNG / gate 확인 |
| `created_table_after_table_starts_below_it-0010-e93fb91c8b2c` | 48.61432% | 48.61432% | 해당 review PNG / gate 확인 |
| `created_table_after_table_starts_below_it-0011-bb7be43f0149` | 38.42346% | 38.42346% | 해당 review PNG / gate 확인 |
| `inline_table_after_field_or_bookmark-0244-1792e69cfaf2` | 14.16561% | 14.16561% | 해당 review PNG / gate 확인 |
| `inline_table_after_field_or_bookmark-0250-4536c5dce2d6` | 14.16561% | 14.16561% | 해당 review PNG / gate 확인 |
| `inline_table_after_field_or_bookmark-0257-dbaa6d12fde5` | 14.16561% | 14.16561% | 해당 review PNG / gate 확인 |
| `updated_style_keeps_tabs_and_borders-0545-e0a6bc5b14ba` | 100.00000% | 100.00000% | 해당 review PNG / gate 확인 |
| `updated_style_keeps_tabs_and_borders-0547-e1b58ace1b99` | 82.33902% | 82.33902% | 해당 review PNG / gate 확인 |

직접 판독: #7613의 두 번째 표 시작점 차이 지속. #7616은 일반·누름틀·책갈피 3개 HWPX의 앞 글자 간격과 표/뒤 글자 위치 차이. #7656은 표 괘선이 같은데 비어 있지 않은 뒤 문단이 아래로 이동. #7658은 문단 배경 하단이 Native 150.9333px / Print 160.3040px이며 색상 배경과 테두리 높이 차이. 동일 Native/fresh WASM 대표 review를 각각 직접 읽었다. 유입 회귀인지에 대한 변경 전 대조는 미완료이며, 현재 출력 gate의 실패와 구분한다.

기준 PDF와 입력 hash/job id는 아래 표에 연결한다. 변환 timeout은 출력 결함의 원인 입증으로 세지 않는다. inline HWP 3건은 최종 failed/900초이며 Print 대조 미검증으로 유지한다.

| 입력 | SHA-256 | Print PDF SHA-256 | job / 결과 |
| --- | --- | --- | --- |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/cell_background_pattern_change--changing_only_the_cell_pattern_changes_the_cell_background/0001.hwpx` | `dd74ebaaa9ecaefb5d754b434051feff168316c0afbf4aeadb944cd31b4ac21f` | `1f8b5fee1ae8fea30a9f66eeeff86c55370e0f0f27df66c38b4ac85608aa8bed` | `557b0a9f-e379-4161-b3ab-c181ce362ee8` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/cell_background_pattern_change--changing_only_the_cell_pattern_changes_the_cell_background/0003.hwpx` | `62aaf8b57f83516a59505515d6082cbf4511259970c6410def814d25e894033a` | `4b0d8c8d84db3315abe7f373957ab25c2cbd960398eeb13a09f2f2b6efdbf7ee` | `a399775d-0f15-4555-aa6c-8ac82691c033` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/cell_background_pattern_change--changing_only_the_cell_pattern_changes_the_cell_background/0005.hwpx` | `4ace892a7d1b80ffbcd9d22e79e3e1ab63e8b656f3085ec2210b8e2c591c371e` | `05e68ea6c5f46d56e1b71358829c050bb35e7cd137288ecf156691a2ed61df5f` | `8bd203c1-2c8c-4f09-be70-24fa3fd516fe` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/cell_background_pattern_change--changing_only_the_cell_pattern_changes_the_cell_background/0007.hwpx` | `fd96b4836e2c49ec6b1e748d55f887c48f81ad6b93a3f87626cd756049278fdc` | `53eb9dc719a2bb90a066fa4f57ff76fd1d72092cd8e90d2a6a5384fc8c997fa7` | `4c9d8033-6291-4fbe-8ba8-8b8b6f9fd5ec` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/cell_background_pattern_change--changing_only_the_cell_pattern_changes_the_cell_background/0009.hwpx` | `07084746450efe8e2e96a15ef35ea0ca6920b96ca6ae948915a1afa4dd901833` | `d899871d6f09d9d58a08936ee4a6429934e01e3366c88155768ffd3499f54e43` | `d43c3e07-8d0f-4825-b518-6a83be8d4e19` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/column_top_table_reserves_outer_margins--paragraphs_after_first_line_table_start_below_its_bottom_margin/0015.hwpx` | `0aad3cf3a34737e4593c81c10989899e224505bbfabe104a4c73667cc0155f4b` | `3a960780f6afb663ec83f0bf1b69e65001fcf68fc54a79c01c9b9914c9df6bfe` | `d872f979-3011-40bf-87b2-d1287ba5f56a` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/column_top_table_reserves_outer_margins--paragraphs_after_first_line_table_start_below_its_bottom_margin/0021.hwpx` | `80d658ee147e7ada087460808be94c6433abf7a0b4f08cd7126a6f159e632661` | `97559a36aa108672fd553edcea48ec8e6cd1d2f1e7c1b786ab924795babe5399` | `c8b4e1ed-da92-4fac-9a0d-a2ced4ad01ed` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures/created_table_after_table_starts_below_it--a_table_created_right_below_a_table_starts_below_it/0010.hwp` | `e93fb91c8b2c28ff87744e8ef1f8be245f650fc3060f2c54eaebdb502e151ea6` | `a16a20a002f83d60f0cdf5ce1cfdd9488ead35bb5983439f7f55bba356420af1` | `0cf2e341-9c53-4a86-873e-13dc205b751d` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures/created_table_after_table_starts_below_it--a_table_created_right_below_a_table_starts_below_it/0011.hwpx` | `bb7be43f0149db7981b3bebac7a8f002f7d617010c9e060517b37793d8cdbf0d` | `532a0728cf80ee150d1f1b0471f407c2368e6ebfb41fc1d9e0b26a39939e1e35` | `b9d014c2-e273-466d-96ca-18eab5dcb6bc` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures/inline_table_after_field_or_bookmark--inline_table_after_bookmark_gets_its_own_row_after_merge/0243.hwp` | `74a49507963845d5e1603c9db50787ab26bd5762dbaab3d919fdf6fa8a918d0c` | `미산출` | `35cedd43-a4f1-4faf-9a82-29dbaa095c06` / failed |
| `output/pr-review/semanticist21-20261008/captured-fixtures/inline_table_after_field_or_bookmark--inline_table_after_bookmark_gets_its_own_row_after_merge/0244.hwpx` | `1792e69cfaf2daa3bb1ee12ec6dc28ffb28e85f56bcfdb87e28068e843f59cfc` | `96ec8c928a03b02f78952fffd5ae7ba57a34b468b56c40c3abcff3a1d17500d0` | `a3257aa5-eb48-4a11-a626-9ef46704b1b7` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures/inline_table_after_field_or_bookmark--inline_table_after_field_gets_its_own_row_after_merge/0249.hwp` | `21819f0c2aaad0f4a2064df266a0f59ec80ee1d2233925e5c63e887f72e00b36` | `미산출` | `c604511f-5b0a-45d2-b334-100b08ccc1b7` / failed |
| `output/pr-review/semanticist21-20261008/captured-fixtures/inline_table_after_field_or_bookmark--inline_table_after_field_gets_its_own_row_after_merge/0250.hwpx` | `4536c5dce2d6bd9141c91440a21fcd47410bb141f44b8e7fb0d5fa3c7de9e382` | `0ae2eadcf1a1ad0bab34290333ac47caf488a71b800659630bd65b14e80eec4b` | `7fbb943c-b6f2-46c4-8221-6ad172fc29fb` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures/inline_table_after_field_or_bookmark--inline_table_gets_its_own_row_after_merge/0255.hwp` | `4f1e524661775dfbc569166f75f2289f1402a020f77c33a7e0a3f9776874d423` | `미산출` | `0f87ea4b-57d9-4505-b5cf-9a1e8c7050ae` / failed |
| `output/pr-review/semanticist21-20261008/captured-fixtures/inline_table_after_field_or_bookmark--inline_table_gets_its_own_row_after_merge/0257.hwpx` | `dbaa6d12fde562d8f1a59448b2edd6946e601e6ccab36b72a415a23fb2ef4193` | `f888cfaeaa2e5b06e1d07c5d5cafb0272857f7059ab885accb8805cb6e7a6cd4` | `1c2a2207-7bdf-4294-9c27-592c748b5852` / succeeded |
| `output/pr-review/semanticist21-20261008/wasm-margin-following-134a646e5.hwpx` | `08d1e92569756587702448f82aefbf86337d7fdcfd900d82643a23a81bbae114` | `534bbfdbf1b68b9da6acb44269f0e20eebd2363e59fd4ab66b38d1b9fdcdc118` | `a5e07f52-3382-4c36-a374-bd9c1767bfc0` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures/table_properties_outer_margin_sync--outer_margin_edit_moves_table_now_and_after_reopen/0522.hwp` | `c5d54efe1d6d10a58d200250c3a2fb409144efa05b332e8361f5bcd760c750ad` | `6a53375c45a68d6bf5fb53bdb8f7572e97fb04cd4955660d26def1c5aa6ec604` | `f2f342a0-4c50-4b10-baaf-af644a45cf17` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures/table_properties_outer_margin_sync--outer_margin_edit_moves_table_now_and_after_reopen/0523.hwpx` | `c083046f7120f86b4e39cbd1e524fee6b1be789ca1680cc0f5e702fe52b610d4` | `3a6ccff438807eb571e9468312ba80fa2d68ac3f5fcb18c343653b3e39fb8e58` | `ff875439-f936-49ef-9302-e7b8006fb46e` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/updated_style_keeps_tabs_and_borders--a_style_tab_stop_moves_the_text_after_the_tab/0545.hwpx` | `e0a6bc5b14ba9c0da977b6319484ce65196199b05f00a16ee12396b22c48bbf3` | `3fac3a82586d718786990e7fc9d16738b993611f878388da37019600c995485d` | `0499a0e4-50b6-45a6-81cd-91f694324a44` / succeeded |
| `output/pr-review/semanticist21-20261008/captured-fixtures-134a646e5/updated_style_keeps_tabs_and_borders--style_borders_fills_and_spacing_reach_the_style_and_its_paragraph/0547.hwpx` | `e1b58ace1b9955a7aa984b580a557cba6d694c69533b89d9daecc09b47b147d0` | `5a9bd75bc850507f45394c240e14f11646ba0c133980e9b24abb9e9581ff0289` | `0f844619-ff94-45cb-89e6-269f1d090eeb` / succeeded |

## 실제 호출 경로와 규칙 판독

- #7656: `float_placement.rs::empty_table_anchor_ladder_lacks_outer_box`의 저장 line origin/단독 표/None split 판독 → `block/entry.rs`의 `ParagraphFloatPlacement {table_top, occupied_bottom}` → typeset table 배치/예약 및 `paragraph_float_placements` 저장 → `layout.rs`의 확정 table_top 소비와 후속 문단 흐름. 다음 문단의 실제 글줄에서 잔여 차이를 관측했다. 요구 높이·예산 실패·이월과 분할/마지막 유닛 경계 검증은 미완료.
- #7658: style JSON → TabDef/BorderFill 참조·borderSpacing → 해당 style을 쓰는 body/cell para shape 및 char shape 변경 → reflow/rebuild_section → renderer/paint에서 소비. 렌더러 파일 미수정만으로 조판 비해당이 아니며, 현재 배경 점유 끝이 독립 Print와 다르다.
- #7657: owned-tree walker가 body 외 table/caption/header/footer/note/master/ruby까지 스타일 번호를 옮기고, HWP master raw PARA_HEADER byte 10 및 section_def 사본을 별도로 보정. Native 실제 sample 저장/다시 열기 계약 통과. 렌더의 para_shape_id·char shape 소비와 style_id clone 수용 조건을 읽었으나 전체 조판 비해당 판독은 아직 완료하지 않았다.
- #7654: leading whitespace 판정(`space`, NBSP, figure space)과 marker prefix의 본래 식을 helper로 추출했으며 paint에 소비하는 문자열/char shape/UTF-16 처리 식은 유지. cursor query가 같은 helper로 원본 offset을 변환한다. Native/fresh WASM에서 한글·astral 문자·끝 공백의 그린 경계와 캐럿을 대조했다.

## 원 PR 피드백과 CI

| PR | 실제 게시 review | 상태 |
| --- | --- | --- |
| #7613 | [5445340620](https://github.com/edwardkim/rhwp/pull/7613#pullrequestreview-5445340620) | CHANGES_REQUESTED / API body·head 재확인 |
| #7641 | [5445376862](https://github.com/edwardkim/rhwp/pull/7641#pullrequestreview-5445376862) | CHANGES_REQUESTED / API body·head 재확인 |
| #7616 | [5445441420](https://github.com/edwardkim/rhwp/pull/7616#pullrequestreview-5445441420) | CHANGES_REQUESTED / API body·head 재확인 |
| #7653 | [5445441918](https://github.com/edwardkim/rhwp/pull/7653#pullrequestreview-5445441918) | CHANGES_REQUESTED / API body·head 재확인 |
| #7656 | [5445750827](https://github.com/edwardkim/rhwp/pull/7656#pullrequestreview-5445750827) | CHANGES_REQUESTED / API body·head 재확인 |
| #7658 | [5445751499](https://github.com/edwardkim/rhwp/pull/7658#pullrequestreview-5445751499) | CHANGES_REQUESTED / API body·head 재확인 |

원 PR head 17개를 live 재조회했다. #7614 exact source의 CodeQL `Perform CodeQL Analysis` 실패, #7652 exact source의 `Check Build & Test worker results` 실패와 queued worker, #7653 정책/reuse/adapter 실패는 별도로 보존했다. `gh run view --log-failed`는 4개 요청 모두 내용이 비어 있어 원인 로그를 확보했다고 보고하지 않는다. 세부 snapshot: `current-state-17.json`, `github-run-*.json`. CI 취소·재실행·설정 변경은 수행하지 않았다.

## 남은 완료 조건

1. 아직 미검증인 원 PR 편집/출력 경로, 반례, source-side 회귀 주장 판독을 완성한다. #7655 다섯 무늬 상태의 실제 패턴·색도 읽고 점수로 대신하지 않는다.
2. 2026-10-08 사용자 지시로 90% 미만 사례는 메인터너 보정으로 처리한다. 같은 누적 branch에서 독립 Print를 근거로 원인별 code 보정을 만들고, 변경한 소비 경로와 대조군을 재검증한다.
3. 최종 통합 code head에서 Rust lint 묶음, workspace build, 전체 nextest와 변경 범위의 필수 gate를 완료한다.
4. gate 충족 후 같은 통합 PR에 필요한 source input/Print PDF/대표 review·overlay/통합 기록/오늘할일을 넣고 정확한 head image URL을 실제 표시한다.
5. 통합 PR CI 모니터링·exact-head merge·upstream/devel 포함 검증·원 PR/이슈별 반영 및 credit·archive/assets·devel sync·소유 확인 뒤 cleanup까지 진행한다.

이 문서는 진행 기록이며 전체 목표의 완료나 승인 보고가 아니다. 실행 로그·중간 JSON·TSV는 ignored output에 보존하고 generated suite는 source PR에 stage하지 않는다.

## 승인된 메인터너 보정 계획 (2026-10-08)

- 승인 원문: “90% 미만인것들 메인터너 보정으로 처리”.
- 대상: #7613의 연속 표 원점, #7616의 inline 표 앞 글자 정렬·뒤 글줄, #7653+#7656의 표 뒤 문단 흐름, #7658의 문단 테두리·배경 점유. 좁은 중첩 표 반례(#7641)는 독립 Print 지표도 확보하여 기존 통합 blocker를 함께 판독한다.
- 규칙: 독립 Print의 실제 괘선/글줄/점유 끝과 입력의 저장 line 원점·높이·간격을 대조한다. 측정과 배치가 같은 결과를 소비하도록 고치며 문서 ID·절대 좌표 예외, 출력 숨김, clamp로 겹침만 제거하지 않는다.
- 생산/소비 추적: block/entry가 만드는 ParagraphFloatPlacement → typeset 요구·예약 높이/예산·이월 → layout의 table_top/후속 flow; composer 줄 구성 → paragraph_layout의 마지막 줄/양쪽 맞춤; paragraph_layout의 para_border_ranges → border_rendering의 최종 점유 상자.
- 검증: 작은 기존 경계와 동일 입력의 Native 직접 출력부터 보정 방향을 확인한다. code head별 fresh WASM을 새로 빌드하고 전체 관련 TSV·대표 review/overlay를 직접 읽는다. 90% 이상과 실제 결함 해소가 확인된 뒤 필요한 정식 관계 검사를 추가하고, lint/전체 회귀/통합 CI/merge/후속 처리로 진행한다.

### 보정 1: 문단 외곽선과 채움의 점유 상자

`paragraph_layout.rs`의 외곽선 점유 조건이 `fill_color.is_none()`을 요구해 테두리+채움인 문단의 후행 줄간격을 배경 상자에서 누락했다. 가시 외곽선의 존재로 점유 조건을 정하고, 같은 상자의 좌우 간격도 `para_border_ranges`에 넘기도록 보정했다. 새 문서의 식별자·절대 좌표 분기는 추가하지 않았다.

- 수정 전 Native/fresh WASM: 테두리/배경 82.33902%, 탭 대조군 100%.
- 수정 후 Native 진단 build: exit 0, 4분 19초. 두 입력 모두 100% (`native-updated_style_keeps_tabs_and_borders-{scores,review}-border-draft`).
- 직접 확대 판독: 녹색 문단 배경의 빠진 후행 영역과 좌우 범위가 개선됐다. Native 채움 하단 158.9333px / Print 160.3040px 및 stroke raster 차이는 잔여 차이로 기록한다. fresh WASM 재검증과 최종 판정 전 추가 판독이 필요하다.
- 기존 #5711 음수/양수 줄간격 관계 검사 2/2 PASS (`logs/fixture-capture-issue_5711_para_border_negative_spacing-border-draft.log`). 테스트 원본·기대값은 바꾸지 않았다. 새 정식 회귀 검사는 아직 추가하지 않았다.
- Native binary 및 source 변경은 현재 로컬 보정 단계이며 세 Clippy·전체 nextest·최종 WASM·CI·merge 증거가 아니다.

#7655 다섯 상태의 실루엣은 양 backend에서 100%였지만 직접 판독에서 patternType 2(세로줄)와 5(십자)가 대각선/대각 격자로 그려졌다. serializer와 공용 PatternFillInfo는 1~6, SVG/WebCanvas의 switch는 0~5를 소비하는 실제 불일치를 확인했다. 별도 원인으로 보정하며 점수 100%로 면제하지 않는다. Print의 무늬 bitmap은 100dpi에서 8 pixel 주기(96dpi 출력 약 7.68px)이고, 현 renderer tile은 6px라 주기도 대조 대상이다.

### 보정 2: 공용 무늬 번호 소비

SVG와 WebCanvas의 switch를 serializer/PatternFillInfo/CanvasKit과 동일한 1~6 번호로 맞췄다. Native 진단 build exit 0(4분 28초), 다섯 편집 상태의 review 생성 성공. 0003의 직접 판독에서 세로 방향은 Print와 일치한다. 6px tile과 Print 약 7.68px 주기의 차이는 남아 있으므로 무늬 전체 완료로 판정하지 않는다. fresh WASM 및 실제 Canvas backend 검증도 남아 있다. 테스트·기대값은 변경하지 않았다. 증적: `native-cell_background_pattern_change-review-pattern-draft/`.

### 보정 3: 비어 있는 표 앵커와 뒤 문단의 독립 점유

단 맨 위 `None` split 표의 저장 사다리가 표 바깥 상자를 담지 않은 경로에서, `ft.host_spacing.after`의 저장 앵커 lh+ls를 다음 글자가 있을 때만 표 뒤에 붙였다. 확정 `occupied_bottom`을 표 상단 + 실제 높이 + 아래 바깥 여백 + 문단 뒤 간격으로 정했다. `place_table_with_text` 예약과 layout의 `lane_flow_bottom`이 같은 확정 끝을 소비하며 후속 문단은 빈 줄도 자기 높이를 차지한다. 독립 Print의 두 입력에서 셋째 문단 baseline은 모두 332.117px, 글자가 있는 둘째는 310.860px다.

- Native build exit 0, 4분 26초(`logs/native-maintainer-empty-anchor.log`).
- 동일 기존 입력 0015/0021 review 최저 모두 100%: 이전 100%/86.01714% → 100%/100%. 전체 1쪽이며 표 괘선·뒤 문단 순서·간격을 `empty-anchor-enlarged.png`로 직접 비교했다.
- 기존 source 계약 1/1 PASS(`logs/fixture-capture-column_top_table_reserves_outer_margins-empty-anchor-draft.log`). 정식 nextest는 별도로 남아 있다.
- 증적 `native-column_top_table_reserves_outer_margins-review-empty-anchor-draft/`. fresh WASM, 예산 실패·이월 대조, 신규 정식 회귀는 아직 미완료.

### 보정 4: composer 줄 소속과 개체 상자의 우선순위

- #7616: 현재 문단의 줄 경계·높이·폭과 조합 결과가 같은 경우 UTF-16 제어 슬롯의 소유 줄을 `composed_tac_line_assignment`로 측정(`tac_control_indices_for_line`)과 paint가 공유한다. 저장 줄과 편집 재조판 줄은 각 출처 안에서 조회하며 혼합 사다리는 기각한다. 저장 캐시 수용 조건을 바꾸지 않았다. 경계 충돌이 있는 TAC 문단은 별도 inline painter 대신 일반 composed painter로 보내 줄별 정렬·높이·표 위치를 함께 소비한다.
- 공백 없는 양쪽 맞춤은 slack을 글자 사이 N-1곳에 배분한다. 기존 N 분배는 마지막 글자 뒤에 여분을 남겨 잉크 오른쪽이 안쪽에 놓였다.
- #7613: 표 공간을 담지 않는 빈 앵커의 확정 개체 상자를 saved-host 포맷보다 먼저 선택한다. 단 시작과 본문 중간 모두 `table_top/occupied_bottom`을 fit·예약·layout lane에 전달한다. 다음 표 앵커의 안내 줄간격 600HU(8px)를 앞 표 아래에 더하지 않는다.
- 중간 진단 `shared-rows-draft`: inline 73.33928%, 연속 표 두 경로 38.42346%; 부족한 실제 간격을 확인하여 추가 보정했다. 이 결과는 완료 근거가 아니다.
- 추가 보정 Native build exit 0, 4분 21초(`logs/native-maintainer-row-gaps.log`). 동일 입력 전체 1쪽 TSV: inline 일반/누름틀/책갈피 세 건 모두 100%, 연속 표 HWP/HWPX 두 건 모두 100%. `*-row-gaps-enlarged.png` 다섯 장에서 앞 글자 맞춤, 표 괘선/간격, 뒤 글자·끝 문단의 순서와 간격을 직접 대조했다.
- Native 진단 계약 95 PASS / 1 FAIL(알려진 좁은 중첩 표 반례), 원 source/기대값 변경 없음. 94 source assertions(기존 92 + #5711 2)와 여백 뒤 문단 반례 통과이며 정식 nextest 증거는 아니다.
- 아직 필요한 범위: fresh WASM/대표 review, 예산 실패·이월에서 안내 줄이 다시 추가되지 않는지 확인, 정식 관계 회귀, lint/전체 회귀.

### 추가 intake

진행 중 새 비 draft [#7659](https://github.com/edwardkim/rhwp/pull/7659), head `db6f70d9c523692080f5318e406a578c11e6c32e`를 발견했다. 신규 각주 스타일 번호와 모양 선택을 수정하며 원 head의 시각/Studio gate는 미실행이다. head fetch 및 jangster77 reviewer 지정 후 `47a304211`·`cd0eb2d25`로 누적 적용했다. 기존 source assertions 3개는 Native 진단에서 통과했으며 실제 인쇄와 fresh WASM 검증은 남아 있다. 기존 17 head는 변경 없음을 재확인했다.

### 진행 중 보정 5: 빈 앵커 예산과 HTML 중첩 표의 실제 점유

- `EmptyHost` 흐름을 명시하여 확정 개체 상자의 후보가 예산에 안 맞아도 더 작은 legacy 높이로 교체되지 않게 한다. 높이 요구·예약·paint 완료는 동일 흐름 판정을 소비한다.
- 독립 작은 페이지 입력(`maintainer_object_boxes-0001-a6554951e542`)은 body 175px이며 Native/Hancom 모두 표를 첫 쪽, 뒤 문단을 둘째 쪽에 놓았다. 아래 바깥 여백까지의 예약 끝은 body를 초과할 수 있으므로 전체 표의 실제 괘선/후속 내용 관계를 확인하고, 예약 높이 자체를 페이지 쪽수만으로 fit 증거라고 하지 않는다.
- 좁은 HTML 셀의 1인치 폭 하한을 제거하고 composer 토큰/자식 개체의 최소 폭을 열 그리드에 아래에서 위로 예약했다. [CSS 자동 표 배치](https://www.w3.org/TR/CSS2/tables.html#auto-table-layout)의 셀 최소 내용 폭 원칙을 적용한다.
- 첫 폭 보정 뒤 source assertions + 진단 반례 99/99 PASS. 재생성 input `f7d474366850...`, Print SHA `9a8ab74341e5182b95f5a77f68519d77cd1c48864024fd94fcd115068c1afe41`, Native 99.48187%였으나 확대 PNG에서 ‘안’의 오른쪽/아래가 잘렸다. **실제 결함 미해결**로 판정하고 정식 테스트 추가를 보류했다. 원본 입력의 Print도 잘리므로 재생성 대조군 통과를 원본 개선으로 바꾸어 보고하지 않는다.
- 실제 원인: import 폭/LineSeg는 cell.padding=141HU를 쓰지만 `apply_inner_margin=false`의 공통 측정·paint는 table.padding=510HU를 쓴다. 현재 `Cell::effective_padding`을 import 폭과 reflow에 소비하고 `HeightMeasurer::measure_table`의 행 크기를 cell/common/raw 저장 크기 및 host LineSeg에 함께 반영하는 보정을 진단 중이다.
- 첫 유효 셀 상자 build는 exit 143으로 종료되었고 성공으로 보고하지 않는다(컴파일 오류 출력 없음). 동일 범위에서 무늬 보정까지 묶은 build를 재실행했다.

### 진행 중 보정 6: 독립 Print의 여섯 무늬

- 1~6 모두 포함한 동일 입력 `216166ae3b7d760fd03d9ebb297e8bb268fca5a13e0534380a8a8465901a58ae`; Hancom job `eee5c48f-346a-4856-9dd9-67fa8707dd20`, PDF SHA `9ac1553a70c91cb86d890ea9d9e2b50cd5c34bc230420db3512a2a9f7e9b41e4`.
- 실제 100dpi bitmap은 8dot 주기, 쪽 원점 기준이다. 3번은 우상향 slash, 4번은 우하향 backslash로 기존 SVG/WebCanvas/CanvasKit의 대각 소비와 반대다. 기존 번호만 맞춘 보정을 전체 무늬 완료로 취급하지 않는다.
- 현재 공용 `PatternFillInfo.tile_size`에 576HU 반복 주기를 출력 DPI로 변환해 생산하고 SVG·WebCanvas·CanvasKit에 전달한다. CanvasKit도 원래 도형 path의 채움 shader를 사용하도록 해 줄이 도형 밖으로 그려지는 기존 경로를 제거한다. WebCanvas의 반복 변환은 [CanvasPattern 표준](https://html.spec.whatwg.org/multipage/canvas.html#dom-canvaspattern-settransform)을 따른다.
- Native/fresh WASM/실제 Canvas 두 backend의 직접 확인·최종 gate는 아직 남아 있다. Skia 기존 단색 fallback을 실제 무늬 출력으로 보고하지 않는다.

#### 보정 5/6 Native 재검증

- Native build v1은 drawing style에 DPI 전달을 빠뜨린 컴파일 오류로 exit 101. 호출 지점에 DPI를 전달한 v2는 exit 0(4분 26초).
- 101개 진단 실행 PASS: 97 source assertions + 2 기존 반례 + 2 관측용 입력 생성. 관측용 2건은 관계 assertion을 갖춘 정식 회귀가 아니다. source/기대값의 완화는 없다.
- 새 narrow input `af77438524a5...`, Print job `a7fbd9b2-df30-468f-b799-154ddacde385`, PDF SHA `b92e437f398ecf1b3243fec9ef34a7ad588f2e28a13072500f524fa1f6b71339`. `narrow-effective-cell-box-native-print-zoom.png`에서 ‘안’의 오른쪽/아래가 온전히 보이고 부모·자식·옆 셀의 순서와 위치가 Print에 대응한다. Native gate는 통과했으며 fresh WASM은 남아 있다.
- 무늬 6종의 `six-patterns-common-boxes-v2-zoom.png`에서 방향과 반복 주기의 큰 차이가 해소됐다. stroke raster 강도와 1~2px 위상 잔여 차이는 기록한다. Native 6종 gate 통과, Canvas 실제 경로/fresh WASM 검증은 남아 있다.
- 최초 작은 용지 사례는 Native 61.2844%였다. 독립 Print의 MediaBox가 314×595pt, 입력은 width59528/height31549/WIDELY로 가로세로 순서가 맞지 않았다. 한컴에서 종이가 세로로 선택되어 표 오른쪽이 잘린 것이다. 원 입력·PDF·실패 증거를 보존하고, width30000을 표 생성 전에 설정한 portrait 대조군을 별도로 생성했다. 원 입력 통과로 바꾸어 보고하지 않는다.
- Studio TypeScript `npx tsc --noEmit`: exit 0. 실제 브라우저 E2E 증거는 아니다.

### 보정 5/6 fresh WASM 및 실제 Canvas 확인

- 고정 code head `f58718f70436e2ee9899539648e1a72f4ea00864`. Native binary SHA `e67cc8e87f83354544aa22c5edfeecc0508678e4f98541305bcd2439734b7645`, WASM SHA `b32bb4711ffcf8092bcf9a3c6b51a713851c2edcd5c3794c81d4ebaf5a9e833a`, JS SHA `e5bd272394d3c54a433af0b90e2851a75c9b03f6acd9735b5c025c7e4738d4ec`. output의 `build-provenance-f58718f70.json` 및 고정 binary/package 사본으로 재현한다.
- 루트 `CARGO_TARGET_DIR=target/pr-review scripts/wasm-pack-locked.sh --target web --out-dir pkg --no-opt` exit 0, 9분 48초. 진단 빌드이며 최종 head 검증은 별도로 남아 있다. pkg/public JS·WASM hash 일치와 public contracts 6/6 PASS.
- fresh WASM 동일 입력 최저값: 연속 표 HWP/HWPX 100/100%, inline 일반/누름틀/책갈피 모두 100%, 단 맨 위 표 뒤 빈/가시 문단 모두 100%, style 탭/테두리·배경 모두 100%. 좁은 유효 셀 상자 새 입력 `af77438524a5...` 100%, 여섯 무늬 모두 100%. 이전 잘린 입력 `f7d474366850...` 99.48187%도 보존하며 새 입력 통과로 이전 출력을 개선했다고 하지 않는다.
- 실제 Chromium Canvas2D/CanvasKit software replay로 여섯 무늬를 확인했다. nearest shader는 소수 DPI에서 대각선이 점선처럼 보였고, 현재 TS Linear 보정 후 `patterns-canvas-linear-print-zoom.png`에서 방향·주기 큰 차이가 사라졌다. stroke AA/1~2px 위상은 잔여 차이다. CanvasKit 제목은 host font 미등록으로 Noto Sans KR fallback이며 글꼴 정합 증거가 아니다. Skia는 기존 단색 fallback이어서 무늬 지원으로 보고하지 않는다.
- #7566 실제 Studio의 `loadDocument → setActiveField → refreshDocumentAgentMutation → clearActiveField → refreshDocumentAgentMutation`을 1행/80행 중첩 표에서 실행했다. 소유 cellPath 보존, 안내문 1→0→1, pageCount 1/4, 실제 visible-page render 완료 경계까지 PASS(`browser-wrapper-activation-f58718f70.json`). 초기 harness의 없는 메서드 호출은 수정하여 재실행했으며 제품 결함으로 세지 않는다. 현재 최종 WASM 재실행은 남아 있다.

### 진행 중 보정 7: 주석 참조 크기·기준선과 미주 시작 경계

- 독립 Print의 8/10/20pt 본문과 각주/미주 여섯 public API 입력을 생성했다. 참조 번호는 nominal size의 약 75%, 본문 실제 기준선보다 약 0.21em 위였다. 선두 미주 control slot 10pt와 뒤 8/20pt text slot을 구분했다. 선두 빈 각주 참조가 숨겨졌던 기존 분기도 보정한다.
- 생산/소비: raw control slot char shape → nominal row font / 75% marker size → 공통 측정 line max와 실제 TextLine 최종 기준선 → FootnoteMarker baseline → SVG/WebCanvas/Skia/paint JSON/CanvasKit. 조기 provisional baseline으로 선두 번호를 발행했던 진단 v2는 실패해 최종 첫 TextLine에서 발행하도록 수정했다.
- Native note-flow-boundary-v2 binary SHA `6774ccd3358f87c44207478843e0f48cc5c5b19f48b9739aecddc2abb7cbed98`, build exit 0(4분 53초), 진단 101/101 PASS. 주석 크기 여섯 input 최저값은 98.42374~100%. 빈 각주/미주 동일 입력은 99.4152/100%. 선두 번호의 실제 확대 판독과 fresh WASM은 아직 남아 있다.
- 마지막 NoLS 일반 본문의 외곽선 없는 terminal occupied height를 미주 시작에 공유한다. 외곽선은 기존 점유 줄간격을 유지하며 저장 LineSeg/개체 host 경로를 새 계약으로 대체하지 않는다. 확정 separator flow_origin은 PageItem → layout이 소비한다. 새 origin을 pagination convergence 비교에 포함해 오래된 페이지 cache 재사용을 막는다.
- 혼재 각주+미주 여섯 독립 입력의 Print: FE는 두 쪽, F/E 단독은 한 쪽. 각주 footer 점유가 있으면 미주를 새 쪽으로 넘기도록 보정했으나 새 쪽에 불필요한 구분선·여백이 남아 v2 최저 30.50431%로 실패했다. 실제 새 쪽 첫 미주는 본문과 나눌 경계가 없음을 직접 확인하여 현재 예약/배치 공통 경로를 추가 보정 중이다. Footnote inline-after-text 미지원 경로를 검증했다고 보고하지 않는다.
- 실제 상공신문 편집 input `b3c9896ea898...` 88.42212%, HWP3 sample 편집 input `6b4f6a375029...` 최저 41.35911%는 여전히 실패다. 전자는 chart/data·본문 재배치, 후자는 둘째 쪽 본문 원점 차이를 직접 확인했다. 참조 번호 수정만으로 전체 출력 완료로 판정하지 않는다.

### 추가 발견과 현재 검증 범위

- 올바른 filename context(`보고서.hwpx`)로 HF template 20개를 재생성·Print했다. header/footer4는 86.61972%, header10 74.10072%, footer10 76.25355%. 직접 확대에서 tab 구간 밑줄 누락과 번호 끝 위치 차이를 확인했다. 파일명 mismatch인 이전 캡처도 보존한다. 현재 intra-run tab의 표시 문자열과 모델 placeholder를 구분하는 보정 중이다.
- field/control slot 23개 및 leading-definition 7개 전체 TSV를 추가 확보했다. 빈 누름틀 안내문은 인쇄에서 숨겨졌지만 내용 좌표를 옆으로 밀고 있었다. 현재 안내문 삽입 전 실제 내용 상자를 생산 위치에 보존해 인쇄 레이어가 같은 상자를 소비하도록 보정 중이다. editor 안내문/활성화와 저장 text/control 슬롯은 유지한다.
- inline equation이 든 셀에 여러 HTML 문단을 붙인 input `87a311ae9d91...` 최저 15.79909%: Native는 표를 앞 글자보다 위에, Print는 앞 글자 뒤에 놓았다. 원 source의 token/slot assertions 통과로 실제 순서 결함을 면제하지 않는다. wide TAC host 저장/재조판 경로를 추적 중이다.
- proper portrait budget input `55aa3cfd4cd8...`: 기존 Native p1 32.11845/p2 0% → EmptyHost의 실제 table body fit와 후속 occupied flow를 구분한 보정 뒤 두 쪽 100%. body-only miss/physical paper miss 대조군 및 fresh WASM 확인은 남아 있다.
- 위 추가 보정은 현재 dirty 진단 단계다. 새 정식 rendering 회귀/fixture/기대값을 추가하거나 완화하지 않았다. 통합 PR 생성·승인·merge·후속 처리 완료가 아니며 최종 code head Native/fresh WASM 직접 판독과 필수 lint/full gate가 남아 있다.

#### 보정 7 이후 Native 진단 결과

- note-guides-v3 build exit 0(4분 18초), binary SHA `5d0a828310cb226810739d66136898ea169d2aec43de8fe7cbd230ce3dd4adca`, 기존 진단 101/101 PASS. 혼재 주석 여섯 입력은 최저 99.71126~100%, 두 쪽 FE에서 불필요한 미주 구분선을 제거했다. 아직 fresh WASM/정식 관계 회귀는 남아 있다.
- 누름틀/컨트롤 23개는 v3에서 18개가 90% 이상, 다섯 실패는 넓은 TAC 표/그림 혼재다: 0079 8.88818%, 0114 12.74659%, 0132/0158 14.37790%, 0148 7.72191%. 처음 실패했던 0069는 0.55249→100%로 인쇄 내용 좌표가 개선됐다. 원 input을 바꾸지 않고 비교했다.
- filename-context HF 20개 v3: header/footer4만 86.61972%로 남았다. header/footer10 밑줄 누락은 개선되어 90% 이상이지만 `hf-header10-display-width-print-native-v3.png` 직접 확대에서 숫자와 밑줄 오른쪽 끝이 약 6px 밀렸다. 통과 점수로 면제하지 않는다.
- 8/10/20pt의 plain `L\tR`, filename/page field, field 뒤 X를 독립 Print했다. plain right tab은 실제 tab stop에서 끝나고 field 포함 block의 정렬 폭은 가시 glyph 폭보다 큰 것으로 관찰했다. 단순 6px 이동을 추가하지 않고 control slot/field box 출처를 추적 중이다. Hancom 저장 HWP 대조도 확보했지만 저장 파일명에 따라 PATH 값이 바뀌므로 최초 input의 정합 근거로 바꾸지 않는다.
- HTML cell-flow-v4 build exit 0(4분 54초), binary SHA `debbb9809350d5e83838aae737d696ab216cddf34db5b0a8c90cf7b47d59a9f5`, paste/caret 진단 5/5 PASS. 셀 문단 vpos/inline host LineSeg를 새 점유로 발행했으나 새 input `a100f93c3c2d...` / Print SHA `67b32e51f3907802a54be129a67b218a80c456e4dd6317df1eb19a0481b5ed63`의 Native는 12.24209%였다. 앞 글자 누락과 표 위치 결함이 남아 **보정 완료가 아니다**. 원 input `87a311ae9d91...`도 보존한다.
- TAC host-frame-v5 build exit 0(4분 25초), binary SHA `deba366df3e1b51116b581fe5a76647c5100fc6bd70b75791a7a22509cd12ad9`, 진단 101/101 PASS. NO_LS 본문 TAC frame에서 별도 머리말/꼬리말을 폭·높이 소비자로 오인하던 거부 조건을 제거하고 표 바깥 여백/실측 높이를 공용 줄 메트릭에 반영했다. 동일 0079 input은 8.88818→74.63732%로 개선했으나 표 뒤 글자가 같은 줄로 붙어 여전히 실패다.
- 추가 원인: 새 inline table factory가 `common.treat_as_char=true`인데 `table.attr`에 TABLE-record 값 `0x04000006`을 넣고 `common.flow_with_text`/CTRL_HEADER 흐름 flag를 누락했다. live/native, HWP 저장, 순수 HWPX 재열기의 flow 판정이 갈린다. 현재 실제 inline 생성 경로의 CommonObjAttr/CTRL_HEADER/cache를 같은 flags로 발행하도록 보정 중이다. 기존 TABLE record attr와 기존 저장 자료의 수용 조건은 바꾸지 않는다.


### 진행 중 보정 9: 표 생성·셀 편집의 실제 줄 소속과 독립 개체 슬롯

- inline table factory의 CommonObjAttr/CTRL_HEADER 흐름 flag를 일치시켰다. 삽입 전 host end를 고정하고 `reflow_paragraph → recalculate_section_vpos`로 뒤 문단 좌표를 갱신한다. TABLE record 속성은 별도의 기존 계약을 유지한다.
- HTML cell paste는 flat/deep 경로 모두 셀 문단 reflow/vpos 갱신 뒤 공통 HeightMeasurer의 `reserve_content_heights → host reflow → section vpos`를 소비한다. 저장 줄의 TAC 소속은 실제 raw control slot과 줄 경계를 사용하며 높이 매칭을 소속 증거로 삼지 않는다.
- Native v7 binary SHA `3fd77aaa4c49e1de094bf1e9182865da02e055cb05d42099c0e4a5afc0f74504`, build exit 0(4분 21초), 진단 101/101 PASS. paste API 재생성 input `433acb46d996...`의 독립 Print PDF SHA `4c6b83ee91a1b39989cee3a973f82fa2046ce70b1792be09ab41185cbf083be4`, job `04af4d5c-1ed7-444a-bf5c-932e3087390f`, PrintMethod=0. Native 98.41537%, 앞 글자/표/뒤 문단 순서 회복. `paste-html-inline-equation-cell-v7-zoom.png`에서 수식 글꼴·연산자 간격 차이가 남아 있으므로 완료 판정은 보류한다.
- NO_LS 혼합 표 host는 표 폭을 뒤 글자에 합치던 방식 대신 원시 control slot을 별도의 U+FFFC 프레임 토큰으로 투영한다. 원본 IR은 변경하지 않는다. 개체 폭/높이와 원시 경계는 공통 occupied-size 측정 → Frame → LineSeg → TAC 배치가 소비한다. 표 끝과 뒤 글자는 다른 물리 줄을 소유할 수 있다.
- Native v9 binary SHA `5717c2c2cd1efb9eb276d5c9b37e0a7e4a8510709fefd9d14c32714feaeb08df`, build exit 0(4분 30초), 진단 101/101 PASS. 기존 Print 입력 24개 중 23개가 90% 이상(최저 99.35414%). `0079-896269c1daf6` 및 API 재생성 `0079-f05117fd896e` 모두 100%. 원본 review PNG에서 앞 글줄 → 표 → 끝 글자/그림 순서와 외곽선을 직접 확인했다.
- 기존 `0148-9dee19f77e58`은 옛 factory가 만든 잘못된 한 줄 cache를 유지해 7.72191%다. 수정 API로 재생성한 `0148-f00e5cab1285`는 실제 두 줄을 저장하고 독립 Print와 Native v10에서 100%. Print job `81e31328-57c9-4252-aa62-5bcf5fd7841f`, PDF SHA `a90f5a82274104102221544a08530f5dfdcb59fb57aae2f61e4bcf454c1b09f6`. 새 산출물 통과를 옛 저장본 렌더 개선으로 보고하지 않는다.
- 추가 Native v7 확인: auto-number trailing placeholder 네 입력 최저 97.09229~97.61647%. leading definition 일곱 입력 중 `0378-39da46633749` 86.31922%는 남아 있고 나머지는 98.24937~100%. fresh WASM 및 최종 확대 판독은 미실행이다.

### 진행 중 보정 10: 본문 각주 소유와 실제 예약 상자

- 상위 문단에 표가 있다는 이유로 형제 본문 각주까지 등록하지 않던 `!has_table` 분기를 제거했다. `FootnoteSource::Body`와 table-cell fragment queue의 `TableCell` source는 별개이며 셀 각주를 중복 등록하지 않는다. bodyless 판정은 새 각주의 원인이 아니었고 그대로 유지했다.
- Native v8 SHA `1d7c1acb84dce29bf445816e4f1ea9e62abb92b8f3915ef3575b8b4e74cbeb7e`, build exit 0(4분 37초), 진단 101/101 PASS. HWP3 변환 입력의 누락 각주는 표시됐으나 2→3쪽 과분할이 발생했다. 이 결과를 통과로 세지 않는다.
- 단일 단의 등록된 본문 각주도 `LayoutEngine::estimate_footnote_area_height_with_metrics → reserve_painted_footnote_area`를 사용한다. 실제 구분선/위아래 여백을 포함한 상자에는 추정용 40px safety margin을 중복 차감하지 않는다. 진입 fit와 분할 루프가 같은 `available_height`를 소비한다. 다단 경로는 이번 공통 측정 적용 밖이며 검증을 승격하지 않는다.
- Native v10 SHA `f0bb932e113837181649f85ccf3c91515b823129b61e4d77ffd081bc8e01f6e1`, build exit 0(4분 34초), 진단 101/101 PASS. 혼재 주석 여섯 독립 입력은 99.71126~100%를 유지했다. HWP3 입력은 다시 2쪽이나 p1 78.05031%, p2 41.35911%: 삽입 전 저장 쪽 끝의 빈 문단을 감추는 cache/guide 경로가 남아 있다. 143E chart 입력은 88.42212%다.
- 각주 삽입으로 가용 높이가 달라진 뒤 문단의 이전 프레임 cache를 API에서 무효화하는 v12 보정을 검증 중이다. 문단 내용과 글자모양은 보존한다. 재생성 입력의 독립 Print/Native를 먼저 확인하며 옛 입력의 실패 증거는 보존한다.

### 진행 중 보정 11: 오른쪽 탭의 동적 필드 상자

- 8/10/20pt filename/page field, 일반 `L\tR`, field 뒤 `X` 대조군의 독립 Print를 생성했다. 필드 블록은 반각 종단 슬롯을 정렬 폭에 포함하지만 일반 문자 블록은 그렇지 않았다. 고정 px 이동 대신 source control/range와 해당 char shape의 글자 크기로 폭을 구한다.
- `right_tab_fields::tail_space/apply_block_width → right_tab_block_width/estimate_line_runs → emit_line_runs → compute_char_positions`로 측정·배치가 같은 정렬 폭을 소비한다. 현재 독립 증거는 한 개 동적 필드가 있는 RIGHT tab 블록이며 여러 필드/개체 혼재 블록은 새 지원으로 주장하지 않는다.
- Native v11 SHA `beda69a9db570d50dc1bbfc2bed219c8868f71d6dc59d50a14d8581a2273da9c`, build exit 0(4분 22초), 진단 101/101 PASS. 초기 컴파일에서 별도 float-host 관측형을 LineSeg로 잘못 취급한 추가 조회는 제거 후 다시 빌드했다. 현재 filename 20개, 크기 대조 6개, plain 3개, suffix 2개를 비교 중이며 최종 결과·확대 판독·fresh WASM은 아직 남아 있다.

현재 code head 뒤 보정은 모두 로컬 진행 중이다. fresh WASM은 여전히 `f58718f70`이므로 위 Native 결과를 최종 WASM 검증으로 사용하지 않는다. 새 정식 렌더링 회귀/fixture/golden은 추가하지 않았다. 남은 90% 미만, 수식 확대 결함, 최종 lint/full nextest/Studio/통합 CI가 해소되기 전에는 통합 PR 생성·승인·merge를 하지 않는다.

- Native v11 최종 filename 20개는 모두 90% 이상(최저 90.22403%). 다만 추가 크기 대조의 PAGE 20pt는 89.91884%로 실패했다. 이 경우 종단 AutoNumber의 표시 placeholder는 raw control의 가시 위치와 달랐다. `resolve_auto_number_placeholder_positions`를 표시 치환과 정렬 폭 조회가 공유하도록 v13에서 보정했다.
- Native v13 SHA `f23f2a6ff0d7923d8571b879b97759b0fcc3f9e1d625bd050a3d45429b346074`, build exit 0(4분 25초), 진단 101/101 PASS. 동일 Print의 8/10/20pt 파일명·쪽번호 여섯 입력은 99.7331~100%. 20pt review PNG에서 번호·underline 끝의 큰 위치 차이가 사라졌다. 최종 확대/전체 head/fresh WASM은 남아 있다.
- Native v12 cache 무효화 입력 HWP3 `0ca435b9dae9...`의 독립 Print job `cae7a3cd-ddde-4685-8cbd-2244c6c0715e`, PDF SHA `71451896fb854b306d2c948aa4b0c9d30488d7763d5ad6b7f6c3f6f681d36716`. p2는 92.06827%로 빈 줄 소속이 개선됐지만 p1은 51.1407%로 악화했다. 첫 control-only host의 참조 누락·표 앞 폭·뒤 줄간격이 남아 있어 이 cache 변경을 완료로 판정하지 않는다. v12 binary SHA `c6be6b2de1d6cedff2dd1bb1eea7bef4763b4e2523998d5d56800422cb13dfd2`, build exit 0(4분 36초), 진단 101/101 PASS.
- control-only 문단에서 개체마다 없는 가시 문자 위치를 늘리던 fallback을 가시 경계 0으로 투영하는 v14를 검증 중이다. 원시 UTF-16 control 순서는 보존한다.
- 143E OLE의 길이 prefix와 `Contents` 원문을 ignored 경로에 보존했다. 기존 bounded VtChart 6 reader는 객체 그래프를 끝까지 읽었지만 column type 1을 거절했다. 독립 source metadata에는 axis 0..2000/4 divisions와 계열 fill `#73b1d1/#cd6565/#ebc452`가 있었다. 일반 fallback의 1702 축·다른 팔레트와 구별되며 아직 제품 코드에 적용하지 않았다.

- v14 Native SHA `6044ab9a9e21e681e789aff274c97c71792d479f0526ed6d0d4f9d67ea45ad43`, build exit 0, 진단 101/101 PASS. 그러나 빈 문단의 모든 `control_text_positions`를 0으로 만들면 주석 삽입 명령이 같은 경계의 표 뒤에 주석을 넣는 부작용이 확인됐다. 새 입력 `9aa1284b66ce...`와 Print job `a0d3724a-208e-4686-950c-bda67f6aed5d`를 보존하고 해당 전역 변경은 되돌렸다. 기존 v12 입력 비교는 여전히 p1 51.1407%로 실패했다.
- v15는 TAC 표와 같은 저장 줄을 소유하는 비-TAC Square 도형 때문에 후행 간격을 버리는 paint 분기를 보정한다. 원시 control 슬롯의 실제 줄 소속을 확인하며, 선행 그림·표·TAC 도형의 별도 점유 경로는 유지한다. 측정은 `tac_reconcile::measure`의 소유 줄 간격을 이미 소비하고 있었다.

- v15 Native의 최종 실행 파일 SHA는 `98616861774255774e93b6cbbe497e7bc98f5e6f2e16e87b692ad72f93d9648b`다(build exit 0, 4분 46초). 빌드 완료 전 이전 실행 파일을 복사한 오류를 확인해 다시 고정했고, Cargo 종료 뒤 진단을 다시 실행해 101/101 PASS를 확인했다. 같은 v12 입력/Print p1은 51.1407%→77.15709%로 개선됐으나 실패다. `hwp3-host-trace-v15.log`의 Table 뒤 흐름은 162.6px였다. 원시 저장 각주/표 줄의 간격 절반만 소비되고, 뒤 문단의 이전 vpos를 무효화했으므로 나머지 간격을 복원할 소비자가 없었다.
- v16은 `tac_host_trailing_spacing_at_boundary`를 `tac_reconcile::measure`, `stored_tac::prepare`, 실제 `layout_table_item`이 함께 호출하게 한다. 다음 문단의 저장 줄이 없거나 무효인 재조판 경계에서는 마지막 원본 TAC 줄의 양의 간격 전량을 소비한다. 원래 저장 줄/현재 RowBreak 줄의 기존 절반 간격 계약과 음수 Fixed 경로는 유지한다. `note_reference::leading`은 일반 문단과 빈 표 호스트의 번호·스타일·전진폭을 공유하고, 표 호스트는 같은 원시 소유 줄에서 표보다 앞선 참조만 그린다. 코드/시각 검증 진행 중이다.
- v13 최종 머리말·꼬리말 파일명 문맥 20개 전체 TSV가 완료됐다. 모두 100%, 누락 없음(`native-hf_filename_context-scores-field-slot-metrics-v13/summary.json`). 추가 8/10/20pt 여섯 Print 대조는 99.7331~100%이며 `hf-tab-20pt-v13-zoom.png`에서 번호·파일명·밑줄 끝을 확대 확인했다. SVG 재조판 잔차와 fresh WASM 확인은 최종 head에서 별도 수행한다.

- v16 Native SHA `84552c6c155c5faa5deb1d4f71eafb0a13cd1f5d47ec90c4d2f13b9dd879ad71`, build exit 0(4분 21초), 기존 진단 101/101 PASS. 기존 `#6812/#6879/#6167/#6179/#7160/#5708` 검사 원문을 ignored driver에서 실행한 32/32도 PASS(기대값 변경 없음). v12 HWP3 입력/같은 독립 Print 두 쪽의 full review gate가 Native에서 통과했다. 원래 v10 입력과 fresh WASM, 참조·간격 확대 확인은 별도다.
- 추가 확인: v15에서 최신 각주 API를 거친 143E 차트 입력 `8394cca6978e...`는 81.87141%로 실패, 옛 입력의 88.42212%와 혼동하지 않는다. 선두 정의+HTML 표+종이 기준 개체 입력 `39da46633749...`도 86.31922%로 남았다. HTML 표 factory의 raw 속성 `0x082A2311`에는 flowWithText·Table numbering·Para 기준 등이 있지만 수동 생성 common은 이 값들과 달랐다. v17은 `parse_common_obj_attr(&raw_ctrl_data)`로 양쪽을 같은 source에서 해소한다. 새 편집 입력과 독립 Print를 다시 생성해 검증한다.

- v16/v17의 같은 HWP3 v12 입력 p1 95.51084%, p2 92.07237%. `hwp3-note-reference-v16-zoom.png`에서 선두 참조 번호·표 안 글자·첫 본문과 하단 번호/구분선을 확인했다. 표 소속과 선두 참조 누락은 해소됐고 글꼴 raster·기존 footer 폭 잔차는 남아 있다. FE/E/F 여섯 대조군은 v16 최저 99.71126%로 유지됐다. 반면 이전 API가 남긴 원래 입력 `6b4f6a375029...`는 v16 p1 82.00126%, p2 41.35911%로 실패한다. 새 입력 통과를 옛 입력의 해결로 보고하지 않는다.
- v17 Native SHA `10719d1c5fba335efd534fcb0582782f3aa6f972967dbec592fa7c12c1e8a8ca`, build exit 0(4분 21초), 진단 101/101 PASS. 수정 source 21개와 base `301840aec6df20cc1dac4cf2a9e20ccccabebf3a`의 hash manifest를 ignored `source-code-v17/`에 고정했다(기존 diff patch에 빠지는 새 모듈 원문 포함). 선두 정의+HTML 표+종이 기준 개체의 새 입력 `4a66fd38f7f6...`, Print job `2a5bdcdd-06c5-4fbb-b5ea-9c0656e3c4aa`, PDF SHA `6bbce4b616b7afb7daad3c5e23e302ca65afbd0d14664923bf64cc959121903d`를 보존했다. full Native review와 fresh WASM 검증은 진행 중이다.
- 차트 metadata 추가 진단에서 저장 VtFont 14byte payload의 크기(f32 8/9/10pt), legend 배치/section rect를 확인했다. `metadata_probe.log`와 `chart-metadata-v15-zoom.png`는 ignored 진단이며 product parser/renderer 확장은 아직 적용하지 않았다.

- e13595c90 fresh WASM build exit 0(9분 22초). `pkg`/Studio public의 JS SHA `e5bd272394d3c54a433af0b90e2851a75c9b03f6acd9735b5c025c7e4738d4ec`, WASM SHA `f80df687085fffaaec3fe1ebc20f95def81a5f30e587c01768afec8021a73368`가 각각 일치했다. ignored `wasm-pkg-e13595c90/`에 고정했고 API/시각 검증은 진행 중이다. v17 새 HTML 표 저장본 `4a66fd38f7f6...`의 Native full review는 100%다.
- 옛 HWP3 각주 입력 추적: `hwp3-old-note-cursor-trace-v17.log`에서 TAC paint 134.1..157.0px 뒤에 오래된 HWP3 높이 부족 보정이 160.7px로 흐름을 다시 늘리고, 후행 간격·여백 후 170.0px가 됐다. 실제 소유 LS는 1996HU로 표 본체 1716HU와 상하 여백 280HU를 이미 담았다. v18은 저장 LH가 실제 표 높이보다 작을 때만 그 fallback을 적용한다.
- 같은 옛 입력의 빈 문단 34..37은 원래 본문 프레임에 들어가지만 각주 예약 뒤 영역을 넘는다. 기존 tail absorber가 35..37을 숨기고 navigator(38)의 저장 reset을 즉시 적용했다. v18은 유효한 저장 빈 줄의 요구 높이를 실제 fit/이월로 소비하고, 그 빈 줄이 이미 연 다음 쪽에서 같은 reset을 중복 적용하지 않는다. `HeightCursor`도 실제 빈 줄에서 시작한 단 원점의 증거로 저장 reset 뒤 사다리를 재고정한다. 새 source 변화의 Native/시각/대조군 검증을 시작한다.

- e13595c90 fresh WASM: 공개 API 계약 6/6 PASS. 실제 Studio 새 브라우저에서 1/80행 중첩 표의 2단계 cellPath와 guide 활성화/복원 1→0→1, pageCount 1/4를 확인했다(`browser-wrapper-activation-e13595c90.json`). 새 HWP3 각주 입력은 WASM 최저 92.07237%, 새 HTML 표 입력은 WASM 100%로 Native와 같았다.
- v18 첫 build는 새 모듈 `Paragraph` import 누락으로 exit 101이며 수정했다. 재빌드 중 `HeightCursor.col_anchor_y`가 paint에서는 용지 좌표, typeset에서는 단 상대 좌표인 사실을 코드와 대조해 기준점을 중복 차감하는 계산 오류를 확인했다. 검증용 해당 빌드만 종료하고 `y - col_anchor_y`로 고친 뒤 재빌드한다. 잘못된 build를 검증 증거로 사용하지 않는다.

- v18 Native build exit 0(4분 30초), 실행 파일 SHA `ae3b5fc14e8ee09afc5513b16c988b706a234d9bd68d89904deea32ceb263ec2`. 진단 101/101, 기존 표·각주 경계 32/32 PASS(기대값 변경 없음). 원래 HWP3 저장본 `6b4f6a375029...`과 원래 Print의 full Native review가 p1 약 95.47%, p2 92.07237%로 개선됐다. `hwp3-old-pages-v18.json`에서 빈 문단 36/37을 실제 다음 쪽으로 이월하고 navigator(38)가 같은 쪽에서 뒤따르는 것을 확인했다. 두 review PNG에서 참조·표·본문·하단 각주·뒤 쪽 순서를 직접 대조했다. source-code-v18/에 새 모듈을 포함한 source manifest를 고정했다. 이 결과의 fresh WASM과 정식 관계 회귀는 아직 남아 있다.
- e13595c90 fresh WASM filename HF 20개 모두 100%, 크기별 필드 6개 최저 99.7331%, 누락 없음. 20pt 필드 review PNG를 직접 대조해 번호·파일명·밑줄 끝 배치를 확인했다. v16 주석 크기 대조 6개의 Native 최저는 98.42374%다. 이 검증을 v18 이후 source 전체 검증으로 승격하지 않는다.

### 진행 중 보정 12: HYhwpEQ Latin 글리프와 공통 전진폭

- ignored `equation_font_probe--hancom/`의 8/10/20pt, italic 소문자·대문자, rm, 숫자·연산자 독립 한컴2020 Print를 확보했다. 실제 변수는 소문자 E0E5..E0FE/대문자 E000..E019, 숫자는 E03D/E034..E03C, rm은 ASCII 글리프를 쓴다. 일반 Latin Modern 글리프와 다르다. `x+y`의 수정 전 보조값은 83.0303%, 기호 대조는 85.32189%였다.
- HYHWPEQ.TTF의 cmap/hmtx(1024 UPEM)와 Print 전진을 대조했다. 48dpi 논리 크기를 반올림한 뒤 glyph advance를 반올림하는 두 단계가 10pt/20pt 변수 전진과 일치했다. 고정 px 간격 대신 source 글꼴·AST 스타일·글자 크기로 선택·전진을 산출한다. 필요한 글꼴을 기존 review 공급 경로에 실제 제공했고 font hash는 별도 로그에 남겼다.
- 측정 producer `hancom_glyphs::measure → EqLayout::layout_hancom_glyphs`의 선택 글리프/개별 원점을 SVG·Canvas2D·Native Skia·CanvasKit layer JSON이 소비하도록 보정 중이다. 아직 Native/fresh WASM/Skia/CanvasKit 직접 검증 전이며 정식 회귀/fixture/기대값은 추가하지 않았다. glyph mapping이 확인되지 않은 문자와 bold/특수 font style은 기존 경로를 유지한다. 저장 수식 bbox의 최종 가로 scale 및 본문/셀 기준선 소비는 별도로 대조해야 한다.

- `3d2709c07bcf693e8ef7316048cb981f46abd192` fresh WASM build exit 0(9분 22초), WASM SHA `05ec62a17acb9856deb5e7c1f77eff643d986cca509a94cc9ebc51c3e04cf3bf`, root/pkg와 Studio public 일치. 원래 HWP3 저장본/원래 Print 두 쪽 fresh WASM 최저 92.07237%, full review gate passed, 공개 API 6/6 PASS. Native와 같은 문단 소속·본문·각주 두 쪽 출력을 확보했다. 현재 수식 dirty 변경의 검증으로 승격하지 않는다.

- v19 Native build exit 0(4분 14초), binary SHA `4baa6a66c3e9b580d91bbbf14bb63671115931809a61886a3e4bc9f4dfd343dd`, 진단 101/101 PASS, Studio TypeScript는 closure 내부 union narrowing 오류 수정 뒤 exit 0. `source-code-v19/` snapshot을 보존했다. 같은 `x+y` Print full review는 86.70886%로 여전히 실패한다. `inline-xy-v19-zoom.png`에서 마지막 bbox 압축과 operator baseline 차이를 확인했다.
- 독립 한컴 Save한 x+y HWP도 원래 1955x1000 상자를 유지했다. 앞뒤 본문을 추가한 Print 대조에서 다음 글자는 저장 control advance를 소비하고 수식은 자연 글리프 크기로 그려졌다. v20은 독립 검증된 unprotected linear Latin 경로에서 공통 `stored_horizontal_scale`을 SVG/Canvas2D/Skia가 사용하고 CanvasKit JSON에 같은 scaleX를 발행한다. 보호 크기·비선형/mixed 경로를 같은 증거로 확대하지 않는다. Native Skia의 공급된 custom Hancom face도 실제 paint에 전달한다. 현재 검증 중이다.

- v20 Native build exit 0(4분 7초), SHA `e52d51da566fc20ed881b98a55f16caa969707cfec7397a8ea8ae6ff2e658135`, 진단 101/101 PASS, Studio TypeScript exit 0. 같은 실제 HTML 셀 수식 input `433acb46d996...` Native full review 100%. 독립 x+y 8/10/20pt 및 앞뒤 본문 대조도 완료된 다섯 입력은 100%. 전체 대조 batch는 아직 실행 중이며 `inline-xy-v20-zoom.png`에서 operator baseline 잔차를 확인해 완료로 선언하지 않았다.
- 추가 Print: baseline70에서도 operator/variable의 상대 기준선 차이가 유지되고 X+X/단독+/rm 숫자를 대조했다. v21은 Roman face ascent(820/1024)와 formula baseline의 차이를 공통 GlyphRun에 발행한다. 이 수치 근거와 글리프 위치/확대 결과는 별도로 보존한다. 연산자의 pad는 같은 AST row의 실제 앞뒤 피연산자와의 경계에서만 계상하며 단독/연속 연산자에는 내부 이항 간격을 넣지 않는다. 알려진 ×÷≤≥의 Unicode 글리프도 같은 메트릭으로 선택한다.
- v12의 각주 뒤 모든 문단 cache 무효화는 2단 차트 host(문단7)의 유효한 저장 줄·단 소속까지 지우는 부작용이 있다. v18은 원래 저장 줄을 유지한 HWP3 입력에서 이미 Native/fresh WASM 통과했다. v21에서는 뒤 문단 무효화를 제거하고 host reflow/본문 pagination이 예약 프레임의 실제 수용을 계산하도록 되돌린다. 새 API 산출/같은 Print 비교는 아직 검증 전이다. 새 정식 회귀·fixture·기대값은 추가하지 않았다.

- v21 Native build exit 0(4분 7초), binary SHA `13ec4ecd88a34591e41e55f0dfde930e0a74242fde6d785d231b9d198979c806`, 진단 101/101 PASS, TypeScript exit 0, 실제 셀 수식 100%. 연속 연산자 대조는 86.83093%로 남았다. SVG 원문에서 `DIVIDE`를 ÷ 대신 여섯 변수로 출력하는 기존 명령 누락을 확인했다. 같은 script의 Print는 ÷를 출력한다. v22는 기존 DIV/DIVDE와 함께 DIVIDE를 같은 연산자로 해소한다.
- v21 새 각주 API가 만든 차트 입력 `0466.hwpx`의 hash가 원래 `b3c9896ea898...`로 돌아왔다. 원래 저장 줄·단 소속을 보존하며 v12 잘못된 무효화 산출물 `8394cca6978e...`를 해결된 원본으로 바꾸어 보고하지 않는다. 차트 원본의 88.42212% 실패는 여전히 남아 있다.
- v22의 수식 가로 여백: common.margin의 좌/우를 inline advance에 포함하고 같은 여백으로 glyph content box를 얻는다. `composer::collect_tac_controls`/`tac_control_occupied_size → Frame/tac width → paragraph_layout`의 두 실제 분기와 table/table_partial/textbox fallback이 같은 helper를 소비한다. 앞뒤 본문 Print에서 저장 상자와 실제 glyph 원점의 좌측 여백을 대조했다. 수식 상자의 높이·원시 control 소속/내용은 변경하지 않는다. 새로운 검증은 진행 중이며 정식 회귀/fixture/기대값은 추가하지 않았다.

- v22 native-skia 첫 build는 새 `Font::new` 호출에 Option을 전달한 타입 오류로 exit 101이며 실제 Typeface/기존 default 생성 분기로 수정했다. 이후 새 수식 font 수집기의 Footnote/Endnote와 Header/Footer를 같은 타입으로 묶은 오류도 exit 101이며 각각의 실제 모델 타입별 arm으로 수정했다. 실패 build는 검증에 포함하지 않는다. 문서 fontsUsed에는 본문/셀/주석/header/footer/textbox/캡션의 실제 수식 font name도 발행해 공급 단계에서 요청할 수 있게 한다. 실제 브라우저의 공급된 face·Canvas2D/CanvasKit 확인은 아직 남아 있다.

- v22 native-skia 정상 build exit 0(4분 13초), SHA `e787ea69b78f731d61f7e46c773d4e25928ee5f0891d5f28e8dbde236f8504cb`. optional feature 도입 뒤 진단 driver의 serde_json 후보가 build/runtime 둘로 늘어난 문제는 Cargo artifact JSON의 runtime opt-level3와 실제 native library 경로로 해소했고 진단 101/101 PASS. 연속 연산자 대조와 실제 HTML 셀 수식 Native full review 모두 100%. 독립 x+y 및 HTML 셀의 실제 Skia PNG도 산출했다. `inline-xy-skia-v22-zoom.png`에서 hinting으로 굵어진 stem 차이를 확인해 v23은 supplied face의 outline을 subpixel/no-hinting으로 출력한다.
- CanvasKit preflight에도 실제 HancomGlyphs 사용을 기준으로 필요한 HyhwpEQ face를 발행한다. Source 문서에 없는 바깥 face를 임의로 정하는 것이 아니라 수식 컨트롤이 지정한 face다. 실제 face 공급과 브라우저 replay 검증은 다음 검증에 포함한다.

- v23 native-skia build exit 0(4분 16초), binary SHA `2a4cc6ee2aafc09f3532683955a21c170bad65fa20377fed6996a71c652f7354`, 기존 진단 101/101 PASS(5.99초), Studio TypeScript exit 0. `source-code-v23/manifest.json`에 실제 source 24개를 고정했다. `inline-xy-skia-v23-zoom.png`에서 독립 Print와 supplied face의 실제 Skia 출력을 확대 대조했고 v22의 굵은 + stem 차이가 줄었다. 전체 15개 Native 수식 full review는 실행 중이며 fresh WASM/Canvas2D/CanvasKit은 아직 미검증이다. 이 checkpoint는 완료/승인 판정이 아니다.

- v23 수식 대조 15개 Native full review는 최저 94.92834%였으나 Roman20 확대에서 sans fallback을 확인했다. SVG font 수집기가 HancomGlyphs의 지정 face를 Latin Modern Math로 잘못 수집했으므로 v24에서 HyhwpEQ의 실제 face를 임베드하도록 보정했다. 점수 통과만으로 완료하지 않는다.
- d649eebe6 fresh WASM build exit 0(9분28초), SHA `c24cc52118df4713631d797cd41db9bd0ba6a032dfdfde964806bea51f38ad66`, pkg/public 일치. 실제 CanvasKit supplied face는 fallback 0으로 x+y/Roman20을 그렸다. Canvas2D는 원본 TTF의 U+FFFF→65535 매핑 때문에 OTS가 파일을 거부했다. renderer 사본의 잘못된 종료 매핑만 glyph0으로 정정했고 정상 Unicode cmap/hmtx/glyf와 모든 정상 테이블이 그대로임을 확인했다(`font-terminal-repair-contract-v24.json`, 사본 SHA `c00c5b74ee92c66212adc60dd54cb289266bec8b67c90ec9858537579a97e1b4`). 로드 뒤 Studio display chain이 실제 감지된 원 face까지 제외하는 별도 소비 경로를 확인해 보정 중이다.
- 옛 0148/0378 입력을 한컴2020으로 정상 저장한 독립 HWP SHA는 각각 `f1161a36fed5501a3eecadbaca017b53287be7e74eb3ebf3a3d1ae335d1cc00a` / `5573322c52bb768a99f7aceeae2b06f8c976130f8899d7c3ea7b8d9503c461a4`. 전자는 저장 한 줄→두 줄(ts18), 후자는 table1000→1282/hostLH1566→1848/후행vpos+282HU로 정정됐다. 이 근거로 source box 모순을 판정하는 v24 실험을 했으나 구성된 새 줄과 최종 표 소유/저장 cursor의 소비가 갈려 0148이 3.3694%, 0378은 86.31922%로 남았다. 해당 불완전한 cache 실험을 제품 source에서 복원하고 ignored source-code-v24/와 실패 증거를 보존했다. 새 정식 회귀/fixture/기대값은 추가하지 않았다.
- v24 Native native-skia build exit 0(4분12초), binary SHA `46751f62fdc8977978c391419919fb368581e4877ece39ed32c103557fba996b`, 기존 진단101/101 PASS. 이 수치로 옛 cache 결함을 해결했다고 판정하지 않는다. TypeScript와 기존 SFNT 검사는 통과했고, font chain 보정 뒤 실제 브라우저 재검증을 계속한다.

- v24 SVG 임베딩 보정 뒤 Roman20은 실제 HyhwpEQ face를 포함하고 같은 원본 Print와 Native full review 100%다. `roman20-font-supply-v25-zoom.png`에서 serif 형상을 직접 확인했다. Canvas2D의 뒤 소비 문제는 감지된 실제 원 face가 `fontFamilyCandidatesForDisplay`의 originalAllowed에서 누락된 것이었다. v25는 현재 호출의 installedFaceName 결과를 표시 체인에 포함하며, 명시적 빈 confirmedLocalFonts는 이전 감지를 사용하지 않는다. 새 유한 대체 mapping이나 font-rule registry/projection은 추가하지 않았다. 기존 font-substitution/SFNT/host-canvas 검사20/20 및 TypeScript PASS. 실제 브라우저 Canvas2D/CanvasKit에서 supplied/detected HyhwpEQ의 Roman20 serif 형상을 직접 확인했고 원본 TTF는 Git에 추가하지 않았다. Native v25 exact build와 최종 fresh WASM은 아직 진행 중이다.

- v25 Native build exit0(4분11초), SHA `1d08597b7707a2e5d9d7fc072b7cfda1e1de01cb71005fc026e68883a157ea10`. 같은 15개 수식 TSV 최저98.85057%/누락0, 실제 HTML 셀 full review100%. `5b46702643ed84326c3405f8a9cafc3ec7ef5b36` fresh WASM exit0(9분29초), SHA `d7f97245c2cffcf3731fe401c74ed329a72f68617c0c09dec5dff61cfc58c861`, root/pkg/public 일치. 이 head의 대문자는 확대 판독상 아직 미충족이었다.
- 전체 수식 확대(`equation-native-font-supply-v25-zoom-{1,2}.png`)에서 PUA 대문자만 정체인 것을 검출했다. 동일 Print의 text matrix는 대문자 `1 0 0.333 -1`이고 소문자/rm은 `1 0 0 -1`이다. Print subset과 설치 TTF의 대문자 glyph bounds/hmtx가 같아 glyph 교체 문제가 아니었다(`equation-font-independent-shear-v26.json`). v26은 대문자 italic의 shear를 GlyphRun에서 생산하고 SVG/Canvas2D/Skia/CanvasKit JSON이 같은 변환을 소비한다. 전진폭은 바꾸지 않는다.
- v26 Native build exit0(4분16초), SHA `05ed2e085652a764f8e315f8c58bdbd7bfc15c66f45042b00ca5ddacbc69df25`, 기존 진단101/101 PASS(5.43초), TypeScript exit0. 대문자26자 및 X+X full review 둘 다100%; `uppercase-shear-v26-zoom.png`에서 Print/SVG/실제Skia의 기울기를 직접 대조했다. build 로그 이름에 tac-cache가 있지만 해당 load-cache draft는 적용되지 않았고 제품 변경은 수식 shear뿐이다. 이 source의 fresh WASM은 아직 남아 있다.
- 실제 셀 브라우저 replay는 공급된 HyhwpEQ와 실제 HCR Batang face를 등록해 Canvas2D/CanvasKit에서 수행했고 CanvasKit 미등록 fallback0이다(`browser-equation-fonts-html-cell-v25-hcr.json`). HCR Batang 실제 설치본 SHA `8664669bd2d6a4fe8b7538706d2bb2a1004c074aa02dca59051e515e731294f5`를 기존 review 폰트 경로의 HANBatang.ttf로 제공했다. 이전 HBATANG.ttf(Haansoft Batang) 공급과 구분한다. 이후 영향되는 본문 비교는 이 실제 face로 재실행한다.


- v27 load producer의 첫 draft는 HWPX Table의 정상 `raw_ctrl_data.len()==0`을 제외하는 잘못된 조건 때문에 원본 둘을 보정하지 못했다. 원시 CTRL byte 존재는 캐시 수용 조건이 아니므로 제거했다. 공용 HTML 높이 producer도 `Table::sync_ctrl_height`로 common.height와 존재하는 원시 CTRL height를 함께 갱신한다. XML 입력에서 raw vector 범위 접근을 하지 않는다.
- v28은 `DocumentCore::from_bytes`에서 최초 compose/pagination 전에 source box가 한 저장 행과 모순되는 지원 TAC host를 정규화한다. 원래 paragraph_flow_end를 먼저 잡고 공용 HeightMeasurer/reserve_content_heights → reflow_line_segs → recalculate_section_vpos를 적용한다. 폰트 대체 폭을 캐시 무효 판정 근거로 사용하지 않는다. 지원하지 않는 여러 행/불균등 단/복잡한 셀은 추정하지 않는다. 실제 측정과 배치는 발행된 동일 모델을 소비한다.
- v28 Native native-skia build exit0, binary SHA `9230c317256180953b6a0fb42f4599b5311de98ec52f8d6b642c868334a2e9f7`; source-code-v28/manifest.json에 변경 source를 고정했다. 기존 진단101/101 PASS(5.67초). 실패했던 **원래 0148/0378 저장본과 원래 Print** full review 둘 다100%/passed다. `old-boundaries-v28-zoom.png`에서 표 외곽/본문/셀 글자/뒤 문단의 포함·순서를 직접 대조했다. 한컴 정상 Save 대조군으로 원본을 치환하지 않았다(정상 Save 0378의 별도 native 진단은86.04899%였음).
- 공개 loader 진단 `cache-load-contract-v28.json`은 다운로드된 독립 Print에 대응하는 저장본176개에서 본문 줄 캐시 변경8문단/표 host5개를 기록했다. 모든 변경 문단의 text와 char_offsets는 유지됐다. 추가3개 HTML/중첩표 원래 입력과 정상 대조군도 다시 시각 검증 중이다. 아직 fresh WASM/최종 전체 gate/새 정식 회귀는 완료하지 않았다.


- c62b44012의 root wrapper fresh WASM은 Rust9분36초 + wasm-opt를 포함한 총18분01초/exit0이다. WASM SHA `4e06c8c8f13433d092f418c011e307cc7e6cf2ce6b4e1cdc6ab1e50b5808e4d6`, glue SHA `7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a`, pkg/public bytes 일치. 후속 v29 dirty Rust는 이 컴파일에 포함되지 않았음을 verification-manifest.json에 명시했다.
- 원래0148/0378과 원래Print의 c62b44012 fresh WASM full review 모두100%/passed다. Native 정상 대조 누름틀25건29쪽 최저100%, 선두제어8건10쪽 최저98.15341%/누락0. scores만 산출한 대조군은 별도 직접 판독/최종 gate를 대체하지 않는다.
- `equation-browser-c62b44012-zoom.png`에서 독립 Print와 실제 Canvas2D/CanvasKit 대문자26자 slant 및 Roman20 serif를 확대 대조했다. `html-cell-browser-c62b44012-zoom.png`에서 동일 셀 수식/본문/두 줄 붙이기/셀 뒤 본문의 순서·간격을 대조했다. 세 browser replay는 실제 HyhwpEQ/HCR Batang 제공, face loaded, CK 미등록 fallback0/unsupported0/runtime gate 통과다. 전체 수식 TSV와 대표 WASM full review는 진행 중이다.
- 추가 old 좁은 중첩표 `wasm-narrow-nested-3f4591b1708c`는 v28에서28.91566%로 실패했고 같은 입력 v25도28.91566%여서 신규 회귀로 세지 않는다. 원본도 보정 대상으로 유지한다. 독립 한컴 정상 Save HWP SHA `02f7abe9121c467e08592b36974556a7f871f42956b587529a0f74276e9d0c12`는 첫 셀의 바깥을 sw1440 두 줄/중첩표 원점3200으로 저장했고 같은 원래Print와 Native100%다. 이 대조군 통과를 원본 통과로 바꾸어 보고하지 않는다.
- v29는 거부된 TAC host 안에서 현재 셀의 공용 padding/최소폭/ParagraphBox와 모순되는 한 저장 행의 폭 캐시를 공용 reflow_line_segs로 다시 발행하는 draft다. 중첩 표는 아래부터 공용 높이를 예약하고 기존 셀 편집의 recalculate_cell_paragraph_vpos를 공유한다. size_protect/세로쓰기/다줄·carve·indent의 별도 box는 이 단일행 판정에서 추정하지 않는다. ZERO WIDTH 문자만 있는 host를 실제 가로 점유로 오판하지 않도록 UnicodeWidth의 0폭을 제외한다. Native build 진행 중/아직 검증 전이다. 새 정식 회귀·fixture·기대값은 추가하지 않았다.
- 차트 표현은 기존 bounded reader 진단만 이어갔다. [한컴 공식 차트 형식 revision1.2](https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D_%EC%B0%A8%ED%8A%B8_revision1.2.pdf)의 ChartType/축 범위/font/Fill/Location 정의와 reader가 확인한 source metadata를 연결했다. 진단 PDF/hash와 offset/field 기록은 chart-input-inspection/에 보존했다. 폰트가 고정된 resize/legend font 진단은 임의의 동일 비율 확대와 point10 맞춤 상수가 정당하지 않음을 보였다. 새 chart engine·제품 parser 확장은 적용하지 않았으며 원래 차트88.42212%의 실패는 남아 있다.


- c62b44012 fresh WASM 수식15개 TSV는 최저100%/90%미만0/누락0이며 `not_evaluated`와 full review를 구분한다. 별도 대표 대문자26자/Roman20/실제HTML셀 WASM full review 3건도 각각100%/passed다.
- v29 Native native-skia build exit0(4분35초), SHA `b1630662dc898d79477ca9275bbc738d3a5fc780455063a31656a7870d330c3f`, 기존 진단101/101 PASS(5.38초). **원래 좁은 중첩표/원래Print** full review는28.91566→100%/passed다. `narrow-nested-original-v29-zoom.png`에서 바깥의 두 줄/옆 셀/중첩 시작 원점을 확대 대조했다. 중첩 안의 일부 잘림은 독립 원래Print에도 있으며 원래 문서의 잘림을 해결했다고 주장하지 않는다. 새 HTML import producer의 너비 예약 검증과 구분한다.
- 추가 영향 원래0148/0378/old HTML narrow/old HTML equation/current HTML equation의 v29 Native full review는 각각100/100/100/96.99223/100%로 통과했다. 복수 행/float carve의 정상 저장 box를 이 비교로 재조판하지 않도록 v30에서 셀 내부 floating control과 non-TAC nested table은 이 single-row width 비교의 적용 경로에서 제외했다. float의 배제 프레임은 별도 근거가 필요하다. v30 Native build 검증 중이며 v29 결과를 v30 결과로 승격하지 않는다. 새 정식 회귀/fixture/기대값은 아직 보류다.

- v30 Native native-skia build exit0(4분12초), SHA `07e9710edf3dad361c14d31786953cc45c8444803d197269afe0654cfcf49b0a`, 기존 진단101/101 PASS(5.90초). 원래0148/0378/좁은중첩표 full review 모두100%/passed다. source-code-v30/manifest.json과 native-cache-originals-review-v30/에 고정했다. 이 checkpoint의 fresh WASM은 root wrapper `--no-opt`로 진행한다: Rust release 최적화 빌드는 유지하며, 앞 checkpoint에서8분 이상 추가된 선택 wasm-opt 단계는 렌더 계약 확인 범위에 필요하지 않다. 완료 전에 새 결과로 보고하지 않는다.


- df28acd633d4834da4489fbbc3264a187f860eed fresh WASM `--no-opt` build exit0(9분26초), SHA `a67796e8b104cf36971612c72096fdd9c0e13cb44c1c2ce694d764f03ac643c2`, glue SHA `7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a`, pkg/public 일치. 원래 좁은 중첩표/원래Print WASM full review도100%/passed다.
- 정상 한컴 Save leading HWP의 별도86.04899% 차이를 source→measurement→placement로 추적했다: 유효 hostLS1848/ctrl.height1282/child cellSz1000/childLS1000+padding282에서 HeightMeasurer의 required row는1282HU였으나 `resolve_row_heights_with_common_fit → trust_declared_row_heights`가 `every_row_is_declared_plus_cell_padding`만으로1000HU로 덮어썼다. Table bbox13.3px와 뒤 문단 원점이 실제 측정에서3.76px 줄었다. source 캐시 자체를 무효화할 근거가 아니었다.
- v31은 별도의 object box가 측정 행합을 HU 단위로 정확히 확인하는데 cellSz 행합과 다를 때, padding 중복이라고 추정해 paint만 줄이는 교정을 적용하지 않는다. 임의의 문서ID/point 상수/px 위치를 추가하지 않았다. 기존 issue1663의 `cellSz + padding` 보정은 object box가 선언 행합을 확인하는 대조 경로에서 유지한다.
- v31 Native native-skia build exit0(4분12초), SHA `e6133a90b87ab00a3268af6b4a8301f148ec4c871af6b39e059028c9b5b5e179`, 기존 진단101/101 PASS(6.00초). 원래0148/0378/좁은중첩표 full review 모두100%, 유효 한컴 Saved leading 대조군86.04899→100%/passed, 원래HWP3 두 쪽은 최저92.07237%/passed다. source-code-v31/와 native-measured-row-boundaries-review-v31/에 실제 source/출력을 고정했다. 이 측정 소비 보정의 fresh WASM과 정상 issue1663 반례는 다음 검증 범위다.
- 원격 목록 재확인에서 초기18개 head SHA는 유지됐고 새 비draft #7661/#7662/#7663이 추가돼 현재21개다. 신규3개는 아직 체리픽하지 않았으며 이번 통합 포함 여부를 별도로 확인 중이다. 초기18개 보정/검증은 계속한다.


- e38f6156f5f7cc86e55d16b6d6fe55a60e9e6ea0 fresh WASM `--no-opt` build exit0(9분23초), SHA `4ffb444ef670ab837a0ba990e8f8b05a8b22d329600fd61b73622edd496070de`, glue SHA `7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a`, pkg/public 일치. 원래0148/0378/좁은중첩표 및 정상Saved-leading full review 모두100%, 원래HWP3 두 쪽 최저92.07237%/passed다.
- 기존 issue1663 대조군은 Native v30/v31 모두 p1 87.31118%/p2 80.8944%다. 앞뒤 실제 PNG SHA가 두 쪽 모두 같고21행 경계도 동일하다. 새 guard 때문에 생긴 회귀라고 세지 않는다. 이 baseline의 미통과를 숨기거나 현재 전체 검증 통과로 보고하지 않는다. `coanchor-independent-v31-p2.png`에서 같은 행 수와 경계를 확인했고 검증 범위를 확대할 때 별도 기존 출력 차이를 확인해야 한다. 새 정식 회귀/fixture 추가는 아직 보류다.
- 전체 nextest를 위해 manifest --prepare 완료(1504 sources/28 harnesses+20 exceptions/48 targets). 최초 build에서 `tests/cases/equation_command_goldens.rs::dump_layout`가 새 LayoutKind::HancomGlyphs를 처리하지 못하는 exhaustive match 오류가 났다. 일반 golden 경로는 EqLayout::new로 Hancom 전용 노드를 만들지 않으므로 baseline을 갱신할 이유가 없다. 보조 dump arm을 보완한 뒤 이 검사와 전체 nextest를 재실행한다. 실행 전 compile failure이며 회귀 test FAIL 재현으로 세지 않는다.
- 기존 bounded-reader Plot136 payload의 후보float source field0/field1 및 tail120/124를 수동 변경한 한컴 Print 진단에서 현재4개 막대의 좌표/폭은 모두 그대로였다. 앞의1.0 또는 뒤0.5 값을 xGap이라고 추정해 제품의 막대 폭을 정하지 않는다. 각 source/PDF hash와 정상zip 검사/수동 변경 provenance는 chart_gap_probe--hancom/, chart_plot_tail_probe--hancom/ 및 독립 관측 JSON에 남겼다. 제품 차트 renderer는 아직 바꾸지 않았다.

### 진행 중 보정 22: 전체 검증에서 확인한 회귀 원인

- 전체 nextest 최종 exit100: 10605 실행/10580 PASS/25 FAIL/50 SKIP, 실행957.029초. 완료 로그와 실패 원문은 `logs/nextest-hancom-helper-retry-v32.log`, 목록은 `nextest-final-summary-v32.json`에 보존했다. 파생 manifest/suite와 기존 baseline은 변경하지 않았다.
- 정책 문서의 실제 CLI 조판은 f58718f70/v8에서215쪽, 실제 각주 예약 보정 v10 이후 v31에서216쪽이다(`policy-pagecount-stage-comparison-v32.json`). p67의 각주 소속과 표의 이월이 달라지는 실측 회귀를 확인했다. 측정 완료 상태와 본문 하단 앵커 선택을 하나의 bool로 표현한 경로를 분리하여 대조 중이며 아직 통과로 보고하지 않는다.
- 차트 범례font10→10.001/9.999의 독립Print는 표시 글꼴 크기9.96pt를 유지하면서 원래2행/22.70px marker에서1행/15.18px marker로 바뀌었다(`chart-font-cache-independent-observations-v32.json`). VtTextBlock::SetFont/LegendSize의 실제 호출도 대조했다. 원 source 캐시 재사용과 변경 뒤 재측정이 다르다는 증거이며 point10 예외로 구현하지 않는다.
- Contents header의 extent/DPI 후보만 바꾼 독립Print는 원래 막대·범례 좌표와 같았다(`chart-header-independent-observations-v32.json`). 이 필드에서 표시 배율을 추정하는 가정을 제품에 추가하지 않는다. VtFont14byte는 f32 Size+u16 Style+u16 Effects+inline VtColor(v1/u32 color)임을 설치 라이브러리 ReadFrom과 공식 속성 정의로 확인했으며 종전의 color/여섯 flag 추정과 구분한다.

- v33의 앵커/측정 플래그 분리만으로 정책 문서는 여전히216쪽이며, 관련 nextest7건 중1PASS/6FAIL(exit100)이다. `paragraph::place_split_paragraph`가 진입 fit과 달리 앵커 플래그로 다시 추정용 안전 여백을 차감하는 소비 지점을 확인해 v34에서 측정 완료 플래그를 사용하도록 연결했다. Native build 진행 중이며 아직 개선으로 판정하지 않는다.
- 원래 표 캐시3건의 같은 관계 검사를 보정 전/후 고정 출력에 실행했다(`stale-tac-relational-before-after-v31.json`). 전부 수정 전FAIL/후PASS: full-width 표가 본문과 같은 물리 줄에 놓임, 셀 글줄이 소유 셀 하단을 넘음, 좁은 셀의 본문 줄 나눔이 누락됨을 각각 검출했다. 수동 draft의 빈 TextRun을 내용 글줄로 세던 진단 오류는 고쳤고 실제 내용·줄·중첩 표의 순서를 검사한다. 새 정식 검사 추가는 아직 하지 않았다.
- 차트 캐시 높이만341→227로 바꾼 독립Print 두 진단이 완료됐다. 처음 저장LRect만 바꾼 경우와 textExtent/후행LRect도 바꾼 경우 모두 marker22.70→15.18px/범례2행→1행/plotBottom325.244→366.799px가 됐다. 실제 글자 기준선은 두 진단 사이3.84px 차이여서, 첫 상자와 최종 글자 원점을 같은 값으로 대체할 수 없다. 입력/PDF SHA 및 좌표는 `chart-cached-box-independent-observations-v33.json`에 고정했다. 수동 진단을 정상 fixture로 분류하지 않는다.

- 사용자 선택에 따라 신규 #7661/#7662/#7663은 초기18개 통합 뒤 별도 검토한다. 이번 누적 후보에 추가 체리픽하지 않는다.
- v34의 분할 예산 연결로도 정책 문서는216쪽이었다. 앵커/측정 플래그 분리 시도는 제거했다. v35는 Native 저장 HWP5의 편집하지 않은 각주 본문이 composer에서 원래 LineSeg를 그대로 소비할 때 기존 source page-cut/continuation owner 계약을 보존한다. 누락·합성·편집 줄은 실제 FootnoteArea 재측정 경로를 유지한다. 저장정보 수용 조건 자체를 완화하거나 기대값을 변경하지 않았다.
- v35 Native build exit0(4분16초), binary SHA `a565df8a0ff6e7e3ab1de255417069f3816a35055b46ea9a0aad3628a503c432`. 실제 정책 문서는215쪽으로 복원됐고 기존 각주·표 이월7검사 모두PASS(exit0,1.744초)다. API 진단101/101도PASS(7.27초). 진단 드라이버의 최초Serde linkage 오류는 default/native feature별 산출물 혼용이었으며 default library의 실제 Cargo artifact JSON을 다시 확보해 같은 feature의 의존성으로 연결했다. 제품 검사 FAIL로 세지 않는다.
- 원래 HWP3 각주0462/원래Print의 v35 Native full review는 두 쪽 최저92.07237%/passed다. 새 각주 보정이 유지되는 대조이며 freshWASM은 별도다. 정책 문서215쪽 전수 보조값은 `native-policy-stored-note-cuts-scores-v35/`에서 진행 중이고 쪽수 복원만으로 시각 통과를 선언하지 않는다.
- Title 저장LRect 높이/Section external·internal margin만 각각 바꾼 세 한컴 Print 진단에서 차트 title/axis/bar/legend 좌표는 그대로였다(`chart-section-box-independent-observations-v35.json`). 범례 cache 높이가 실제 배치에 적용된 앞 진단과 다른 결과다. Title 저장 상자를 그대로 재사용하거나 이 margin 필드로 현재 출력 위치를 추정하는 가정은 보류한다.


### 진행 중 보정 23: 저장 각주 복구의 fresh WASM과 남은 회귀

- `bd42c480cbc30a520755487e3564df1ced3e122a` fresh WASM wrapper `--no-opt` build exit0(10분06초), WASM SHA `cbda0b27e7dde0bb6c65202fd34584e8ce1bb975893e8c1ba62a0807d1d4d9bc`, JS SHA `7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a`. `wasm-build-bd42c480c.json`에서 root pkg/Studio public 일치를 확인했다. 공개 API 6/6 PASS. 원래 HWP3 0462/원래Print 두 쪽 fresh WASM full review 최저92.07237%/passed이고 p2 review PNG에서 navigator/screen 본문과 footer를 직접 대조했다.
- v35 정책 문서 Native 전215쪽 보조값 산출 완료: 쪽수와 누락 없음이 확인됐지만 p44 74.4937%, p158 74.72316%, p168 82.93414%, p176 89.0736%, p184 71.10305%, p208 86.19535%, p211 81.66205%로 보류다. 변환 오류가 아니며 전수 gate 통과로 보고하지 않는다. `policy-v35-p44/158/184-compare.png` 직접 판독에서 표·캡션 위치 및 후속 본문/줄바꿈 차이를 확인했다. fresh WASM 전215쪽 보조값도 별도로 진행한다.
- 남은 기존 검사24건 재실행 exit100(6.419초): 6PASS/18FAIL. issue6718의 같은 실제 issue5966 원본/143쪽/본문 하한 검사가 PASS로 복구됐다. 앞서 복구한 6건과 함께 전체25FAIL 중7건이 해결됐으며, 새 전체 nextest 통과를 선언하지 않는다. 목록/원문은 `logs/nextest-remaining-regressions-v35.log`에 보존했다.
- bounded chart reader의 ignored metadata 초안은 원래 Contents 전체9876바이트를 경계·객체 참조에 따라 끝까지 읽어 type1, 3series, 원래 제목/범례 font·CP949 문자열·두 캐시 상자를 회수했다. 숨긴 Z축의 null text 객체는 빈 문자열 객체와 구별해 허용했다. 제품 parser/renderer는 아직 변경하지 않았다.
- 차트 title wordWrap0→1을 바꾸면 title 저장 firstLRect가 실제 출력에 적용됐다. 같은 wordWrap1에서 height391→541은 title baseline132.72→136.44pt 및 plot top을 이동시켰고 legend/plot bottom은 그대로였다(`chart-title-wrap-independent-observations-v36.json`). 설치본의 `VtChartText::Size → SetWordWrap(1) → VtTextBlock::SetWordWrap`가 바뀐 속성에서 캐시를 무효화하는 호출과 일치한다. title 상자가 무조건 무시된다는 앞 가정을 수정한다. 수동 진단 입력은 정상 저장 fixture로 분류하지 않는다.
- issue5598의 두 기존 합성 입력은 원래 page width/height가0/0이며 한컴 Print가 용지 범위 오류로 거부했다. 원본과 실패 job을 보존했고 용지만 A4로 지정한 ignored 대조군 두 개와 실제 원본2995759(Downloads)의 Print를 확보했다. 실제 원본 SHA `1ad6ec8dc787b1e1bda60cfca63890d4d83190065945bb4851633a09f775f9ba`, Print SHA `2b7c4d5d71a79d35e1cdf191857ac93cbfd828d1e2e80413698801921330ec33`. 대조군 통과를 원래0/0 용지 입력의 한컴 일치로 바꾸어 보고하지 않는다.
- v36은 Frame에서 재조판된 projected row도 TAG_IMPLEMENTATION_PROPERTY/soft-wrap 말미 separator 계약으로 판정한다. physical_frame_rows라는 배치 경로를 이유로 separator의 전진을 빼지 않는다. 내부 공백이 없는 이 줄의 char-spacing도 보존 separator 슬롯을 포함해 나누며 다른 줄은 기존 N-1 글리프 경계를 유지한다. 원본5598 검사에서 양쪽 모두 수정 전 bbox 초과3.4px가 검출됐고 변경 뒤 검증 중이다. 새 정식 검사·fixture·기대값·baseline 변경은 없다.

- v36 기존 검사4/4 PASS(exit0,0.076초): 좁은 셀의 내어쓰기/무내어쓰기 두 경로와 issue4956 폭 변경, 음수 자간 마지막 글리프 대조다. header 자체를 수정할 필요가 없었으며 원래 본문의 tab+soft-wrap 말미 공백이 중복 점유하던 같은 원인으로 설명된다. `logs/nextest-projected-wrap-spacing-v36.log`에 수정 전/후 기존 기대값 유지 증거를 연결한다. 새 Native/fresh WASM 시각 확인은 아직 남아 있다.
- 사용자 추가 지시(2026-10-08): 시험지 관련 회귀 실패가 있으므로 관련 회귀 테스트를 모두 제거한다. 시험지 보정은 다른 작업에서 진행 중이다. exam_* 및 3-월별 교육/실전 통합, 시험 해설 발췌 eq-002 입력의 테스트/공유 코퍼스 항목을 AST·입력 연결로 확인하며 제거할 예정이다. 문서 입력·독립 PDF 및 이전 실패 증거는 보존한다. 이 지시는 일반 문서의 실패 기준값 완화나 검증 통과 선언의 근거가 아니다.

### 시험지 회귀 검사 이관에 따른 제거 (2026-10-08)

- 사용자 승인 범위에 따라 시험지 원본 또는 시험지 생성에 연결된 기존 검사 293개를 제거했다(1차268개+잔여25개). 시험지 전용 검사 파일53개를 삭제하고 혼합 파일의 일반 문서 검사는 유지했다. `exam-regression-removal-applied{,-residual}.json`에 소스·검사명·전후 SHA를 보존했다.
- 공유 코퍼스13개 수집 경로에서 `tests/fixtures/transferred_exam_regressions.txt`의 정확한52개 시험지 입력을 제외했다. `tests/common/corpus_scope.rs`는 그 목록만 적용한다. 이에 연결된 baseline221행만 제거했으며 남은 행의 값·허용치는 원래 값 그대로다. `exam-corpus-removal-applied.json`에 수집 경로와6개 baseline의 제거 행 수를 기록했다. module path를 사용하는14개 integration source의 suite-policy 예외를 등록했으며, 파생 suite/manifest는 제출 대상이 아니다.
- production AST 감사에서8개 혼합 Rust source의 실행 구현은 기존7d8f766455c8913a084c103dd7ace0cf8b314f6b와 같았다(`exam-removal-production-ast-audit.json`). 빈 시험지 builder cfg(test) module도 제거했다. 소스 HWP/HWPX/HML와 독립 한컴 PDF는 그대로 보존했다.
- 보조 helper 정리 때 implicit format capture `{BASE_SAMPLE}`를 사용하지 않는 상수로 잘못 판정한 컴파일 오류4개가 발생했다. 원래 상수를 복원하고 같은 검증을 다시 실행했다. 최종 `cargo check --locked -p rhwp --tests --target-dir target/pr-review` exit0/35.98초, `cargo fmt --all`, `git diff --check`, manifest/base 비교 및 unit tiers/base 비교 PASS. manifest는1451sources/6244staticattrs/48integrationtargets, unit tiers는4113tests/297modules다. 정책 base는60efc4b7b380b2a183c20c3ee0ab24a60bc2d832다. 최종 로그는 `logs/check-exam-removal-final.log`에 있다. 전체 nextest 및 push 직전 lint 묶음은 아직 남아 있으며 이 검증을 전체 통과로 보고하지 않는다.
- 사용자가 요청한 수식 포함 파일 목록은 실제 파싱된 Equation control을 본문·중첩 표·도형 글상자·각주/미주·머리말/꼬리말·바탕쪽에서 재귀 순회해 확인했다. 제외52개 중 HWP/HWPX36개, 별도 HML fixture1개에 수식이 있다. `removed-exam-files-containing-equations.tsv`에는 정확한 파일 경로와 수식 개수를, `.json`에는 입력 SHA-256을 기록했다. 파싱 실패0건이며 HWPX14개는 XML 수식 개수와도 모두 일치했다(`removed-exam-hwpx-equation-xml-crosscheck.json`). `removed-equation-regression-source-mapping.tsv`에는 해당 입력과 연결된 삭제 검사174개/소스42개의 대응을 기록했다. 목록은 회귀 검사의 이관 범위를 뜻하며 수식 출력 보정 완료의 근거가 아니다.

### 제거 후 일반 문서 재검증과 셀 선두 캐시 보정 (v37/v38)

- 52cc8ee94의 일반 문서 text-overlap/off-canvas 전체16partition씩 및5593 기존 검사를 실행했다. 시험지 제외로 크기 기반 partition 배정이 바뀌므로 이전 실패 partition만 선택하지 않았다. exit100, build8분33초,33검사/28PASS/5FAIL/203.696초(`logs/nextest-general-remaining-v37.log`). 남은 실패는5593, text-overlap partition1/7/12, off-canvas partition12다. 신규 겹침 입력은 basic/issue1994_behindtext_table_20200830.hwp(1), issue2217/20200830.hwp(1), task1716/table_scattered_header_rowbreak.hwpx(2), table_scattered_header_rowbreak.hwp(2), 페이지 밖은 basic/shortcut.hwp(6)다. 일반 문서 기대값·허용치는 유지했다.
- 원격 refresh에서 #7615 head가 ed9c2847523c18f658344ec89cb7ed1a426807ae →4a6c27c5ffa54e4988a7e0e7956115d933725aca로 바뀐 것을 확인했다. 새 head에는 후속 #7662의377bcb71e7404a741105a28303a3fae8b671fc9d 및 그 선행 검사·갭 예약 제거가 포함된다(`pr7615-refresh-v37.json`). 사용자 지시에 따라 초기18개 검증은 처음 고정한7615 SHA를 유지하고 이 추가 변경은 #7661–#7663 후속 검토와 연결한다. 최신7615 head 전체를 이미 검증했다고 보고하거나 조기 종료하지 않는다.
- 5593 원본 SHA3973715acdb0…를 Hancom2020 PrintMethod0으로 출력했다. 독립 PDF SHA e79fe1a94d53a2843e717cf134e0fce2367b29c92bce3a73528cecb8a69a1af3 /1쪽. 선언6400HU 셀은 Print에서 약85.2px 높이를 유지하고 개체는 가운데에 있다. v35 Native는 셀92.2px/개체y100.1px로 불필요하게 성장하고 뒤 문단도7px 내려간다. Native 보조점수94.15%인데 직접 review PNG에서 차이가 보여 완료로 판정하지 않았다(`native-5593-before-v37/cell-front-object/review/review_001.png`).
- 동일 원본을 한컴에서 재저장한 정상 HWP 대조군 SHA3335ccea0197135d4de35b50d895a8e01af0a31724c7a40af9deaf4b717f5ba2를 확인했다. 선언 셀/개체 높이6400/5830HU는 그대로이며 셀 첫 저장 줄 vpos800→0, 부모 표 줄높이3560→6966HU로 정정됐다. 한컴이 만든 실제 대조군과 수동 진단 입력을 구분한다. 원본은 수정하지 않았다(`hancom-save-5593-v37.json`, `hancom-normalized/5593-hancom-export.hwpx`).
- 정렬 속성만 Top/Bottom으로 바꾼 수동 진단 입력의 독립 Print에서도 선언 셀 높이가 유지됐다. 개체 top은 Top74.796875pt/Center76.238281pt/Bottom77.675781pt다(`5593-independent-alignments-v37.json`). 선두 저장 vpos만0으로 바꾼 별도 진단은 Native 셀85.3px, 개체Top100.1/Center102.0/Bottom104.0px를 보였지만 이를 원본 수정 완료로 바꾸어 보고하지 않는다(`5593-zero-initial-line-probe-v37.json`).
- v38은 저장 조판 정보 수용을 완화하지 않는다. 이미 물리적으로 거부한 부모 TAC 행의 셀에서, line grid/HWP3/흐름 개체가 없는 첫 문단의 저장 시작 띠가 문단 위 간격+저장 줄 내부 leading보다 큰 경우 캐시를 거부하고 공용 cell reflow로 새 LineSeg와 후속 사다리를 만든다. foreground/background 개체는 줄 흐름을 전진시키지 않는다. 실제 저장 위 간격·내부 leading·grid·flow 개체가 있는 경계는 보존한다. HU 정수 비교로 동등한 간격의 소수 오차를 피한다. `tac_cache_repair::repair_cell_width_caches → reflow_line_segs/recalculate_cell_paragraph_vpos → reserve_content_heights → parent reflow`의 생산 결과를 측정과 배치가 소비한다. source에 시험지 ID·절대 화면 좌표·특정 높이 예외는 넣지 않았다. compile check exit0/1분43초이며 기존 검사와 Native/fresh WASM 시각 확인은 이어서 진행 중이다. 새 정식 회귀·fixture·baseline 변경은 없다.
- v38 기존5593/5598 검사3개는2PASS/1FAIL(exit100, build8분27초,0.084초)이며5593은 여전히 같은 y100.1px로 실패했다(`logs/nextest-initial-cell-band-v38.log`). 실제 호출을 추가 추적해 `reflow_line_segs_impl`이 원래 첫 LineSeg.vertical_pos를 새 ParagraphBox frame에 보존하고, `recalculate_cell_paragraph_vpos → apply_cell_vpos_ladder`도 그 원점을 재사용하는 것을 확인했다. v39는 거부된 셀 사다리를 먼저 clear한 뒤 canonical reflow를 호출한다. 이는 최종 좌표를 clamp하거나 임의로 vpos를 덮는 보정이 아니다. 원본 캐시의 판정·공용 재구성·측정/배치 소비를 잇는다. 같은3검사 재실행 중이며 이전 실패를 통과로 보고하지 않는다.
- shortcut 원시 진단의6건은 모델 공백 바탕쪽이 아니라 실제 본문 메뉴 제목 런이다. `파일/편집/보기/입력/모양`은 bbox1812.36px/extra_char_spacing892.17px, `창기타`는1359.27px/439.08px다(`shortcut-run-details-v37.log`). 해당 문단의 저장 첫 줄은 제목, 다음 줄은 별도 TAC 표를 소유한다(ColumnDef 앞 슬롯8, 제목 뒤 Table 슬롯 시작10 또는11). 단순히 bbox만 줄이거나 진단 대상에서 빼지 않는다. 원본 Justify의 제목 줄 종료/표 줄 참여 계약을 독립 PDF와 대조하며 보정할 예정이다. 기존 pr_7382_review.md 보정293–297의 동일 입력 전체7쪽 최저96.12686% 결과와 현재 후보를 구분한다. 새 후보의 해당 검증은 아직 없다.
- v39 기존5593 및5598 양쪽 내어쓰기 대조는3/3 PASS(exit0,0.088초, `logs/nextest-initial-cell-band-v39.log`). 5593의 기대값·입력은 그대로 유지했으며 이전v37/v38 실패와 연결된다. 현재 코드에서 거부 캐시의 원점을 실제로 제거한 뒤 공용 재조판 사다리가 높이와 배치를 함께 공급한다. Native/fresh WASM 출력 검증은 다음 게이트로 남아 있다.
- shortcut의6개 parent 제목은 font color16777215(흰색)이고, 실제 회색 헤더 표의 셀에는 별도 같은 제목이 있다. 색만으로 런을 제외하지 않는다. 배분 간격은 글자 다음 시작 위치를 전진시키며 backend의 `TextStyle::glyph_fit_advance`는 양수 extra_char_spacing을 glyph 폭에서 제외한다. 진단 `glyph_band_horizontal`은 공백이 없는 런에 전체 advance bbox를 그대로 써 마지막 글자 뒤 배분 간격도 잉크로 센다. 동일 replay/ink 계약의 누락인지 독립 PNG와 나머지 겹침 입력에서 확인 중이다. 앞의 제목 종료 추정만으로 정렬을 변경하지 않았다.

### 진행 중 보정 24: 실제 글꼴 공급과 진단 글리프 끝 소비

- 코드 head41b9be7f2b014f73ddeb4cb9668524d06b5db94d의 Native binary SHA d127aee7500a8edca9b9eeb8da1bd510dd01a85a052b55184fd32b867b355ebd, fresh WASM SHA cb4fb73566972d7d16d740eaa7f025a6c42554fae7b8d4087e332914b192d5e2를 고정했다. root pkg/Studio public의 JS/WASM byte 일치와 공개 API6/6 PASS다. 5593 Center/Top/Bottom의 Native/fresh WASM full review는 각각99.90031/99.90143/99.90031%, 모두 passed/본문 끝 차이0이다. 같은 배율로 셀 외곽·개체·뒤 문단을 직접 확인했고 정상 셀 높이가 유지된다. 원본/기대값 변경은 없다.
- shortcut은 양재난초체M 바탕쪽 쪽번호를 SVG에 내장하지 않아 Sans 숫자가 출력됐다. 실제 YNCH05.TTF(family Yj NANCHO Medium/양재난초체M, SHA d398aac5b56ab85b49343a3815ddad857e8b4dee01bc4f6c07eabe518faf2be9)의 동일 byte 사본을 ignored verified-font-supply/양재난초체M.ttf로 공급했다. 41b Native/fresh WASM 전체7쪽 TSV가 동일하고 최저96.14775%(p4), 누락0/90미만0이다. Native p4/p7 full review gate passed이며 한컴Print의 쪽번호·표 외곽·제목·본문을 직접 대조했다. 이전 누락 글꼴 캡처는 실패 기록으로 보존한다. 저작권 글꼴 파일은 커밋하지 않는다.
- scattered HWP의 Haansoft Batang/HWPX의 한컴바탕은 실제 HBATANG.TTF로 동일 face임을 fc-scan으로 확인했다. 이전 캡처의 해당 embedded font set은 비어 있었다. HBATANG.TTF 및 HMKMM.TTF 원 파일의 byte 동일 사본과 SHA를 verified-font-supply/verified-fonts.json에 고정하고 전52쪽씩 다시 측정 중이다. 루트 글꼴 디렉터리만 지정하면 filename lookup이 하위 symlink 디렉터리를 검색하지 않는다는 실제 소비 경로를 확인했다. 낮은 이전 점수를 배치 결함이나 불가피한 글꼴 예외로 승격하지 않는다.
- 진단 보정의 독립 근거: scattered p18의 하배/수관 두 런은 glyph 원점185.7467/211.6267, 글자폭13.3333px이며 이웃 셀 시작232.5067px 안쪽에서 끝난다. logical bbox는51.76px로 마지막 배분 간격12.5467px를 포함한다(scattered-ink-details-v40.log 및 실제 SVG). shortcut의 6개 parent 메뉴 제목도 배분 advance가 마지막 글자를 늘리지 않으며 실제 SVG 원점/크기에서 페이지 안쪽이다. 색·문서명으로 제외하거나 원 레이아웃을 줄이지 않는다.
- glyph_advance::tracked_glyph_end는 backend의 split_into_clusters → replay_positions_for → glyph_fit_positions → TextStyle::glyph_fit_advance를 동일하게 소비해 양수 배분의 glyph 끝을 구한다. 음수 자간의 advance보다 넓은 glyph 여유는 보존하며, finite/폭 계약이 없으면 종전 상자를 유지한다. glyph_band_horizontal이 공백 trimming 결과와 이 끝을 소비하고 flow_extent_bbox의 줄 합집합 및 text-overlap이 같은 결과를 사용한다. 원본 renderer/model·bbox·baseline·허용치·기대값은 바꾸지 않는다. 기존 계약 검사 및 실패 corpus 재검증을 이어서 수행한다. 새 정식 회귀·fixture는 아직 추가하지 않았다.
- Native scattered p18의 같은96DPI 영역을 확대해 glyph가 이웃 셀을 침범하지 않음을 확인했다(scattered-supplied-p18-ink-detail-v40.png). 동시에 독립Print와 약 한 행의 시작 위치 차이를 직접 확인했다. 실제 글꼴 공급을 전체 조판 통과로 보고하지 않으며 전쪽 점수와 이어받기 경계를 확인한 뒤 별도 보정한다.
- 차트는 기존 bounded metadata 초안으로 원 Contents type1/축0..2000/4division/series3/title 및 cached legend text를 다시 회수했다(column-metadata-v40.txt). 설치본 legend-item-layout의50.0 및0.25 상수를 실제 ELF load segment에서 읽어 chart-layout-constants-v40.json에 보존했다. 이 값만으로 문서별 좌표를 고정하거나 TWIP 단위를 확정하지 않았으며 제품 parser/renderer는 아직 바꾸지 않았다.
- 실제 글꼴 공급 뒤 Native scattered HWP의 전체52쪽 TSV는 최저41.05675%(p51), 90%미만41쪽/누락0이다. 원 face 내장 성공을 배치 개선으로 보고하지 않는다. 이전41.03131%와 구분해 native-general-supplied-fonts-v40/scattered-hwp/silhouette.tsv에 보존했다. 현재 입력 두 개를 source SHA로 고정한 새 한컴 Print 작업으로도 대응 기준 PDF를 재확인한다.
- 진단 보정 v40 기존 검사43/43 PASS(exit0, build9분42초, 실행214.216초/slow2): glyph-band 계약10개, text-overlap16partition, off-canvas16partition,5593대조1개. v37의 일반 실패5개가 기존 입력·기대값을 유지한 채 모두 복구됐으며 새 정식 회귀·baseline·래칫 변경은 없다. 원본6개 off-canvas 및 일반 겹침6건이 같은 glyph-band 소비로 해소됐다(logs/nextest-glyph-advance-v40.log). 이 결과는43개 선택 범위이며 전체 nextest/최종 lint/시각 보류 해소는 아직 남아 있다.
- 최초 Native Clippy v40은 앞선 메인터너 수식 코드 hancom_glyphs.rs의 needless_character_iteration/redundant_guards 두 건으로 exit101이었다. ASCII scalar 전수 검사를 동치인 str::is_ascii로, Roman/Italic guard를 동치 enum pattern으로 바꿨다. glyph/advance/shear/범위·기대값은 변경하지 않으며 같은 lint를 다시 실행한다. 이미 통과한 검사에 이 실패를 숨기지 않는다.
- 두 원본의 새 독립 Print가 완료됐다: HWP SHA65bbf7f22c8a…/Hancom2024 13.0.0.3901/PrintMethod0/PDF SHA d28a3c73b6b411e0b797b529894f0bc1d9f73fcb88112ab954cf2d2ef56bd6a7; HWPX SHAa36d3effddd7…/Hancom2020 11.0.0.9136/PDF SHA3e02139ddf2e9b58aa8d7e43c4c29e38735acfb8280685e0d916e28a1e7d0601. pdf/semanticist21-20261008/ 및 hancom-jobs/에 source hash/job/실제 출력 계약을 보존했다. HWP 새52쪽 Print와 기존41b Native PNG를 진단 png-pair로 비교해도 최저41.05675%이고 이전Print 대비 쪽별 점수 차이는 최대0.06639%p다. 새 head 재렌더링 증거로 보고하지 않는다. 실제 행 배치·분할 차이는 남아 있으며 reference 버전 표기만으로 면제하지 않는다.
- Native Clippy 경고 보정 뒤 동일 명령 cargo clippy --locked --target-dir target/pr-review -- -D warnings가 exit0/2분29초로 통과했다(logs/clippy-native-glyph-advance-v40-retry.log). WASM 전용 Clippy/workspace build/all-targets Clippy/정책 base 비교와 최종 full nextest는 아직 완료되지 않았다.

### 진행 중 보정 25: 전체 target lint 및 일반 표 경계 추적

- 2c483ce7b0ec535729257cf026475207ab320be6에서 WASM 전용 Clippy exit0/187.394초, workspace build exit0/264.042초다. all-targets Clippy는 시험지 검사 제거 뒤 남은 배열 원소가 하나인 기존 두 loop를 검출해 exit101이었다. issue_3893_bindata_sequence_ref 및 split_caret_axis의 동일 입력/검사 본문을 일반 block+let으로 옮긴다. 검사·입력·기대값·허용치를 바꾸거나 warning을 면제하지 않는다. 같은 all-targets lint와 base 정책 검사를 재실행한다(lint-glyph-advance-v40.json 및 로그).
- 글꼴 공급 후 Native 전수는 scattered HWP52쪽 최저41.05675%, HWPX52쪽 최저42.17084%, note-overlay4쪽 최저87.45003%, behind-variant4쪽 최저87.45049%로 실제 시각 보류가 남는다. source41b의 Native/fresh WASM scattered HWP52쪽 TSV는 동일하다. 두 종료/진행 상태는 각각 별도 로그로 확인하며 코퍼스 검사43PASS를 시각 통과로 대체하지 않는다.
- 일반 표 경계 추적은 DocumentCore 공개 입력 파싱/원본 셀·LineSeg 조회 → compose_section/HeightMeasurer::measure_section → block::prepare cut_row_heights/whole_row_fit_heights → continuation::fragment::budget/scan → row::required_height/whole_row_height → table_partial 실제 배치로 연결했다. scattered p18에서 Native 시작64행/Print 시작63행(용출성)이며, 앞 p16의 표18 첫 제목 원점도 Native564.1px/Print586.69px로 먼저 어긋난다. table15의 이어받기와 선행 공간을 함께 조사한다.
- RHWP_TABLE_DRIFT 진단에서 표6/15/18/62의 cut_row_h와 실제 MeasuredTable 행 벡터는 동일(diff0)이다. 따라서 단순히 더 큰 측정 벡터로 바꿔 공간을 늘리는 가정을 채택하지 않는다. 원본 line/선언·SourcePrint 문단 소속·내용 컷·물리 공간을 대조한다(scattered-source-cells-v41.tsv, scattered-measured-rows-v41.tsv, scattered-cut-measure-drift-v41.log, scattered-pages-v41.json). 새 조판 source/정식 fixture 변경은 아직 없다.
- 최초 loop 정리는 같은 입력을 쓰는 앞 함수에 적용되어 뒤 pics_survive_hwpx_roundtrip의 단일 원소 loop가 남았다. 재실행으로 검출해 뒤 함수도 같은 block+let으로 고쳤다. 공유 corpus_scope.rs 외부 mod가 같은 파생 suite에 두 번 로딩되는 duplicate_mod도 검출했다. 14개 원본의 private inline module에 동일 helper를 include!하고 불필요해진 path/root_mod 정책 override14개를 제거했다. helper 함수 본문/시험지52목록/입력·기대값은 동일하며 warning 면제는 추가하지 않았다. prepare:1451sources/6244staticattrs/48targets, all-targets Clippy retry exit0/12.89초, manifest/base 및 unit-tier/base PASS다(corpus-helper-inline-loading-v41.json 및 로그).

### 진행 중 보정 26: 저장 첫 조각의 완전 내용 컷과 물리 빈 띠

- 독립 한컴Print p1 SVG의 실제 괘선을 추출했다. 같은 row0..21의 Native/Print 외곽 차이는 약0.7..1.5px지만 마지막22행 바닥은 Native1067.2px/Print1081.536px로14.336px 차이가 난다. Native 제목 글줄도 마지막 셀 가운데 정렬에서8.6px 위다(scattered-independent-row-boundaries-page1-v41.tsv). 실물 위치를 formal golden/픽셀 assertion으로 고정하지 않는다.
- budget의 원본 첫 frame 조회는 이미Some(829.8667px)로 수용됐지만 scanner가 보호 블록 앞에서23행을 완전 소비한 결과는816.1px이다. 최근접 source_end24는 원자 블록과 실제 선택 컷의 소유를 대신할 수 없다. source_end를 억지로 맞춰 다음 행을 앞당기지 않는다. 저장 Common.height가 본문 안에 들어가며, 선택한 온전한 행의 예약 합과 실제 소비가 일치할 때에만 그 frame의 나머지 물리 띠를 현재 마지막 행에 보존한다.
- 생산/소비 연결: 기존 prepare_table_fragment_budget의 원본 frame 인증 → scan_table_fragment의 완전 컷/누적 consumed+end_row_height_override → emit_table_fragment의 partial_height/PageItem → table_partial의 실제 마지막 행 상자. 첫 조각/원본 저장/비편집/무컷/미정정 높이에서만 적용하며, 내용 단위·행 소유·이월 cursor를 바꾸지 않는다. 편집·재조판·부분 cut·각주 예산 초과·온전한 행합과 소비 불일치는 종전 경로를 보존한다. 새 정식 검사/fixture 추가는 전쪽 Native/fresh WASM90% 및 직접 판독 뒤까지 보류한다.

- v41 Native build exit0/4분17초, parent f1f8f9be6b40b8d552cc11d881f862819b10e5eb+patch SHA16a40984b1b6b72a06aed4be1332d453e6af42142ae4d83408281044c03c6164, binary SHA6b559f7cc90220505aece4c0bf9a37fb735a681bccc09fff4c48f876412ba597다. 실제 첫 조각은0..23행/829.8667px로 예약·그리기가 같은 물리 끝을 소비하며52쪽 전체 항목의 행/내용 컷 소유는 이전 후보와 동일하다. Native p1/p2/p3 직접 review: p1 약99.07%, p2 약97.10%, p3 56.53165%로 gate exit1/보류다. p1 마지막 행 외곽·셀 가운데 글줄은 개선됐지만 p3 뒤 표 원점·행 높이 차이는 남았다(native-stored-frame-tail-review-v41/ 및 scattered-pages-stored-frame-tail-v41.json). 새 fixture/기대값은 추가하지 않았다.
- 최신 upstream/devel f0e7228f6dd2ea1437724e53ad640c40c56d204b(#7619/#7667/#7630)의 저장 셀/재조판/float host 보정을 초기18개 후보에 반영한다. merge-tree 사전 비교의 충돌은 stored_reset.rs 한 곳이며, 초기 후보의 consume_saved_empty_run_reset 조건과 upstream의 stored_page_boundary_invalidated_by_reflow 조건을 동시에 보존한다. 이전60ef 기반 lint/43검사 결과를 새 통합 head의 완료 증거로 재사용하지 않으며 새 base 비교/영향 검사와 실제 출력 검증을 재실행한다.
- 최신 devel 충돌 해결: stored_reset.rs의 trigger에 consume_saved_empty_run_reset 및 stored_page_boundary_invalidated_by_reflow 두 독립 guard를 함께 둔다. 앞 조건은 이미 소비한 empty-prefix reset의 중복 전환을 막고, 뒤 조건은 재조판 직후 저장 빈 줄에서 옛 자동 쪽 경계를 복원하는 것을 막는다. 명시적 쪽/단 경계와 새로운 저장 본문·개체 owner를 각각 보존한다. merge 후 기존 관련 검사와 upstream 신규 경계 대조를 함께 실행한다.

### 최신 devel 통합 후보 검증과 사용자 지적 영역

- source ff7d647db94afd37b764905ed668239d3c0ef5bc/base f0e7228f6dd2ea1437724e53ad640c40c56d204b: Native build exit0/254.752초, binary SHA85b83eeebebd851b70e72f69b2a2215a535b80b4bd79f888b6e20e9e0979a4f9다. manifest/base 및 unit-tiers/base PASS다. 기존 선택 검사61/61 PASS(exit0, 전체588.446초/실행7.954초): upstream7330/7548/7618/7621의14검사, 기존7379의14검사,2097/6639/1133/TAC/glyph-band/5593대조33검사다. 이름별 실제 실행수는 nextest-upstream-integration-v42-counts.json에 기록했다. 필터에 포함한5598 이름은 실제 실행0건으로 그 검증을 주장하지 않는다. 새 전체 nextest/최종 lint/freshWASM은 아직 남아 있다.
- 같은 ff7d Native의 upstream 공개9입력/11쪽 전수 TSV는 최저92.52230%(cell-overlay), 나머지100% 또는 body-single-symbol99.94321%이며 누락0/90미만0이다. 대표 cell-fragments와 body-single-symbol full review/overlay를 직접 확인했다(각각100%/99.94321%, native-upstream-boundary-controls-v42/ 및 native-upstream-boundary-review-v42/). 셀 그림 뒤 제목, 단독 기호와 별표 세 글줄, 빈 문단/후속 본문 소속이 유지된다. 이 결과를 일반 scattered52쪽의 미통과 해소로 보고하지 않는다.
- 사용자 첨부 review_003.png(2026-10-08)는 v41 p3=56.53165%로 명시적 미통과다. 첫쪽 개선만으로 승인하거나 통합하지 않는다. 독립 Print p3 첫 인장시험(61행)을 현재 Native는 p2에 놓고 p3을편평시험62행부터 시작한다. 원본61행에 cell-protect/keep-with-next/widow/keep-lines/page-break 플래그는 모두 없다(scattered-row-boundary-flags-v42.log).
- 실제 원본/Print의72행: native44.2133px, Print36.7604px. 69..72의 짧은4행 병합 선언9448HU와 선행89.3067px가 요구하는 마지막 높이는36.6667px다. 현재 더 큰38..72의35행 선언61900HU 잔여가 마지막 행을44.2133px로 키운다. 이전 조각의 물리 빈 띠와 종료 병합 요구의 소유를 대조한다(scattered-span-physical-reservation-evidence-v42.json). 진단 좌표를 정식 expected/golden으로 고정하지 않았다.
- 진단 반례: 원본 HWPX61행의 저장 말미 spacing600→0, 및 같은 두 셀의 실제 문단 lineSpacing160→100+기존LineSeg 제거를 각각 독립 Print했다. 두 경우 모두인장시험은 여전히 p3에 있으므로 말미 줄간격만으로 잘못된 p2 수용을 설명하는 가정을 채택하지 않는다. 원본과 수정 provenance/job/Print SHA는 scattered-boundary-probe-inputs-v42.json 및 hancom-jobs/에 보존했다. 수동 진단 입력을 새 정상 fixture/원본 대체로 쓰지 않는다.

### 진행 중 보정 27: 반복 제목행이 여는 표 조각의 바깥 프레임

- 독립 반례 Print(2020 11.0.0.9136, method0/one-up): 같은 원본 HWPX 첫 표의 바깥 위/아래 여백141→0HU에서 인장시험61행이 p3→p2로 이동한다. 위/아래 각각141→216HU에서는 p3에 남는다. 셀 내용/행·열 그리드/선언 높이는 동일하다(scattered-outer-margin-probe-inputs-v43.json, scattered-outer-margin-print-observations-v43.json). Native의 p2는 before/bottom 모두0으로 첫 행 공간을 과대 평가했다.2×141HU=3.76px 예약 후 남은20.8267px는 원래 행20.88px보다 작다. 임의 상수/문서ID 없이 원본 margin을 소비한다.
- 생산/소비: 새 stored_header_frame 공통 술어(유효 저장/비편집/문단-단 기준 TopAndBottom RowBreak/실제 반복 제목행) → partial_rowbreak_fragment_spacing_px의 위/아래 예약 → fragment_placement.table_top 및 fragment_outer_bottom_overhead → layout_partial_table의 같은 반복 바깥 프레임. 기존 별도 위여백 경로는 새 프레임이 소유할 때 중복 가산하지 않는다. 원본 줄 segmentWidth가 양수라는 이유로 제목행의 새 물리 프레임을 버리지 않는다. TAC/합성·편집/비반복/다른 앵커·wrap 경로는 비적용이다. 영향을 받는 일반 대조와 Native 출력부터 확인하고 freshWASM/최종회귀를 이어간다. 아직 개선 완료나90% 통과로 판정하지 않는다.
- 앞 probe의 빈 띠/줄간격/작은 rowspan 수정은61행 소속을 바꾸지 않았다. 이것을 새 guard 근거로 재인용하지 않는다. 반복 바깥여백과 종료 큰 병합의 물리 예약은 별도 검증 경로로 남긴다.

- v43 Native build exit0/252.406초, binary SHA8265d44ff6456e2a93cb71b59c7bcb70b4d88c700336b9b263df6056b5db6dd7. 기존2097/6639/7379/1133/TAC 검사36/36 PASS(exit0,7.517초)다. Native p3 직접 review는56.53165→70.45078%로 여전히 gate exit1/보류다(native-stored-header-frame-review-v43/). 인장시험61행의 p3 소속은 복구됐지만, 앞 조각의 빈 물리 밴드가 병합 선언 잔여에서 빠지지 않아72행 높이/후속 표 원점이 아직 다르다. p1..3 결과를52쪽 전체나 freshWASM 검증으로 대신하지 않는다.

### 진행 중 보정 28: 병합 선언 잔여와 수용된 물리 행 상자

- 독립 Print p2 마지막60행은41.5521px로 원래20.88px보다20.6721px 크고, p3의72행은36.7604px다. 반복 제목행/바깥 여백을 예약한 Native p2의 남은20.8267px는 내용 단위를 수용하기 부족하지만 앞 조각을 닫는 물리 공간이다. 이 공간을 앞 행에 수용하면35행 병합38..72의 선언825.3333px를 끝행에서 다시 채우지 않아야 한다. 짧은69..72 병합은 여전히36.6667px를 요구한다. 독립 좌표는 진단 증거이며 기대값으로 고정하지 않는다.
- 구현 계획: 반복 제목행의 저장 프레임에서 실제 수용한 완전 행 상자를 continuation cursor에 기록 → 다음 조각의 종료 병합 선언에서 이전 물리 상자를 차감 → 하나의 행 높이 계획을 scanner/emit/PageItem/paint에 전달한다. 부분 내용 컷·재조판·편집에는 기존 경로를 유지한다. 제목행 반복 높이는 병합 내용의 소비로 세지 않는다. 각주 재스캔 뒤에만 빈 공간을 확정하고, 다음 행이 이미 들어간 예산/미완료 내용/비병합 경계에는 빈 공간을 추가하지 않는다.

- v44 parent1169ca645d125e356b62ec0d6cc660d2b3cd6049, Native build exit0/252.303초, binary SHA37431af25d96eb2f1818853f574301054afa4648f93e96d309251b153bd8722b. 변경 source와 새 physical_rows 모듈은 native-build-physical-row-plan-v44.json/source-code-v44에 각각 SHA로 고정했다. p1/p2/p3 Native full review는99.72513/99.49893/99.05228%로 수치 gate passed/exit0다. 세 review PNG를 직접 확인했고 p3 확대 판독도 별도로 실행했다.52쪽 전체/freshWASM 통과는 아직 주장하지 않는다.
- 단위 컷은 바꾸지 않고 받아들인 완전 행의 물리 예약을 기록한다. scan/refit → 공통 row plan을 적용한 MeasuredTable/whole-row fit/cut → emit/PageItem.fragment_row_heights → 일반 layout 및 cursor-rect probe의 같은 행 높이로 연결한다. 기존 PageItem 직접 생성과 remap은 None/clone으로 옛 경로를 유지하며 검사 기대값·입력은 바꾸지 않았다. 기존 #6976 쪽 하단 상자 접기와 100HU footer band는 예약·흐름과 별도 paint 소비여서 원래 경로를 유지한다. 표 외곽과 마지막 셀 상자 좌표를 같은 숫자로 보고하지 않는다.
- 기존 검사46/46 PASS(exit0,7.666초,1090skipped):2097/6639/7379/1133/TAC 및 PageItem 직접 생성이 있는6935/7032 대조다. fmt check, unit-tiers/base(f0e7228f6dd2ea1437724e53ad640c40c56d204b) PASS. manifest 최초 실행은 source 변경 뒤 파생 harness drift를 검출했다. --prepare로 새 source fingerprint를 재생성한 뒤 같은 base check가PASS(1455source/6258attrs/48targets)다. 파생 파일은 제출하지 않는다. 새 정식 렌더링 회귀·fixture·golden은 추가하지 않았다.
- 독립 여백 대조 p2/p3: margin0는99.45708/80.62738%로 gate exit1, margin+75는99.44336/95.95205%다. 두 변형 모두61행의 앞/뒤쪽 소속은 독립Print와 같다. margin0 p3는 앞 표가 거의 겹치지만 후속 제목/표의 원점이 다르다. 수동 변경한 source LineSeg와 재인쇄의 관계를 대조할 범위이며 숫자90 미만을 면제하지 않는다(physical-row-plan-validation-v44.json).
- 수치 통과 후 직접 검출한 미충족: 원래 Print p3 비고는 “바다모래인”만 있고 “경우”는 p4 첫 행에 이어진다. Native는 두 줄을 p3에 넣었다(p3-last-note-rhwp_png/pdf_png-v44.png, pdftotext p3/p4). 실제 원본 HWP pi9,row9,col8의 같은 문단 두 저장 줄은 text_start0/6, vpos0/0, LH1000/LS600/tag393216이며 dirty/keep/page-break가 없다(scattered-last-note-source-v44.log). 전체 행 fast path와 이 내용 컷의 소유를 계속 보정한다. 점수99.05%로 이 차이를 면제하거나 통합 승인하지 않는다.

### 진행 중 보정 29: 저장 첫 행 프레임의 문단 내부 원점0 컷

- 실제 HWP pi9의9행 col8은 같은 문단의0/6 text_start가 각각 vpos0/LH1000/LS600/tag393216이다. 이는 수동 메타데이터가 아니며 원본 HWP와 대응 HWPX에 같다. 원본 raw 행0..8 합18246HU < 첫 common frame21399HU <9행 끝21412HU다. 독립2024Print는 두 줄을 p3/p4에 분리한다. 단순0→0을 전체 행으로 흡수하는 가정에 대한 실제 반례다.
- 생산/소비 계획: 같은 원본 문단의 순서가 있는 양수 줄 전진과 선언 첫 행 프레임 경계를 함께 인증 → SourceFrameQuery의 전체 행 fast path 제외 → CellUnit의 strict source-frame cut → 기존 saved_multirow_opening_frame의 선언 상자/내용 컷 분리 → continuation/paint의 같은 cut. 같은 두0 줄만 가진 합성/편집/재조판/첫 frame을 닫지 않는 행은 이 새 원본 경계의 근거로 사용하지 않는다. 뒤 병합 라벨의 보존은 기존 opening-frame 수용성 검사가 그대로 확인한다.

- 최신 GitHub 재조회(v45): 비 draft24/draft6, 초기18개 이후7661/7662/7663/7670/7671/7673은 별도 검토 대기다. 사용자 선택에 따라 초기18개 통합 전에는 추가 체리픽하지 않는다. #7615 최신head4a6c27c5ffa54e4988a7e0e7956115d933725aca는 deferred7662 의존 갱신이 포함된 별도 delta이며 이번 원 적용headed9c2847523c18f658344ec89cb7ed1a426807ae와 구분한다(github-open-pr-snapshot-v45.json). 최신 upstream/devel은 f0e7228f6dd2ea1437724e53ad640c40c56d204b로 바뀌지 않았고 HEAD 기준 behind0이다.

- v45 Native build exit0/254.386초, binary SHA085389c3e70b9f81eb379bafe5becd2b9b11b8ed385f99f7c9dbf30de14d2b61. 원본 HWP/HWPX Print 모두 p3“바다모래인”/p4“경우”임을 다시 확인했다. 실제 새 내용 컷은 p3 pi9 rows0..10/endCut[1,1,2,1], p4 row9/startCut[1,1,2,1]이고52쪽을 유지한다. p3 Native는99.69210/99.68701%이며 두 줄의 페이지 소속도 직접 일치한다. p4는69.39683/69.34749%로 여전히 gate exit1이다. 관련 기존 검사 실제36/36 PASS(exit0,7.809초); 로그 재대조: #7032의7건은 실제 포함됐고 #6935/2097/1133은 실행0건이다. 해당 미실행 범위의 통과를 주장하지 않는다(nextest-zero-origin-text-frame-v45-counts.json).
- p4의 실행으로 확인한 원인은 first_fragment_blank_band의 base_remaining 생산값이다. 원시 행3166HU−첫 물리 프레임3153HU=13HU(0.1733px)만 continuation.start_row_height_override로 넘겼다. scanner의 carried-row 조기 경로는 이 값을 그대로 예약하고, paint는 이 높이를 그대로 적용한 뒤 남은“경우”의17.0933px 글줄 상자만 확장해 row10과 겹쳤다. 단일 행 source cut의 다음 프레임은 물리 최소 잔여와 남은 내용+패딩의 큰 값을 생산하여 같은 cursor/scanner/PageItem/paint로 전달한다. 블록 컷은 multi-row accounting을 유지한다. 기존 기대값 변경은 없다.

- 사용자 지시(2026-10-08): 현재 보정을 checkpoint commit한 뒤 최신 upstream/devel로 rebase하고 계속한다. v45 수치/내용 컷 증거와 아직 빌드하지 않은 이어받기 내용 높이 보정을 구분해 보존한다. source tree 동일성과 저장 reset의 두 독립 guard를 rebase 후 확인하며, 새 producer의 Native/freshWASM 출력은 재검증한다.

- rebase 완료: 사용자 요청 전 checkpoint `cc8c1c4ec3d10ec947f66fb427c4764a0da055a0` → 최신 base `f0e7228f6dd2ea1437724e53ad640c40c56d204b` 위 새 head `ab46e1c7431d1010b0b183d7d43b40453a087aaa`. `git diff refs/codex/semanticist21-pre-rebase-20261008-v46 HEAD`는 빈 결과로 tracked tree 전체가 동일하다. 저장 reset의 두 guard를 모두 보존하고 생성 JS를 특정 stash에서 복원했다. v46 Native build를 새 head에서 시작했으며 이전 v45 결과를 새 내용 높이 생산값의 검증으로 재사용하지 않는다(rebase-checkpoint-v46.json).

- v46 rebase head `ab46e1c7431d1010b0b183d7d43b40453a087aaa` Native build exit0/253.506초, binary SHA `64dc4b71890ea628f0d488b3afebb06f6220ff67ed65b9e30f4237278beaaa59`다. 원본 HWP p3/p4는99.69210/99.15457%, HWPX는99.68701/99.13639%로 두 수치 gate exit0다(native-continuation-content-height-review-v46/). 실제 p4 row9 높이는0.1733→20.88px이며 row10은119.2px에서 시작해 “경우”와 겹치지 않는다. 이전 p4의 추가 응결54행도 사라지고 독립 Print와 같이53행 비표면적에서 끝난다. 좌표는 진단값이며 assertion으로 고정하지 않았다.
- v46 직접 판독의 별도 미충족: p4 마지막53행의 병합 셀 col0 “플라이애시” 및 col7 “·제조회사별”은 독립 Print에 있지만 Native에서 비어 있다. 두 셀은 실제 원본의5행 병합/선언8985HU이며 다음 저장 줄이 원점0에서 재개한다. 첫 보이는 줄은 들어가지만 `rowbreak_straddle_cut_units → cell_units_fitting_height`의 코드와 원본 메트릭을 대조하면 완전 유닛 높이에 말미 spacing600/552HU가 포함돼 첫 조각의0유닛 수용으로 이어지는 것으로 추정된다. 이 분기의 실행 trace와 재개 컷 보존을 후속 검증한다. 이 경계의 source와 독립 p4/p5 텍스트 증거를 scattered-row53-source-owners-v46.json/scattered-print-p4-p5-label-v46.txt에 보존했다. 정식 검사·기대값 변경 및 완료/승인은 계속 보류한다. fresh WASM/전체52쪽은 새 head로 이어서 검증한다.

- v46 기존 관련 회귀46/46 PASS(exit0,전체306.122초/실행7.776초,1470skipped). 실제 실행 모듈별 수량은 nextest-continuation-content-height-v46-counts.json이며6935/7032 및2097/1133도 포함한다. fresh WASM root wrapper 빌드와 원본 HWP/HWPX 전체52쪽 Native TSV를 시작했다. 최종 전체 회귀/lint 및 다른 미통과 문서의 범위는 남아 있다.

### 진행 중 보정 30: 병합 셀의 저장 첫 글줄과 다음 프레임 컷

- 독립 원본 HWP/HWPX Print p4/p5는53행의 “플라이애시”/“·제조회사별” 한 줄씩을 p4에, 남은 “시멘트 (KS L 5211)”/빈도 문단들을 p5에 둔다. Native v46 p4 trace에서는 두 셀의 문단 방출이 없고 p5는 su0으로 전체 문단을 다시 시작한다(row53-owner-trace-v46.log/row53-next-owner-trace-v46.log). 첫 프레임의 가시 내용과 뒤 간격이 다른 소유라는 실제 반례다.
- 적용 경로 추적: 실제 첫3개의 원본 저장 줄 0→0→LH+LS를 가진 동일 시작·끝의 병합 셀들이 첫 frame 경계를 공유 → native_first_line_restart_cut_trailing_trim의 독립 source spacing → cell_units_fitting_height의 같은 prefix end/su 선택 → rowbreak_straddle_cut_units 및 straddle_continuation_demand의 같은 소유 컷 → PageItem의 행 경계와 table_partial의 가시 줄. 단일 셀의 불완전 단서·다른 rowspan 경계·편집/합성/재조판은 인증하지 않는다. 다음 조각에서도 앞 physical 높이에 같은 fitting 함수를 적용해 eu==su를 보존한다.
- 구현 방향: 병합 셀의 shared restart는 같은 행에 우연히 존재하는1행 셀 전체에 강제하지 않고 같은 시작·끝을 가진 원본 병합 peer에서 증명한다. 말미 간격은 인증된 source cut의 첫 유닛 fit에만 제외하고, 유닛의 일반 높이를 다시 쓰거나 paint에서 내용을 복제하지 않는다. 기존 양수 되감김·TAC·단일행 viewport 및 다른 원점0 정상 대조를 다시 실행한다. source/WASM 빌드 종료 후 수정한다. 정식 검사/fixture 추가는 관련 전체 Native/freshWASM90%와 확대 판독 뒤까지 보류한다.

- v46 fresh WASM wrapper exit0/597.310초, WASM SHA `066c60e35fa592344ab5b8d0b864398b4c75dda205a46ea8533081ec420dd7f5`, JS SHA `7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a`로 root pkg/public이 동일하다. --no-opt의 Native 진단 빌드이며 optimized 배포 artifact와 구분한다. HWP p3/p4 fresh WASM은99.69210/99.15457%로 Native와 같고 p4 review 직접 판독에서도53행 두 셀의 내용 이동이 남는다. HWPX/전체52쪽은 실행 중이다.

- v47 원본 프로필 재조회: HWPX는 hwp5_origin이 아닌 genuine hwpx_stored_layout이며 실제 첫3개 LineSeg가 HWP와 같다(scattered-spanning-first-frame-hwpx-source-v47.log). 이 저장 병합 프레임에도 같은 source 계약을 적용하고 기존 단일행 HWPX 경로는 별도 계약을 유지한다. parent `32ce04e790cdf9b3b59a3464662f9d61cb9767ef`+source SHA `e40026715fdfb8ba80ddba7ebfc3264fac62e6695f6a1e187f202d28b0eb06da`, Native build exit0/281.461초, binary SHA `87daa4a670a4187a8739f0c70413b35992bb6a06da0a6c850cc873af6e079b87`다. 실행 trace p4의 두 셀은 cut(0,1)로 첫 줄을, p5는 cut(1,MAX)로 남은 줄을 방출한다. 다음 조각은 앞쪽의 첫 줄을 재방출하지 않는다. 같은 source의 재측정/내용 높이는 기존 shared trim을 소비하며 행 경계는 유지한다. 비교 이미지와 기존 회귀는 진행 중이며 완료/승인으로 판정하지 않는다.

- v47 Native p3/p4/p5는 HWP99.69210/99.72851/96.44533%다. p4/p5 직접 review에서 첫 표제의 소속·후속 줄 보존은 개선됐다. 다만 p5의 잘린 셀을 Top으로 강제하는 기존 effective_align 분기로 남은 “시멘트 (KS L 5211)”와 빈도 문단들이 위로 붙는 미충족을 확인했다. 관련 기존 회귀 50/50 PASS(exit0,전체344.583초); #6761 원점 재시작의 실제/부정 대조도 포함한다(nextest-shared-span-restart-v47-counts.json). fmt 및 manifest/base(f0e7228...) PASS, 파생 harness는 미제출이다. 이 checkpoint의 수치 통과를 정렬 차이의 통과나 승인으로 바꾸지 않는다.
- v46 Native HWP 전체52쪽 TSV가 완료됐으며 최저45.96876%/90미만29쪽/누락0이다(native-continuation-content-height-full-v46-hwp-summary.json). HWPX 전체 비교는 계속 실행 중이다. v46 freshWASM HWPX p3/p4는99.68701/99.13639%로 Native와 같지만 동일 표제 소속 문제를 가진다. 이 역사 후보를 v47/후속 source의 전체52쪽 검증으로 재사용하지 않는다.
- 다음 보정: authenticated shared restart의 cut(1,끝)과 완료된 물리 rowspan이 남은 프레임의 정렬 owner다. straddle cut을 원본 padding 직후 한번 산출해 composition window/line ranges/effective_align이 같은 결과를 소비하게 한다. 열린 꼬리 프레임·중첩 표·미인증 재시작은 원래 경로를 유지한다. 원본 모델의 Center에 따라 현재 프레임 안에서 남은 내용만 정렬하며 독립 Print p5의 위/아래 간격과 대조한다. 새 절대 px 기대값은 만들지 않는다.

- v47 대응 HWPX p3/p4/p5도99.68701/99.67973/96.61612%로 수치 gate는 exit0다. HWP p4는 새 표제/빈도 첫 줄을 포함해99.72851%이며 p5의 잔여 내용은 보존되지만 앞에 기록한 정렬 차이는 면제하지 않는다.
- v48 후보: rowbreak_straddle_cut_units를 composition 선택 전에 한 번 산출하고 같은 Option cut을 가시 줄 범위에 재사용한다. shared source 첫줄 restart에서 start1/모든 잔여 unit/완료된 물리 rowspan을 가진 마지막 프레임만 원래 valign을 소유한다. source 인증은 dirty·cell-format-vpos·controls가 없는 원본 병합 peer에서 확인한다. 새 native_first_line_restart_cut_trailing_trim의 반환값은 scanner/잔여 높이 측정과 같은 source spacing이며 별도 px 보정이 아니다. full cargo fmt/check와 후속 변경 파일 rustfmt/check는 exit0다. parent94cfcc9e8의 Native 새 빌드를 시작했다.

- v48 Native build exit0/275.837초, binary SHA `c6f2e8227af6bede6a7fa404a34a581c791a93e635887aa28d90cfbd5ef95c3a`다. source2개는 native-build-shared-span-alignment-v48.json/source-code-v48에 SHA로 고정했다. HWP p3/p4/p5는99.69210/99.72851/99.07224%다. p5 trace에서53행 표제/빈도의 원점은 각각131.8/110.5px로 원래 모델 Center의 남은 프레임 정렬을 소비한다. 아직 freshWASM/전쪽/최종 검증 통과를 주장하지 않는다.
- v46 원본 두 파일 전수52쪽 TSV 완료(exit1): HWP 최저45.96876%/29쪽 미통과, HWPX 최저46.40781%/29쪽 미통과, 둘 다 누락0이다(native-continuation-content-height-full-v46-summary.json). 이 기록은 이전 source ab46e1c74의 진단 범위이며 현재 후보의 gate 통과로 대체하지 않는다.
- 후속 직접 검출: p5 하단96행/9행 병합의 “콘크리트 혼입용 방수재”는 독립 Print에3줄이 있지만 Native는2줄만 놓고 “방수재”를 p6에 넘긴다. 실제 저장 줄은0/1600/3200/4800HU로 원점 재시작이 없는 연속 텍스트다. `cell_units_fitting_height`가 마지막 보이는 줄의 spacing600HU까지 요구해 선택한 물리 행밴드에서3번째 줄을 배제한다. Print p6는 “(KS F 4926)”만 남은 병합 프레임에 가운데 정렬하지만 Native는 두 줄을 위에 붙인다(scattered-print-p5-p6-bbox-v48.html/scattered-page6-owner-v48/). 단순 첫줄 재시작만 정렬 owner라고 가정한 범위도 이 실제 반례로 확대 검토한다. 일반 저장 plaintext rowspan의 가시 prefix 끝과 source advance를 구분하고 마지막 물리 프레임의 실제 잔여 내용/원래 valign을 공통 cut으로 소비하게 하는 후속 보정이 필요하다. 새 기대값/절대 px assertion으로 차이를 숨기지 않는다.

- v48 HWPX p3/p4/p5는99.68701/99.67973/99.02628%다. HWP p5의 direct review에서53행 잔여 표제/빈도는 독립 Print와 같은 가운데 정렬로 개선됐지만96행 “방수재” 및 빈도 후속 줄의 불필요한 이월 차이는 남는다. 관련 기존50/50 PASS(exit0,전체315.112초/실행8.028초,1677skipped), manifest/base PASS다. source SHA와 Native 비교는 위에 고정했고 freshWASM/최종 전체 검증은 미완료다. 사용자“메인터너 보정으로 해결” 지시에 따라 같은 scope의 출력 보정을 계속한다.

### 진행 중 보정 31: 일반 저장 병합 셀의 가시 줄 끝과 다음 줄 전진

- 원본96행의 연속0/1600/3200/4800HU는 마지막 보이는 줄의 glyph/LH와 그 뒤 source spacing600HU를 구분한다. Print p5는 “방수재”까지, p6는 “(KS F 4926)” 한 줄만 마지막 물리 프레임에 가운데 정렬한다. 초기 재시작 인증만으로 마지막 저장 프레임의 정렬을 정하는 가정을 제거한다.
- 생산/소비: 새 stored_span_content의 비편집·비재조판·비TAC RowBreak/원본 plaintext rowspan 인증 → cell_units_fitting_height의 prefix 끝에서 source 줄뒤 간격만 제외/전체 전진량은 유지 → 이전 eu와 다음 su에 같은 함수 사용 → straddle_continuation_demand의 잔여 내용 높이 및 table_partial의 한번 계산한 cut → 끝난 물리 rowspan/완전 잔여 창의 원래 valign과 실제 paint. 마지막 whole-cell unit은 이미 LS를 제외하므로 다시 차감하지 않는다. controls·synthetic LS·dirty·음수 spacing·없는 원본 줄은 비적용이다.
- 작은 원점 재시작에 맞추는 추가 수치·문서ID·픽셀 clamp는 없다. 새 source 모듈은55줄 수준이고 신규 formal 회귀/fixture/golden은 추가하지 않는다. 원본 HWP/HWPX p5/p6와 앞53행 경계의 무회귀, 기존50검사 및 이후 Native/freshWASM 전수·확대 판독으로 계속 검증한다.

- v49 Native 최초 컴파일은 새 child module의 인증 메서드 접근 범위 오류(E0624)로68.419초/exit101이었다. 실제 table_partial 소비 지점이 볼 수 있는 layout 내부 범위로 수정하고 재빌드했다. retry exit0/251.204초, binary SHA `0aa3d8af2d3d6ccb2c1d2576d62d24eae17c8b9f19c1fd9adf0d2540670fe18a`다. 실패 metadata/log 및 수정 후 source SHA를 분리 보존했다(native-build-stored-span-content-v49-failed-visibility.json/native-build-stored-span-content-v49.json/source-code-v49-retry/).
- v49 실제 원본 HWP/HWPX의 p5는96행 “콘크리트 혼입용 방수재”3줄과 빈도3줄을 보존한다. p6는“(KS F 4926)”/“때마다”만 남겨 같은 마지막 물리 셀 프레임의 Center를 적용한다(span-page-owners-v49.json). 독립 Print의 두 p6 단어 yMin115.654320pt와 Native 같은 영역의 줄 원점154.3px를 직접 대조했다. source53행 p4→p5의 컷과 Center도 유지한다. 진단 좌표를 새로운 기대값으로 고정하지 않는다. 3..6쪽 PNG/관련 기존 회귀/그 뒤 freshWASM 전수 검증은 진행 중이다.

- v49 원본 HWP p3/p4/p5/p6 Native는99.69210/99.72851/99.71529/99.81823%, HWPX는99.68701/99.67973/99.61458/99.72035%다. 두 파일 모두4쪽 선택 범위의 수치 gate exit0이며 관련 최저99.61458%다. HWP p5 하단과 p6 첫 병합 셀을 같은2배 크기 Native/Print crop으로 직접 확인했고 “방수재”/후속KS와 빈도 마지막 줄의 소속·센터·뒤 셀 포함이 일치한다(row96-p5-native-print-zoom-v49.png/row96-p6-native-print-zoom-v49.png). HWPX p6 full review도 직접 확인했다. 해당 plain 행 프레임의 컷을 확인했으며 p6 화학식 첨자와 μm의 직접 판독은 아래 추가 기록을 따른다. 알려진 줄간격 차이를 수치 통과로 면제하지 않았다.
- v49 기존 관련50/50 PASS(exit0,전체319.996초/실행7.862초,1677skipped); 원점 재시작 실제/부정 대조와 빈 문단/원자 창/이어받기/원래 외곽 대조를 같은 후보에서 실행했다. 실제 모듈 수량은 nextest-stored-span-content-v49-counts.json이다. source fmt/manifest/base PASS. 아직 Native/freshWASM 전체52쪽 및 다른 일반 문서의 미충족, 최종 lint/전체 회귀가 남아 있어 checkpoint로만 기록한다.

- v49 p6 화학식 MgO/SO3/SiO2의 아래첨자·정체, 45μm의 기준선·앞뒤 글자 간격을 Native HWPX/독립 Print의 같은3배 crop에서 직접 대조했다(chemical-subscripts-p6-hwpx-zoom-v49.png). 내용/첨자/간격의 가시 결함은 확인되지 않았으며 AA와 괘선 농도 차이는 남는다. HWP p3 최신 review도 직접 확인해 사용자 지적의 마지막 표/뒤 제목·비고 컷이 유지됨을 확인했다. source checkpoint77953f3c8에서 freshWASM wrapper 빌드를 시작했다.

- v49 fresh WASM wrapper exit0/591.983초, code head `77953f3c8a64b295f82c171a8dc4e26dde614a24`와 WASM SHA `76035a19d7ffe1626a41501c1be3c993ceca831838311b46767669895b7fc75a`를 고정했다. root pkg/public JS 및 WASM 해시가 각각 같으며 --no-opt의 Native 진단 산출물이다. 같은 pinned package/binary의 p3..6 full review와 HWP/HWPX 전수52쪽 fresh WASM TSV를 시작했다. Native 전수52쪽도 별도 pinned binary에서 실행 중이다. 새 formal 회귀/기대값 및 통합 판정은 전쪽 최저값과 직접 판독이 완료될 때까지 보류한다.

- v49 freshWASM p3..6 full review가 두 원본 모두 exit0으로 완료됐다. HWP99.69210/99.72851/99.71529/99.81823%, HWPX99.68701/99.67973/99.61458/99.72035%로 Native와 같다. HWP p3 및 HWPX p6 review를 직접 확인했다. 사용자 지적 p3의 큰 표 외곽·행/쪽 소속과 뒤 제목은 복구됐으며 범위 최저99.61458%다. Native HWP52쪽 TSV는 최저45.96876%/28쪽 미통과/누락0으로 완료됐고 HWPX 및 freshWASM 전수는 진행 중이다(native-stored-span-content-full-v49-hwp-summary.json). 선택 범위 통과로 전수·통합을 승인하지 않는다.

### 진행 중 보정 32: 앞 조각의 물리 행 상자로 병합 컷 이어받기

- 독립 Print p6에는138행 빈도 셀의 첫3줄이 있고 p7은 “: 1일 타설량마다”부터 시작한다. Native v49는 p6에서 이미 방출한 “세제곱미터 미만인 경우”를 p7에서 다시 방출한다. 실제 원본138행/span3의 저장 줄은0/1300/2600/3900HU이며 다음 문단에서1300HU로 되감긴다(scattered-row138-source-hwp-v50.log). source/출력 분류로 면제하지 않는다.
- 실제 호출 경로: record_stored_fragment_rows가 p6 마지막 물리 행 밴드를 원장에 기록 → stored_fragment_row_plan이 다음 MeasuredTable/PageItem로 전달 → rowbreak_straddle_cut_units는 이를 받은 뒤에도 단일행 셀이 있는 이전 행을 row_cut_content_height로 재측정해 더 작은 내용 높이로 덮어씀 → p6 eu3/p7 su2의 중복과 종료 셀 요구 증가 → 뒤 행/쪽 끝 내용의 이동이다. 저장 plaintext span의 이전 완전 행은 실제 원장/공통 resolved 행 상자를 소비하고, 합성·편집·중첩/controls 등 비인증 입력은 종전 내용 높이 경로를 유지한다. 부분 행 컷은 같은 source cut을 계속 사용한다.
- 보정은 별도 prior-row 높이 helper를 추가해 기본 straddle 및 시작 컷 걸침의 paint 구제에서 같은 결과를 소비하도록 한다. 새 formal 회귀/fixture/기대값 추가는 관련 전체 Native/freshWASM90% 및 직접 확대 판독 뒤까지 보류한다.

- v49 freshWASM p6의 SO3/SiO2 첨자·정체·글자 간격을 독립 Print의 같은3배 crop으로 직접 확인했다(chemical-subscripts-p6-hwpx-wasm-zoom-v49.png). Native와 freshWASM의 원본 두 파일 p3..6 RGB raster 비교는8쪽 모두 차이 bbox=None이다. 직접 확대 판독한 Native p5/p6의 소속·Center·MgO/45μm 영역도 같은 실제 raster임을 별도로 확인했다. p7 이전 source trace는138행 빈도 셀의 su2를 명시한다(scattered-p7-trace-v49.log). 후속 source v50은 별도 빌드 중이며 v49 결과를 v50의 검증으로 재사용하지 않는다.

- v50 Native build exit0/293.195초, parent77953f3c8+3개 source SHA 및 binary SHA `13c7898912aa13d841f55033aec9ca74a487ceab4d8fc0a52255c714d9a70b7b`를 native-build-stored-span-prior-height-v50.json/source-code-v50에 고정했다. 원본 p7 실행 trace에서138행 빈도 su3가 적용되어 중복 문장이 없어지고 조각139..170으로 쪽 끝169행을 보존한다. 다음 p8은170..212, p9는212..244이며 전체52쪽을 유지한다(scattered-pages-v50.json/scattered-p7-trace-v50.log).
- v50 HWP p3..8 Native는99.69210/99.72851/99.71529/99.81823/99.52407/99.62649%다. p7은67.27483→99.52407%, p8은61.24208→99.62649%로 개선됐다. 최신 p7/p8 review PNG를 직접 확인해 중복 제거·마지막 PC강연선 행·다음쪽 첫 인장하중 및 p8말미 외관검사 소속·내용 보존·표 외곽을 확인했다. p7 비고의 kg/m³ 위첨자와 ± 양쪽/글줄 간격을 같은3배 Native/Print crop에서 직접 대조했다(p7-spacing-superscript-native-print-zoom-v50.png). HWPX 선택 범위와 전체52쪽은 계속 검증 중이다.
- v50 기존50/50 PASS(exit0,실행8.768초/1677skipped); 이름별 실제 수량은 nextest-stored-span-prior-height-v50-counts.json, 전체 소요는 같은 exit JSON이다. source 추가로 파생 suite fingerprint를 prepare했으며 신규 formal 테스트/fixture/기대값은 추가하지 않았다. Native 전수52쪽은 새 pinned binary에서 시작했고 아직 통합/완료 gate는 보류다.
- v49 Native HWP/HWPX 전수52쪽은 exit1으로 종료됐다. 각각 최저45.96876/46.40781%와28쪽 미통과/누락0이다(native-stored-span-content-full-v49-summary.json). 같은 v49 freshWASM HWP 전수도 Native와 동일한52개 페이지별 수치다. HWPX freshWASM 전수는 진행 중이며 이 역사 후보의 결과를 후속 v50 head의 증거로 대체하지 않는다.

- v50 HWPX p3..8 Native도99.68701/99.67973/99.61458/99.72035/99.51656/99.59727%로 full review exit0이다. 두 원본 선택12쪽 최저99.51656%이며 HWPX p7/p8 review도 직접 판독했다. 중복 빈도/PC강연선 및 다음쪽 외관검사의 소속, 뒤 표 괘선/가시 줄은 독립 Print와 일치한다. source fmt 전체 check, manifest/base 및 unit-tiers/base(f0e7228...)는 PASS다. existing50 검증 총354.259초/실행8.768초이며 새 전체 Rust lint/전수nextest는 아직 남아 있다. source를 checkpoint 커밋하고 그 정확한 head에서 freshWASM을 다시 빌드한다.

- v50 전수 출력의 최신 raster를 --png-pair로 재사용해 조기 진단했다(재출력은 같은 pinned Native binary의 진행 중 전수 작업, 입력 PNG 해시는 각각 silhouette_manifest.json에 기록). p9=99.32384%, p10..15=91.20541/97.77033/99.67256/98.81025/99.09792/99.27556%, p16..19=99.64007/99.56841/99.10979/97.37686%다. p9 직접 대조에서212행 시작과244행 끝이 독립 Print와 같고 p10/p11 내용 소속도 크게 개선됐다. 단, p10 첫 partial 블록 조각의 하단 프레임은 Native 약497.1px/원본 Common.height507.28px로 짧고 독립 Print에서도 하단 괘선/셀 가운데 차이가 보인다. 91.20541% 수치로 면제하지 않고 후속 프레임 보정 대상으로 남긴다.
- source p10 trace는 table12 첫 source_frame=Some(507.28,988.16), source_end7, available508.6, 실제 cut row6/12개 슬롯 및 consumed489.3px다. paint는 마지막 셀 padding을 넣어497.1px로 늘리지만 저장 프레임의 나머지 물리 공간은 사라진다(scattered-p10-drift-v50.log/scattered-source-frame-v50.log). source frame의 선택 컷과 실제 물리 요구를 scanner→emit→PageItem→paint에서 같이 보존하는 후속 조사이며 아직 source 추가 변경은 없다.
- v50 조기 Native p20..24는59.15986/64.64593/48.03726/48.53776/58.86768%, p28/p29는55.49027/64.76842%로 미통과다. p25..27은99.84791/99.65724/96.08807%다. 전수 전체/직접 판독/freshWASM 결과는 계속 실행 중이며 일부 높은 수치로 완료·승인을 판정하지 않는다.

- 사용자 “review.png 로 보여줘야 내가 같이 보지” 지시를 반영했다. 이후 수정/보류 영역을 현재 head의 Native/Print/overlay 세 패널 review PNG로 대화에 함께 표시한다. 최신 전수 raster에서 canonical visual_sweep.make_compares/make_overlay_compares/make_review_panels로 p10/p19/p20을 생성했다(native-stored-span-prior-height-selected-review-v50/review/ 및 provenance.json, create-selected-review-v50.py). p20=59.15986%의 실제 review_020.png를 공유했고 미통과를 유지한다.
- v50 freshWASM build는 source dc18b952716db98d7c9aa758c5eb8055645f86f0에서 exit0/620.321초, WASM SHA b695a553c1b9e129934cfa0763d6034ef1ff67bba4d36a5ffd13d02df691de6a다. root pkg/public JS·WASM 해시는 각각 동일하며 --no-opt 진단 산출물로 구분한다. 같은 pinned binary/package의 p3..8 및 새 실패 경계p19/p20 review를 시작했다. Native p20에서 이미 미충족이므로 freshWASM 전수52쪽을 추가로 시작하지 않고 선행 경계 보정과 Native 전수를 이어간다.

- v50 Native HWP52쪽 전수 TSV가 완료됐으며 최저48.03726%/미통과15쪽/누락0이다(native-stored-span-prior-height-full-v50-hwp-summary.json). 직전v49의28쪽 미통과에서13쪽이 수치 기준을 넘었으나 p10의 직접 판독 미충족과 다른15쪽이 남는다. HWPX 전수는 계속 실행 중이다. v49 freshWASM HWPX52쪽도 최저46.40781%/28쪽 미통과/누락0으로 완료됐으며 역사 source의 두 backend 전수 수치는 같다.

### 진행 중 보정 33: 같은 행의 서로 다른 저장 소유자가 입증하는 원점0 경계

- 독립 Print p19는146행의 “IDDV”/“KS M ISO 2507-1,” 첫 줄을 마지막 밴드에, p20은 “(ISO 3633)”/“KS M ISO 2507-2”부터 둔다. 원본146행 col7은 같은 문단의 두 실제 LS가0/0·LH1000·LS0이며 text_start는 전진한다. col1/span3은 IDDV의0과 다음 ISO 문단의0→1580(LH1000+LS580)이다. 일반 문단 로컬0만으로는 판정하지 않고 서로 다른 source 소유자의 같은 row에서의 restart/뒤 사다리를 함께 확인한다(scattered-source-frame-v50.tsv 및 최신 review_019/020.png).
- 실행 경로는 advance_row_cut의 [1,1]/13.3px 콘텐츠 컷 → row_cut_content_height의 실제 패딩 포함20.9px → row_step의25px 고아 기각 →146행 통째 이월 → 앞145행에 남은 물리 띠 기록 → 다음 source rowspan의 불필요한 전진이다(scattered-p19-trace-v50.log). 일반 완전 행의 공간 원장은 앞 수정대로 유지한다.
- 새 공유 경계 인증은 비편집/비재조판/비TAC TopAndBottom RowBreak의 원본 단일행 같은 문단0/0 및 같은 시작행의 실제 병합 peer의0/0→LH+LS를 함께 요구한다. target의 spacing0은 경계 존재와 공간 크기가 다른 상태이므로 허용하되, 증명 없는 첫 조각 경로는 기존 양수 spacing+Common.height의 물리 교차를 그대로 유지한다. 인증을 CellUnit의 frame-break/whole-row 제외/내용 컷 및 orphan keep가 같은 helper로 소비하게 한다. 모든0·합성·dirty·편집·개체·뒤 사다리가 없는 peer는 인정하지 않는다. 기존6761의 실제/부정 대조를 함께 검증하고 새 formal 테스트/fixture 추가는 전쪽 Native/freshWASM90%와 확대 판독까지 보류한다.

- v51 최초 Native build exit0/282.366초, binary SHA0d1b516d8140f04b2036a392a450579911854d918e1ae9a2afb3cb6c0bd31850다. 실행에서53쪽으로 늘어나 원본52쪽 계약을 위반했으며 실패 후보를 보존한다(native-build-shared-zero-origin-frame-v51.json/source-code-v51/scattered-pages-v51.json). p19에서는146행을 계속 이월하고 p20에 첫 줄만 있는 별도 조각이 생긴다.
- 추가 추적으로 초기 의심한 패딩 중복은 원인이 아님을 확인했다. content13.3/padding7.5, 컷[1,1]은 맞지만 stored_zero_origin_rewind_keep가 이미 끝난 col4의1줄에도 원점0 되감김을 요구하고, 다른 plain-reset gate는 표 전체의 모든 행에 single-row 선언이 있어야 하므로 원본의 병합 전용 행에서 차단된다(scattered-p19-drift-v51.log). 경계 인증과 공간 크기는 그대로 두고, 이미 완료된 소유자와 실제 이어질 source 소유자를 구분하는 공통 row-cut 판정을 추가한다.

- v50 Native 원본 두 파일 전수52쪽은 exit1으로 종료했다. HWP/HWPX 최저48.03726/48.41629% 및 각각15쪽 미통과/누락0이다(native-stored-span-prior-height-full-v50-summary.json). 같은 code head의 freshWASM p3..8/p19/p20 review도 종료(exit1)했다. 두 backend의 선택 범위 페이지별 수치는 각 원본마다 동일하며 HWPX p19/p20=97.48563/59.68074%다. p3..8의 실제 개선은 유지되지만 p19→20의 잘못된 소속은 면제하지 않는다.


### 한컴 기준 PDF 보존: untracked 자료 정리

- 사용자 지시 “현재 untract 된 pdf 는 뭐임? 필요없으면 제거, 필요하면 커밋”에 따라 `pdf/semanticist21-20261008/`의210개 PDF(10,134,567바이트)를 확인했다. 원본 기준, 편집·합성 대조 및 진단·반증 대조이며 기존 추적 PDF/이번 폴더 내부의 바이트 동일 중복은0이다. 현재 검토의 독립 출력 증거로 보존한다.
- `references.tsv`에 입력 경로·입력 SHA256, PDF SHA1/SHA256·크기·쪽수, Creator/Producer, engine·job ID·상태·print_method 및 로컬 작업 기록을 연결했다.210개 입력 해시는 현재 파일과 모두 일치하고210개 변환 상태는 succeeded다. 출력 계약은 작업의 status.result.pdf_print_method에서 읽었으며210개 모두 method0을 확인했다. 원본 문서의 info.printMethod와 변환 결과 계약을 구분한다. 변환 서비스 주소/비밀 환경 값은 포함하지 않는다. 상세 검사 원장은 ignored `output/pr-review/semanticist21-20261008/pdf-reference-audit-v51.json`에 보존한다.
- 이 커밋은 기준 PDF 보존이며 새 formal 회귀 fixture/golden 추가나 시각 gate 통과 판정이 아니다. 편집·합성 입력은 현재 ignored 증거 경로에 보존되어 있으며 최종 제출 증적의 입력 재현성 정리가 아직 남아 있다. 진행 중 v51 source는 별도로 검증한다.


- v51 retry Native는 binary SHA4225ced5c10f1a2c3293668767dac27f224885b2d22c551d0fee11201e99cde7/source-code-v51-retry로 고정했다. 원본52쪽을 유지하며 HWP p3/p4/p19/p20=99.69210/99.72851/99.35269/96.98379%다. 실제 review_019/020.png를 대화에 표시했다. IDDV/KS M ISO2507-1은 p19에, ISO3633/KS M ISO2507-2는 p20에 보존되지만 p20의 ISO 병합 셀은 원본 Center보다 위에 있다. 높은 점수로 완료 판정하지 않는다.
- 다음 원인 경로: advance_row_cut의 일반 행 컷은 row_span1 소유자만 기록 → start_cut_covers_cell은 같은 시작행의 rowspan도 소관으로 판정 → cell_cut_window가 unrelated 단일행 슬롯을 rowspan의 su로 읽음 → is_in_split_row 때문에 공통 straddle cut/완료 프레임 Center 경로가 차단된다. 일반 행 공간과 블록 공간을 명시적으로 전달하고, rowspan은 앞 조각이 실제 소비한 물리 밴드로 공통 유닛 창을 계산하게 한다. reservation의 straddle_continuation_demand와 paint의 rowbreak_straddle_cut_units/resumes_inside_own_start_row에 같은 시작 컷 공간을 전달한다. 블록 소유자의 컷 슬롯은 유지한다. 새로운 px assertion/fixture는 추가하지 않는다.

- v51 retry 기존 경계 회귀50/50 PASS(exit0, 전체327.230초/실행7.447초,1677skip)다. Native 선택8쪽은 두 원본 모두 exit0/312.569초이며 HWPX p3/p4/p19/p20=99.68701/99.67973/99.46133/97.15814%다. freshWASM/전수 및 가운데 정렬 차이는 후속 보정에서 다시 검증하며 완료를 선언하지 않는다.


### 진행 중 보정 34: 일반 행 컷과 병합 셀의 소유 공간

- start_cut_covers_cell/일반 end 판정과 cell_cut_window의 행 슬롯은 row_span1만 소유한다. 같은 행의 rowspan은 start_row_height_override와 명시적인 start_cut_is_block으로 앞 조각 소비 높이를 복원한다. 기존 빈 컷/병합 전용 행과 BlockCut 슬롯의 소유 계약은 유지한다.
- 전달 경로: FragmentInput.start.start_cut_is_block → BlockRowScanVars → RowScanQuery.required_height → straddle_continuation_demand → rowbreak_straddle_cut_units/resumes_inside_own_start_row다. paint도 PageItem.start_cut_is_block에서 동일 helper에 전달한다. 시작 RowCut/끝 BlockCut이 섞인 조각은 끝 블록 슬롯을 보존하고 시작의 물리 prefix만 공유 cut으로 소비한다. 한 쪽 공간이 다른 쪽 공간을 대신하지 않는다.
- 최종 저장 plaintext span은 공통 가시 창의 마지막 내용이 완료되었을 때 기존 Center/Bottom 정렬을 적용한다. 절대 위치 clamp·문서ID·기대값 완화는 추가하지 않았다. source snapshot/source SHA는 source-code-v52/native-build-row-cut-span-ownership-v52.json에 남기고 관련 기존67개 및 원본 HWP/HWPX의 p3/p4/p19/p20부터 검증한다. 같은 행 회귀 #7226, 이어받기 요구 #6981, 시작/끝 보존 #6803, #6024, #6756을 기존50개에 포함한다.

- v52 첫 Native build exit0/256.852초, binary SHAf89d269356484284e05be8a2097f04c5b5c1ba924c87e7de6124dee1cb4584d5다. 실제 p20 트리에서 IDDV가 다시 보인다(scattered-p20-trace-v52/render_tree_020.json). 이 후보는 실패이며 원인/증거를 보존한다. RowCut 슬롯 오소유를 제거한 뒤 start_row_height_override가 있는 분기의 resolved 높이−잔여 높이에서 자기 상단 패딩을 뺀 예산이 첫 줄 점유보다 작아 공통 su0으로 돌아갔다. 저장 선언이 첫 물리 프레임만 나타내는 경우 그 차이만으로 앞 컷의 소비 내용을 복원할 수 없다. 실제 앞 RowCut의 padding 포함 점유 높이를 소비의 하한으로 함께 사용하고, 더 큰 원장의 빈 물리 밴드는 보존하는 공통 prefix 보정이 필요하다. 회귀와 최초 sweep은 실패 후보의 증거로 구분해 완료까지 보존하며 formal 기대값은 바꾸지 않는다.

- v52 기존67개는 exit100/전체349.258초, 실행20.526초로62PASS/5FAIL/2127skip이다. #6024 원본의 전체 쪽수106vs105, #7226 교육과정의414vs413, #6981 같은 원본의 종료 뒤 노동인권 소속, #6803의 이어받기와 완전 보존2검사에서 실패했다. 기대값은 유지한다. 실패 원본의 Print/Visual Sweep을 추가하며, 기존 #6803 5쪽 PDF 대조는 pinned 실패 binary에서 시작했다. #6024·#7226/#6981·#6803의 원본3개는 버전별 MCP Print 계약을 새로 확인한다.

- 실패 원본 #6803의 Native v52 전쪽5페이지 Visual Sweep은 exit1이며99.26183/93.32244/73.17339/34.27747/33.25166%다. 최신 실패 binary와 추적 독립 PDF의 출력 비교이고 최저33.25166%로 원본은90% 선행 조건을 충족하지 않는다. 테스트/기대값을 변경하지 않으며 실제 출력부터 보정한다(native-failed-6803-original-review-v52/6803-original/review/). 새 MCP Print와 freshWASM의 대응도 남아 있다.

- v52 retry1 build exit0의 실제 트리도 IDDV 중복을 유지하므로 실패 후보(source-code-v52-retry)로 보존한다. 앞서 적은 물리 차이만의 가설은 충분하지 않았다. 같은 행의 일반 RowCut은 별도 start_row_height_override 없이도 이미 시작한 병합 셀의 물리/내용 prefix를 소유한다. 빈 컷/병합 전용 행만 height override를 요구하고, 일반 비어 있지 않은 RowCut은 자체로 이어받기를 입증하도록 조건을 교정한다.
- #6803의 앞 eu와 뒤 su도 서로 다른 높이 공간이었다. 앞 helper는 cell_h를 사용했지만 뒤는 이전 행 점유와 실제 RowCut prefix를 사용한다. 새 rowbreak_cut_prefix_height는 동일 원점의 실제 이전 행 원장+accepted RowCut 점유를 산출하고 일반 걸침 helper의 끝 컷과 #6803 블록 시작/일반 끝 경로가 함께 소비한다. 끝 블록 컷 슬롯은 별도로 유지한다. 원본에 따라 조건을 좁히거나 px 기대값을 바꾸지 않는다.

- 원인 분리를 위해 pinned v51(보정33)과 v52 retry1에서 실패 원본 쪽수를 다시 읽었다. #6024는 두 후보 모두106쪽, 교육과정은 두 후보 모두414쪽, #6803는5쪽이다(failed-original-page-count-comparison-v52.json). #6024/#7226의 쪽수 실패를 새 컷 공간 보정이 유입한 회귀라고 단정하지 않는다. 기존 검토 후보에 이미 남은 실패이며 원본 출력 보정이 필요하다. 교육과정의 독립415쪽 기준 PDF는 기존 pr_7544_review.md에 같은 원문 대응과 쪽수 불일치가 명시되어 있다. 새 MCP가 queue 대기 중이어도 기존의 미충족·미검증을 그대로 보존한다.

- v52 retry2 Native build exit0/249.830초, binary SHAf5b1ac023d02adaf5deb90e1d195758d182d9d9f2f66b37a3050fc34abc3a5ca다. 원본 p20 실행 트리에서 IDDV 중복이 제거되고 ISO 두 줄 y112.5/133.5가 독립 Print112.286/133.406과 대응한다. 새 review PNG와 기존67개를 다시 실행한다. #6803 공용 JSON 탐색은 row_span/model 소유 정보가 없어 다른 표의 같은 row/col을 섞으므로 정식 소유 검증으로 사용하지 않는다. 원본의 typed 회귀2개로 확인하며 잘못된 탐색 집계를 not_evaluated로 교정했다.

- v52 retry2 HWP p3/p4/p19/p20 review는99.69210/99.72851/99.35269/97.44457%다. 최신 p20 review_020.png를 대화에 표시해 IDDV 중복 제거와 ISO 두 줄의 세로 Center를 확인했다. 같은 첫줄 Native “(ISO ”는 줄끝 공백을 포함해 x188.2/폭34.3인 반면 독립 Print 잉크는 x191.36/폭27.835다. 가시 텍스트·공백의 정렬 폭과 줄말 공백 소유를 후속 확인한다. 하단 괘선의 작은 차이도 남아 있어 수치만으로 확대 판독을 통과하지 않는다.

- v52 retry2 선택 Native HWP/HWPX8쪽 review는 exit0/336.111초이며 HWPX p3/p4/p19/p20=99.68701/99.67973/99.46133/97.58977%다. 기존67개는 exit100/전체346.771초/실행20.623초,64PASS/3FAIL/2127skip이다. #6803 이어받기·완전 보존2검사는 수정 전 v52에서 FAIL/수정 후 retry2에서 PASS로 복구했고 원본/기대값은 그대로다. 남은 #6024 쪽수와 #7226 쪽수/#6981 종료 뒤 소속은 앞 후보에도 남은 원본 실패로 계속 보정한다. typed 검사의 복구를 #6803 전체 피델리티 승인으로 승격하지 않는다.
- 정확한 retry2 binary에서 원본 두 파일52쪽 Native 전수 TSV를 시작했다(native-row-cut-span-ownership-full-v52-retry2). 소스 형상/입력/기준/명령/선택 PNG는 source-code-v52-retry2 및 명령 JSON/검토 폴더로 고정했다. freshWASM, 최종 Rust lint/전체 nextest/Studio, 원본3개의90% 선행 검증과 나머지 최초18개 범위가 아직 남아 있으며 PR 생성·승인·merge는 보류한다. source 보정의 로컬 체크포인트만 커밋한다.


### 진행 중 보정 35: 줄바꿈 구분 공백과 가운데 정렬의 가시 폭

- 원본 p20의 ISO3633 두 저장 줄과 독립 Print의 대응에서 첫줄 말미 ASCII 공백은 다음 단어로의 줄바꿈 구분자이며 가시 중심에 포함되지 않는다. Native는 글리프 뒤 공백 폭6.4px의 절반만큼 첫줄을 왼쪽에 둔다. 소스의 center_excludes_trailing_ws가 is_last_line_of_para만 허용한다는 가정이 원인이다.
- 경로: estimate_composed_line_width에서 원본 줄의 전진 폭 생산 → effective_text_width/기존 trailing_space_width_after_last_inline_object가 각 run의 실제 스타일로 구분 공백 점유를 계산 → x_start의 Center 정렬 원점 → emit_line_runs의 실제 glyph 원점이다. 문단 마지막 여부 제한을 제거해 기존 순수 텍스트 Center의 가시 폭 계약을 모든 줄에 적용한다. 문서ID/절대 px 보정은 넣지 않으며 텍스트·char offsets·줄 전진과 소유는 그대로 보존한다. TAC 개체가 있는 줄, 밑줄 말미/공백만 있는 줄은 기존 가드·helper를 따른다. #7081/#6173/#6028의 기존 회귀와 원본 Print 대조를 연결한다. 새 formal 검사/fixture/기대값은 추가하지 않는다.
- 앞 source6c041a692의 #6803 재출력 전쪽5페이지는 exit1로99.26183/93.32244/73.88708/34.42714/33.25166%다. typed 내용 보존2검사의 복구와 실제 배치 미충족을 구분한다. 이 원본도 먼저90% 출력 보정 대상으로 남기며 테스트를 바꾸지 않는다.

- 실패 원본3개의 MCP Print가 완료됐다. 모두 engine2020/11.0.0.9136, pdf_print_method0/hancom2020_pdf_driver_one_up이며 입력/다운로드 해시를 검증했다. 위임전결규정103쪽(PDF90eda86b2b4dfeabc601cd9b583408c6528a9efabf4e626fd6f2e28e3bcb2b97), 교육과정415쪽(PDF82fe8eefecbf3ad5176549976a8cd486ac0bb9c113e7ac34510a69ee6ccbf17f), #6803 5쪽(PDFb6941c58dde2b2af5c6d5103f549af11efb4e6e7b451436245d22f97ee0078cf)이다. references.tsv에 같은210개 형식으로3개를 추가한다. 실패 회귀의 원본103/415쪽과 현재Native106/414쪽 및 기존 기대105/413쪽을 각각 구분한다. 기대값이 Print와 다르다는 이유만으로 수정하지 않으며 실제 전체 출력을90% 이상으로 먼저 보정한다.

- v53 Native build exit0/270.637초, binary SHAdcbc04665f608394f57c8b83fc6a412697508ec23e4ffb0b43f9cfcf6b6aa524다. ISO 첫줄 x188.2→191.2px/독립 Print191.36px로 가시 중심 차이를 제거하고 줄 전진·문자열은 보존했다. HWP p19/p20=99.99671/97.72314%, HWPX=99.92627/97.86418%의 새 review를 직접 읽었다(native-center-wrapped-space-review-v53). 아직 하단 괘선의 작은 차이는 남는다. 기존7081 Center/Right·6173 TAC·6028 밑줄4/4 PASS(exit0/296.718초,실행0.113초/631skip)이며 검사·입력·기대값을 바꾸지 않았다.
- 앞 source6c041a692 HWP52쪽 전수 TSV 최저48.03726%/미통과13쪽/누락0이다. 미통과는22/23/24/28/29/30/31/39/48/49/50/51/52이며 v50의15쪽에서20/21쪽이 개선됐다. HWPX 전수는 계속 진행 중이다. 현재v53 재출력 p22는47.99365%의 직접 미충족이며 원인 조사는 다음 source 단계다.


### 진행 중 보정 36: 첫 프레임 밖의 원본 두 문단 행 컷

- p21/p22 독립 Print는237행의 “압축강도”/KS M ISO844를 p21, 다음 문단“(경질형 에폭시수지)”를 p22에 둔다. 실제 두 문단 LS는 각각0·LH1000·첫LS300HU이고 padding과 두 줄 점유/간격 합이cellSz2866HU를 정확히 닫는다. 현재p21 실행은 content budget14.1px에 first unit17.3px(글줄13.3+다음줄간격4)을 강제 선택하지만 가시 cutoff/paint에서도 간격을 남겨20.9px짜리 유효 줄 조각을 배제한다(scattered-p21-cut-trace-v53.log).
- 기존 native_saved_two_line_row_frame은 동일 shape를 Common.height의 첫 프레임 교차에만 허용해 후속 프레임의 컷을 놓친다. 첫 프레임 위치 질의는 유지하고, 실제 RowCut의 source 두 줄/정확한cellSz 계약은 별도 공유 결과로 산출한다. 저장 표가 첫 프레임을 넘는 근거는 각 행의 명시적 single-row 최대 선언 높이 합이라는 하한이다. single-row 없는 병합 전용 행은0하한으로 포함하고 새로운 높이를 추정하지 않는다. 이 하한이Common.height를 넘으면 소유된 프레임은 전체 표가 아니다.
- 공통결과는 native_saved_reset_cut_trailing_trim → advance_row_cut 예산 수용·row_cut_content_height/물리패딩 → row_step source-boundary keep → partial paint의같은 가시 컷으로 소비한다. 문단별0만으로 판정하지 않고 실제 원본dirty/합성LS/control/정확한선언상자/스타일 계약을 확인한다. HWP/HWPX 원본 둘과 기존6761/7379 등 경계 검사를 실행하며 새 fixture/기대값은 추가하지 않는다.

- source6c041a692의Native HWPX52쪽도 종료(exit1)됐으며 최저48.41629%/미통과13쪽/누락0이다. source 이전 후보의 전수 HWP/HWPX 정리는 native-row-cut-span-ownership-full-v52-retry2-summary.json에 있다. 현재source 단계의 최종 증거로 이전 전수를 재사용하지 않는다.
- 두 문단 행의 간격 trim은 부분 컷의 다음 unit이 남아 있을 때만 적용한다. 완료된 whole-cell 마지막unit은 이미 LS를 제외하므로 두 번 빼지 않는다. 기존 첫 프레임 위치 질의는 hwp5/원본 행-prefix 교차 계약을 그대로 유지하고 후속 RowCut helper와 별도로 호출한다. source-code-v54의 새 helper는90줄이며 source3파일+SHA를 새 build에 고정한다.

- v54 첫 build exit0/260.076초이나 원본 HWP/HWPX dump-pages에서 p21 end237/endCut없음, p22 start237/startCut없음으로 보정 효과가 없었다. readonly 원본 진단은 row237의 원본 두 LS, padding283+283, spacing_before/after0과 1000+300+1000+566=2866HU의 정확한 선언 상자를 확인했다(plain-row-frame-source-v54.log). 원인은 컷 trim의 호출이 아직 첫 선언 프레임 전용 native_saved_two_line_row_frame에 남아 있던 연결 누락이다. retry1에서 후속 RowCut용 stored_two_line_row_cut_frame을 호출하도록 수정한다. 첫 프레임 예약의 별도 인증은 유지하며, 효과 없는 v54를 시각 통과로 보고하지 않는다.

- v54 retry1 Native build exit0/255.321초, binary SHA cadc1391f0cb2750de5a3a5f50bc4892d69e3f9c27585494f40edd32beb44ec5다. 두 원본은52쪽이며 p21 end238/endCut[1,1], p22 start237/startCut[1,1]로 독립 Print와 동일한 원본237행의 문단 소유를 복구했다. table pi18의 일반 셀 row237에서 p21 압축강도/KS M ISO844, p22 (경질형 에폭시수지)/빈 완료 method를 직접 확인했다(scattered-row237-source-owned-cells-v54-retry1.json, independent-print-row237-text-pages-v54-retry1.json). JSON으로 rowspan 소유를 판정한 증거가 아니다.
- Native 선택12쪽 review exit1/469.618초다. HWP p3/p4/p21/p22/p23/p24=99.87432/99.96125/100.00000/94.98743/50.15023/56.14305%, HWPX=99.86229/99.92357/99.95350/95.71094/49.94848/55.94398%다. p22/p23 review PNG를 직접 표시했으며 p22 상단 표·뒤 제목은 복구됐지만 하단 첫 조각/다음 쪽 내용은 미충족이다. 앞 후보p22 47.99365%와 구분한다. fresh WASM과 원본 전체90%는 아직 미검증이다.
- 기존 경계67검사는64PASS/3FAIL/2127skip(exit100/339.041초,실행19.006초)다. 정상64개는 유지되며 실패도 기존 #6024(106vs105), #7226(414vs413), #6981 노동인권 소속 세 개 그대로다. 해당 원본의 전체90% 출력 선행 보정이 끝나지 않았으므로 검사·기대값·baseline을 변경하지 않았다.

### 진행 중 보정 37: 첫 저장 프레임의 잉크 뒤 빈 밴드와 문단 내부 이어받기

- 같은 원본 table pi26의 첫 선언 프레임은22340HU이고 row0..3의 독립 single-row 상자 합은16064HU다. row4 frequency/remarks의 같은 문단 저장 줄은4200→0HU로 재개하며 첫 프레임 마지막 잉크와 padding 끝은21830HU다. 선언 프레임에는510HU의 잉크 없는 밴드가 남고, 다음 줄을 순차로 소비하면23230HU여서 첫 프레임을 넘는다(logs/scattered-table26-declared-frame-v54-retry1.log). Print p22는 마지막 작업시/불가능시 줄을 p23 첫 행에 둔다. 직전 Native는 전체 행 fast path가 저장 reset을 무시해 p22에 모두 소비했다.
- 기존 stored_row_reset_closes_declared_frame은 ink+padding이Common.height에 정확히 닿을 때만 true여서 원본의 양수 빈 밴드를 부정한다. first frame의 마지막 가시 줄·선언 끝·다음 줄의 요구 끝을 구분하고, 실제 같은 문단 양수→0 reset과 원본 사다리·declared row 범위로 빈 밴드를 인증한다. 단순 좌표 clamp/문서ID/허용치 증가는 넣지 않는다. 생산 술어는 cell_units의stored_frame_break_before와 whole-row 우회에 공유되고 실제 advance_row_cut → saved_multirow_opening_frame_height의Common.height 예약 → physical row carry → partial paint로 대조한다. 첫 프레임을 실제 예산에 수용하지 못하면 정상 이월 경로를 유지한다.

- v55 Native build exit0/254.272초, SHA c44287969fb9090b78e2c9845d315970cca7ff5ba10eed17922389dad90b3a2e다. 원본 두 개52쪽이고 p22 table26 endCut[1,1,4,4], p23 startRow4/startCut[1,1,4,4]로 작업시/불가능시 줄의 Print 대응 쪽 소유가 복구됐다. 22–24쪽 Native review exit1/236.057초: HWP95.45728/96.77602/58.89865%, HWPX96.23749/97.06903/57.77669%다. p23/p24 review를 직접 표시했다. p23 하단 row22 물리 밴드는 여전히 작고 p24의 여러 행이 아래로 밀려 남은 원인이다.
- v55 기존67검사는62PASS/5FAIL/2127skip(exit100/339.763초,실행19.249초)이다. 앞64PASS/3FAIL을 그대로 말한 중간 집계 오류를 즉시 정정했다. 새 FAIL은 #6981 giant_straddle_cell_that_must_be_cut_is_not_grown(#6803 원본의 A9~10 칸 밖 글줄)과 #6935 real_collision_document_preserves_text_across_fragments(4vs3쪽)이다. 같은 원본/기준 PDF로 전쪽 Native Visual Sweep을 시작했고, 실패 검사·기대값·baseline은 유지한다(native-new-failed-originals-review-v55).
- readonly 원본 판정 진단(first-frame-blank-band-candidates-v55.log)은 새 범위 조건이 main table26 row4 col4/5뿐 아니라 #6803 pi6 row8 col4와 #6935 pi0 row4 col1에도 발동함을 확인했다. 원본 LS와 선언 높이가 한 셀의 경계를 설명하는 것만으로 행 전체의 물리 컷을 확정한 가정이 부족했다. retry1은 source ordinary-row의 미완료 owner들이 같은 ink/padding 끝과 같은 문단 내부 원점0 재개를 공유하는 결과를 확인한다. 완료 owner는 해당 source frame에 들어가며, 걸친 셀은 ordinary RowCut의 peer로 승격하지 않는다. 단일 미완료 owner의 느슨한 Common.height 대조는 기존 capacity/BlockCut 경로에 남긴다. 문서ID·수치 예외가 아닌 RowCut 소유 결과의 인증 변경이고, 기존 정확한 first-frame closure 계약은 유지한다. source-code-v55-retry1 두 파일+SHA에서 재빌드/동일 경계 재검증한다. 아직 최종 통과가 아니다.

- v55 retry1 build exit0/255.309초, binary SHA fff08b7b8e5807cf907247bb6db1155147bf3faa85670a7e12c76226f2c58f78이다. 같은 원본의 shared unfinished owners가 없는 #6935는3쪽으로 복원됐고 #6803는5쪽이다. 원본 main두 파일은52쪽과 p22 endCut[1,1,4,4]/p23 startCut을 유지한다(pages-*-v55-retry1.json). 67개 기존 검사 최종64PASS/3FAIL/2127skip(exit100/342.087초,실행19.191초)이며 새 유입2FAIL은 검사 변경 없이 PASS로 복구됐다. 기존 #6024/#7226 쪽수와 #6981 노동인권 소속 실패는 그대로 남아 있다.
- 새 main두 원본 p22–24의Native review exit1/224.973초이며 HWP95.45728/96.77602/58.89865%, HWPX96.23749/97.06903/57.77669%다(native-stored-first-frame-blank-band-review-v55-retry1). 앞 실패 후보 v55 이미지 재사용이 아닌 retry1 새 바이너리 재출력이다. source가 바뀐 두 실패 원본의 전쪽Native review도 exit1/163.269초다: #6803 p1..5=98.94637/93.43892/75.15690/32.30497/35.53280%, #6935 p1..3=85.79861/54.92011/94.06013%(native-new-failed-originals-review-v55-retry1). #6935의직전 e13 원본도같은85.79861/54.92011/94.06013%였으며 3쪽 복원과 피델리티 실패를 구분한다. #6935 p2직전 review를 직접 표시해 상단 남은 줄과 뒤 표가 밀리는 결함을 판독했다. 원본전쪽90%와fresh WASM 선행 검증은 미충족/미검증이고 formal검사/fixture/기대값/baseline은 바꾸지 않았다.
- 다음 main경계는 table26 row22에서 시작한 rowspan label/frequency의 물리 시작 밴드와 p24 이어받기다. raw single-row row22는1566HU이지만 Print p23 마지막 밴드와 Native의높이가 다르다. row22 col0 span15/col5 span13은 같은 문단두 LS0/0, col4 span9는두 문단LS0/1300이며 row23 이후 여러 행과 마지막 요구높이가 이를 소비한다. 아직 한 가지 원인으로 단정하지 않고 source-owned cut·physical row prefix·straddle_continuation_demand·최종paint를 추적한다. PDF의 반복되는 동일 문구를 좌표만으로 같은 paragraph owner라고 판정하지 않는다. 관련 direct PNG/trace는 기존폴더에 보존한다.

- source122dd1b62 fresh WASM wrapper --no-opt exit0/594.153초다. Docker 없음의 기존 진단 설정이며 optimized 배포 검증은 아니다. WASM SHA cf971b0729c348d407434c4934e8862f65e994aa8f355ad763f36bd4450b9aa4, JS7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a이며 root pkg/Studio public 각각 일치했다. pinned package wasm-pkg-stored-first-frame-blank-band-v55-retry1-122dd1b62에서22–24쪽 review와 원본52쪽 전수TSV를 시작했다.
- 같은 source의 Native HWP52쪽 TSV는 누락0/최저51.53884%(p52)/90%미만11쪽이다. 미달은24/28/29/30/31/39/48/49/50/51/52이며 앞6c 후보13쪽에서22/23쪽이 개선됐다. HWPX와fresh WASM 전수는 아직 진행 중이다. 현재 다음source 변경 뒤 이전 전체를 최신 결과로 재인용하지 않는다.

### 진행 중 보정 38: 저장 병합 셀의 첫 물리 밴드와 원본 내용 컷

- 독립 HWP Print p23 SVG의 괘선을 같은96dpi로 추출했다(independent-print-p23-row-bands-v56.json). 마지막 밴드는1041.578125..1080.260416=38.682291px이며 Native는20.88px만 예약했다. 원본 row22 col4의 연속 LS0/1300·LH1000·padding283+283의 첫 저장 내용은2866HU=38.213333px다. 같은 행에서 시작하는 col0/span15와col5/span13은 각각 같은 문단의두LS0/0·text_start 증가·동일column/width로 새 프레임 재개를 기록한다. 문단 간 원점0이나 한 owner만으로 row frame을 정하지 않는다.
- rowSpan9 col4의 선언15087HU는 현재 단일행 선언 합14094HU보다993HU 크다. 첫 물리 밴드를 single-row 선언1566HU로만 잡은 뒤 선언 잔여를 마지막 걸침 행에 더해 row29/30이 이미 늘어난다. paint D6981R row30의 need20.9/have180.3에서는 추가 성장이 없음을 직접 확인했다(scattered-p24-span-demand-trace-v55-retry1.log). paint에서 높이를 clamp/줄이는 보정으로 해결하지 않는다.
- source 공유 결과는 공동 첫 프레임에서끝나는 내용 prefix+실효 padding의 물리 최소와, 원점0에서 재개하는 각 source owner의첫단위 컷이다. HeightMeasurer의 ordinary-row 측정 뒤/rowspan 선언 배분 전에 같은 최소를 반영하고, LayoutEngine fallback도 같은 결과를 소비한다. straddle 컷·후속 요구높이·paint에는같은 source owner 컷을 적용해 밴드가커져도 다음 프레임의 글자를 미리 소비하지 않는다. 정상 단일 owner/합성LS/편집본/control혼재/전통적인닫힌rowspan 선언 경로는 원본 witness가 없으므로 이 source 결과가 없다. 작은 실제 경계와 기존67검사부터 대조하고, 관련 전체 Native/fresh WASM90% 전에는 formal검사/fixture/baseline을 추가·변경하지 않는다.

- v56 최초 Native build exit0/285.412초, p23–25 두 원본6쪽은 모두90% 이상이다(HWP97.96819/99.55059/99.99178%, HWPX98.07385/99.46439/99.92535%). 다만 직접 p23 review를 읽은 결과 row22 col0의 기층과col5의 정기 사용불가가 독립 Print p24 소유인데 p23에 함께 출력됐다. 점수 개선을 승인으로 판정하지 않는다. 기존67검사는64PASS/3FAIL/2127skip(exit100/362.370초,실행19.339초)이며 기존 원본 실패3개의 검사/기대값은 유지했다.
- retry1은 다른 시작 행의 컷이 새 병합 셀의 owner까지 소비했다는 가정을 제거한다. 첫 프레임 endRow=cellRow+1의 source owner 컷(0,1)은 앞쪽의 다른 행 startCut과 독립적으로 결정한다. 같은 셀의 시작 행에서 이미 컷/잔여높이가 있으면 기존 부분 소비 원장을 유지한다. 다음 행의 빈 startCut으로 재개할 때 su=1을 공유하고, 이후 끝 컷은 기존 소비 원장으로 계산한다. 정상 native 2행/2문단 계약은 그대로며 문서ID/px 예외를 추가하지 않는다. 실제 예약·배치 호출 모두 rowbreak_straddle_cut_units를 소비한다. 동일 원본 재출력과 기존67검사로 다시 확인하며 최초v56 캡처는 retry1 증거로 재사용하지 않는다.

- source122 Native 전수는 exit1/1973.968초, freshWASM 전수는 exit1/2212.610초로 둘 다 종료됐다. HWP/HWPX 각각52쪽/누락0, Native=freshWASM 전체 페이지 점수가 일치한다. HWP 최저51.53884%(p52), HWPX52.34972%(p52), 각각90%미만11쪽(24/28/29/30/31/39/48/49/50/51/52)이다. full-score-observation-122dd1b62-v56-complete.json 및 전수TSV로 연결하며 진행 중인v56 retry1의 검증과 구분한다.

- 다음 미달 구간의 원인 조사용으로 source122의 pinned Native를 다시 실행했다(native-scattered-next-boundary-review-v55-retry1, HWP28–31쪽). p28 review_028.png를 대화에 직접 표시했다. 독립 Print의 첫 모양 및 치수·포장용·시험방법 KS F4001에 대응하는 내용이 앞쪽에 없고, 하단 경계는 Native가 다음 시험항목을 먼저 소비하는 차이가 보인다. 점수55.49206%의 위치 차이만으로 분류하지 않고 table pi33의27→28쪽 저장 줄/셀 소유와 컷 원장부터 확인한다. 이 출력은source122 원인 조사이며 새v56 retry1의 검증으로 인용하지 않는다.

- v56 retry1 Native build exit0/265.239초, binary SHA 84757c57899e7d1106c22a501fd1f026dc95fba0cd084abb441e83e4a0ba8d10다. 두 원본23–25쪽 review exit0/208.641초: HWP98.38810/99.55059/99.99178%, HWPX98.52414/99.46439/99.92535%다. 새 p23/p24 review를 대화에 직접 표시했고, first owner는p23에만/다음 owner는p24에만 존재함을 table pi26,row22,col0/5로 연결했다(scattered-source-owner-observation-v56-retry1.json). 절대좌표를 formal 기대값으로 추가하지 않았다. CLI의-p가0기반이라는 점을 진단 command에 바로잡고, 페이지파일명으로 실제23/24쪽을 확인했다. freshWASM 및 기존67개 재실행은 아직 진행/대기이며 전쪽90% 승인은 보류한다.

- v56 retry1 기존67개 검사는64PASS/3FAIL/2127skip으로 종료됐다(exit100/344.754초,실행20.636초). 기존 원본 #6024 위임전결규정 쪽수, #7226 교육과정 쪽수, #6981 교육과정 노동인권 소속 세 실패가 유지됐고 추가 실패는 없다. 원본의 전쪽90% 선행 개선이 미완료여서 검사/기대값/baseline은 그대로다. Native 검증을 마친 source만 로컬 체크포인트로 고정하고 같은 head의 freshWASM 검증을 이어간다. PR 생성·승인·merge는 아직 하지 않는다.
- 같은 v56 retry1의 다음 경계 HWP p27 review는96.18082%지만 하단 row51의 새 포장용 콘크리트 평판 첫 행을 한컴보다 앞 쪽에 소비한다. 독립 Print는 row50에서 p27을 끝내고 row51을 p28로 이월한다. source Table pi33,row51의 rowspan11 owner는 문단/줄 원점이0/1500/3000으로 연속하고 KS F4001 owner는 한 줄이다. score와 직접 판독을 분리해 해당27쪽도 완료로 판정하지 않는다. scanner의 BLOCK_UNIT_MAX_ROWS=3과 실제 블록/예산을 원인 후보로 확인하며 행 수 제한을 곧바로 변경하지 않는다.

### 진행 중 보정 39: 저장 병합 셀의 연속 owner와 조각 최소 점유

- v56 retry1 HWP27–31쪽 새 review는 exit1/215.046초로96.18082/55.49206/64.77340/65.38166/69.69944%다. p27/p29 review를 직접 표시했고 p27 점수 통과와 소유 결함을 구분했다. 정확한 compiled source는 c06b6b32bc111906a47560565c24f5769edceb0d와5개 파일 SHA 모두 일치한다(source-commit-stored-span-opening-frame-v56-retry1.json). 새 head의 freshWASM 빌드는 같은 target/pr-review에서 순차로 실행 중이다.
- readonly 실제 MeasuredTable에서 pi33,row51 블록은51..62/229.68px이고3행 제한으로 보호 분류가 안 된다(scattered-table33-measured-blocks-v56-retry1.tsv 및 로그). 하지만 독립 Print p28→p29는row77..81의4행 블록을 실제로 쪼갠다. 따라서 페이지에 들어가는 모든 병합 블록의 통째 이월은 올바른 일반 규칙으로 입증되지 않았다. 행 수 제한을 단순히 확대하지 않는다.
- 독립 Print p27의 마지막 괘선은1060.119792px/마지막 row50, p28의 마지막 row77..79 조각은999.385416..1080.260416=80.875px이다. 그 안의 두 일반 행 경계는1020.322917/1041.098959이고 마지막 밴드는39.161457px여서 선언1566HU=20.88px와 다르다. row77 col0의연속 저장 문단LS0/1500/3000/4500,LH1000,padding283+283의 내용 점유는6066HU=80.88px다. 네 owner를모두 p28에 표시하고 p29의row80는본문 표면층 두께만 남는다. 단순 row합/각 unit의순차 높이로 내용 owner를 분할하면 독립 출력과 다르다. source owner prefix와실제 부분 조각의 물리 최소를같은 결과로 예약/배치해야 한다는 원인 후보로 연결한다(independent-print-next-row-band-observation-v56-retry1.json, scattered-next-owner-independent-bbox-v56-retry1.json). 이좌표는진단 근거이며formal px 핀이 아니다.
- row51의frequency 저장 prefix는LS0/1600/3100/4600,LH1000,padding566으로6166HU=82.213333px다. 첫ordinary row가남은 공간에 들어간다는 이유만으로 source prefix의첫 owner만 앞 쪽에서소비하면나머지 label/시험방법 소유가 어긋난다. 정확한 예약 후보는전체 블록 높이를강제하는것이 아니라 해당source frame의 unit소유와최소 점유다. 부분 조각의누적 높이·수용 실패컷/이월·최종paint분기에 실제로 전달되는지 분석하며 코드가 바뀌기 전까지 해결된 것으로판정하지 않는다.
- GitHub를 다시 조회해semanticist21 open24 non-draft/6draft, 신규·종료·head/draft 변경0임을 확인했다(github-open-pr-snapshot-v56-retry1.json). 초기18개와deferred6개 범위를 유지하며원격 변경은 하지 않았다.

- c06b6b32b freshWASM wrapper --no-opt exit0/575.819초, WASM230ba446f2a2466abd39898db7c0a66b4611a0961d7e11949f1e042ddac1488c, JS7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a다. root pkg/Studio public 해시가 각각일치했다. pinned wasm-pkg-stored-span-opening-frame-v56-retry1-c06b6b32b의두 원본23–25쪽 full review는 exit0/440.139초이며 Native와각페이지의 점수가동일하다(HWP98.38810/99.55059/99.99178%, HWPX98.52414/99.46439/99.92535%). 아직27–31등남은 배치 결함이 있으므로 전쪽 승인이나formal 회귀 추가·기대값 변경의근거로승격하지 않는다.
- 실제Studio 기존vite7700에서 새 Chromium context/캐시 비활성/goto+reload 후 원본HWP를로드했다. 응답buffer수집기가navigation 뒤끝나지 않은자체하네스 오류를관측해 동일하게살아있는브라우저에서CDP로문서52쪽/view52쪽/font loaded/canvas4와23/24쪽owner를직접읽었다. 실제http public WASM13,852,065bytes의SHA가pinned와일치했고동일owner분리가유지됐다(browser-studio-stored-span-opening-frame-v56-retry1-observed.json,oktrue). 최초수집기는제품FAIL증거로세지않으며원로그를보존했고,관측완료뒤자체browser만종료했다. 화면profile의실제로딩확인은Print Visual Sweep의시각승인과구분한다. 최종범위전체Studio/API/CI 검증을대체하지않는다.

- v57의 생산 결과는 같은 source cell의 모든원본LS가문단을넘어엄격히증가하는연속사다리의inkEnd+실효상하padding이다. 원점되감김/같은원점·dirty/reflow·control혼재·TAC·RowBreak외경로·선언cell.height와모순된사다리는새계약을입증하지않으므로기존컷경로를유지한다. 긴page-larger source도기존unit walker를유지하며,새 쪽의 유효 row budget에들어가는연속source frame만한 owner묶음으로예약한다. 빈 문단의LS도점유에포함한다.
- 실제경로: float_placement/stored_span_content_frame의원본점유 → LayoutEngine.stored_span_fragment_minimum의cell원점+내용최소 → scan_block_table_split_rows의행진입예산판정/최종fragment consumed+end_row_height_override → emit_table_fragment의partial_height/PageItem/record_stored_fragment_rows → stored_fragment_row_plan의수용행원장/renderer.layout의fragmentMeasurement → table_partial의마지막행높이/rowbreak_straddle_cut_units의같은실제prefix다. 원점계산에는cut cache대신paint의MeasuredTable 또는이미수용한fragment_row_heights를전달한다. record도원래MeasuredTable이아닌이번실제fragment계획을기록해닫힌span의잔여와다음owner가같은물리prefix를소비한다.
- 수용한마지막행에만source내용점유와실제행합의차이를기록한다. 모든중간row 후보에같은최소를더해중복예약하지않고,의미컷과물리높이Override를분리한다. 기존header의꼬리빈밴드fill조건은넓히지않으며,연속source가있는비header표의물리원장보존만같은계약으로확장했다. 먼저원본의실제27→28→29쪽컷/owner/빈밴드와기존67검사를보며,최종90%전formal회귀/fixture/기대값/baseline은변경하지않는다. 새분기의명시적start/end내용컷및page-larger경계는기존경로이고최종유효성검증은아직진행하지않았다.

- v57 최초build exit0/255.406초, binarya3f1241c14f03ee556f9f8e5035a1b19011183bc0219b569f5d3dcb4d7ef2711지만빠른원본컷검사에서main53쪽/교육과정416쪽으로서실패후보로보존했다. 최초경계차이는p3 pi9endRow9 vs기존10/endCut이며새p4가row9하나의partialcut만소비한뒤강제새쪽을만들었다. #6935는3쪽/#6803는5쪽/위임전결106쪽이다(pages-*-v57.json).
- readonly원본MeasuredTable대조에서pi6의61..84와pi9의0..9에는source 최소가실제physical을넘는owner가없다(continuous-span-first-mismatch-v57.log). 새p2 usedHeight는1009.1467→988.7733으로줄었다. floating 합순서의미세양수차이에end_row_height_override를만든것이숫자크기와무관하게기존stored header tail/frame선택을막은자체오류다. retry1은source 범위를좁히는대신원래비교의부동소수점합오차상한(EPSILON×두높이규모×합항수)을넘는실제점유증분일때만override를만든다. 실제source밴드18.24px를줄이거나px허용치/문서ID조건을넣지않는다. 새SOURCE_SPAN_FRAME진단을통해수용한증분을기록한다. 기대값·baseline은유지하며다시동일원본컷과smallNativeSweep부터실행한다.

- retry1 build exit0/265.443초(binaryd927d5d6512fc2e15b61e12c4657e4ce2b0bef370196f0c9fbb4b668e7bbe081)뒤같은53/416쪽을유지했다. 새진단은pi6 rows23..61의physical962.347/minimum962.800/last21.333으로실제증분0.453333px를보여서직전합산오차만의원인가설은불충분했다. 원인판정을이실행증거로정정한다. 부동소수점noise guard는수치로새override상태를만들지않는용도로유지한다.
- 실제누락소비분기는scan_table_fragment의stored header tail이다. end_row_height_override.is_none을내용완전소비의대용으로써서새source minimum의양수확장도압축/partialband처럼배제했다. retry2는빈endCut/완전행조건과기존독립sourceFrame/body예산검사를유지하고,override가없거나원래whole-row 높이이상의확장인경우그행의저장빈밴드도수용한다. 음수압축/실제unit컷은기존조건그대로다. 새source내용증분18.24px의row77..79는기존wholeRows/consumed관계에서빈tail대상이아니며새px/문서ID조건을추가하지않는다. source값→최종예약→PageItem→paint의override선택까지대조한소비지점수정이다.

- retry2 build exit0/257.224초/binary9bbfa9dcaf48ef25cf6e8b9dc318f5659ee0fd0bc5e69ae4f3c3dd5290865686에서HWP52쪽과p3/p4원소유가복원됐다. HWP의새p3/p4/p27..31은99.87432/99.96125/99.77061/99.14356/99.97446/97.15990/99.13203%이며p28 review를직접표시해첫label·KS F4001·하단row77..79의4개문단owner와다음쪽row80소유를확인했다. 기존67개64PASS/3FAIL/2127skip(exit100/350.207초,실행19.411초),검사/기대값변경없다. 하지만HWPX53쪽/교육과정416쪽이고freshWASM아직없으므로완료나새fixture근거로보지않는다.
- HWPX최초새차이는p5 pi9 row96에서나타났다. Native HWP의label/frequency마지막LS는0으로재개하지만HWPX원본XML에는4800HU로연속되어있다. 이는파서보정/수동LS가아니며version.xml은정식Hancom2020 11.0.0.2129,입력첫commit은7f6c770b8(Task1716)이다. styles59/61및대상row51/77의styles도widowOrphan/keepWithNext/keepLines=0이다. 원본저장LS의연속성만으로모든내용이한쪽source frame을소유한다는가정은두형식의독립Print와충돌한다. 형식/문서ID예외를붙이지않고그가정을제거한다(scattered-full-source-*-v57.tsv와원본XML).
- retry3의공통생산결과는연속source line의각prefix별ink/padding끝이다. 행진입은기존MIN_TOP_KEEP_PX정책을source내용점유에적용한최소prefix를예약하고,최종내용창은실제해당cell의남은fragment budget에들어가는최대prefix를선택한다. short owner는전체가같은최소이며page-larger/originalreset/dirty/reflow의기존walker계약을유지한다. 완료된cell을현재끝경계의걸친owner로사용하지않는다. 실제band의증분은같은마지막행override/누적예약/수용원장으로공유하며모든줄을첫프레임에강제하지않는다. 새source birth가설·수정방향이바뀌었으므로retry2의PNG를retry3의검증으로재사용하지않는다.

- v57 retry3 Native build exit0/256.813초, binary SHA59491481a5e833e0fd5c8b1f465388cbe3a2da83a5ebd2f6d965ffa7b4c07a6b다. 원본 HWP/HWPX가 모두52쪽으로 복구됐으며 p5 pi9 end99 / p6 start99로 실제 연속 HWPX 저장 줄의 쪽 경계 반례를 유지한다. p27 pi33 end51 / p28 51..80 / p29 80..126 / p30 126..171 / p31 171..214로 앞 HWP 개선도 보존했다(pages-*-v57-retry3.json). #6935=3, #6803=5, #6024=106, 교육과정=414쪽이며 독립 Print의3/5/103/415쪽과 구분한다. 기존 실패 기대값을 바꾸지 않았다.
- retry3 실제 RenderTree에서 pi9 row96 label은 p5 앞3줄/p6 마지막 규격 문단, pi33 row77 col0의4문단은 전부 p28/같은 셀 p29는빈내용이다(stored-continuous-span-page-ownership-v57-retry3.json). 독립 Print bbox도 실제 대응 셀의 글자를 확인한다(independent-print-prefix-page-ownership-v57-retry3.json). PDF 전체의 반복 규격 문자열 존재만으로 owner를 판정하지 않는다. p3 새 review99.87432%를 직접 표시·판독했고 표 외곽/행/하단 제목 위치가복구됐다. p3–6/23–25/27–31 선택 Native review와52쪽 전수TSV, 기존67검사가 진행 중이며 아직 최종 head의 freshWASM/전체90%·lint/전체회귀·원격 통합은 미완료다.

- retry3 기존67개 재실행은 exit100/137.222초/실행21.060초로64PASS/기존3FAIL/2127skip이다. 최초 handle75726은 컴파일 도중143 종료됐고 summary가없으며 Cargo/Rust가없음을확인한 뒤 rerun했다. 환경 종료를 test FAIL로 집계하지않는다(nextest-stored-continuous-span-reservation-v57-retry3-interrupted.json 및 rerun exit/log). 실패원본의검사·기대값·baseline은그대로다.
- retry3 p25는99.99178→94.64765%로낮아졌으며 새 review를직접표시해 마지막첫조각의밑괘선이18.72px 위로올라간실제차이를확인했다. pi29 row0..42의 source최소가949.907→950.360px/last21.333px로증가했고 saved_first_fragment_source_frame(1004.04px)을예약하는경로의end_override없음조건이complete row를잘못제외했다. continuation에서이미해결한것과동일하게 첫조각도minimum growth와compressed cut을구분한다. 기존source frame/내용컷/기존0.5비교는유지하며단순출력clamp나문서ID예외는추가하지않는다. retry4에서첫조각tail과두형식반례/기존67개를재실행하며retry3이미지를최신완료증거로재사용하지않는다.

- retry4 Native build exit0/282.480초, binary SHA6f21310e6cefc99be76ff8d5e8434cb0a603a6ff36781e3a5785196d66d098f7이다. 두원본52쪽을유지하며 p25점유가989.08→1007.8px로이전독립출력대응프레임을복구했다(pages-*-v57-retry4.json). 두형식p25새review는계속진행중이며기존retry3결과로최종승인하지않는다.
- 2026-10-09 새fetch에서upstream/devel=f0e7228f6→5168e3256ad58eda425d84fdd4d67e9b47bb74b1을확인했다. #7644/#7620의새상위저장표·중첩컷보정이실제소비경로에들어오므로현단계source/진단기록을체크포인트커밋후최신base로rebase한다. 기존Native/freshWASM결과는원래source의이력으로유지하고rebase후정확한head에서재검증한다. generated Studio rhwp.js의기존작업은보존한다.

- rebase완료head=3cfd4b6a2c39f13fc3b983834b85cfa4df5ba429/base5168e3256이다. pre/post82커밋, range-diff삭제/추가0이며변경표시1개는float helper의새upstream함수명주변context만바뀐것이다. Git본문의42개cherry-pick출처도순서/해시가모두유지됐다(rebase-v57-20261009.json, range-diff, rebase-v57-contributor-provenance.json). 기존StudioJS는7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a로정확히복원했으며stash4cc75327016aa2270367e1b73db331d6656b319b도보존했다. rebase초기동시git status의일시적index lock은이미없어진것과실행Git없음을확인한후동일rebase를continue했고commit누락없음을검증했다.
- 새base고정manifest/unit-tier정책검사PASS다(1458sources/6264static attrs/48targets, 4113units/297modules). 파생suite배정이달라졌으므로기존67함수의13모듈을새생성파일에서재매핑하고upstream의#7644/#7620/빈문단뒤charshape3모듈을추가해16모듈/13targets명령을준비했다(nextest-stored-continuous-span-rebased-v58-command.json). 과거suite번호를그대로재사용하지않는다. 새headNative v58빌드는진행중이며기존컴파일결과로rebasehead를통과판정하지않는다.
- 이전source retry3선택24쪽Native는최종exit0/961.799초이며두형식52쪽/p3–6/23–25/27–31측정누락0이다. HWP최저94.64765%(p25), HWPX최저95.36790%(p25);27–31쪽HWP최저97.15990/HWPX97.33927%다. p25의직접판독결함은retry4에서HWP99.99178%로복구했다. rebase중기준PDF가일시적으로없어retry4의HWPXp25실행은변환환경오류로미완료다. oldsource를최신head증거로인용하지않으며currenthead/원본해시/참조PDF를다시고정해재실행한다.

- rebase 후v58 Native build exit0/255.326초이며binary SHAee5c65148e9722b859be00e3e63e6f7e784ced271ee313b9192d7ff7c18c95c9이다. 컴파일된11개소스해시는정확한3cfd4b6a2commit과모두일치한다(source-commit-stored-continuous-span-rebased-v58.json). 새basehead에서기존67+상위3모듈의회귀와두형식p3–6/23–25/27–31Native review, 잔여직접차이p10/39/48–52review를재실행중이다. 이전retry3 HWP전수52쪽은미달6쪽(39/48/49/50/51/52),최저51.53884%이며그대로최종승인에사용하지않는다.

- 정확한 rebase head 3cfd4b6a2에서 73개 경계 검사가 exit100/417.677초/실행23.166초로 70PASS/기존3FAIL/2767skip 종료됐다. 이전 67개는 64PASS/3FAIL을 유지하며, 새 upstream의 세 모듈 6개 검사도 모두 PASS다. 실패는 #6024 쪽수와 #7226 쪽수/#6981 마지막 내용 소속 그대로다(nextest-stored-continuous-span-rebased-v58-exit.json 및 log). 원본의 독립 Print 전쪽90% 선행 조건을 아직 충족하지 못했으므로 테스트·기대값·baseline은 변경하지 않았다.
- 같은 v58 head에서 두 원본52쪽/p25 점유1007.8px, #6935=3/#6803=5/#6024=106/교육과정414쪽을 재확인했다(pages-*-v58.json). 현재 Native 선택 review와 잔여 직접 결함 review가 진행 중이며, Cargo 종료를 확인한 뒤 root wrapper로 fresh WASM을 순차 시작했다(wasm-build-stored-continuous-span-rebased-v58). 기존 7700 Studio의 package/public 동기화와 실제 reload도 새 package에서 확인할 예정이다.
- 잔여 p39의 독립 Print는 첫 data row가 source pi53 row15 “단열성”이고 현재 Native는 row16 “내화성”부터다. p38의 같은 표는 Native가 row15까지 앞 쪽에서 소비한다. 독립 bbox와 원본의 Common.height25152HU/행 높이, 실제 첫 조각 예산을 연결해 잘못된 소유를 추적 중이다(independent-print-page39-v58.txt, scattered-original-table-53-v57.dump.txt, pages-hwp-v58.json). 반복 규격의 텍스트 존재나 최근접 행 경계만으로 정답 컷을 인증하지 않는다. 아직 다음 보정 구현/검증을 마치지 않았으며 새 formal 검사·기대값은 추가하지 않는다.

- 새 v58 HWP 선택12쪽의 실루엣은 모두97.15990% 이상이고 p25=99.99178% 복구를 유지한다. 별도 잔여 review의 HWP p10/39/48/49/50/51/52는91.26492/59.42662/67.50142/77.18501/81.04910/53.91818/51.53884%다. p39 새 review PNG를 직접 표시·판독했으며 첫 data row 누락/뒤 표의 상향 이동을 확인했다. p10도90% 초과 점수로 이전 직접 차이를 면제하지 않는다. 두 sweep의 HWPX 단계와 fresh WASM은 계속 진행 중이므로 전수 완료·승인으로 보고하지 않는다.

### 진행 중 보정 40: 저장 첫 조각의 물리 프레임과 행 소유

- v58 Native 개선12쪽/두형식24쪽 review가 exit0/932.016초로 종료됐다. HWP 최저97.15990%, HWPX 최저97.33927%이며 p25는99.99178/99.92535%다. 잔여14쪽 review는 exit1/585.662초이고 p39 HWP59.42662/HWPX59.59609%, p48–52도 두형식 모두90% 미달이다. p39를 직접 표시·판독했다.
- 같은3cfd4b6a2의 freshWASM root wrapper --no-opt는 exit0/599.193초다. WASM d58b8545f12ef2b806a0534bb8b49a1874a4b233ee9275b330203c21855b1b8f, JS7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a이며 pkg/Studio public이 각각 일치했다. pinned package의 선택24쪽 review와 실제 Studio reload를 시작했다. optimized 배포 산출물 검증은 아니다.
- p38 pi53 원본 Common.height25152HU=335.36px인데 실제 row scanner 예산은336.7px/source_end16/overflow allowance20.2px다(scattered-first-frame-physical-budget-v58.log). 그래서336.333px의16행을 소비하고 p39를 row16부터 시작한다. 독립 Print p38 첫조각의 괘선은745.265625..1080.260416=334.994791px이고 row14 방음성 뒤 빈밴드를 포함한다. p39 첫행은row15 단열성이며, 저장 개체 프레임이 다음 행까지 소유한다는 최근접 추측이 틀렸음을 확인했다(independent-print-p38-row-bands-v58.json/svg, independent-print-page39-v58.txt).
- 기존 stored_row_boxes_exceed_first_frame의 원본 single-row 선언 합/spacing 하한이 Common.height를 넘으면 전체표가 그 상자 안에 있다는 가정은 성립하지 않는다. 인정된 저장 첫 프레임과 원본 무편집/무재조판·credible 조건을 연결해 그 물리 높이를 실제 row budget으로 소비한다. 행 소유는 현재 단일 row/unit walker가 정하며 최근접 행 강제 수용과 별도 overflow allowance를 사용하지 않는다. 내용 컷·최소 높이·선언 프레임의 빈밴드 예약은 실제 PageItem/paint에 전달한다. 전체 표가 선언에 들어가는 정상 대조군과 편집·각주·재조판 경로는 기존 계약으로 대조한다. 임의 px 허용치/문서ID/출력clamp/내용 숨김은 추가하지 않는다. 기존73개 및 원본 두형식/실패원본의 새 실행으로 방향을 확인하며 formal 회귀·기대값·baseline 변경은 계속 보류한다.
- GitHub 재조회는24non-draft/6draft로 범위와head가 유지된다(github-open-pr-snapshot-v58.json). 초기18개와 이후6개를 순서대로 처리하는 범위를 유지하며 원격 변경은 하지 않았다.

- v59 Native build exit0/262.397초, binary SHAaf53f124fa0b911267a22a209a9b7d416b2df86b4f31f7e6a3c2c7f6d23a25ca다. HWP/HWPX 각각52쪽이며 p38 table53 end15/p39 start15로 독립 Print의단열성 쪽 소유를 복구했다. p39 table56 end10/p40 start10과 기존 p3/4/23–31 내용컷도 유지했다. #6935=3/#6803=5/#6024=106/교육과정414쪽이다(pages-*-v59.json). 같은 소스의 p38–40 Native review와73개 기존 경계 검사를 진행 중이며 아직 최종 통과 판정은 아니다.
- v58 실제 Studio reload는 exit0/oktrue, servedWASM=d58b8545f12ef2b806a0534bb8b49a1874a4b233ee9275b330203c21855b1b8f/예상 해시 일치, 문서52쪽/글꼴loaded/23→24쪽 row22 label·frequency의 원본 소유 유지다(browser-studio-stored-continuous-span-rebased-v58.json). 별도 화면 동작 확인이며 Print Visual Sweep 승인과 구분한다. v58 package의 새 WASM 비교는 계속 진행 중이다.

- v59 선택6쪽 Native는 exit0/280.896초로 HWP38/39/40=99.75942/99.97808/100.00000%, HWPX=99.80002/99.97060/99.93943%다. p38/39 review를 직접 표시하고 선언 밴드/단열성 소유/뒤 표 위치가 독립 Print에 맞는지 확인했다. 하지만 기존73개는68PASS/5FAIL/2767skip(exit100/382.640초/실행19.929초)이고 새 FAIL은 #1133 두 형식의 첫조각 endRow4vs5다. tool session의 관측 exit143과 wrapper가 남긴 nextest exit100/명시summary를 구분하며 실행으로 확인한5FAIL을 기록한다. 검사·기대값은 유지한다.
- #1133은 합성이 아닌 실제 samples/issue_1133.hwp(ea03105fa7fa22398f3e370d222f37213e986ecca4b9b97d2847f1bbfe998d3b)/samples/hwpx/issue_1133.hwpx(2cf479d94a70d2a834ae7bc1f5ec5fa3d7ff895792cded48d0bf92ca19341bee)다. 독립3쪽 Print pdf/issue7382-regression-review/issue_1133-2020.pdf(46ca5b1abc40d9479e9eea7994b951ebb82d177ebc448cd09688170effd850ae), pdf/hwpx/issue_1133.pdf(ca066fdc5cda9fd8c47c9a29ba9a546e70c6acdaa96df4d6c66e0b8e02cdbc45)로 전쪽 Native 비교를 시작했다(native-new-failed-originals-issue1133-v59). 두 입력 모두3쪽이지만 첫조각 행 소유가 달라졌으므로 쪽수 유지로 통과 판정하지 않는다.
- #1133 첫5행 선언/측정 합614.0667px가 source602.7333px보다11.3333px 크지만 마지막 row4의 전체 내용은 남은58.2933px에 들어가는지 별도의 공통 complete-cut 높이로 확인할 수 있다. 첫 v59는 선언 상자와 내용 최소를 같은 것으로 취급해 이 unused physical space를 절대 불가로 보았다. retry1은 전부 소비 가능한 일반행의 물리 나머지를 source_complete_frame_last_row에 전달해 기존 scan/paint가 같은 높이를 사용한다. span crossing은 별도 continuation owner/cut의 요구이므로 완료행 상자에서 그 내용을 추측하지 않는다. Main row15의 최소20.88px는 남은19.9067px에 못 들어가므로 실제 이월을 유지한다. 조건만 더 붙여 대상을 좁히지 않고 내용 점유와 선언 공간을 구분하는 원인층을 수정한다. 원본 모든 관련 쪽의 Native/fresh WASM90%가 끝나기 전에는 formal 검사/기대값/baseline을 바꾸지 않는다.

- source3cfd4b6a2 v58 freshWASM 선택24쪽 review는 inner exit0/1124.250초이며 terminal session의외부143과구분한다. 두형식52쪽/측정누락0,12쪽씩의 Native/freshWASM 점수가모두동일하다(native-wasm-selected-score-parity-v58.json). 최저HWP97.15990/HWPX97.33927%이고새v59소스나미달잔여쪽을통과판정하는증거로재사용하지않는다.
- 첫v59의새실패원본#1133 전쪽Native는exit1/168.248초다. HWP p1/2/3=96.19594/94.11056/30.55666%, HWPX=96.21383/94.11150/30.55666%로실제 p3 내용/위치회귀를확인했다. 독립기대값이나px검사를바꿔실패를없애지않으며retry1의공통완료행최소/물리나머지소비연결로출력을복구한다. 실패PNG와원본은보존한다.

- retry1 Native build exit0/254.735초, binary SHA661381aa64bace8e058d4b01512afc1fb96b3752a4720587a17887b4e4251a4d다. Main두형식52쪽/p38 end15/p39 start15와#1133 두형식3쪽/first0..5/next5..8을복구했다. #1133새전쪽Native는 HWP96.19594/98.92603/99.86388%, HWPX96.21383/98.92668/99.86388%이며 main38–40 점수도앞후보와같다. 하지만교육과정은414→411쪽으로바뀌었고73검사68PASS/5FAIL/2767skip(exit100/360.678초/실행20.461초)이다. 새FAIL은#7226인접예산의실제본문넘침과#6981전남조례83쪽소유다. source입력변형/실문서검사를구분하고테스트·기대값은바꾸지않는다.
- retry1의잘못된가정은'선택한행뒤에span이안이어지면전체내용이끝났다'였다. row_complete_cut_content_height는현재행에서시작한single-row owner의요구만계산하며,그행에서끝나는이전rowspan owner의미소비유닛을포함하지않는다. 작은반환높이로끝나는span행을압축하면뒤block·glyph실제배치가본문을넘을수있다. retry2는실제active owner집합이전부현재행의ordinary owner일때만이완료행최소를소비한다. 끝나는/이어지는span의owner·유닛컷은기존walker의실제결과에남기며추측하지않는다. SourceFrame전체heightbudget규칙과1133ordinary행의물리나머지는유지한다. 실제원본/경계/독립Print재실행에서main39와1133복구를보존하면서새회귀가사라지는지확인한다.

- retry2 Native build exit0/255.425초, binary SHA9d7e743c0dbd1fe119f82976fa9b26ff70b674d41f347919024eb2f0cc852d12다. source-code-v59-retry2와4개소스해시로고정했으며,정확한후보에서기존73개/원본Main38–40/새실패원본#1133전쪽6페이지를재실행중이다. 수정전실패·이전Native개선자료를새후보통과로보고하지않는다. #1133절대px검사의의미교정은원본두형식의Native/freshWASM전쪽90%및독립기대값대조후에만진행한다. 정확한쪽/행/내용소유검사는유지한다.

- retry2 최종 기존73개는 exit100/364.897초/실행18.753초로70PASS/기존3FAIL/2767skip이다. 새#1133두검사와#7226인접예산/#6981빈밴드검사는구현수정만으로PASS복구됐다. 정확한검사·기대값·baseline은그대로다(nextest-stored-partial-frame-budget-v59-retry2-exit.json/log).
- 같은retry2의Main두형식38–40쪽review는exit0/223.104초, HWP99.75942/99.97808/100.00000%, HWPX99.80002/99.97060/99.93943%다. 새실패원본#1133두형식전쪽review도exit0/169.647초, HWP96.19594/98.92603/99.86388%, HWPX96.21383/98.92668/99.86388%다. 원본SVG와독립PDFSVG를같은96dpi좌표·200%배율로재렌더한vector-zoom-p39-v59-retry2.png에서단열성/시험방법/개폐반복성/시험빈도·괘선을직접판독했다. 큰위치·줄소유차이는복구됐고작은원점/획차이는별도계측과구분한다. 진단crop은formal절대px기대값이아니다.
- 교육과정414쪽과정상대조#6935=3/#6803=5, 위임전결106쪽을재확인했다. 실패원본의독립Print415/103쪽과모든쪽90%조건은아직미충족/미검증이므로검사·기대값변경및PR통합을보류한다. Main전체미달48–52쪽과나머지초기18개범위,최종Rustlint/전체nextest/editedAPI/원격CI·merge·후속처리는남아있다. 현재소스와검토기록을로컬체크포인트로커밋하고정확한head의freshWASM/원본3쪽및영향3쪽을재검증한다.


### 진행 중 보정 41: 후반 표의 원본 줄 경계 대조

- 지침 변경 여부만 확인하라는 요청에서는 `upstream/devel...e5f66600c`와 미커밋 diff의 AGENTS/CLAUDE/manual/tech 지침 변경이 없음을 읽기 전용으로 확인했다. 기존 visual_fixture_evidence의 원본 Native/fresh WASM90% 선행 조건과 의미 검사 계약을 유지한다.
- 같은 e5f66600c의 pinned Native9d7e743c로 Main 두 형식47–52쪽 review를 새로 실행했다(exit1/408.300초). HWP98.20888/67.50142/77.18501/81.04910/53.91818/51.53884%, HWPX98.14675/67.01272/77.04138/80.89954/53.24873/52.34972%이고 전체52쪽씩이다. 첫 조각 보정으로39쪽이 개선됐지만48–52쪽의 남은 배치 결함은 복구되지 않았다(native-stored-partial-frame-budget-remaining-review-v59-retry2/). 이전 v58 PNG와 최신 재출력을 구분했다.
- 독립 Print47쪽에는 row99의 ‘중금속 함량(6가 크로’ 첫 줄이 있고48쪽 첫 데이터는 ‘뮴)’이다. 현재 Native의47쪽 pi59는59..99/endCut=[],48쪽은99..144/startCut=[]여서 row99 전체를 뒤 쪽으로 이월한다. 두 genuine 원본의 row99는한 문단/두 저장LS로ts0→12, vpos0/0, 같은column/segmentWidth10612HU, LH1000/LS600을가지며 dirty 수동입력으로 만든 자료가 아니다(scattered-row99-boundary-original-evidence-v59.json 및 두 원본dump). 실제DIAG_SCAN은 content budget17.2px, padding7.5px에서firstunit21.3px/endCut[1]을선택하는 시도를 보였다. 선택된 내용 컷의 간격/물리 높이와 orphan/최종 예산 소비 경로를 원인 후보로 추적한다. 원본의 동일0 위치만으로 새 프레임을 선언하거나 허용치를 늘리지 않으며, 정확한 분기·일반성·비적용 반례를 확인하기 전에는 완료로 판정하지 않는다.
- #1133 Native 2/3쪽 canonical review를 직접 열어 앞 flow table, 전형표0..5, 다음쪽5..8, 중첩 유의사항, 뒤 내용과 표 외곽을 대조했다. 이 Native 판독은 freshWASM 선행 조건을 대신하지 않는다. 정확한 기존 소유 검사는 유지하며 절대px 기대값 교정은 같은 원본 두형식의 Native/freshWASM 전쪽90% 확인 뒤에 진행한다.
- GitHub 읽기 전용 재조회는 비draft24/draft6, 신규·종료·head/draft변경0이다(github-open-pr-snapshot-v59.json). 초기18개 우선과 deferred6 별도 검토 범위를 유지했다. 원격 push/PR/approval/merge/postmerge는 수행하지 않았다.

- e5f66600c freshWASM wrapper --no-opt은 exit0/578.307초, 소스해시14개 변화0이다. WASM69f22597370ec3172b5e0654531bd0223a0bab7e038507d4a98c1c4acac835c0/JS7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a가root pkg/Studio public과일치했고pinned wasm-pkg-stored-partial-frame-budget-v59-retry2-e5f66600c에보존했다. 실제Studio새브라우저/goto+reload/캐시비활성으로52쪽/글꼴loaded/canvas 및 servedWASM해시 일치를 확인했다(browser-studio-stored-partial-frame-budget-v59-retry2.json,oktrue). 이는Print시각통과와구분한다.
- 첫WASM두sweep는디스크ENOSPC로환경실패했다. 파일쓰기실패로stored_zero_origin.rs가비워진자체편집오류는즉시e5f66600c 원본과동일하게복구했다. 실패로그/빈exit기록의환경원인을보존했고제품회귀로세지않았다. 종료된과거WASM실행의중복SVG는같은크기와SHA-256을확인한뒤같은내용의hardlink로공유해원래경로/바이트를보존했다(space-recovery-identical-svg-v59.jsonl). 임시로압축한과거SVG1개도압축해제해원래경로/해시로복구했다(space-recovery-v59.json). 공유target/pr-review와현재작업자료는삭제하지않았다. 공간확보후e5pinned #1133전쪽과Main38–40쪽을새space-retry1경로로재실행중이다.
- v60후보의생산계약은같은원본문단의인접저장줄/전진하는textStart/동일양수폭·열의0→0재시작이다. 원본rowbox합이Common첫개체frame을넘는경우그첫box만으로뒤쪽줄프레임을배제하지않는다. 원본문단사이로컬원점/dirty/reflow/다른열·폭/완전개체상자/합성LS는새근거가아니다. source재시작→cell_units의stored_frame_break→ordinary advance_row_cut/native_saved_reset_cut_trailing_trim→row_cut_content_height의같은가시끝→row_step orphan판정및실제split_total예산→fragment의내용컷·물리높이→table_partial paint로연결한다. 같은내용의Native/HWPX인증줄은같은종료간격을소비한다. rowspan소유완료나꼬리blank예약을추측하지않고실제unit walker를유지한다. 새분기는미검증후보이며테스트·baseline을바꾸지않고기존원본컷/대조군부터재실행한다.

- #1133 e5pinned 두형식 전쪽 freshWASM은 공간재시도 exit0/265.294초다. HWP96.19594/98.92603/99.86388%, HWPX96.21383/98.92668/99.86388%로Native모든값과동일하고쪽수3씩이다(wasm-new-failed-originals-issue1133-v59-retry2-space-retry1/). 새WASM3쪽canonical review를직접표시했다. 이후v60 source는별도후보이므로이전e5결과를그후보의최종시각통과로승격하지않는다.
- 최초v60 Nativebuild는 exit0/276.773초, binary59b4731ad45a88689f33f02fe05a76ef5718bd9a25a6743a2f713bc41027b934다. 원본빠른덤프는Main52/#1133각3/#6935=3/#6803=5/교육과정414/위임전결105쪽이다. Main47→48은여전히row99통째이월이며출력복구를선언하지않는다. 원본row19에는single-row cell 선언이없어raw row높이하한0이들어있다. 기존prefix의0배제는row99가이미첫Commonframe을지났다는충분한하한도지워버린다. 독립선언prefix하한166511HU가첫frame24829HU보다이미크므로row19의모르는양수물리높이는이판정을뒤집지못한다. retry1은0하한을정확한opening컷인증에는쓰지않으면서알려진앞공간의하한에는보존한다. merged-onlyrow의0을실제빈공간이나내용완료로추측하지않는다. 수정후다시Nativebuild/원본컷/경계검증부터진행한다. 새formal검사·기대값변경없다.

- e5pinned Main38–40 freshWASM도 공간재시도 exit0/471.918초로두형식52쪽/선택3쪽씩 누락0이다. HWP99.75942/99.97808/100.00000%, HWPX99.80002/99.97060/99.93943%로Native와모두동일하다. #1133전쪽과합한12페이지 parity/최저90%를native-wasm-original-score-parity-v59-retry2.json으로대조했고Main39freshWASMcanonical review를직접표시했다. 후반미달이나미검증초기18개전체범위를통과로바꾸지않는다.

- retry1 Native build는 exit0/264.055초이며 binary SHA는 b6ad53abf25dbbdbc1babfc6f00ab124946c9b5524c2d18f1a4bfe4fe3f9df28이다. 소스 3개 해시를 source-code-v60-retry1에 고정했다. 원본 빠른 dump에서 두 형식은 52쪽이며, p47 pi59 59..100/endCut[1] → p48 99..145/startCut[1], p49 145..191, p50 191..237, p51 pi59 237..238/pi62 0..40, p52 pi62 40..54이다. 독립 Print의 row99 두 줄 소유 경계를 복구했다. #1133은 각각3쪽, #6935=3/#6803=5/교육과정414/위임전결105쪽이다(pages-original-countercheck-v60-retry1.json). 이는 쪽수·내용 컷 관측이며 시각90%나 정식 회귀 통과 증거와 구분한다. 현 후보의47–52쪽 두 형식 Native review와 기존73검사를 병행 실행 중이며 Cargo는 하나로 제한했다. 기대값/fixture/baseline 변경은 없다.

- retry1 기존73개는 exit100/365.046초/실행21.514초로69PASS/4FAIL/2767skip이다. #6024쪽수검사는106→105로다시PASS지만독립Print103쪽과전쪽90%는여전히미충족이므로출력완료로보고하지않는다. 기존#6981종료소유/#7226쪽수2개외에#6761의rewind_without_ladder_continuation_is_not_split_evidence/later_confirmed_rewind_does_not_authorize_an_earlier_cut이새로실패했다. 실제원본뒤사다리를일부러깨뜨린독립계약반례이며기대값변경으로없애지않는다. 실패테스트의기준원본samples/issue6782/1480000-201900042-chemical-product-labeling-study.hwp(SHA398d03a5d5e4d6e857086be532d6d9ed0cec9c8ad06f95c17bbb7f83056ae860)과독립PDF103쪽(pdffile명2020/실제CreatorHwp2022 0.0.0.0, SHA f8e5c0408e221080ede9a9a67b153d02d792d22961c738e46749641f32a32e79)을확인했고관련39–40쪽Native원본출력부터비교중이다. 수동변형의정답을원본PDF로대신하지않으며계약과원문피델리티를구분한다.
- retry2는선택한0→0쌍뒤의저장줄이있다면그재시작의LH+LS전진과일치하는지생산단계에서확인한다. terminal두줄은뒤사다리가없으므로그검사와구분한다. 원본row99두줄은그terminal경계다. 행의orphan수용은완료된형제하나가다른미완료셀을입증하지못하므로모든미완료ordinaryowner가실제선택컷의같은source경계를입증해야한다. cross-paragraph기존계약은유지하고in-paragraph0원점인증을공유한다. 더뒤의다른유효후보로현재선택컷을인증하지않는다. source/frame요구를정정하며문서ID·px허용치·추측span종료를추가하지않고검사·baseline을유지한다.
- retry1 Main47–52두형식Native review는 inner exit1/478.510초이며외부session143과구분한다. HWP99.81596/99.98427/100.00000/99.84891/80.03252/100.00000%다. HWPX의전쪽선택값도증적에보존했고미달은p51=82.38518%다. 48쪽canonical review를표시해첫‘뮴)’줄, 삭제표밴드와뒤래커도료의소유·괘선·간격복구를직접대조했다. 51쪽canonical review에서는pi62첫프레임의원점/누적괘선차이가남아있음을읽었다. 같은원본hostLS7396HU=98.613333px와현재flow97.28px의차이로savedfirstframe의인증이배제되는지실제측정/배치소비를추적한다. 허용치를넓혀인증하거나clamp로맞추지않는다. 최종쪽52의단독100%로51의미달을면제하지않는다.

- #6761 기준 원본 Native39–40쪽은 exit1/48.573초이며103쪽으로독립PDF와쪽수는같다. 관련쪽점수92.48720/71.53246%로40쪽미달이다(native-new-failed-originals-issue6761-v60-retry1/). 새40쪽canonical review를직접표시해이어받기표의반복머리행·글줄·후속문단의간격차이를읽었다. 정상원본검사PASS를90%피델리티로바꾸지않으며,수동사다리반례검사나px기대값을수정하지않는다. 같은원본freshWASM/원문40쪽출력보정은남아있다.
- retry2 Nativebuild는exit0/256.772초이며binarySHAaeaa58c73a1e52dda4fe55a7cd13026f8f21f886d0432bf5d90571b5b3c39067이다. source-code-v60-retry2/소스3개해시로고정했다. 빠른원본컷에서Main52쪽/row99 endCut[1]→startCut[1]을유지했다. 현후보의같은Main47–52쪽Native와기존73개를재실행해새반례FAIL이사라지는지확인중이다. 이전retry1시각개선은최신후보의검증으로재사용하지않는다.

- retry2의 기존73개 재검사는 exit100/358.857초/실행19.969초로71PASS/기존2FAIL/2767skip이다. 새#6761반례2개는검사·기대값변경없이PASS복구됐다. #6024가105쪽으로PASS지만독립Print103/전쪽90%는아직미충족이며검사PASS를원문시각승인으로보고하지않는다. 교육과정은415쪽으로독립Print와같지만기존#7226의413쪽핀과#6981종료쪽소유검사는실패한다. 같은원본의전쪽Native/freshWASM90%및독립소유대조가끝나기전까지그검사·baseline을바꾸지않는다.
- retry2 Main47–52 Native 재검증은진행중이며HWP6쪽의점수는retry1과같다. HWPX도완료되는값을해당최신run에보존한다. 현후보소스/검토기록을로컬체크포인트로고정하고같은정확한코드의freshWASM을검증한다. Main51/#6761원본40/나머지초기18개미달및최종lint·전체nextest·편집API·원격CI/merge·후속처리는남아있다. Source3개해시와컴파일snapshot을commit에대조해귀속시키며생성JS와진단/로그는stage하지않는다.

- retry2의Main47–52두형식Native최종review는exit1/477.744초이다. HWP99.81596/99.98427/100.00000/99.84891/80.03252/100.00000%, HWPX99.79842/100.00000/99.69468/99.57791/82.38518/99.99428%다. 두형식각6쪽중미달은51쪽하나이며52쪽전체소유를유지한다. source-commit-stored-later-zero-origin-frame-v60-retry2.json에서컴파일소스3개와commit eb29a7a9ce171c2cc5dfd9ff4689eb7a0d64eaae의바이트해시가모두같음을확인했다. 같은정확한head의freshWASM wrapper는공유target에서순차실행중이다. 독립Print51쪽괘선좌표는independent-print-p51-row-bands-v60-retry2.json/SVG로보존했고추가근거없이flow/paint를clamp하거나px상수로맞추지않는다.


### 진행 42 — 반복 머리행 조각의 실제 물리 하단과 수용 원장 대조

- eb29a7a9c fresh WASM은 exit0/574.324초, 소스16개해시불변이다. WASM SHA053c420ce12fad9ae5d05309099fd0416a8807f40587ad82f1a4b0a5d5fd2492 및 JS7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a가루트pkg/Studio public과일치한다. actual Studio 새브라우저/goto+reload/캐시비활성으로서빙해시·52쪽·글꼴loaded·canvas/오류0을확인했다(browser-studio-stored-later-zero-origin-frame-v60-retry2.json,oktrue). 이는독립Print시각승인과구분한다. 같은pin의Main47–52와#1133전쪽/#6761원본39–40Native/freshWASM비교를시작했다.
- Main51잔여원인을최종paint까지추적했다. 이전50쪽pi59 191..237의예약partial_height1005.3867px를accepted_row_boxes에기록하지만layout/table_partial.rs의최종page_cap_fold가1.3333px를접어실제상자는1004.0533px다. 이물리차이를선언33483HU의row218..237종료span에서이미사용한공간으로잘못차감해51쪽종료행이25.7067px로축소된다. 원본뒤LS3876HU/5476HU/7396HU와현재pi62flow97.28px가1.3333px어긋나첫Common frame인증도배제된다. native-physical-fold-before-v61.log 및 native-print-p50-row-band-drift-v61.json으로관측을보존했다. Print A4축척의일반누적차이와이별도수용/paint차이를구분한다.
- 보정후보는기존single_cell_page_fragment_bottom 생산결과를원본반복머리행의continuation행예산에서도소비한다. source/body하단→기존물리page cap→prepare_table_fragment_budget 행예산→scan의실제마지막밴드→record_stored_fragment_rows 원장→stored_fragment_row_plan 종료span잔여→PageItem/MeasuredTable→table_partial 최종높이순서다. footnote/zone의더작은가용은유지하고새100HU/px상수나허용치완화를추가하지않는다. 첫조각/편집/reflow/반복머리행없는경로는기존계약을유지한다. 끝유닛의내용컷과선언빈밴드는분리하며앞반례#6761두개/기존73개/원본전체쪽수와47–52의새Native로먼저대조한다. 아직미검증후보이며검사·기대값·baseline은변경하지않는다.

- 최초v61후보Nativebuild exit0/267.375초/binary3b0176019b23284f5d6baa50d180c3fba26d211e78ce64f4f263b5e84db52cee다. 빠른원본검사는Main HWP54/HWPX53으로독립52쪽과충돌했다. #1133각3/#6935=3/#6803=5/#6024=105/교육과정415쪽은유지됐다(pages-original-countercheck-v61.json). 이를보정완료로보고하지않는다. 실제logical advance가선택할내용컷예산에서physical inset을먼저빼는가정이잘못됐으므로제거한다. 해당무효후보의nextest컴파일은원본쪽수회귀가확인된뒤자체소유cargo-nextest/cargo/rustc만SIGTERM했고최종exit-15/107.021초이며test summary없다(cancel-reason.json). 출력공백때문에중단한것이나testFAIL이아니다.
- retry1은기존scanner의내용컷선택/요구높이를유지하고refit후내용완전소비행의빈tail예약에서만같은physical cap을쓴다. logical줄전진과실제빈밴드를분리하며원래높이를수용한뒤paint에서다시확장하지않는다. 행의최소요구보다작게압축하지않고기존컷·declared source·dirty/reflow인증은유지한다. 정확한physical마지막행override가수용원장과paint로함께전달되는지앞반례/새원본컷/시각검증부터재실행한다. 최초후보의page수/시각을retry1근거로재사용하지않는다.
- #6761원본40의eb29Native/freshWASM은둘다71.53246%다(39쪽92.48720%). 40쪽SVG에‘돋움’이Noto Sans KR ExtraLight(SHA67b4003e2be99ea44a7c957e4b35acde1b5e6e82a54fe195c5598c3c617bc2e3)로대체되어있고독립PDF에는Dotum이실제로embed되어있음을확인했다. win10-ted의Windows설치gulim.ttc를읽기전용복사하여독립face이름/해시를verified-ms-font-supply-v61/provenance.json에기록했다. 해당새경로를명시한출력으로글꼴공급차이를따로대조하며위치결함이나90%gate를면제하지않는다.

- eb29의20개대응페이지Native/freshWASM점수는모두동일하다(native-wasm-original-score-parity-v60-retry2.json). Main47–52각6쪽의freshWASM최종exit1/645.276초이며미달은51쪽두형식뿐이다. #1133각3쪽은Native exit0/170.867초,freshWASM exit0/253.546초로최저96.19594%다. #6761두쪽Native exit1/50.765초,freshWASM exit1/102.934초이며미달40쪽71.53246%다. 새WASM48쪽canonical review99.98427%를직접표시해상단‘뮴)’의소유와삭제밴드/뒤본문·괘선을대조했다. 후속v61후보의완료나초기18개전체통과로승격하지않는다.

- Windows gulim.ttc그대로인font supply는돋움export에반영되지않아기존점수그대로였다. TTCollection의Dotum원본face를독립sfnt TTF로추출한명시경로는실제SVG에Dotum(embedded SHA1c08ea8777fe0f775c4139d7ff4f68886d7f5608c52dd4581a22bf19175417cf)을embed했다. 공급파일SHAd090c4cb35ec18af0938ba0f0cf2c4d92e210a61c309eb533aaa122720ffb2a5와embed의glyf/hmtx/hhea/maxp/OS2동일·rawcmap차이및bestUnicodeCmap비교는issue6761-dotum-embedded-font-proof-v61.json에기록했다. 해당eb29 Native재비교 exit1/65.175초의39–40쪽점수는[(39, 92.43318), (40, 72.36214)]%다. 글꼴공급만으로해결됐다고보고하지않으며실제표원점/행/후속문단차이와freshWASM최종검증은남는다. TTC원본·face추출절차/해시는standalone-faces-provenance.json에보존했다.

- retry1 Native build exit0/254.956초/binarycf1536031259e270be069cf7b44fddcdbc3769f1a4270cf84a10fdf2248e2896이며소스2개해시불변이다. 두원본52쪽/47→48 row99부분컷및51 pi59 237..238/pi62 0..40을유지했다(pages-main-countercheck-v61-retry1.json). 직접RenderTree/드리프트에서pi59종료행25.7067→27.04px와pi62 typesetflow97.28→98.6133px,원본Common888.84px의source_frame인증복구를확인했다(native-physical-fold-after-v61-retry1/ 및log). 하지만실제pi62paint는y190.8/높이879.0px이며독립Print192.427/887.833과차이가남는다. 기존첫blank-band 수용의override없음조건/별도saved_opening_frame 생산조건과후속흐름의outerbottom소비까지추적중이다. source_frame인증복구만으로51쪽출력완료를선언하지않으며추가원점/높이소비보정과재검증이필요하다. 현재기존73개재검사/47–52새Native는진행중이다.

- retry1 HWP51쪽Native canonical review는80.03252→95.67443%로개선됐고직접표시했다. 하단괘선/빈밴드차이가남아95%점수로완료를면제하지않는다. HWP47/48/49/50/52는99.81596/99.98427/100/99.84815/100%다. HWPX전체선택과기존73개최종summary는진행중이며현코드freshWASM은아직없다.
- 최신fetch에서upstream/devel=6f66932a73ee6fd05a74a83bdc438c9499a89a12를확인했다(upstream-base-advance-v61.json). 초기대상#7612가83131ecd1으로별도병합됐고후속기록이upstream에들어왔다. 이는이번스레드의remote merge로보고하지않는다. 현재컴파일/원본sweep의기준5168e3256은유지해해당결과를정확하게완결하고체크포인트커밋후새base로rebase한다. 새base소스의최종gate는별도로재실행한다.

- retry1 최초 nextest handle81936은 외부exit143으로 컴파일 중 종료됐고 test summary가 없다. 재감사에서 Cargo/nextest/rustc 실행0을 확인한 뒤 남아 있던 running metadata를 terminal/environment_interrupted/innerexit 미상으로 정정하고 interrupted.json을 보존했다. 소스2개 해시 불변을 확인해 공유cache에서 동일73개 명령을 rerun1 재실행한다. 제품FAIL로 집계하지 않으며 실행 중인 작업을 중복 재시작하지 않았다.

- GitHub최신조회는open non-draft23/draft6,새PR0/head·draft변경0이며이전24개중#7612만닫혔다(github-open-pr-snapshot-v61.json). #7612 API state=MERGED/mergeCommit=83131ecd186bd7483671b7886d659ecb4fd380ca를확인했다(github-pr7612-merged-v61.json). 초기18개의기여자적용이력은유지하며아직열린초기17개검토와유보6개후속순서는바꾸지않는다.

- retry1 기존73개의동일command warm-cache 재실행은최종exit100/126.460초/실행20.661초로71PASS/기존2FAIL/2767skip이다(nextest-stored-header-physical-page-cap-v61-retry1-rerun1-exit.json 및log). #6761두negative계약도유지됐고새FAIL없다. 남은#6981종료소유/#7226쪽수는원본전체Native/freshWASM90%선행조건미완료로검사·기대값·baseline을변경하지않았다. Native47–52전체선택review의최종값을완결한뒤같은소스해시의로컬체크포인트와최신upstream rebase를고정한다.

- retry1 Native47–52두형식review는최종exit0/498.070초이다. HWP99.81596/99.98427/100/99.84815/95.67443/100%, HWPX99.79842/100/99.97037/99.85027/96.23354/99.99428%다. 두형식선택12쪽누락0/각전체52쪽이며새HWPX51 canonical review도직접표시했다. 90%통과와남은하단괘선·첫source빈밴드/뒤흐름차이를별도로기록한다. 현후보의freshWASM/전수TSV·최종lint/full nextest와다른초기PR미달은아직미완료다. 현단계2개Rust파일/통합기록을체크포인트커밋하며generated Studio JS/출력로그/진단산출물은포함하지않는다.

- 체크포인트89b1d00c8b202b267b27c43b5e84c9e8474cb4c8을6f66932a7위로rebase한head는b0678b4631a8627b32cc3f12464b824f8c7d0106다. pre/post85커밋·42개cherry-pick원출처가같은순서로유지됐다. src/crates/Cargo/build/tests/scripts의pre/post전체Gitdiff는0이며range-diff의삭제/추가2쌍은이미upstream에병합된#7612두기여패치가빈커밋으로보존된결과다(rebase-v61-range-diff.txt 및rebase-v61-20261009.json). Studio JS는원래SHA7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a로복원됐고stash4dce188fad06e67ecddf11de5c86ec8992725a9c도보존했다. 새base의manifest/unit-tier정책검사를준비하며51쪽직접차이/#6761원본40쪽과나머지미달보정을이어간다. 새head최종gate/원격통합은아직완료하지않았다.


### 진행43 — 인증된 첫 물리 프레임과 마지막 후속 흐름의 소비 연결

- 새base6f66932a7의prepare/manifest --check 및unit-tier --check는PASS(1458sources/6264attrs/48targets,4113units/297modules)다. 현재head b0678b463에서추가소비보정을시작한다.
- 독립원본pi62 Common66663HU=888.84px/hostLS7396HU와Print192.427..1080.260px프레임이근거다. 기존stored_partial_fragment_height가그첫상자를이미인증하고행예산888.84를반환하지만scan의내용완전소비prefix0..40은878.7733px다. 별도saved_multirow_opening_frame_height는intra-row endCut만수용해여기서는없고,emit이내용컷없는첫상자의10.0667px빈밴드를버린다. 또한원본반복머리행continuation의종료outerbottom은typeset이예약하지만확정placement없으면paint후속y에빠진다. 이전headerbody의원장이선택한실제행높이/후속물리흐름과다르게소비되는문제다.
- 기존producer의인증결과를FragmentBudget에명시적으로전달한다. refit완료후완전한행prefix이며내용minimum을늘리는unusedband만동일마지막행override/누적예약/수용원장/paint로보낸다. 내용컷을추가소비하거나현재원점을clamp하지않는다. 일반intra-row reset/중첩block컷의기존경로는유지한다. 첫개체프레임의확정tableTop과원본반복머리행continuation의tableTop/occupiedBottom은같은budget결과에서계산해기존ParagraphFloatPlacement로두소비지점에전달한다. 이미있는placement를재해석하거나배치뒤고정px로맞추지않는다.
- 현재별도후보이며기존73개의정상/negative계약·Main52쪽/47–52컷과두형식Native를먼저검증한다. 앞95%점수로남은괘선차이를면제하지않고새formal시험/fixture/baseline을추가·변경하지않는다.

- #6761원본40의직접vector대조에서Print 표상단136.010416px/Native132.28px,원본outer.top283HU=3.773333px를확인했다(issue6761-independent-row-origin-v62.json). table높이는Native599.7px/Print598.229168px로비슷하며전체frame원점차이가먼저다. 기존column_rowbreak_fragment_opens_outer_top의Para분기는object_only_saved_table_anchor와prior-rowspan없음을필요로하고,shared intra_paragraph_rowbreak_reopens_outer_top은host.textempty만허용한다. 실제원본은이전쪽에서소비한표제host가있고양쪽ordinaryowner의유효intra-para새프레임컷으로재개한다. 단지빈host가아니라는이유로개체의새세로프레임여백을잃는다. 기존의원본host-line유효성producer와현재선택된sourcecut을연결할후속보정을검토하며수동LS사다리반례/일반걸침rowspan에는새프레임을추측하지않는다. 현재v62Cargo가끝나기전Rust소스는수정하지않는다.

- v62 Native build exit0/257.102초/binary26c10fe2242f9e5e2eba288681e709a0c0e79da014f611a49397d82a23f9a182이며소스5개해시불변이다. 두원본52쪽/#1133각3/#6935=3/#6803=5쪽을먼저확인했고나머지빠른원본검사와두형식51쪽Native review를진행중이다. v62pin은후속원본호스트조건보정과구분해보존한다.
- #6761후속후보는현재선택된유효intra-para새프레임컷에원본호스트유효성을연결한다. host가빈단독Table일때의기존계약과,stored_head_host_lines_are_valid가인증한표제줄/한개의Table·notes만있는원본을같은세로프레임으로처리한다. 원본제어슬롯/줄기하가없거나선행글줄이개체와겹치는host는넓히지않는다. 실제source cut을가진ordinaryowner가새쪽에서재개한다는근거없이prior-rowspan전체에여백을추가하지않는다. 같은intra_paragraph_rowbreak_reopens_outer_top의결과를budget와paint가소비하고,그뒤resolved_table_top/확정placement의덮어쓰기도대조한다. 원본39–40의새Native/정확한Dotum공급및73개의사다리반례부터재검증한다.

- v62 빠른 원본8개은Main각52/HWP1133=3/HWPX1133=3/#6935=3/#6803=5/#6024=105/교육과정415쪽으로모두유지됐다(pages-original-countercheck-v62.json). 두형식51쪽Native review는exit0/83.911초로HWP99.91930/HWPX99.88665%다. 새HWP51 canonical review를직접표시해‘아.기타’제목과다음표원점/하단·마지막행중앙정렬이복구된것을대조했다. Native RenderTree의후속blank/title원점은127.3/148.6px,다음표는192.7px/높이889.1px이며독립Print의축척과같은영역에서읽었다. 아직현소스freshWASM및73개최종/전쪽TSV승인근거는없다. v62의전체52쪽Native TSV와v63의원본host확장후보빌드를진행한다.


- v63 Native build는 exit0/267.567초, binary `afb026b65e78d933658c8292a253978dfcd4dd2fa5ae8b23ed2e02fecc2b742e`이며 8개 source hash 불변이다(`native-build-stored-title-host-source-frame-v63.json`). 정확한 Dotum 공급에서 #6761 원본39–40 새 Native sweep은 exit0/55.172초, 92.43318/96.48485%다. 40쪽은 글꼴만 바로잡은 이전72.36214%에서 개선됐고, tabletop136.1px/독립Print136.010416px 및 뒤 본문을 canonical `native-issue6761-stored-title-host-source-frame-v63/issue6761-original/review/review_040.png`로 직접 표시·대조했다. 39쪽도 직접 표시했으나 첫 조각의 하단 괘선 차이가 남아 점수로 완료를 면제하지 않는다. 현source fresh WASM은 미실행이다.
- v63 기존73개 집중 nextest는 최종 exit100/385.524초/실행22.496초로71PASS/기존2FAIL/2767skip이다(`nextest-stored-title-host-source-frame-v63-exit.json`, 대응log). 실패는 #6981 `completed_terminal_cut_does_not_allocate_an_empty_page`와 #7226 `the_fix_does_not_add_a_page`이며 새 실패는 없다. #6761의 수동 사다리 negative 두 계약은 유지됐다. 원본전체 Native/fresh WASM90% 선행 검증 전 검사·기대값·baseline을 변경하지 않았다. fmt check도 통과했다.
- 같은v63 pin의 빠른 원본9개는 Main HWP/HWPX52쪽, #1133 HWP/HWPX3쪽, #6935=3/#6803=5/#6024=105/교육과정415/#6761=103쪽이다(`pages-original-countercheck-v63.json`). #6024의 Print103과 actual105는 미해결이며 기존검사105PASS를 fidelity 승인으로 사용하지 않는다. Main47–52 두형식과 #1133 전쪽 새Native 경계검사는 진행중이다. 이전v62의Main 전쪽 TSV도 별도pin으로 진행하며 현v63 결과로 바꾸어 쓰지 않는다.
- 최신GitHub read-only snapshot은 non-draft23/draft6이며 v61 대비 새PR/닫힘/head·draft변경 모두0이다(`github-open-pr-snapshot-v63.json`, `-delta.json`). 최초18개 적용이력/현재열린17개와 유보6개 후속검토 순서를 유지한다. 이 스레드의 원격 push/PR/merge/close는 미수행이다.
- #6761 39쪽 직접 판독 후 `native-issue6761-p39-drift-v63.log`에서 첫 상자 `source_frame=185.28`, 예산185.3, 실제내용174.7, row3종료컷과 40쪽 `start_cut=[1,1]`를 확인했다. 원본Common13896HU=185.28px의 빈 하단 밴드가 보존되지 않는다. 기존 `saved_multirow_opening_frame_height`의 원본 내용컷·선언상자 인증과 `first_fragment_blank_band`의 object-only/빈host 선택을 대조한다. visible-title host에도 새 in-paragraph stored frame 컷과 실제 원본host line 인증이 있으면 같은 빈 밴드 소유를 적용하고, 마지막행 override→수용원장→예약→paint가 함께 소비하도록 후속 보정한다. 유닛 소유는 바꾸지 않으며, 원본39–40와 기존negative/정상 경계를 다시 검증한다. 현재8개 source의 체크포인트 후 다음 단계에서 구현한다.


### 진행44 — 표제 호스트의 첫 저장 상자 원점과 빈 밴드

- 체크포인트 `8f3c1e0db94f38195b192a216817a3ea6e8a54c9`에 v63의8개Rust source와 통합기록을 고정했다. 컴파일한8개 SHA와 commit blob의 SHA가 모두 같다(`v63-build-to-commit-proof.json`). generated Studio JS는 포함하지 않았다. 이 커밋의 focused71PASS/기존2FAIL 및 Native 선택scope는 최종수용 판정이 아니다.
- #6761 원본39쪽 독립Print vector는 table top836.682292/bottom1021.760416px, 높이185.078124px다(`issue6761-independent-first-frame-v63.json`). 원본Common13896HU=185.28px와 대응하며 v63 실제174.9px의 하단 차이를 입증한다. Native 첫 원점833.8px에도 차이가 있다. TABLE_SPLIT_AVAIL의 동일 예약 원점은 cur702.7+outer.top3.773333+vert_offset_overhead−1.106667px인데 확정placement가 없어 paint가 outer.top을 잃는다.
- 기존 `saved_multirow_opening_frame_height`가 인증한 첫 상자와 `row_cut_starts_intra_paragraph_stored_frame`의 실제 새프레임 컷을 연결한다. 호스트는 `stored_rowbreak_outer_frame_host`의 원본제어슬롯/유효LineSeg 근거로 수용한다. 별도empty-text 추측은 제거한다. 기존 `first_fragment_blank_band`는 독립선언/원본컷/요구높이·가용예산을 그대로 검사하며, 인증된 완전prefix 또는 내용컷있는 첫상자 모두 같은 tableTop/occupiedBottom placement를 생산한다. 실제 내용유닛을 늘리지 않고 마지막행 물리override/원장/예약/paint/뒤흐름으로 전달한다. 기존placement가 있으면 보존한다.
- v64후보는 `fragment/emit.rs`만 후속변경하며 fmt check 통과 뒤 같은target/pr-review Native build 중이다. v63의기존검사·페이지수·시각을 새후보 통과로 재사용하지 않는다. 원본39–40/73개사다리negative와 Main/#1133/기타원본 경계를 새pin으로 재실행하고 fresh WASM을 이어간다. 검사·fixture·baseline 변경은 없다.

- 최초v64 Native buildはexit0/269.637초/binarye3d878a747addcb2f084ce06bde109777e8189884b00cb21ecdf9242d3b1a42e다. 빠른9개원본쪽수는v63와같고39쪽height185.5px로복구됐다. 다만top833.8px가남는다(`native-issue6761-p39-drift-v64/`,log). 예산에이미prepared.host_placement가있어emit fallback origin은미사용이며, `block/prepare.rs`의`unconstrained_host_placement → for_first_fragment`가originalouter.top을빼는실제생산지점을확인했다. 기존margin복구는body-filling/column-anchor만소비해valid원본head-title Para anchor를놓친다. v64 retry1은그생산지점에서기존`stored_head_host_lines_are_valid`로인증된원본RowBreak title anchor의outertop을보존한다. dirty/reflow session/합성줄은확장하지않는다. 행예산은같은준비placement를읽고paint는ctx의확정tableTop을읽으며뒤에`resolved_table_top`이선택되는것까지대조한다. 단순origin clamp나문서전용수치는추가하지않는다. 검사와freshWASM은retry1소스에서처음실행한다.

- v63 pin의두Native 경계작업은최종exit0으로종료됐다(`native-original-boundaries-v63-exit.json`): Main47–52두형식12쪽은최저99.79842%, 단계456.333초; #1133전쪽두형식6쪽은최저96.19594%, 단계160.815초다. source_unchanged=false는작업시작후v64소스를편집한관측값이며, 작업은불변v63실행파일만사용했다. 재검사한binary SHA와8f3c1e0db commit의8개source hash가build metadata와같다. 해당결과를v64후보에귀속하지않는다.
- v62별도pin의Main HWP전체52쪽TSV는누락0/최저93.25821%(10쪽)/90%미만0이다. HWPX전쪽작업은진행중이며v63/v64전체scope의최종승인근거로재사용하지않는다.
- 후속freshWASM/전쪽검증의공간확보를위해실행중인작업이없고summary가있는v49/v50의종료된4개검토폴더에서동일파일명·size·SHA의SVG296개만hardlink공유했다. 경로/bytes/증적을모두보존했고physical8,777,682,944bytes를확보했다(`space-recovery-identical-closed-svg-v64.jsonl`, 최종232.105초). 첫안전검사의self-shell오탐으로mutation전멈춘뒤actualscript argv를검사하도록고쳐재실행했으며다른경로/cache/source는정리하지않았다.

- v62 전쪽원본작업은timeout1800초에서최종exit124로종료됐다. HWP52쪽TSV는완결됐으나HWPX는38쪽raster만있고나머지는미측정이다. 종료handle/실제process종료를확인한뒤동일v62불변pin의39–52나머지14쪽만별도tail TSV로진행한다(`native-stored-partial-frame-physical-extent-tail-v62-exit.json`). 시간제한종료를제품FAIL로집계하거나미완료scope를90%승인으로표시하지않는다. 현v64 retry1 전쪽은별도3600초의새출력으로처음부터104쪽을실행한다.
- v64 retry1 Native build는최종exit0/263.203초/9개sourcehash불변,binary `4428c9fdbd4e24f5f3b59386f612aedf0572f3128fe1f8ac9158d214b8f4bf3c`다. 빠른9개원본쪽수는이전과같고#6761원본39쪽bbox는top837.6/height185.5px로독립Print836.682292/185.078124와같은영역에서대조한다. 생산된준비placement의outer.top보존이최종resolved_table_top까지전달됐음을확인했다(`native-issue6761-p39-drift-v64-retry1/`). 아직새PNG직접판독/점수와73개최종결과는미완료다. 새39–40Native,Main전체104쪽Native TSV(두형식각52쪽),같은73개nextest를실행중이며freshWASM은Cargo순차게이트이후다.

- v64 retry1 #6761원본39–40 Native sweep는최종exit0/55.513초/9개source hash불변이며39쪽98.94265/40쪽97.45412%다. canonical 두review를직접표시해첫표하단과마지막행/뒤쪽반복머리행·이어받는‘함유혼합물’줄소유/괘선·후속본문을같은영역에서대조했다. 40쪽bbox는136.1/599.7px로유지되면서첫행글줄의원점도개선됐다. `native-issue6761-stored-title-host-opening-frame-v64-retry1/` 및exit/log에보존했다. freshWASM은아직없으므로최종승인아니다.
- 사용자가처음지적한Main3쪽도현v64 retry1로새캡처했다: HWP99.89467/HWPX99.84971%. HWPcanonical `native-stored-title-host-opening-frame-p3-review-v64-retry1/scattered-hwp/review/review_003.png`를직접표시해‘인장시험’/첫표전체행·다음표/뒤문단과Print의원점을대조했다. 이전v41의56.53%이미지를현결과로재사용하지않았다. 나머지Main전쪽TSV/#1133전쪽은진행중이다.
- 동일v64 retry1 기존73개nextest는최종exit100/406.611초/실행22.003초,71PASS/기존2FAIL/2767skip이고9개소스해시불변이다(`nextest-stored-title-host-opening-frame-v64-retry1-exit.json`,log). #6761negative두계약포함새FAIL없다. 남은#6981종료소유/#7226쪽수의원본교육과정415쪽전수Native/freshWASM90%선행증거는아직없어검사·기대값·baseline을변경하지않았다. 현재두Rust파일과통합기록을체크포인트커밋하고rootfreshWASM으로다음출력경로를검증한다.

- 체크포인트 `fe201fa05f25ee744fb465a8b8a7348d38576d18`의9개소스SHA가v64 retry1 Native build의9개snapshot SHA와모두같다(`v64-retry1-build-to-commit-proof.json`). 이head로rootwrapper의freshWASM web/no-opt진단빌드를시작했다(`wasm-build-stored-title-host-opening-frame-v64-retry1.json`). 최적화배포산출물검증은아직없으며Native/freshWASM최종시각·Studio확인은별도남는다. 현재Rust/test소스는동결하고공유target의Cargo는한개만실행한다.
- v64 retry1 Main3쪽두형식Native작업과#1133전쪽두형식Native작업은모두최종exit0/107.573초및223.774초로종료됐고9개소스해시불변이다(`native-original-boundaries-v64-retry1-exit.json`). #1133의6쪽최저96.19594%/누락0이며앞v63결과와같다. 검사변경은없다.
- v62같은불변pin의HWPX tail39–52는exit0/302.271초다. 기존1–38/새39–52의PNG52쌍을전부Pillow.verify하고중복PDF39–52bytes도같음을확인해별도완결scope에hardlink보존했다(`native-stored-partial-frame-physical-extent-completed-v62/provenance.json`). 첫결합에서Poppler숫자padding을세자리로추정해찾기실패한자체오류는파일명숫자로쪽을대응시키고재실행해수정했다. Native/PDFbytes를변경하지않았다. 전체52쪽TSV/90%미만0을같은역사source의증거로기록하며새v64전체head승인으로대신하지않는다.

- fe201fa05의rootfreshWASM web/no-opt빌드는최종exit0/599.703초/9개source hash불변이다(`wasm-build-stored-title-host-opening-frame-v64-retry1.json`). pin은`wasm-pkg-stored-title-host-opening-frame-v64-retry1-fe201fa05`,WASM SHA`5da9a953cb1094c6f02a36038c7eee461ebddc3512ae7c3117d016457e00b044`,JS SHA`7462392b9b4dadb36c0b60cc0afa9de3fc6e41a861b150010ea51f0a7974737a`이며pkg/public이같다. 새WASM이publicJS의추가6개runtime import를실제로요구함을WebAssembly.Module.imports로확인했고node syntax도통과했다(`studio-js-required-wasm-imports-v64-retry1.json`). 필수binding이므로generated publicJS를검증기록과함께커밋한다. 아직실제Studioreload/시각비교와최적화배포산출물검증은없다.
- 현v64 retry1 Main HWP전체52쪽Native TSV는누락0/최저93.25821%(10쪽)/90%미만0으로종료됐다. HWPX전체는진행중이다. 새10쪽canonical NativeHWP93.25821/HWPX94.98450%도직접대조했으나하단괘선과마지막행글줄에남는차이가보인다. 90%점수로해결완료를선언하지않고Git동기화후그물리밴드소비경로보정을이어간다.
- 실패#6981의`build_page_render_tree(377)`/‘노동인권’가정은정본PDF의페이지375–381텍스트에서노동인권이380–381쪽에있는것과충돌한다. Native같은구간도노동인권380–381쪽이다(`education-terminal-page-pin-independent-text-v64-retry1.json`). 관련‘선언문작성’경계는별도시각대조가필요하다. 텍스트추출을fidelity승인으로대체하지않으며원본415쪽전수Native/freshWASM90%선행검증전테스트·기대값·baseline을수정하지않는다.
- 사용자의커밋→upstream/devel동기화→rebase→계속지시에따라새fetch를완료했고upstream/devel은기존6f66932a7 그대로다. local devel은local-only0/behind65,현재review는upstream-only0/local87이다. freshWASM이종료됐으며원본전체Native는고정실행파일에서계속된다. 소스checkout없이localdevelref를fast-forward fetch로갱신하고현재review의rebase를실행해각SHA/clean여부를기록한다.

### 진행45 — 완전한 행 내용 컷과 첫 표 상자의 물리 공간

- 요청한 Git 단계는 `9b658ed6563a9c51c39ea170d0df117fb485d5eb` 커밋 뒤 완료했다. local devel/upstream/devel 모두 `6f66932a73ee6fd05a74a83bdc438c9499a89a12`, review rebase는 already up to date이고 작업 트리는 clean이었다(`rebase-v64-20261009.json`). 원본18개/유보6개 검토 순서와 적용이력은 유지한다.
- v64 retry1 Main 전쪽 Native는 exit0/2061.454초/소스9개해시불변으로 끝났다. 두형식 각52쪽/누락0/미달0이며 최저 HWP93.25821/HWPX94.98450%(각10쪽)다. 실제10쪽의 하단 괘선 차이가 남아 최종승인으로 쓰지 않는다(`continuation-state-v65-after-sync.json`).
- 수정 전 실제 원본 HWP의 pi12 Common38046HU=507.28px와 독립Print10쪽 top573.614584/bottom1080.260416px는 같은 첫 물리 상자를 입증한다. Native top574.2/height497.1px, scanner 예약489.3px다. 첫6행의 실제 측정합345.28px 뒤 row6은 블록 컷 `[1,3,3,8,1,0,0,0,0,0,0,0]`로 남지만 현재 행의 모든 소유 유닛을 소비하고 다음 행 소유는0이다. 수동 입력이 아닌 원본 dump/vector/렌더 트리에서 관측했다(`main-p10-original-table12-v65.dump.txt`, `main-p10-independent-physical-frame-v65.json`, `native-main-p10-physical-frame-before-v65/`). 일반 plain-text reset은 없으므로 기존 reset 계약을 완화하지 않는다.
- 후보는 authenticated `stored_partial_frame_height`의 완전 prefix 판정을 컷 표현이 아니라 실제 블록 셀 순서/유닛 완전소비/후속소유0으로 확인한다. 이전 행들이 실제 paint와 같은 measured frame을 쓰는지 기존 `whole_fragment_row_uses_measured_height`/`row_block_cut_uses_measured_height`로 확인하고 같은 행 계획의 prefix를 뺀 마지막 물리 높이를 생산한다. scanner의143.99px 내용 높이에서 measured151.55px를 빼는 추정은 쓰지 않는다. 생산결과 → emit 예약/마지막행 override → PageItem/MeasuredTable → table_partial의 정확한 마지막행 높이와 resolved_table_top → 뒤흐름을 추적한다. 기존 내용 컷·이어받기 소유는 유지하며 빈 밴드가 다음 내용 유닛을 소비하지 않는다. 편집/reflow/유닛 일부소비/미인증 프레임은 비적용이다.
- 작은 원본9개 쪽수/73개 기존 계약/Main10–11쪽과 #6761 경계의 새 Native를 먼저 실행한다. fresh WASM/영향 전쪽90%와 직접 판독 전 테스트·fixture·baseline은 변경하지 않는다. 아직 후보 계획이며 보정 완료/원격 제출은 아니다.

- 최초v65 Native build exit0/251.684초/binary `bd3bf0c02adebf92b47532b60e81b59caf4af1689ac4464e0b8ac9706a628c2c`, 빠른 원본9개 쪽수 불변이다. Main10–11 Native4쪽은 exit0/141.566초: HWP97.63165/98.07847,HWPX97.62819/98.54269%다. #6761 39–40은 exit0/53.855초/98.94265/97.45412% 유지, 기존73개 nextest는 exit100/349.685초/실행19.857초/71PASS/기존2FAIL/2767skip이며 source10개 hash불변이다. 모든 작업 최종종료를 확인했다. 새로운 검사·기대값 변경은 없다.
- canonical Main10–11 review를 직접 대조했고10쪽에는 여전히 작은 하단 차이가 남는다. 예약/기본RenderTree는507.28px이나 실제Print SVG의 마지막 괘선은1076.28px다. 실제 SVG 경로의 `DIAG_FOLD pi=12 rows=0..7 trail=8.00 slack=5.23 fold=5.23`가 선언 빈 밴드를 뒤 줄간격으로 다시 접는 소비지점임을 입증했다(`native-main-p10-complete-block-frame-svg-drift-v65-command.json`, 대응log). 기본 트리 좌표를 실제 SVG 판독 대신 사용하지 않는다.
- retry1은 같은 원본 블록의 완전유닛/미소비 후속소유/원본 프레임 인증 query를 예약과 실제 paint가 함께 소비한다. 이미 생산한 첫 resolved origin/마지막행 물리override와 프레임 높이가 대응할 때 원본 가운데 정렬 공간을 보존하고 줄간격 fold에서 제외한다. 일반 부분 유닛·편집/reflow/미인증 상자·좁은 가용 예산에는 적용하지 않는다. 기존 source-frame producer와 내용 컷은 유지하며 임의 clamp나 문서별 숫자를 추가하지 않는다. retry1 소스의 새 Native/기존계약/freshWASM을 다시 검증한다.
- 종료된 v62/v64 전쪽 SVG의 파일명·size·SHA가 같은102개를 hardlink 공유해3,919,171,584bytes를 확보했다(`space-recovery-identical-full-main-svg-v65.jsonl`). 경로/내용은 보존하고 실행중인 출력·공용target은 변경하지 않았다.

- retry1 Native build는 exit0/255.751초/binary `57be27721b8e343a1e4042b05f1ee2ba55c61edc2a2f24883af055c5546b7310`/소스11개해시불변이다. 지침 갱신 직후 최초 작은 Native 작업은 tool handle이 사라지고 실제 대응 process도 없는 것을 확인했다. 미완결 artifact는 interrupted/exit미상으로 보존했으며 제품FAIL/PASS로 집계하지 않았다. 동일pin의 별도 rerun1은 최종종료까지 확인했다.
- Main10–11 Native rerun1은 exit0/145.552초: HWP99.95503/98.07847,HWPX99.80029/98.54269%다. 새canonical10쪽 두형식을 직접 표시해 마지막행 글줄/가운데 정렬/하단 괘선을 Print와 대조했다. 실제SVG 하단은 v64의1071.053333 → 최초v65의1076.28 → retry1의1081.506667px다. 후자는 원본Common507.28px와 같은 물리 상자이며 독립Print의A4 축척 하단1080.260416px와 대응한다(`main-p10-svg-physical-frame-after-v65-retry1.json`). 고정px 기대값은 추가하지 않았다.
- #6761 원본39–40은 exit0/56.377초/98.94265/97.45412% 유지,39쪽 새review도 직접 표시했다. 원본9개 쪽수는 모두 이전과 같다(`pages-original-countercheck-stored-complete-block-frame-v65-retry1-rerun1.json`). 기존73개 nextest는 공용 절대target `/home/tsjang/rhwp/target/pr-review`에서 exit100/375.705초/실행22.166초/71PASS/기존2FAIL/2767skip, 소스11개해시불변이다. 검사·기대값·fixture·baseline 변경은 없다.
- 현재Main 전체104쪽 Native TSV는 같은 불변pin으로 새출력 중이다(`native-main-complete-block-frame-full-v65-retry1-exit.json`). 별도 원본 #6803의5쪽은98.94637/93.43892/75.15690/32.30497/35.53280%, #6935의3쪽은85.79861/54.92011/94.06013%로 기존미달을 유지한다(`native-remaining-rowspan-originals-v65-retry1/`). 해당범위를 이번Main 보정으로 해결했다고 보고하지 않는다. 초기18개의 남은미달/교육과정 전쪽선행검증/최종lint·전체nextest·freshWASM·Studio/API·원격CI/merge·후속처리는 계속 남는다.
- 이4개Rust source와 통합기록을 로컬 체크포인트로 고정한다. 컴파일snapshot의11개 SHA를 commit blob과 대조하며, 같은코드의rootfreshWASM/실제Studioreload/Native·WASM 전쪽 비교를 이어간다. 현재대표PNG 통과는 전체scope 승인이나 원격제출이 아니다. 새지침에따라 이후Cargo/WASM은 기본workspace의 공용 절대target을 명시한다.

- `337671d13a70a48196fcb0e612412490be4b9613` 커밋 이후 최신 fetch/rebase를 다시 확인했다. local devel/upstream/devel은 `6f66932a73ee6fd05a74a83bdc438c9499a89a12`, HEAD 변화 없음/clean이다. Native104쪽은 최종 exit0/2018.050초/소스11개 해시불변이며 HWP 최저97.15676%(30쪽), HWPX 최저95.41221%(7쪽), 각52쪽/누락0/미달0이다. fresh WASM build exit0/599.444초, WASM SHA `e21a5e772506f9982aba41272d8163b36aa4a5a0e58c388ba332e01b726e6856` 및 pkg/public 일치/실제 Studio reload ok를 확인했다. Native/fresh WASM 대응6쪽 점수도 동일하다(`native-wasm-current-boundary-score-parity-v65-retry1.json`). full WASM104쪽은 아직 실행하지 않았다.
- 현재 최소값30쪽 canonical review를 직접 판독해 `·재질변화시마다`의 쪽 소유 차이를 발견했다. Print30쪽에는0개/31쪽에는1개, Native30쪽에는1개/31쪽에는0개다. 같은 r168c1의 `분` 글줄도 다음쪽 소유를 침범한다. 점수가97%여도 해결 완료로 판정하지 않는다(`native-main-current-minimum-review-v65-retry1/review/review_030.png`, `main-print-text-p30-31-v65.txt`, `main-native-text-p30-v65.json`, `main-native-text-p31-v65.json`).

### 진행46 — 걸친 셀의 저장 프레임과 내용 소유 경계

- 원본 pi33 r168c9의 저장 줄은 vpos0/1500/3000/0, r168c1은0/1500/3000/0/1500/3000이다. 실제 source 재시작과 독립 Print30–31쪽 소유가 근거이며 수동 합성 메타데이터가 아니다. 현재 straddle 경로는 물리 행 prefix를 `cell_units_fitting_height`에 전달하지만 저장 프레임 경계가 아닌 전체 높이만으로 소유를 결정한다. 일반적인 opt-in 유닛/수용 컷 진단으로 실제 높이·reset flag·원본 줄 연결을 확인하고 생산→예약→paint의 공통 소유 경계를 보정한다. 고정px, 문서ID, 임의 clamp 또는 첫 reset에서 무조건 멈추는 조건은 사용하지 않는다.
- #6803 원본 r8c4에는 같은 문단의 vpos2000→0 경계가 있고 Print2쪽의 마지막 글줄이 Native3쪽으로 밀린다. 실제 측정 앞8행891.786667px와 source first frame936.04px, 마지막줄 점유와 간격을 구분해 별도 원인을 확인한다(`issue6803-source-reset-cut-diagnosis-v65.json`). 현재 전쪽 미달이므로 기존 테스트/기대값/fixture/baseline은 유지한다.

- opt-in 진단 빌드는 exit0/280.157초/소스12개해시불변, binary `803069469954d85b390cd82d08ebf702b857b1c8c3a05e31e0a9831e67ded16d`다. 실제 r168c1/r168c9의 첫3개 unit은 각20px, 네 번째는20/13.333px이며 hard_break=true/stored_frame_break=false다. 원본1566HU 세행과 달리 수용된 물리 높이는78.693333px여서 기존 컷이0..4/다음쪽4..끝을 고른다. #6803 r8c4는26.666667px 단위이며 원본 intra-para 재시작은 unit2이다. 로그와 source snapshot을 보존했다(`native-main-source-unit-diagnostics-v66-command.json`, 대응log, `native-issue6803-source-unit-diagnostics-v66-command.json`).
- Main 후보는 원본 행 prefix가 바로 source 첫 재시작 cut을 선택하고, 같은 시작행의 독립적인 두 병합 셀이 같은 행 경계를 인증할 때 그 source owner를 사용한다. 첫줄0부터 저장 LH/LS대로 전진하다 양의 좌표에서0으로 돌아오는 원본만 대상이며, 원본 행이0/유효하지 않거나 단독 reset/편집·reflow/부분행컷·override는 기존 계약을 유지한다. 이 source cut은 물리 행 높이/빈밴드를 축소하지 않는다. `stored_span_row_boundary_owner → rowbreak_straddle_cut_units → straddle_continuation_demand 및 table_partial`의 동일 start/end owner로 전달한다. 기존 수용 원장의 물리 공간과 뒤흐름은 유지하며 `분`/마지막 빈도 줄의 소유만 바로잡는 후보를 먼저 실제 출력으로 검증한다.

- 최초 후보의 fmt/build는 Rust2021 미지원 let-chain으로 실패했다(build exit101/68.233초). 에이전트의 문법 오류로 보존하고 Option 사전 계산으로 수정해 재실행한다. 제품FAIL 재현이나 검증PASS로 집계하지 않는다.

- retry1 Native build exit0/255.763초/소스13개해시불변, binary `d35a60ab7d126deb5753197035ab30a33dec65cb6130ffac0df5ca44bb232530`이다. 기존 소유진단의 다음쪽4..끝이 실제3..끝으로 바뀌었고 앞쪽0..3과 이어진다. 원본9개 쪽수는 모두 유지됐다(`pages-original-countercheck-shared-source-span-owner-v66-retry1.json`). 새 Native30–31 두형식 sweep은 exit0/143.438초/소스불변이며 HWP30–31=97.83234/100%, HWPX30–31=98.21598/99.97839%다. HWP두쪽/HWPX30의 canonical review를 직접 표시·판독했다. `재질변화시마다`는 양쪽의 실제 트리에서30쪽0개/31쪽1개이며 원본Print의 소유와 일치한다.
- #6761 원본39–40의 같은 소스 Native 대조는 exit0/89.187초/98.94265/97.45412%로 이전과 같다. 정확한 base6f66932a7의 manifest/unit-tier 정책 검사도 PASS(1458source/6264attrs/48targets,4113unit/297modules)다. 기존73개 nextest와 Main10–11은 아직 실행 중이다. 새code의 전체Native/freshWASM/최종lint와 초기18개의 다른 미달 보정은 남으며, 이 경계 통과를 전체승인으로 바꾸지 않는다.

- 기존73개 nextest는 최종 exit100/381.285초/실행21.041초로71PASS/기존2FAIL/2767skip이며 소스13개 해시불변이다. #6761 두 negative 계약도 PASS이고 새FAIL은 없다. Main10–11 두형식 새Native도 exit0/210.070초이며 이전 점수를 유지했다(HWP99.95503/98.07847,HWPX99.80029/98.54269%). 모든 Cargo/관련 sweep의 terminal 결과를 확인했다. 교육과정의 기존2FAIL은 원본415쪽 전체 Native/freshWASM90% 선행검증 미완료로 검사·기대값·baseline을 변경하지 않았다.
- 현3개 Rust source와 통합기록을 로컬 체크포인트로 고정한다. opt-in 유닛 진단도 포함해 원본 줄→유닛→실제 start/end 소유를 재현할 수 있다. 새회귀/fixture 추가는 아직없으며 현재Native 경계 결과는 새code의 전쪽/freshWASM 승인과 구분한다. 이후 #6803의 첫 물리 프레임을 셀 좌표로 정규화하는 producer 누락을 보정한다.


### 진행47 — 첫 표 상자를 분할 셀 좌표로 정규화

- `17832b15e5c242333f7656f8c28227b3914d4779` 커밋에 저장 행의 source owner 보정을 고정했다. 컴파일한13개 source SHA와 commit blob이 전부 같다(`v66-retry1-build-to-commit-proof.json`). 최신fetch/local devel동기화/rebase 후 base6f66932a7/HEAD불변/clean을 확인했다(`rebase-v66-retry1-20261009.json`).
- #6803 원본8행 이전의 원시 행 합은66884HU=891.786667px이며 실제 수용된 측정값과 같다. 원본 첫 Common 상자는70203HU=936.04px다. 기존 `native_intra_para_saved_reset_trailing_trim`는 ‘첫 상자에 앞선 여러 행이 있을 수 있다’고 설명하지만 마지막 조건에서 셀 내부57.093333px만 전체936.04px와 비교한다. 앞8행을 빠뜨려 source unit2 컷의 마지막 간격 보정을0으로 만들고 Print2쪽 소유의 글줄을3쪽으로 밀어낸다. 고정allowance나 clamp가 아닌 원본 프레임의 셀좌표 정규화 누락이다.
- producer는 편집/reflow되지 않은 원본 plain-text 셀과 유효 원시 행 prefix가 첫 선언 상자를 가로지르는 경우 그 상자를 해당 셀 원점 기준으로 정규화한다. 같은 결과를 기존 컷 수용/가시 높이/paint의 마지막 간격 계산이 소비한다. 셀 시작전 상자·불명확한 행 높이·합성/dirty source는 새 경로로 인증하지 않는다. 첫 소유2개 유닛의 끝은53.333333px+패딩3.76px이며 셀 상자는44.253333px라서 마지막 줄간격 중12.84px를 다음 프레임으로 보낸다. 원본이 남긴0.493333px 밴드는 보존한다.
- 작은 원본9개 쪽수/73개 정상·negative 계약과 #6803 전체5쪽의 새Native를 먼저 확인한다. Native/freshWASM 전쪽90% 선행조건 전 기존검사·기대값·fixture·baseline은 변경하지 않는다. Main17832의 새로운 전체Native/freshWASM 및 전체scope 최종gate도 아직 남는다.

- v67 최초Native build exit0/260.094초/source15개불변/binary `96c76028fc58456145fe3dc52a3d1249df02ceb7c0faa9dde8117cab4880fef4`다. 원본9개 쪽수는 유지됐다. #6803 첫 endCut이1→2, 첫 수용936.04px로 Print2쪽의 마지막 글줄이 복구됐다. 새8쪽Native sweep은 최종exit1/170.137초다. #6803=98.94637/97.63115/55.40141/45.02854/35.53280%, #6935=85.79861/54.92011/94.06013%이며 새2–3 canonical review를 직접 표시했다.2쪽만 개선됐고 전체는 미충족이다.
- 기존73개 nextest는 exit100/351.165초/실행18.614초/source불변/70PASS/3FAIL/2767skip이다. 새FAIL `#6981 giant_straddle_cell_that_must_be_cut_is_not_grown`의 원본은 교육과정이 아니라 같은 #6803이다. `A9~10, `의 셀 밖 글줄1개를 검출했다. 실제 원본전체 sweep의 미달과 연결하고 검사·기대값은 유지한다. 기존 교육과정2FAIL도 그대로다.
- Print3쪽 PDFium 경로에서 반복머리행 이후 row8 하단648.87px, Native 실제SVG592.826667px를 확인했다(`issue6803-p3-independent-grid-v67.json`, `issue6803-p3-native-svg-grid-v67.json`). 첫 부분행이 소비한44.253333px 뒤 원본/측정행569.186667px의 물리 잔여524.933333px를 이어받아야 한다. 현재 scanner/paint는 남은 글줄의470.4px만 이어받아 빈밴드54.533333px와 가운데 정렬 공간을 잃는다. 기존 `next_start_row_height_override`의 저장 행 잔여 생산은 first_fragment_blank_band가 있을 때만 열려, 이미Common전체를 채운 이번 첫컷에서는 닫힌다. 내용끝과 물리공간을 같은 상태로 다룬 결함이다.
- retry1은 원본 첫 프레임 컷과 앞 행의 실제 측정 prefix가 대응할 때 그 행의 남은 물리공간을 continuation cursor의 정확한 startRowHeightOverride로 전달한다. 원본FirstCommon→선택sourcecut/이미수용된첫상자→원래측정행잔여→다음scanner예약/rowledger→실제cellheight/완료된내용의가운데정렬이 같은 값으로 이어진다. 부분내용이 다시 잘리는 경우 가운데정렬은 켜지 않는다. 임의 숫자로 행을 늘리거나 clip을 숨기지 않으며 새원본/기존회귀를 재실행한다.

- 회귀 적격성도 출력 회귀와 함께 심사한다. 새 #6981 실패의 원본 #6803은 이미 전쪽Native에서90%미만이므로 현재 정식 기준 문서로 적격하다고 보고하지 않는다. 검사 자체는 고정위치가 아닌 글줄의 셀 포함 관계를 확인하고, 새후보에서 실제 `A9~10, ` 침범을 검출했으므로 출력에서 고친다. 교육과정의 고정쪽377/쪽수413 검사는 Print의380–381 소유/전체415쪽과 충돌하는 독립 증거가 있다. 두 검사의 원본전체 Native/freshWASM 적격성을 먼저 충족한 뒤 소속·순서·보존 관계로 전환한다. 미달 원본의 기대값을 현재 출력에 맞춰 갱신하거나 삭제해서 통과시키지 않는다.

- 회귀 검사 조건의 추가 감사는 `regression-criteria-audit-v67-retry1.json`에 source hash/위치/독립 근거를 고정했다. #6981 all-page helper는 render 오류를 건너뛰며, ‘늘리지 않는다’는 검사는 포함 관계만 확인하고 #6795 반례는 탈출 건수3만 고정해 대상/크기 악화를 입증하지 못한다. #6803은 조각 안 `dedup`으로 중복 증거를 지우고, 연속 prefix만 확인해 원본 끝까지의 보존을 입증하지 못하며, ‘용지 밖 없음’ 검사에40px 탈출을 허용한다. #6935의1958자 총량은 글자별 보존/쪽 소유 증거가 아니다. 이는 코드 검토상 검사 부족이며 실행 FAIL과 구분한다. 기존 #7226의413쪽/#6981의고정377index/#6024의105쪽은 독립Print415/380–381/103과 충돌한다. 관련 원본 Native/freshWASM 전체90% 전 기대값·검사·fixture/baseline을 변경하지 않고 출력부터 보정한다.
- v67 retry1 Native build는 exit0/256.419초/source15개불변, binary `215c54b74b0135f2365ee3f8a31471087814d0703d4308464a9a3f8b7b40b588`다. 원본9개 쪽수는 모두 유지됐다. 같은 pin의 원본8쪽 Native sweep은 exit1/171.351초/source불변이며 #6803=98.94637/97.63115/91.62217/52.85687/31.86659%, #6935=85.79861/54.92011/94.06013%다. 새3–4 canonical review를 직접 표시·판독했다. 첫 행의 물리 잔여를 scanner/paint가 보존해3쪽은 개선됐지만 왼쪽 병합 셀의 source 첫 빈문단 컷/뒤쪽 작은intra-para reset과 쪽 소유 차이가 남는다. 전체 미충족이며 새code freshWASM 미실행이다.
- v67 retry1 기존73개 nextest는 최종 exit100/358.583초/실행20.295초/source15개불변/70PASS/3FAIL/2767skip이다. #6803 탈출 줄은 이전후보의`A9~10, `에서`형태 및 외관`으로 바뀌었고 아직 셀 포함을 충족하지 않는다. 나머지는 교육과정415≠고정413/고정후속쪽의`노동인권` 부재다. 실행 FAIL·기준값 충돌·코드상 약한 검사 조건을 각각 분리하고 미달 문서의 PASS를 적격성으로 쓰지 않는다. 체크포인트에 현재 보정과 실패/미검증을 함께 고정하고, 소스·기대값 해시를 연결한다.

### 진행48 — 잘못된 회귀 검사 의미의 교정

- 사용자 추가 지시 “회귀테스트가 잘못되었으면 회귀테스트를 수정해야함”에 따라 독립적으로 확인된 기준 오류와 검사 누락을 현재 branch에서 교정한다. 기존의 시각 선행 조건 때문에 오류를 계속 고정하지 않고, 검사 교정과 문서의 정식 적격성을 분리한다. Native/freshWASM 전쪽90% 미충족 문서는 계속 수용 보류이며, 검사 PASS를 시각 승인으로 쓰지 않는다.
- 직전 source 보정은 `733f723d604d1782f0a10de2f1e551bbf8c59564`에 고정했다. Native 컴파일15개 source hash가 commit blob과 모두 같다(`v67-retry1-build-to-commit-proof.json`). 최신fetch/local devel동기화/rebase 후 base6f66932a7/HEAD불변/clean을 확인했다(`rebase-v67-retry1-20261009.json`).
- 같은 원본의 tracked Print PDF를 다시 읽어 교육과정415쪽(SHA82fe8eef...)과 위임전결103쪽(SHA90eda86b...)을 확인했다. 위임전결 원본 SHAde43607f...의 두 라벨은 PDF/현Native 전체에서 각각1회다(`education-independent-page-count-v68.json`, `issue6024-independent-test-expectation-audit-v67-retry1.json`). 잘못된413/105 기준을415/103으로 교정하고, 고정쪽 기반 라벨 검사 대신 전쪽 본문의 두 전체 라벨 보존을 검사한다. 따라서 남은105쪽 실제 출력은 새103 기대값에서 FAIL이어야 한다.
- #6981 후속쪽은 원본의 유일한 최종표와 다음 구역의 유일한 제목 표 소속으로 찾아 쪽 순서/빈쪽/누락·중복을 검사한다. 고정377index를 제거하고 all-page helper의 render 오류를 실패로 처리한다. 거대 셀 검사의 이름·주장을 실제 셀 포함 검사와 일치시켰다. #7226 대상 닻의 유일성도 확인한다. #6803은 원본 표 소속을 선택해 전쪽 paint 순서의67문단을 검사하고, 조각 내부 dedup/선택쪽/마지막 prefix 누락을 제거하며 용지 포함 관계를 일반 정밀도0.5px로 검사한다. 정상 출력 고정좌표·SVG hash·fixture/golden 변경은 없다.
- 최초 v68 focused 실행은 파생suite의 새 배정 뒤 이전 target번호 제한을 재사용해46개만 포함됐다(exit100/82.982초/실행8.507초/45PASS/#6024의105≠103 FAIL). 이 결과를73개 전체 검증으로 쓰지 않는다. 마지막 fmt 후 파생suite를 다시 준비하고 번호 제한을 제거한 동일semantic filter로 재실행한다. 정확한 base의 manifest 검사는1458source/6264attrs/48targets로 PASS이며 최초 generated drift 실패 로그도 보존했다. generated suite/manifest는 커밋하지 않는다.
- v68 retry1의73개 재실행은 최종exit100/417.832초/실행22.137초/source19개불변/71PASS/2FAIL/10309skip이다. 수정한 교육과정의415쪽/원본소속 후속제목은 PASS, #6803 전쪽67문단·순서/용지 포함 강화도 PASS다. 실제FAIL은#6803의`형태 및 외관` 셀 침범과#6024의105≠103쪽으로 남는다. 결과를 통과시키기 위해 기준을 되돌리지 않는다. 후속제목 검사에는 마지막으로 해당원본제목 노드의 실제글자 보존과 같은쪽 중복 검출을 추가하고 그함수만 다시 검증한다. 다른72개와 production15개 source는 이 후속변경의 영향을 받지 않는다.
- 후속제목 노드의 실제`노동인권 교육` 글자 보존/같은쪽 중복까지 강화한 최종함수는 별도exit0/24.380초/실행6.452초/1PASS/source19개불변으로 확인했다(`nextest-regression-terminal-owner-v68-retry2-exit.json`). 새배정의suite010을 파일에서 찾아 실행했고 정확한base manifest도 PASS다. #7226의전쪽닻조회에서도render 오류를 건너뛰지 않도록 교정해 해당module3개를 마지막으로 재실행한다. production renderer는 이 검사 교정 단계에서 변경하지 않았다.
- #7226 마지막module 검증은 exit0/21.867초/실행6.720초/3PASS/source19개불변이다(`nextest-regression-page-count-owner-v68-retry3-exit.json`). 정확한base manifest도 PASS다. 73개 재실행 뒤 수정한 local 함수1개와 render오류 처리의 영향module3개를 모두 재검증했고 검증후 source 변경은 없다. 기존2FAIL은 실제#6803 셀 침범/#6024 전체쪽수 차이로 기록하며 원본90% 적격성/초기18개 최종lint·전체Rust·freshWASM·원격CI/merge·후속 처리는 남는다. 이번4개 기존검사의 의미 교정을 별도일반커밋으로 고정하고 source hash/명령/결과를 연결한다.
- 최종 fmt check와 git diff check는 PASS다(`logs/fmt-regression-criteria-correction-v68.log`). 지침 파일 변경은 없다.

### 진행49 — 빈 저장 줄의 첫 병합 셀 소유

- 검사교정은 `20fa595d664214af377c843dfbc92bad4c4ac73b`에 고정했고 검증snapshot19개 SHA/commit blob이 모두 같다(`v68-validation-snapshot-to-commit-proof.json`). 최신fetch/local devel동기화/rebase 후 base6f66932a7/HEAD불변/clean이다(`rebase-v68-20261009.json`).
- #6803 r8c0/r8c1은 원본빈문단의vpos0/2000/0, ordinary r8c4는 같은첫구간의vpos0/2000/0을 소유한다. Common936.04−실제앞8행891.786667=44.253333px이고 첫2개의빈줄 잉크+패딩은43.76px로 이상자에 들어간다. 현재 `cell_units_fitting_height`의빈spacer tail 제외로 병합owner는 첫조각0..1/다음1..끝, ordinaryowner는0..2/다음2..끝을 고른다. Print3쪽A1/A9 라벨과 Native의약22px 차이는 다음조각의소유가한빈줄늦는실제출력과연결된다(`native-issue6803-span-source-cut-v67-retry1/`, 대응log 및canonical3쪽). 빈줄은 저장 line box와 줄전진을 갖는 원본이므로 가시글자가없다는 이유로소유를바꾸지않는다.
- 첫 Common의셀원점정규화를 plain-text 병합셀에도공유하고, 원본첫lader/양수→0재시작/선언상자의잉크와마지막간격/독립형제owner/실제측정prefix·ordinarycut의동일상자를함께확인해첫sourceowner를생산한다. `rowbreak_straddle_cut_units`의start/end가동일query를소비하고예약의`straddle_continuation_demand`와실제paint가같은범위를쓴다. 물리행과빈band·내용유닛을분리하고일반emptyspacer의높이·컷조건을전역으로완화하지않는다. 다른첫상자/수동LS·편집·reflow/불명확rawrow·커진prefix는현재계약을유지한다. 원본5쪽의새Native와정상/negative 기존검사에서먼저확인하며새fixture·golden은추가하지않는다.
- v69 Native build는 exit0/261.615초/source19개불변, binary `bdaf9b7c91f6a1f40e65ce545fd976d3267f1a0a6d9f2ebc29edcd561d15e3f1`다. 원본9개의쪽수는유지됐다. 실제diagnostic의병합owner는첫0..2/다음2..34로같은source경계를소비한다(`native-issue6803-shared-empty-source-frame-owner-v69/`, 대응log). 원본8쪽sweep은최종exit1/171.153초/source불변이며#6803=98.94637/97.63115/93.32701/52.85687/31.86659%, #6935=85.79861/54.92011/94.06013%다. 새3쪽canonical review를직접표시·판독했다. sourceowner는복구됐으나마지막글줄의3→4쪽이월과뒤4–5쪽소유/표grid차이는미해결이며현code freshWASM은미실행이다.
- 28개현재파생target의동일semantic filter에서기존73개nextest는exit100/441.583초/실행20.222초/source19개불변/71PASS/기존2FAIL/6053skip이다. #6803 셀침범과#6024의105≠103을유지하며새FAIL은없다. 재작업전원본/기대값/fixture는변경하지않았다. SourceCommon의첫병합owner와물리구간을checkpoint로고정한뒤, 이후ordinarycell의작은reset과독립적인걸침셀의큰reset을같은원본프레임으로인증하는후속query를검토한다.

### 진행50 — 원인 분석과 수정·검증을 묶는 작업 방식

- 사용자 지시에 따라 개별 보정마다 Native 빌드→73개 회귀→커밋을 반복하는 방식을 바꾼다. v69의 Native 빌드261.615초와 집중 검증441.583초 중 실제 테스트 실행은20.222초였다. 같은 조판 경계의 수정 범위와 반례를 먼저 확정하고 한 묶음으로 구현한 뒤 Native/fresh WASM 빌드와 영향 범위 검증을 진행한다. 작은 경계의 직접 판독 뒤 필수 전체 검증을 진행하는 순서는 유지한다.
- 분석 기준은 clean HEAD `1071f044e1e267f66b2ce573d3015c31dbbdbb32`와 v69 pinned binary `bdaf9b7c91f6a1f40e65ce545fd976d3267f1a0a6d9f2ebc29edcd561d15e3f1`다. 이 실행파일로 #6803/#6935/#6024 전체 render tree와 #6024 전체 텍스트를 다시 확보했다. 네 명령은 각각 exit0/0.317·0.166·2.676·1.772초/source19개불변이며 `batch-analysis-issue*-v70-command.json`, 대응 `-exit.json`/`logs/`에 보존한다. 분석 중 production 코드 변경·재빌드는 없다.

| 대상 | 원본·독립 Print·현재 실행에서 확인한 차이 | 함께 추적할 실제 소비 경로 |
| --- | --- | --- |
| #6803 | ordinary r10c4의 원본 reset2 대신 cut1, r13c4의 reset6 대신 cut5. 병합 r8c0/c1은2→34→67/68을 소비해 후속 큰 source reset36/70과 어긋난다. p3/p4의 host overhead0과 독립 Print의 표 위 원점 차이도 있다. | `stored_intra_paragraph_frame.rs`의 source owner → `table_layout.rs:15084`의 마지막 간격과 ordinary cut → `emit.rs:1046`의 실제 행 잔여 carry → `table_layout.rs:20360/20536`의 예약·paint owner → `table_layout.rs:18138`의 최종 위 원점. |
| #6935 | 첫 컷[5,9] 때문에 독립 Print의 첫쪽 마지막 줄이 다음쪽으로 이동한다. 다음 block의 scanner cut460.8은 잔여458.9를 넘고 실제 paint927.9는 본문918.5를 넘는다. 원본 첫 행74.4133과 실제 측정121.68의 차이는 약47.27이며 raw prefix를 실제 좌표로 대용할 수 없다. | 첫 행 측정 → ordinary 마지막 줄·source Common 인증 → block cut의 실제 요구 높이 → 누적 예약·예산 실패시 컷 → paint. Square wrap이므로 #6803의 TopAndBottom 조건 완화만으로 해결됐다고 주장하지 않는다. |
| #6024 첫 초과쪽 | 전쪽 내용 대응에서 Native34쪽은 para70 r167의 `일반사항` 한 행으로 따로 생긴다. Print33쪽은 같은 행까지 포함한다. 실제 scanner는 마지막 행43.7467/잔여38.0/내용28.5·완전소비를 관측하고 이월한다. | `height_measurer.rs:3936`의 병합 선언 제약 → cut/whole-row 높이 → `scan/runner/row_step.rs`의 요구 높이·마지막 밴드 수용 → `emit.rs:64/109`의 완성 조각 extent → 최종 행 상자·후속쪽. |
| #6024 두 번째 초과쪽 | Native79쪽의 para132 표는 Print76쪽에 있다. Print77쪽은 para131 표 전체를 별도 배치한다. 원본 para131의31행 모두 header 속성이며 Native는 첫22행을 먼저 출력한 뒤 다음쪽에0..30을 다시 출력한다. 원본 floating 표와 뒤 TAC 표의 쪽 소유·중복을 함께 보정해야 한다. | 원본 header coverage/host LS·floating anchor → `leading_header_rows`/fragment budget의 반복 범위 → 첫 수용·이월과 뒤 TAC의 쪽 소유 → 실제0..30 행 출력 횟수. |

- #6024의103개 독립 Print와105개 최신 Native 전쪽을 whitespace/page-number 정규화5-gram으로 대응시켰다(`batch-analysis-issue6024-page-ownership-v70.json`). 내용 기반 대응은 진단이며 시각 일치율이 아니다. 34쪽/79쪽의 차이는 원본 para70/130/131/132 덤프, 최신 render tree, Print33/76/77쪽 직접 판독으로 교차 확인했다. 기존 overflow 로그의53/55쪽만 고치면 초과2쪽을 해결한다는 가정은 폐기한다.
- 독립 PDF의 벡터 괘선 좌표는 `batch-analysis-independent-grid-v70.json`에 보존했다. 저장 cellSz·raw 행합·실제 측정·수용한 물리 공간을 구분하고, 측정 증가분이 잘못됐다는 결론은 그 값이 생긴 규칙과 Print의 같은 경계를 대조한 뒤 정한다. source reset과 shared owner 인증·물리 잔여·원점은 같은 묶음에서 보정하고, 모든 줄 reset을 강제 쪽 나눔으로 바꾸지 않는다.
- 묶음 검증은 같은 소스 snapshot에서 원본9개 쪽수/내용 소유, 기존 집중73개와 정상·비적용 대조군, 영향 경계의 Native/fresh WASM review·overlay를 실행한다. 그 뒤 초기18개 전체 영향 범위의 전쪽90% 적격성 및 필수 lint·전체 Rust·fresh WASM/Studio 검증을 완료한다. 현재 두 기존 FAIL과90% 미달/미검증은 그대로 보류이며 초기18개 범위·후속6개 별도 검토·원격 CI/merge·후속 처리의 완료 조건을 줄이지 않는다.

### 진행51 — 공통 저장 프레임·전체 제목 표의 묶음 보정

- v70은 실제 측정 행의 원점 계획을 `LayoutEngine`에 공급하고, ordinary source 첫 컷과 겹치는 병합 셀의 독립 저장 프레임으로 작은 reset을 인증한다. 마지막 간격·유닛 소유·분할 행의 물리 잔여를 scanner/예약/paint가 같은 query에서 소비한다. 원본 빈 host가 재개하는 위 원점도 반영한다. 전체31행이 제목인 원본 표는 현재 쪽에 들어가는 일부를 먼저 출력·반복하지 않고 기존 deferred-frame 경로로 전체를 다음쪽에 배치하며 뒤 TAC는 원래 쪽에 남긴다. source ID/고정 쪽·픽셀 예외, 새 회귀 fixture/golden은 추가하지 않았다.
- v70 Native build는 exit0/259.632초/source23개불변, binary `7907a5bffc84bc71ed9a5d596be849c042bdddf2aa19d1ccebf32503212b6482`다. 원본9개 중 #6024만105→104쪽이고 나머지52/52/3/3/3/5/415/103은 유지됐다. 최신 #6024 para131은78쪽에31행의201개 셀을 각각1회 출력하며 이전의 첫22행 중복을 제거했다. 원본 para70 r167의34쪽은 남는다. `native-batch-issue*-v70/`, `pages-original-countercheck-shared-source-frame-batch-v70.json`, `shared-source-frame-batch-v70-result-audit.json`에 명령·source SHA·전체 관측을 보존한다.
- v70 Native 전8쪽 sweep은 exit1/162.338초/source불변이다. #6803=98.94637/97.63733/96.42149/94.76612/94.70697%로 전5쪽의최저94.70697%, #6935=90.11088/72.15378/94.06013%다. 두 문서의4쪽/2쪽 canonical review를 직접 표시·판독했다(`native-source-frame-batch-review-v70/`). #6803의 ordinary 컷2/6, 병합owner36/70과 셀 포함은 개선됐지만 현code freshWASM은미실행이다. #6935의 오른쪽 병합 셀이 다음source 첫줄까지 당긴 차이는 남아 재검토한다.
- 같은source23개의 기존73개 nextest는 exit100/449.327초/실행19.516초/72PASS/1FAIL/6053skip이다. #6803 셀침범 검사는 PASS로 바뀌고 #6024의104≠103만 FAIL이다. PASS를 원본 전체 Native/freshWASM 적격성으로 승격하지 않는다. 기준값·검사 파일을 바꿔 실패를 숨기지 않았다.
- 직접 대조에서 이번 구현의 두 오류도 확인했다. `split_end_limit`는 padding 없는 유닛 높이인데 이를 마지막 행의 상자 높이로 차감해 padding을 prefix와 마지막 상자에 이중 예약했다. 실제 상자 높이를 동일 query로 차감하고 독립 Print와 대응하는 기존 source-frame 하단 계약 안에서 물리 상자를 선택하도록 교정한다. 또 이월 제목표 뒤의 source0 제목 `25. 국민신문고과`가 표 아래에 수용돼 다음쪽 소유를 잃는다. source의 TAC 후 dense 빈줄→0 본문 경계에서 후속문단의 최소physical page를 명시적으로 전달한다. 표 높이를 body 높이로 늘리지 않는다.
- v71 묶음에는 위 두 자체 오류, #6935의 여러 완전 행을 가로지르는 첫병합owner, 원본 빈host의 독립source 경계 인증을 함께 반영한다. 첫Common이 ordinary 마지막 컷을 인증하고 병합 셀의 진짜 stored page reset이 같은 프레임을 소유할 때만 여분의 물리 빈밴드를 보존한다. 수정후 동일 전체 원본·경계·정상대조를 다시 확인하며 아직 lint·freshWASM/Studio·초기18개 전체시각·원격 통합 완료가 아니다.
- 전쪽 검증 공간을 위해 종료metadata가 있는 이전7개 실행의 SVG를 ignored `closed-svg-lossless-archive-v70/`에 무손실 분할 압축 보관한다. 반복base64를 content SHA로 공유하고 모든SVG의 정확한 바이트 재구성 SHA를 확인·recipe를 fsync한 뒤 원래SVG를 제거한다. review/overlay/compare PNG·PDF·TSV·원본과 Cargo 캐시는 보존한다. 복원은 `archive-closed-svg-v70.py --restore <recipe의상대SVG경로>`이며 종료 집계는 `summary.json`에 기록한다. 이 압축은 이전 시각 판정을 새head 검증으로 바꾸지 않는다.

### 진행52 — v72 묶음 보정의 독립 근거·소비 경로

- #6935 Print2쪽 벡터 괘선(`issue6935-p2-independent-all-grid-v72.json`)은 아래 표의 제목52.2627px·첫 내용행109.8px·다음82.4693px이다. Native의 첫 내용행109.9px 자체가 크게 짧다는 이전 해석을 정정한다. 실제 차이는 제목 r7의 content-cut48.5733와 measured52.3467 간3.7733px로, 원본2개 저장 줄·원래 안 여백을 선언 cellSz566HU로 줄이면서 생긴다. `stored_whole_row_padding_frame`으로 같은 원본·줄사다리·패딩 차이를 확인하고 prepare의 전체 행 예약과 partial의 block 밖 완전행 paint가 동일 measured 상자를 사용한다. 부분 컷·편집·중첩·불명 저장정보는 제외한다.
- 사용자가 지적한 하단 선은 Square RowBreak의 미완성 end_cut 조각에서 원본 cell bottom을 현재 조각 끝에 수집한 것이다. 실제 종료 조각의 괘선은 보존하고, 기존 Square continuation의 열린 윗변과 대응하는 내부 컷 하단도 원본 셀 경계로 그리지 않는다. 좌표 보정·내용 숨김으로 선을 없애는 처리가 아니다.
- #6024 Print33쪽 원본 마지막166/167행 괘선은949.201→987.08→1023.68px(`issue6024-p33-independent-tail-grid-v72.json`)이다. Native는166행을74.7px로 늘린 뒤167행을 다음쪽에 출력한다. source r167의38px·전체 내용28.4533은 현재 잔여38px에 들어가고, measured43.7467의5.7467 증가분은 내용이 아니라 r160 부모 선언 최소값이다. `stored_complete_terminal_row_frame`은 실제 마지막 행·원본38px 수용·모든 종료 부모가 현재 조각에서 시작·원본 저장 줄/개체 없음·모든 부모의 전체 내용이 각 실제 상자에 들어감을 확인한다. `row_step`에서 수용 전 동일 물리 높이를 선택하고 end override를 emit/paint에 전달한다. 중간행·이미 내용 컷을 소비한 부모·미수용 내용에는 적용하지 않는다. 허용치는 기존0.1px의HU 변환 경계이며 baseline/쪽수 기대값은 바꾸지 않는다.
- v71 결과: Native6803 전5쪽 최저95.40319%,6935 최저78.30120%(2쪽),6024 원본104쪽/Print103쪽. v70 기존73검사72PASS/1FAIL(6024); v71 코드의 테스트·fresh WASM 증거로 대용하지 않는다. v72는 위3개 수정의 같은 소스 묶음으로 빌드·원본9개·기존 집중검사·영향 review를 다시 실행한다.

- v72 Native 묶음 빌드exit0/256.607초/source31개 불변, binary `556f8ac5088546a24da33639f54e1b4794956276b2973f40a47c306168229298`(`native-build-shared-source-frame-batch-v72-exit.json`). 원본9개 쪽수는Main52/52,1133 3/3,6935 3,6803 5,교육415,6761 103 유지;6024는104→103으로 독립 Print와 일치한다. `issue6024-terminal-ownership-audit-v72.json`은 r167c7 `일반사항`이33쪽에1회, `25. 국민신문고과`가78쪽에1회, 사무분장조정/조사관교육훈련이전쪽1회임을 확인한다.
- v72 Native8쪽 sweep은exit0/170.684초/source불변(`native-source-frame-batch-review-v72-exit.json`),6803 전5쪽 최저95.40319%,6935 전3쪽 최저91.57542%,누락0/gate passed이다. [6935 2쪽 review](../../../output/pr-review/semanticist21-20261008/native-source-frame-batch-review-v72/issue6935/review/review_002.png)를 직접 열어 사용자가 지적한 하단닫힘선 제거와94.30407%를 확인했다. 3쪽 실제 종료 선·주석 내용은 유지된다.
- 6935 3쪽 직접 판독에서 빈왼쪽칸 상단의 짧은 선은 남는다. 이미 소진한r9/r10의0높이가 내부 괘선을 조각 원점으로 옮겨, 첫grid index0만 비우는 기존 열린윗변 처리를 우회한다(`source-frame-batch-v72-native-audit.json`). 최저값통과만으로 이 차이를 완료 처리하지 않으며 다음 보정 묶음에서 실제 경계 소유를 검증한다.
- v72 기존73개 nextest는exit0/471.677초/실행21.579초/source31개 불변,73PASS/6053skip이다(`nextest-shared-source-frame-batch-v72-exit.json`). 이전v70의6024 104≠103 FAIL은같은원본·변경없는기대103에서PASS로 바뀌었다.6024 전103쪽 Native TSV와fresh WASM은 실행중이고 원본전체의회귀적격성·최종lint·전체Rust·원격CI/merge는 미완료다. 새 fixture/golden/회귀 검사는 추가하지 않는다.
- 이전SVG 보관은terminal1976개·원래52,010,668,175바이트·압축34,198,613바이트·실제회수25,315,713,024바이트로 끝났다. 첫SVG를 원래경로에 실제 복원해 정확한SHA 일치까지 확인했다(`closed-svg-lossless-archive-v70/summary.json`, `restore-audit.json`). 원본·PDF·PNG·TSV·Cargo캐시는 보존했다.

### 진행53 — v72 전쪽 실행이 드러낸 미달·다음 묶음 범위

- #6024 Native 전103쪽 TSV는exit1/1269.923초/source31개 불변·누락0이며32쪽이90% 미만,최저35쪽38.84165%다(`native-source-terminal-frame-full-v72-exit.json`, `issue6024/silhouette.tsv`, `summary.json`). 103쪽 일치·73기존검사PASS를 문서 전체의 적격성으로 승격하지 않는다. 실패쪽은12,14,19,26,27,35,36,40,43,47,48,49,51,53,55,56,59,62,66,67,68,76,83,84,85,91,92,94,96,97,98,99다.
- 수정 경계33/76/77/78의 직접review는exit1/195.675초·76쪽74.01609%이며33쪽98.80354%다(`native-source-terminal-frame-review-v72/`). 33쪽 r167의 같은쪽수용은 확인했지만 위쪽r137c1 라벨도 차이가 있다. 독립Print32쪽은 첫문단 `위원회`만 소유하고33쪽은 다음문단 `정보보안 및 개인정보 보호`를 소유한다. 실제원본p0/p1의0→0 저장쪽리셋이CellUnit index1에hard/page_frame_reset=true로 존재하나 straddle의물리높이43.3733→내용cut0..2가 그 경계를 넘겨 다음문단첫줄까지32쪽에 소비한다(`native-batch-zero-slot-label-v72-readonly`, 원본para70덤프). 저장reset검출추가보다 실제straddle컷·예약·paint가이미검출된동일owner를소비하도록보정하는것이 다음원인경로다.
- #6935 2쪽하단선은 수정전1→수정후0,3쪽실제종료선은보존임을 같은원본RenderTree관계로확인했다(`issue6935-fragment-edge-ownership-audit-v72.json`).3쪽의0높이행에서원점으로이동한상단선은별도미해결이다. 새회귀·golden추가와통합PR은보류한다.
- 원격base는79abd9e49675f4286eaed38197f7e837afb744e7의#7693으로1커밋전진했다(`upstream-base-readonly-v72.json`).Rust엔진변경없이문서·Visual Sweep/exporter가변경됐다. 현재증거를보존하고커밋→fetch/devel동기화→rebase뒤새도구의전체쪽수·compat·색상범례계약으로fresh WASM/Native증거를확인한다.사용자지침파일을이작업에서직접수정하지않았다.

### 진행54 — 리베이스·시각 경계와 WASM 세대 API 묶음

- 보정커밋e12cd3886을보존한뒤devel을79abd9e49675로fast-forward하고94개로컬커밋을충돌없이rebase했다. 새code head는f3c8cd7d370668e36322db71c1c2c820260124ae이다.1354개Rust소스/빌드입력바이트동일·upstream조상·clean을확인했다(`rebase-source-identity-v72.json`). 새도구Native8쪽도exit0/167.749초/동일소스·전체쪽수검사로재출력했다(`native-source-frame-batch-review-v72-rebased/`).
- root wrapperfresh WASM은exit0/1101.409초/source31개불변이다. pkg/public/snapshot WASM SHA는2dc5a5730f42e9395261bc23217699bfde10a7d5c65b0ae4717435d4332f48d9로일치하며JS도일치한다(`wasm-package-source-frame-batch-v72-audit.json`). 그러나새exporter가요구한setHangul2024Compat가WASM에없어실제8쪽비교는exit1/6.692초·未출력이다.재빌드만으로해결되었다고보고하지않는다. `DocumentCore::set_hangul2024_compat`는존재하지만Deref만으로JS공개API가되지않는것이원인이다. 새layout_generation모듈은그기존함수·기존캐시무효화/재조판에직접위임한다.2020 Print비교는명시적으로2022를선택한다.
- #6935의남은3쪽윗선은block start_cut=[4,16,3,3]에서소진된0높이행의내부edge가원점에겹친것이다. Square·반복제목없음·블록시작컷에서만원점의0높이edge를확인하고,해당행에cut0/알수없는새셀owner가있으면원래선을보존한다. 이미소비한owner만있을때재개조각의새윗변으로그리지않는다. 높이·내용·최종닫힘선을바꾸지않으며Native/fresh WASM묶음재검증은v73에서실행한다.

### 진행55 — v73 Native 경계 확인과 미달 페이지 묶음 진단

- 경로는 `maintainer_general`이며 보조 문서는 `intake_and_review`, `local_validation`, `multi_pr_update_branch`, `visual_fixture_evidence`다. 최신 base의 90% 선행 조건·확대 판독·기존 회귀 이관 조건을 다시 확인했다. 사용자 승인된 메인터너 보정을 계속하며 지침 파일은 수정하지 않는다.
- v73 Native 빌드는 exit0/260.307초/source 불변이며 binary SHA는 `07258c8ee5fe2380dc030538778893b7d115fbdbfa4ea8bd25f77802cd7092c1`이다. 전9개 원본 쪽수는 v72와 동일한52/52,3/3,6935 3,6803 5,6024 103,교육415,6761 103이다. countercheck의 `unchanged=false`는 v64의6024 104쪽과 비교한 값이며 v72→v73 회귀가 아니다.
- Native6803·6935 전8쪽 sweep은 exit0/161.647초/source 불변,최저95.40319%/91.57542%,전체쪽수5/5·3/3·누락0이다. [6935 새3쪽 review](../../../output/pr-review/semanticist21-20261008/native-source-frame-batch-review-v73/issue6935/review/review_003.png)를 직접 읽어 빈왼쪽칸 위 짧은선 제거와 최종종료선·뒤주석 보존을 확인했다. `issue6935-fragment-edge-ownership-audit-v73.json`은2쪽 경계가v72와 동일하고3쪽 원점내부선만 제거됐음을 기록한다. fresh WASM 빌드는 진행 중이며 Native 결과로 대용하지 않는다.
- #6024 미달32쪽의 전쪽내용 대응 진단에서 모두 같은번호Print와 가장 잘 대응했다(`issue6024-full-below90-alignment-v72.json`). 이는 전역 쪽밀림 해소의 진단이며 시각 승인 근거가 아니다. v73 대표12/14/35/51/55/96쪽 재출력은 exit1/162.242초, 점수71.48133/53.48369/38.84165/48.53494/65.96649/48.27492%다. 35쪽은 이전쪽 업무행 잔여가 첫행으로 넘어와 다음표 시작과 내용컷까지 달라지고,51쪽은 첫업무행 높이 차이가 이후 행 경계에 누적되며,96쪽은같은표의내용행소속이앞쪽으로차이난다. 원본13개표의저장구조를묶음덤프하고측정·컷·paint경로를분석한다. 같은source의Native/fresh WASM을마칠때까지코드변경을동결한다.
- 원격 읽기전용 재조회는 열린30개 중 비draft23개이며 초기18개 중#7612는외부merge되어열린17개,후속6개는별도검토로유지한다(`open-pr-current-v73.json`). #7615 최신head의후속#7662꼬리는아직수용하지않았다. 원격변경·통합PR·CI/merge/후속처리는미수행이다.

- 독립 Print의51쪽 벡터 괘선을96dpi좌표로 추출했다(`issue6024-print-fragment-edges-v73.json`). Print50쪽r5의물리높이는974.2969→1025.1198=50.8229px,51쪽첫r6은142.0833→183.7969=41.7136px이다. Native는각각45.8/28.5px이다. `row_step.rs`의BAND_CUT가content limit35.7을수용하지만physical override를주지않아paint가content30.7+padding15.1로45.8을선택한다. `emit.rs`는완전소비된end_cut도accepted_row_boxes기록에서제외한다. 다음조각은실제수용prefix대신원래row5box를소비한다. 같은기록에34/35쪽의r23전체내용이월과95/96쪽의r28–30행이월을독립Print와연결했다(`issue6024-source-frame-consumer-diagnosis-v73.json`). 다음묶음은내용완전소비·실제수용빈밴드·closing span잔여를같은결과로연결하는범위다. 아직이변경을production에적용하거나해결완료로판정하지않았다.

- v73 최적화 fresh WASM은 exit0/1080.079초/source 불변으로 완료됐다. WASM SHA `503d25a88872861ef878dab749534a9fc268d16a77838a979d4ce8b79b4e8f42`, JS SHA `4628f097cddafb073362cc2ff03d19744de3e037aa62d311b4bd4386528c7a20`가 root pkg/Studio public/snapshot에서 일치한다. 기존에 추적하던 `rhwp-studio/public/rhwp.js`에는 wrapper가 생성한 새API8줄이 추가됐다. ignored pkg/WASM/파생suite는stage하지않는다.
- 실제 Studio 새로고침·#6935원본로드에서새WASM served SHA일치·font loaded·canvas3개·pageerror0을확인했다(`browser-studio-shared-source-frame-v73.json`). setHangul2024Compat(false→true→false) 실행은쪽수3→3→3과2022 RenderTree원상복원을확인했다. 이는generationAPI공개·기존캐시무효화위임·브라우저반영의증거이며2024독립Print정합검증은아니다. freshWASM8쪽sweep과기존73개회귀는실행중이다.

- fresh WASM6803·6935전8쪽sweep도exit0/182.505초/source불변으로끝났다. 두문서최저95.40319%/91.57542%·전체쪽수5/5·3/3·누락0으로Native와같다. 같은8쪽Native/fresh WASM RenderTree 전체가동일함을확인했다(`source-frame-batch-v73-backend-ownership-audit.json`). [WASM6935 2쪽review](../../../output/pr-review/semanticist21-20261008/wasm-source-frame-batch-review-v73/issue6935/review/review_002.png)와 [3쪽review](../../../output/pr-review/semanticist21-20261008/wasm-source-frame-batch-review-v73/issue6935/review/review_003.png)를직접읽어이어받는열린변·실제종료선·뒤주석소속을확인했다. #6024전103쪽·교육415쪽및나머지문서,최종lint/전체Rust/원격CI/merge를이8쪽결과로대신하지않는다.

- v73기존73개집중nextest는exit0/442.636초/실행18.259초,73PASS/6053skip,source23개불변으로끝났다(`nextest-shared-source-frame-batch-v73-exit.json`). 전체Rust·최종lint는별도미완료다. 검사 뒤소스변경에이결과를재사용하지않는다. 다음#6024묶음전현재변경을일반커밋으로고정하고upstream/devel정합을재확인한다.

### 진행56 — 내용 완료 행과 실제 수용 밴드의 공통 기록 보정(v74)

- v73 커밋은 `3505cb8cd8d09f3c125fdb08c9ccec6efece78ff`이다. fetch한 upstream/devel과local devel은모두79abd9e49675이며rebase는up-to-date였다. 검증한Native 소스23개와커밋바이트·생성Studio JS·fresh pkg의동일성을확인했고커밋직후clean이었다(`commit-source-identity-v73.json`).
- 독립Print34/35·50/51·95/96의source/consumer진단에따라두경로를한묶음으로보정한다. 첫source partial frame의내용완료행은기존ordinary owner만허용하는조건대신같은행에서닫히는모든span owner의원본줄유닛이실제prefix+남은밴드에완전히들어가는지를확인한다. 아직이어지는span·편집/재조판/합성줄·줄리셋은이완료증명에서제외한다. 첫source frame인증은기존호출자조건을유지한다.
- 내용이완전히소비된원본rowspan BAND_CUT는남은물리행예산을padding·빈밴드까지같은end_row_height_override로예약·배치한다. emit의accepted_row_boxes에도완전소비된일반end_cut의이물리높이를기록하여다음closing span이실제수용prefix를한번차감한다. 일반행#5585빈밴드이어받기·블록컷·마지막표행의기존경로는그대로검증대조군으로둔다. #6024 r137의0→0문단owner는별도미검증이다.
- 수정전대조군후보#5585 원본17쪽의Native/fresh WASM전쪽TSV를v73고정binary/pkg로실행중이다. 이실행은v74현재소스검증이아니며code source hash가진행중변경됐더라도입력·binary/pkg는3505cb8cd의검증산출물로고정되어있다. v74묶음은Native빌드→원본9개·영향review와같은대조군→기존집중회귀·fresh WASM을묶음실행한다. 신규fixture/golden/회귀추가·통합PR은아직보류한다.

- #5585 v73고정Native 전17쪽TSV는exit1/208.395초,15쪽40.95189%·16쪽33.72728%로미달했다. 기존fixture를정상대조군으로자동판정하지않는다. runner의source_unchanged=false는실행중v74원인보정을시작한것이며v73binary/pkg의변경은없다. 이결과는3505cb8cd와byte동일이확인된v73검증산출물의수정전증거다. fresh WASM결과·미달쪽직접review·v74같은입력대조는후속검증으로남긴다. #5585회귀기대값은변경하지않는다. v74 fmt·fmt check는exit0이며Native빌드는진행중이다.

- v74 Native 묶음빌드는exit0/274.593초/source불변, binary SHA `27be2550264b5ec1b878abc66b906ded09be0fe6d28938139acbf644e9b57adf`로완료됐다(`native-build-shared-source-frame-batch-v74-exit.json`). 원본9개·6024영향16쪽review·6803/6935전8쪽review·5585전17쪽TSV를같은source/binary로묶음실행중이다. 영향경계의직접판독전에는전체검증완료로보고하지않는다.

- v74 원본9개는6024 103쪽·Main52/52·1133 3/3·6935 3·6803 5·교육415를유지하지만6761은103→105로회귀했다. 최초차이는62쪽(구역3/para0/ctrl2) 첫조각endRow6→5이며원본r5에그림bin45/46이있다(`issue6761-section3-para0-v74.dump.txt`, `batch-analysis-issue6761-v74`). 새plain-line certificate를기존일반행의완료content box까지강제한것이잘못된가정이다. 일반행은기존측정box에그림도포함한다. 기존ordinary_owners경로를복원하고새certificate는앞행에서시작한closing span의추가증명에만적용한다(v74 retry1). 임계값·기대쪽수는변경하지않는다.
- v74 6024 35쪽review를직접읽었으며38.84165→99.42%로앞업무행이월·표끝·뒤제목·다음표첫행소속이개선됐다(`native-source-terminal-frame-diagnosis-v74/issue6024/review/review_035.png`). 같은묶음의6803/6935 Native 전8쪽은exit0/166.508초/source불변이다. 전체6024·6761회귀해소·fresh WASM및최종gate로대용하지않는다. 현재v74 retry1을같은원본9개와영향경계·대조군에서다시확인한다.

- v74 retry1 Native 빌드는 exit0/258.262초/source 불변, binary SHA `0c527aa96db817c432b4464314d909c302ab292049536856e5e6385953f01c02`다. 원본9개 재확인에서6761은105→103으로복구됐고6024 103·Main52/52·1133 3/3·6935 3·6803 5·교육415를유지했다(`pages-original-countercheck-shared-source-frame-batch-v74-retry1.json`). 쪽수복구를내용/시각무회귀증거로대신하지않고영향review를재실행한다.
- 초기v74의6024 영향16쪽review는exit1/468.721초,8쪽이90%미만(12/14/53/55/96/97/98/99)이며누락0이다. 실행중retry1소스변경으로source_unchanged=false였고고정v74binary의진단증거로보존한다. #5585는v73 Native/fresh WASM과v74 Native 모두17쪽중15/16쪽40.95189/33.72728%미달로같으며,최종검증이나정상대조군으로승격하지않는다. 기존회귀의기대값·baseline은변경하지않았다.

### 진행57 — 완료된 병합 셀의 뒤 행 소유와 합산 경계(v75)

- v74 retry1 Native6803/6935 전8쪽은exit0/165.299초,source불변이다.6024 영향16쪽은exit1/458.651초/source불변으로8쪽미달이남는다. 같은문서의13개원본표와전103쪽RenderTree 진단은exit0/2.376초로수집했다.
- 독립Print13쪽은r28 일반사항까지,14쪽은r29 오보·왜곡 보도 대응부터소유한다. 현source row completion은r25의긴병합owner가r35까지이어진다는이유로r28을배제했다. 저장줄내용전체가현재prefix+밴드에들어가는것과병합셀의물리종료행은별개다. 원본줄의완전한수용은유지하고,뒤물리행을continuation이계속소유하도록완료내용인증에서동일종료행가정을제거한다. 뒤유닛이실제로필요한owner·저장reset·편집/재조판은전체내용fit인증으로계속기각한다.
- Print95쪽의r28–30묶음은측정prefix810.44px와완료마지막행29.7466666667px로source901.4933333333px에들어간다. 기존block/개별row 실제합산순서를독립진단에서재현하면2.2737367544323206e-13px초과로엄격비교가실패한다(`issue6024-p95-complete-source-block-fp-diagnosis-v74-retry1.json`, `diagnose-complete-source-block-fp-actual-scan-v74-retry1.rs`). block 수용은기존runner의f64::EPSILON·실제연산개수기준roundoff만인정한다. content·물리행예산·paint좌표·시각90% gate의허용치를늘리지않는다. 실제초과인블록은기존이월/분할경로를유지한다. 두원인보정을한번에빌드하고원본9개·영향경계·73기존계약을검증한다.
- v75 Native build는exit0/254.622초/source불변(binary `feb935e1e1acbe552b39c1df0cfeeab2191773ea7a836ea17e1380f3a6de77fa`)이며원본9개쪽수는v74 retry1과같다.73개집중nextest도exit0/505.636초(실행23.565초),73PASS/6053skip/source불변이다.6803/6935 전8쪽Native는exit0/163.000초다. 테스트통과를시각무회귀로승격하지않는다.
- v75 6024 영향18쪽직접출력에서95/96쪽은90% gate를넘었으나14쪽첫두행겹침·51쪽첫업무행물리높이소실이관측됐다.51쪽점수는v74 retry1의통과에서58.39647%로악화됐고52/53쪽도미달이다. 현재변경의실제회귀로기록하고구현을고친다. 전103쪽TSV는고정v75binary로계속실행한다.
- 55쪽독립Print SVG의윗변은127.21875px,Native는123.5px다. 원본anchor 덤프에서61은표제앞control(pos0),106은표제뒤control(pos9)임을확인했다(`issue6024-source-anchors-v75.txt`). 첫조각준비에서`for_first_fragment`가제거한outer top을저장head control에만돌려주는가정이tail control의원본위여백283HU를잃는다. 같은유효저장host 계약의head/tail placement가동일바깥상자를소비하도록수정한다. 교육·합성/편집·호스트와겹치는줄은기존저장host 유효성·profile 조건으로제외한다.
- 내용완료조건의확대에서발생한회귀는원본끝행의실제physical row plan과paint 소비를추적한다. 기존옵트인TABLE_DRIFT에선택한original/accepted/resolved행상자를남겨최초손실계층을확인한다(v76진단). 아직51쪽등회귀해소·fresh WASM·최종검증근거는없다.
- v76 Native build는exit0/280.544초/source불변이며binary SHA `cbae672d0312a12ab878633ccd3b0efcd9f189fcf354eb85adb8d3f79e523da9`다. 옵트인진단은exit0/2.927초로실행됐다.61/102의첫accepted row ledger는각각37.2133333333/50.8266666667px를정확히기록했다. 뒤continuation의`start_row_height_override`가있으면`stored_fragment_row_plan`이조기반환하여그ledger를소비하지않는다. `DIAG_5714`도5.3466666667/13.3866666667px를새행높이로이월함을확인했다(`issue6024-next-row-tail-owner-diagnosis-v75.json`, `logs/batch-analysis-issue6024-v76.log`).
- v77은저장첫source row의완료증명과scan의실제끝행이일치하고end cut이없는경우,그이전행의blank tail을새행override로만들지않는다. 이미수용된physical ledger를다음closing span이차감하고새ordinary row의원래측정상자를보존한다. 같은행의부분컷과기존명시블록이어받기는별도소유로유지한다. source row 완료인증·블록roundoff·표제뒤outer top·잘못된tail owner수정을같은후보로원본9개·기존73+#5714/#5585·영향PNG에서검증한다.
- 55쪽outer top 보정의새Native sweep은exit0/38.751초/source불변,65.96649→99.74833%다. 표원점은123.5→127.3px로독립Print127.21875px와같은영역에서대조했고`native-source-frame-top-margin-v76/issue6024/review/review_055.png`를직접표시했다. fullsource v77/fresh WASM의최종근거로대신하지않는다.


### 진행58 — v77 실제 소비 복구와 남은 전체 검증

- v75 고정 binary의6024 전103쪽 TSV는 exit1/1232.193초로 종료했다. 75쪽이90% 이상이고28쪽이미달(12/14/19/27/40/47/48/49/51/52/53/55/56/59/62/66/67/68/76/83/84/85/91/92/94/97/98/99),최저47쪽45.66481%이며누락0이다. 실행중v76/v77 source 보정으로source_unchanged=false이며현재후보의성공증거가아닌수정전전체증거다(`native-source-terminal-frame-full-v75-exit.json`, `native-source-terminal-frame-full-v75/issue6024/silhouette.tsv`). 75/103=72.8%는이후보의페이지gate통과비율이며초기18개작업의전체완료율이아니다.
- v77 Native 묶음빌드는exit0/254.220초/source불변, binary SHA `7890ad6fbf1a2a1fe23289a96610d4448cbc1d6fe671ca7f2f18c96799b275b4`다(`native-build-shared-source-frame-batch-v77-exit.json`). 원본9개쪽수는v73/v75와같다.6024 countercheck의false는v64의104쪽과비교한것으로새회귀가아니다. 전103쪽진단도exit0/2.825초/source불변이다.
- 실제다음조각에서61/102의accepted prefix가소비되며,14쪽r29/r30은126.3/154.7px의연속28.5px상자로복구됐다.51쪽r6은142.1px부터41.8px이고뒤r7은184.0px부터시작한다. 독립Print의142.0833→183.7969px와같은영역에서대조한다(`issue6024-tail-consumer-before-after-v77.json`, `logs/batch-analysis-issue6024-v77.log`). [v77 14쪽review](../../../output/pr-review/semanticist21-20261008/native-source-terminal-frame-diagnosis-v77/issue6024/review/review_014.png)를직접열어60.00982→99.78%와겹침해소·내용순서를확인했다. 전체18쪽의terminal결과및51쪽PNG는별도확인한다.
- v77 Native6803/6935 전8쪽sweep은exit0/166.109초/source불변,최저95.40319/91.57542%·전체쪽수5/5·3/3·누락0으로통과했다. [6935 2쪽review](../../../output/pr-review/semanticist21-20261008/native-source-frame-batch-review-v77/issue6935/review/review_002.png)를직접읽어하단이어받기열린변·뒤내용보존을확인했다. 새v77 fresh WASM은아직실행하지않아v73의WASM8쪽통과로대신하지않는다.
- 기존5714의76076 원본과정확한독립2020 Print(82쪽)의35–37쪽대조도실행했다. v77은exit1/68.975초/source불변이며36쪽73.26187%로미달한다. 직접review의행경계차이를정상대조군으로승격하지않고수정전v73같은쪽과비교한다. 테스트기대값·golden·픽셀허용치는바꾸지않는다. 전체82쪽·fresh WASM의90%적격성은미검증이다.
- 6024의96쪽은점수가90%이상이어도Print의r59 `직제개정 요구`가97쪽으로이월된다. 현재block59..61의전체56.9px는잔여31.5px에안들어가지만ordinary r59의완전한28.4533px상자는들어간다. 재시도end_cut은첫행6owner완료/다음행4owner미소비이며cut_h13.3px라기존MIN_TOP_KEEP_PX기각경로에서emit전에이월된다(`issue6024-p96-complete-row-orphan-diagnosis-v77.json`). 미완료텍스트고아와완전한행상자를구분하는독립내용·물리높이근거를분석한다. 전역최소높이를낮추거나새정식6024회귀를추가하지않았다.
- 진행률보고에서실행완료·90%통과·현재head검증을구분한다. 현재집중78검사/6024영향18쪽은실행중이며,6024전쪽재검증·교육415쪽과6761등전체Native/fresh WASM·최종lint/전체Rust/Skia·통합PR/CI/merge/후속처리는남는다. 초기18개체리픽적용완료를초기18개검토완료로보고하지않는다.

- 76076 수정전v73의같은35–37쪽도exit1/128.268초로36쪽73.26187%다. 세쪽RenderTree SHA가모두v77과바이트동일하여이번묶음의새배치회귀는아님을확인했다(`issue5714-native-before-after-v77-audit.json`). 이것은upstream base 자체의적격성증거가아니며,90%미달의기존회귀를삭제/완화하거나정상대조군으로승격하지않는다.
- [v77 51쪽review](../../../output/pr-review/semanticist21-20261008/native-source-terminal-frame-diagnosis-v77/issue6024/review/review_051.png)를직접읽어58.39647→95.25%와첫r6·뒤업무행순서복구를확인했다. 하단r28의내용/빈밴드차이는남는다. 원본block27..29의hard reset으로end cut을수용한뒤r28부분을다음쪽에이어받는경로이며전체업무소속정확성의완료증거로대신하지않는다. 35쪽도99.42%의같은영역·표끝·뒤제목을직접대조했다.

- v77 집중nextest는exit100/474.332초(실행19.727초)/source불변,78개중76PASS/2FAIL/6048skip으로종료했다(`nextest-shared-source-frame-batch-v77-exit.json`). 두FAIL은기존5585의row3동일컷빈밴드이어받기와끝표뒤outer bottom이다. actual fragment는14쪽rows0..4/endcut없음→15쪽rows4..10/endcut[1,1]→16쪽rows9..10/startcut[1,1]이고뒤문단gap은-0.24px다. testcase가기대한row3빈밴드·아래1.87px와다르다. 기존73개및5714두검사는통과했지만전체묶음통과로보고하지않는다.
- 실패원본5585의17쪽전체TSV와14–16쪽review를현재v77고정binary로재실행한다. 수정전v73와현재v77의14/15/16쪽RenderTree는모두바이트동일이다(`issue5585-before-after-native-v77-audit.json`). 이는이번묶음의새회귀가아님을입증하지만기존실제배치결함또는테스트오류판정을해소하지않는다. 독립Print·현재출력·완전내용/빈밴드소유를직접확인하고원본을90%이상으로만든뒤의관계검사로검토한다. expectation/픽셀허용치/테스트제거는수행하지않았다.
- Cargo 집중검증의terminal을확인한뒤같은동결소스의root 최적화fresh WASM을공용target/pr-review로시작했다(`wasm-build-shared-source-frame-batch-v77-command.json`). 빌드가끝나고pkg/public/hash·실제브라우저·Native와동일영역sweep까지확인하기전에는WASM검증완료가아니다.

- v77 6024 영향18쪽sweep은exit1/638.469초/source불변·누락0·전체쪽수103/103으로끝났다.13쪽은90%이상이고12/53/97/98/99의5쪽은71.48133/50.65125/67.60932/74.76649/73.52941%로미달이다(`native-source-terminal-frame-diagnosis-v77-exit.json`, `native-source-terminal-frame-diagnosis-v77/summary.json`). v75의9미달쪽에서14/51/52/55가gate를회복했지만전체103쪽의최종적격성으로승격하지않는다.96쪽review도직접표시해96.83%에도마지막r59가빠지는실제차이를확인했다.
- v77 5585 실패영역14–16쪽review는exit1/86.978초/source불변이며15/16쪽40.95189/33.72728%다. 두PNG를직접열어15쪽에서다음row9 `【구글】Advertising Program`이앞당겨지고16쪽에는그제목이없으며뒤본문도위로이동함을확인했다. 기존문서최저90%조건을충족하지못하므로새회귀추가나기대값갱신은하지않는다.기존FAIL은소스동일v73와같은실제출력차이로기록한다.17쪽전체TSV는실행중이다.
- 이묶음은부분보정체크포인트로일반커밋한다. source byte identity와base ancestry는ignored검증기록에고정하며fresh WASM빌드·upstream동기화후소스가바뀌면해당영향검증을다시실행한다. 집중검사2FAIL·위미달/내용소속차이가남아push/PR검토요청/통합완료단계가아니다.사용자지침파일·테스트기대값·fixture/golden은이번묶음에서변경하지않았다.
