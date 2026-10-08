---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-08
---

# semanticist21 비 draft PR 통합 검토

## 현재 판정

**머지 보류 — 메인터너 보정과 검토 진행 중.** 초기18개 비 draft PR의 고유41개 contributor commit을 누적 적용했다. 최초 검토의 미충족과 이후 보정 결과는 각각의 source SHA에 연결해 아래에 기록한다. 현재 남은 일반 문서의 시각 미통과·내용 소속 차이·최종 head 검증이 해소되기 전에는 통합하지 않는다.

- 브랜치: `review/semanticist21-20261008`
- 현재 통합 base: `f0e7228f6dd2ea1437724e53ad640c40c56d204b` (최신 devel 재조회 일치). 최초 검토 base `60efc4b7b380b2a183c20c3ee0ab24a60bc2d832`는 역사 검증의 기준이다.
- contributor 누적 head: `cd0eb2d25380f6db60a4b8bb99ed90f7e1d5c84c` (#7659 포함). 메인터너 보정 v44 source는 `64cc5b5e805e2b05ab3fd64fa5aa4803e0d6357a`이며, 이후 후보와 검증은 아래 단계별 기록을 따른다.
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
