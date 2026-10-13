# PDF → rhwp IR 로컬 POC

브라우저 확장/WASM을 목표로 PDF frontend를 실험하는 독립 Rust crate다.
제품 workspace, 루트 Cargo 설정, Studio bundle에 통합하지 않는다.

## 현재 체크포인트

- `pdf_oxide 0.3.78`에서 span/image/path를 추출하여 기존 rhwp Document IR로 lowering한다.
- 괘선 격자 검출기와 계층형 표/문단 상태머신이 소유권·후보·보류 근거를 남긴다.
- 기본 변환은 배치형 글상자·그림·도형이다. `--tables`는 geometry-confirmed 표 후보를
  실제 Table/Cell/Paragraph IR로 생성하며 불확실한 후보는 배치형 경로에 보류한다.
- 글상자는 명시적 0 여백과 단일 줄 SQUEEZE로 저장한다. 제품 serializer가 생략한 0 여백은
  POC 전용 구조화 ZIP/XML adapter에서 명시한다.
- `--painted-rules`는 flat-cap 축 방향 획의 물리 면적을 채운 Rectangle으로 표현한다.
- 2026-10-14 메인테이너가 기존 B본의 한컴 열기·조판 배치를 확인했다.
  기준 rhwp SHA: `552f5228d2dbfd6046c838f14241704f1bcb8a05`.
  확인 HWPX SHA-256: `a691dbb3309fb7f45372720bf51e59b058395efcf680040278f5bbc2b1817225`.
  기존 파일은 `output/poc/pdf-ir-20261013/pdf-oxide-hancom-v2/table-painted-page/compiled.hwpx`다.
  이 판정을 다른 source SHA, A본, 편집 후 저장·재열기에 자동 적용하지 않는다.

## 재현

저장소 루트에서 실행한다. Rust 1.93.1, wasm32 target, wasm-bindgen-cli 0.2.127,
Node, Poppler, Python의 Visual Sweep 의존성과 Studio의 puppeteer-core가 필요하다.
Chrome 경로가 macOS 기본값과 다르면 `CHROME_PATH`로 지정한다.

```sh
cargo +1.93.1 run --manifest-path tools/pdf-ir-poc/Cargo.toml --release --locked \
  --target-dir target/pr-review -- pdf/issue5875/nested_table_text_caption-2020.pdf \
  output/poc/pdf-ir/table-painted-page --painted-rules

node tools/pdf-ir-poc/verify-local.mjs "$PWD" /path/to/wasm-bindgen output/poc/pdf-ir
node tools/pdf-ir-poc/verify-visual.mjs "$PWD" output/poc/pdf-ir
```

`--painted-rules`를 생략하면 기존 Line을 유지한 A본이다. `--tables`를 지정하면 실제 표
객체를 생성한다. 두 옵션은 개별 모드이며 함께 지정하지 않는다. CLI는 입력 첫 페이지만 변환한다.
검증기는 텍스트 대조군과 표 입력을 Native/fresh WASM에서 재열고, source/input/module 해시와
실행 로그를 output에 남긴다. 독립 텍스트·raster는 원본 PDF에서 새로 생성한다.
원본 전체 12쪽과 선택한 첫 쪽을 혼동하지 않으며, rhwp 검사를 한컴 통과로 대신하지 않는다.

## 표 객체화 구현

- 괘선 격자의 행/열/병합을 Cell 주소, rowspan/colspan, 물리 크기 및 cell_grid에 연결한다.
- 상태머신의 셀별 문단 후보를 실제 Paragraph로 만들고, span 스타일의 UTF-16 위치를 보존한다.
  추가된 공백은 PDF glyph 간격으로 추정한 것이며 authored 공백 복원을 보장하지 않는다.
- 그림은 소유 셀의 inline Picture 문단에 넣는다. 셀의 첫 텍스트 앞 간격은 문단 속성과
  첫 LineSeg frame에 함께 기록한다. 그림 앞 간격은 앞 문단의 후행 간격 또는 첫 셀 패딩으로
  표현하여 그림 원점 이전에 소비한다. 측정/배치가 다른 간격을 중복 추정하지 않게 한다.
- 격자 선과 셀 안 글상자는 실제 표로 대체하므로 중복 출력하지 않는다. 셀 밖의 내용은 보존한다.
- 균일 실선의 색/굵기를 BorderFill에 연결한다. HWP의 굵기 단계로 양자화하며 혼합 스타일은 보류한다.
- colspan/rowspan, 빈 셀, 보류 후보, 실패 시 내용/스타일 보존 및 HWPX 왕복을 검사한다.
- 검증 표 입력: 실제 표 1개, 5행 3열 15셀, 세 번째 행/세 번째 셀 내부 그림 2개.
  원본 338자의 내용은 공백을 제외해 보존한다. 표 밖 제목 글상자 1개를 유지한다.
- 47개 POC 검사, Native/fresh WASM 보고서/PNG 일치와 독립 PDF 첫 페이지 비교를 실행한다.
  표 객체 출력의 실루엣은 Native/WASM 각각 92.37282%다. 글꼴/간격 차이는 남아 있다.
- **새 표 객체 출력은 한컴 미검증**이다. 기존 B본의 확인을 새 모드로 승격하지 않는다.
  재열기 조판/편집 후 저장 및 전쪽 변환도 별도로 확인해야 한다.

```sh
cargo +1.93.1 run --manifest-path tools/pdf-ir-poc/Cargo.toml --release --locked \
  --target-dir target/pr-review -- pdf/issue5875/nested_table_text_caption-2020.pdf \
  output/poc/pdf-ir/table-structured-page --tables
```

## 추적 범위와 후속 과제

Git에는 Rust/JS 소스, Cargo.lock, 이 문서만 보존한다. WASM/HWPX/SVG/PNG와 실행 manifest는
ignored output에 둔다. 검증·빌드 target은 저장소 공용 `target/pr-review`다.

후속 과제는 한컴의 새 표 객체 조판/편집 검증, 글꼴과 세부 간격 복원, 혼합 테두리,
borderless/중첩 표 및 원본 전쪽 변환이다. 기존 B본은 비교 기준으로 보존한다.

보안 한계: 입력/추출 결과 예산만 있으며 parser 내부 선행 할당·압축 폭탄·CPU 상한은
보장하지 않는다. 제품 통합 전에 Worker 종료 정책과 손상/공격성 PDF 검증이 필요하다.
