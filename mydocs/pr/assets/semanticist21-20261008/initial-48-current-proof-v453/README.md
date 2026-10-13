# 초기 체리픽 원본 48개 현재 출력 검증

- production: `ddf17f86476293b114b6236bfa6cf1ebc9d9d474`; base: `552f5228d2dbfd6046c838f14241704f1bcb8a05`.
- 원본48개·전52쪽 Native/fresh WASM 최저99.62848%, 90% 미만·누락0, 각 문서 독립 한컴 Print PDF 쪽수 일치.
- 전체52쪽의 현재 Native/fresh RGBA와 이전 v396 Native RGBA가 동일하다. 현재24개 편집 입력과는 원본 SHA가 다르므로 별도로 확인했다.
- 현재 대표6입력의 review12개를 직접 읽었다. 머리말 서식·연속 표·원본 바깥 여백·빈 미주·6개 배경 무늬의 소유/경계/앞뒤 내용과 각주 번호를 확인했다.
- 같은 좌표 4배 확대3개로 머리말 이름/쪽번호/밑줄, 본문 위첨자 번호, 각주 구분선/번호를 대조했다. Native/fresh는 같으며 글자 획·래스터의 미세 차이는 남는다.
- 각 review/overlay의 파일명에 원본 SHA 앞12자와 backend를 보존했다. 실행 명령·입력/PDF SHA·전체 TSV는 ignored `output/pr-review/semanticist21-20261008/nested-frame-alignment-provenance-v453/initial-48-current-plan.json` 및 양쪽 결과/qualification에 보존한다.
- 표 바깥 여백 편집 입력2개는 [원본 입력 기록](../initial-outer-margin-inputs-v453/README.md)에 바이트 동일하게 보존했다.
- 전체 Rust10,267 통과와 별도로 남은 Skia/doc·가맹사업 fresh 전쪽·정식 관계 회귀 게이트 완료 후 승인한다.
