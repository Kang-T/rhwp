---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7692 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **병합 칸 높이와 줄 상자**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `135e2b075a8bee4aec792b003e9fa89618c32b5c`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `fa6cc7eab488a5b800f97b4df7c3df0f094616c9` | `62888424d8a2663af40dabea4ba3c83181017243` |
| `6b11130813d4c9e9ee8cdd4f5e9dc2e215a788d5` | `fc0ede395feedcb45b4834077f14e9aedc62ae6c` |
| `135e2b075a8bee4aec792b003e9fa89618c32b5c` | `338856f43010d0e2d0f08d67b700397949e5e927` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

선언 높이 constraint 생산 → 공용 rowspan solver → 측정 row heights와 실제 layout row heights 소비를 확인한다. 양쪽 constraint에서 내부 행 간격을 같은 방식으로 제외해 #7832의 행 간격 계약을 보존했다.

## 독립 기대값과 검증 계획

내부 행 간격이 있는 span, 이미 채운 행의 초과 높이와 빈 행, 일반 그림 칸·강제 줄바꿈을 대조한다. 80168 HWP/HWPX의 표 뒤 문단 악화와 1342000 전쪽을 확인한다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.


## 원 PR의 최신 동기화 확인 — 2026-10-12

최신 fork head는 `0c7bede4a0780b22e43f170c25b6c116c50b6835`다. 기존 검토 head와 고정 BFA를 제외한 새 commit은
upstream/devel 동기화 merge 하나뿐이며 새 작성자 고유 commit은 없다. 기존 적용 고유
commit과 출처 표는 유지한다. fork는 solver에 내부 행 간격 함수를 전달한다. 현재 후보는 측정·배치 양쪽에서 constraint를 만들 때 같은 내부 간격을 먼저 제외하는 보정으로 동등한 행 합 계약을 유지한다.

combined merge diff와 graph 근거는 ignored
`pr-7692-latest-merge-combined.diff`, `source-pr-refresh-merge-review.json`,
`nine-source-pr-live-20261012.json`에 있다. 원 PR 상태는 OPEN이며 원격 devel은 BFA다.


## 고정 base와 전체 시각 미달 대조 — 2026-10-12

정확한 글꼴 공급으로467 production 전쪽 TSV를 재산출했다. HWP는157/157쪽,
Native/fresh 최저35.10490%·미달31쪽이다. HWPX는 Native/fresh 모두156/156쪽,
최저1.49475%·미달110쪽으로 최종 종료를 확인했다.
HWPX90쪽 review에서 rhwp의 앞 표 끝 「계획」과 Print의 조문 대비표가 같은 쪽에 있어
내용 소속이 다르다. 이 차이를 글꼴·DPI 차이로 분류하지 않는다.

같은 입력·독립Print·글꼴로 고정upstream baseBFA를 실제 비교했다. HWP37·93쪽은
최저35.08606%, HWPX10·90쪽은 각각7.23121%·1.47757%였다. 이 선택 쪽의 미달은
이번 원고유 commit에서 새로 생긴 것이 아니다. base의 전쪽·fresh를 실행한 것으로
보고하지 않는다. 기존 전체 피델리티 후속은 열린#7783에 있고#7445 본문 표에도 있다.
현재 미달이나 신규 실물 회귀 추가 조건을 그 과거 이관으로 면제하지 않는다.

원문·TSV·로그는 `current-regulatory-results-467-correct-face-final.json`,
`current-regulatory-results-467-exact-base-bfa-selected.json`,
`regulatory-hwpx-exact-base-bfa-results.json` 및 대응score 디렉터리에 있다.


## 새 실물 검사 후보 등록 보류 — 2026-10-12

고정BFA에 없던 incoming 검사를 전쪽 시각 선행 조건에 맞춰 활성 등록에서 보류했다.
보류 대상은 #7672의4건과 #7692의3건, 합계7건이다. 원문은 바이트와 해시를 유지해
[보존 기록](../assets/issue7445/incoming-unqualified-regressions-20261012/README.md)에 있다.
이는 기존 devel 검사 이관이나 출력 결함 해결이 아니다. 이전10241PASS는 이 후보들을
포함한 실행 결과이며 등록 조건까지 충족한 것으로 승격하지 않는다. 정상 대조·전쪽 시각
수용은 계속 필요하고, 최종 파생 suite 준비·필수 lint·전체 정식 검사 수는 재확인한다.


## 그림 칸의 before/after와 Print 글꼴 프로그램 재확인 — 2026-10-12

원본14행 표를 복제한 작은2쪽 대조군에서, 정확한BFA의 칸(1,1)은152.8px이고
현재4ce는147.5px였다. 현재값은 입력의 칸 여백·그림 offset·그림 높이 합과 일치한다.
이 대조군은 원본4쪽의 개선이나 정상 회귀 등록으로 승격하지 않았다.

사용 글자를 보완한 HBATANG 서브셋에도 `『』` 표시 차이가 남아 독립 Print의 실제
TrueType 프로그램을 추출·대조했다. Print의HCR Batang은1000em·낫표500advance이고
`/usr/local/share/fonts/hwp-convert-mcp-hanbatang/HANBatang.ttf`의 동일 글리프 윤곽선·
전진폭과 일치한다. HBATANG.TTF(한컴바탕)는1024em·낫표1024advance로 **다른 face**다.
Unicode상Wide라는 분류로 이 글꼴의500advance를 전각으로 바꾸지 않았다.

올바른 HCR원 프로그램을 바이트 그대로 공급하고 current Native/fresh 전쪽을 다시
산출했다. 작은2쪽 대조군은 양쪽 최저94.56671%로90% 이상이지만 첫 표 원점·글리프의
남은 직접 판독이 필요하여 정식등록을 계속 보류한다. 원본4쪽은Native 최저39.21772%,
fresh 최저36.90483%, 각각1·2쪽이90% 미만이며 전체쪽수4/4다. 원본을 대신 새 입력만
통과시킨 것으로 보고하지 않는다.

ignored 증거: `owned-normal-picture-before-after-cell.json`,
`picture-corner-quote-independent-program-evidence.json`,
`validated-print-hcr-batang-program/provenance.json`,
`owned-picture-correct-hcr-{native,wasm}-{native,wasm}-results.json`.
실행 중인 미주 검증의 기존 글꼴 파일은 변경하지 않았고 올바른HCR공급 경로를 따로
고정했다. 작은 입력의 원본과 독립Print·원본4쪽의 기준은 각각 유지했다.
