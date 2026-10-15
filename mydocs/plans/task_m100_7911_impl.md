# Task #7911 PDF2IR 제품 통합 구현계획

- Issue: [#7911](https://github.com/edwardkim/rhwp/issues/7911)
- 작성일: 2026-10-15 (Asia/Seoul)
- 상태: **구현계획 승인 대기**. 수행계획은 메인테이너 승인 완료.
- [수행계획](task_m100_7911.md), [1단계 조사 보고](../working/task_m100_7911_stage1.md)
- 브랜치: `task_m100_7911`; 조사 head `92df2f3cbdc7012f5e91635512ceda68debecfac`
- 고정 base: `origin/devel` `a5e88ad682c805e1d845502ddc9521b94ea8eccd`
- POC: `poc/pdf-ir` `6bc3847e0ff3e1302736914859903f3c9c51bb69`

## 1. 설계 결정

승인 요청안은 **공통 파서에 opt-in PDF frontend를 연결하는 직접 IR 컴파일**이다.

```text
PDF bytes + PdfImportOptions
  -> parser/pdf (추출, 좌표 정규화, 구조 인식, lowering)
  -> ParsedDocument { Document, PDF import metadata }
  -> DocumentCore 공통 로드/compose/paginate
  -> 기존 Native/WASM 렌더, HWPX/HWP 저장
```

제품이 POC executable이나 HWPX 직렬화·재파싱을 호출하지 않는다.
PDF 특유의 추론은 frontend 소유이고 renderer에 PDF 전용 좌표 보정 분기를 만들지 않는다.
질문/보기 중심의 `parser/ingest` 스키마는 범용 PDF frontend로 사용하지 않는다.

### 1.1 POC 이력과 모듈 소유

- 구현 승인 후 작업 브랜치에서 고정 POC head를 `--no-ff` merge하여 9개 이력을 보존한다.
  먼저 merge simulation과 변경 경로를 확인한다. devel에는 직접 merge하지 않는다.
- 원 POC `tools/pdf-ir-poc/`는 별도 workspace로 보존한다. root에 의존하는 POC를
  제품 dependency로 추가하면 순환이므로 제품 컴파일러는 `src/parser/pdf/`에 이관한다.
- 제품용 모듈은 `mod`, `extract`, `recognition`, `table_detection`, `table_hierarchy`,
  `table_topology`, `lowering`, `structured_lowering`으로 책임을 구분한다.
  POC의 probe·JSON snapshot·독립 table extraction 진단은 제품 hot path에 복제하지 않는다.
- 과거 실패 checkpoint와 출력 기록은 성공 증거로 승격하지 않는다. POC 동결 복제는
  비교 기준이고 제품과 두 구현을 계속 병행 개발하는 구조는 이번 목표가 아니다.

### 1.2 Feature와 의존성

- 새 `pdf-import` feature는 **기본 비활성**으로 시작한다. CLI/WASM 모두 명시해 빌드한다.
  활성 제품 경로를 실제 검증해야 하며, feature-off 빌드 성공만으로 완료하지 않는다.
- `pdf_oxide = "=0.3.78"`, `default-features = false`, optional dependency를 출발점으로 한다.
  root `Cargo.lock`에 의도적으로 반영하고 이후 검증은 `--locked`로 수행한다.
- `office_oxide`는 전이 의존성이다. POC의 직접 pin을 이유 없이 root에 추가하지 않는다.
  해석된 버전·라이선스·NOTICE와 불필요한 OCR/ML/Python/native feature 유입을 확인한다.
- WASM의 구버전 getrandom `js` feature 필요 여부는 feature tree와 실제 빌드로 확인한다.
  서버·Python·native PDF runtime·원격 OCR은 필수 의존성이 아니다.
- 기본 활성화, 외부 비신뢰 PDF에 대한 안전성 보장, 배포 번들 자동 채택은 별도 판단이다.
  opt-in은 자원 검증을 면제하지 않는다. 확인되지 않은 안전성을 홍보하지 않는다.

## 2. 파일별 변경과 API

| 경계 | 예정 변경 | 보존할 계약 |
| --- | --- | --- |
| `Cargo.toml`, `Cargo.lock` | optional PDF dependency·feature | 기존 기본 feature와 workspace 유지 |
| `src/parser/mod.rs` | `FileFormat::Pdf`, `%PDF-` 감지, PDF metadata 및 분기 | 기존 parse API의 기본 동작·raw 봉인 |
| `src/parser/pdf/` | `PdfImportOptions`, 직접 IR 생성, 제한·오류·진단 | 공통 Document 모델만 반환 |
| `src/model/provenance.rs`, `document.rs` | `SourceFormat::Pdf`와 올바른 profile 분류 | PDF를 Hwp5/Hwpx 출처로 가장하지 않음 |
| `src/document_core/commands/document.rs`, Core 필드 정의 | PDF 옵션 진입점, 공통 parsed-document 로드 helper | fixup·compose·paginate·봉인 순서 유지 |
| `src/wasm_api.rs` | PDF source 이름, 명시적 import·diagnostics | 기존 생성자와 HWP/HWPX 입력 유지 |
| CLI dispatch/commands/document_io/metadata | `import-pdf`, 기존 입력 경로의 PDF 지원·안내 | 종료 코드·stdout JSON·원자적 저장 규약 |
| 모델/파서/`serializer/hwpx/shape.rs` | 필요한 경우 명시적 0 글상자 여백 보존 | 원본 부재와 0을 일괄 동일시하지 않음 |
| `tests/cases/`, 기존 WASM harness | 제품 경로 계약·회귀 | 기존 suite 수집 정책 유지 |
| CLI·빌드·지원 형식 문서 | 사용 예, 제한, opt-in 상태 | 미지원 범위를 지원으로 표시하지 않음 |

새 enum variant/public struct field가 외부 exhaustive match/struct literal에 미치는 영향을
확인하고 변경 기록에 알린다. 기존 함수 서명을 불필요하게 바꾸지 않는다.

### 2.1 Parser와 Core

- 기존 `parse_document*`와 `DocumentCore::from_bytes`는 feature-on PDF의 **전체 쪽**을 연다.
  feature-off에서도 PDF를 감지하고 `PDF_IMPORT_DISABLED`로 명시적으로 거부한다.
- 옵션 진입점은 `parser::pdf::parse_pdf_with_options(bytes, &PdfImportOptions)`와
  `DocumentCore::from_pdf_bytes(bytes, &PdfImportOptions)`를 추가하는 안이다.
  옵션 경로도 기존 경로와 동일한 provenance 봉인·Core 초기화 helper를 소비한다.
  기존 Core의 큰 initializer를 PDF용으로 복사하지 않는다.
- `ParsedDocument`와 Core에 별도 PDF import metadata를 둔다. HML의 메타데이터 분리 패턴을 따른다.
  공통 Paragraph에 PDF parser 객체를 삽입하거나 원본 PDF 전체를 진단 JSON에 복사하지 않는다.
- 공통 IR이 요구하는 UTF-16 char count·control mask·참조 무결성을 producer에서 맞춘다.
  HWPX 전용 로드 fixup을 얻으려고 PDF를 HWPX로 표시하지 않는다.

### 2.2 CLI와 WASM

- 기존 `export-hwpx input.pdf output.hwpx`, `convert input.pdf output.hwp`, `info`/렌더 명령은
  공통 document loader를 통해 전체 PDF를 처리한다. feature-off 안내를 함께 갱신한다.
- 선택 가져오기는 `rhwp import-pdf input.pdf output.hwpx --source-pages 0,6-7 --json`을 제안한다.
  기존 CLI의 0-based 페이지 규약을 따른다. 이 옵션은 입력 쪽 선택이며 렌더 `--page`와 다르다.
  미지정은 전체 쪽, 잘못된 문법·중복·내림차순·범위 초과는 exit 2, 처리 실패는 exit 1이다.
  출력·검증의 기존 코드 3/4 의미를 재정의하지 않는다.
- WASM은 기존 생성자의 PDF 전체 열기와 정적 `HwpDocument.importPdf(bytes, options)`를 제공한다.
  `sourcePages`는 0-based 증가 배열, 생략은 전체 쪽이다. 결과 객체는 기존 렌더·저장 API를 쓴다.
  `getPdfImportDiagnostics()`로 source/output page mapping·경고·임시 문단 정책을 조회한다.
- 오류는 안정적인 PDF code를 포함하고 JS 오류에도 전파한다. 실패한 새 문서를 기존 세션에
  부분 설치하지 않는다. 한 Worker에서 실패 뒤 정상 입력을 여는 검사를 실행한다.
- 확장/Studio UI 전면 개편은 제외한다. 실제 제품 JS import로 실행하고 POC WASM을 대신 쓰지 않는다.

## 3. 페이지·문단·구조 계약

- 하나의 Document builder가 모든 선택 쪽의 font/CharShape/ParaShape/TabDef/BorderFill,
  BinData, instance ID를 소유한다. 쪽별 Document를 단순 concatenate하지 않는다.
- 초기안은 PDF 물리 쪽마다 원본 크기·원점에 맞는 Section을 생성한다. 작성 당시 section의 복원이 아니다.
  서로 다른 크기·빈 쪽·비연속 선택의 순서와 쪽 대응을 보존하고 의도치 않은 빈 페이지를 검사한다.
- 전체 가져오기는 atomic all-or-error다. 회전·CropBox 등 구현하지 않은 변환은 안정적인
  unsupported code로 거부하고 첫 쪽만 반환하거나 조용히 빠뜨리지 않는다.
- 표 확정은 기존 EFSM과 canonical boundary contributor, 위상 검증, 부모 셀 포함·소유권 검증을
  함께 사용한다. 원본 획은 소유한 표 테두리 또는 fallback paint 중 한 번만 출력한다.
- 모호한 가족은 `Deferred`로 진단하고 원래 visual content를 보존한다. table 검출 실패와
  PDF page import 실패를 구분한다. 3쪽의 Deferred를 임의로 Complete로 바꾸지 않는다.
- 자식 materialization 실패 시 가족의 스타일·탭·이미지 풀을 rollback한다. 원본 내용의 중복·누락을 검사한다.
- 문단은 일단 **관측한 줄 단위 임시 Paragraph**로 만든다. 원본 논리 문단/읽기 순서를 완전 복원했다고
  주장하지 않는다. source page·bbox·관측 줄 소속·추론 사유를 bounded metadata에 남긴다.
- LineSeg는 PDF 물리 관측에서 생성한 layout 결과이며 한컴 저장 캐시가 아니다.
  좌하단 PDF 좌표를 페이지/셀 기준 좌상단으로 한 번 정규화하고 pt→HWPUNIT·baseline/height를 함께 생성한다.
  측정과 실제 배치가 같은 줄/객체 점유 결과를 소비하는지 최종 원점 선택까지 추적한다.
- 물리 문단 여백은 POC R3의 `2 * hu(points)` 계약을 보존해 CHAR 플래그가 섞이지 않도록 한다.
  탭은 TabDef와 실제 저장 폭을 함께 기록한다. inline 표 호스트의 baseline·여백을 별도 추측하지 않는다.
- 임의 clamp·겹친 텍스트 숨김·저장 LineSeg 일괄 삭제로 출력만 맞추지 않는다.

### 3.1 HWPX 후처리 제거

현재 serializer는 글상자 네 여백이 0이면 `textMargin`을 생략한다. POC는 ZIP XML 후처리로
명시적 0을 넣어 한컴의 기본 여백 적용을 막았다. 직접 IR만 반환하면 이 처리가 빠진다.

제품은 부재와 명시적 0을 구분하는 모델 표현을 도입하고 parser/serializer에서 보존하는 안을 따른다.
구체 필드 이름은 `text_margin_explicit` 등 기존 모델과 가장 작은 변경으로 정한다.
기존 생성·파싱 지점의 기본값 의미를 먼저 확인한다. 모든 기존 글상자에 무조건 0을 쓰지 않는다.
이 공통 변경은 별도 commit과 HWP/HWPX/HML 대조군으로 검증한다.
공통 모델 확장이 예상보다 넓거나 저장 의미를 바꾸면 구현계획을 보완하고 승인을 받는다.

## 4. 자원·오류 게이트

현재 추출 후 카운터는 parser 내부 선행 할당의 안전장치가 아니다.
검토한 0.3.78은 decoder 호출 후 크기 검사 경로와 Brotli `read_to_end`가 있으며,
Flate 개별 제한이 있어도 누적 decoded budget을 증명하지 못한다.
`open_with_config`의 `_config`도 이 경로의 유효한 bytes/options 제한 API가 아니다.

| 범위 | 초기 예산/정책 | 검증 위치 |
| --- | --- | --- |
| 원본 | 최대 16 MiB, 전체 128쪽 | bytes 복사 전 / page count 확인 직후 |
| 페이지 객체 | spans 20,000, images 32, paths 20,000 | 가능하면 추출 생성 중, 추출 후 검사와 구분 |
| 구조 | table 후보 128, 셀 4,096; 중첩 깊이 상한 | 후보 확장·격자/행렬/재귀 처리 전 |
| decoded 내용·이미지 | 문서 누적 budget, 이미지 width*height 및 bytes checked arithmetic | decoder/이미지 대규모 할당 전 |
| 진단 | 항목 수·문자열 길이·총 bytes 상한 | 수집 전, 잘린 경우 별도 표시 |
| CPU/취소 | 단계별 시간 측정, Worker deadline/취소 | browser Worker; heap 한계와 별도 |

위 숫자는 초기 POC 정책을 계승하는 제안이지 성능·안전성의 실측 보장이 아니다.
구현 단계에서 누적 decoded bytes/이미지 픽셀/깊이의 수치와 실제 적용 지점을 확정해 기록한다.
상한에 맞는 오류를 반환하고 일부 결과를 성공으로 반환하지 않는다.
크기 산술 overflow, 다단 filter, 압축 확장, 과다 객체·후보·깊이를 경계 입력으로 검사한다.

외부 crate의 할당 전 제한이 공개 API로 불가능하면 **배포 준비 판정을 보류**한다.
필요한 최소 upstream/fork patch의 범위·라이선스·회귀를 먼저 보고하고 구현계획 변경 승인을 받는다.
opt-in 표시나 Worker 종료를 메모리 안전성 입증의 대체로 삼지 않는다.
프로세스 전역 환경변수를 요청마다 바꿔 제한을 구현하지 않는다.
PDF JavaScript·첨부 실행·외부 링크 fetch·시스템 글꼴 설치는 하지 않는다.
암호 PDF는 이번 범위에서 명시적으로 미지원 처리하고 HWP 암호 오류로 오분류하지 않는다.

## 5. 검증과 증거

정본은 [local validation 4.3](../manual/pr_review/local_validation.md#43-변경-범위별-기본-검증),
[시각 증거](../manual/pr_review/visual_fixture_evidence.md),
[한컴 Print 계약](../manual/mcp_hwp2024Convert_usage.md)이다.

1. **작은 계약:** format 감지/feature-off, 손상·암호·예산·실패 후 정상 열기, 빈/다중/선택 쪽,
   resource ID 충돌, 실제 표·중첩 소유권, 원본 괘선 단일 소비를 제품 경로로 실행한다.
2. **실물 선행 시각:** 원본 PDF 전체 12쪽과 7/8쪽 분리본, B본/단일 표 대조군을
   Native/fresh 제품 WASM으로 재출력한다. 원본 PDF는 PDF frontend의 물리 관측 기준이며,
   생성 HWPX/HWP의 실제 재조판은 버전에 맞는 독립 한컴 Print로 별도 비교한다.
3. **회귀 수집:** 전쪽 90% 이상·쪽수·직접 판독 선행 뒤 실물 관계 검사를 `tests/cases/`에 이관한다.
   논리적 소유·내용 순서·누락/중복·포함/겹침을 검사한다. POC 71개 결과를 제품 통과로 복사하지 않는다.
   source-side cfg test나 자동생성 suite 파일을 그대로 이관하지 않는다.
4. **최종 Rust 게이트:** suite manifest prepare/check, fmt/check, default 및 `pdf-import`의
   Native/WASM Clippy, workspace build와 all-targets Clippy를 실행한다. feature-off도 유지한다.
   긴 전체 회귀는 focused/시각 결과를 보고하고 별도 승인 뒤 정본의 nextest release-test,
   unit tiers, 관련 Native Skia 게이트를 실행한다.
5. **fresh WASM:** 저장소 루트의 locked wrapper로 web bundle을 만들고 `--features pdf-import`를
   전달한다. 공용 target을 사용하고 실제 JS/WASM SHA·Worker 로드·다중 페이지 렌더/저장을 기록한다.
6. **회귀 비교:** 기준 devel과 head의 같은 기존 HWP/HWPX/HML 대조군을 비교한다.
   Native/WASM 어느 쪽이든 페이지별 0.01%p 이상 하락하면 90% 이상이어도 재검토한다.
   base가 PDF를 지원하지 않는 항목은 N/A로 기록하고 0점·통과로 꾸미지 않는다.
7. **비용:** feature-on/off WASM bytes와 gzip 크기, cold/warm 추출·인식·lowering·Core·저장 시간,
   측정 가능한 Native peak RSS/브라우저 메모리를 따로 기록한다. 메모리 측정 불가를 0으로 쓰지 않는다.

Cargo는 `/Users/edwardkim/mygithub/rhwp/target/pr-review`를 공용으로 순차 실행한다.
출력/명령 로그·입출력 SHA·전쪽 TSV·대표 PNG·Print job은
`output/pr-review/issue7911/`에 보존하고 commit하지 않는다.
기준 입력과 재사용 가능한 독립 PDF의 Git 포함은 별도로 확인한다.
회귀의 절대 px pin은 완료 페이지 Native/WASM 각각 98% 이상·직접 판독·±2px 조건 없이 추가하지 않는다.
회귀 실패를 보정하기 위해 baseline/ignore를 조용히 바꾸지 않는다.

## 6. 구현 순서와 승인

| 단계 | 산출물 | 다음 단계 조건 |
| --- | --- | --- |
| 1 (현재) | 호출 경로 조사, 이 구현계획, stage1 보고·commit | 메인테이너 구현계획 승인 |
| 2 | POC 이력 보존, optional frontend·공통 builder·단위/여백·자원 게이트 | focused 결과·stage2 보고·commit·단계 승인 |
| 3 | CLI/WASM 직접 연결·진단·도움말, fresh 제품 경로 실행 | focused/시각 결과·stage3 보고·commit·전체 회귀 승인 |
| 4 | 전체 회귀·비용·한컴 Print·최종 보고 | 메인테이너 최종 보고 승인 후 push/PR 생성 별도 승인 |

PR 경로는 `collaborator_self_merge`를 기본으로 `intake_and_review`, `local_validation`,
`visual_fixture_evidence`, `rework_and_exceptions`를 적용한다. 9,337줄 POC 도입을
작은 변경으로 취급하지 않으며 CI 통과만으로 merge하지 않는다.
PR 번호는 미리 정하지 않는다. 코드 head CI 성공 뒤에만 review·오늘할일 trailing 기록을 추가한다.
이미 생성한 오늘할일은 보존하되 이번 설계 단계에서 더 갱신하지 않는다.

본 계획 승인은 2단계 구현 착수 승인이다. push, PR 생성, merge, close, 기본 feature 활성화나
미검토 dependency fork 도입의 포괄 승인이 아니다. 현재 **PR 준비 완료 상태는 아니다**.
