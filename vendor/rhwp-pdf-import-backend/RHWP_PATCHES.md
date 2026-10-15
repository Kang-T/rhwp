# rhwp PDF Import Backend

이 패키지는 PDFOxide의 공식 release가 아니다. `pdf_oxide 0.3.78`을 기반으로 한
rhwp 내부 보안 패치 dependency이며, 원 저작권·라이선스·NOTICE·상표 정책을 보존한다.
`README.md`와 `upstream/Cargo.toml.orig`는 upstream 배포본의 기록이다.
소스 루트의 `Cargo.toml.orig`도 원문이지만 Cargo는 패키징 시 이 경로를 현재 manifest로
다시 생성하므로, 패키지 안의 원본 확인에는 `upstream/Cargo.toml.orig`를 사용한다.
코드 import 호환성을 위한 lib 이름 `pdf_oxide`와 별개로 package 이름은
`rhwp-pdf-import-backend`, 내부 버전은 `0.3.78-rhwp.1`이다.

## 원본과 추적

- 출처: <https://crates.io/crates/pdf_oxide/0.3.78>
- upstream repository: <https://github.com/yfedoseev/pdf_oxide>
- `.crate` SHA-256: `95b1ecc1a60a314fbd1aea5fe8f700ec32af1efa70fef5c6879162444da3437d`
- 위 hash를 기존 rhwp `Cargo.lock`과 대조한 archive에서 복제했다.
- archive에 없는 `rustfmt.toml`은 같은 upstream commit
  `ad49c4cb3638dc950a29350ef882161372fde473`에서 읽어 보존했다.
  루트 포맷 정책은 변경하지 않았다.
- `UPSTREAM_FILES.json`은 원본 archive의 파일별 SHA-256 목록이다.
  변경·추가 파일과 원문 보존을 이 목록으로 구별한다. upstream 갱신 시 보안 diff를 다시 감사한다.
- MIT 선택지로 사용하며 `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `TRADEMARKS.md`를 보존한다.
- 외부 배포 승인이 없으므로 `publish = false`다. 별도 게시·배포는 수행하지 않았다.

## 보안 패치 경계

- `PdfDocument::from_bytes_with_limits`에서 요청별 `ImportBudget`을 생성한다.
  bytes 입력과 xref bootstrap부터 같은 context를 사용한다.
- 파싱된 `StreamData`는 객체/캐시 clone을 지나도 원래 context를 공유한다.
  legacy writer가 생성하는 unbound bytes와 한 문서의 요청 budget을 구별한다.
- stream 단계와 문서 누적량을 append/reserve 전에 검사한다. Flate/Brotli/LZW,
  RunLength/ASCII, 다단 filter와 predictor를 처리하며 제한 초과를 부분 성공으로 만들지 않는다.
  Flate의 제한된 경로는 잘린 prefix 복구를 하지 않는다.
- 객체/배열/사전 깊이·개수, xref `/W`·`/Index`, object stream `/N`을 검사한다.
  제한된 constructor는 손상 xref의 전체 파일 재구성과 암호 PDF를 명시적으로 거부한다.
- content stream은 기존 operator parser를 요청 context로 실행한다.
  process-global cap을 바꾸거나 text prescan의 묵시적 truncation을 안전장치로 사용하지 않는다.
  수치 등 content operand는 컨테이너 객체 quota와 분리하고 operator당 64개로 제한한다.
  operand 내부의 배열/사전에는 동일한 중첩·컬렉션·문자열 제한을 적용한다.
- operator·graphics stack·문자열·표시 텍스트·span 예산을 검사한다.
- 실제 JPEG header/decoder 한계와 PNG writer를 검사하며, 정상 RGB JPEG에는 기존
  image decoder를 유지한다. CMYK 경로의 기존 반전·색 변환 의미를 보존한다.
- typed `ImportResourceLimit`과 sticky fatal 상태를 사용한다. rhwp frontend는 작업 경계에서
  이 상태를 다시 확인하고 최초의 정적 실패 사유를 보존한다. 다음 문서는 새 budget을 사용한다.
- `office_oxide =0.1.9`를 호환성 제약으로 고정한다.
- 제한된 ToUnicode CMap은 전역 cache를 우회하고 요청별 source/scan/확장 예산을 사용한다.
  lazy clone은 raw bytes와 parsed 결과를 Arc로 공유하며 원래 budget을 보존한다.
  비ASCII hex 입력을 byte 경계로 잘라 panic하는 경로를 제거했다.
- CID `/W`·`/W2`의 배열/범위 확장 전에 별도 font metric 예산을 소비하고,
  공개 FontInfo 반환 경계에서 fallback이 fatal 오류를 삼키지 못하도록 검사한다.
- page content 결합의 separator와 cache 반환 복사도 확장 전에 같은 요청 예산을 소비한다.
- 내부 Rust 의존성은 `rlib`만 생성한다. Cargo가 package/source별 hash로 산출물을 구별하게 하여
  공유 target의 비해시 `libpdf_oxide.rlib` 충돌을 피한다. C API·writer 소스는 보존하지만
  이 내부 package에서 독립 cdylib/staticlib 제품을 배포하는 계약은 제공하지 않는다.
- Rust 1.93에서 알려지지 않은 upstream lint 한 항목을 제거하고, 기존 boolean 식 한 곳을
  동치 식으로 정리했다. backend 자체의 `-D warnings` 검사를 위한 호환성 변경이다.
- 후속 미검증 보완: 요청별 페이지 트리 깊이 32, 페이지 노드/객체 로드 방문량 100,000,
  활성 객체 로드 수 32, Form 깊이 16, XObject 방문량 10,000 정책을 추가했다.
  page/object cache hit의 fatal 상태 확인과 tree fallback의 오류 전파를 보완했다.
  제한된 page count는 실제 tree를 순회하며 `/Count`와 flat-tree hint를 신뢰하지 않는다.
  텍스트 Form의 조용한 depth/개수 skip과 예산을 우회하는 span/stream cache 재사용도 막았다.
  수정 전 5개 계약 실패를 재현한 뒤 최초 재빌드는 디스크 부족으로 중단됐다.
  공간 확보 후 Count 기반 subtree skip도 보완하고, span/glyph의 반복 해석 작업량을 반영한
  총 44개 계약과 Native/WASM lib lint를 통과했다. 실물 Native HWPX도 보완 전과 동일했다.
  전체 탐색/cache 감사·제품 공개 API·비신뢰 입력 배포 준비 완료를 뜻하지 않는다.

## 제한과 남은 게이트

### 후속 캐시 보완

- span/character cache hit의 fatal 확인, span 보관·반환과 character 반환의 소유 payload
  복사 전 checked 크기 계상을 추가했다. 한 복사의 요청 payload 32 MiB와 문서 누적 decoded
  256 MiB에 포함하며 allocator overhead나 전체 heap 측정값으로 표시하지 않는다.
- Form stream은 path·image handle·legacy materialized image 소비자 간 Arc로 공유하여
  cache hit/삽입 때 decoded Vec을 복제하지 않는다. 기존 50 MiB 보관 정책은 유지한다.
- inline/XObject image handle의 raw compressed copy와 decode 진입/반환은 같은 요청 budget을
  확인한다. legacy 무제한 API의 함수 서명과 기본 동작은 유지했다.
- 수정 전 신규 4개 실패를 확인했고, 개별 복사 경계와 Form 반복 소비 대조군까지 포함한
  자원 계약 45개 및 기존 frontend/여백 5개가 통과했다. Native 실물 HWPX 3건도 동일했다.
  Native/WASM root·backend library Clippy가 통과했다. 전체 cache/codec/Worker 종료 증거가 아니다.
- 현재 rhwp Native/WASM feature tree에서 이 backend의 활성 feature는 없다.
  JBIG2/JPEG 2000은 unsupported stub이고 qcms/lcms2는 연결되지 않는다.
  추가 feature를 켠 소비자에게 이 감사 범위를 일반화하지 않는다.
- 최신 source의 fresh web WASM 진단 실행에서도 실물 HWPX 3건이 Native와 동일했고,
  비활성 JBIG2/JPX 오류를 포함한 실패 뒤 정상 입력을 처리했다. 제품 공개 API·browser
  Worker·시각 검증이나 자원 계약 45개의 WASM 실행 결과로 표시하지 않는다.

### 남은 검증

후속 CCITT DeviceGray 제한 경로는 기존 Group 3/4 행 해석기를 재사용하면서 전이 Vec의
초기/성장·packed row·grayscale 예약에 요청 예산을 적용한다. 압축/픽셀 길이가 같은
경우에도 압축을 해석한다. 알려진 이미지 높이의 모든 행을 읽어야 성공이며 예산 없는
fax fallback/흰 대체/부분 복구를 호출하지 않는다. 비제한 API의 기존 복구는 보존한다.
DecodeParms filter index와 참조, checked 정수 변환을 적용한다. 제한 경로는
Columns/양수 Rows와 이미지 geometry의 일치를 요구하며 폭 65,535 초과를 지원하지 않는다.
Rows 부재/0은 검증한 이미지 높이를 사용한다. 모든 유효 CCITT PDF의 수용을 뜻하지 않는다.
작은 반례 4개는 수정 전 FAIL, 보완 후 Native 자원 계약 60개/frontend·여백 5개와
Native/WASM의 root/backend library Clippy가 PASS다. 새 release/web WASM의 Node 실행도
CCITT 오류 6개를 거부하고 정상 Group 3/4의 12개 조합 및 1x1 사례를 독립 PNG 디코더로
픽셀 대조했다. Indexed CCITT/색 공간·mask/palette, 제품 공개 API·Worker·시각 검증과
전체 이미지/메모리 안전성 완료로 표시하지 않는다. 현재 아카이브의 독립 소비자 결과는
2단계 기록에서 구분한다.

후속 Brotli 보완은 `brotli-decompressor =6.0.1`의 custom allocator/stream API로
private scratch를 할당 전 계상한다. 별도 개별 할당 32 MiB와 문서 누적 작업 예산을
같이 적용한다. 초기 상태 할당 실패 확인·표준 window·부분 출력 실패 처리를 추가했다.
수정 전 작은 반례 2개는 FAIL이었다. 두 차례 ENOSPC 중단 뒤 승인된 캐시 9개만 정리했고,
현재 source에서 Native 자원 계약 50개/frontend 4개/여백 왕복 1개와
root/backend Native·WASM library Clippy가 통과했다. 빈 입력의 scratch는 의존성 소스에
따라 초기 table과 블록 table 두 개를 모두 포함하도록 검사 가정을 정정했다.
추가 다중 chunk·전체 잘린 prefix·부분 할당 실패 경계도 통과했다.

새 release/web WASM의 Node 실행에서 손상 Brotli 4개를 거부한 뒤 정상 변환을 수행했다.
정상/빈 Brotli 내용은 비압축 내용과 HWPX가 동일했고 실물 12쪽/7·8쪽 변환도 이전 정상
결과와 일치했다. 이는 직접 Rust frontend의 진단 probe다. Native 55개 계약의 WASM 실행,
공개 제품 API·Studio·Worker·시각 검증이나 총 메모리 상한 입증으로 표시하지 않는다.
현재 source의 packaged consumer 검증과 별도 crate 배포 준비 완료는 구분한다.

이 변경은 크기 최적화가 아니며 일반 PDF 쓰기/변환 기능을 삭제하지 않는다.
decoded/copy/구조 작업 예산은 총 RSS/heap 상한이 아니다. CMap source 1 MiB,
요청 CMap scan 작업량 64 MiB/매핑 100,000개, CID metric 확장 100,000개를 적용한다.
128-byte entry 작업 단위와 source 정규화 가중치는 allocator 측정값이나 RSS 증명이 아니다.
모든 codec의 내부 scratch, embedded font/대체 인코딩, 참조/Form 탐색 및
나머지 postprocess/cache의 선행 할당·누적 작업량 감사는 별도 종료 조건이다.
일부 경로만 통과한 결과로 안전성을 보장하거나 비신뢰 입력 배포 준비 완료로 표시하지 않는다.
rhwp import metadata의 `parser_allocation_bounds_proven`는 이 게이트가 끝날 때까지 false다.

루트 workspace는 이 package를 제외하고 optional path dependency로만 소비한다.
workspace patch에만 의존하지 않지만, crates.io 소비자를 위해서는 이 내부 dependency의
별도 배포 모델이 확정되어야 한다. `cargo package --list` 확인만으로 packaged consumer의
빌드나 배포가 검증됐다고 판단하지 않는다.
