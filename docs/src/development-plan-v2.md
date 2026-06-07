# EnergyPlus Rust Port 개발 운영 계획서 v2

> 목적: 기존 EnergyPlus 결과와의 정합성을 최우선으로 유지하면서, **Rust-only** 기반의 schema-native 입력, model compiler, 명시적 상태 관리, graph-first 검증, 자료구조 중심 최적화, structured diagnostics, reproducible release 체계를 갖춘 차세대 EnergyPlus-compatible 엔진을 개발한다.  
> 기준 문서 버전: 2026-06-04, v2  
> 초기 기준 oracle: EnergyPlus 26.1.0  
> 대상 개발 환경: Windows 중심. Linux/macOS는 초기 필수 대상이 아니다.  
> 코드 언어 정책: **프로덕션 구현 언어는 Rust로 고정한다. C/C++/Fortran/assembly kernel 혼합은 초기 계획에서 제외한다.**

---

## 0. v2 수정 요약

이 문서는 기존 v1 계획서에 다음 내용을 반영해 전체 구조를 재정렬한 버전이다.

1. **Rust-only 원칙 명문화**
   - 기존 EnergyPlus C++ source는 reference/oracle source로만 사용한다.
   - Rust port 내부에 C++, Fortran, assembly kernel을 섞지 않는다.
   - 성능 최적화는 Rust 자료구조, Rust compiler 최적화, safe Rust 우선, 필요 시 제한적 `unsafe Rust` 또는 Rust 표준 SIMD 경로로만 검토한다.
   - 기본 release path는 deterministic scalar compatibility mode이다.

2. **자료구조 중심 최적화 통합**
   - `RawModel → TypedModel → SimulationModel → ModelGraph → ExecutionPlan → SimulationState → ResultStore` 흐름을 핵심 구조로 확정한다.
   - 이름 기반 참조를 simulation runtime 전에 typed ID로 치환한다.
   - schedule/weather/output/component dispatch는 runtime string lookup 없이 handle/index 기반으로 바꾼다.
   - surface/zone/construction/radiation/HVAC/Plant 계산에 대해 `CalcView`, grouping, cache, execution plan 개념을 추가한다.

3. **Model compiler 접근 추가**
   - EnergyPlus-compatible engine을 단순 interpreter가 아니라 schema-native model compiler로 설계한다.
   - epJSON 입력을 raw object로 읽은 뒤, validation/default/reference/graph/execution plan까지 compile stage로 분리한다.
   - compile stage마다 test와 diagnostic을 둔다.

4. **성능 구조 개선 추가**
   - compiled schedule/weather, construction coefficient cache, radiation matrix/cache, component registry, output handle registry, dependency/cache invalidation graph, trace/profile store를 포함한다.
   - `compatibility`, `diagnostic`, `fast`, `experimental` 모드를 분리하되, release 기본값은 compatibility로 유지한다.
   - fast mode는 Rust-only 내부 최적화만 허용한다.

5. **milestone 재정렬**
   - 기존 milestone을 “기능 구현 순서”가 아니라 “정합성 검증 가능한 구조가 생기는 순서”로 재정렬한다.
   - 각 milestone마다 어디서 어떤 test를 수행할지 명시한다.

---

## 1. 이 문서의 역할

이 문서는 개발팀 전체가 공유할 기준 문서이다. 단순한 아이디어 정리가 아니라, 다음 항목을 하나의 개발 운영 기준으로 묶는다.

1. 프로젝트 목표와 비목표
2. Rust-only 코드 언어 정책
3. 기준 EnergyPlus version과 oracle 관리
4. portable EnergyPlus binary/source 관리
5. repo 구조와 crate boundary
6. model compiler 및 runtime architecture
7. 자료구조 최적화 원칙
8. 성능 개선 구조
9. setup 방식
10. CLI와 주변프로그램 재정의
11. 사용자 문서 및 개발자 문서 관리
12. 정합성 검증 전략
13. milestone과 release 전략
14. CI와 regression test 운영
15. issue 기반 test case 관리
16. 최종 release artifact 구성
17. 개발팀 작업 규칙

이 문서에서 말하는 “EnergyPlus 정합성”은 단순히 최종 `eplusout.csv` 값이 비슷한 수준을 의미하지 않는다. 입력 해석, schedule/weather 값, graph resolution, timestep state, component result, output variable/meter, annual aggregate를 단계적으로 비교할 수 있어야 한다.

---

## 2. 핵심 결론

이 프로젝트의 방향은 다음 한 문장으로 정의한다.

> EnergyPlus를 Rust로 다시 쓰되, 기존 EnergyPlus 26.1.0의 결과 정합성을 기본값으로 유지하고, 코드 구조는 epJSON/schema-native model compiler, typed ID 기반 internal model, explicit SimulationState, graph-resolved systems, structured ResultStore, traceable execution plan 방향으로 현대화한다.

따라서 다음 원칙이 적용된다.

```text
코드 언어:
  Rust-only

기본 목표:
  compatibility-first

입력:
  IDF-native가 아니라 epJSON/schema-native

실행 구조:
  interpreter가 아니라 model compiler + execution plan runtime

상태:
  global state가 아니라 explicit SimulationState

자료구조:
  string/object lookup이 아니라 typed ID, handle, cache, graph, compiled table

설비:
  object list가 아니라 graph-resolved air/plant/control model

출력:
  file-only가 아니라 ResultStore + legacy export

검증:
  output 비교만이 아니라 compile-stage 및 trace-level comparison

배포:
  PC 설치 EnergyPlus에 의존하지 않고 portable oracle 고정

성능 개선:
  Rust 자료구조, cache, execution plan, grouping, profiling 중심

개선 알고리즘:
  기본값이 아니라 experimental mode
```

---

## 3. 공식 EnergyPlus 기준 사실

이 섹션은 설계 판단의 근거로 둔다. 외부 사실은 프로젝트 시작 시점과 release 전마다 다시 확인해야 하며, release note에는 기준 날짜와 oracle version을 명시한다.

### 3.1 EnergyPlus 26.1.0

초기 기준 oracle은 EnergyPlus 26.1.0으로 고정한다. 기준 binary와 기준 source tag는 항상 같은 version이어야 한다.

```text
oracle binary:
  EnergyPlus 26.1.0 executable

oracle source:
  EnergyPlus v26.1.0 source archive

목적:
  binary = 결과 baseline 생성
  source = 알고리즘, 변수, testfile, unit test reference 확인
```

### 3.2 epJSON 방향

EnergyPlus 문서에는 입력 구문이 IDD/IDF에서 epJSON으로 이동 중이고, schema가 장기적으로 fundamental input schema가 될 것이라는 방향이 제시되어 있다. 따라서 Rust port의 core input은 IDF가 아니라 epJSON/schema-native이다.

```text
IDF:
  legacy import target

epJSON:
  primary input format

RawModel:
  epJSON 원형 보존

TypedModel:
  Rust struct/enum/typed ID 기반 simulation-ready model
```

### 3.3 API/state 방향

EnergyPlus API는 state object를 중심으로 이동하고 있다. Rust port는 이 방향을 더 엄격하게 적용한다.

```text
EnergyPlus 방향:
  managed state
  API state object
  callback state object

Rust port 방향:
  immutable Model
  mutable SimulationState
  explicit CacheState
  no mutable global state
```

### 3.4 auxiliary programs

EnergyPlus는 Weather Converter, Ground Heat Transfer, View Factor, Transition, EP-Macro, EP-Launch, ReadVarsESO, HVAC Diagram 등 여러 auxiliary program을 제공한다. Rust port에서는 이를 흩어진 exe 묶음이 아니라 `eplus-rs` subcommand 기반 toolchain으로 재정의한다.

### 3.5 regression culture

EnergyPlus에는 build 간 regression을 수행하기 위한 도구와 test culture가 존재한다. Rust port의 regression은 다음을 기준으로 한다.

```text
기준:
  EnergyPlus 26.1.0 oracle

비교 대상:
  output file
  structured ResultStore
  trace
  diagnostics
  compile-stage artifacts
```

---

## 4. 프로젝트 목표와 비목표

### 4.1 목표

1. **EnergyPlus 26.1.0과의 정합성 확보**
   - 초기에는 일부 객체 subset만 지원하더라도, 지원 범위 안에서는 정합성을 엄격하게 관리한다.
   - 정합성은 annual total만이 아니라 schedule/weather, graph, timestep state, component result, output variable까지 내려가서 관리한다.

2. **Rust-only 기반 구조 현대화**
   - 프로덕션 구현 언어는 Rust로 고정한다.
   - C/C++/Fortran/assembly kernel을 섞지 않는다.
   - Rust type system, ownership, error handling, module boundary를 활용한다.

3. **Model compiler architecture**
   - epJSON을 바로 실행하지 않고 compile stage를 둔다.
   - compile stage는 validation, default resolution, typed conversion, reference resolution, graph build, execution plan generation으로 나뉜다.

4. **자료구조 중심 성능 개선**
   - string lookup 제거
   - typed ID 사용
   - compiled schedule/weather
   - calculation view
   - grouping/cache
   - output handle registry
   - cache invalidation graph
   - trace/profile store

5. **EnergyPlus-like 사용자 경험**
   - `eplus-rs run input.epJSON -w weather.epw -d output` 같은 단순 실행 경험을 제공한다.
   - `eplusout.err`, CSV export 같은 familiar output은 제공하되, 내부 native output은 `ResultStore`로 둔다.

6. **개발 환경 재현성**
   - `setup.ps1` 한 번으로 Rust toolchain, portable EnergyPlus oracle, reference source, docs tool이 준비되게 한다.
   - 개인 PC에 설치된 EnergyPlus, Python package, PATH, registry에 의존하지 않는다.

7. **EnergyPlus의 미래 구조 제안**
   - schema-native input
   - model compiler
   - explicit state
   - graph-first validation
   - execution plan runtime
   - typed diagnostics
   - traceable solver
   - ResultStore 중심 output
   - mode-separated algorithm evolution

### 4.2 비목표

1. 초기부터 EnergyPlus 전체 기능 구현
2. IDF parser를 core로 삼는 것
3. C++ 코드를 줄 단위로 기계 번역하는 것
4. C/C++/Fortran/assembly kernel을 섞어 성능 최적화하는 것
5. 초기부터 GPU/SIMD/unsafe 최적화에 집중하는 것
6. EnergyPlus보다 다른 답을 빠르게 내는 것
7. 계산 알고리즘을 초기부터 임의로 개선해 baseline과 달라지게 만드는 것
8. GUI를 core로 만드는 것
9. Word/PDF 문서를 원본으로 관리하는 것
10. 사용자 PC의 EnergyPlus 설치본을 기준 oracle로 쓰는 것

---

## 5. Rust-only 코드 언어 정책

### 5.1 기본 정책

```text
프로덕션 구현 언어:
  Rust

초기 최적화 수단:
  Rust 자료구조 개선
  Rust compiler 최적화
  cache
  execution plan
  profiling
  safe Rust 우선

초기 제외:
  C kernel
  C++ kernel
  Fortran kernel
  hand-written assembly
  external native numerical kernel 혼합
```

### 5.2 EnergyPlus source의 역할

EnergyPlus source는 reference이다. Rust port의 일부로 compile하지 않는다.

```text
.reference/energyplus-src/26.1.0/
  - 읽기 전용 reference
  - 알고리즘 확인
  - testfile 확인
  - 변수명/출력명 확인
  - porting-map 작성

crates/
  - 실제 Rust 구현
```

### 5.3 unsafe Rust 정책

초기 milestone에서는 `unsafe`를 쓰지 않는 것을 기본으로 한다.

허용 가능성이 있는 경우:

```text
- benchmark로 확인된 극소수 hot path
- safe Rust로 동일 성능 확보가 어렵다고 입증된 경우
- compatibility mode가 아니라 fast mode에 한정되는 경우
- scalar reference implementation과 bit/tolerance 비교 test가 있는 경우
```

금지:

```text
- parser, model validation, graph validation, diagnostics에서 unsafe 사용
- 이유 없이 pointer arithmetic 사용
- release 기본 path에서 검증되지 않은 unsafe 사용
```

### 5.4 SIMD 정책

초기 release에서는 SIMD를 목표로 하지 않는다. 장기적으로 Rust 표준 생태계 안에서만 검토한다.

```text
compatibility mode:
  deterministic scalar path

fast mode future:
  Rust-only SIMD 가능성 검토
  floating-point order 변화 허용 여부를 tolerance policy로 관리

금지:
  C/C++/Fortran/assembly SIMD kernel 링크
```

---

## 6. 전체 architecture 개념

### 6.1 전체 pipeline

```text
Legacy IDF
  ↓ official ConvertInputFormat or eplus-rs legacy import

epJSON
  ↓ schema validation

RawModel
  ↓ default resolution
  ↓ enum/unit validation
  ↓ typed conversion

TypedModel
  ↓ name reference → typed ID
  ↓ model normalization

SimulationModel
  ↓ graph build
  ↓ pre-simulation diagnostics

ModelGraph
  ↓ execution plan generation

ExecutionPlan
  ↓ state initialization

SimulationState
  ↓ timestep runtime

ResultStore / DiagnosticStore / TraceStore
  ↓ exporters

CSV / SQLite / JSON / Parquet / EnergyPlus-like err/csv outputs
```

### 6.2 Model compiler stage

Rust port는 입력을 직접 실행하지 않고, compile stage를 명확히 둔다.

| Stage | 입력 | 출력 | 주요 test |
|---|---|---|---|
| Parse | epJSON | RawModel | object count, raw field preservation |
| Schema validation | RawModel | ValidatedRawModel | enum, field, required object |
| Normalize | ValidatedRawModel | NormalizedRawModel | default resolution, canonical ordering |
| Typed conversion | NormalizedRawModel | TypedModel | unit, enum, typed struct |
| Reference resolution | TypedModel | SimulationModel | name → typed ID |
| Graph build | SimulationModel | ModelGraph | zone/surface/node/loop graph |
| Execution plan | ModelGraph | ExecutionPlan | deterministic order, unsupported topology |
| Runtime init | ExecutionPlan + SimulationModel | SimulationState | initial state, output handles |

### 6.3 Runtime principle

```text
Model:
  immutable during simulation
  shareable across multiple SimulationState

SimulationState:
  mutable timestep-dependent data
  resettable
  traceable

ExecutionPlan:
  compiled execution order
  no runtime string dispatch

CacheState:
  explicit derived-data validity
  invalidation dependency graph

ResultStore:
  native structured output
  legacy output is export layer
```

---

## 7. 핵심 자료구조 설계

### 7.1 RawModel

목적:

```text
- epJSON 원형 보존
- unknown object 보존 가능
- migration/diff/reporting에 사용
- EnergyPlus ConvertInputFormat 결과와 비교 가능
```

예상 구조:

```rust
pub struct RawModel {
    pub version: String,
    pub objects: IndexMap<ObjectType, IndexMap<ObjectName, RawObject>>,
}

pub struct RawObject {
    pub fields: IndexMap<FieldName, RawValue>,
    pub source_span: Option<SourceSpan>,
}
```

### 7.2 TypedModel

목적:

```text
- simulation 의미를 가진 Rust struct
- enum/default/unit 처리 완료
- 이름 참조는 아직 일부 남을 수 있음
```

예상 구조:

```rust
pub struct TypedModel {
    pub version: Version,
    pub building: Building,
    pub timesteps: TimestepConfig,
    pub run_periods: Vec<RunPeriod>,
    pub site: Site,
    pub zones: Vec<Zone>,
    pub surfaces: Vec<Surface>,
    pub constructions: Vec<Construction>,
    pub materials: Vec<Material>,
    pub schedules: ScheduleStore,
    pub outputs: OutputRequests,
}
```

### 7.3 SimulationModel

목적:

```text
- runtime-ready immutable model
- 모든 name reference를 typed ID로 변환
- 계산에 필요한 static data 준비
```

예상 ID:

```rust
pub struct ZoneId(pub u32);
pub struct SurfaceId(pub u32);
pub struct ConstructionId(pub u32);
pub struct MaterialId(pub u32);
pub struct ScheduleId(pub u32);
pub struct NodeId(pub u32);
pub struct ComponentId(pub u32);
pub struct LoopId(pub u32);
pub struct OutputHandle(pub u32);
```

### 7.4 NameMap

문자열 참조는 loading/compile 단계에서만 사용한다.

```rust
pub struct NameMap<T> {
    by_name: HashMap<NormalizedName, T>,
    by_id: Vec<NormalizedName>,
}
```

규칙:

```text
- simulation timestep 중 object name string lookup 금지
- diagnostics/report/export에서만 name 역참조 허용
- name normalization 규칙은 compile stage에서 고정
```

### 7.5 ModelGraph

Graph는 계산 전 검증과 execution plan 생성을 위한 핵심이다.

```rust
pub struct ModelGraph {
    pub zone_surface: ZoneSurfaceGraph,
    pub construction_material: ConstructionMaterialGraph,
    pub air_nodes: NodeGraph,
    pub plant_nodes: NodeGraph,
    pub components: ComponentGraph,
    pub controls: ControlGraph,
}
```

Graph validation 대상:

```text
- missing reference
- duplicate node
- dangling node
- zone equipment list 누락
- unsupported plant topology
- circular dependency
- unreachable component
- invalid control target
```

### 7.6 ExecutionPlan

ExecutionPlan은 runtime에서 매번 판단하지 않도록 미리 만든 실행 순서이다.

```rust
pub struct ExecutionPlan {
    pub stages: Vec<Stage>,
    pub output_plan: OutputPlan,
    pub trace_plan: TracePlan,
}

pub struct Stage {
    pub name: StageName,
    pub steps: Vec<ExecutionStep>,
    pub dependencies: Vec<StageId>,
}

pub enum ExecutionStep {
    UpdateWeather,
    EvaluateScheduleGroup(ScheduleGroupId),
    UpdateInternalGains(ZoneGroupId),
    SimulateSurfaceGroup(SurfaceGroupId),
    SolveZone(ZoneId),
    SimulateComponent(ComponentId),
    ResolveAirLoop(LoopId),
    ResolvePlantLoop(LoopId),
    WriteOutput(OutputGroupId),
}
```

### 7.7 CalcView

TypedModel은 사람이 이해하기 좋은 구조이고, CalcView는 계산에 유리한 구조이다.

```rust
pub struct CalcView {
    pub zones: ZoneCalcView,
    pub surfaces: SurfaceCalcView,
    pub constructions: ConstructionCalcView,
    pub radiation: RadiationCalcView,
    pub schedules: CompiledScheduleStore,
    pub weather: WeatherTimestepSeries,
}
```

목적:

```text
- cache-friendly layout
- grouping
- 반복 계산 최소화
- fast mode 준비
```

### 7.8 SimulationState

```rust
pub struct SimulationState {
    pub time: TimeState,
    pub environment: EnvironmentState,
    pub weather: WeatherState,
    pub schedules: ScheduleState,
    pub zones: ZoneStateStore,
    pub surfaces: SurfaceStateStore,
    pub air_nodes: NodeStateStore,
    pub plant_nodes: NodeStateStore,
    pub hvac: HvacStateStore,
    pub plant: PlantStateStore,
    pub controls: ControlStateStore,
    pub cache: CacheState,
}
```

### 7.9 CacheState

Cache invalidation은 EMS/control/radiation/daylighting 정합성에 중요하다.

```rust
pub struct CacheState {
    pub valid: CacheValidFlags,
    pub dependencies: CacheDependencyGraph,
}

pub enum CacheKind {
    WeatherDerived,
    SolarPosition,
    SurfaceSolar,
    ZoneRadiation,
    Daylighting,
    HvacControl,
    PlantDispatch,
    OutputAggregation,
}
```

### 7.10 ResultStore / DiagnosticStore / TraceStore

```rust
pub struct ResultStore {
    pub time_index: Vec<TimeStamp>,
    pub series: Vec<OutputSeries>,
    pub meters: Vec<MeterSeries>,
    pub metadata: Vec<OutputMetadata>,
}

pub struct DiagnosticStore {
    pub diagnostics: Vec<Diagnostic>,
}

pub struct TraceStore {
    pub weather_trace: Option<WeatherTrace>,
    pub schedule_trace: Option<ScheduleTrace>,
    pub zone_trace: Option<ZoneTrace>,
    pub surface_trace: Option<SurfaceTrace>,
    pub node_trace: Option<NodeTrace>,
    pub component_trace: Option<ComponentTrace>,
    pub loop_trace: Option<LoopTrace>,
}
```

---

## 8. 자료구조별 최적화 영역

### 8.1 Input object store

기존 감각:

```text
object type string
object name string
alpha fields
numeric fields
field index
```

Rust port 방향:

```text
RawModel:
  epJSON 보존

TypedModel:
  typed struct

SimulationModel:
  typed ID / compact immutable data
```

최적화 효과:

```text
- runtime field lookup 제거
- default/enum/unit 오류를 compile stage에서 검출
- EnergyPlus schema 변화에 대응 쉬움
- GUI/API/doc generation에 사용 가능
```

### 8.2 Name reference

대상:

```text
Zone name
Surface name
Construction name
Material name
Schedule name
Node name
Branch name
Loop name
Component name
Output key
```

정책:

```text
- compile stage에서 ID로 변환
- runtime에서는 ID만 사용
- diagnostics/export에서만 name 역참조
```

### 8.3 Schedule

기존 입력 schedule을 runtime마다 해석하지 않는다.

```text
Raw schedule
  ↓ compile
CompiledSchedule
  ↓ timestep O(1) lookup
```

구조:

```rust
pub enum CompiledSchedule {
    Constant { value: f64 },
    Dense { values: Vec<f64> },
    RunLengthEncoded { spans: Vec<ScheduleSpan> },
}
```

test:

```text
- Schedule:Constant exact comparison
- Schedule:Compact day type comparison
- 8760 h or run-period timestep schedule trace
```

### 8.4 Weather

EPW를 raw row 기반으로 매번 조회하지 않는다.

```rust
pub struct WeatherTimestepSeries {
    pub dry_bulb: Vec<f64>,
    pub dew_point: Vec<f64>,
    pub rel_humidity: Vec<f64>,
    pub wind_speed: Vec<f64>,
    pub wind_dir: Vec<f64>,
    pub direct_normal: Vec<f64>,
    pub diffuse_horizontal: Vec<f64>,
    pub global_horizontal: Vec<f64>,
}
```

test:

```text
- EPW metadata
- RunPeriod/calendar
- timestep interpolation
- design day support later
```

### 8.5 Geometry / Surface / Construction

계산용 view를 별도로 둔다.

```rust
pub struct SurfaceCalcView {
    pub zone_id: Vec<ZoneId>,
    pub area: Vec<f64>,
    pub tilt: Vec<f64>,
    pub azimuth: Vec<f64>,
    pub construction_id: Vec<ConstructionId>,
    pub boundary_kind: Vec<BoundaryKind>,
}
```

grouping:

```text
surfaces_by_zone
surfaces_by_construction
exterior_surfaces
interior_surfaces
fenestration_surfaces
orientation_groups
```

효과:

```text
- surface loop 단순화
- construction coefficient cache 활용
- radiation cache와 연결
- trace/filter/export 용이
```

### 8.6 Construction thermal cache

```rust
pub struct ConstructionThermalCache {
    pub ctf: Option<CtfCoefficients>,
    pub rc: Option<RcNetwork>,
    pub fd_grid: Option<FiniteDifferenceGrid>,
}
```

정책:

```text
compatibility mode:
  EnergyPlus-compatible coefficient와 계산 순서 우선

fast mode:
  grouping/cache/vectorized update 검토
```

### 8.7 Radiation / solar cache

```rust
pub struct SolarPositionCache {
    pub sun_vectors: Vec<SunVector>,
}

pub struct RadiationCalcView {
    pub zone_matrices: Vec<ZoneRadiationMatrix>,
    pub surface_solar_cache: SurfaceSolarCache,
}
```

우선순위:

```text
1. solar position precompute
2. orientation grouping
3. exterior/interior radiation path 분리
4. view factor/radiation matrix cache
5. shading candidate grouping later
```

### 8.8 HVAC/Plant graph

핵심 구조:

```rust
pub struct SystemGraph {
    pub nodes: Vec<Node>,
    pub components: Vec<Component>,
    pub branches: Vec<Branch>,
    pub loops: Vec<Loop>,
    pub edges: Vec<Edge>,
}
```

must-have diagnostics:

```text
- dangling node
- duplicate node
- unsupported common pipe topology
- component not in equipment list
- invalid branch component order
- invalid setpoint manager target
```

### 8.9 Component registry

문자열 dispatch를 제거한다.

```rust
pub enum ComponentKind {
    FanConstantVolume(FanConstantVolumeId),
    CoilCoolingDxSingleSpeed(CoilCoolingDxSingleSpeedId),
    CoilHeatingElectric(CoilHeatingElectricId),
    PumpVariableSpeed(PumpVariableSpeedId),
    ChillerElectricEir(ChillerElectricEirId),
}
```

### 8.10 Output registry

Output variable lookup은 initialization에서 끝낸다.

```rust
pub struct OutputRegistry {
    pub variables: Vec<OutputVariable>,
    pub meters: Vec<Meter>,
    pub lookup: HashMap<OutputKey, OutputHandle>,
}
```

runtime:

```text
OutputHandle로 write
MeterHandle로 aggregate
CSV/SQLite/JSON export는 후처리
```

---

## 9. 성능 구조 개선 전략

### 9.1 성능 개선의 우선순위

성능 개선은 다음 순서로만 진행한다.

```text
1. EnergyPlus oracle과 compatibility scalar path 정합성 확보
2. profiling/trace로 병목 확인
3. 자료구조 개선
4. cache / execution plan 개선
5. algorithmic iteration 개선
6. fast mode로 격리된 최적화
7. compatibility 결과 변화 여부 확인
```

금지:

```text
- 정합성 baseline 없이 최적화
- 처음부터 SIMD/unsafe 중심 개발
- 결과 차이 원인을 추적할 수 없는 최적화
```

### 9.2 Model compiler 최적화

compile stage에서 가능한 것을 모두 끝낸다.

```text
- schema validation
- default resolution
- enum parsing
- unit conversion
- name reference resolution
- schedule compilation
- weather preprocessing
- graph build
- unsupported topology validation
- output handle registration
- execution plan generation
```

runtime에서 하지 말아야 할 것:

```text
- object type string lookup
- object name string lookup
- field name lookup
- schedule compact interpretation
- output variable lookup
- node name resolution
```

### 9.3 Execution plan runtime

timestep마다 동적으로 무엇을 할지 찾지 않고, compile된 `ExecutionPlan`을 순회한다.

장점:

```text
- branch 감소
- trace 위치 명확
- 병렬화 후보 식별 가능
- diagnostic stage 지정 가능
- EnergyPlus-compatible plan과 fast plan 분리 가능
```

### 9.4 Multi-mode runtime

```text
compatibility:
  default
  deterministic scalar path
  EnergyPlus 정합성 우선

diagnostic:
  trace/invariant/diagnostics 강화
  느려도 원인 분석 우선

fast:
  cache/grouping/parallel-friendly path
  compatibility와 별도 tolerance 관리

experimental:
  algorithmic solver 실험
  release 기본값 아님
```

### 9.5 Profiling 기본 지표

각 run에서 최소한 다음을 기록할 수 있어야 한다.

```text
total runtime
parse time
schema validation time
typed conversion time
graph build time
execution plan build time
weather preprocessing time
schedule preprocessing time
simulation runtime
output export time

per timestep:
  weather update time
  schedule update time
  surface heat balance time
  zone heat balance time
  hvac time
  plant time
  output write time
  iteration count
  residual
```

### 9.6 Performance regression

성능 test는 정합성 test와 분리한다.

```text
정합성 regression:
  값이 맞는가

성능 regression:
  이전보다 느려졌는가
```

성능 baseline 예:

```text
data/performance_cases/
  perf_001_minimal_1zone/
  perf_010_many_surfaces/
  perf_020_many_zones/
  perf_030_schedule_heavy/
  perf_040_output_heavy/
  perf_050_hvac_graph_heavy/
```

### 9.7 병렬화 정책

초기 병렬화 우선순위:

```text
1. test case 병렬 실행
2. independent simulation 병렬 실행
3. output export 병렬화 검토
4. surface/zone group 병렬화는 fast mode에서만 검토
```

compatibility mode에서 조심할 점:

```text
- floating-point reduction order 변경 가능
- trace 순서 변경 가능
- non-deterministic diagnostics order 가능
```

따라서 compatibility mode는 deterministic을 우선한다.

### 9.8 Rust-only kernel 정책

`ep_kernel` 같은 crate를 만들 수는 있지만, Rust-only 원칙을 지킨다.

```text
ep_kernel:
  scalar Rust reference implementation
  optional Rust-only optimized implementation later
  no C/C++/Fortran/assembly link
```

대상 후보:

```text
- psychrometric functions
- curve evaluation
- surface loop scalar functions
- simple vector updates
```

하지만 초기 milestone에서는 kernel 분리보다 자료구조와 정합성이 우선이다.

---

## 10. Repo 구조

### 10.1 최종 목표 구조

```text
energyplus-rs/
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  rustfmt.toml
  README.md
  LICENSE
  NOTICE.md

  .cargo/
    config.toml

  crates/
    ep_schema/
    ep_raw_model/
    ep_model/
    ep_units/
    ep_compile/
    ep_graph/
    ep_plan/
    ep_state/
    ep_cache/
    ep_weather/
    ep_schedule/
    ep_geometry/
    ep_radiation/
    ep_heat_balance/
    ep_hvac/
    ep_plant/
    ep_controls/
    ep_output/
    ep_trace/
    ep_profile/
    ep_compare/
    ep_legacy/
    ep_toolchain/
    ep_api/
    ep_cli/

  apps/
    eplus-rs/
    eplus-rs-launch/

  scripts/
    setup.ps1
    setup-rust.ps1
    setup-energyplus-binary.ps1
    setup-energyplus-source.ps1
    setup-docs.ps1
    check.ps1
    test.ps1
    docs.ps1
    docs-serve.ps1
    docs-check.ps1
    oracle-smoke.ps1
    compare-regression.ps1
    compare-trace.ps1
    perf.ps1
    package.ps1
    clean.ps1
    find-ref.ps1

  config/
    default.toml
    local.toml.example
    local.toml        # gitignore

  tools/
    oracle/
      energyplus.lock.toml
      NOTICE.md
      README.md
    schema/
      README.md
    generators/
      schema_codegen/
    ci/

  data/
    schema/
    fixtures/
      weather/
      schedules/
      minimal/
    testcases/
      001_minimal_1zone/
      002_uncontrolled_1zone/
      003_schedule_weather/
      004_graph_validation/
      005_first_runnable/
      006_ideal_loads/
      007_simple_hvac/
    baselines/
      energyplus-26.1.0/
    regression_cases/
      github_11610_vrf_scheduled_priority/
      github_11608_plant_scheme_priority/
      github_11599_vspump_two_way_common_pipe/
      github_4787_zone_equipment_list_missing/
    performance_cases/
      perf_001_minimal_1zone/
      perf_010_many_surfaces/
      perf_020_schedule_heavy/

  examples/
    001_minimal_1zone/
    002_ideal_loads/

  docs/
    book.toml
    src/
      SUMMARY.md
      introduction.md
      quick-start.md
      user-guide/
      compatibility/
      developer-guide/
      architecture/
      data-architecture/
      performance/
      porting-map/
      adr/
      release/

  .runtime/
    energyplus/
      26.1.0/
        windows-x64/
          EnergyPlus/
            energyplus.exe
            ConvertInputFormat.exe
            Energy+.idd
            ...
    # gitignore

  .reference/
    energyplus-src/
      26.1.0/
        EnergyPlus/
          src/
          tst/
          testfiles/
          doc/
    index/
      26.1.0/
    # gitignore

  target/
    # gitignore
```

### 10.2 초기 commit 구조

처음부터 모든 crate를 만들 필요는 없다. 초기에는 아래 정도로 시작한다.

```text
energyplus-rs/
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  .cargo/config.toml

  crates/
    ep_schema/
    ep_raw_model/
    ep_model/
    ep_compare/
    ep_cli/

  apps/
    eplus-rs/

  scripts/
    setup.ps1
    setup-energyplus-binary.ps1
    setup-energyplus-source.ps1
    check.ps1
    test.ps1
    docs.ps1

  tools/
    oracle/
      energyplus.lock.toml

  config/
    default.toml
    local.toml.example

  data/
    testcases/
      001_minimal_1zone/

  docs/
    book.toml
    src/
```

---

## 11. Toolchain 고정

### 11.1 Rust toolchain

`rust-toolchain.toml`을 root에 둔다.

```toml
[toolchain]
channel = "1.xx.x"
components = [
    "rustfmt",
    "clippy"
]
targets = [
    "x86_64-pc-windows-msvc"
]
```

초기에는 `stable` 대신 구체 버전을 박는 편이 좋다.

### 11.2 Cargo.lock

`Cargo.lock`은 반드시 commit한다. 이 프로젝트는 library crate만이 아니라 executable/toolchain 성격이 강하므로 dependency resolution을 고정해야 한다.

### 11.3 .cargo/config.toml

```toml
[build]
target = "x86_64-pc-windows-msvc"

[env]
RUST_BACKTRACE = "1"
EPLUS_RS_DATA_DIR = { value = "data", relative = true }

[alias]
check-all = "check --workspace --all-targets"
test-all = "test --workspace --all-targets"
```

개인 PC 경로는 여기에 넣지 않는다.

---

## 12. Portable EnergyPlus oracle 관리

### 12.1 원칙

기존 EnergyPlus는 두 형태로 관리한다.

```text
1. oracle binary
   - EnergyPlus 결과 생성용
   - .runtime/ 하위에 위치
   - gitignore

2. oracle source
   - C++ 알고리즘, testfiles, unit test 확인용
   - .reference/ 하위에 위치
   - gitignore
```

둘은 같은 version이어야 한다.

초기 기준:

```text
EnergyPlus 26.1.0
tag: v26.1.0
platform: windows-x64
```

### 12.2 energyplus.lock.toml

`tools/oracle/energyplus.lock.toml`:

```toml
[energyplus]
version = "26.1.0"
tag = "v26.1.0"
platform = "windows-x64"

[binary]
source = "github-release"
url = "TO_BE_FILLED_WITH_RELEASE_ASSET_URL"
sha256 = "TO_BE_FILLED"

[source]
url = "https://github.com/NREL/EnergyPlus/archive/refs/tags/v26.1.0.zip"
sha256 = "TO_BE_FILLED"
extract_dir = ".reference/energyplus-src/26.1.0/EnergyPlus"

[runtime]
install_dir = ".runtime/energyplus/26.1.0/windows-x64/EnergyPlus"

[required_files]
energyplus_exe = "energyplus.exe"
convert_input_format_exe = "ConvertInputFormat.exe"
idd = "Energy+.idd"
```

### 12.3 PC 설치본을 쓰지 않는 이유

```text
- 개발자마다 EnergyPlus version이 다름
- PATH에 잡힌 executable이 다름
- Python virtualenv/PYTHONPATH 영향을 받을 수 있음
- registry/ini path가 달라질 수 있음
- baseline 결과가 달라질 수 있음
```

모든 oracle 실행은 다음 경로만 사용한다.

```text
.runtime/energyplus/26.1.0/windows-x64/EnergyPlus/energyplus.exe
.runtime/energyplus/26.1.0/windows-x64/EnergyPlus/ConvertInputFormat.exe
```

### 12.4 reference source 사용 원칙

`.reference/energyplus-src/`는 읽기 전용 reference이다. 여기에 있는 C++ 코드를 수정하지 않는다.

Rust 코드에는 출처를 추적 가능하게 남긴다.

```rust
// Ported with reference to EnergyPlus v26.1.0
// src/EnergyPlus/ScheduleManager.cc
// See docs/src/porting-map/schedule.md
```

---

## 13. setup.ps1 설계

### 13.1 목표

`setup.ps1` 한 번으로 개발 가능한 PC가 되어야 한다.

실행:

```powershell
.\scripts\setup.ps1
```

해야 할 일:

```text
1. Windows 환경 확인
2. rustup 확인
3. rust-toolchain.toml 기준 toolchain 설치/확인
4. rustfmt/clippy 확인
5. portable EnergyPlus binary 다운로드/검증/압축해제
6. EnergyPlus reference source 다운로드/검증/압축해제
7. config/local.toml 자동 생성
8. EnergyPlus oracle version 확인
9. ConvertInputFormat 동작 확인
10. 최소 IDF 실행 smoke test
11. Rust epJSON loader smoke test
12. docs tool 준비는 optional 또는 별도 setup-docs.ps1
```

### 13.2 setup skeleton

```powershell
$ErrorActionPreference = "Stop"

Write-Host "== eplus-rs setup =="

.\scripts\setup-rust.ps1
.\scripts\setup-energyplus-binary.ps1
.\scripts\setup-energyplus-source.ps1

if (-not (Test-Path "config")) {
    New-Item -ItemType Directory -Path "config" | Out-Null
}

if (-not (Test-Path "config/local.toml")) {
    Copy-Item "config/local.toml.example" "config/local.toml"
}

cargo check --workspace --all-targets

.\scripts\oracle-smoke.ps1

Write-Host "Setup complete."
```

### 13.3 oracle smoke

smoke test는 다음을 확인한다.

```text
- energyplus.exe 존재
- ConvertInputFormat.exe 존재
- Energy+.idd 존재
- energyplus.exe version 확인
- 최소 IDF 실행
- eplusout.err 생성
- fatal error 없음
- IDF → epJSON 변환 가능
- Rust loader가 epJSON 읽기 가능
```

---

## 14. Crate 역할

### 14.1 ep_schema

```text
- epJSON schema loading
- schema version 확인
- object/field metadata 제공
- enum/default/unit metadata 제공
- 향후 schema-generated Rust code의 기반
```

### 14.2 ep_raw_model

```text
- epJSON을 raw object store로 읽음
- object type/name/field 보존
- unknown object 보존 가능
- migration 전후 diff 가능
```

### 14.3 ep_model

```text
- simulation-ready typed model
- Zone, Surface, Construction, Material, Schedule 등 struct
- default resolution
- unit conversion / unit typing
- name reference → typed ID 변환 전 단계
```

### 14.4 ep_compile

v2에서 추가된 핵심 crate이다.

```text
- RawModel → TypedModel → SimulationModel compile pipeline
- default resolution
- enum/unit validation
- name reference resolution
- compile-stage diagnostics
- compile artifact generation
```

### 14.5 ep_graph

```text
- Zone-Surface graph
- Construction-Material graph
- AirLoop graph
- PlantLoop graph
- Node graph
- Control graph
- graph validation
```

### 14.6 ep_plan

v2에서 추가된 crate이다.

```text
- ExecutionPlan 생성
- Stage/Step 정의
- output plan
- trace plan
- compatibility plan과 future plan 분리
```

### 14.7 ep_state

```text
- SimulationState
- TimeState
- WeatherState
- ZoneState
- SurfaceState
- NodeState
- HVACState
- PlantState
- ControlState
```

### 14.8 ep_cache

v2에서 추가된 crate이다.

```text
- CacheState
- CacheDependencyGraph
- derived data validity flags
- actuator/control 이후 invalidation
```

### 14.9 ep_weather

```text
- EPW reader
- weather metadata
- hourly/subhourly weather value
- WeatherTimestepSeries
- EnergyPlus weather value 정합성 test
```

### 14.10 ep_schedule

```text
- Schedule:Constant
- Schedule:Compact
- schedule type limits
- CompiledSchedule
- timestep/hourly schedule value trace
```

### 14.11 ep_geometry / ep_radiation

```text
ep_geometry:
  - surface geometry
  - orientation
  - adjacency
  - zone boundary relation
  - view factor input

ep_radiation:
  - solar position
  - shortwave distribution
  - longwave radiation
  - view factor matrix/cache
```

### 14.12 ep_heat_balance

```text
- zone heat balance
- surface heat balance
- conduction model
- internal gains coupling
- compatibility mode heat balance
```

### 14.13 ep_hvac / ep_plant / ep_controls

```text
ep_hvac:
  air side, zone equipment, fan, coil

ep_plant:
  plant loop, pump, boiler, chiller, heat exchanger

ep_controls:
  setpoint manager, availability manager, operation scheme, supervisory control
```

### 14.14 ep_output

```text
- OutputRegistry
- Output:Variable
- Output:Meter
- ResultStore
- CSV/SQLite/JSON/Parquet export
- legacy ESO/MTR optional import/export
```

### 14.15 ep_trace

```text
- timestep state snapshot
- schedule/weather trace
- zone/surface/node trace
- component result trace
- solver iteration/residual trace
- selected operation scheme trace
```

### 14.16 ep_profile

v2에서 추가된 crate이다.

```text
- stage별 runtime measurement
- per-timestep measurement
- performance regression report
- benchmark metadata
```

### 14.17 ep_compare

```text
- EnergyPlus oracle output 읽기
- Rust output 읽기
- tolerance 비교
- trace 비교
- report 생성
```

### 14.18 ep_legacy

```text
- IDF import
- EP-Macro wrapper or parser
- ExpandObjects compatibility wrapper
- ESO/MTR legacy handling
- Transition/migration compatibility
```

### 14.19 ep_toolchain

```text
- model import pipeline
- validate pipeline
- compile pipeline
- graph pipeline
- run pipeline
- result export pipeline
- compare pipeline
```

### 14.20 ep_cli

```text
- command parsing
- user-facing messages
- ep_toolchain 호출
```

계산 로직을 넣지 않는다.

---

## 15. CLI와 주변프로그램 재정의

### 15.1 하나의 multi-command CLI

EnergyPlus 주변프로그램들을 여러 exe로 다시 만들지 않고, `eplus-rs` subcommand로 재정의한다.

```text
eplus-rs
  model
  weather
  graph
  compile
  run
  result
  trace
  compare
  profile
  oracle
  legacy
  dev
```

### 15.2 주요 명령

```powershell
eplus-rs oracle setup
eplus-rs oracle run data\testcases\001_minimal_1zone
eplus-rs oracle convert input.idf --out input.epJSON

eplus-rs model import input.idf --out input.epJSON
eplus-rs model validate input.epJSON
eplus-rs model summary input.epJSON
eplus-rs model normalize input.epJSON
eplus-rs model migrate input.epJSON --to 26.1.0

eplus-rs compile input.epJSON --out build\case001.eplusrs-model
eplus-rs compile inspect build\case001.eplusrs-model

eplus-rs weather inspect weather.epw
eplus-rs weather validate weather.epw

eplus-rs graph validate input.epJSON
eplus-rs graph geometry input.epJSON --out geometry.json
eplus-rs graph hvac input.epJSON --out hvac_graph.json
eplus-rs graph plant input.epJSON --out plant_graph.json

eplus-rs run input.epJSON -w weather.epw -d output
eplus-rs run input.epJSON -w weather.epw -d output --trace
eplus-rs run build\case001.eplusrs-model -w weather.epw -d output

eplus-rs result export output --format csv
eplus-rs result summarize output
eplus-rs compare output data\baselines\energyplus-26.1.0\case001
eplus-rs trace compare output\trace.json baseline\trace.json
eplus-rs profile output\profile.json
```

### 15.3 기존 auxiliary program mapping

| 기존 EnergyPlus 도구 | Rust port 재정의 | 위치 | 우선순위 |
|---|---|---|---|
| EP-Launch | optional GUI wrapper | apps/eplus-rs-launch | 낮음 |
| IDFVersionUpdater/Transition | schema migration | ep_compile/ep_legacy, model migrate | 중간 |
| ConvertInputFormat | model import/export | ep_raw_model, ep_toolchain | 높음 |
| EP-Macro | legacy preprocessor | ep_legacy | 낮음~중간 |
| ExpandObjects/HVACTemplate | model expand/compiler | ep_compile/ep_toolchain | 중간 |
| Weather Converter | weather inspect/validate/convert | ep_weather | 높음 |
| Slab/Basement | ground boundary tool or wrapper | future Rust crate | 중간 |
| View Factor | geometry/radiation utility | ep_geometry/ep_radiation | 중간 |
| HVAC Diagram | graph visualization | ep_graph | 높음 |
| ReadVarsESO | result import/export | ep_output | 중간 |
| convertESOMTR | legacy output conversion | ep_output | 중간 |
| CSVproc | result summarize | ep_output | 낮음 |
| CoeffConv/Check | curve diagnostics | future Rust crate | 낮음 |

---

## 16. 문서 관리

### 16.1 원칙

문서 원본은 Markdown으로 둔다. Word/PDF는 원본이 아니다.

```text
docs/src/*.md  → mdBook HTML
Rust doc comment → cargo doc
release note → CHANGELOG.md
ADR → docs/src/adr/*.md
```

### 16.2 docs 구조

```text
docs/
  book.toml
  src/
    SUMMARY.md
    introduction.md
    quick-start.md

    user-guide/
      installation.md
      setup.md
      command-line.md
      input-formats.md
      running-simulation.md
      outputs.md
      diagnostics.md
      examples.md
      limitations.md

    compatibility/
      overview.md
      oracle-energyplus.md
      supported-objects.md
      tolerance-policy.md
      regression-tests.md
      version-policy.md

    developer-guide/
      repository-structure.md
      crate-boundaries.md
      setup-environment.md
      coding-style.md
      testing.md
      tracing.md
      adding-a-new-object.md
      adding-a-new-component.md

    architecture/
      overview.md
      schema-native-input.md
      model-compiler.md
      immutable-model-mutable-state.md
      graph-resolved-systems.md
      execution-plan.md
      result-store.md
      diagnostics.md

    data-architecture/
      raw-typed-simulation-model.md
      typed-id-reference-resolution.md
      compiled-schedule-weather.md
      calc-view.md
      cache-state.md
      output-registry.md

    performance/
      overview.md
      profiling.md
      compatibility-vs-fast-mode.md
      performance-regression.md

    porting-map/
      input.md
      weather.md
      schedule.md
      heat-balance.md
      hvac.md
      plant.md
      output.md

    adr/
      ADR-0001-schema-native-input.md
      ADR-0002-portable-oracle.md
      ADR-0003-reference-source.md
      ADR-0004-immutable-model-mutable-state.md
      ADR-0005-trace-based-comparison.md
      ADR-0006-graph-first-validation.md
      ADR-0007-rust-only-implementation.md
      ADR-0008-model-compiler-architecture.md
      ADR-0009-execution-plan-runtime.md

    release/
      changelog.md
      migration-guide.md
```

---

## 17. 정합성 검증 전략

### 17.1 기본 원칙

EnergyPlus와의 정합성은 다음 계층으로 관리한다.

```text
Level 0. 실행 성공 여부
  - fatal 없음
  - warning/severe/fatal count
  - output file 생성

Level 1. 입력 해석
  - object count
  - object name
  - field/default resolution
  - enum/string interpretation
  - unsupported object report

Level 2. compile artifact
  - typed model summary
  - resolved references
  - compiled schedule
  - weather timestep series
  - output handle registry

Level 3. model graph
  - zone-surface relation
  - construction-material relation
  - air node relation
  - plant node relation
  - component connection
  - control target

Level 4. execution plan
  - stage order
  - step count
  - selected compatibility plan
  - unsupported topology detection

Level 5. intermediate state
  - zone air temperature
  - surface temperature
  - node temperature
  - mass flow
  - humidity ratio
  - load request

Level 6. component result
  - fan power
  - coil load
  - pump power
  - boiler/chiller load
  - operation scheme selection

Level 7. output variable/meter
  - hourly/timestep series
  - meters
  - report variables

Level 8. aggregate result
  - monthly energy
  - annual energy
  - unmet hours
```

### 17.2 tolerance policy

| 항목 | 비교 방식 |
|---|---|
| object count | exact |
| object names | exact |
| field enum | exact |
| typed ID resolution | exact |
| execution plan stage count | exact, where applicable |
| schedule discrete value | exact 또는 매우 작은 tolerance |
| weather value | near-exact |
| zone temperature | abs tolerance + RMSE |
| surface temperature | abs tolerance + RMSE |
| mass flow | abs/relative tolerance |
| component load | abs/relative tolerance |
| hourly energy | RMSE + max error |
| monthly/annual energy | relative error |
| warning/fatal count | exact 또는 classified |

초기 tolerance는 느슨하게 시작하고 milestone별로 강화한다. 단, tolerance 변경은 release note에 명시한다.

### 17.3 comparison report

`eplus-rs compare`는 다음을 생성해야 한다.

```text
compare-report.md
compare-summary.json
compare-timeseries.csv
diff/
  raw-model.json
  typed-model.json
  graph.json
  execution-plan.json
  schedules.csv
  weather.csv
  zones.csv
  surfaces.csv
  nodes.csv
  components.csv
  outputs.csv
```

report 내용:

```text
- case id
- oracle version
- eplus-rs version
- input file hash
- weather file hash
- compile artifact hash
- supported object coverage
- failed comparison list
- largest differences
- first divergence timestep
- warning/fatal summary
- profile summary, if available
```

---

## 18. Test case 구조

### 18.1 test case 폴더

```text
data/testcases/001_minimal_1zone/
  case.toml
  input.idf
  input.epJSON
  weather.epw
  README.md
  expected-support.toml
```

### 18.2 baseline 폴더

```text
data/baselines/energyplus-26.1.0/001_minimal_1zone/
  eplusout.err
  eplusout.audit
  eplusout.eio
  eplusout.csv
  eplusout.sql
  eplusout.rdd
  eplusout.mdd
  metadata.json
```

### 18.3 compile artifact 폴더

Rust port 자체 compile artifact도 저장할 수 있다.

```text
data/artifacts/eplus-rs-0.5.0/001_minimal_1zone/
  raw-model.json
  typed-model.json
  graph.json
  execution-plan.json
  output-registry.json
```

### 18.4 case.toml 예시

```toml
id = "001_minimal_1zone"
title = "Minimal 1-zone model"
category = "input-weather-schedule"
oracle_version = "26.1.0"

[input]
idf = "input.idf"
epjson = "input.epJSON"
weather = "weather.epw"

[features]
has_hvac = false
has_plant = false
has_ems = false
has_airflow_network = false

[expected_support]
parse = true
validate = true
compile = true
graph = true
execution_plan = true
simulate = false

[tolerance]
weather_abs = 1e-9
schedule_abs = 1e-12
zone_temp_abs = 1e-3
annual_energy_rel = 1e-4
```

---

## 19. Test 종류와 실행 시점

### 19.1 개발자 local test

```powershell
.\scripts\check.ps1
```

포함:

```text
- cargo fmt --check
- cargo clippy
- cargo test
- basic docs check optional
```

언제 실행:

```text
- commit 전
- PR 전
- crate boundary 수정 후
```

### 19.2 oracle smoke test

```powershell
.\scripts\oracle-smoke.ps1
```

포함:

```text
- portable EnergyPlus 실행 확인
- ConvertInputFormat 확인
- 최소 IDF 실행
- baseline 생성 가능성 확인
```

언제 실행:

```text
- setup 직후
- oracle version 변경 후
- release 전
```

### 19.3 compile-stage test

명령 예:

```powershell
eplus-rs model validate data\testcases\001_minimal_1zone\input.epJSON
eplus-rs compile data\testcases\001_minimal_1zone\input.epJSON --out build\001.eplusrs-model
eplus-rs compile inspect build\001.eplusrs-model
```

포함:

```text
- RawModel parse
- schema validation
- default resolution
- typed conversion
- reference resolution
- compiled schedule/weather 준비 여부
- output handle registry
```

언제 실행:

```text
- ep_schema/ep_raw_model/ep_model/ep_compile 수정 시
- 새 객체 추가 시
```

### 19.4 weather/schedule test

포함:

```text
- EPW read
- calendar/day type
- timestep weather value
- Schedule:Constant
- Schedule:Compact
- compiled schedule table
```

언제 실행:

```text
- ep_weather 수정 시
- ep_schedule 수정 시
- heat balance 이전 milestone gate
```

### 19.5 graph validation test

포함:

```text
- zone-surface graph
- construction-material graph
- plant/hvac node graph
- missing reference
- dangling node
- unsupported topology
```

언제 실행:

```text
- ep_graph 수정 시
- HVAC/Plant 객체 추가 시
- issue-based regression 추가 시
```

### 19.6 execution plan test

포함:

```text
- stage order
- step count
- output plan
- trace plan
- unsupported topology가 runtime 전 검출되는지
```

언제 실행:

```text
- ep_plan 수정 시
- graph structure 변경 시
- runtime stage 추가 시
```

### 19.7 simulation subset test

포함:

```text
- uncontrolled 1 zone
- simple heat balance
- ideal loads
- selected output variables
```

언제 실행:

```text
- ep_heat_balance 수정 시
- ep_output 수정 시
- milestone v0.6 이후 모든 PR
```

### 19.8 trace comparison test

포함:

```text
- first divergence timestep
- schedule/weather trace
- zone/surface state
- selected operation scheme
- component output
```

언제 실행:

```text
- 결과 정합성 PR
- solver 변경
- HVAC/Plant 변경
```

### 19.9 performance regression test

명령:

```powershell
.\scripts\perf.ps1
```

포함:

```text
- compile time
- graph build time
- execution plan time
- simulation time
- output export time
- memory proxy metric, if available
```

언제 실행:

```text
- major data structure 변경 후
- release candidate
- nightly CI
```

---

## 20. Issue 기반 regression suite

EnergyPlus GitHub issue는 단순 참고자료가 아니라 Rust port의 regression case 후보로 사용한다.

### 20.1 초기 후보

| Case | 원 이슈 | 목적 |
|---|---|---|
| github_11615_virtualenv_pythonengine | #11615 | portable runtime 환경 격리 |
| github_11610_vrf_scheduled_priority | #11610 | schema/IDD/runtime enum 불일치 방지 |
| github_11608_plant_scheme_priority | #11608 | operation scheme priority trace |
| github_11599_vspump_two_way_common_pipe | #11599 | unsupported plant topology graceful error |
| github_4787_zone_equipment_list_missing | #4787 | graph validation과 명확한 diagnostics |

### 20.2 issue case 구조

```text
data/regression_cases/github_11599_vspump_two_way_common_pipe/
  case.toml
  input.idf
  input.epJSON
  oracle/
    energyplus-26.1.0/
      eplusout.err
      crash-notes.md
  expected-rust/
    diagnostics.json
  notes.md
```

### 20.3 issue case의 목적

모든 issue case를 EnergyPlus보다 먼저 “정확히 계산”할 필요는 없다. 초기 목적은 다음이다.

```text
- EnergyPlus가 cryptic error/hard crash를 내는 경우
- Rust port는 graph validation 단계에서 typed diagnostic을 내야 함
- unsupported면 unsupported라고 명확히 fail해야 함
```

---

## 21. 계산 알고리즘 정책

### 21.1 compatibility mode

기본 실행 모드이다.

```text
- EnergyPlus 26.1.0 결과 재현 우선
- 수치 알고리즘 변경 최소화
- timestep 구조 보존
- output variable 의미 보존
- regression 기준
- deterministic scalar Rust path
```

### 21.2 diagnostic mode

```text
- trace 강화
- invariant check 강화
- graph/runtime state snapshot 강화
- 성능보다 원인 분석 우선
```

### 21.3 fast mode

```text
- Rust-only 자료구조 최적화
- grouping/cache 적극 사용
- 병렬화 가능성 검토
- compatibility와 별도 tolerance 관리
- 기본값 아님
```

### 21.4 experimental mode

```text
- graph residual solver
- adaptive timestep
- state-space heat balance
- reduced-order model
- alternative radiation solver
- solver acceleration
```

기본값으로 사용하지 않는다.

### 21.5 mode 분리 예시

```powershell
eplus-rs run input.epJSON -w weather.epw --mode compatibility
eplus-rs run input.epJSON -w weather.epw --mode diagnostic
eplus-rs run input.epJSON -w weather.epw --mode fast
eplus-rs run input.epJSON -w weather.epw --mode experimental
```

---

## 22. Milestone 계획

### Versioning 재정의 — 2026-06-04

public semver tag는 “setup이 됐다”가 아니라 “사용자가 받을 수 있는 build artifact와 실행 가능한 command가 있다”를 기준으로 만든다.

따라서 setup-only, docs-only, oracle-only 단계는 public version이 아니라 foundation checkpoint로 관리한다. 기존에 임시로 올렸던 setup/inspection tag는 취소하고, 현재 기준 첫 public release는 다음 조건을 만족하는 `v0.1.0`으로 다시 시작한다.

```text
public version 조건:
  - eplus-rs.exe 또는 동등한 binary artifact가 빌드됨
  - release zip artifact가 생성됨
  - 사용자가 실행 가능한 command가 존재함
  - local verification script가 통과함
  - release note가 존재함
  - tag push 후 GitHub Release 생성 가능성을 확인함

foundation checkpoint:
  F0, F1처럼 표기
  public semver tag 없음
```

### 현재 진행 기준 보정 — 2026-06-07

v0.8부터 v0.10까지의 실제 진행은 초기 milestone 초안보다 더
보수적인 증거 계층으로 재정렬되었다. 현재 repo의 active 기준은 개별
`docs/src/operations/v0.*.0-plan.md` 및 readiness 문서이며, 이 v2
계획서는 다음 보정 원칙을 따른다.

```text
v0.8:
  heat_balance_nomass_001의 Zone Mean Air Temperature conformance gate.
  IdealLoads 호환성 claim이 아니다.

v0.9:
  surface_temperature_nomass_001의 zone/surface temperature conformance gate.
  fenestration, solar, dynamic exterior heat-balance claim이 아니다.

v0.10:
  thermostat, zone equipment, IdealLoads typed graph 및 baseline-only output
  evidence gate.
  IdealLoads load-conformance claim이 아니다.

v0.11 진입 전 hardening:
  - v0.10 fixture의 EnergyPlus warning을 허용 목록 기반으로 관리한다.
  - IdealLoads heating/cooling rate가 전부 0인 baseline만으로는 통과하지 않는다.
  - report skeleton은 first/last뿐 아니라 min/max/nonzero count와
    EnergyPlus warning summary를 기록한다.
  - compiler는 HVAC numeric range, equipment sequence, duplicate
    connection, missing reference, unsupported object type negative test를 갖는다.
  - NodeList와 node registry는 v0.11 진입 전 typed foundation으로
    고정하되, node temperature/flow/humidity conformance claim에는
    포함하지 않는다.
```

이 보정은 false conformance를 막기 위한 release 운영 기준이다.
버전명보다 중요한 것은 해당 버전이 실제로 어떤 증거를 blocking gate로
잠갔는지이며, 다음 버전으로 넘어가기 전에는 직전 버전의 fixture,
report, negative test, smoke gate가 모두 그 증거 수준에 맞게 정리되어야
한다.

### F0 — Reproducible Setup / Oracle Foundation

목표:

```text
개발 환경과 기준 oracle을 재현 가능하게 만든다. public release는 아니다.
```

완료 조건:

```text
- rust-toolchain.toml 고정
- Cargo workspace 초기화
- Rust-only policy 문서화
- setup.ps1 작동
- portable EnergyPlus 26.1.0 binary 다운로드/검증
- reference source 다운로드/검증
- config/local.toml 자동 생성
- oracle smoke test 통과
- docs skeleton 빌드
- 최소 test case 존재
```

정합성 test:

```text
- EnergyPlus 26.1.0으로 최소 IDF 실행
- eplusout.err 생성
- IDF → epJSON 변환 확인
```

### v0.1.0 — RawModel Inspection

목표:

```text
실제 IDF/epJSON을 읽고 raw model summary를 만드는 CLI와
release artifact를 제공한다.
```

완료 조건:

```text
- epJSON parser
- RawObjectStore
- schema version 확인
- object count
- 지원/미지원 object report
- eplus-rs.exe release build
- release zip artifact
```

정합성 test:

```text
- EnergyPlus ConvertInputFormat 결과 epJSON을 읽음
- object count 비교
- key object name 비교
- package artifact 생성 확인
```

### v0.2.0 — TypedModel Coverage / Reference Diagnostics

목표:

```text
RawModel을 typed model로 변환하는 지원 범위가 명시된 workflow를 만들고
name reference를 typed ID로 해석한다.
```

완료 조건:

```text
- typed model compile CLI coverage contract
- enum/default/unit 처리
- NameMap
- typed ID
- missing reference diagnostics
- typed compile fixture and oracle-generated epJSON smoke
```

정합성 test:

```text
- object reference exact comparison
- default handling report
- invalid enum diagnostics
```

### v0.3.0 — Model Compiler / Compile Artifacts

목표:

```text
epJSON을 simulation-ready compile artifact로 변환한다.
```

완료 조건:

```text
- ep_compile pipeline
- SimulationModel
- compile artifact 저장/검사
- output registry skeleton
- compile report
```

정합성 test:

```text
- compile artifact hash/report 생성
- 지원 객체 coverage 자동 report
```

### v0.4.0 — Weather/Schedule Compatibility

목표:

```text
weather와 schedule 값을 EnergyPlus와 비교 가능하게 만든다.
```

완료 조건:

```text
- EPW reader
- WeatherTimestepSeries
- RunPeriod/calendar/day type
- Schedule:Constant
- Schedule:Compact subset
- CompiledSchedule
- timestep schedule trace
- weather trace
```

정합성 test:

```text
- 8760 h schedule value 비교
- weather metadata 비교
- weather timestep value 비교
```

### v0.5.0 — Graph Validation / Execution Plan

목표:

```text
입력 모델을 graph로 해석하고 execution plan을 생성한다.
```

완료 조건:

```text
- Zone-Surface graph
- Construction-Material graph
- Node graph skeleton
- graph validate CLI
- ExecutionPlan skeleton
- structured diagnostics
- issue #4787 type case에서 더 명확한 error 가능
```

정합성/issue test:

```text
- EnergyPlus가 cryptic node error를 내는 case를 graph validation에서 조기 검출
- execution plan artifact 생성
```

### v0.6.0 — SimulationState / ResultStore / First Simulation Subset

목표:

```text
제한된 epJSON subset을 Rust engine으로 끝까지 실행한다.
```

지원 범위:

```text
- 1 zone
- simple geometry
- simple construction
- internal gains
- no HVAC 또는 uncontrolled
```

완료 조건:

```text
- SimulationState
- CacheState skeleton
- TimeState
- WeatherState
- ZoneState
- ResultStore
- DiagnosticStore
- basic output export
```

정합성 test:

```text
- EnergyPlus와 zone temperature 방향성 비교
- output time axis 비교
- fatal 없이 run 완료
```

### v0.7.0 — Trace / Compare Release

목표:

```text
결과 차이의 원인을 추적할 수 있게 한다.
```

완료 조건:

```text
- trace.json
- compare trace
- first divergence detection
- compare-report.md
- compare-summary.json
- profile summary skeleton
```

정합성 test:

```text
- schedule/weather/zone trace 비교
- output variable/meter 비교
```

### v0.8.0 — IdealLoads Compatibility

목표:

```text
건물 부하 계산을 EnergyPlus IdealLoads와 비교한다.
```

완료 조건:

```text
- heating/cooling setpoint schedule
- IdealLoads-like load calculation
- annual/monthly/hourly comparison
- tolerance report
```

정합성 test:

```text
- annual heating/cooling load relative error
- monthly profile comparison
- hourly RMSE
```

### v0.9.0 — Simple HVAC Subset

목표:

```text
단순 HVAC component를 지원한다.
```

지원 후보:

```text
- Fan:ConstantVolume
- simple heating coil
- simple cooling coil
- PTAC subset
- ZoneHVAC:IdealLoadsAirSystem 확장
```

정합성 test:

```text
- selected testfile subset
- fan power
- coil load
- zone load
```

### v0.10.0 — Air/Plant Graph Preview

목표:

```text
AirLoop/PlantLoop를 graph로 해석하고 제한적으로 실행한다.
```

완료 조건:

```text
- AirLoopGraph
- PlantLoopGraph
- operation scheme selection trace
- flow request trace
- unsupported topology diagnostics
```

정합성/issue test:

```text
- #11608 scheme priority case
- #11599 unsupported topology case
```

### v1.0.0 — Stable Compatibility Subset

목표:

```text
명시된 지원 범위 안에서 EnergyPlus-compatible Rust engine으로 신뢰 가능한 release.
```

완료 조건:

```text
- supported object table 고정
- tolerance policy 고정
- regression suite 공개
- performance regression suite 초기화
- docs 완비
- release artifact 재현성 확보
- unsupported 기능은 명확히 fail
```

중요:

```text
v1.0.0은 EnergyPlus 전체 대체가 아니다.
v1.0.0은 명시된 subset에 대한 안정 release이다.
```

---

## 23. Release 전략

### 23.1 자체 version과 EnergyPlus version 분리

```text
eplus-rs version:
  v0.1.0부터 시작
  setup-only 단계는 F0/F1 foundation checkpoint로 관리

oracle version:
  EnergyPlus 26.1.0
```

release title 예:

```text
eplus-rs 0.1.0 for EnergyPlus 26.1.0 RawModel inspection
```

### 23.2 release artifact

기본 artifact:

```text
eplus-rs-v0.1.0-windows-x64.zip
  bin/
    eplus-rs.exe
  scripts/
  docs/
  examples/
  config/
    default.toml
    local.toml.example
  tools/
    oracle/
      energyplus.lock.toml
      NOTICE.md
  LICENSE
  NOTICE.md
  README.md
```

오프라인 artifact optional:

```text
eplus-rs-v0.1.0-windows-x64-offline-oracle.zip
  .runtime/energyplus/26.1.0/...
  .reference/energyplus-src/26.1.0/... optional
```

### 23.3 release 전 checklist

```powershell
.\scripts\setup.ps1
.\scripts\check.ps1
.\scripts\test.ps1
.\scripts\docs-check.ps1
.\scripts\oracle-smoke.ps1
.\scripts\compare-regression.ps1
.\scripts\perf.ps1
.\scripts\package.ps1
git tag -a vX.Y.Z -m "eplus-rs vX.Y.Z"
git push origin vX.Y.Z
```

필수 확인:

```text
- cargo fmt 통과
- clippy warning 없음
- unit test 통과
- integration test 통과
- oracle smoke 통과
- mdBook build 통과
- cargo doc 통과
- regression report 생성
- performance report 생성
- package zip 압축 해제 후 smoke test 통과
- release note 작성
- tag push 기반 .github/workflows/release.yml 실행
- GitHub Release 생성 및 zip asset 업로드
- supported object table 최신화
- known limitations 최신화
```

`scripts\github-release.ps1`는 GitHub Actions 경로를 사용할 수 없을 때의
local/manual fallback으로만 사용한다.

### 23.4 release note 필수 항목

```text
- eplus-rs version
- oracle EnergyPlus version
- 기준 source tag
- supported platforms
- supported input formats
- supported object coverage
- compile-stage coverage
- passed regression cases
- performance regression summary
- tolerance policy
- known limitations
- breaking changes
- upgrade instructions
- issue-based regression additions
```

---

## 24. CI 운영

### 24.1 CI 원칙

CI는 로컬 script를 그대로 호출해야 한다.

```text
local:
  .\scripts\check.ps1

CI:
  .\scripts\check.ps1
```

### 24.2 CI 단계

```text
CI quick:
  - cargo fmt
  - cargo clippy
  - cargo test
  - docs-check

CI oracle:
  - setup EnergyPlus oracle
  - oracle smoke
  - selected compare

CI nightly:
  - full regression subset
  - issue-based regression
  - performance regression
  - docs build
  - package dry run
```

### 24.3 PR gate

초기 PR gate:

```text
- quick CI 통과
- 관련 test 추가
- docs/porting-map 업데이트
- supported object table 업데이트, 해당 시
- data architecture 문서 업데이트, 해당 시
```

release branch gate:

```text
- oracle CI 통과
- regression subset 통과
- performance regression 확인
- release notes 업데이트
```

---

## 25. Branch 전략

```text
main:
  항상 build/test 가능한 개발 중심 branch

release/eplus26.1:
  EnergyPlus 26.1.0 oracle 기준 stable release branch

compat/eplus27.1:
  추후 oracle version migration 검토 branch

experimental/*:
  solver 개선, algorithm 실험
```

규칙:

```text
- oracle version 변경은 main에서 직접 하지 않는다.
- compat branch에서 baseline 변화와 migration issue를 먼저 검토한다.
- release branch에는 검증된 bugfix만 backport한다.
- experimental branch 결과는 compatibility branch에 직접 merge하지 않는다.
```

---

## 26. Coding 규칙

### 26.1 Rust style

```text
- rustfmt 필수
- clippy -D warnings
- panic 금지, library code에서는 Result 반환
- unwrap/expect는 test 또는 명확한 invariant에만 허용
- unit type/newtype 사용 권장
- raw string name은 model loading 이후 typed ID로 변환
- unsafe 사용은 별도 문서와 benchmark/test 없이는 금지
```

### 26.2 Error/Diagnostics

library 내부 error와 user-facing diagnostic을 구분한다.

```rust
pub enum ModelError {
    MissingReference { object: ObjectRef, field: FieldName, target: String },
    UnsupportedObject { object_type: String },
    InvalidEnumValue { object: ObjectRef, field: FieldName, value: String },
}
```

diagnostic 예:

```json
{
  "severity": "severe",
  "code": "MissingZoneEquipmentListReference",
  "object_type": "ZoneHVAC:LowTemperatureRadiant:ConstantFlow",
  "object_name": "BTKUEHLUNG_1DIM",
  "message": "The zone equipment object is connected to plant loops but is not listed in ZoneHVAC:EquipmentList.",
  "suggested_action": "Add the object to the corresponding ZoneHVAC:EquipmentList."
}
```

### 26.3 Source reference

포팅된 구현은 porting-map에 기록한다.

```text
Rust file:
  crates/ep_schedule/src/compact.rs

Reference:
  docs/src/porting-map/schedule.md
```

---

## 27. Supported object coverage 관리

### 27.1 coverage 단계

각 EnergyPlus object는 다음 상태를 가진다.

```text
NotStarted
Parsed
Validated
Typed
ReferenceResolved
GraphResolved
Planned
Initialized
Simulated
OutputCompared
TraceCompared
Documented
```

### 27.2 table 예시

| Object | Parse | Validate | Typed | Ref | Graph | Plan | Simulate | Compare | Notes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| Version | yes | yes | yes | n/a | n/a | n/a | n/a | yes | |
| Building | yes | yes | yes | n/a | partial | partial | partial | partial | |
| Timestep | yes | yes | yes | n/a | n/a | yes | yes | yes | |
| RunPeriod | yes | yes | yes | n/a | n/a | yes | yes | yes | |
| Site:Location | yes | yes | yes | n/a | n/a | yes | yes | yes | |
| Zone | yes | yes | yes | yes | yes | yes | yes | yes | |
| BuildingSurface:Detailed | yes | yes | yes | yes | yes | partial | partial | partial | |
| FenestrationSurface:Detailed | planned | planned | planned | planned | planned | planned | planned | planned | |
| Schedule:Constant | yes | yes | yes | yes | n/a | yes | yes | yes | |
| Schedule:Compact | partial | partial | partial | yes | n/a | partial | partial | partial | |
| ZoneHVAC:IdealLoadsAirSystem | planned | planned | planned | planned | planned | planned | planned | planned | |
| PlantLoop | planned | planned | planned | planned | planned | no | no | no | |

이 표는 가능하면 자동 생성한다.

---

## 28. 사용자 경험 설계

### 28.1 EnergyPlus-like simple mode

```powershell
eplus-rs -w weather.epw -d output input.epJSON
```

결과:

```text
output/
  eplusrs.err
  eplusrs.audit
  eplusrs.csv
  eplusrs.sql optional
  result.json
  diagnostics.json
  trace.json optional
  profile.json optional
```

### 28.2 Modern explicit mode

```powershell
eplus-rs model validate input.epJSON
eplus-rs compile input.epJSON --out build\case.eplusrs-model
eplus-rs graph validate input.epJSON
eplus-rs run build\case.eplusrs-model -w weather.epw -d output --trace
eplus-rs result export output --format csv
eplus-rs compare output baseline
```

### 28.3 실패 방식

지원하지 않는 객체/구조는 조용히 무시하지 않는다.

```text
- unsupported object
- unsupported topology
- unsupported algorithm
- unsupported output variable
```

은 structured diagnostic으로 명확히 실패해야 한다.

---

## 29. Security/reproducibility

### 29.1 다운로드 검증

setup에서 다운로드하는 모든 binary/source archive는 SHA256을 검증한다.

```text
- EnergyPlus binary
- EnergyPlus source archive
- optional vendored dependencies
```

### 29.2 외부 환경 차단

oracle 실행 시 다음을 가능한 범위에서 격리한다.

```text
- PATH 최소화
- PYTHONPATH 제거
- VIRTUAL_ENV 무시
- working directory 명시
- executable absolute path 사용
```

### 29.3 License/NOTICE

EnergyPlus binary/source를 reference/oracle로 사용할 때 license notice를 유지한다.

`tools/oracle/NOTICE.md`:

```text
This project uses an unmodified EnergyPlus release as a reference oracle
for compatibility testing and porting.

Reference version:
- EnergyPlus 26.1.0
- Source tag: v26.1.0

This project is not the official EnergyPlus distribution.
```

---

## 30. 최종 개발 순서 요약

```text
0. repo skeleton
1. rust-toolchain + Cargo workspace
2. Rust-only policy
3. setup.ps1
4. portable EnergyPlus binary/source lock
5. oracle smoke
6. docs skeleton
7. epJSON RawModel loader
8. TypedModel
9. name reference → typed ID
10. model compiler
11. compiled schedule/weather
12. graph validation
13. execution plan
14. SimulationState
15. ResultStore / DiagnosticStore
16. first runnable subset
17. TraceStore
18. compare output/trace
19. IdealLoads
20. simple HVAC
21. Air/Plant graph
22. issue-based regression
23. performance regression
24. v1.0 stable compatibility subset
```

---

## 31. 팀 작업 규칙

### 31.1 새로운 기능 PR에 필요한 것

```text
- 구현 코드
- unit test
- integration test 또는 testcase
- docs/porting-map 업데이트
- supported object table 업데이트, 해당 시
- data architecture 문서 업데이트, 해당 시
- compatibility note, 해당 시
```

### 31.2 새로운 object 추가 시

```text
1. ep_schema 확인
2. ep_raw_model parse 확인
3. ep_model typed struct 추가
4. default/enum/unit 처리
5. reference resolution 필요 여부 확인
6. graph relation 필요 여부 확인
7. execution plan 필요 여부 확인
8. simulation 필요 여부 확인
9. output variable 필요 여부 확인
10. testcase 추가
11. EnergyPlus oracle baseline 생성
12. docs 업데이트
```

### 31.3 계산부 수정 시

```text
- 기존 regression 결과 변화 확인
- first divergence timestep 확인
- performance profile 변화 확인
- tolerance 변경이 필요한 경우 문서/회의 필요
- compatibility mode 결과 변화는 release note에 기록
```

### 31.4 자료구조 수정 시

```text
- compile artifact compatibility 확인
- existing testcases recompile 확인
- graph/execution plan 변화 확인
- profile 비교
- docs/data-architecture 업데이트
```

### 31.5 experimental 기능

```text
- 기본값으로 켜지지 않음
- mode로 격리
- compatibility regression 실패를 유발하면 안 됨
- release note에서 experimental로 명시
```

---

## 32. References

이 문서는 아래 공개 자료를 기준으로 작성되었다. 프로젝트 시작 시점과 release 전에는 각 링크의 최신 상태를 다시 확인한다.

1. EnergyPlus GitHub repository  
   https://github.com/NREL/EnergyPlus

2. EnergyPlus 26.1.0 GitHub release  
   https://github.com/NREL/EnergyPlus/releases/tag/v26.1.0

3. EnergyPlus epJSON Input Schema documentation  
   https://energyplus.readthedocs.io/en/latest/schema.html

4. EnergyPlus State API documentation  
   https://energyplus.readthedocs.io/en/latest/state.html

5. EnergyPlus API usage documentation  
   https://bigladdersoftware.com/epx/docs/23-2/input-output-reference/api-usage.html

6. EnergyPlus Auxiliary Programs documentation  
   https://energyplus.readthedocs.io/en/latest/auxiliary-programs/auxiliary-programs.html

7. EnergyPlus IDF/epJSON conversion notes  
   https://bigladdersoftware.com/epx/docs/22-1/essentials/essentials.html

8. EnergyPlusRegressionTool  
   https://github.com/NREL/EnergyPlusRegressionTool

9. Issue #11615: Virtualenv leaks into PythonEngine  
   https://github.com/NREL/EnergyPlus/issues/11615

10. Issue #11610: VRF Scheduled master thermostat  
    https://github.com/NREL/EnergyPlus/issues/11610

11. Issue #11608: Plant/Condenser operation scheme priority  
    https://github.com/NREL/EnergyPlus/issues/11608

12. Issue #11599: Variable speed pump and two-way common pipe hard crash  
    https://github.com/NREL/EnergyPlus/issues/11599

13. Issue #4787: Zone equipment omitted from equipment list causing cryptic errors  
    https://github.com/NREL/EnergyPlus/issues/4787

---

## 33. 최종 요약

이 프로젝트는 “Rust로 빠르게 다시 만든 EnergyPlus”가 아니다. 목표는 다음이다.

```text
EnergyPlus 26.1.0과 같은 답을 낼 수 있는 compatibility-first Rust engine을 만들고,
그 과정에서 EnergyPlus가 장기적으로 가야 할 구조를 명확히 제안한다.
```

v2 기준 가장 중요한 변화는 다음이다.

```text
1. 구현 언어는 Rust-only로 고정한다.
2. 성능 개선은 외부 kernel 혼합이 아니라 자료구조와 runtime architecture에서 찾는다.
3. EnergyPlus model을 interpreter식으로 처리하지 않고 compile-stage artifact로 만든다.
4. runtime은 typed ID, graph, execution plan, ResultStore 중심으로 구성한다.
5. compatibility mode는 deterministic scalar Rust path로 유지한다.
6. fast/experimental mode는 compatibility를 침범하지 않는다.
```

최종 release의 신뢰성은 기능 개수보다 다음에서 나온다.

```text
- reproducible setup
- version-locked oracle
- reference source map
- schema-native model
- model compiler
- typed ID reference resolution
- explicit SimulationState
- graph-first validation
- execution plan runtime
- structured diagnostics
- trace-based regression
- performance regression
- transparent release notes
```
