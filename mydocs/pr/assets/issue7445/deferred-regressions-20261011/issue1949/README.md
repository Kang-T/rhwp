# #7813로 이관한 거대 셀 편집 회귀 원문

사용자가 최초 체리픽의 보정 범위 밖이며 이미 #7445에 이관된 검사를 제거하도록 지정했다. 기존 [#7813](https://github.com/edwardkim/rhwp/issues/7813)의 실제 parent가 #7445이고 본문에 아래 네 함수가 명시돼 있음을 API로 재확인했다. 새 이슈를 만들지 않는다.

이 네 함수는 이번 체리픽에서 새로 추가한 검사가 아니다. 원본 문서는 contributor의 영향 관측 목록에도 등장하지만, 그 사실을 별도 편집·캐시 회귀 보정의 추가 승인으로 간주하지 않는다. 앞서 이 문서를 현재 보정 보호 목록에 묶었던 분류는 이번 지시로 정정했다.

검사 원문은 [failed-wasm-tests.rs.txt](failed-wasm-tests.rs.txt)에 보존한다. helper와 전체 소스는 변경 전 `6416f23304942706d9747396d73b2b585104de68`의 `src/wasm_api/tests.rs`에 남는다. HWP/HWPX·Print PDF와 다른 API·캐시·합성 계약은 보존한다. 구현을 되돌리거나 baseline을 현재 값으로 완화하지 않는다.

- `issue2214_scoped_cache_coherence_preserves_transient_pagination`
- `issue2424_resumable_pagination_commits_only_after_final_fragment`
- `issue2424_resumable_delete_commits_only_after_final_fragment`
- `issue2424_new_edit_stales_old_job_and_sync_flush_restarts_latest_revision`

직전 source `406041dec89d09d9e3fe93cbe606d29ef2eb17dd`의 집중 nextest는24건 중22PASS/2FAIL이며, 이 네 함수 가운데 컷 변화 개수113/관측108 및 변경 페이지 목록 불일치가 실패했다. 이관은 원인 해결·시각 수용·최종 회귀 통과가 아니다. 115쪽 현재 Native 비교에도90% 미만이 남는다. #7813의 base 증거와 현재 branch의 새 컷 소유 문제를 구분하며, 해당 이슈에서 전쪽 Native/fresh WASM90% 이상·확대 판독·내용 소유/누락/중복 검증 뒤 관계형 회귀를 복원한다. 로그·TSV·중간 JSON은 ignored output에 보존한다.

네 검사만 사용하던 cut snapshot struct와 helper3개도 같은 원문에 보존하고 제거해 dead-code 경고를 남기지 않는다. 다른 검사 함수는 유지한다.
