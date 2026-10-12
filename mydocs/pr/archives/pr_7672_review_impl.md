---
kind: evidence
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-12
---

# PR #7672 체리픽·메인터너 보정 검토 계획

## 상태와 검토 범위

검토 진행 중이며 수용·승인 판정은 아직 하지 않았다. 범위는 **미주 제목 소유 경계**다.

- branch: `review/planet6897-followup-20261011`
- 고정 base: `bfa28a77a2714482f647ee6a7b9aa7f7876d8404`
- 원 PR head: `b387d1358256c0ae02f68b8c2a497026c3f0d1e9`
- 기본 경로: `collaborator_external_pr` 9.1.1, 주 작업공간 체리픽 통합
- 보조 경로: intake, local validation, visual fixture, multi PR, rework

| 원 고유 commit | 적용 commit |
| --- | --- |
| `d9976901da859be26367b32f5220efa01618de5c` | `72fb611e6cfbbac2526447639d0c8396f0030d54` |

원 fork history를 보존하고 중복 선행 commit·merge commit은 다시 적용하지 않았다.

## 구현 주장과 확인할 경로

미주 등록 범위 → 첫 문단 소유 조회 → 미주 측정·배치의 제목 여부를 추적한다. #7832의 미주 범위 검사와 새 첫 문단 검사를 함께 보존했다.

## 독립 기대값과 검증 계획

같은 미주 본문의 문 접두사·다른 접두사 편집 대조군을 저장하고 같은 편집본의 독립 Print를 확보한다. 원본 출력 무변화로 편집 경로를 비해당 처리하지 않는다.

최종 통합 source의 해당 lint·회귀·Native/fresh WASM 전쪽 TSV·대표 review 확대 판독이
완료되기 전에는 통과로 판정하지 않는다. 기여자 PNG·기여자 CI는 최종 통합 head의 증적을
대신하지 않는다. raw TSV·중간 JSON·로그는 ignored
`output/pr-review/planet6897-followup-20261011/`에 보존한다.

입력/Print SHA, 실제 source SHA·명령·관측값과 `충족 / 미충족 / 미검증 / 비해당`
판정은 실행 결과로 이 기록을 보완한다. 필요한 오늘할일은 최종 검토 묶음과 함께 통합 PR에
포함하며 별도 기록 PR을 만들지 않는다.


## 편집본 출처 재확인과 시각 실패 — 2026-10-12

첫 편집본을 current Native CLI `467bfd1d8`로 같은 원본·같은 명령으로 다시 생성했다.
`2023-note348-mun.hwp`는 이전 `f083228cd` 생성본과 바이트 및 SHA-256
`29317054c045cdb4d4fbbd92a9a86ac1cd8c1bf81dda8873166c752de8b1516b`가 동일하다.
따라서 이전 CLI가 만든 잘못된 편집본이라는 가정을 시각 실패의 원인으로 사용하지 않는다.
기존 편집본에 대응하는 독립 Print는 계속 동일 입력의 기준이다.

AB production의 기존 전쪽 raster에서 1·15·20쪽 review를 만들어 직접 판독했다.
15쪽(37.26871%)에서 같은 미주 문항의 세로 간격 및 뒤 칼럼 시작 내용이 독립 Print와
다르다. 글꼴만의 차이·낮은 평균·과거 입력이라는 분류로 면제하지 않았다.
같은 입력의 current production Native 전쪽 재출력은 진행 중이다.

ignored output의 `pr7672-current-edit-input-byte-comparison.json`,
`pr7672-edited-native-scores-valid-subsets/pr7672-2023-note348-mun/pr7672-2023-note348-mun/review/review_015.png`,
`pr7672-current-467-native-sweep.log`에 연결한다. 기존 raster 판독과 current 재출력을
구분하며 미주 제목 소유 보정의 전체 시각 수용은 아직 미충족이다.


## 미주 높이 진단의 독립 대조 — 2026-10-12

동일 편집본의 2020 Print와 새 2024 Print 모두 전쪽 90% gate를 충족하지 않는다.
원본을 새 2024 엔진에서 그대로 인쇄한 출력과 편집본의 15쪽은 100% 일치하여,
문 접두사 편집·rhwp 저장 자체가 그 쪽 차이의 원인이라는 가정은 배제했다.
기존 원본 PDF와 새 Print는 별도로 보존했으며 어느 쪽도 버전 표기만으로 무효로
판정하지 않는다. 원본/새 Print 공통 HyhwpEQ 128글자와 Haansoft Dotum 245글자의
전진폭·분해 윤곽선은 동일하다.

frozen production `467bfd1d8`의 실제 미주 debug에서 22번 미주 p12·p26·p35의
formatter 높이는 저장 LineSeg와 일치한다. 빈 줄에 기본 12pt를 잘못 가산한다는
가정은 배제했다. 같은 미주 제목은 Print와 맞지만 이후 알짜 풀이 제목은 약15px,
따라서 문단과 다음 미주 제목은 약22.5px 아래로 밀린다.

독립 한컴 2024 재저장은 원본의 미주 LineSeg를 유지하면서 1,591수식 중60개의
common 높이를 변경했다(미주 수식54개). 22번 미주의 단순 prime 수식은1125→900HU,
prime+위첨자는1163→1050HU로 바뀌었고, 이 차이는 관측한 누적 전진 차이와 대응한다.
이는 원본·재저장본 모두 실제 한컴 생성본임을 유지한 대조다. 현재 AST intrinsic 높이가
새 한컴 높이와 같은 경우는54개 중33개뿐이므로 모든 수식을 AST 값으로 치환하는
구현은 채택하지 않았다. cache 수용·제어 크기·미주 누적과 실제 paint의 공유 경로를
더 확인해야 하며 전체 시각 수용은 계속 미충족이다.

ignored 증거: `pr7672-note22-height-debug.log`,
`pr7672-original-vs-edited-same-engine2024-print-p15.json`,
`pr7672-old-fresh-print-equation-font-probe.json`,
`pr7672-original-resaved-equation-common-comparison.json`,
`pr7672-equation-intrinsic-original.json`, `pr7672-equation-intrinsic-resaved.json`.


## 원 PR의 최신 동기화 확인 — 2026-10-12

최신 fork head는 `c70ad95b65653acd51a63a05d3dbbfbac883421b`다. 기존 검토 head와 고정 BFA를 제외한 새 commit은
upstream/devel 동기화 merge 하나뿐이며 새 작성자 고유 commit은 없다. 기존 적용 고유
commit과 출처 표는 유지한다. 첫 미주 문단 소유 조회와 BFA의 등록 원본 범위 판정을 함께 유지한 병합이다. 현재 후보도 두 판정을 구분한다.

combined merge diff와 graph 근거는 ignored
`pr-7672-latest-merge-combined.diff`, `source-pr-refresh-merge-review.json`,
`nine-source-pr-live-20261012.json`에 있다. 원 PR 상태는 OPEN이며 원격 devel은 BFA다.


## 저장 캐시 변경의 진단 한계 — 2026-10-12

독립 한컴 재저장본의 새 수식 크기를 쓰고, 변경된 수식 소유 줄의 저장 캐시를 그 차이만큼
조정한 진단clone을 실행했다(46줄,20쪽). 원본·독립 재저장 파일은 바꾸지 않았다.
같은 fresh 원본2024Print에 대한 진단14쪽89.36291%,15쪽37.20063%이며,
14쪽의 다음 문항 첫 내용과15쪽의 칼럼 시작 소속이 여전히 다르다. 정상 텍스트 줄로의
누적 차이 전파를 추가한 진단도14쪽 점수를 바꾸지 않았다.
수식 common 크기 차이는 근거 있는 원인 단서지만 이를 캐시 덮어쓰기만으로 해결했다고
판정하지 않는다. 이 진단clone을 원본 일치·새golden·정식fixture 근거로 사용하지 않는다.

증거는 `pr7672-resaved-equation-cache-diagnostic.rs`,
`pr7672-resaved-equation-cache-raster/metrics_014.json`, `metrics_015.json`,
대응review와 `pr7672-resaved-equation-cache-propagation-raster/`다.
