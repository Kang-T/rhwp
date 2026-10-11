---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7669 후속 체리픽·보정 계획

## 현재 상태

검토 진행 중이다. 선행 #7628은 통합 PR #7832로 병합됐으며, 그 보정 결과를
기준으로 #7669 고유 변경을 검토한다. 아직 시각·회귀 검증이나 수용 판정이 끝나지 않았다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `b8f36d6922a432ab30cb9d70a3ac4a87c98d3f37`
- 기본 경로: `collaborator_external_pr`의 9.1.1 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework
- 원 fork history와 branch는 그대로 보존한다.

## 적용 순서와 충돌 보정

| 원 commit | 적용 commit | 범위 |
| --- | --- | --- |
| `f411a51b7e78c63aa066f971666f7f94386a6254` | `bad64136b1b325a738dfc4d029b36b6f1a8b4bd6` | 저장 행 잔여·끝 조각 정렬·문단 기준 그림 |
| `2cd1cfa1ad` | `9e3f982a2` | 축소 sample·관계형 회귀 3건 |
| `41945eeee8` | `fd8d606cf` | 기여자 제공 대표 PNG, 현재 통합 head의 증적으로 재사용하지 않음 |

기존 선행 PR commit과 devel merge commit은 다시 체리픽하지 않는다.
`float_placement.rs`와 `table_partial.rs`에서 발생한 충돌은 다음처럼 해결했다.

1. 기존 공용 `cell_wrap_vertical_offset_hu`를 유지하고 partial picture 배치가 같은
   결과를 사용하게 했다. 원 PR의 마지막 merge에서 버린 과거 별도 Square helper를
   되살리지 않았다. 기존 일반 표의 구현도 보존했다.
2. #7832가 보정한 HWP5/HWPX 저장 쪽 프레임 및 고아 줄 정렬 조건을 보존하고,
   내용 컷을 모두 소비한 저장 끝 조각의 정렬 소유 조건을 추가했다.

## 구현 근거와 검증할 소비 경로

원본 `36308670`의 저장 선언 행 높이와 독립 한컴 Print PDF의 두 조각 물리 높이를
근거로 한다. 끝 조각의 세 줄·관련 사진·caption·뒤 `끝.`의 소속과 순서도 확인한다.
이미 #7832에 같은 원본의 보정과 관계형 검사가 있으므로 추가 변경의 필요성 및
그 보정과의 중복·충돌을 현재 출력으로 대조한다.

`emit.rs` 잔여 생산 → continuation cursor의 `start_row_height_override` →
scan 요구/예약 높이 → `table_partial.rs` 행 상자·cut unit 정렬 → 실제 그림 원점을
추적한다. 같은 helper 호출만으로 측정·최종 배치 일치를 단정하지 않는다.

## 남은 검증과 처리 순서

1. Native 최신 바이너리와 같은 원본/독립 Print PDF 전쪽 TSV 및 대표 review를 산출한다.
2. 원본·축소 sample의 조판 XML 동일성과 실제 출력 좌표 보존을 대조한다.
3. fresh WASM을 같은 source에서 빌드하고 같은 입력·글꼴·인쇄 프로필로 전쪽을 비교한다.
4. 관련 모든 쪽 90% 이상과 확대 판독 뒤 관계형 회귀를 판정한다. 원 PR이 공개한
   다른 문서의 점수 하락·쪽수 불일치는 기존 차이라는 이유만으로 면제하지 않는다.
5. #7669의 필요 보정을 커밋하고, 남은 #7672·#7692·#7701·#7805·#7814·#7815·#7825·#7828을
   의존 순서로 누적 검토한다. 동일 commit 또는 patch는 중복 적용하지 않는다.
6. 최종 후보의 해당 lint·회귀·실제 출력 backend·시각 게이트를 완료한 뒤 원 PR별
   review와 필요한 오늘할일을 같은 통합 PR에 포함한다. 최신 CI·merge·post-merge는
   기존 사용자 승인 범위와 canonical 절차를 따른다.

실행 로그·TSV·중간 JSON은 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.
검증한 source SHA와 원 PR 제공 자료의 SHA를 구분해 후속 결과를 이 파일에 연결한다.

## #7669 첫 경계 검증 결과 (통합 전 source)

검증한 source는 `f083228cd1eaa12db194a6c275f695fdadcfc4b8`다. 이후 8개 PR을
누적한 최신 후보에 이 결과를 그대로 적용하지 않는다. 첫 빌드는 stored-tail helper 누락으로
E0599가 발생했고, 별도 `table_partial/tail_alignment.rs`로 helper를 복원한 뒤 Native·fresh WASM
빌드가 각각 성공했다. 최적화된 WASM package와 Studio public의 JS/WASM SHA-256도 일치한다.

| 대응 입력 / 독립 Print | Native 전쪽 | 최적화 fresh WASM 전쪽 |
| --- | --- | --- |
| `samples/issue7344/original/36308670_seoul_approval.hwpx` / `pdf/issue7344/36308670-2020.pdf` | 2쪽, 99.46689 / 99.99288% | 2쪽, 같은 점수 |
| `samples/issue7095/36308670_split_row_declared_tail.hwpx` / `pdf/issue7095/36308670-split-row-declared-tail-2020.pdf` | 2쪽, 99.46689 / 99.99918% | 2쪽, 같은 점수 |

축소본의 조판 XML 6개는 원본과 byte 동일하며 이미지 5개만 축소했다. 축소본의 대응 Print는
Hancom 2020 MCP engine 2020, PrintMethod=0으로 별도 생성했다. 입력 SHA-256은
`4982d53c16ba51c0366aea8eb9303d748a26750849f582fb47413e1a80979de4`, Print는
`42ad85ae601c832778e7501c57b6d10448bc7741c1e00f564beb344c6e88da02`다.

실행은 96dpi, compat 2022, full embedded fonts이며
`RHWP_FONT_PATH=/opt/hnc/hoffice11/Shared/Fonts:/usr/local/share/fonts:/usr/share/fonts:/home/tsjang/.local/share/fonts`를 사용했다.
`pr7669-native-build-2.log`, `pr7669-fresh-wasm-build.log`,
`candidate-native-full-scores/`, `pr7669-optimized-wasm-scores/`에 명령과 전쪽 TSV를 보존했다.
대표 Native review 1·2쪽에서 표 외곽·이어받기·그림 포함·caption·뒤 `끝.`의 순서를 직접 확인했다.
제목·본문의 글꼴 획 무게와 사진 raster 차이는 남는다. Native와 최적화 WASM 4 raster는 byte 동일하다.

이 원본은 #7832 직전 보존 바이너리의 layout 출력과도 동일했다. 따라서 첫 원본 점수만으로
#7669의 추가 개선을 주장하지 않는다. 기여자가 보고한 다른 문서의 악화·쪽수 불일치와
통합 후보의 최종 시각·관계형 회귀 검증이 남아 있다. 공개 CLI의 정렬 관계 진단은
정식 Rust 회귀 통과를 대신하지 않는다.
