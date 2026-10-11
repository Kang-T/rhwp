---
kind: review
status: active
canonical: mydocs/manual/pr_review/collaborator_external_pr.md
last_verified: 2026-10-11
---

# PR #7698 — 메인터너 보정

## 범위와 승인

사용자가 메인터너 보정 후 기여자의 원 PR을 merge로 처리하도록 지시했고, 실제 문서는 공개 샘플 대조군을 사용하도록 선택했다. `maintainerCanModify=true`를 확인했으므로 원 contributor head 직접 보정 경로(9.3.1)를 사용한다. 원 commit을 rewrite하거나 별도 통합 PR로 대체하지 않는다.

원 head는 `1489ca1d95e3d6e6b888870d3b14cf240866ef00`, fork ref도 동일하다. 같은 가시성 branch `review/scari-pr7698-20261011`에서 미공개 reviewer 문서 commit만 원 head 위로 replay했다. 앞선 최신-base 진단 후보와 운영 기록은 `refs/review/pr7698/latest-base-diagnosis`에 보존했다. 원 PR에는 source head보다 최신인 devel 운영 기록을 복사하지 않았다.

## 원인과 보정 계약

기여자의 ViewText 선택·fallback·크기 제한·암호 검증 구현은 유지한다. 반환값이 암호나 파생 키가 아니라 문단 레코드인 helper를 `decode_viewtext_paragraph_stream`으로 명명하고 반환 계약을 설명한다. 실제 password 인자·암호 구현·CodeQL query/language/path 정책은 바꾸지 않는다. 실제 반환값과 기존 로그의 의미를 명확히 하는 보정이며, 새 SARIF에서 효과를 확인하기 전에는 CI 해결로 판정하지 않는다.

기여자의 `tests/cases` 집중 회귀 6건을 보존한다. 추가 검사는 실제 CLI의 strict/lenient 암호 ViewText 열기를 실행하고 stdout/stderr에 공개 시험용 credential이 나오지 않는지 확인한다. 이 검사는 로그 보존 계약의 대조군이며, helper 이름 변경 전 결함을 검출한다고 주장하지 않는다.

## 검증 계획과 진행

- 필수 fmt/native·WASM32·workspace Clippy, workspace build, manifest base 비교와 관련 집중 회귀를 순차로 실행한다. 공유 target은 `/home/edward/mygithub/rhwp/target/pr-review`다.
- Docker WASM을 빌드하고 실제 `openWithPassword`로 공개 암호 문서 및 ViewText 재현 구조를 확인한다.
- 공개 #5169 문서의 원 스트림·저장 LineSeg를 유지하는 암호 변형은 생성 방식과 한컴 수용/Print 출력, 원본 대조를 구분한다. 비공개 기업 원본이 확인된 것으로 쓰지 않는다.
- source push 전 fork/head를 재확인하고 LFS 사전 판독·dry-run을 수행한다. 새 head의 full CI/CodeQL을 확인한다. merge 직전 exact head·required checks·최신 base merge-tree를 재확인하며 merge 방법은 원 contributor commit을 보존하는 merge다.

실행 결과와 남은 범위는 완료 시 이 기록 및 [검토 기록](pr_7698_review.md)에 연결한다. 현재 로컬 수정 진행 중이며 원격 반영·CI 해소·merge 완료가 아니다.

## 메인터너 직접 판정 준비 (2026-10-11)

사용자는 시각 예외 승인 대신 직접 판정용 sample/Studio 준비를 지시했다. 따라서 시각 게이트 예외를 승인받은 것으로 기록하지 않는다. 보정 code SHA는 `f90c362511d46255c353bc8867401b769b12cdf4`이며 원격 push/merge 전이다.

필수 Rust lint 묶음·workspace build·base 고정 manifest 검사는 통과했고 집중 회귀는 7 PASS다. Docker `docker compose --env-file .env.docker run --rm wasm`도 통과했다. root `pkg/`와 Studio `public/`의 JS/WASM SHA-256이 일치하며 실제 CDP Chrome UI가 받은 WASM도 `0a34c4db42cab41126b4346dc991d3bd57514d7746994b35f4e44bffe51cd761`로 확인했다. 전체 nextest는 아직 실행 중이다.

공개 #5169 원본은 Native와 fresh WASM 모두 18쪽, 독립 한컴 2020 Print는 6쪽이다. 전체 Sweep 두 실행은 쪽수 불일치로 실패했고 전쪽 TSV·90% 통과를 주장하지 않는다. 공개 원본은 `samples/issue5169_viewtext_changetracking.hwp`, PDF는 `pdf/issue7674/issue5169_viewtext_changetracking-2020.pdf`다. 자체 암호화 변형은 한컴에서 거부돼 정상 fixture로 채택하지 않았다.

별도 암호 ViewText 대조군은 기존 공개 한컴 EncryptVersion 4 문서의 모든 stream을 byte-for-byte 유지하고 encrypted BodyText stream을 ViewText에 복제했다. 새 암호 연산·문단/LineSeg 변경은 없다. 생성기와 입력은 ignored `output/pr-review/pr7698-20261011/maintainer/clone-known-viewtext.rs`, `known_password_with_viewtext.hwp`다. 한컴 2024가 정상 열고 Print 64쪽을 생성했다(job `0253ae87-7eff-4e25-8ba1-7135ed03f8da`, build 13.0.0.3901, `frame_print_to_pdf_ex_one_up`, method 0). 이 대조군은 암호 ViewText 공존 수용 확인이며 비공개 기업 원본의 Section1 누락 재현으로 승격하지 않는다.

`http://localhost:7700/`에 Docker WASM을 반영한 Studio를 실행하고 CDP Chrome에 위 두 문서를 실제 file-input 이벤트로 각각 열어 두었다. 원본 18쪽·암호 대조군 64쪽 및 Canvas 표시를 확인했다. 실행·hash 증거는 ignored `maintainer/studio-preparation.json`, `studio-*.png`와 `browser-contract-result.json`이다. 브라우저 API 합성 8개 조합의 본문 선택·오답 거부는 통과했으며, 정상 암호 문서는 64쪽으로 열렸다. 합성/자체 비교 통과는 한컴 시각 통과가 아니다.

## #5169 1쪽의 삭제 이력 표시 결함 확인

메인터너는 원본의 1쪽부터 한컴에서 보이지 않는 텍스트가 rhwp에 나온다고 지적했다. 이를 단순 페이지네이션 차이로 설명한 앞선 해석을 정정한다. source `f90c362511d46255c353bc8867401b769b12cdf4`에서 원본 ViewText는 현재 내용과 삭제 이력을 함께 담고 있다. 독립 한컴 2020 변환 HWPX(job `51ee32e5-a33d-4936-b495-e8f55a69b0be`, version 11.0.0.9136)의 `deleteBegin/deleteEnd`는 HWP5 RangeTag type `0x11`·하위 24bit ID와 대응하며, header의 해당 `trackChange`는 `type="Delete" hide="1"`이다. 형식 추론만으로 삭제라고 단정하지 않고 실제 한컴 저장본의 독립 대응으로 확인했다.

| 1쪽 내용 | raw ViewText | 한컴 Print·독립 삭제 이력 대응 |
| --- | --- | --- |
| 표제 | 연구제안계획서 | `제안`: UTF-16 [2,4), RangeTag 0x11/ID 11 → Delete/hide=1. 한컴에는 연구계획서 |
| 이전 제목 | 무역구제제도분야 협상 관련 | 삭제 범위 [0,4), [4,6), [6,15) / IDs 17,1,17이 전체 문구·문단 끝을 포함. 한컴에는 표시되지 않음 |
| 날짜 | 20187. 436 | 삭제 [4,5), [7,9) / IDs 20,194. 한컴에는 2018. 6 |

실제 호출 경로는 `src/parser/mod.rs:796`의 ViewText 수용(압축 해제·첫 PARA_HEADER만 검사) → strict `:879`/lenient `:1008`의 본문 선택 → `src/parser/body_text.rs:316`에서 RangeTag 원값 보존 → `src/renderer/layout/paragraph_layout.rs:7976`에서 type=2 형광펜만 처리하며 삭제 범위의 표시·점유 의미를 적용하지 않는 것이다. 삭제 텍스트가 문단 IR·줄 구성에 그대로 들어가 페이지 공간도 점유한다. 단순 paint 은폐·좌표 clamp로 해결할 범위가 아니다.

이 비암호 경로와 ViewText 우선 정책은 검토 base `90fa2f8955c2dd21e9f0ea8fd0c6ee8dc5996a47`에도 있다. #7698은 이 기존 정책을 암호 경로로 확장했다. 기존 #5169 이슈/회귀는 전체 컨트롤 수(표 10개)로 ViewText 수용을 검사하지만, 변경 추적의 숨긴 컨트롤도 포함한 구조 개수는 현재 보이는 본문이라는 근거가 아니다. 따라서 `한글은 ViewText가 있으면 그것을 렌더한다`는 기존 문서·테스트의 표현을 표시 상태의 계약으로 재검토해야 한다. 기존 회귀·baseline은 임의 변경하지 않았다.

ViewText만 제외하고 다른 원 스트림을 모두 보존한 **진단 대조군**은 rhwp에서 6쪽이 되고 표제·제목·날짜가 독립 Print와 일치한다. 이 변형은 source 수정 또는 정상 한컴 생성 fixture가 아니며, 모든 문서에서 BodyText를 우선해야 한다는 일반 규칙/전쪽 시각 통과를 입증하지 않는다. ViewText의 변경 이력 보존과 최종본/검토 표시 정책을 구분해 보정 범위를 결정해야 한다. 특히 BodyText 없는 #7674의 경로를 없애는 방식은 허용되지 않는다.

진단 자료는 ignored `output/pr-review/pr7698-20261011/maintainer/diagnosis-5169/`의 `raw-stream-summary.json`, `cover-range-examples.json`, `hancom-track-summary.json`, `hancom-track-examples.json`, 원본 대응 `issue5169_hancom2020.hwpx`, 변형 `bodytext-only-control.hwp` 및 `bodytext-only-info.json`이다. 한컴의 변경 내용 보기 계약은 [공식 도움말](https://help.hancom.com/hoffice110/ko-KR/Hwp/view/toolbar/menu_review.htm)을 함께 대조했다.

전체 nextest 첫 실행은 8,121 PASS/1 FAIL/50 SKIP, 미실행 2,423건이었다. 실패는 이번 세션에서 만든 한컴 재저장 진단본을 active `samples/`에 둔 상태에서 기존 표본 전수 스캔이 새 파일의 겹침을 검출한 것이다. 신규 실험 파일 2개를 ignored `maintainer/rejected-experiments/`로 보존 이관했고(원본·기존 표본·검사·baseline 유지), `--no-fail-fast`로 전체 회귀를 다시 실행 중이다. 이 환경 교정과 제품 결함 보정은 구분한다. source push/merge는 아직 수행하지 않았다.


## 범위 확장과 구현 진입

사용자가 #5169의 숨긴 변경 이력 표시 결함도 이번 PR에서 해결하도록 지시했다. 삭제 범위가 숨긴 문단 끝·컨트롤을 포함하는 경우를 본문 선택 계약과 함께 처리한다. 한컴이 저장한 표시 상태가 독립 근거이며 문서 ID나 표 개수로 표시 본문을 선택하지 않는다. 같은 visible IR을 측정·Canvas·SVG·PNG가 소비하고 원 이력 보존과 저장 경로도 확인한다. BodyText가 없는 암호 ViewText 경로는 유지한다.

앞선 f90c362511 보정의 전체 nextest 재실행은 10,545 PASS / 50 SKIP다(`maintainer/full-nextest-clean-samples.log`). 이는 숨긴 변경 이력 수정 전 대조군이며 새 구현의 통과 증거로 재사용하지 않는다. 기존 #5169 회귀의 표 개수 기대값은 현재 가시성 계약과 독립 Print를 검증한 후 재검토하며, 90% 미달 상태에서 baseline이나 기대값을 갱신하지 않는다.


## 가시성 구현과 중간 검증

`TRACK_CHANGE`(DocInfo tag 96)의 순번 ID·Insert/Delete 유형·hide 값은 독립 한컴 HWPX의 229개 항목 모두와 대응했고 불일치는 0건이다. 이 설정과 RangeTag의 UTF-16 범위를 사용해 parser에서 visible paragraph stream을 생성한다. 숨긴 확장 컨트롤은 소유 subtree와 함께 제거하고, 숨긴 문단 끝은 같은 목록의 다음 문단을 이어받는다. PARA_TEXT 없는 char_count=1 문단의 암묵적 문단 끝도 같은 규칙을 따른다. 일반 빈 문단은 남기며, 삭제된 문단 끝 앞의 문단 모양은 이어받기에 보존한다. 원본 section raw와 DocInfo는 유지한다. 실제 숨김으로 UTF-16 축이 바뀐 문단 및 삭제된 문단 끝으로 이어붙인 문단의 줄 정보만 재조판하고, 가시 내용이 바뀌지 않은 문단의 유효한 저장 LineSeg는 보존한다. strict/lenient 모두 같은 projection을 거쳐 측정·각 backend에 전달한다.

원본 raw를 입력으로 같은 production projection을 실행한 진단은 최상위 문단 99개·표 4개로, 독립 한컴 최종 내용과 대응했다. 원본 BodyText와 visible projection의 재귀 문단 수는 모두 228개이며 완전 삭제·이어받기 후 표제와 삭제 문구의 표시를 확인했다. 이는 독립 Print 전쪽 시각 통과를 대신하지 않는다.

초기 글꼴 대조군은 6쪽이지만 1/3/4쪽이 90% 미달이었다. 실제 설치 face를 공급한 경우에도 남는 배치 차이를 글꼴 예외로 분류하지 않았다. 1쪽 표제의 첫·둘째 줄은 중앙 정렬 폭이 soft-wrap 말미 공백을 포함해 반 칸 좌측에 있었고, 독립 Print는 비가시 말미 공백을 제외했다. 마지막 줄 여부 조건을 제거해 같은 줄의 TAC·밑줄·전부 공백인 줄의 기존 표시 계약을 유지하며 중앙 정렬을 보정한다. 최종 head의 전쪽 Native/fresh WASM Sweep 및 회귀 검증은 진행 중이다. 첫 Clippy의 is_multiple_of 지적을 수정했고 Native Clippy와 release-test CLI build는 통과했다.


3·4쪽의 미달은 초기 구현이 같은 section의 **바뀌지 않은 문단**까지 저장 LineSeg를 제거한 탓이었다. 원본 ViewText의 `따라서` 문단은 숨긴 범위가 없고 저장 줄 시작은 0/39, `고위관리` 문단은 0/56이다. 실제 한컴 Print의 줄 나눔과 대응하므로 이를 review-mode 무효 정보라고 단정한 가정을 제거했다. 가시 축이 바뀐 문단·이어붙인 문단만 재조판한 production helper 진단 결과는 6쪽 모두 90% 이상(99.44/92.34/91.69/97.06/99.87/100.00%)이다. 변형 입력 진단은 최종 원본의 Native/fresh WASM 증적을 대신하지 않으며, 원본 기준 재검증을 이어간다.
