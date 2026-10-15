---
kind: report
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-15
---

# PR #7661 리뷰 — 셀·쪽 속성의 그림·그라데이션 배경 보존

## 최종 판정

**머지 보류 — 원래 API 결함과 수정 효과는 재현했지만, 기존 Studio 소비자의 새 배경 삭제 요청과 필수 시각 증적 미검증이 남아 있다.** 이번 검토에서는 제품 보정·원격 push·승인·merge·댓글 게시를 수행하지 않았다. 검토 지시에 따른 reviewer 지정만 반영했다.

## 접수 정보와 기존 맥락

- [PR #7661](https://github.com/edwardkim/rhwp/pull/7661), 작성자 `semanticist21`, base `devel`, non-draft, 원 head `f1da6189670357b4a2394c73d7106137ab4b38c4`.
- 원 head CI는 성공, 조회 시점 `MERGEABLE/CLEAN`. 2 commits / 3 files / +177 −2. assignee=author, milestone=v1.0.0, labels=bug/rust/api/roundtrip/table로 트리아지 완료 상태다. reviewer는 `edwardkim`을 지정하고 API로 재확인했다. 별도의 종료 대상 이슈 참조는 없다.
- 기본 경로 [maintainer 일반](../manual/pr_review/maintainer_general.md), 보조 intake_and_review·local_validation·visual_fixture_evidence·post_merge와 review_template를 적용했다. 기존 기여자이므로 첫 기여자 경로는 적용하지 않았다.
- 이전 [누적 검토 진행78](archives/semanticist21_20261008_integrated_review.md)은 읽기 전용 보류였다. 이번 사용자의 개별 처리 지시로 검증을 시작했으며 초기18개 통합에 새로 포함된 것으로 설명하지 않는다.
- 의존 [#7655](https://github.com/edwardkim/rhwp/pull/7655)는 개별 merge가 아닌 [#7882](https://github.com/edwardkim/rhwp/pull/7882) 통합으로 devel에 반영됐다. 현재 `border_fills_equal`은 `a.fill == b.fill`로 전체 Fill을 비교한다. 이 의존성을 새 미해결 조건으로 요구하지 않는다.
- 고정 최신 base `a5e88ad682c805e1d845502ddc9521b94ea8eccd`, clean merge tree `ef7751dbee09a0e2a5a8bfa23a492a441a1a8b72`, 로컬 통합 commit `8410f8179493a83866e5b45aa00f361700546662`. 원 PR과 base를 함께 담은 검토 후보이며 원격 head의 새 CI로 세지 않는다.

## API 수정 효과와 실제 소비 경로

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
