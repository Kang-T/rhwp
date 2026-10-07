---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-08
---

# semanticist21 비 draft PR 통합 검토

## 현재 판정

**머지 보류 — 검토 진행 중.** 18개 비 draft PR의 고유 41개 contributor commit을 누적 적용했다. #7613·#7616·#7641·#7656·#7658에서 실행으로 gate 미충족을 확인했다. 나머지 PR의 부분 계약 통과를 전체 검토·CI·merge 완료로 취급하지 않는다.

- 브랜치: `review/semanticist21-20261008`
- base: `60efc4b7b380b2a183c20c3ee0ab24a60bc2d832`
- contributor 누적 head: `cd0eb2d25380f6db60a4b8bb99ed90f7e1d5c84c` (#7659 포함). 현재 추가 메인터너 보정은 미커밋 진단 단계.
- 최초 17 PR 검증 code head: `134a646e5a5325daf52d614e0395094175e78301` (아래 최초 검증 결과의 역사 head)
- 최초 검토 head: `8a9f87300a47293d644f8b862906c989adf34f84` (새 source 적용 전 역사 결과)
- 47 source/test 파일, 5421 추가 / 823 삭제. contributor 원 commit은 `cherry-pick -x`로 보존. merge commit 제외, #7615 중복 의존 commit 4개는 재적용하지 않음.
- draft 제외: #7559, #7572, #7573, #7588, #7589, #7598.
- 통합 PR 생성/push/merge/원 PR close/후속 처리: 아직 미수행. 실패 gate를 통과한 것으로 보고하거나 일부 PR만으로 scope를 줄이지 않음.

## PR별 진행

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
