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
