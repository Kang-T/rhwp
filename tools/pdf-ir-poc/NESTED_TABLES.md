# 상태머신과 중첩 표 인식 POC

## 범위

2026-10-14, `poc/pdf-ir`의 로컬 실험이다. 제품 parser/renderer와 Cargo workspace는
변경하지 않는다. 기존 기본 표 인식 EFSM을 유지하고, 중첩 격자의 위상 검증과 셀 합성,
내용 소유권을 확장한다. PDF에는 작성 당시의 표 객체가 없을 수 있으므로 결과는 추론이다.

## 기본 표 상태머신

`Searching → Candidate → Validating → GeometryConfirmed → RecognizingCells → Complete`.
격자·소유권·계층 검증이 실패하면 `Deferred`로 보류한다. 완료 뒤 자식이 보류되면
`HoldSubtree`로 연결된 가족 전체를 보류한다. 표 안 문단은 별도 EFSM과 context를 사용한다.
자식 표는 부모 셀 문단 인식의 장애물이며 그 앞뒤 문장을 하나의 문단으로 합치지 않는다.

## Algebraic Topology 적용

1. 축 방향 실선의 연결 성분에서 닫힌 외곽·분할·병합 셀을 구성한다. 선이 없는 곳을 임의로
   메우지 않는다. 각 셀 둘레가 실제 원본 획으로 덮여 있는지도 확인한다.
2. 교점/분할점을 0-cell, 경계의 기본 구간을 1-cell, 직사각형 셀을 2-cell로 만든다.
3. `F2` 계수에서 `C2 --d2--> C1 --d1--> C0`를 계산한다. 각 면의 경계에 `d1`을 적용한
   결과가 0인지 검사한다. `d2` rank는 sparse XOR 소거로 계산한다.
4. 연결 성분과 rank로 Betti 수를 계산하여 채워진 표 복합체의 `(b0,b1,b2)=(1,0,0)`를
   확인한다. 괘선 그래프의 cycle rank와 채워진 복합체의 `b1`은 다르다.
   정상 격자의 여러 순환은 셀 면으로 채워지므로 복합체의 구멍이 아니다.

이는 유한 셀 복합체의 정합성 검사다. 닫힌 다이어그램도 같은 위상을 가질 수 있으므로
호몰로지만으로 문서의 의미가 표임을 증명하지 않는다. 분할선·행/열/병합 조건도 필요하다.
셀 체인과 호몰로지의 수학적 근거는 [Hatcher, Algebraic Topology 제2장](https://pi.math.cornell.edu/~hatcher/AT/ATchapters.html)을 따른다.

## Operad 관점의 셀 합성

표를 순서 있는 셀 슬롯들의 연산, 자식 표를 해당 슬롯에 대입되는 연산으로 취급한다.
자식 격자는 검증된 부모 표의 **단일 셀 내부**에 엄격히 포함되어야 한다.
가장 가까운 부모를 선택하고, 슬롯 번호와 정규화된 affine embedding `(x,y,width,height)`를 기록한다.
3단계 중첩도 동일 규칙을 재귀적으로 적용한다. 형제 표는 서로 겹치지 않아야 한다.

단위 정사각형 embedding의 합성 `A(B(u))`를 구현하여 항등과 결합 법칙을 검사한다.
이는 [May, Multicategories Associated to Operads](https://www.math.uchicago.edu/~may/PAPERS/MultCat1.pdf)의
합성 관점에서 착안한 **제약된 공간 대입 모델**이다. 일반 operad의 모든 공리·대칭 작용이나
operadic homology를 구현한 것으로 주장하지 않는다. 포함 관계만으로 표 후보를 생성하지 않는다.

셀 경계 접촉·교차, 부모 슬롯 불명확, 비층상 교차는 보류한다. 부모와 연결된 획의 중첩 격자는
현재 연결 성분 검출기가 분리하지 못할 수 있으므로 지원하지 않는다. 작은 좌표 차이를 clamp하여
억지로 슬롯에 넣지 않는다. `EPS=0.5pt`는 기존 괘선 정규화 허용치다.

## 소유권과 출력 계약

- text/image는 가장 깊은 단일 셀에만 배정한다. 부모에는 자식 표 참조만 남긴다.
- 자식 격자의 분할선을 가로지르는 내용은 부모 셀로 우회 배정하지 않고 모호성으로 남긴다.
- 페이지/표/셀 context가 각 계층에서 분리되며 max_context_depth를 초과하면 가족을 보류한다.
- 보고서의 부모 선언을 신뢰하지 않고 recognition에서 셀 기하를 다시 대조한다.
- **중첩 Table IR 객체화는 아직 보류**한다. `--tables`에서도 해당 가족의 원본 배치형
  글상자·그림·괘선을 전부 유지한다. 인식 성공과 한컴 조판 성공은 별도 판정이다.
- 기존 단일 표 객체화와 한컴에서 확인한 B본은 변경하지 않는다.

제한: 괘선 512개, 기본 tile 4096개, 표 후보 128개, 합성 깊이 16, 체인 incidence 65536개,
XOR 소거 작업 200만 개로 제한한다. recognition의 context 제한은 별도로 적용한다.
이는 parser 내부 할당·압축 폭탄·CPU 격리까지 보장하지 않는다.

## 검증

`tests/nested_tables.rs`의 합성 기하 입력으로 3단계 중첩, 형제, 병합 셀, 내용 단일 소유권,
그림 소유권, 경계 교차, 잘못된 부모, 깊이 초과, 위상적 구멍/비연결/중복 면, 자원 제한을 검사한다.
`fixtures/nested-grid.content`는 독립적으로 작성한 3단계 PDF paint program이다. 생성기는
stream length와 xref를 실제 byte 길이로 계산한다. Native/fresh WASM에서 같은 PDF를 추출하고
인식 보고서를 대조하며, 보류된 HWPX가 배치형 B 출력과 동일한지 확인한다.
이 자료는 합성 계약이지 한컴에서 작성·인쇄한 중첩 표의 대체 증거가 아니다.

실제 PDF 전쪽 인식 진단:

```sh
cargo +1.93.1 run --manifest-path tools/pdf-ir-poc/Cargo.toml --release --offline --locked \
  --target-dir target/pr-review --example nested_probe -- \
  pdf/issue5875/nested_table_text_caption-2020.pdf output/poc/pdf-ir/nested-probe.json
```

전쪽 인식 진단과 전쪽 HWPX 변환을 구분한다. 페이지별 미지원 오류를 결과에 남기며
성공 페이지만 전체 문서의 통과 증거로 삼지 않는다. 검증 결과와 미검증 범위는 README에 기록한다.
