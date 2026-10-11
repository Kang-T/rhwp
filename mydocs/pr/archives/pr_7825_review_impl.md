---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7825 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **문단의 글자 단위 여백**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `b368487aa2bac4234c1dd1755b1a768485d65212`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `d3cc21e2f1927fdc32cf3e8d4ccea5541c10ff5c` | `542b7622f5f5dc51d6dd2f9c5585d1e513566f6a` |
| `b368487aa2bac4234c1dd1755b1a768485d65212` | `2d6bb006abd7f35f3f3350fca5049840a879c9cb` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

stored unit decode·기준 스타일 크기 → composer/layout의 유효 여백·indent → 실제 줄 구성과 문단 원점을 확인한다. 측정 뒤 별도 원점 덮어쓰기도 대조한다.

## 독립 기대값과 검증 계획

합성 9개와 같은 입력 Print 전체를 비교한다. 들여쓰기 경계·정렬·base char style·위아래 간격을 확인하고 절대 원점 pins는 시각 통과 뒤 독립 관계형 계약으로 보정한다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.

## 합성 줄 말미 공백 정렬 보정 근거

통합 source `8e56e22c1b82a420f57958566738d4b6cc37dfcf`의 9개 합성 입력 전쪽 대조에서
여러 입력이 90% 미만이었다. 실제 바탕 글꼴 공급과 Full 임베딩을 확인한 뒤에도 미달이 남았다.
`base_charpr0_20pt`의 PDF subset과 실제 Batang.ttf의 한글 11개 글리프는 윤곽·폭이 동일하다.
첫 긴 줄의 줄 나눔은 같지만 마지막 단어의 시작은 Native 652.688px, 독립 Print 658.843px였다.
이는 배치 고정용 좌표가 아니라 같은 영역의 수정 전 관측값이다.

`compute_line_extra_spacing`은 저장 줄에서 말미 구분 공백을 제외하면서 합성 줄에서는
포함했다. 합성 여부는 정렬 규칙을 달리할 근거가 아니다. 내부 공백 수와 가시 내용의 폭을
같은 결과로 계산하고 `emit_paragraph_line_runs`도 말미 공백의 배분 여분을 회수하게 보정한다.
구분 공백의 원문 소유와 자연 advance는 보존하며 내용을 삭제하거나 bbox를 clamp하지 않는다.
생산(가시 줄 폭·내부 공백 수) → 배분 extra_word_spacing → char_width_decision →
run bbox/후속 x 소비를 함께 확인한다. 실제 배치 고정 px 검사는 추가하지 않았다.

기존 `synthetic_wrap` 산술 테스트는 이 가정을 고정하므로 시각 선행 조건을 충족한 뒤
독립 정렬 관계로 재검토한다. Native/fresh WASM 전쪽과 #4956 쪽 여백 편집 대조군을 검증하며,
진단값만으로 보정 완료 또는 원 PR 수용으로 판정하지 않는다.

## 말미 구분 공백 보정의 시각 증거와 관계형 검사

source `ab5112e29a2a23535525e4fe1dc96936defbe4e1`의 #7816 합성 9개 및 #7824 대조군은
Native/fresh WASM 전쪽 최저 99.83377% 이상이고 쪽수가 Print와 일치했다.
`base_charpr0_20pt`는 두 경로 모두 100%였다. 새 fresh WASM package의 wasm SHA-256은
`8e2fa96106bb6dc660fc8092766f3f2140d92472ba227334779f2ded88a8bc13`이다.
fresh WASM 대표 review와 같은 영역의 확대 PNG에서 마지막 단어·본문 오른쪽 끝·줄 나눔을
직접 대조했다. 가시 글자 위치의 기존 말미 공백 배분 차이는 사라졌고 글리프 획 raster 차이는
남는다. 전체 문서 시각 일치의 수치를 글리프 획의 완전 동일성으로 해석하지 않는다.

정식 원본 `tests/cases/issue_synthetic_wrap_trailing_separator_alignment.rs`는 첫 긴 문단의
소유 줄 3개와 말미 공백 소유를 확인하고, 앞 두 줄의 마지막 가시 한글 glyph advance 끝이
소유 줄 오른쪽 끝과 맞는 관계를 검사한다. 절대 원점 대신 독립 Batang 1em advance와 줄 상자를
사용한다. #7701까지의 source `f22cdb0359cd0c0c400e1b0348c7f8c63f53be0c`에서 정식 원본을
실행해 해당 정렬 관계로 FAIL을 확인했다. 주 작업공간의 production source를 강제 재컴파일한
라이브러리에서 같은 정식 원본이 PASS했다(`trailing-separator-test-after-verified.log`).
이는 focused 검사이며 최종 통합 head의 전체 회귀·lint를 대신하지 않는다.

ignored 검토 output의 `integration-small-native-results-trailing-space.json`,
`integration-small-native-results-fresh-wasm.json`,
`integration-trailing-space-fresh-wasm-review/issue7816-base_charpr0_20pt/review/review_001.png`,
`integration-trailing-space-fresh-wasm-review/last-words-enlarged.png`와
`trailing-separator-test-before.log`에 연결한다. 전체 통합 검증의 남은 실패를 이 증거로
면제하지 않으며, #7816 기존 원점 pins의 관계형 전환도 별도로 완료해야 한다.

공유 target에서 중간 worktree를 빌드한 뒤 주 작업공간의 Cargo가 중간 라이브러리를
fresh로 오판했다. 최초 `trailing-separator-test-after.log`는 해당 중간 라이브러리에서
실행된 결과라 보정 후 판정에서 제외한다. `src/lib.rs`의 내용은 바꾸지 않고 mtime만
갱신해 재컴파일했다. 검증 라이브러리 SHA-256은
`cb86bf181478945ad6e50ab0fcdc2ec76a33ce8a4f36e544a1c350155091b0a8`이며,
`shared-cargo-worktree-stale-artifact.json`에 발견·교정 근거를 보존했다.
