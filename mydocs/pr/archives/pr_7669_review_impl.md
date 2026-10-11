---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-11
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
