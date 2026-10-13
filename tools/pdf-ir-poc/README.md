# PDF → rhwp IR 로컬 POC

브라우저 확장/WASM을 목표로 PDF frontend를 실험하는 독립 Rust crate다.
제품 workspace, 루트 Cargo 설정, Studio bundle에 통합하지 않는다.

## 현재 체크포인트

- `pdf_oxide 0.3.78`에서 span/image/path를 추출하여 기존 rhwp Document IR로 lowering한다.
- 괘선 격자 검출기와 계층형 표/문단 상태머신이 소유권·후보·보류 근거를 남긴다.
- 현재 변환은 배치형 글상자·그림·도형이다. 검출한 표를 Table/Cell IR로 만들지는 않는다.
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

`--painted-rules`를 생략하면 기존 Line을 유지한 A본이다. CLI는 입력 첫 페이지만 변환한다.
검증기는 텍스트 대조군과 표 입력을 Native/fresh WASM에서 재열고, source/input/module 해시와
실행 로그를 output에 남긴다. 독립 텍스트·raster는 원본 PDF에서 새로 생성한다.
원본 전체 12쪽과 선택한 첫 쪽을 혼동하지 않으며, rhwp 검사를 한컴 통과로 대신하지 않는다.

## 추적 범위와 다음 단계

Git에는 Rust/JS 소스, Cargo.lock, 이 문서만 보존한다. WASM/HWPX/SVG/PNG와 실행 manifest는
ignored output에 둔다. 검증·빌드 target은 저장소 공용 `target/pr-review`다.

다음 단계는 geometry-confirmed 후보를 실제 Table/Cell/Paragraph IR로 연결하는 것이다.
셀 병합·그림 소유·읽기 순서·내용 누락/중복·직렬화·Native/WASM 배치를 확인하고,
판단이 불확실한 후보는 배치형 경로에 보류한다. 기존 B본은 비교 기준으로 보존한다.

보안 한계: 입력/추출 결과 예산만 있으며 parser 내부 선행 할당·압축 폭탄·CPU 상한은
보장하지 않는다. 제품 통합 전에 Worker 종료 정책과 손상/공격성 PDF 검증이 필요하다.
