---
kind: report
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-15
---

# PR #7661 리뷰 — 셀·쪽 속성의 그림·그라데이션 배경 보존

## 최종 판정

**머지 보류 — API와 Studio 배경 보존 보정은 검증했으나, 문단 배경의 기존 렌더링 누락까지 수정하는 승인 범위를 진행 중이다.** 원 PR head를 직접 merge 대상으로 삼지 않는다. 원격 push·승인·merge·댓글은 수행하지 않았다. 아래 초기 검토와 후속 메인터너 보정을 구분한다.

## 접수 정보와 기존 맥락

- [PR #7661](https://github.com/edwardkim/rhwp/pull/7661), 작성자 `semanticist21`, base `devel`, non-draft, 원 head `f1da6189670357b4a2394c73d7106137ab4b38c4`.
- 원 head CI는 성공, 조회 시점 `MERGEABLE/CLEAN`. 2 commits / 3 files / +177 −2. assignee=author, milestone=v1.0.0, labels=bug/rust/api/roundtrip/table로 트리아지 완료 상태다. reviewer는 `edwardkim`을 지정하고 API로 재확인했다. 별도의 종료 대상 이슈 참조는 없다.
- 기본 경로 [maintainer 일반](../manual/pr_review/maintainer_general.md), 보조 intake_and_review·local_validation·visual_fixture_evidence·post_merge와 review_template를 적용했다. 기존 기여자이므로 첫 기여자 경로는 적용하지 않았다.
- 이전 [누적 검토 진행78](archives/semanticist21_20261008_integrated_review.md)은 읽기 전용 보류였다. 이번 사용자의 개별 처리 지시로 검증을 시작했으며 초기18개 통합에 새로 포함된 것으로 설명하지 않는다.
- 의존 [#7655](https://github.com/edwardkim/rhwp/pull/7655)는 개별 merge가 아닌 [#7882](https://github.com/edwardkim/rhwp/pull/7882) 통합으로 devel에 반영됐다. 현재 `border_fills_equal`은 `a.fill == b.fill`로 전체 Fill을 비교한다. 이 의존성을 새 미해결 조건으로 요구하지 않는다.
- 고정 최신 base `a5e88ad682c805e1d845502ddc9521b94ea8eccd`, clean merge tree `ef7751dbee09a0e2a5a8bfa23a492a441a1a8b72`, 로컬 통합 commit `8410f8179493a83866e5b45aa00f361700546662`. 원 PR과 base를 함께 담은 검토 후보이며 원격 head의 새 CI로 세지 않는다.

## 초기 검토: API 수정 효과와 실제 소비 경로

`helpers.rs::fill_json_values`는 기존의 단색·없음 판정을 유지하고 Gradient/Image를 같은 이름의 JSON 종류로 읽는다. 셀·표·쪽·글자·문단 조회가 이 결과를 사용한다. `html_table_import.rs::create_border_fill_from_json`은 `borderFillId`의 정의를 먼저 복제하고 gradient/image readback이면 기존 채우기를 보존한다. `none`은 계속 명시적 삭제이며 `solid`는 색/무늬 편집이다. 정의 재사용은 위의 Fill 전체 비교를 거친다.

이 수정은 문서 ID·고정 좌표에 맞춘 예외가 아니라 입력 종류·소유 정의를 보존하는 API 계약이다. 다만 getter의 새 종류를 실제 소비자가 어떻게 다시 쓰는지까지 확인해야 한다. API에서의 보존을 UI 해결로 확대하지 않는다.

### 실행으로 확인한 결함

현재 Studio의 `CharShapeDialog`와 `ParaShapeDialog`는 `populateFromProps()`에서 gradient/image를 `none` 선택값으로 표시한다. 이어 `collectMods()`가 원래 종류와 비교해 `fillType: "none"`을 생성하고, 아무것도 바꾸지 않은 `handleOk()`가 `onApply`를 호출한다.

Node 24의 TypeScript strip/transform으로 **제품 class의 실제 populateFromProps → collectMods → handleOk**를 실행했다. 가져온 번역 함수, 미리보기·언어 선택 처리, DOM 입력 컨트롤은 대역이다. 편집 대상 값 비교·배경 선택·변경 수집·적용 분기는 제품 메서드를 그대로 사용한다. 실제 브라우저/WASM 검증으로 보고하지 않는다.

| 대화상자 | API 종류 | UI 표시 | 아무 변경 없이 확인한 결과 |
| --- | --- | --- | --- |
| 글자 | gradient / image | none | 각각 `{fillType:"none"}` 적용 요청, 2건 |
| 문단 | gradient / image | none | 각각 `{fillType:"none"}` 적용 요청, 2건 |
| 글자·문단 대조군 | none / solid | 원래 종류 | 변경 없음, onApply 0회, 4건 |

실제 명령 연결 `rhwp-studio/src/command/commands/format.ts:260–291`은 이 mods를 `applyCharPropsToRange`·`applyParaPropsToRange`에 넘긴다. 조회는 `formatting.rs:693/775`의 공통 helper를 거치고, apply 경로는 같은 파일의 `create_border_fill_from_json`을 호출한다. 그 함수에서 `fillType:"none"`은 `Fill::default()`가 된다. callback 이후에 gradient/image를 복원하거나 이 키를 제거하는 분기는 없다. 최종 WASM 저장·재열기의 직접 재현은 미실행이다.

기존 devel getter는 gradient/image를 none으로 읽었으므로 이 무변경 UI 경로에서는 종류 차이가 없어 삭제 요청을 새로 만들지 않았다. PR의 올바른 readback과 기존 UI의 조합에서 생기는 새 회귀다. 셀·쪽 대화상자가 기존부터 none을 쓰는 별도 잔여 문제와 구분한다. 그라데이션/그림 편집기 전체를 새로 만들 필요는 없고, 미지원 원래 값의 유지와 명시적인 삭제 선택을 구분하면 된다.

## 완료한 검증과 재사용 범위

같은 공용 `/home/edward/mygithub/rhwp/target/pr-review`를 순차로 사용했다. 원 PR의 두 검사를 바꾸지 않고, 먼저 최신 base의 두 제품 파일만 복원한 음성 대조를 실행한 뒤 통합 후보의 원래 파일로 되돌려 다시 실행했다. 임시 제품 파일 변경은 전부 복원했다. 생성 suite/manifest는 PR source에 포함하지 않는다.

```text
cargo nextest run --locked --cargo-profile release-test   --target-dir /home/edward/mygithub/rhwp/target/pr-review   --test regression_suite_007   -E 'test(gradient_image_fill_readback::)' --no-fail-fast
```

| 범위 | 실제 결과 |
| --- | --- |
| 최신 devel + 원 PR 검사만 | 2 FAIL / 0 PASS, exit 100. 셀·쪽 조회 none, 편집 후 및 HWP/HWPX 재열기 Fill 손실로 실패. 환경·빌드 실패가 아님. |
| 최신 devel + 원 PR 구현 | 2 PASS / 0 FAIL, exit 0. 셀·쪽의 모델 Fill 보존 확인. 렌더링 일치 검사로 세지 않음. |
| 실제 UI 메서드 진단 | 새 배경 삭제 요청 4건 재현, 정상 대조 4건. DOM 대역이며 실제 브라우저/WASM 미실행. |
| fmt·manifest·Git | fmt check와 고정 base manifest check 통과. 1,465 sources / 28 suites + 20 exceptions. 자동 merge 충돌 없음·공백 정상, 임시 source 복원 확인. |

[원 head CI](https://github.com/edwardkim/rhwp/actions/runs/37667764031)의 실제 head SHA가 `f1da6189670357b4a2394c73d7106137ab4b38c4`이고 completed/success임을 재조회했다. archive a–d 검사·lint 성공 및 [CodeQL](https://github.com/edwardkim/rhwp/actions/runs/37667763560), Adapter·Proptest 결과를 확인했다. 코드 보정 없는 검토이므로 녹색 CI의 광범위 검사를 중복 로컬 실행하지 않았다. CI의 Native Skia·WASM Build는 skipped이며 실제 fresh WASM 증거로 사용하지 않는다. 최신 base 통합 후보 전체 회귀의 성공을 주장하지 않는다.

## 검증 입력 커밋 확인

아래 5개는 실제 focused 검사 입력이며 통합 commit `8410f8179493a83866e5b45aa00f361700546662`의 blob과 실행 파일 바이트가 같다. 별도 개인 corpus는 사용하지 않았다.

| 저장소 경로 | SHA-256 |
| --- | --- |
| `samples/issue7235/156086935_none_image_fill.hwp` | `b12aa445776bef38a003e8679865a9a0ed2f729ecef18099625c976efe64d6ae` |
| `samples/hwpx/mel-001.hwpx` | `ec75dc24ade52e055c54ea0529af100a623027c465571fda7edaa97f6e736d8b` |
| `samples/hwpx/el-school-001.hwpx` | `e9636dccf3fdfed897f9cdc5d3a358d93093a6d2a509592898413493ae67a354` |
| `samples/basic/NewYear_s_Day.hwp` | `1be2736df6b21687f5c1627bac307cc689171928f6ceb0a235af0bc727eb6372` |
| `samples/issue2816/imgbrush_total_page_fill.hwpx` | `75d28e63f4610c751ea9cc56a41634c3f7a7761b817170a6b84acfa90511237f` |

원본 Print 후보는 `samples/issue7235/156086935_none_image_fill-2020.pdf`, `pdf/hwpx/mel-001-hwpx-2020.pdf`, `pdf/hwpx/el-school-001.pdf`, `pdf/basic/NewYear_s_Day-hwp-2020.pdf`, `pdf/issue2816/imgbrush_total_page_fill-hwpx-2020.pdf`에 이미 있다. 존재를 확인했지만 수정된 편집본과의 대응·출력 유효성은 미검증이다. 기존 기준을 먼저 재사용 검토하며, 원본 PDF를 여백이 변경된 편집본의 Print로 자동 간주하지 않는다.

## 조판 원칙과 증거 판정

| 항목 | 판정·근거 |
| --- | --- |
| 구현 근거·일반성 | API 계약 범위 충족: 실제 원본의 Fill 보존, 종류·정의 기준이며 sample 예외 없음. |
| 측정·배치 공통 결과, 분할·이어받기, 줄 소속 | 비해당: 해당 계산·기하·pagination 코드를 변경하지 않음. 채우기 paint 결과 영향은 아래 별도 미검증. |
| 최종 소비자·backend | 미충족: 기존 글자·문단 UI가 새 종류를 삭제 요청으로 바꾸는 경로 실행. Native PNG/Studio/WASM/SVG 실제 paint는 미검증. |
| 사례·기대값 독립성 | API 2건은 원본 Fill 및 저장 전 대조군을 기준으로 실제 FAIL→PASS. UI 진단은 synthetic 속성·DOM 대역 범위로 제한. |
| 기준값·회귀 | golden/baseline/허용치 변경 없음. 모델 Fill 검사 통과를 렌더링 위치의 정답으로 승격하지 않음. |
| 시각 선행·확대 판독 | 미검증: 동일 편집본 Print 및 최신 base/후보의 전쪽 Native/fresh WASM TSV·대표 PNG·직접 판독 없음. |
| 검증 입력 커밋 | focused 원본 5개 충족. 최종 편집본·시각 기준 대응은 아직 미검증. |
| 주장·검증 범위 | API 보존 효과만 확인. cellzone 잔여7곳은 기여자의 기존 보고이며 이번에 전수 재측정하지 않음. UI 전체 해결/시각 통과를 선언하지 않음. |

## 보완 요청과 해제 조건

1. 기존 글자·문단 대화상자가 미지원 gradient/image를 유지하도록 보완한다. 그대로 확인·다른 속성만 변경·명시적 채우기 제거를 구분하고, none/solid 정상 대조도 유지한다. 단순 문자열 가드나 API 검사만으로 UI 소비 검증을 대신하지 않는다.
2. 원래 API 편집 → HWP/HWPX 저장 → 재열기와 실제 Studio 동작을 fresh Docker WASM에서 확인한다. 기존 셀·쪽 대화상자 및 cellzone은 수용 범위와 잔여 범위를 명시한다.
3. 기존 원본 Print 후보를 먼저 검토하고, 검증하는 편집본에 대응하는 독립 Print와 Native/fresh WASM 전쪽 TSV·대표 review/overlay를 새 head로 제시한다. 90% 최소 및 최신 devel 대비 0.01%p 하락 gate·직접 판독을 모두 확인한다.

ignored `output/pr-review/pr7661-20261015/`에 메타데이터·focused-results·입력 manifest·진단 코드/JSON/로그·원본 Print 후보 목록을 보존한다. 기여자 댓글 초안은 `contributor-comment-ko.md`이며 아직 게시하지 않았다. 원 PR 브랜치나 원격 상태를 임의 보정하지 않고 작업지시자 승인 뒤 게시한다.

## 메인터너 보정 1: 대화상자 보존 계약

작업지시자의 “메인테이너 보정으로 진행” 승인 후 같은 `review/pr7661-20261015` branch에서 보정했다. collaborator 외부 PR 절차에 따라 검토 branch를 기본 작업공간으로 옮겼고, 임시 worktree만 해제했다. 현재 기록 경로는 이 문서이며 기존 임시 worktree 경로를 사용하지 않는다.

- 원 contributor head: `f1da6189670357b4a2394c73d7106137ab4b38c4`.
- devel 통합 commit: `8410f8179493a83866e5b45aa00f361700546662`.
- UI 보정 commit: `d18d59f8f2bdaea5be5f7463f822601f6d447d20`.
- 공용 target: `/home/edward/mygithub/rhwp/target/pr-review` 유지.

`fill-selection.ts`가 원래 gradient/image 종류를 “기존 … 유지” 선택지로 유지한다. none은 명시적 삭제이고 solid만 색·무늬 입력을 활성화한다. 글자·문단 대화상자는 미지원 종류를 유지할 때 단색 입력의 잔류값을 전송하지 않는다. 테두리/채우기를 바꾸면 원래 `borderFillId`를 함께 보내 전체 정의를 상속하며, 단색을 바꾸면 색·무늬 종류·무늬 색을 모두 전송한다. 다른 글자·문단 속성만 바꿀 때 채우기 payload를 새로 만들지 않는다.

실제 경로는 `getter Fill 종류/id → populateFillSelection → collectMods → 기존 format command/API → create_border_fill_from_json의 원래 정의 복제 → 전체 Fill 비교 → 저장·재열기`다. 이 단계에서 renderer·줄 높이·pagination은 변경하지 않았다.

| 검증 | 결과와 한계 |
| --- | --- |
| 실제 제품 대화상자 메서드 회귀 30건 | 수정 전 제품 class로 26 FAIL / 4 PASS, 수정 후 30 PASS. DOM·미리보기는 대역이며 실제 브라우저 증거는 아래 별도다. |
| Studio 전체 Node 검사·TypeScript | 최종 1,870 PASS / 0 FAIL / 2 skip. Docker wrapper로 오래된 pkg 선언을 갱신한 뒤 TypeScript 통과. |
| 실제 Chrome DOM + fresh Docker WASM | image/gradient × 글자/문단 × 무변경/다른 속성/none/solid의 16건. 실제 선택·확인 클릭, public format API, live/HWP/HWPX 재열기에서 종류와 테두리 보존. command 라우터 전체의 자동 UI 조작 검사는 아니다. |
| 원 PR API의 fresh WASM 대조 | 기존 공개 원본 5개, 셀 8곳·쪽 2곳을 live/HWP/HWPX에서 확인. 정규화한 전체 fillBrush XML도 보존. 편집본의 전체 Print 시각 검증을 대신하지 않는다. |
| Docker WASM | locked wrapper·공용 target으로 성공. pkg/public과 실제 Chrome 응답 SHA-256 `0ff83fdc16b7f3a73fc7b29070260b3b486663c1f2d537a95b6053bb94591b92`. UI만 바뀐 보정 commit에도 같은 Rust 산출물을 재사용했다. |
| Native Skia | `cargo build --locked --release --bin rhwp --features native-skia --target-dir /home/edward/mygithub/rhwp/target/pr-review` 성공. CLI SHA-256 `407725365232991276cd71f7cf11b3785935a94d594f85000e7095df6a1edad4`. |

상세 실행은 ignored `output/pr-review/pr7661-20261015/`의 `browser-verification.json`, `wasm-api-readback.json`, `logs/studio-tests-final.log`, `logs/typescript-final.log`, `maintainer-code-proof.json`에 보존했다. 초기 전체 binary 빌드는 불필요한 q 도구까지 포함해 중단한 실행이며 성공으로 세지 않는다. 이후 CLI 지정 빌드는 별도로 성공했다.

## 메인터너 보정 1의 독립 출력 비교

공개 `el-school-001.hwpx`의 기존 글자·문단 속성을 새 ID로 복제하고 원본 그림 채우기 또는 `mel-001.hwpx`의 그라데이션을 연결한 수동 UI 검증 입력이다. 텍스트·저장 LineSeg·쪽 속성·이미지 자원은 유지했다. 한컴 정상 생성 원본이라고 부르지 않는다. 두 입력과 실제 브라우저에서 글자 크기/문단 여백만 바꿔 저장한 네 HWPX 모두 한컴 2020에서 열어 Print PDF를 생성했다. actual engine `11.0.0.9136`, `pdf_print_method=0`, `hancom2020_pdf_driver_one_up`이며 각 1쪽이다. 생성 방법·해시·MCP job/profile은 fixtures manifest 및 sanitized status JSON에 남겼다.

같은 PDF·fonts·96 DPI·compat2022에서 고정 base `a5e88ad682`와 보정 1을 비교했다. cached base producer의 Rust source는 고정 base와 동일함을 `src/crates/Cargo` diff로 확인했고 새 캡처에는 현재 도구를 사용했다. 6개 문서 모두 각 backend의 base/head PNG RGBA가 동일하다. 따라서 이 보정에서 0.01%p 이상 하락한 페이지는 없다. **독립 PDF와의 일치 및 자동 gate는 별도다.**

| 1쪽 문서 | Native base = 보정 1 | fresh WASM base = 보정 1 |
| --- | --- | --- |
| image | 92.31721% | 91.94430% |
| image-char-unrelated | 88.07046% | 87.89580% |
| image-para-unrelated | 92.29444% | 91.94491% |
| gradient | 90.36146% | 90.58653% |
| gradient-char-unrelated | 86.49732% | 86.91110% |
| gradient-para-unrelated | 90.35778% | 90.57237% |

최신 head 대표 image/gradient 글자 편집 review PNG를 직접 열어, 문단의 그림·그라데이션 미출력 및 일부 글꼴 굵기 차이를 확인했다. 단순 텍스트 추출·쪽수·점수만으로 시각 통과를 선언하지 않았다. 자동 판정은 `re_review_required`이며 원래 5개 API 원본의 **편집본** 전쪽 Print 비교도 아직 미검증이다. 수동 입력 6개 통과/실패와 혼합하지 않는다.

자료는 `output/pr-review/pr7661-20261015/{native,wasm}-{base,head}/<key>/silhouette.tsv`, `visual-decision.json`, `visual-review/<key>/<backend>/review_001.png`에 있다. 복사 가능한 그림 내장 HTML은 `/mnt/e/tmp/pr7661-maintainer-review/visual-review.html`이며 같은 폴더에 입력과 PDF를 복사했다. 원본 TSV·로그·중간 JSON 및 미달 fixture/golden은 커밋하지 않는다.

## 승인된 후속 범위: 문단 배경 렌더링

작업지시자는 위 자료를 안내받은 뒤 **“문단 배경 렌더링까지 보정”**을 승인했다. 시각 예외 승인은 없으며 자동 gate는 유지한다.

위반 계약은 문단 BorderFill의 그림·그라데이션도 실제 문단 배경 상자에 그려져야 한다는 것이다. 독립 근거는 위 같은 입력의 한컴 Print이며 이 범위는 조판 흐름·문단 높이를 바꾸지 않는 paint 소비 수정이다.

값 추적: `resolve_border_style`의 `gradient/image_fill` 생산(`style_resolver.rs`) → `paragraph_layout.rs`의 실제 줄 상자·spacing·border inset 범위 수집 → `layout.rs::render_para_border_groups`의 연속 테두리 그룹과 최종 상자 → 현재 `fill_color`만 소비하는 Rectangle. 일반 본문 호출은 `layout.rs`, 셀 문단 호출은 `layout/table_layout.rs`에 있다. 셀 자체 배경은 이미 `render_cell_background`에서 같은 종류와 이미지 효과를 소비하므로 이를 공통 paint 결과로 재사용한다. 채우기 종류가 달라도 stroke가 같다는 이유로 첫 문단의 Fill을 뒤 문단에 복사하지 않으며, 채우기를 나누기 위해 연결된 테두리에 새 가로선을 만들지 않는다.

구현 전 반례는 연결 테두리 내 서로 다른 채우기/채우기 없음, 같은 채우기 연속 문단, 본문/셀 문단, 분할된 문단 조각이다. 실제 저장 속성과 독립 Print의 관측으로 기대를 정하고 영향 영역을 먼저 비교한다. 새 정식 배치 회귀는 Native/fresh WASM 90%와 직접 판독 이후에 추가하며 absolute px/전체 SVG 해시를 핀하지 않는다. 현재 대화상자/API 검증을 새 renderer 검증의 대용으로 삼지 않는다.

## 추가 승인: API 대조 문서 6쪽의 저장 좌표계 전환

작업지시자는 기존 6쪽 위치 차이도 이번 범위에서 보정하도록 승인했다. 같은 편집 HWPX와 독립 한컴 2020 Print의 14쪽 전량을 대조한 결과, 첫 표(문단 97) 뒤부터 약 5px 차이가 누적 없이 유지된다. 입력은 실제 HWP를 public API로 편집·저장한 것이며 임의 LineSeg 수정본이 아니다. 해당 표의 저장 줄 높이 10,377HU는 표 10,095HU와 위·아래 바깥여백 141HU씩을 포함한다. 다음 문단의 저장 시작 11,129HU는 이 높이와 후행 간격 752HU의 합이다. 표 호스트의 이전 쪽 저장 원점 67,373HU를 그대로 더하면 이 닫힌 새 쪽 좌표계를 인식하지 못하고 간격을 절반으로 줄인다.

추적 경로는 `composer의 저장 줄·후속 원점 → float_placement::stored_tac_line_successor_advance → typeset/controls/tac_reconcile::measure의 점유 상한 → layout::tac_next_line_full_spacing의 실제 후행 흐름`이다. 저장 원점 리셋의 판정은 온전한 단일 TAC 줄 상자와 다음 원점의 정확한 닫힘으로 설명해야 한다. 개체와 다른 줄이 섞인 문단·dirty/계산 LineSeg·닫히지 않는 후속 좌표에 이 판정을 확장하지 않는다. 절대 px 보정이나 문서별 예외는 추가하지 않는다. 구현 뒤 같은 원본 전쪽과 정상 대조군을 다시 확인한다.
