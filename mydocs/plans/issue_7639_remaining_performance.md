---
kind: plan
status: active
canonical: mydocs/plans/issue_7639_remaining_performance.md
last_verified: 2026-10-13
---

# #7639 남은 성능 개선

Issue: #7639

## 기준과 승인 범위

작업지시자가 #7639 남은 성능 개선 처리를 지시했다. base는 최신 devel `22317614b03722e95349a1d3d71c86923363c6f5`다. 원 기여자의 두 반복 검사 개선은 #7877에 반영됐고 추가 열린 관련 PR은 없었다. 담당자를 메인테이너로 지정하고 별도 local worktree에서 진행한다. 원격 코드 push/PR/comment는 아직 승인·실행 범위에 포함하지 않는다.

## 독립 계약과 구현 순서

1. 현행 코드를 실제 제품 진입점으로 측정하고 전쪽 SVG·쪽 소속을 보존한다. CLI open과 DocumentCore open/셀 한 글자 삽입은 별도 시간이며 동일 환경·입력·profile·횟수로 재측정한다.
2. `typeset/table/block/prepare.rs`의 rowspan_touched를 행마다 전체 셀 탐색하는 O(rows*cells)에서 셀의 시작/끝 이벤트와 행 prefix로 계산하는 O(rows+cells)로 바꾼다. 기존 `row_span>1 && start<=row && row<start+span` 구간을 정확히 보존하고 queried row_count는 MeasuredTable 범위를 사용한다. 빈/0span/범위 밖/겹치는 span도 기존 predicate와 같아야 한다. 이 단계에는 재사용 캐시가 없다.
3. 행 시작 셀·단일 행 컷 슬롯(안정적 col 순서)·rowspan 후보의 원본 source 순서를 구분한 읽기용 색인을 구성한다. 무효 선언·중복 col·원본 cell index도 기존 탐색의 의미를 유지해야 한다. 모든 span을 모든 행에 복제해 메모리를 키우지 않는다.
4. raw row heights와 rowspan shrink 재계산을 해당 불변 source snapshot 범위에서 공유한다. float 변환/합산 순서는 그대로 유지한다. 이후 줄/내용 조건은 원본 문단과 실제 normalization을 계속 읽는다.
5. 포인터 주소만으로 일시 clone/정규화 표를 섞지 않는다. source owner와 immutable snapshot의 실제 생명주기를 확인한 뒤 cache 경계를 정한다. 초기/편집 재조판, 문서 교체, 구조 편집, 설정 변경, Undo/Redo의 무효화 경로를 검증한다.

## 호출 경로

- rowspan_touched producer → 온전한 행 높이 선택·rowspan 행 보호 → 블록 준비/scan 예약 → 실제 분할/paint. 값과 배치 정책은 바꾸지 않는다.
- row_cut_cell_order → RowCut 슬롯 소속 → 컷 측정·실제 셀 배치·저장 프레임 writeback. col 정렬의 안정성과 원본 셀 인덱스를 보존한다.
- whole_fragment_row_uses_measured_height / row_uses_reflow_physical_frame → 예약 높이 선택·physical frame → scan/emit/layout. 새로운 예외나 좌표 보정 없이 탐색 범위만 좁힌다.
- get_raw_row_heights / rowspan_declared_overflow_shrink → native saved frame·source prefix·straddle continuation 수요 → 실제 cut/배치. helper의 조건·반환값·부동소수 합산 순서를 변경하지 않는다.

## 검증과 알려진 차이

초기 공개 샘플 `samples/issue2063_huge_cellbreak_table.hwp`는 current161쪽, 기존 독립 PDF 두 개는162쪽이다. 성능 결과와 한컴 조판 완료를 구분하며 사용자에게 기존 조판 차이의 처리 범위를 문의했다. 샘플 경로를 안내했고 아직 범위 선택은 받지 않았다. 이 차이를 허용치/기대 쪽수로 숨기지 않는다.

동일 전쪽 출력 비교는 성능 최적화의 의미 보존 증거이고 한컴 일치 증거를 대신하지 않는다. 원본·독립 Print·Native/fresh Docker WASM gate와 확대 판독, cache 반례, formal 회귀·세 Clippy 등 제출 gate는 별도로 충족한다. 현재 측정·로그·프로브는 기본 작업공간 ignored `output/pr-review/issue7639-perf-20261013/`에 보존한다.

## 첫 보정 관측과 색인 수명 계약

첫 rowspan event 보정은 span 순수 계약 2건과 전체 161쪽 SVG 바이트 동일성을 통과했다. API baseline(워밍업 제외 2회)의 open 10.5399–10.6043초 / insert 8.3774–8.4745초 대비 첫 보정은 open 10.5559–10.5647초 / insert 8.1658–8.1845초로 큰 개선을 입증하지 못했다.

색인은 RowMembers의 원본 순서와 안정적 col 정렬을 공유한다. 캐시에는 Weak만 두고, strong 소유자는 항상 해당 불변 Table borrow를 함께 보유하는 TableRows guard로 제한한다. 따라서 원래 Table이 소멸·이동·변경 가능해진 뒤에는 색인 strong 소유자도 없어 주소 재사용이 hit가 되지 않는다. Arc raw-height 값을 외부에 복제해도 원래 RowIndex를 소유하지 않는다. scope 밖은 원래 탐색으로 fallback한다. 별도의 주소 지문·문서 ID·첫/끝 셀 속성으로 같은 표를 추측하지 않는다.

scope는 블록 prepare, continuation 동기 drain과 각 step, 온전한 표 paint와 partial-table 실제 row-domain 선택 뒤에 연결한다. 임시 정규화·중첩 표의 기하 대상이 원본과 다르면 각 실제 Table에 별도 scope를 둔다. raw heights와 declared shrink는 원래 Table 메서드를 동일한 불변 borrow에서 lazy 1회 호출하며 pixel 변환·합산·정책 판정은 기존 consumer에 남긴다.

## 행 색인 보정의 관측

행 색인과 블록 준비의 행별 검사 두 곳을 연결한 phase3 Native build는 성공했다. 동일 입력·환경·release-test profile에서 워밍업 제외 2회 측정은 open 5.4427–5.4487초 / insert 4.1007–4.1080초다. baseline 대비 약 48% / 51% 감소지만 두 번의 국소 측정이므로 다른 기기·문서의 일반 성능을 보장하지 않는다. 삽입 시간은 DocumentCore 셀 편집과 재조판이며 저장·브라우저 paint 시간은 포함하지 않는다.

phase3의 전체 161쪽 SVG는 baseline과 페이지별 SHA-256 및 파일 집합이 모두 같다. 이는 기존 출력 보존 증거이며 독립 한컴 162쪽 출력과의 일치를 뜻하지 않는다. 증적은 ignored `output/pr-review/issue7639-perf-20261013/phase3/`의 `source-hashes.json`, `provenance.json`, `edit-probe.jsonl`, `svg-parity.json`에 보존한다. rowspan 순수 계약과 #6123·#6756·#7114 focused 검사 파일 4개는 모두 통과했다. 구조 편집·문서 교체·Undo/Redo, 전체 회귀, 세 Clippy, Docker WASM 및 독립 Print 시각 게이트는 아직 완료하지 않았다.

## Studio 지연과 수명 계약 보강

사용자가 요청한 devel `22317614b`의 Native Skia / Docker WASM 빌드는 별도로 성공했고 기본 작업공간에서 유지한다. 이 WASM을 Chrome 155 CDP의 실제 Studio에서 공개 샘플로 확인했을 때, helper의 문서 로드부터 Canvas 표시까지 23.908초였다. 두 번째 상세 계측은 22.094초이며 문서 로드 10.872초, 글꼴 대기 0.190초, Canvas view 준비 10.993초였다. Canvas 내부에서 요청 수집 0.188초, 응답 등록 0.0001초, `selectCanvasMetrics`의 재조판 9.445초, 실제 5회 layer tree 조회 합계 1.089초를 관측했다. 다운로드나 CDN 지연으로 오인하지 않는다. 이 helper 계측은 메뉴·대화상자의 모든 시간을 포함한 실제 사용자 클릭 벤치마크는 아니다.

경로는 `WasmBridge.loadDocument → HwpDocument(data) → 최초 조판`과 `CanvasMetricSession.prepare → selectCanvasMetrics → DocumentCore.rebuild_derived_state → Canvas 조판`이다. backend별 메트릭을 보존해야 하므로 두 번째 조판을 임의로 생략하지 않는다. 행 색인 보정은 두 경로가 공유하는 반복 탐색에 적용한다. 첫 측정·상세 측정 및 PNG는 ignored `output/pr-review/issue7639-perf-20261013/browser/`에 보존한다.

코드 검토에서 Drop glue만 있는 guard는 NLL이 마지막 필드 사용 뒤 Table borrow를 끝내도록 허용하여, Arc 소유자가 남아 있는 동안 수정하는 코드가 컴파일될 수 있음을 확인했다. 현행 호출은 불변 Table 인자를 동기적으로 사용하지만 제시한 일반 수명 계약에는 이 보호가 부족했다. TableRows에 명시적인 Drop을 추가해 strong owner의 종료까지 drop checking이 borrow를 유지하도록 보강했다. 실제 table_rows 모듈을 사용하는 독립 컴파일 프로브에서 기존 구조는 수정 허용, 보강 구조는 E0502, 명시적 guard drop 뒤 수정은 허용이었다. 이는 수명 계약 검사이고 조판 결함의 수정 전 FAIL 증거로 보고하지 않는다.

보강을 위해 첫 candidate Docker 컴파일을 직접 중지했다(exit 137; OOM 실패가 아님). 해당 로그는 phase3의 `lifetime-review-aborted-*`로 보존했다. 보강 후 Native Clippy는 통과했으며 phase4 Native·Docker WASM 재빌드와 성능·출력 보존 재확인을 진행한다. phase3 시각·시간 결과를 phase4의 완료 증거로 그대로 적용하지 않는다.

## 보강본의 실제 WASM 관측

phase4 Native / Docker WASM 빌드는 성공했다. 브라우저가 실제 받은 WASM 해시는 devel `7a12583cc041376ee2dcf8986dab451059e52a3669c5a47df4eb669233cccc32`, 보강본 `f97bae32f29de71797743d3961f96068098602541dbeca3cccae379cdc6527dd`이며 각 빌드 증적과 일치한다. 기본 작업공간 devel과 7700 서버는 유지하고 보강본은 별도 7796 Studio에서 실행했다.

| 관측 경로 | devel | phase4 보강본 |
| --- | --- | --- |
| 같은 Chrome 155 / Canvas2D에서 첫 표시(각 2회) | 23.775–23.910초 | 11.369–12.906초 |
| 첫 조판을 포함한 문서 로드 | 12.165–12.199초 | 5.593–6.722초 |
| Canvas 메트릭 선택의 재조판 | 9.478–9.493초 | 4.173–4.262초 |
| 같은 셀에 한 글자 삽입 API(1회) | 8.219초 | 3.935초 |
| 해당 API 삽입부터 view 재표시까지(1회) | 9.645초 | 5.336초 |

서로 다른 두 Native 실행의 API open 5.523–5.863초 / insert 4.051–4.167초도 다시 확인했다. 이는 사용자 메뉴·키보드 입력·IME의 모든 경로를 포함한 일반 성능 보장은 아니다. 소수 회 측정의 환경·JIT·font callback 편차를 고려하며, `fonts_ms`에는 글꼴 준비 중 일어난 Canvas 재조판 callback 시간이 포함될 수 있어 CDN 지연으로 해석하지 않는다. 메트릭 선택을 생략하거나 표시 backend를 바꾸지 않았다.

phase4의 전체 Native 161쪽 SVG 파일 집합과 페이지별 바이트는 baseline과 같다. 같은 원본 첫 페이지의 실제 Canvas 버퍼(1123×794)는 devel / phase4의 픽셀이 완전히 같았다. 앞서 element screenshot의 차이 6,352px은 문서 내용이 아닌 왼쪽 8열의 전체 높이 capture band에 한정됐으며 raw Canvas 비교로 구분했다. 이 결과는 출력 보존 증거이며 독립 한컴 162쪽 문서의 일치나 전쪽 Canvas 검증을 대체하지 않는다.

phase4에서 기존 focused 검사 4개 파일 / 10건이 모두 통과했다. 증적은 `output/pr-review/issue7639-perf-20261013/phase4/`와 `browser/`에 있으며 `build-provenance.json`, `source-hashes.json`, `svg-parity.json`, `focused-tests.json`, `*-open.json`, `*-edit.json`, `raw-canvas-parity.json`으로 연결한다. 새 원본 확인 탭은 `http://localhost:7796/?review=issue7639-candidate-review`다.

완료하지 않은 범위는 전체 Rust 회귀, 전체 Rust lint 묶음, 구조 편집·문서 교체·Undo/Redo의 추가 실행, 독립 Print에 대한 전체 Native/fresh WASM TSV 및 확대 판독이다. 기존 161/162쪽 차이의 처리 범위도 아직 확정되지 않았다. 이 상태에서 PR 생성·push·이슈 종료를 실행하지 않는다.
