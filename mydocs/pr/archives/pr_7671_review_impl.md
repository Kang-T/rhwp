# PR #7671·#7673 통합 호환 보정 — ZIP 9 엔트리 이름 API

## 원인과 변경

원 PR의 회귀 입력 생성 함수는 `ZipFile::name()`을 문자열로 사용했다.
현재 기준 `2efefc6a1f9812a188e32c1cb283e9ef7962ea3e`의 ZIP 9에서는
반환형이 `Result<Cow<str>, ZipError>`이므로 두 소스의 비교와 `start_file` 호출,
총 네 곳에서 E0369/E0277 컴파일 오류가 발생했다.

- `tests/cases/auto_number_after_control.rs`
- `tests/cases/new_page_column_definition.rs`

네 호출에서 엔트리 이름을 `expect("HWPX ZIP 엔트리 이름")`으로 확인한다.
입력 XML 변경, 자동 번호 위치·내용, 단 너비·방향·구분선 assertion은 유지했다.

## 확인한 증거와 남은 검증

동일 생성 함수를 복사해 실행하는 로컬 진단 helper는 변경 전 네 컴파일 오류로 실패했고,
변경 후 19개 입력을 생성하며 종료 코드 0으로 완료했다.
이 결과는 정식 regression suite 실행을 대신하지 않는다. 정식 focused 실행과 lint,
한컴 Print 및 Native/fresh WASM 시각 검증은 이어서 수행한다.

로그는 ignored `output/pr-review/semanticist21-20261015/logs/`의
`visual-input-generation-20261015.log`,
`visual-input-generation-after-zip-compat-20261015.log`에 보존했다.
최종 판정은 개별 PR review에 연결한다.
