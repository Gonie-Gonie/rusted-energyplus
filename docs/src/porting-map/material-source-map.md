---
status: active
claim_level: source-mapped
owner: compiler
last_reviewed: 2026-07-16
---

# Material Object Family Source Map

Reference version: EnergyPlus 26.1.0

Reference tag and commit:

- `v26.1.0`
- `6f2e40d10250a105b49966baa24d843711e61048`

Primary reference sources:

- `Energy+.schema.epJSON` for the public object inventory, required fields,
  defaults, enums, and numeric limits
- `src/EnergyPlus/HeatBalanceManager.cc::GetHeatBalanceInput` for common
  material-input orchestration
- `src/EnergyPlus/Material.cc::GetWindowGlassSpectralData`,
  `GetMaterialData`, `GetVariableAbsorptanceInput`, and
  `CalcScreenTransmittance`
- `src/EnergyPlus/Material.hh` for the base-material class hierarchy
- `src/EnergyPlus/WindowManager.cc::CalcWindowScreenProperties`,
  `ReportGlass`, and `CalcNominalWindowCond` for bounded screen initialization,
  ordinary-window construction/material EIO, and Blind report-skip behavior
- `src/EnergyPlus/PhaseChangeModeling/HysteresisModel.cc::GetHysteresisData`
- `src/EnergyPlus/HeatBalFiniteDiffManager.cc::GetCondFDInput`
- `src/EnergyPlus/MoistureBalanceEMPDManager.cc::GetMoistureBalanceEMPDInput`
- `src/EnergyPlus/HeatBalanceHAMTManager.cc::GetHeatBalHAMTInput`
- `src/EnergyPlus/DataSurfaces.cc::GetVariableAbsorptanceSurfaceList`
- `src/EnergyPlus/HeatBalanceSurfaceManager.cc::UpdateVariableAbsorptances`

## Inventory Boundary

The EnergyPlus 26.1 schema exposes exactly 34 objects in this checkpoint's
material-family boundary:

- 22 base material definitions named `Material`, `Material:*`, or
  `WindowMaterial:*`
- 12 `MaterialProperty:*` overlays or datasets that attach behavior or data to
  a base material

`WindowGap:DeflectionState` and `WindowGap:SupportPillar` are referenced by
`WindowMaterial:Gap`, but they are not part of this 34-object name boundary.
`WindowProperty:*`, `WindowThermalModel:Params`, constructions, frames and
dividers, and surface properties are also separate inventories.

Inventory coverage and typed implementation coverage are deliberately
different:

| Measure | Count | Checkpoint meaning |
|---|---:|---|
| inventoried public objects | 34 / 34 | Every in-boundary EnergyPlus 26.1 object is named below with its source owner and order. |
| base definitions | 22 / 22 inventoried | `GetMaterialData` processing order is locked below. |
| overlays and datasets | 12 / 12 inventoried | Common-startup or algorithm-local owner and order are locked below. |
| typed Rust material variants | 22 | Five complete opaque-family slices, the `WindowMaterial:Glazing` `SpectralAverage` branch, and the complete `RefractionExtinctionMethod`, glazing `EquivalentLayer`, `WindowMaterial:Gas`, gap `EquivalentLayer`, `WindowMaterial:GasMixture`, ordinary `WindowMaterial:Shade`, shade `EquivalentLayer`, drape `EquivalentLayer`, ordinary `WindowMaterial:Screen`, screen `EquivalentLayer`, ordinary `WindowMaterial:Blind`, blind `EquivalentLayer`, thermochromic glazing-group, simple-glazing-system, complex-fenestration gap, and complex-fenestration shade objects have distinct payloads. |
| complete bounded base-definition slices | 21 / 22 | `Material`, `Material:NoMass`, `Material:AirGap`, `Material:InfraredTransparent`, `WindowMaterial:Glazing:RefractionExtinctionMethod`, `WindowMaterial:Glazing:EquivalentLayer`, `WindowMaterial:Gas`, `WindowMaterial:Gap:EquivalentLayer`, `WindowMaterial:GasMixture`, `WindowMaterial:Shade`, `WindowMaterial:Shade:EquivalentLayer`, `WindowMaterial:Drape:EquivalentLayer`, `WindowMaterial:Screen`, `WindowMaterial:Screen:EquivalentLayer`, `WindowMaterial:Blind`, `WindowMaterial:Blind:EquivalentLayer`, `Material:RoofVegetation`, `WindowMaterial:GlazingGroup:Thermochromic`, `WindowMaterial:SimpleGlazingSystem`, `WindowMaterial:Gap`, and `WindowMaterial:ComplexShade` have their source-effective fields and bounded compiler contracts typed. |
| standalone typed datasets | 1 / 12 | `MaterialProperty:GlazingSpectralData` is typed in a separate deterministic standalone arena and name map; it is not a `MaterialDefinition` variant. |
| typed material overlays | 3 / 12 | `MaterialProperty:VariableAbsorptance` is typed in a separate overlay arena after its eligible base-material and schedule dependencies are available. `MaterialProperty:PhaseChangeHysteresis` and `MaterialProperty:PhaseChange` are typed in separate attachment arenas keyed directly by their existing material targets. None is a `MaterialDefinition` variant. |
| complete bounded public-object slices | 25 / 34 | The 21 complete base-definition slices, standalone glazing spectral dataset, variable-absorptance overlay, phase-change-hysteresis attachment, and phase-change temperature-enthalpy attachment are complete within their declared bounded compiler contracts. |
| partial bounded public-object slices | 1 / 34 | Only `WindowMaterial:Glazing` with `Optical Data Type = SpectralAverage` is typed; `Spectral`, `SpectralAndAngle`, and `BSDF` remain explicitly unsupported. |
| wholly deferred public objects | 8 / 34 | The remaining 8 overlays/datasets are wholly deferred; no base definition is wholly deferred. |

The inventory scaffold began at CP58; the counts and typed states above are
cumulative through the current checkpoint. Complete inventory does not mean
complete schema, validation, runtime, optics, moisture, phase-change, or
heat-transfer behavior.

## Common Startup Order

`GetHeatBalanceInput` establishes this common material-input order:

1. `GetWindowGlassSpectralData` reads
   `MaterialProperty:GlazingSpectralData`.
2. `GetMaterialData` reads the 22 base definition families in the exact order
   below, then calls `GetVariableAbsorptanceInput` at its tail.
3. `GetHysteresisData` reads
   `MaterialProperty:PhaseChangeHysteresis`.

The spectral dataset therefore exists before a glazing definition resolves a
spectral-data reference. Variable absorptance exists only after base materials
have been created; Rust publishes its typed overlay only after the shared
schedule namespace is also available. Rust then publishes the hysteresis
attachment in the same source-relative order; EnergyPlus upgrades an
already-created public `Material` or `Material:NoMass` target in place.

`MaterialProperty:PhaseChange` is not part of common startup. EnergyPlus reads
it lazily only when a ConductionFiniteDifference surface causes
`GetCondFDInput` to run. That routine first reads
`HeatBalanceSettings:ConductionFiniteDifference`, then PhaseChange, and then
`MaterialProperty:VariableThermalConductivity`. Rust publishes its immutable
PhaseChange attachment after the common-startup hysteresis attachment but
validates it eagerly for every compiled model; the no-CondFD source branch and
this deliberate fail-closed difference are part of the contract below.

## Standalone Glazing Spectral Dataset Typed Contract

`MaterialProperty:GlazingSpectralData` is a complete bounded standalone input
checkpoint, not a twenty-third material variant. The compiler reads it before
all `GetMaterialData` base definitions and stores it in a separate normalized
name map and deterministic compiler-ordered dataset arena. Every definition is
parsed and validated even when unused. The fixed-then-extension point vector
preserves source field order, but broad IDF/epJSON dataset declaration-order
parity is not claimed. A valid unused dataset remains runtime-inert inside an
otherwise-supported model: it adds no `MaterialDefinition`, construction
layer, arbitrary-run blocker, or runtime execution state.

The positional numeric stream mirrors the EnergyPlus 26.1 InputProcessor
shape. Fixed keys are read in quartet order
`wavelength_N, transmittance_N, front_reflectance_N, back_reflectance_N` for
`N = 1..4`, followed by extension objects with the same four fields. With no
extensions, the last present fixed numeric field determines the active span;
omitted positions inside that span read as zero. A name-only definition has
zero points. A non-empty extension establishes all sixteen fixed numeric
positions before its extension points, so omitted fixed values are again
zero-filled. The active span must end on a complete quartet, and zero through
800 points are accepted; an incomplete final quartet or a point count above
800 fails closed before a typed model is published.

Each source-ordered point applies the bounded source rules:

- transmittance below 0.001 is silently replaced by 0.001 before validation
- wavelength is in `[0.1, 4.0]` microns and strictly increases
- transmittance is at most 1.01
- front and back reflectance are each in `[0, 1.02]`
- transmittance plus either directional reflectance is at most 1.03

Normalized duplicate dataset names and all invalid point/order/count states
fail closed. This dataset checkpoint does not activate any glazing consumer.
`WindowMaterial:Glazing` `Spectral` remains compiler-blocked despite the typed
dataset arena; `SpectralAndAngle` and `BSDF` remain blocked on their separate
data and complex-fenestration paths. Spectral reference linkage,
interpolation, angular or hemispherical optics, WindowManager consumers,
constructions, surfaces, EIO serialization, runtime numerical behavior, broad
dataset declaration-order parity, exact diagnostic text/order/multiplicity,
and conformance remain unclaimed.

### `GetWindowGlassSpectralData` state contract

<!-- routine-state-contract:v1 begin get_window_glass_spectral_data -->
GetWindowGlassSpectralData

read_state:
- `MaterialProperty:GlazingSpectralData` definitions at the common-startup stage in deterministic compiler order: normalized name plus the positional numeric stream formed from fixed quartets 1 through 4 followed by extensible quartets; a name-only object has zero points, while omitted values inside the active numeric span read as zero
- for each complete quartet, wavelength, transmittance, front reflectance, and back reflectance; point count is bounded from zero through 800, and every dataset is validated even when no glazing references it

write_state:
- a separate normalized glazing-spectral-data name map and deterministic standalone `GlazingSpectralData` arena whose records own source-field-ordered `GlazingSpectralPoint` vectors; neither arena is a `MaterialDefinition` variant
- source-effective validated point values: transmittance below 0.001 is silently replaced by 0.001; wavelength remains in [0.1,4.0] microns and strictly increasing, transmittance is at most 1.01, each reflectance is in [0,1.02], and transmittance plus either reflectance is at most 1.03
- compile failure before typed-model publication for an incomplete final quartet, more than 800 points, duplicate normalized dataset names, invalid wavelength/order, invalid reflectance/transmittance, or either optical-sum violation; source warning/severe/fatal wording is not claimed

history_state_ownership:
- no cross-call or runtime history; the compiled model owns immutable dataset descriptors and point vectors for the lifetime of that typed model

unsupported_state:
- glazing-material dataset reference resolution and every active `WindowMaterial:Glazing` `Spectral`, `SpectralAndAngle`, or `BSDF` consumer; WindowManager optical tables, constructions, surfaces, reporting, and runtime state

inactive_branches:
- valid unused datasets, including zero-point name-only definitions, remain compile-time data only and add no arbitrary-run blocker or runtime execution state
- the no-extension branch derives its numeric span from the last present fixed field; any non-empty extension establishes all four fixed quartets before the extension points, with omitted positions zero-filled as by the source InputProcessor

unsupported_active_branches:
- `WindowMaterial:Glazing` `Spectral` remains compiler-blocked despite the dataset arena; `SpectralAndAngle` and `BSDF` remain blocked on their separate data and complex-fenestration paths

not_claimed_branches:
- active spectral reference linkage, interpolation or angular/hemispherical optics, WindowManager consumers, constructions, surfaces, EIO serialization, runtime numerical behavior, broad dataset declaration-order parity, exact diagnostic text/order/multiplicity, and conformance
<!-- routine-state-contract:v1 end get_window_glass_spectral_data -->

## Variable Absorptance Overlay Typed Contract

`MaterialProperty:VariableAbsorptance` is a complete bounded typed-input
overlay, not a material variant. Each record owns a normalized overlay name,
a resolved `MaterialId`, and either scheduled or function-driven control state
in a separate deterministic arena. Only `Material` and `Material:NoMass`
targets are accepted because those are the two public inputs created with
EnergyPlus `Group::Regular`; AirGap, InfraredTransparent, RoofVegetation, and
every window family fail closed. Overlay names use a separate namespace, so an
overlay may have the same name as its target material.

All four optional dependency names are read before the selected control is
validated. A nonblank name that does not resolve has the source-effective null
pointer state and is silently discarded. Scheduled control requires at least
one resolved thermal or solar schedule and rejects any resolved function.
Function control requires at least one resolved thermal or solar Curve/Table
identity and rejects any resolved schedule. The three function signals are
`SurfaceTemperature`, `SurfaceReceivedSolarRadiation`, and
`SpaceHeatingCoolingMode`; missing or blank control defaults to
`SurfaceTemperature`. User schedules resolve through the typed shared
`ScheduleId` namespace, while the two EnergyPlus built-ins `Constant-0.0` and
`Constant-1.0` have explicit typed sentinels. Function references retain the
normalized identity and object type of one of the twenty EnergyPlus 26.1
`Curve:*` families or `Table:Lookup`; their payload validation, dimensions,
interpolation, and evaluation remain deferred with those raw-only dependency
objects. The incidental AirflowNetwork wind-pressure-coefficient alias in the
same C++ curve map is outside the public Curve/Table field contract and fails
closed in this checkpoint.

EnergyPlus permits multiple differently named overlays to overwrite the same
material in input order. Rust does not yet recover IDF declaration order for
this object, so the bounded checkpoint accepts at most one overlay per target
and fails closed on a second target occurrence. Normalized overlay-name
duplicates, ambiguous Curve/Table identities, collisions between a user
schedule and either built-in schedule name, invalid fields, missing/wrong-type
targets, selected-family absence, and resolved opposite-family dependencies
all fail before overlay identity or target ownership is reserved. Broad
diagnostic and declaration-order parity remain unclaimed.

Every typed overlay, including one attached to an unused material, blocks
arbitrary runtime execution. Rust does not yet build the exterior-first-layer
surface list, evaluate schedules or functions, select the three trigger
signals, or apply the source `[0.0001, 0.9999]` clamp. In particular, runtime
support does not reproduce the EnergyPlus 26.1 scheduled-solar defect that
tests the solar schedule pointer but reads the thermal schedule pointer. The
object emits no dedicated EIO row, so this checkpoint adds compiler and
support-boundary tests but no case manifest, proof variable, runtime numerical
claim, or conformance claim.

### `GetVariableAbsorptanceInput` state contract

<!-- routine-state-contract:v1 begin get_variable_absorptance_input -->
GetVariableAbsorptanceInput

read_state:
- deterministic compiler-ordered `MaterialProperty:VariableAbsorptance` definitions after all 22 base material families; Rust delays publication until the shared schedule namespace is also typed
- normalized overlay name, required material name, defaulted four-way control signal, and optional thermal/solar function and schedule names
- existing base-material registry, typed user-schedule registry plus `Constant-0.0`/`Constant-1.0`, and the raw EnergyPlus 26.1 Curve/Table name namespace

write_state:
- a separate normalized overlay-name map and `MaterialVariableAbsorptance` arena; each record resolves exactly one `Material` or `Material:NoMass` target and stores either scheduled dependencies or one of the three function signals with deferred Curve/Table identities
- source-effective null state for each nonblank dependency name that does not resolve; selected control still requires at least one resolved dependency, and a resolved opposite-family dependency fails compilation
- compile failure before identity or target reservation for malformed fields, invalid control, missing or non-Regular target, ambiguous dependency identity, selected-family absence, resolved opposite-family state, normalized overlay-name duplicate, or a second overlay for one target

history_state_ownership:
- no runtime history in this checkpoint; the compiled model owns immutable overlay descriptors while surface activation and timestep updates remain unsupported

unsupported_state:
- Curve/Table payload typing, dimensional validation, interpolation and evaluation; exterior variable-absorptance surface-list construction; thermal/solar surface-array mutation; trigger evaluation; clamping; construction and surface behavior; reporting and conformance

inactive_branches:
- within the declared public Curve/Table and shared schedule namespaces, unresolved optional names become null as `Curve::GetCurve` or `Sched::GetSchedule` would; they are harmless when another selected-family dependency resolves
- overlay names share no namespace with material names, while at most one valid overlay may target each material in the bounded checkpoint

unsupported_active_branches:
- every valid overlay is typed but run-blocking, including overlays attached only to unused materials
- source last-wins multiplicity and IDF declaration-order recovery are deferred; a repeated target fails closed

not_claimed_branches:
- exact source diagnostic severity/text/order/early-return behavior, Curve/Table dependency validation, `GetVariableAbsorptanceSurfaceList`, `UpdateVariableAbsorptances`, the scheduled-solar pointer defect, EIO serialization, runtime numerical behavior, and conformance
<!-- routine-state-contract:v1 end get_variable_absorptance_input -->

## Phase-Change Hysteresis Attachment Typed Contract

`MaterialProperty:PhaseChangeHysteresis` is a complete bounded typed-input
attachment, not a material variant. Its object key is the referenced material
name itself; there is no independent overlay name or namespace. Each valid
record retains a normalized target snapshot, resolves a `MaterialId`, and is
stored in a deterministic `MaterialPhaseChangeHysteresis` arena with its own
typed ID. The underlying `MaterialDefinition` stays unchanged.

The actual EnergyPlus 26.1 gate accepts `Group::Regular`. Both public
`Material` and `Material:NoMass` are created in that group, even though the
schema memo describes a regular material and NoMass remains R-only. Rust
therefore accepts those two public target variants and rejects AirGap,
InfraredTransparent, RoofVegetation, and every window family. EnergyPlus can
also resolve internally generated `~FC_Concrete` and `~FC_Insulation_n`
targets; those aliases remain outside this bounded public target set because
Rust does not yet synthesize F/C-factor materials.

All thirteen numeric fields are required, finite, and strictly greater than
zero, including both Celsius peak temperatures. In source field order they
are total latent heat; liquid conductivity, density, and specific heat;
high/peak/low melting-curve temperatures; solid conductivity, density, and
specific heat; and high/peak/low freezing-curve temperatures. The schema and
reader define no defaults, enums, upper bounds, or relationship checks among
the two peaks, curve widths, or solid/liquid properties. The typed attachment
groups the liquid and solid states and the melting and freezing curves, then
stores the two source-initialized derived values: transition specific heat is
the solid/liquid mean, and initial prior specific heat is the solid value.

Numeric and key validation precede target reservation. A blank key, missing
or wrong-family target, malformed/nonpositive field, or second
case-insensitive attachment for the same material fails closed. An invalid
first occurrence does not reserve the target or consume an ID. A valid
hysteresis attachment may coexist on the same Regular target with the earlier
typed `MaterialProperty:VariableAbsorptance` overlay, matching the common
startup order.

Every typed attachment, including one on an unused material, blocks arbitrary
runtime execution. Rust does not replace base-material pointers, set mutable
`hasPCM` state, allocate CondFD node histories, calculate hysteretic enthalpy,
specific heat, density, or conductivity, or supply `ThermalStorage:PCM`.
EnergyPlus 26.1 also contains material-shared history, R-layer cast,
transition-state, curve-selection, EMPD/HAMT replacement, and EMS pointer
hazards in these downstream paths; none is executed by this checkpoint. The
object emits no dedicated EIO row, so this checkpoint adds compiler and
support-boundary tests but no case manifest, proof variable, runtime numerical
claim, or conformance claim.

### `GetHysteresisData` state contract

<!-- routine-state-contract:v1 begin get_hysteresis_data -->
GetHysteresisData

read_state:
- deterministic compiler-ordered `MaterialProperty:PhaseChangeHysteresis` definitions after the earlier variable-absorptance overlay; the object key is the existing material reference and has no independent overlay namespace
- thirteen required finite numeric fields in source order: total latent heat; liquid conductivity/density/specific heat; high/peak/low melting-curve temperatures; solid conductivity/density/specific heat; and high/peak/low freezing-curve temperatures; every value is strictly greater than zero, with no defaults, enums, upper bounds, or cross-field rules
- the existing public base-material registry, where actual EnergyPlus `Group::Regular` admits `Material` and `Material:NoMass`; Rust does not synthesize internal `~FC_Concrete` or `~FC_Insulation_n` targets

write_state:
- a separate deterministic `MaterialPhaseChangeHysteresis` attachment arena whose records own a typed ID, normalized target snapshot, resolved `MaterialId`, total latent heat, grouped liquid/solid thermal states, and grouped melting/freezing curves; the object key is not published as a separate name map
- source-initialized transition specific heat equal to the solid/liquid mean and initial prior specific heat equal to the solid value; the referenced base `MaterialDefinition` remains unchanged
- compile failure before attachment ID or target reservation for a blank key, missing or non-Regular-group public target, any missing/malformed/nonpositive numeric field, or a second case-insensitive attachment for one material

history_state_ownership:
- no mutable phase, reversal, enthalpy, or prior-specific-heat history in this checkpoint; the compiled model owns immutable attachment descriptors while every downstream consumer remains unsupported

unsupported_state:
- source material-pointer replacement and `hasPCM`; CondFD material/node allocation, phase and reversal histories, hysteretic enthalpy/Cp/density/conductivity evaluation, construction and surface behavior, `ThermalStorage:PCM`, EMS pointer rebinding, reporting, and conformance

inactive_branches:
- all-positive inputs remain valid regardless of melting/freezing peak order or liquid/solid property relationships because the source defines no cross-field constraint
- `Material:NoMass` passes the source `Group::Regular` gate even though downstream CondFD R-only branches can bypass PCM evaluation

unsupported_active_branches:
- every valid attachment is typed but run-blocking, including one attached only to an unused material
- internally generated F/C-factor material targets and coexistence with deferred PhaseChange, VariableThermalConductivity, EMPD, or HAMT inputs remain outside the executable boundary

not_claimed_branches:
- exact source diagnostic severity/text/order/multiplicity, material-pointer replacement and EMS-address behavior, mutable hysteresis state-machine and known source defects, CondFD or PCM-storage numerical behavior, generic CondFD EIO/output variables, runtime numerical behavior, and conformance
<!-- routine-state-contract:v1 end get_hysteresis_data -->

## Phase-Change Temperature-Enthalpy Attachment Typed Contract

`MaterialProperty:PhaseChange` is a complete bounded typed-input attachment,
not a material variant. Its object key is the referenced material name itself;
there is no independent attachment name or namespace. Each valid record owns a
typed ID, retains a normalized target snapshot, resolves a `MaterialId`, and is
stored in a separate deterministic `MaterialPhaseChange` arena. The underlying
`MaterialDefinition` stays unchanged and Rust does not invent a `hasPCM` flag.

The optional or blank
`temperature_coefficient_for_thermal_conductivity` defaults to zero and may be
any finite value. The optional source-ordered `values` array contains zero or
more complete `{temperature, enthalpy}` pairs; both members are required and
finite whenever a pair exists. Temperatures must strictly increase, while
enthalpies must be nondecreasing. Equal or negative enthalpies and negative
temperatures or coefficients therefore remain valid. EnergyPlus 26.1's
executable epJSON schema declares no minimum or maximum array length, so Rust
accepts zero, one, two, three, more than one hundred, and every other complete
pair count. It does not impose the textual IDD's stale three-through-one-hundred
description.

The actual source target gate is `Group::Regular`, which admits the public
`Material` and `Material:NoMass` variants. AirGap, InfraredTransparent,
RoofVegetation, and every window family fail closed. EnergyPlus can also resolve
internally generated `~FC_Concrete` and `~FC_Insulation_n` targets; those remain
outside this public bounded target set because Rust does not synthesize them.
Blank or missing targets, wrong families, malformed/incomplete/nonfinite pairs,
ordering violations, and a repeated case-insensitive PhaseChange target fail
before attachment identity or target ownership is reserved. EnergyPlus instead
performs these checks only when CondFD initializes and lets later repeated
targets overwrite earlier `MaterialFD` state. Rust validates eagerly and rejects
repetition because it does not recover a reliable source overwrite order.

EnergyPlus permits the same target to coexist with VariableAbsorptance,
PhaseChangeHysteresis, and later VariableThermalConductivity input. Rust types
the first two alongside PhaseChange; VariableThermalConductivity state remains
deferred. Every PhaseChange attachment, including one on an unused or NoMass material, blocks
arbitrary runtime execution. Rust does not allocate `MaterialFD`, replace an
empty table with the source three-point `-100` sentinel, interpolate enthalpy,
derive specific heat or conductivity, allocate CondFD construction/surface/node
state or history, reproduce Hysteresis/VTC precedence, or execute EMS and
reporting paths. Source-accepted one- and two-point tables are typed but never
executed because downstream CondFD assumes at least three points. The object
emits no dedicated EIO row, so this checkpoint adds compiler and
support-boundary tests but no case manifest, proof variable, runtime numerical
claim, or conformance claim.

### `GetCondFDInput` state contract

<!-- routine-state-contract:v1 begin get_cond_fd_input -->
GetCondFDInput

read_state:
- deterministic compiler-ordered `MaterialProperty:PhaseChange` definitions after the common-startup hysteresis attachment; the object key is the existing material reference, the optional or blank conductivity-temperature coefficient defaults to zero, and the optional source-ordered `values` array contains zero or more complete temperature/enthalpy pairs with no executable-schema item-count cap
- the existing public base-material registry, where actual EnergyPlus `Group::Regular` admits `Material` and `Material:NoMass`; Rust does not synthesize internal `~FC_Concrete` or `~FC_Insulation_n` targets
- finite coefficient, temperature, and enthalpy scalars with no scalar bounds; source-ordered temperatures must strictly increase and enthalpies must be nondecreasing, so equal or negative enthalpies and negative temperatures or coefficients remain valid

write_state:
- a separate deterministic `MaterialPhaseChange` attachment arena whose records own a typed ID, normalized target snapshot, resolved `MaterialId`, conductivity-temperature coefficient, and source-ordered vector of complete temperature/enthalpy points; the object key is not published as a separate name map
- the referenced base `MaterialDefinition` remains unchanged and no mutable `hasPCM` state is invented
- compile failure before attachment ID or target reservation for a blank or missing target, a non-Regular-group public target, malformed/incomplete/nonfinite values, temperature or enthalpy ordering violations, or a second case-insensitive PhaseChange attachment for one material

history_state_ownership:
- no mutable `MaterialFD`, sentinel, CondFD-node enthalpy, specific-heat, or timestep history in this checkpoint; the compiled model owns immutable attachment descriptors while every downstream consumer remains unsupported

unsupported_state:
- the preceding `HeatBalanceSettings:ConductionFiniteDifference` pass, following `MaterialProperty:VariableThermalConductivity` pass, `MaterialFD` allocation and `tk1`/`TempEnth`/three-point `-100` sentinel state, construction/surface/node allocation, interpolation and specific-heat/conductivity execution, EMS, generic CondFD reporting, and conformance

inactive_branches:
- `Material:NoMass` passes the source `Group::Regular` gate even though downstream R-only CondFD execution bypasses the temperature-enthalpy table
- zero points are valid typed empty state; Rust applies the executable-schema coefficient default to an entirely empty object and does not reproduce EnergyPlus 26.1's native-epJSON negative-presence-count defect or its later sentinel replacement
- EnergyPlus skips target and table-order validation when no CondFD surface invokes `GetCondFDInput`; Rust deliberately validates every definition eagerly and fails closed

unsupported_active_branches:
- every valid attachment is typed but run-blocking, including one attached only to an unused material
- source-accepted one- and two-point tables remain typed but are never executed because downstream CondFD directly assumes the first three points; the source's first-three-enthalpy-sum activation quirk for negative tables is also not executed
- internally generated F/C-factor targets and source last-wins repeated-target overwrite behavior remain outside the executable boundary

not_claimed_branches:
- the textual IDD three-through-one-hundred description, native-epJSON zero-pair presence-count behavior, exact diagnostic severity/text/order/multiplicity, source duplicate overwrite order, `MaterialFD` sentinel and interpolation/specific-heat/conductivity behavior including the known boundary-conductivity defect, Hysteresis/VTC precedence, generic EIO/output variables, runtime numerical behavior, and conformance
<!-- routine-state-contract:v1 end get_cond_fd_input -->

## Base Definition Source Order

The following table is the public-object processing order inside
`Material::GetMaterialData`. It is not the schema presentation order.

| Source order | Public object | Source role | Cumulative typed state |
|---:|---|---|---|
| 1 | `Material` | regular opaque material with thickness, conductivity, density, specific heat, roughness, and absorptances | bounded typed `Regular` variant |
| 2 | `Material:NoMass` | regular R-only opaque material with roughness, resistance, and absorptances | bounded typed `NoMass` variant |
| 3 | `Material:AirGap` | opaque air-space resistance material | bounded typed `AirGap` variant |
| 4 | `Material:InfraredTransparent` | infrared-transparent material | bounded typed `InfraredTransparent` variant |
| 5 | `WindowMaterial:Glazing` | detailed glazing definition | bounded typed `SpectralAverage` branch; `Spectral`, `SpectralAndAngle`, and `BSDF` deferred |
| 6 | `WindowMaterial:Glazing:RefractionExtinctionMethod` | glazing using refraction/extinction input | complete bounded typed variant with source-parity normal-incidence derivation |
| 7 | `WindowMaterial:Glazing:EquivalentLayer` | equivalent-layer glazing | complete bounded typed variant; dedicated consumer family, construction and runtime deferred |
| 8 | `WindowMaterial:Gas` | single-gas window gap | complete bounded typed variant with resolved standard/custom gas properties |
| 9 | `WindowMaterial:Gap:EquivalentLayer` | equivalent-layer gap | complete bounded typed variant with vent mode and resolved standard/custom gas properties |
| 10 | `WindowMaterial:GasMixture` | multi-gas window gap | complete bounded typed variant with an ordered one-to-four standard-gas mixture |
| 11 | `WindowMaterial:Shade` | window shade | complete bounded typed variant with source defaults, derived properties, and safe ordinary-window layering |
| 12 | `WindowMaterial:Shade:EquivalentLayer` | equivalent-layer shade | complete bounded typed variant with source defaults, asymmetric visible storage, and a deferred equivalent-layer construction consumer |
| 13 | `WindowMaterial:Drape:EquivalentLayer` | equivalent-layer drape | complete bounded typed variant with source defaults, asymmetric visible storage, source-effective pleats, and a deferred equivalent-layer construction consumer |
| 14 | `WindowMaterial:Screen` | exterior window screen | complete bounded typed variant with source defaults, solid-fraction optical projections, and safe exterior-only ordinary-window layering |
| 15 | `WindowMaterial:Screen:EquivalentLayer` | equivalent-layer screen | complete bounded typed variant with source sentinels, storage quirks, and a deferred equivalent-layer construction consumer |
| 16 | `WindowMaterial:Blind` | slatted blind | complete bounded typed variant with source-effective defaults, slat-property constraints, and safe ordinary-window layering |
| 17 | `WindowMaterial:Blind:EquivalentLayer` | equivalent-layer blind | complete bounded typed variant with source blank-group/index quirks, warning-only geometry recovery, and a deferred equivalent-layer construction consumer |
| 18 | `Material:RoofVegetation` | eco-roof material and vegetation state | complete bounded typed input and dry-soil opaque projection; dynamic EcoRoof runtime blocked |
| 19 | `WindowMaterial:GlazingGroup:Thermochromic` | thermochromic glazing-group parent | complete bounded typed ordered-state parent; construction generation and runtime selection deferred |
| 20 | `WindowMaterial:SimpleGlazingSystem` | derived simple glazing system | complete bounded typed block model; dedicated family, construction and runtime blocked |
| 21 | `WindowMaterial:Gap` | complex-fenestration gap, including optional deflection-state and support-pillar references | complete bounded typed variant with copied gas/helper state; dedicated consumer and runtime deferred |
| 22 | `WindowMaterial:ComplexShade` | complex-fenestration shade | complete bounded typed variant with source defaults/projections; dedicated consumer and runtime deferred |

Two conditional internal injections occur between public-object steps and must
not be mistaken for additional public schema variants:

- after `Material`, `~FC_Concrete` is created when any F-factor or C-factor
  construction exists
- after `Material:NoMass`, one `~FC_Insulation_*` R-only material is created
  for each such construction

The routine counts `Material:NoMass`, `Material:InfraredTransparent`, and
`Material:AirGap` before step 1, but those count reads do not change the
creation order above. After step 22, the routine performs optional material
reporting, registers EMS absorptance actuators for regular materials, and only
then calls `GetVariableAbsorptanceInput`.

## Overlay And Dataset Source Order

There is no single global order among CondFD, EMPD, and HAMT overlays. Each
manager reads its own input lazily on the first use of that heat-transfer
algorithm. The exact order guaranteed by each source owner is:

| Source sequence | Schema family number | Public object | Kind | Cumulative typed state |
|---|---:|---|---|---|
| common HB 1 | 34 | `MaterialProperty:GlazingSpectralData` | standalone glazing dataset read by `GetWindowGlassSpectralData` | complete bounded typed dataset; runtime-inert while unused |
| common HB 2 tail | 27 | `MaterialProperty:VariableAbsorptance` | base-material overlay read by `GetVariableAbsorptanceInput` after all 22 base families | complete bounded typed overlay; all definitions runtime-blocked |
| common HB 3 | 25 | `MaterialProperty:PhaseChangeHysteresis` | Regular-group material attachment read by `GetHysteresisData` | complete bounded typed attachment for public Material/NoMass targets; all definitions runtime-blocked |
| CondFD 1 | 24 | `MaterialProperty:PhaseChange` | temperature/enthalpy attachment read first by `GetCondFDInput` after its CondFD settings pass | complete bounded typed attachment for public Material/NoMass targets; all definitions runtime-blocked |
| CondFD 2 | 26 | `MaterialProperty:VariableThermalConductivity` | temperature/conductivity overlay read second by `GetCondFDInput` | deferred |
| EMPD 1 | 23 | `MaterialProperty:MoisturePenetrationDepth:Settings` | regular-material moisture overlay read by `GetMoistureBalanceEMPDInput` | deferred |
| HAMT 1 | 28 | `MaterialProperty:HeatAndMoistureTransfer:Settings` | HAMT base settings | deferred |
| HAMT 2 | 29 | `MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm` | HAMT sorption dataset | deferred |
| HAMT 3 | 30 | `MaterialProperty:HeatAndMoistureTransfer:Suction` | HAMT suction dataset | deferred |
| HAMT 4 | 31 | `MaterialProperty:HeatAndMoistureTransfer:Redistribution` | HAMT redistribution dataset | deferred |
| HAMT 5 | 32 | `MaterialProperty:HeatAndMoistureTransfer:Diffusion` | HAMT diffusion dataset | deferred |
| HAMT 6 | 33 | `MaterialProperty:HeatAndMoistureTransfer:ThermalConductivity` | HAMT thermal-conductivity dataset | deferred |

The schema family number preserves the canonical 34-object inventory order:
base definitions are 1 through 22 and the `MaterialProperty:*` entries are 23
through 34. The source-sequence column, not that schema number, controls
initialization behavior.

## Bounded Typed Contract

The bounded base-definition contract retains 21 complete variants plus one
explicitly partial `WindowMaterial:Glazing` variant for its `SpectralAverage`
branch. The standalone glazing spectral dataset is typed separately and does
not change the 22-variant material-definition count. The variable-absorptance
overlay is likewise held outside `MaterialDefinition`; it resolves only
Regular/NoMass targets and leaves their immutable base payloads unchanged.
The phase-change-hysteresis attachment also lives in a separate arena, uses
its object key directly as the Regular/NoMass material reference, and leaves
the base payload unchanged while its runtime upgrade remains blocked.

### `Material` / regular

EnergyPlus schema and source require:

- roughness
- thickness greater than 0 m
- conductivity greater than 0 W/m-K
- density greater than 0 kg/m3
- specific heat greater than or equal to 100 J/kg-K

Thermal, solar, and visible absorptance default to 0.9, 0.7, and 0.7. The
source derives nominal resistance as thickness divided by conductivity. The
bounded Rust variant owns these required values directly and derives both
area-normalized resistance and heat capacity.

### `Material:NoMass`

EnergyPlus schema and source require:

- roughness
- thermal resistance greater than or equal to 0.001 m2-K/W

Thermal, solar, and visible absorptance use the same 0.9, 0.7, and 0.7
defaults. EnergyPlus stores this as a regular material with `ROnly = true`;
the bounded Rust model represents that distinction directly as the `NoMass`
variant.

### `Material:AirGap`

EnergyPlus requires thermal resistance greater than 0 m2-K/W. The object has
no roughness or absorptance inputs: `GetMaterialData` fixes roughness to
`MediumRough`, stores the input as an R-only resistance, and leaves the base
thermal, solar, and visible absorptances at 0. The bounded Rust variant owns
the positive resistance directly and exposes those fixed source values.

Construction validation restricts an air gap to a middle layer. It cannot be
the outside/first or inside/final layer, so a construction containing an
`AirGap` must have material layers on both sides. Dynamic air-gap heat
transfer and window-gap families are outside this opaque R-only variant.

### `Material:InfraredTransparent`

The public object supplies only its name. `GetMaterialData` fixes its R-only
thermal resistance to 0.01 m2-K/W and its thermal, solar, and visible
absorptances to 0.9999, 1.0, and 1.0. The bounded Rust variant stores those
fixed source values without inventing user-configurable fields.

The intended construction invariant is a single IRT layer. Upstream
`CheckAndSetConstructionProperties` applies that check only when the outside
layer is infrared-transparent, leaving a malformed non-first-layer IRT input
outside that validation branch. The bounded Rust compiler enforces the
intended invariant wherever an IRT reference appears instead of reproducing
that upstream validation gap; exact malformed-input diagnostic parity is not
claimed.

### `WindowMaterial:Glazing` / `SpectralAverage`

The bounded fifth variant accepts only `Optical Data Type = SpectralAverage`.
It owns the EnergyPlus source fields for thickness; normal
solar and visible transmittance plus front/back reflectance; infrared
transmittance and front/back hemispherical emissivity; conductivity; dirt
correction factor; solar-diffusing state; Young's modulus; and Poisson's
ratio.

The compiler enforces the EnergyPlus 26.1 contract used by this branch:

- thickness and conductivity are greater than 0
- solar/visible transmittances and reflectances plus infrared transmittance
  are in `[0, 1]`
- front/back infrared emissivity and Poisson's ratio are in `(0, 1)`, and
  Young's modulus is greater than 0
- dirt correction is in `(0, 1]`
- each solar or visible transmittance-plus-corresponding-reflectance sum and
  each infrared transmittance-plus-corresponding-emissivity sum is at most 1
- the source defaults are retained, including 0 for omitted normal-incidence
  optical values, 0.84 emissivity, 0.9 W/m-K conductivity, 1.0 dirt
  correction, `No` solar diffusing, 72 GPa Young's modulus, and 0.22
  Poisson's ratio

`Spectral` is not approximated with zero optical properties. Although the
earlier `GetWindowGlassSpectralData` stage now owns a complete bounded typed
dataset arena, dataset-reference resolution and the active spectral glazing
consumer remain blocked. `SpectralAndAngle` remains blocked on bivariate
table/curve typing, and `BSDF` remains blocked on the complex-fenestration
path.

### `WindowMaterial:Glazing:RefractionExtinctionMethod`

The sixth source-order variant owns all fields declared by the EnergyPlus
26.1 object. Thickness, solar/visible indices of refraction, and
solar/visible extinction coefficients are required. Infrared transmittance,
one shared front/back infrared emissivity, conductivity, dirt correction, and
solar-diffusing state retain source defaults of 0, 0.84, 0.9 W/m-K, 1.0, and
`No`.

The compiler enforces thickness and both extinction coefficients greater than
0, both refraction indices greater than 1, infrared transmittance in
`[0, 1)`, emissivity in `(0, 1)`, conductivity greater than 0, and dirt
correction in `(0, 1]`. Infrared transmittance plus emissivity must be
strictly less than 1. Shared material-name reservation follows the source
order after regular glazing, so mixed-case duplicates against any earlier
opaque or glazing family fail.

For each solar or visible band the typed projection applies the EnergyPlus
26.1 equations with interface reflectivity
`rho = ((n - 1) / (n + 1))^2`, internal transmittivity
`tau = exp(-k * thickness)`, denominator `D = 1 - (rho * tau)^2`,
`T = tau * (1 - rho)^2 / D`, and
`R = rho * (1 + (1 - rho)^2 * tau^2 / D)`. Solar front/back
reflectance share the solar result. The visible front uses the visible
result, while visible back intentionally copies solar front reflectance,
matching the observable EnergyPlus 26.1 assignment rather than the
surrounding symmetric-glass comment. Raw n/k inputs remain in the typed
payload; EIO exposes only the derived optical values.

### `WindowMaterial:Glazing:EquivalentLayer`

The seventh source-order object accepts only `SpectralAverage`, which is also
the missing-or-blank default. EnergyPlus 26.1 lists `Spectral` in the schema
but rejects it in `GetMaterialData`; the optional spectral dataset name is
ignored for the only supported mode. Rust therefore validates that optional
field as a string but does not retain a reference that EnergyPlus never
consumes.

The typed payload preserves all 26 numeric inputs. Front/back beam-beam solar
transmittance and reflectance are required and bounded to `[0, 1]`.
Beam-beam visible values default to 0, all eight beam-diffuse solar/visible
values default to 0, and those optional numeric fields are also bounded to
`[0, 1]`. The six diffuse-diffuse solar/visible values preserve the
distinction between `Autocalculate` and an explicit bounded number. Shared
infrared transmittance defaults to 0 in `[0, 1]`, front/back infrared
emissivity default to 0.84 in `(0, 1)`, and thermal resistance defaults to
0.158 m2-K/W and must be greater than 0. The source block performs no
transmittance-plus-reflectance or infrared-plus-emissivity sum validation, so
this compiler slice deliberately does not invent one.

Equivalent-layer material has a dedicated `MaterialFamily::EquivalentLayer`
consumer boundary. A regular `Construction` fails closed if any layer uses
this family because upstream permits it only through
`Construction:WindowEquivalentLayer`, which remains raw-only. The fully typed
material is also explicitly run-blocked by the existing `Window*` unsupported
surface boundary.

This is static input typing, not an ASHWAT or window-runtime port. EnergyPlus
stores but does not pass the visible inputs into its EQL optical model, emits
none of them in EIO, overwrites runtime back beam-beam solar transmittance with
the front value, and uses thermal resistance only for movable insulation.
Those source-actual behaviors, equivalent-layer construction ratings,
fenestration surfaces, dynamic optics, and heat transfer remain unclaimed.

### `WindowMaterial:Gas`

The eighth source-order object requires `gas_type` and a thickness greater
than 0. The bounded enum is exactly `Air`, `Argon`, `Krypton`, `Xenon`, or
`Custom`. For the four standard gases, the typed payload uses the EnergyPlus
26.1 built-in conductivity, viscosity, specific-heat, molecular-weight, and
specific-heat-ratio constants. Optional custom-property fields are still
type-checked and schema-bounded when supplied, but their valid values are
discarded when the source replaces the property record with the selected
standard-gas constants.

For `Custom`, each gas property uses `A + B*T + C*T^2`. Blank coefficient
fields receive the input processor's numeric zero. Viscosity A and
specific-heat A are therefore effectively required and greater than 0, while
molecular weight is effectively required and schema-bounded to `[20, 200]`
g/mol. Conductivity A, B, and C have no individual sign restriction, but their
value at 300 K, `A + 300*B + 90000*C`, must be greater than 0; nominal thermal
resistance is thickness divided by that value. Viscosity B/C and
specific-heat B/C remain unrestricted. An explicitly supplied
`specific_heat_ratio` must be greater than 1, but a missing Custom ratio
remains 0, matching the EnergyPlus 26.1 legacy numeric-blank value that its
source block does not validate. The bounded implementation preserves this
observable schema/source mismatch instead of inventing an effective-required
ratio.

### `WindowMaterial:Gap:EquivalentLayer`

The ninth source-order object shares the single-gas property representation
and standard-gas constants above, but belongs to the equivalent-layer
consumer family. Its EnergyPlus 26.1 epJSON gas enum is unusually uppercase:
exactly `AIR`, `ARGON`, `KRYPTON`, `XENON`, or `CUSTOM`. Thickness must be
greater than 0, and `gap_vent_type` is required as exactly `Sealed`,
`VentedIndoor`, or `VentedOutdoor`. The schema note describes both vented
modes as air-only, but `GetMaterialData` does not enforce that relationship,
so the bounded compiler preserves rather than rejects a schema-valid
non-air/vented combination.

Standard gases replace valid supplied custom values with the same fixed
EnergyPlus 26.1 property records as `WindowMaterial:Gas`. `CUSTOM` preserves
the same three `A + B*T + C*T^2` coefficient sets, effective required
viscosity A, specific-heat A, and molecular weight, schema bounds, missing
specific-heat-ratio zero, positive 300 K conductivity check, and nominal
resistance derivation. No vent default is invented: although the source
storage initializes to `Sealed`, the 26.1 schema requires the field and the
source's alpha-blank guard checks the required gas field rather than the vent
field, so valid input always parses the supplied vent token.

The implementation deliberately stops before typing
`Construction:WindowEquivalentLayer` or adding ASHWAT runtime execution. In
particular, the 26.1 equivalent-layer transfer path copies specific-heat
coefficients into its viscosity slots, and its FRA evaluation repeats the
linear coefficient for each quadratic term. `BuildGap` also emits a severe
error and replaces the construction-local thickness with 0.0001 m when the
typed material thickness is below that value, while EIO continues to report
the original material thickness. Reproducing or correcting these source
quirks requires a later runtime boundary and is not claimed here.
Ordinary `Construction` rejects this equivalent-layer gap, arbitrary-run
assessment counts it as explicitly unsupported, and the typed payload is
never projected through an opaque-material accessor.

`window_material_gap_equivalent_layer_001` adds a separate nonblocking
diagnostic EnergyPlus 26.1 EIO gate. Its clean oracle fixture emits seven
`Construction:WindowEquivalentLayer` gap-layer occurrences across all five
gas types and all three vent modes, including one reused Argon material; one
valid but unreferenced Air definition is absent. The Rust bridge compares the
exact fixture sequence, material identity, canonical gas and vent types, and
thickness after the source `{:.3R}` serialization policy. Material definition
order differs from occurrence order. Construction names and IDF declarations
are aligned with converted-epJSON canonical order, so arbitrary IDF
declaration-order parity remains unclaimed. The exact construction, host, and
detailed-window rows are oracle-only fixture locks. EIO omits Custom
coefficients/properties and nominal resistance, and the gate adds no
construction typing/rating, optics, ASHWAT/BuildGap, runtime, or conformance
claim.

### `WindowMaterial:GasMixture`

The tenth source-order object has a positive thickness and an integer
`number_of_gases_in_mixture` from 1 through 4. Its component gas enum is
limited to `Air`, `Argon`, `Krypton`, and `Xenon`; `Custom` is not a
schema-valid mixture component. Gas 1 and Gas 2 type/fraction fields are
unconditionally required by the 26.1 schema even when the declared active
count is 1. The bounded compiler validates both required pairs and then
preserves only the declared active prefix, so a one-gas mixture deliberately
discards its schema-required Gas 2 dummy pair.

Gas 3 and Gas 4 fields are schema-optional. When either position is active,
its type is effectively required by `GetMaterialData`, but a missing active
fraction retains the input processor's numeric blank value of 0.0. An
explicit 0.0 fraction remains schema-invalid, while every explicitly supplied
fraction must be greater than 0 and no greater than 1. Supplied inactive
fields are still enum/type/bounds checked before valid inactive data is
discarded. Missing active gas types fail safely in Rust instead of reproducing
the 26.1 invalid-enum indexing path.

EnergyPlus applies no fraction-sum, uniqueness, or normalization rule.
Ordered active components therefore retain duplicate gases, non-unit sums,
and missing optional active fractions as zero. Every active component resolves
the exact standard-gas property record already shared by the single-gas
variants. Nominal resistance is intentionally thickness divided by only the
first component's 300 K conductivity; it is not a mixture-property average.
Runtime window routines later consume the ordered raw fractions, but that
thermal behavior is not implemented or claimed here.

The variant belongs to the ordinary fenestration family and participates in
the same `Glass (Gas-or-GasMixture Glass){0..3}` construction alternation as
`WindowMaterial:Gas`. It is outside the equivalent-layer family;
`Construction:WindowEquivalentLayer` typing and validation remain deferred.
The later typed `WindowMaterial:Gap` complex-fenestration reference path may
also consume a copied gas-mixture state, while its
`Construction:ComplexFenestrationState` consumer and runtime remain deferred.
Arbitrary-run assessment explicitly blocks the typed mixture before
execution.

EnergyPlus 26.1 prints the shared `WindowMaterial:Gas` EIO header when any gas
mixture exists but has no `GasMixture` data-row case in the construction-layer
report switch. Generic `Material Details` reporting can echo a mixture
definition's name, fixed roughness, and thickness, but it exposes none of the
component count, types, fractions, order, or first-gas nominal-resistance
shortcut. `window_material_gas_mixture_001` gates that bounded generic row
instead of inventing a dedicated mixture EIO shape. It compares all six typed
fixture definitions by normalized name, requires exactly one generic row per
definition including the unused mixture, and locks `MediumRough`, source
`{:.4R}` thickness serialization, and the fixed zero resistance,
conductivity, density, specific-heat, and absorptance columns. The shared
`WindowMaterial:Gas` header is exact and has zero gas data rows.

Those definition rows cannot establish component count, species, fractions,
order, first-gas nominal resistance, construction occurrence, reuse, or
whether a definition is used. The fixture's two exact seven-layer
`WindowConstruction` rows and its host/window rows are therefore oracle-only
integrity locks. Broad IDF declaration order, mixture conductivity or other
thermal properties, construction ratings, window runtime, surface behavior,
and conformance remain unclaimed.

### `WindowMaterial:Shade`

The eleventh source-order object requires six optical values plus positive
thickness and conductivity. Solar transmittance, solar reflectance, visible
transmittance, visible reflectance, and infrared transmittance are each
bounded from 0 inclusive to 1 exclusive. Infrared hemispherical emissivity is
greater than 0 and less than 1. The solar transmittance-plus-reflectance,
visible transmittance-plus-reflectance, and infrared
emissivity-plus-transmittance pairs must each remain strictly below 1.

EnergyPlus fixes roughness to `MediumRough`, derives solar absorptance as
`max(0, 1 - solar transmittance - solar reflectance)`, treats the material as
resistance-only, and derives nominal resistance as thickness divided by
conductivity. It does not assign visible absorptance while reading this
object, so the bounded payload deliberately preserves the source-initialized
zero instead of deriving `1 - visible transmittance - visible reflectance`.

The six optional airflow/placement fields retain their schema defaults:
shade-to-glass distance is 0.05 m in the inclusive range [0.001, 1], the top,
bottom, left, and right opening multipliers are each 0.5 in [0, 1], and
airflow permeability is 0 in [0, 0.8]. The compiler type-checks and bounds
every supplied value before constructing the distinct `WindowShadeMaterial`
payload.

The shade joins the ordinary fenestration family through a deliberately
bounded Construction subset. It accepts one exterior shade followed directly
by one through four panes in the ordinary
`Glass ((Gas|GasMixture) Glass){0..3}` alternation, or the corresponding
interior shade directly after the last glass. A between-glass shade is
accepted only as `Glass, Gap, Shade, Gap, Glass` for double glazing or
`Glass, Gap, Glass, Gap, Shade, Gap, Glass` for triple glazing. At most one
shade and eight total layers are allowed, and any shade combined with
solar-diffusing glass is rejected.

For a between-glass shade, the adjacent gap widths may differ by at most
0.0005 m. Their source-effective five-slot gas type/fraction signatures must
match exactly. A single-gas material leaves its four inactive gas-type slots
at the `Custom` default with zero fractions, whereas a gas mixture resets its
inactive slots to `Invalid` with zero fractions. A single gas and a one-gas
mixture therefore do not match even when their active species and fraction
are the same. Matching `Custom` single-gas gaps intentionally ignores
polynomial coefficient records because EnergyPlus 26.1 compares only gas
type, fraction, and width at this construction boundary.

Rust also rejects `Shade, Gap, Glass` and `Glass, Gap, Shade` instead of
reproducing EnergyPlus 26.1's inconsistent validation paths: the former can
reach an invalid adjacent-gap access, while the latter can pass despite the
diagnostic contract that exterior/interior shades must directly adjoin
glass. This is an explicit safety hardening, not a diagnostic-parity claim.
Arbitrary-run assessment counts every typed shade definition, including an
unused one, and blocks execution because window shading, optics, thermal
behavior, daylighting, and controls remain unported.

`window_material_shade_001` adds a separate nonblocking diagnostic
EnergyPlus 26.1 EIO gate. Its generic `Material Details` comparison is
definition keyed: every fixture Shade must appear exactly once, including an
unused definition. It locks `MediumRough`, zero resistance, density, specific
heat, and visible absorptance, source `{:.4R}` thickness, source `{:.3R}`
conductivity, and source `{:.4R}` infrared emissivity and derived solar
absorptance. The generic row exposes no visible reflectance, infrared
transmittance, shade-to-glass distance, opening multipliers, airflow
permeability, nominal resistance, occurrence, reuse, or control state.

The specialized `WindowMaterial:Shade` comparison is a duplicate-aware
construction-layer occurrence multiset. Reusing one Shade in multiple
ordinary window constructions emits repeated rows, while a definition absent
from every shade construction emits none. Material name plus source
`{:.3R}` thickness, conductivity, infrared emissivity, solar transmittance,
visible transmittance, and solar reflectance are exact, as is the one
specialized header. The clean fixture deliberately excludes between-glass
Shade reporting because EnergyPlus skips that construction's window report
after an oracle warning. Construction and surface rows are fixture-integrity
locks only. Broad row/declaration order, construction ratings, active
shading-control semantics, surface behavior, daylighting, optics/thermal
runtime, and conformance remain unclaimed.

### `WindowMaterial:Shade:EquivalentLayer`

The twelfth source-order object preserves all eleven EnergyPlus 26.1 numeric
inputs in a distinct equivalent-layer shade payload. Beam-beam solar
transmittance defaults to 0 in the inclusive range [0, 0.8]. Front/back
beam-diffuse solar transmittance and reflectance are required and individually
bounded [0, 1). The three optional visible inputs have no schema default, but
the source clears its numeric input buffer before reading each object, so a
blank or absent value remains 0. Infrared transmittance defaults to 0.05 in
[0, 1), and front/back infrared emissivity each default to 0.91 in (0, 1).

The compiler also applies the five source-owned strict-sum checks: shared
solar beam-beam transmittance plus the corresponding front or back
beam-diffuse transmittance and reflectance must be below 1; the three visible
inputs must sum below 1; and infrared transmittance plus each directional
emissivity must be below 1. Equality fails. Exact upstream diagnostic text is
not claimed, including the source's missing-space typo in three continuation
messages.

EnergyPlus stores the shared solar beam-beam value in both front and back TAR
slots, but stores the three nominally shared visible inputs only in the front
TAR slots; the back visible slots remain their initialized zero. The Rust
payload exposes this asymmetric source state instead of inventing back-side
symmetry. Roughness is fixed to `MediumRough`, the object is resistance-only,
front/back thermal absorptance project from the corresponding infrared
emissivity, and thermal transmittance projects from the infrared
transmittance. Base resistance, nominal resistance, thickness, conductivity,
density, specific heat, and scalar absorptance fields remain zero or absent;
no ordinary-shade solar/visible absorptance derivation is applied.

The object belongs to `MaterialFamily::EquivalentLayer`. Ordinary
`Construction` therefore rejects it, and the still-deferred
`Construction:WindowEquivalentLayer` consumer is not typed or inferred from
the source's weak layer validation. Arbitrary-run assessment counts every
typed definition, including unused definitions, and blocks execution. First
typed evidence is the named compiler/runtime test set.

`window_material_shade_equivalent_layer_001` adds a separate nonblocking
diagnostic EnergyPlus 26.1 EIO gate. Its generic `Material Details` comparison
is definition keyed and requires exactly one row for every equivalent-layer
shade, including the unused definition. Those rows lock `MediumRough` plus
zero resistance, thickness, conductivity, density, specific heat, and all
three scalar absorptances. They expose none of the eleven optical inputs,
directional thermal projections, use/reuse state, or construction occurrence.

The specialized `WindowMaterial:Shade:EquivalentLayer` comparison locks a
duplicate-aware, fixture-local ordered sequence of equivalent-layer construction
occurrences. The
defaulted used shade appears once, the high-precision shade reused by two
constructions appears twice, and the unused definition is absent. Material
name, the front/back duplicate of shared beam-beam solar transmittance, the
four directional beam-diffuse solar values, infrared transmittance, and both
infrared emissivities are locked with EnergyPlus `{:.4R}` serialization, as is
the exact one-row header. The three visible inputs do not appear in this EIO
table and remain outside external evidence.

The header expectation also reproduces the shared source gate: an
`Output:Constructions` object must select `Constructions`, and at least one
ordinary, complex-fenestration, or equivalent-layer window construction must
exist. Selecting only `Materials` does not request this specialized table.

Occurrence derives from `Construction:WindowEquivalentLayer` layers: a
surface-unused construction still emits its material occurrence, while
additional fenestration surfaces do not multiply it. A fixture-only
`EnergyManagementSystem:ConstructionIndexVariable` nominally references that
surface-unused construction to suppress EnergyPlus's unused-construction
warning; EMS behavior is not compared. The exact construction, host, and
window rows in the fixture remain oracle-integrity locks only. Equivalent-layer
construction typing and packing, arbitrary declaration-order parity, ASHWAT
roller-blind coefficients, openness-adjusted longwave behavior, visible
optical use, ratings, surfaces, EMS, runtime, broad diagnostics/order, and
conformance remain unclaimed.

### `WindowMaterial:Drape:EquivalentLayer`

The thirteenth source-order object preserves all thirteen EnergyPlus 26.1
numeric inputs in a distinct equivalent-layer drape payload. Shared
front/back solar beam-beam transmittance defaults to 0 in [0, 0.2].
Front/back solar beam-diffuse transmittance and reflectance are required and
individually bounded [0, 1). The three optional visible beam-beam
transmittance, beam-diffuse transmittance, and beam-diffuse reflectance inputs
have no schema default, but preserve the source-cleared numeric-buffer value 0
when blank or absent. Infrared transmittance defaults to 0.05 in [0, 1), and
front/back infrared emissivity each default to 0.87 in (0, 1). Pleated-fabric
width and length each default to 0 m and are bounded below by 0.

The compiler deliberately applies only the three checks owned by this source
block. Front solar beam-beam transmittance plus front beam-diffuse
transmittance and reflectance must remain strictly below 1. The three visible
inputs must also sum to less than 1. Front infrared transmittance plus front
emissivity fails only when greater than 1, so exact equality is accepted even
though the upstream diagnostic says `not < 1.0`. EnergyPlus 26.1 performs no
corresponding back-side solar or back-side infrared sum check here; Rust does
not invent either omitted validation. Exact diagnostic text and field-label
mistakes remain unclaimed.

EnergyPlus copies the shared solar beam-beam value to both directional TAR
records but assigns the three nominally shared visible values only to the
front record; the back visible record remains initialized to zero. Rust
exposes that asymmetry. Roughness is fixed to `MediumRough`, the material is
resistance-only, directional thermal absorptance equals the corresponding
infrared emissivity, and thermal transmittance equals infrared transmittance.
Base and nominal resistance, thickness, conductivity, density, specific heat,
and scalar absorptances remain zero or absent.

Pleat state is source-effective rather than a direct copy of the two bounded
inputs. Width and length are retained, and `is_pleated()` is true, only when
both inputs are present and both are nonzero. If either input is blank or
zero, both effective dimensions remain zero and the drape is non-pleated.
This preserves the all-or-nothing source branch without manufacturing a
one-sided pleat.

The object name participates in the same normalized material namespace and
source-order duplicate detection as every preceding material family. The
payload belongs to `MaterialFamily::EquivalentLayer`, ordinary
`Construction` rejects it, and `Construction:WindowEquivalentLayer` remains
deferred. Arbitrary-run assessment counts every typed drape definition,
including unused definitions, and explicitly blocks execution.

First typed evidence remains the named compiler/runtime test set.

`window_material_drape_equivalent_layer_001` adds a separate nonblocking
diagnostic EnergyPlus 26.1 EIO gate. Its generic `Material Details` comparison
is keyed by normalized name and requires exactly one row for every fixture
drape definition, including the definition absent from every construction.
Those rows lock `MediumRough` plus zero resistance, thickness, conductivity,
density, specific heat, and all three scalar absorptances. They expose none of
the thirteen optical, infrared, emissivity, or pleat inputs, directional
thermal projections, use/reuse state, or construction occurrence.

The specialized comparison preserves the malformed upstream shape exactly
rather than normalizing it. The literal header has 14 comma-separated tokens:
it advertises separate front/back beam-beam values and contains an empty
seventh token. Each data row has only 12 tokens and emits material identity,
the shared N1 beam-beam value once, N2-N5 directional solar beam-diffuse
values, N9 infrared transmittance, N10/N11 emissivities, and N12/N13 effective
pleat dimensions. N1, N2-N5, and N9-N11 use EnergyPlus `{:.4R}` serialization;
N12/N13 use `{:.5R}`. Visible N6-N8 are absent.

The exact fixture-local specialized sequence is A,Z,Z,P,Q: the defaulted
drape appears once, the high-precision drape appears twice through two
construction layers, and the two one-sided pleat inputs each report effective
zero/zero dimensions. The definition absent from every construction is
excluded. Surfaces do not multiply rows, and surface-unused constructions
still emit independently of surface use. Fixture-only
`EnergyManagementSystem:ConstructionIndexVariable` references merely suppress
oracle unused-construction warnings and are not compared.

The specialized header requires an `Output:Constructions` object selecting
`Constructions`, at least one drape definition, and any ordinary,
complex-fenestration, or equivalent-layer window construction. Selecting only
`Materials` requests the generic table but not the specialized table.
Equivalent-layer construction typing and packing, arbitrary IDF declaration
order, ASHWAT drape coefficients and numerical optics/thermal/pleat behavior,
visible optical use, ratings, daylighting, surfaces, EMS behavior, runtime,
exact input-validation diagnostics, broad ordering, and conformance remain
unsupported.

### `WindowMaterial:Screen`

The fourteenth source-order object accepts exactly `DoNotModel`,
`ModelAsDiffuse`, or `ModelAsDirectBeam` for reflected-beam transmittance
accounting; a missing or blank value defaults to `ModelAsDiffuse`. Diffuse
solar and visible reflectance, screen-material spacing, and screen-material
diameter are required. Both reflectances are bounded [0, 1), spacing and
diameter must be positive, and diameter must be strictly less than spacing.
Thermal hemispherical emissivity defaults to 0.9 in (0, 1), while conductivity
defaults to 221 W/m-K and must be positive. Screen-to-glass distance defaults
to 0.025 m in [0.001, 1], all four opening multipliers default to 0 in [0, 1],
and the output-map angle resolution defaults to 0 and accepts only the exact
numeric values 0, 1, 2, 3, or 5 degrees.

Let `r = diameter / spacing`, direct-normal open-area transmittance
`tau = (1 - r)^2`, and solid fraction `f = 1 - tau`. EnergyPlus scales the
input solar and visible reflectances and thermal emissivity by `f`. The typed
payload preserves those source-effective assembly reflectances and thermal
absorptance, derives solar and visible absorptance as
`max(0, 1 - tau - effective_reflectance)`, and projects visible and thermal
transmittance plus airflow permeability to `tau`. Roughness is fixed to
`MediumRough`, the material is resistance-only, thickness is the input wire
diameter, and nominal resistance is `f * diameter / conductivity`. The
compiler retains the source's three strict checks against the adjusted
assembly values: each check is `tau + raw * (1 - tau) < 1`, where `raw` is the
solar reflectance, visible reflectance, or thermal emissivity input. It does
not check `tau + raw reflectance`; rejecting that unadjusted sum would be a
regression. Valid schema inputs plus `diameter < spacing` make the three
adjusted sums algebraically less than 1 in real arithmetic; retaining the
explicit checks preserves source parity at finite-precision extremes.

The name joins the shared normalized material namespace immediately after
`WindowMaterial:Drape:EquivalentLayer`, and the payload belongs to
`MaterialFamily::Fenestration`. The safe ordinary-`Construction` subset is
exactly one exterior Screen followed directly by the existing one-through-four
pane `Glass ((Gas|GasMixture) Glass){0..3}` stack. Interior and between-glass
screens, multiple shading devices, solar-diffusing glazing with a screen,
screen-only stacks, and all other placements are rejected. In particular,
Rust rejects `Screen, Gap, Glass`: EnergyPlus 26.1's broad construction check
misses that forbidden gap, after which window initialization treats layer 2
as glazing and reaches an unsafe cast/assert path. This safety rejection is a
bounded typed divergence, not a claim of broad diagnostic parity.

Arbitrary-run assessment counts every typed screen definition, including
definitions unused by any construction, and explicitly blocks execution.

`window_material_screen_001` adds a separate nonblocking diagnostic
EnergyPlus 26.1 static EIO gate. The generic `Material Details` comparison
requires exactly one row for each fixture definition in the source Z,M,A
sequence, including M, which is unused by every construction. It locks
`MediumRough`, zero resistance, density, and specific heat, source
`{:.4R}` wire-diameter thickness, source `{:.3R}` conductivity, and source
`{:.4R}` solid-fraction-adjusted thermal, solar, and visible absorptance.
The reflected-beam enum, raw reflectance/emissivity, spacing, glass distance,
opening multipliers, map resolution, and nominal resistance are absent from
that generic table.

The specialized table preserves the exact 12-token source header, including
the missing space in `Screen To GlassDistance`. Its exact construction-layer
occurrence sequence is A,Z,Z: the defaulted material occurs in construction B,
the high-precision material occurs in C and D, and unused M is absent.
Thickness uses `{:.5R}`; conductivity, solid-fraction thermal absorptance,
normal-incidence beam solar transmittance, normal solar and visible
reflectance, diffuse solar and visible reflectance, diameter/spacing ratio,
and screen-to-glass distance use `{:.3R}`.
Each screened fixture construction becomes the exterior Screen plus the exact
layer tail of bare construction A. The comparator fails closed for a screened
occurrence without a matching bare fenestration construction because
EnergyPlus can skip its material row during nominal-window calculation.

The EnergyPlus source header condition is broader than occurrence use:
selecting `Constructions` with at least one Screen definition and any window
construction can validly emit the exact header with zero Screen data rows.
The bounded Rust comparator predicts header presence only inside its typed
ordinary-fenestration `Construction` scope; complex and equivalent-layer
window header activation remains unclaimed. Within that scope the parser
accepts the header-only shape, while the declared fixture separately requires
A,Z,Z.

The bounded comparator reproduces the source normal-incidence
`CalcScreenTransmittance` path and the reverse-order 18 by 18
`CalcWindowScreenProperties` quarter-hemisphere integration for the
fixture's two `ExteriorScreen`-activated definitions only. A constant-zero
control schedule still lets EnergyPlus initialize A and Z. Two surfaces share
construction C without multiplying its row; surface-unused construction D
still emits a second Z occurrence because C initialized the shared material.
The fixture-only `EnergyManagementSystem:ConstructionIndexVariable` reference
to D merely suppresses the oracle's unused-construction warning.

The primary lane selects both `Constructions` and `Materials`; a
Materials-only lane emits generic Z,M,A rows without a specialized header,
and a Constructions-only lane emits specialized A,Z,Z rows without the generic
table. All three oracle runs complete with zero warnings and zero severe
errors. This evidence does not promote either parent routine wholesale.
`DoNotModel` specialized optics, zero-reflectance source-NaN behavior,
duplicate/multiple-control active-selection order, arbitrary activation or
declaration order, general angle-dependent TAR evaluation, transmittance-map
generation, opening-multiplier behavior, window optics and heat transfer,
`WindowShadingControl` or surface behavior, ratings, daylighting, broad
diagnostics/order, and conformance remain deferred.

### `WindowMaterial:Screen:EquivalentLayer`

The fifteenth source-order object has ten numeric inputs. Screen beam-beam
solar transmittance defaults to the EnergyPlus `Autocalculate` sentinel and,
when numeric, is bounded [0, 1). Beam-diffuse solar transmittance and
reflectance plus beam-beam visible transmittance, beam-diffuse visible
transmittance, and diffuse visible reflectance are required in [0, 1).
Infrared transmittance defaults to 0.02 in [0, 1), and infrared emissivity
defaults to 0.93 in (0, 1). The schema advertises positive wire-spacing and
wire-diameter defaults of 0.025 m and 0.005 m, but `GetMaterialData` tests the
input blank flags before copying either numeric buffer value. A missing or
blank pair therefore leaves the source material's effective geometry at its
initialized 0 m / 0 m rather than applying those schema defaults. Rust
preserves that source quirk. Explicit geometry follows the source's
greater-than-0.00001 m threshold and diameter-below-spacing relationship.
EnergyPlus can emit a Severe message, substitute 0.025 m or 0.005 m, and
continue through some violations of those rules; Rust fail-closes those
geometry-repair branches and does not materialize the post-Severe values.

The solar beam-beam value remains `AutoOrNumber`; Rust does not replace the
`Autocalculate` state with the geometry-derived openness inside material
input. EnergyPlus copies N1-N3 to both front and back solar TAR records. Its
visible assignments are intentionally asymmetric: N4 and N5 populate only
the front beam record, while N6 populates the front diffuse-diffuse
reflectance slot, not the beam-diffuse reflectance slot. Every back-visible
slot remains initialized to zero. Infrared transmittance and emissivity are
shared front/back, directional thermal absorptance equals emissivity, and
thermal transmittance equals infrared transmittance. Roughness is fixed to
`MediumRough`; the object is resistance-only and has no base or nominal
resistance, thickness, conductivity, density, specific heat, or scalar
solar/visible absorptance projection.

Only the two effective optical-sum gates present in the source block are
enforced: numeric solar beam-beam transmittance plus solar reflectance, and
front visible beam-beam transmittance plus front diffuse-diffuse visible
reflectance, must each remain below 1. Beam-diffuse transmittances are absent
from those sums. The nominal infrared diagnostic reads the untouched scalar
`AbsorpThermal` member, which remains zero, instead of the directional
emissivity-derived members; Rust therefore does not invent an infrared
transmittance-plus-emissivity sum restriction. When a numeric N1 exceeds the
geometry-derived openness by more than one percent, EnergyPlus emits a Severe
message and rewrites the diameter without setting the local input-error flag.
The bounded Rust compiler rejects that one-sided recovery branch rather than
materializing a value after a Severe diagnostic. Smaller N1 values and the
source's non-triggering side of the asymmetric comparison are not converted
into a symmetric closeness rule.

The normalized name joins the shared material namespace immediately after
ordinary `WindowMaterial:Screen`, and the payload belongs to
`MaterialFamily::EquivalentLayer`. Ordinary `Construction` rejects it;
`Construction:WindowEquivalentLayer` remains deferred. Arbitrary-run support
assessment counts every typed definition, including definitions unused by any
construction, and blocks execution. Equivalent-layer construction packing,
`CheckAndFixCFSLayer`, `IS_OPENNESS`, ASHWAT optics and thermal behavior,
ratings, surfaces, exact diagnostic recovery and text, runtime execution, and
conformance remain unsupported.

`window_material_screen_equivalent_layer_001` adds a separate nonblocking
diagnostic EnergyPlus 26.1 static EIO gate and bounded Rust parser/CLI
comparator without promoting any of those deferred consumers. Its generic
`Material Details` comparison requires exactly one `MediumRough`/all-zero row
for every fixture definition in source Z,M,A order, including
`M UNUSED EQL SCREEN`, which appears in no construction. The specialized
comparison preserves the malformed source shape exactly: its header has nine
comma-separated tokens, while each row has twelve. The exact
construction-occurrence sequence is A,Z,Z; A contains the default
`Autocalculate`/blank-geometry definition, and B and C reuse the high-precision
Z definition. M is excluded.

The A row locks EnergyPlus's raw `Autocalculate` value as `-99999.0000` and
blank wire spacing/diameter as `0.00000`/`0.00000`. Each specialized row emits
the shared beam-beam solar value once, duplicates N2 and N3 into front/back
solar slots, emits shared infrared transmittance once, duplicates N8 into
front/back emissivity slots, and omits visible N4-N6. Solar and infrared values
use source `{:.4R}` serialization; wire spacing and diameter use `{:.5R}`.
Construction B is shared by two surfaces without multiplying its Z row, while
surface-unused A and C are referenced only by fixture
`EnergyManagementSystem:ConstructionIndexVariable` objects and still emit
their A and Z occurrences.

The primary lane selects both `Constructions` and `Materials`; Materials-only
emits generic Z,M,A rows with no specialized header, and the CLI maps the
expected specialized-parser missing-header result to an empty occurrence set.
Constructions-only emits specialized A,Z,Z rows with no generic table. All
three EnergyPlus runs must complete with zero warnings and zero severe errors.
Exact equivalent-layer construction and surface topology are fixture-integrity
locks only. This case does not serialize EIO, type or execute
`Construction:WindowEquivalentLayer`, reproduce
`CheckAndFixCFSLayer`/`IS_OPENNESS`/ASHWAT behavior, or claim ratings, EMS,
surface, runtime, diagnostic-text, broad ordering, or conformance parity.

### `WindowMaterial:Blind`

The sixteenth source-order object has one orientation and 27 numeric inputs.
`Horizontal` and `Vertical` are the only slat-orientation values, with
`Horizontal` selected for a missing or blank field. Slat width and separation
are required in (0, 1] m. Slat thickness defaults to 0.00025 m in (0, 0.1] m,
slat angle defaults to 45 degrees in [0, 180], and conductivity defaults to
221 W/m-K and must be positive. The required optical inputs are front/back
beam-solar reflectance, front/back diffuse-solar reflectance, and beam-visible
transmittance; together with every optional optical input they are bounded
[0, 1). Beam- and diffuse-solar transmittance default to zero. Beam-visible
front/back reflectance and diffuse-visible front/back reflectance have no
schema default, but missing numeric fields retain the source input buffer's
effective zero; diffuse-visible transmittance also defaults to zero. Infrared
transmittance defaults to zero, and front/back infrared emissivity each
default to 0.9, all in [0, 1).

Blind-to-glass distance defaults to 0.05 m in [0.01, 1] m. Top, bottom, left,
and right opening multipliers default to 0.5, 0.0, 0.5, and 0.5 respectively
and are each bounded [0, 1]. Minimum and maximum slat angles default to 0 and
180 degrees and are each independently bounded [0, 180]. The active
`GetMaterialData` branch treats every blind as fixed-angle input and leaves
the source's variable-slat validation block commented out. Rust therefore
does not invent a minimum-less-than-maximum relationship for N26/N27 or
require the fixed slat angle to lie between those two user fields.

Ten source optical sums must each remain strictly below one: N6+N7, N6+N8,
N9+N10, N9+N11, N12+N13, N12+N14, N15+N16, N15+N17, N18+N19, and N18+N20.
The source additionally requires six beam/diffuse pairs to agree within an
absolute tolerance of 1e-5: N6=N9, N7=N10, N8=N11, N12=N15, N13=N16, and
N14=N17. Rust accepts exact 1e-5 differences and rejects only larger ones.
EnergyPlus repeats some visible-sum diagnostics through a combined check and
later individual checks; this checkpoint preserves the semantic constraints,
not duplicate diagnostic multiplicity or exact message text.

A slat width below its separation is warning-only and still materializes;
the compiler retains that source warning instead of converting it into an
error. Blind-to-glass distance must nevertheless be at least half the slat
width. For geometry, the source derives a minimum angle as
`asin(thickness / (thickness + separation))` in degrees only when width is
greater than separation, otherwise zero, and derives the maximum as 180
degrees minus that minimum. It checks the input slat angle against that
derived interval only when `separation + thickness < width`. Rust reproduces
that gated relationship without applying the interval outside the source
gate.

`WindowBlindMaterial` projects N6-N11 into the source front/back solar
beam-diffuse and diffuse-diffuse slat-property slots, N12-N17 into the
corresponding visible slots, and N18-N20 into shared front/back infrared
transmittance plus directional emissivity slots. Unassigned slat-property
slots remain initialized to zero. Roughness is fixed to `Rough`, resistance-only
behavior is true, and base/nominal resistance, thickness, conductivity,
density, specific heat, scalar absorptances, and base directional thermal
projections remain zero or absent; the slat conductivity is retained only in
the blind payload.

The normalized name joins the shared material namespace immediately after
`WindowMaterial:Screen:EquivalentLayer`, and the payload belongs to
`MaterialFamily::Fenestration`. The safe ordinary-`Construction` subset
accepts exactly one exterior or interior Blind directly against the existing
one-through-four-pane `Glass ((Gas|GasMixture) Glass){0..3}` stack, or one
between-glass Blind in the source-defined double- or triple-pane position.
At most one `WindowMaterial:Shade`, `WindowMaterial:Screen`, or
`WindowMaterial:Blind` may appear, and solar-diffusing glazing cannot be
combined with any such device. Between-glass Blind gaps must have the same
source-effective five-slot gas type/fraction signature, may differ in width
by no more than 0.0005 m, and must have a combined width at least as large as
the blind slat width. The unsafe exterior/interior Blind-Gap-Glass end holes
fail closed, as do every other placement outside the bounded patterns.

Arbitrary-run assessment counts and blocks every blind definition, including
definitions unused by any construction.

`window_material_blind_001` adds a nonblocking diagnostic EnergyPlus 26.1
static EIO parser/comparator gate. The generic `Material Details` lane
requires exactly one normalized-name row for each typed definition, including
construction-unused M, and the smoke fixture locks source declaration order
Z,M,A. Every generic row is `Rough` and has zero resistance, thickness,
conductivity, density, specific heat, and thermal, solar, and visible
absorptance. `Output:Constructions, Materials` activates this table, while a
Constructions-only run emits no generic rows.

The specialized header is exact:
`! <WindowMaterial:Blind>,Material Name,Slat Width {m},Slat Separation {m},Slat Thickness {m},Slat Angle {deg},Slat Beam Solar Transmittance,Slat Beam Solar Front Reflectance,Blind To Glass Distance {m}`.
It and every specialized row each have exactly nine comma-separated tokens.
Rows preserve construction-layer
occurrences rather than definition or surface multiplicity: the fixture's
exact sequence is A,Z,Z and excludes M. The seven numeric fields are raw
input N1 slat width, N2 separation, and N3 thickness serialized with source
`{:.4R}`, then N4 angle, N6 beam solar transmittance, N7 front beam solar
reflectance, and N21 glass distance with source `{:.3R}`. `slatTAR` remains
the raw input record used by `ReportGlass`; `CalcBlindProperties` writes the
separate angle-indexed `TARs` tables. The Rust comparator therefore does not
reproduce or claim computed blind optics.

Within the bounded Rust scope, `Output:Constructions, Constructions`, at least
one Blind definition, and at least one typed ordinary fenestration
Construction require the specialized header; a header with zero data rows is
valid. The primary lane emits generic and specialized evidence, Materials-only
emits generic Z,M,A with no specialized header, and Constructions-only emits
specialized A,Z,Z without generic rows. All three EnergyPlus 26.1 oracle runs
complete with zero warnings and zero severe errors. Construction C is shared
by two surfaces without multiplying its Z row. Surface-unused construction D
still emits the second Z; its fixture-only
`EnergyManagementSystem:ConstructionIndexVariable` reference merely suppresses
an oracle unused-construction warning.

The comparator predicts exterior/interior Blind rows only when an exact bare
companion glazing stack exists. It fails closed for between-glass Blind and
missing-bare report behavior because `CalcNominalWindowCond` returns an error
flag and `ReportGlass` skips those construction rows. Exact bare/blinded
`WindowConstruction`, detailed-window, `WindowShadingControl`, and EMS rows
are fixture-integrity locks only. Rust EIO serialization,
`CalcBlindProperties`, beam-beam and all other blind optics, ratings,
daylighting, control/surface/EMS behavior, variable-angle and window thermal
runtime, between-glass or missing-bare reporting, broad declaration or
diagnostic ordering, `Construction:WindowEquivalentLayer`, and conformance
remain unclaimed.

### `WindowMaterial:Blind:EquivalentLayer`

The seventeenth source-order object has one slat orientation, one slat-angle
control, and 21 numeric inputs. `Horizontal` and `Vertical` are accepted, with
`Horizontal` as the missing/blank default. Its six required numeric fields are
slat width and separation, N7/N8 front/back beam-diffuse solar reflectance,
and N14/N15 front/back diffuse-diffuse solar reflectance. Width and separation
are bounded (0, 0.025] m. Slat crown defaults to 0.0015 m in [0, 0.00156], and
slat angle defaults to 45 degrees in [-90, 90]. `FixedSlatAngle`,
`MaximizeSolar`, and `BlockBeamSolar` are the exact control tokens, with
`FixedSlatAngle` selected by default.

All 17 optical inputs N5-N21 are bounded [0, 1). N5 and N6 front/back
beam-diffuse solar transmittance default to zero; N7 and N8 front/back
beam-diffuse solar reflectance are required. N9 and N10 visible
transmittances default to zero, while N11 and N12 visible reflectances have no
schema default. EnergyPlus tests all four blank flags before assigning any
visible beam state, so an incomplete N9-N12 group leaves both sides at their
initialized zeros even when some values were supplied.

N13 diffuse-diffuse solar transmittance has a numeric default of zero, while
N14 and N15 directional solar reflectances are required. The source assigns
the solar diffuse-diffuse record only when all three fields were explicitly
present. Consequently, omitting N13 leaves N13-N15 source-ineffective even
though its numeric buffer value is zero and N14/N15 are present. N16-N18 are
optional visible diffuse-diffuse inputs. When all three are present,
EnergyPlus 26.1 tests their blank flags but mistakenly copies N13-N15 into the
visible record; the supplied N16-N18 numeric values are validated but not
retained. An incomplete N16-N18 group leaves the visible diffuse-diffuse
record zero.

N19 infrared transmittance and N20/N21 directional emissivities are guarded
individually. A missing field leaves its class-initialized zero, so the schema
emissivity defaults of 0.9 do not become source-effective unless the values
are explicitly present. Effective N19 is shared front/back, directional
thermal absorptance equals effective emissivity, and thermal transmittance
equals effective infrared transmittance. All beam-beam TAR slots remain zero.

Only four source optical sums are enforced: raw N5+N7, N6+N8, N9+N11, and
N10+N12 must each be strictly below one. The source adds no diffuse-diffuse
solar/visible or infrared sum, and Rust does not invent one.

Geometry recovery is warning-only and order-sensitive. The source first
warns when the original width is below the original separation without
changing either value. It then replaces separation below 0.001 m with
0.025 m; replaces width below 0.001 m or greater than or equal to twice the
corrected separation with that separation; and replaces crown greater than
or equal to half the corrected width with zero. Rust retains these
source-effective recovered values and warnings. Inputs outside the schema
angle bounds fail compilation before the source's out-of-range angle recovery
would be materialized.

`WindowBlindEquivalentLayerMaterial` fixes roughness to `Rough`, is
resistance-only, has no nominal resistance, and retains the effective
geometry, directional solar/visible TAR state, infrared/thermal projections,
and slat-angle control. Its normalized name joins the shared material
registry immediately after ordinary Blind, and its family is
`MaterialFamily::EquivalentLayer`. Ordinary `Construction` rejects it with
the common equivalent-layer boundary; only the still-deferred
`Construction:WindowEquivalentLayer` is its intended construction consumer.
Arbitrary-run assessment counts and blocks every definition, including
definitions unused by a construction.

`window_material_blind_equivalent_layer_001` adds a bounded nonblocking EIO
parser/comparator fixture and two diagnostic proof variables. Its generic
Material Details rows are one-per-definition in source order Z,M,A, including
construction-unused M, with exact `Rough` and eight zero base fields. The
specialized header has 18 tokens, including four source `Slate` typos and a
final `Slat Angle Control` label, but every payload has only 17 tokens because
the angle-control value is omitted. Identity and orientation precede 14
numeric fields, all serialized with source `{:.5R}`. Visible inputs N9-N12 and
N16-N18 are not reported. A proves default Horizontal orientation, 0.0015 m
crown and 45 degree angle plus zero defaulted beam transmittances, solar
diffuse state, and infrared state while retaining required beam-diffuse
reflectances. Z proves a distinctive negative angle, high-precision geometry
and optics, and byte-identical duplicate reuse.

EnergyPlus 26.1's Blind:EquivalentLayer format has no trailing newline. The A
payload is directly followed on the same physical line by Construction B, the
first Z by Construction C, and the final Z by the next EIO header. The bounded
parser recovers those source-actual logical boundaries and compares exact
A,Z,Z occurrence order while excluding M. Two detailed windows share B without
multiplying its Z row; surface-unused A and C are retained only by fixture EMS
construction-index references. Primary, Materials-only, and
Constructions-only independently lock report activation and complete with zero
warnings and zero severe errors.

This evidence does not type or pack `Construction:WindowEquivalentLayer` and
does not promote `CheckAndFixCFSLayer`, ASHWAT coefficient
generation/evaluation, slat-angle control execution, optics and thermal
calculations, surfaces, ratings, daylighting, runtime, EIO serialization,
broad diagnostic parity, or conformance.

### `Material:RoofVegetation`

The eighteenth source-order object owns vegetation inputs plus the dry-soil
material state consumed by EnergyPlus's EcoRoof path. The name is the only
required field. The compiler applies the EnergyPlus 26.1 defaults and exact
schema bounds to all 15 numeric fields: plant height 0.2 m in (0.005, 1], leaf
area index 1 in (0.001, 5], leaf reflectivity 0.22 in [0.05, 0.5], leaf
emissivity 0.95 in [0.8, 1], minimum stomatal resistance 180 s/m in
[50, 300], soil thickness 0.1 m in (0.05, 0.7], dry-soil conductivity
0.35 W/m-K in [0.2, 1.5], dry-soil density 1100 kg/m3 in [300, 2000],
dry-soil specific heat 1200 J/kg-K in (500, 2000], thermal absorptance 0.9 in
(0.8, 1], solar absorptance 0.7 in [0.4, 0.9], visible absorptance 0.75 in
(0.5, 1], saturation moisture 0.3 in (0.1, 0.5], residual moisture 0.01 in
[0.01, 0.1], and initial moisture 0.1 in (0.05, 0.5].

Roughness accepts the six common opaque-material values and defaults to
`MediumRough`. `Simple` and `Advanced` are the exact moisture-diffusion
tokens, with blank or missing input selecting `Advanced`, which maps to the
source's `SchaapGenuchten` branch. `Soil Layer Name` defaults to
`Green Roof Soil` and is type-checked, but `GetMaterialData` explicitly
ignores A2; the source-effective Rust payload therefore does not retain it.

EnergyPlus stores the vegetation values, dry-soil properties, absorptances,
porosity, residual and initial moisture, and calculation method in
`MaterialEcoRoof`. Nominal thermal resistance is soil thickness divided by
dry-soil conductivity. Rust exposes the same dry-soil roughness, thickness,
conductivity, density, specific heat, surface absorptances, resistance, and
area heat capacity through the opaque-material boundary while retaining the
plant and moisture state in `RoofVegetationMaterial`.

The one source-owned cross-field recovery is warning-only: when initial
volumetric moisture exceeds saturation, EnergyPlus reports the combination
and resets initial moisture to saturation. The compiler materializes that
same clamp. It does not invent relationships among residual, initial, and
saturation moisture beyond this one check.

#### Bounded generic `Material Details` diagnostic

`material_roof_vegetation_001` adds a nonblocking diagnostic for the generic
EnergyPlus 26.1 `Material Details` report. Each payload has exactly 11
comma-separated tokens, in this order: the `Material Details` row label,
normalized material name, thermal resistance, roughness, thickness,
conductivity, density, specific heat, thermal absorptance, solar absorptance,
and visible absorptance. `Material:RoofVegetation` takes the generic material
branch; EnergyPlus emits no dedicated RoofVegetation header or row.

The fixture's exact source definition order is `Z USED EXPLICIT ROOF
VEGETATION`, `M DEFAULTED UNUSED ROOF VEGETATION`, then `A UNUSED EXPLICIT
ROOF VEGETATION`. All three definitions emit exactly one generic row in that
Z,M,A order. Z is the only definition used by a construction, while M and A
are unused by every construction; definition reporting therefore includes
both used and unused stored materials and does not multiply Z for use or
reuse. This locks only the fixture-local source order, not broad material
declaration or report order.

Thermal resistance, thickness, and all three absorptances use EnergyPlus
`{:.4R}` serialization. Conductivity, density, and specific heat use
`{:.3R}`. The values are the dry input snapshot written before EcoRoof runtime
updates; resistance is soil thickness divided by dry-soil conductivity. The
generic row does not expose plant inputs, the ignored soil-layer label,
moisture values or recovery, or the moisture-diffusion method.

| Fixture lane | `Output:Constructions` selection | Generic `Material Details` | Shared `Material CTF Summary` |
|---|---|---|---|
| primary | `Constructions, Materials` | exact header plus Z,M,A | exact shared header plus one used-Z row |
| Materials-only | `Materials` | exact header plus Z,M,A | absent |
| Constructions-only | `Constructions` | absent | exact shared header plus one used-Z row |

The CTF header, the single used-Z CTF row, and the fixture construction row are
oracle-only fixture-integrity locks; they are not case outputs or proof
variables and do not establish CTF or construction parity. This checkpoint
also excludes EcoRoof runtime and water balance, plant/moisture/method/soil
label parity, the one-used-material-across-surfaces rule, broad ordering or
diagnostic parity, and conformance.

An EcoRoof material is intended to be the outside layer. Upstream sets
`TypeIsEcoRoof` and searches for illegal interior EcoRoof layers only when
the first layer is already EcoRoof, so an interior-only occurrence behind a
regular outside layer passes the 26.1 construction check and silently loses
EcoRoof behavior. The bounded compiler fails closed for every non-outside
`Material:RoofVegetation` reference instead of reproducing that validation
hole. This deliberate diagnostic divergence matches the existing
infrared-transparent fail-closed policy.

Every typed RoofVegetation definition, including an unused one, is explicitly
run-blocked before arbitrary Rust execution. EnergyPlus's dynamic EcoRoof
state is shared across surfaces, requires one effective EcoRoof material for
all used vegetated constructions, and supports only the CTF heat-transfer
algorithm. Those singleton/state updates, moisture redistribution,
evapotranspiration, irrigation, surface coupling, runtime execution, and broad
diagnostic parity remain deferred. The bounded generic diagnostic above does
not promote any of those behaviors or conformance.

### `WindowMaterial:GlazingGroup:Thermochromic`

The nineteenth source-order object is a material-namespace parent whose
extensible `temperature_data` list contains ordered pairs of optical-data
temperature in C and a referenced glazing material. Temperatures have no
default, range, ordering, uniqueness, or maximum-count rule. The compiler
therefore preserves the input order, duplicate temperatures, and duplicate
glazing references without sorting, interpolation, or deduplication.

Each child name resolves case-insensitively after the ordinary glazing input
families have been read. The bounded implementation accepts only the already
typed `WindowMaterial:Glazing` `SpectralAverage` branch and
`WindowMaterial:Glazing:RefractionExtinctionMethod`, matching their upstream
`Group::Glass` classification. Missing references, equivalent-layer glazing,
another thermochromic parent, and every other material type fail compilation.
`WindowMaterial:SimpleGlazingSystem` is not a valid bounded child: despite its
schema object-list membership, EnergyPlus reads it after the thermochromic
group, so the source lookup cannot resolve it at this point.

The legacy IDD declares a minimum of one temperature/glazing pair. EnergyPlus
26.1 nevertheless accepts a missing or empty epJSON array while the parent is
unused, then dereferences `matRefs(1)` and crashes if that empty parent is used
by a `Construction`. Rust deliberately restores the minimum-one invariant at
compile time. This fail-closed safety check is a documented divergence from
the unused-empty source hole, not a runtime-parity claim.

The typed parent keeps a copyable start/count descriptor into a flat model
arena of `(temperature, MaterialId)` states. A dedicated
`ThermochromicGroup` material family prevents ordinary opaque, fenestration,
and equivalent-layer consumers from mistaking the parent for a completed
glazing layer. Every ordinary `Construction` reference is rejected until the
source behavior that replaces the parent with its first glazing and creates a
master plus one child construction per state is ported. Arbitrary-run support
assessment also counts and blocks every group definition, including unused
parents.

No thermochromic-specific EIO row exists. The generic `Material Details` path
indexes the parent's unset `Invalid` roughness as `-1`, so the observed 26.1
`VerySmooth` zero row depends on undefined behavior and is not promoted to a
Rust comparison. State order is visible only indirectly through generated
`_TC_<rounded-temperature>` construction names. Child-construction creation,
duplicate rounded names, timestep state selection, window optics and thermal
behavior, daylighting adjustment, surface output variables, EIO
serialization, runtime, and conformance all remain deferred.

### `WindowMaterial:SimpleGlazingSystem`

The twentieth source-order object is a compact performance-index input that
EnergyPlus expands into one source-effective equivalent glass layer. U-factor
including film coefficients is required and greater than zero. Solar heat-gain
coefficient is required in the strict interval `(0,1)`. Visible transmittance
is optional; when supplied it uses the same strict interval, and when absent
the typed payload retains `None` rather than erasing the distinction between
source input and derived state.

`WindowSimpleGlazingMaterial::from_performance_indices` reproduces the complete
material-owned `MaterialGlass::SetupSimpleWindowGlazingSystem` block model. It
removes the source-correlated winter interior and exterior film resistances,
derives layer resistance, chooses thickness from the inverse-resistance
threshold, and calculates effective conductivity. It then follows the exact
U-factor branches below 3.4, from 3.4 through 4.5, and above 4.5 W/m2-K and the
SHGC thresholds at 0.15 and 0.7206 to derive normal-incidence solar
transmittance. The intermediate-U summer-film calculation intentionally
preserves the EnergyPlus 26.1 expression
`(low - high) * interpolation_fraction + low` for both film resistances. This
is a reversed interpolation direction, not a corrected high-minus-low blend.

The derived summer film resistances and inward-flow fraction produce equal
front/back solar reflectance. Missing visible transmittance copies the solar
transmittance and reflectance, while explicit input uses the separate source
front/back cubic visible-reflectance correlations and their
`0.999 - visible_transmittance` clamps. The source-fixed state is
`VerySmooth`, zero infrared transmittance, 0.84 front/back infrared emissivity
and thermal absorptance, unit dirt correction, and a false solar-diffusing
flag. Generic solar and visible absorptance projections remain zero.

If film removal produces a non-positive layer resistance, EnergyPlus warns,
sets it to 0.001 m2-K/W, and continues. Rust preserves that materializing
recovery with `film_resistance_clamped` and the dedicated
`SimpleGlazingFilmResistanceClamped` warning; the resulting high-U layer has
0.002 m thickness and 2 W/m-K conductivity. Exact EnergyPlus warning wording,
order, and multiplicity are not claimed. Separately, a finite schema-valid tiny
positive U-factor can overflow reciprocal resistance and derive zero
conductivity. EnergyPlus 26.1 then emits Severe followed by Fatal without a
material report row; Rust preserves the fail-closed outcome with
`InvalidSimpleGlazingDerivedConductivity` before reserving or materializing the
definition, without claiming the source diagnostic flow.

The object joins the shared case-insensitive material namespace after the
thermochromic group. An earlier family therefore owns a cross-family collision,
and a thermochromic child lookup cannot resolve a SimpleGlazing definition
even when the input text declares it first. Later material consumers can
resolve the typed identity normally.

The payload uses the dedicated `MaterialFamily::SimpleGlazing`. This checkpoint
does not treat the whole-system input as ordinary detailed glass: every
`Construction` reference explicitly fails closed with
`UnsupportedSimpleGlazingSystemConstruction`, regardless of layer count or
position. Arbitrary-run assessment independently counts and blocks every typed
definition, including unused definitions, as
`UnsupportedSurfaceBoundary`/`RunBlocked` with no runtime class.

#### `SetupSimpleWindowGlazingSystem` state contract

<!-- routine-state-contract:v1 begin setup_simple_window_glazing_system -->
SetupSimpleWindowGlazingSystem

read_state:
- one validated WindowMaterial:SimpleGlazingSystem definition: name, positive U-factor with film coefficients, solar heat-gain coefficient in (0,1), the explicit-visible-input flag, and the optional visible transmittance in (0,1); the calculation reads no other material, construction, surface, weather, timestep, or history state

write_state:
- the source-effective single-layer VerySmooth material state: winter-film-removed resistance, thickness, conductivity, NominalR-equivalent resistance, normal-incidence solar and visible transmittance/reflectances, zero infrared transmittance, 0.84 front/back emissivity and thermal absorptance, unit dirt factor, and false solar-diffusing flag
- a materializing high-U warning when derived layer resistance is non-positive, after replacing it with 0.001 m2-K/W; Rust records this as film_resistance_clamped and SimpleGlazingFilmResistanceClamped without claiming source warning text

history_state_ownership:
- no cross-call history or cache; each typed material owns its immutable raw performance indices, optional visible-input state, derived block-model fields, and clamp flag

unsupported_state:
- Construction ownership and layer packing, specialized WindowMaterial:Glazing and WindowConstruction report state, angular or hemispherical optical tables, surface/window thermal state, daylighting, ratings, output variables, and runtime state

inactive_branches:
- winter interior-film resistance branches at U-factor 5.85; thickness branches at inverse layer resistance 7; solar correlations below 3.4, from 3.4 through 4.5, and above 4.5 W/m2-K with SHGC thresholds 0.15 and 0.7206; the intermediate-U summer-film path deliberately preserves the EnergyPlus 26.1 `(low - high) * fraction + low` reversed interpolation direction
- missing visible transmittance copies the derived solar transmittance and reflectance, while explicit visible input uses separate front/back cubic reflectance correlations and the source 0.999-minus-transmittance clamps

unsupported_active_branches:
- exact EnergyPlus diagnostic wording, order, and multiplicity for the materializing high-U warning; non-finite or schema-invalid inputs are rejected before this mapped routine boundary
- a finite schema-valid tiny positive U-factor can overflow reciprocal resistance and derive zero conductivity (EnergyPlus 26.1 emits Severe then Fatal, with no material row); Rust preserves the fail-closed outcome with InvalidSimpleGlazingDerivedConductivity before material identity reservation, without claiming the source diagnostic wording, order, multiplicity, or post-materialization flow

not_claimed_branches:
- TransAndReflAtPhi incident-angle dependence, hemispherical averaging, normalized specialized glazing serialization, construction conductance/U-factor/SHGC reporting, window ratings, surfaces, daylighting, window heat transfer, runtime execution, Rust EIO serialization, broad diagnostics, and conformance
<!-- routine-state-contract:v1 end setup_simple_window_glazing_system -->

#### Bounded generic `Material Details` diagnostic

`window_material_simple_glazing_system_001` adds a nonblocking diagnostic for
the generic EnergyPlus 26.1 `Material Details` report through
`run_compare_window_material_simple_glazing_system`. The warning-free, no-zone
fixture keeps all three SimpleGlazing definitions unused and locks the
fixture-local source IDF sequence Z,M,A. Z uses U-factor 2.7 with no visible
transmittance. M uses the same U-factor while changing SHGC and supplying an
explicit visible transmittance; because the generic row exposes only the base
thermal material projection, Z and M have identical numeric payloads. A uses
U-factor 5 and produces a distinct resistance, thickness, and conductivity.

Every payload has exactly 11 comma-separated tokens: `Material Details`,
normalized material name, thermal resistance, roughness, thickness,
conductivity, density, specific heat, thermal absorptance, solar absorptance,
and visible absorptance. The rows are `VerySmooth`, density and specific heat
are zero, thermal absorptance is 0.84, and solar and visible absorptance remain
zero. Resistance, thickness, and all three absorptances use source `{:.4R}`;
conductivity, density, and specific heat use source `{:.3R}`.

The lane selecting both `Constructions` and `Materials` and the Materials-only
lane each emit exactly one generic header followed by Z,M,A. The
Constructions-only and blank/default lanes emit neither the generic header nor
any generic rows. No lane contains a specialized `WindowMaterial:Glazing` or
`WindowConstruction` header or data row because the fixture defines no window
construction. This bounded selector and row-shape evidence does not promote
`GetMaterialData` or `ReportGlass` wholesale.

SHGC, explicit or defaulted visible transmittance, derived normal-incidence
solar/visible transmittance and reflectance, construction use/reuse occurrence,
specialized glazing and construction reporting, angular and hemispherical
optics, construction ratings, window thermal behavior, surfaces, daylighting,
output variables, Rust EIO serialization, runtime, broad diagnostic and
declaration-order parity, and conformance remain unclaimed.

### `WindowMaterial:Gap`

The twenty-first source-order object defines a gap exclusively for a complex
fenestration system. Thickness is required and strictly positive. The schema's
exact `gas_or_gas_mixture_` key, including its trailing underscore, is also
required and resolves case-insensitively against an already compiled
`WindowMaterial:Gas` or `WindowMaterial:GasMixture`. Pressure defaults to
101325 Pa; `GetMaterialData` requires it to be strictly positive even though
the epJSON schema declares no minimum.

EnergyPlus copies the referenced gas state into the complex gap rather than
turning the referenced ordinary gap into its consumer. The typed
`WindowComplexGapGasComposition` therefore stores either one copied gas type
and property record or the copied ordered mixture, together with the source
`MaterialId` only as graph provenance. It does not copy the referenced
material name, thickness, or nominal resistance. `WindowComplexGapMaterial`
owns its own thickness and pressure, and deliberately invents no nominal
resistance. The source-fixed base state is `Rough`, R-only, with zero
resistance, conductivity, density, specific heat, and absorptances. The
bounded diagnostic below promotes only those generic report projections and
the gap's own thickness; it does not expose or promote the copied gas/helper
state.

`WindowGap:DeflectionState` and `WindowGap:SupportPillar` remain raw-only
helper families outside the 34-object material inventory. The compiler
resolves only helpers referenced by a gap, case-insensitively and lazily. A
referenced deflection helper validates its nonnegative deflected thickness,
initial temperature, and initial pressure, then retains only the deflected
thickness because that is the only field copied by `GetMaterialData`; the
initial temperature and pressure are discarded. A missing deflection
reference leaves the copied thickness at zero. A referenced support-pillar
helper requires positive spacing and radius and copies only those two values;
an omitted reference remains `None`. No relationship between nominal and
deflected thickness, or between pillar radius and spacing, is added because
the source block enforces none.

The upstream reference path performs an unchecked downcast after an object-list
lookup and allocates the gap before several later failures. Rust deliberately
fails closed on missing, ambiguous, malformed, or wrong-family references and
reserves the shared material identity only after all fallible validation and
resolution completes. The dedicated `MaterialFamily::ComplexFenestration`
keeps the payload out of opaque, ordinary fenestration, equivalent-layer,
thermochromic, and simple-glazing consumers. Every ordinary `Construction`
position rejects it. Its sole intended
`Construction:ComplexFenestrationState` consumer, complex-window packing,
deflection execution, pillar conduction, optics, thermal behavior, surfaces,
ratings, daylighting, and reporting remain deferred. Arbitrary-run support
assessment counts and blocks every typed definition, including unused gaps,
as `UnsupportedSurfaceBoundary`/`RunBlocked` with no runtime class. Rust EIO
serialization, runtime, and conformance remain outside this typed checkpoint.

#### Bounded generic `Material Details` diagnostic

`window_material_gap_001` adds a nonblocking diagnostic for the generic
EnergyPlus 26.1 `Material Details` report through
`run_compare_window_material_gap`. Its warning-free, no-zone source-IDF
fixture keeps three Gap definitions unused and locks only the fixture-local
source sequence Z DEFAULT PRESSURE, M SAME THICKNESS DIFFERENT STATE, A
DIFFERENT THICKNESS. Z and M have the same own thickness of 0.0127 m despite
different referenced gas, pressure, deflection, and pillar state; A has a
0.006 m own thickness.

The lane selecting both `Constructions` and `Materials` and the Materials-only
lane each emit exactly one exact 11-token generic header and one target row
per Gap in Z,M,A order. Each target row locks normalized identity, `Rough`,
source `{:.4R}` own thickness, and zero resistance, conductivity, density,
specific heat, and thermal, solar, and visible absorptances. The two required
`WindowMaterial:Gas`/`WindowMaterial:GasMixture` source definitions emit their
own unrelated generic rows, and Materials-enabled lanes also emit an empty
`Material:Air` header; target filtering excludes both.

Constructions-only and blank/default lanes emit no generic `Material Details`
header or data. Constructions-enabled lanes may nevertheless emit empty
generic `Construction CTF`, `Material CTF Summary`, `Material:Air CTF
Summary`, and `CTF` headers; these are outside the window-specific absence
claim. No lane emits a dedicated `WindowMaterial:Gap`,
`WindowMaterial:Glazing`, or `WindowConstruction` header or data row.

`ConvertInputFormat` reorders the Gap definitions A,M,Z, and the converted
epJSON EIO follows A,M,Z. The evidence therefore claims only source-IDF
fixture-local Z,M,A order, not broad IDF/epJSON declaration-order parity. Gas
species, mixture fractions/order, referenced gas identity/thickness, gap
pressure, helper state, use/occurrence, complex-fenestration packing,
deflection/pillar algorithms, optics/thermal behavior, surfaces, ratings,
daylighting, Rust EIO serialization, runtime, broad diagnostics, conformance,
and whole-routine `GetMaterialData` behavior remain unclaimed.

### `WindowMaterial:ComplexShade`

The twenty-second and final source-order object in `GetMaterialData` defines
the thermal and opening/slat state for a complex-fenestration shading layer.
The six case-insensitive layer types map as follows: `VenetianHorizontal` and
`VenetianVertical` retain their directional Venetian identities, `Woven`,
`Perforated`, and `BSDF` retain those identities, and
`OtherShadingType` maps to the source diffuse-shade layer type.
`OtherShadingType` is the default.

The EnergyPlus 26.1 schema/default contract is:

- thickness 0.002 m and conductivity 1 W/m-K, each strictly greater than zero
- IR transmittance 0 in `[0,1]`, and front/back emissivity 0.84 in `(0,1]`
- top, bottom, left, and right opening multipliers 0 and front opening
  multiplier 0.05, each in `[0,1]`
- slat width 0.016 m, spacing 0.012 m, thickness 0.0006 m, angle 90 degrees,
  conductivity 160 W/m-K, and curve 0 m; the three dimensions and
  conductivity are positive, angle is in `[-90,90]`, and curve is nonnegative

`GetMaterialData` copies every one of those source-effective values. It fixes
the base state to `Rough` and R-only, stores IR transmittance plus directional
emissivities, and deliberately assigns back emissivity to scalar
`AbsorpThermal` while front/back emissivity remain in
`AbsorpThermalFront`/`AbsorpThermalBack`. Resistance and nominal resistance
remain zero; density, specific heat, and solar/visible absorptance also remain
zero. The bounded Rust payload preserves those projections without deriving
thickness divided by conductivity and without inventing an IR-transmittance
plus emissivity sum.

The scalar schema bounds apply to every layer type. The additional source
slat relationship checks run only for `VenetianHorizontal` and
`VenetianVertical`: positive width, spacing, thickness, and conductivity;
angle within `[-90,90]`; and curve either zero or not below half the slat
width. EnergyPlus 26.1 `Material.cc` rejects a positive curve strictly below
`SlatWidth/2` but accepts exact equality despite its diagnostic/IDD wording.
The typed compiler intentionally preserves that input-stage equality quirk.
Downstream TARCOG later rejects any nonzero curve whose absolute value is less
than or equal to half the width; that runtime-only stricter check is not
promoted into this material compiler boundary. Non-Venetian inputs receive no
invented slat relationship or geometry validation.

The object has no internal reference. Its normalized name participates in the
shared material namespace, and all field/relationship validation completes
before identity reservation. It uses the same
`MaterialFamily::ComplexFenestration` boundary as `WindowMaterial:Gap`.
Ordinary `Construction` rejects either complex-fenestration material in every
position. The intended `Construction:ComplexFenestrationState` consumer and
its alternating solid/gap packing, directional optical matrices,
`WindowThermalModel:Params`, opening-area conversion, TARCOG/WCE thermal and
slat behavior, BSDF optics, shading flags, surfaces, ratings, and daylighting
remain deferred. Arbitrary-run support assessment counts and blocks every
definition, including unused definitions, as
`UnsupportedSurfaceBoundary`/`RunBlocked` with no runtime class.
EIO/reporting beyond the bounded generic diagnostic below, Rust serialization,
runtime execution, broad diagnostic parity, and conformance remain unclaimed.

`window_material_complex_shade_001` adds only a bounded, nonblocking,
diagnostic EnergyPlus 26.1 generic `Material Details` comparison through
`crates/ep_cli/src/window_material_complex_shade.rs::run_compare_window_material_complex_shade`
and the `WindowMaterial:ComplexShade Generic Definition Details` proof
variable. Its warning-free, no-zone source-IDF fixture keeps seven unused
definitions in fixture-local Z,Y,X,W,V,U,T order. The both-selector
`Constructions,Materials` and Materials-only lanes each contain one exact
11-token generic header and seven exact target rows; Constructions-only and
blank/default contain neither the generic header nor target data.

Every target row is `Rough`, has zero resistance, density, specific heat,
solar absorptance, and visible absorptance, reports its definition's own
thickness and conductivity, and maps BackEmissivity to thermal absorptance.
An empty `Material:Air` header in Materials-enabled lanes and empty generic
Construction CTF, Material CTF Summary, Material:Air CTF Summary, and CTF
headers in Constructions-enabled lanes remain outside target evidence. No
dedicated `WindowMaterial:ComplexShade`, `WindowMaterial:Glazing`, or
`WindowConstruction` data row appears. `ConvertInputFormat` reorders the
definitions T,U,V,W,X,Y,Z and converted epJSON EIO follows that order, so only
source-IDF fixture-local Z,Y,X,W,V,U,T ordering is claimed.

Layer type, infrared transmittance, front emissivity, opening multipliers,
slat geometry and properties, curve state, construction use or occurrence,
`Construction:ComplexFenestrationState` packing, TARCOG/WCE/BSDF behavior,
thermal behavior, Rust EIO serialization, runtime, broad diagnostics, and
conformance remain unclaimed. The comparator does not promote
`GetMaterialData` or another parent source routine wholesale.

`MaterialFamily` and `ConstructionKind` separate opaque and fenestration
consumers. `Material:RoofVegetation` joins the opaque family with a dedicated
outside-layer invariant. The two ordinary glazing variants, `WindowMaterial:Gas`,
`WindowMaterial:GasMixture`, `WindowMaterial:Shade`,
`WindowMaterial:Screen`, and `WindowMaterial:Blind` use the ordinary
fenestration family, while
equivalent-layer glazing, `WindowMaterial:Gap:EquivalentLayer`, and
`WindowMaterial:Shade:EquivalentLayer` plus
`WindowMaterial:Drape:EquivalentLayer` and
`WindowMaterial:Screen:EquivalentLayer` plus
`WindowMaterial:Blind:EquivalentLayer` share the separate equivalent-layer
family. Thermochromic glazing-group parents use their own deferred-consumer
family, SimpleGlazing definitions use a separate fully blocked family, and
complex-fenestration gaps and shades share a dedicated deferred-consumer
family. An ordinary `Construction` accepts the
unshaded `Glass ((Gas|GasMixture) Glass){0..3}` subset, the bounded exterior,
interior, double-between, and triple-between Shade or Blind patterns above,
and one exterior Screen directly before that plain window stack. It rejects
gas-only, trailing-gas, adjacent-glass, adjacent-gas, invalid shading-device
placement, overlong, and mixed opaque/window stacks. A
`BuildingSurface:Detailed` cannot reference that fenestration construction.
The opaque runtime cache, execution plan, and construction-material CLI
comparison filter it out, while hand-built typed models that cross the family
boundary fail with dedicated runtime errors. Arbitrary-run support assessment
also counts every typed `WindowMaterial:Gas` and `WindowMaterial:GasMixture`
occurrence as explicitly unsupported and run-blocks it before execution; the
same explicit run block applies to every typed equivalent-layer gap, shade,
drape, screen, or blind, every ordinary screen or blind definition, and every
RoofVegetation definition, thermochromic group, SimpleGlazing definition, and
complex-fenestration gap or shade. Glazing
thickness, conductivity, and
asymmetric infrared emissivity, gap thickness and resolved single-gas or
ordered mixture properties, and the equivalent-layer shade/drape TAR inputs
plus equivalent-layer screen sentinel, visible, infrared, and wire-geometry
state, ordinary-blind slat geometry/optics, and equivalent-layer blind
geometry, blank-group/index quirks, TAR, infrared, thermal, and control state
stay in their dedicated payloads and are never projected through opaque
material accessors.

The compiler preserves EnergyPlus family order by compiling all `Material`
objects, then all `Material:NoMass`, `Material:AirGap`, and
`Material:InfraredTransparent` objects, followed by
`WindowMaterial:Glazing` and then
`WindowMaterial:Glazing:RefractionExtinctionMethod` and
`WindowMaterial:Glazing:EquivalentLayer` objects, followed by
`WindowMaterial:Gas`, `WindowMaterial:Gap:EquivalentLayer`,
`WindowMaterial:GasMixture`, `WindowMaterial:Shade`, and
`WindowMaterial:Shade:EquivalentLayer`, then
`WindowMaterial:Drape:EquivalentLayer`, `WindowMaterial:Screen`, and
`WindowMaterial:Screen:EquivalentLayer`, followed by `WindowMaterial:Blind`,
`WindowMaterial:Blind:EquivalentLayer`, `Material:RoofVegetation`, and
`WindowMaterial:GlazingGroup:Thermochromic`, then
`WindowMaterial:SimpleGlazingSystem`, then `WindowMaterial:Gap`, then
`WindowMaterial:ComplexShade`, and keeps their names in the shared material
registry. Because the thermochromic group is compiled first, its
child references cannot resolve the later SimpleGlazing family.
`material_opaque_variants_001` adds
nonblocking diagnostic EnergyPlus 26.1 grouped-EIO evidence for its exact
static fixture: construction and layer counts plus every outside-to-inside
material name, order, and thermal resistance are compared for the regular,
two-adjacent-AirGap, reversed, and sole-layer IRT construction groups. The
generic `Material CTF Summary` row emitted for IRT is treated as a row shape,
not as an independent object-type discriminator.

`window_glazing_spectral_average_001` separately adds nonblocking
EnergyPlus 26.1 exact-EIO evidence for the bounded glazing branch. Its clean
oracle run emits exactly one `WindowMaterial:Glazing` row, and the Rust bridge
compares all 16 data fields: normalized material name, optical data type,
blank spectral dataset, thickness, every solar/visible/infrared optical
value, both thermal emissivities, conductivity, dirt factor, and
solar-diffusing state. Young's modulus and Poisson's ratio remain typed-only
evidence because EnergyPlus omits them from that EIO row. Exact
`WindowConstruction` plus host and fenestration `HeatTransfer Surface` rows
prove that the oracle material is assigned to an actual detailed window;
they do not prove a Rust window-surface or runtime path.

`window_glazing_refraction_extinction_001` adds the corresponding
nonblocking EnergyPlus 26.1 exact-EIO gate for the complete alternative-input
object. It locks a clean oracle run, the normalized `SpectralAverage` material
row, all 16 emitted fields, the asymmetric solar/visible formula results, and
the 26.1 visible-back copy quirk. Thickness is exact-gated against EIO; only
the four raw solar/visible n/k inputs are reported as typed-only evidence
because EIO omits them. Exact
`WindowConstruction` plus host and fenestration `HeatTransfer Surface` rows
prove actual use only on the oracle side; the Rust window runtime remains
unimplemented.

`window_glazing_equivalent_layer_001` adds a dedicated nonblocking
EnergyPlus 26.1 exact-EIO gate for the complete typed equivalent-layer
material. Its row has 18 CSV tokens total: the row label plus 17 data fields.
Rust compares the material identity, `SpectralAverage`, the emitted blank
dataset slot, and all 14 solar/infrared numeric fields. The 11 visible inputs
and thermal resistance remain typed-only because this EIO row omits them.
Parser and CLI tests separately lock the `-99999` EIO sentinel used when the
three solar diffuse-diffuse inputs are `Autocalculate`, without treating it as
an ASHWAT-derived value. EnergyPlus emits a material row for each
equivalent-layer construction-layer occurrence; every oracle occurrence is
validated by material identity while construction occurrence parity remains
unclaimed. The exact one-solid `Construction:WindowEquivalentLayer`, opaque
host, and detailed-window rows prove fixture use only on the oracle side.
Their layer semantics, U-factor, SHGC, solar transmittance, surface behavior,
and runtime are not Rust parity claims.

`WindowMaterial:Gas` compiler tests lock all four standard-gas constant records
and source replacement of valid custom fields; Custom coefficient storage,
missing-ratio zero, 300 K conductivity and nominal-resistance derivation;
required fields, enum and numeric bounds, shared-name and source order; typed
coverage; and the exact `Glass (Gas Glass){0..3}` Construction subset.
`window_material_gas_001` adds a nonblocking EnergyPlus 26.1 exact-EIO gate for
the duplicate-aware multiset of gas-layer occurrences in ordinary window
Constructions. It covers Air, Argon, Krypton, Xenon, and Custom, repeats a
reused gas, excludes an unused definition, and compares material name, gas
type, and thickness for every occurrence. Multiplicity, name, and canonical
type are exact; Rust thickness is normalized with the source `{:.3R}` policy
before an exact numeric comparison. EIO does not expose Custom
coefficients, resolved properties, or nominal resistance, so those remain
typed-test evidence only. Runtime-boundary tests separately lock the explicit
arbitrary-run block. Rust/EnergyPlus row-order parity, window optics and thermal
execution, runtime behavior, and conformance remain unclaimed.

`WindowMaterial:Gap:EquivalentLayer` compiler tests lock its uppercase gas
tokens, required vent mode, all standard/Custom property behavior, shared
material names, source order, consumer family, and explicit runtime block.
`window_material_gap_equivalent_layer_001` adds the clean seven-occurrence
exact-EIO sequence described above, including duplicate reuse and unused
definition exclusion. EIO exposes only material name, canonical gas type,
source-formatted thickness, and canonical vent type; the remaining typed
fields and every construction/runtime behavior remain outside the gate.

`WindowMaterial:GasMixture` compiler tests lock the one-through-four active
component shapes and order, the required Gas 1/Gas 2 input pairs, one-gas
dummy-pair discard, missing active Gas 3/Gas 4 fraction zero, supplied-field
bounds, safe missing-active-type failure, inactive-field validation and
discard, standard-gas-only enum, non-unit sums, duplicates, first-gas-only
nominal resistance, source order, shared names, ordinary-construction
alternation, consumer family, and explicit runtime block. EnergyPlus emits no
dedicated mixture construction-layer data row.
`window_material_gas_mixture_001` instead adds the clean six-definition
generic-`Material Details` gate described above, including the unused
definition, the exact shared header with zero gas rows, and two oracle-only
seven-layer construction summaries. Component and occurrence semantics remain
typed/source evidence because the generic report omits them.

`WindowMaterial:Shade` compiler tests lock required-field and exclusive-bound
validation, source defaults and fixed values, all three optical-sum checks,
solar and visible absorptance behavior, nominal resistance, shared-name and
source order, the bounded exterior/interior/between-glass Construction
patterns, solar-diffusing rejection, adjacent-gap gas signatures and width
tolerance, safe failure of the two exterior/interior gas-adjacency holes,
consumer family, and explicit runtime block.
`window_material_shade_001` separately locks the bounded generic definition
and duplicate-aware specialized construction-occurrence EIO shapes described
above, including reuse and unused-definition behavior, without promoting
window execution or conformance.

`WindowMaterial:Shade:EquivalentLayer` compiler tests lock all eleven inputs,
the four required fields, the source-effective blank-zero visible fields,
schema defaults and inclusive/exclusive endpoints, all five strict-sum gates,
the duplicated front/back solar beam-beam value, the front-only visible TAR
storage and zero back-visible quirk, fixed roughness/resistance-only and
thermal projections, shared-name/source order, equivalent-layer consumer
family, ordinary-Construction rejection, typed coverage, and explicit runtime
block. `window_material_shade_equivalent_layer_001` separately locks the
all-definition generic zero rows and the duplicate-aware specialized
equivalent-layer construction occurrences described above, without promoting
construction, surface, window execution, or conformance.

`WindowMaterial:Drape:EquivalentLayer` compiler tests lock all thirteen inputs,
the four required directional solar fields, defaults and inclusive/exclusive
bounds, source-effective blank-zero visible fields, and exactly the source's
three sum gates. They preserve acceptance of front infrared equality and the
absence of back solar/back infrared sum validation, duplicated front/back
solar beam-beam storage, front-only visible TAR storage with zero back-visible
state, fixed roughness/resistance-only behavior and thermal projections,
all-or-nothing effective pleat dimensions, shared-name/source order,
equivalent-layer family classification, ordinary-`Construction` rejection,
typed coverage, and the explicit runtime block.
`window_material_drape_equivalent_layer_001` separately locks every generic
definition row and the malformed-header, duplicate-aware A,Z,Z,P,Q
construction-occurrence sequence described above, without promoting
construction, surface, EMS, window execution, or conformance.

`WindowMaterial:Screen` compiler tests lock the exact reflected-beam enum and
default, four required fields, every schema default and inclusive/exclusive
bound, the discrete map-resolution set, positive spacing and diameter plus
the strict diameter-below-spacing relationship, solid-fraction optical and
nominal-resistance derivations, shared-name/source order, Fenestration family
classification, the exterior-only plain-window Construction subset, explicit
rejection of the unsafe `Screen, Gap, Glass` source hole, typed coverage, and
the all-definition runtime block. `window_material_screen_001` separately
locks all generic definition rows, the exact specialized header, the
duplicate-aware A,Z,Z occurrence sequence, the fixture-bounded normal and
diffuse initialization replay, and independent Materials/Constructions report
activation without promoting window runtime or conformance.

`WindowMaterial:Screen:EquivalentLayer` compiler tests lock all ten numeric
inputs, the exact five required fields, inclusive/exclusive schema bounds, the
preserved `Autocalculate` state, source-effective blank geometry at 0 m / 0 m,
and the explicit geometry thresholds. They also lock duplicated solar state,
front-only visible assignments, N6's diffuse-diffuse slot, initialized-zero
back-visible state, shared infrared and directional thermal projections, the
two source-owned optical sums, omission of beam-diffuse transmittances from
those sums, and omission of the ineffective scalar-`AbsorpThermal` infrared
sum. The asymmetric greater-than-one-percent openness recovery fails closed;
shared-name/source order, EquivalentLayer family classification,
ordinary-`Construction` rejection, typed coverage, and the all-definition
runtime block are covered. `window_material_screen_equivalent_layer_001`
separately locks the generic Z,M,A definition rows, malformed nine-token
specialized header, exact twelve-token A,Z,Z occurrence rows, raw
`Autocalculate` and blank-geometry sentinels, source `{:.4R}`/`{:.5R}`
serialization, and independent Materials/Constructions report activation.
Equivalent-layer construction behavior, surfaces, EMS, runtime, and
conformance remain unclaimed.

`WindowMaterial:Blind` compiler tests lock the orientation enum/default, all
27 numeric inputs, the seven required numeric fields, every schema or
source-effective default and inclusive/exclusive bound, all ten strict optical
sums, and the six beam/diffuse equality rules with their greater-than-1e-5
failure threshold. They also lock the surviving width-below-separation
warning, half-width blind-to-glass distance rule, gated inverse-sine slat-angle
geometry, and absence of an N26/N27 relationship. Source optical-slot
projection, initialized-zero base state, `Rough`/resistance-only behavior,
shared-name/source order, Fenestration family classification, the bounded
exterior/interior/double-between/triple-between ordinary-Construction
patterns, common Shade/Screen/Blind device-count and solar-diffusing limits,
adjacent-gap signature/width equality plus the blind gap-sum width rule,
fail-closed unsafe end holes, typed coverage, and the all-definition runtime
block are covered. `window_material_blind_001` separately locks its bounded
generic and specialized static EIO rows without promoting `CalcBlindProperties`
or blind numerical runtime.

`WindowMaterial:Blind:EquivalentLayer` compiler tests lock the exact six
required fields, both enums and defaults, all 21 numeric bounds, the four and
only four source strict optical sums, and the blank-sensitive N9-N12 and
N13-N15 assignment groups. They also lock the N16-N18 gate that copies
N13-N15 rather than the supplied visible values, the individually blank
N19-N21 initialized-zero state, zero beam-beam slots, source warning/recovery
order for separation, width, and crown, `Rough`/resistance-only state,
directional infrared/thermal projections, shared-name/source order,
EquivalentLayer family classification, ordinary-Construction rejection,
typed coverage, and the all-definition runtime block. The dedicated static EIO
case separately locks exact generic definitions and malformed no-newline A,Z,Z
occurrences without promoting equivalent-layer construction packing, ASHWAT
behavior, optics/thermal/control execution, surfaces, ratings, daylighting,
runtime, EIO serialization, or conformance.

`Material:RoofVegetation` compiler tests lock every default, all 15 numeric
schema ranges including their inclusive/exclusive endpoints, both public
enum/default contracts, the ignored-but-type-checked soil-layer name, shared
material identity and source order, the warning-only initial-moisture clamp,
and the dry-soil opaque projections. Construction tests accept an outside
RoofVegetation layer and deliberately fail closed for every interior
occurrence, including the upstream interior-only validation hole. Support
assessment tests count and run-block all typed definitions, including unused
ones. `material_roof_vegetation_001` separately locks only the generic
11-token dry-input definition rows and selector matrix described above.
Dynamic EcoRoof state, surface use, CTF coupling, moisture and plant physics,
broad EIO behavior, runtime, and conformance remain unclaimed.

`WindowMaterial:GlazingGroup:Thermochromic` compiler tests lock the shared
material namespace and nineteenth-object source order, ordered arena storage,
case-insensitive resolution to both supported ordinary glazing variants, and
the intentional preservation of unsorted temperatures and duplicate states.
They also lock required entry types, missing and wrong-family references, the
minimum-one fail-closed rule, ordinary-Construction rejection, typed coverage,
and the all-definition arbitrary-run block. They do not claim source handling
of unsafe empty parents, child-construction generation, dynamic state
selection, EIO output, or runtime behavior.

`WindowMaterial:SimpleGlazingSystem` model and compiler tests lock the complete
source block-model fields, `Option` visible-input identity, low/intermediate/high
U-factor branches, SHGC thresholds, the reversed intermediate summer-film
interpolation, fixed material state, explicit-visible reflectance polynomials,
and high-U warning/clamp recovery. They also lock exclusive numeric bounds,
required/type diagnostics, the shared material namespace and source order,
the unavailable thermochromic-child relationship, typed coverage, universal
ordinary-`Construction` rejection, and the all-definition arbitrary-run block.
The dedicated static case separately locks only the bounded generic
`Material Details` evidence above. The tests and case do not promote
specialized glazing or construction reporting, incident-angle or hemispherical
optics, window thermal behavior, Rust EIO serialization, runtime, or
conformance.

`WindowMaterial:Gap` model and compiler tests lock its twenty-first-object
source order, exact trailing-underscore gas field, positive own thickness and
pressure/default, case-insensitive Gas/GasMixture resolution, copied single or
ordered-mixture state, and source-material ID as graph provenance only. They
also lock lazy case-insensitive raw-helper resolution, validation then discard
of deflection initial temperature/pressure, copying only deflected thickness
and optional pillar spacing/radius, missing/ambiguous/wrong-family fail-closed
behavior, identity reservation after every fallible step, the dedicated
complex-fenestration family, universal ordinary-`Construction` rejection, typed
coverage, and the all-definition arbitrary-run block. They do not claim a
nominal resistance, helper-family typed inventory, relationship constraints
absent from the source, `Construction:ComplexFenestrationState`, specialized
window reporting, window algorithms, runtime execution, or conformance. The
dedicated static case separately locks only the bounded generic `Material
Details` evidence above.

`WindowMaterial:ComplexShade` model and compiler tests lock its
twenty-second-object source order; all six layer types and their default; all
16 scalar defaults and bounds; the fixed Rough/R-only projections; every
IR, directional-emissivity, opening, and slat field; the
BackEmissivity-to-`AbsorpThermal` projection; zero resistance and
`NominalR`; and the absence of an invented energy sum. They also lock the
Venetian-only curve relationship including acceptance at exactly half the
slat width, the absence of invented non-Venetian geometry validation, the
shared namespace and complex-fenestration family, universal ordinary
`Construction` rejection, typed coverage, and the all-definition
arbitrary-run block. Representative evidence is
`compiler::tests::window_material_complex_shade::window_complex_shade_materializes_source_defaults_and_source_order`.
The tests do not claim the complex-fenestration-state consumer, directional
matrices, TARCOG/WCE, BSDF optics, EIO/reporting, runtime, broad diagnostic
parity, or conformance. The dedicated static case separately locks only the
bounded generic `Material Details` evidence above.

`MaterialProperty:VariableAbsorptance` model and compiler tests lock both
eligible target variants; all four case-insensitive control signals and the
SurfaceTemperature default; typed user and built-in schedule references;
deferred Curve/Table identity; source-null unresolved optional names; thermal,
solar, and dual selected dependencies; selected-family-before-opposite-family
validation; exact wrong-family target rejection; normalized overlay names;
ambiguous dependency and duplicate-target fail-close behavior; validation
before identity reservation; typed coverage; and object-count inclusion. The
support-boundary test attaches overlays to both used and unused materials and
requires one explicit all-definition `UnsupportedSurfaceBoundary` run block.
No test claims surface activation, curve/schedule evaluation, timestep
mutation, clamping, the source scheduled-solar pointer defect, EIO, runtime
numerics, or conformance.

`MaterialProperty:PhaseChangeHysteresis` model and compiler tests lock both
actual public `Group::Regular` target variants (`Material` and
`Material:NoMass`); the exact thirteen-field source order; exhaustive
required, finite-number, and strict-positive validation; absence of invented
peak/property relationships; grouped liquid/solid and melting/freezing state;
source-derived transition and initial specific heat; normalized target
snapshots; case-insensitive duplicate-target rejection; validation before ID
or target reservation; coexistence with VariableAbsorptance; typed coverage;
and object-count inclusion. The support-boundary test attaches hysteresis to
both used and unused materials and requires one explicit all-definition
`UnsupportedSurfaceBoundary` run block. No test claims internal F/C-factor
targets, material pointer replacement, `hasPCM`, CondFD or PCM-storage
consumption, mutable histories, hysteresis equations, EIO/output reporting,
runtime numerics, or conformance.

`MaterialProperty:PhaseChange` model and compiler tests lock both actual public
`Group::Regular` target variants (`Material` and `Material:NoMass`); missing and
blank coefficient default zero; zero, one, two, three, and 101 complete pairs;
unbounded finite negative scalars; strict temperature increase; nondecreasing
enthalpy with equality; malformed/incomplete/nonfinite rejection; normalized
target snapshots; case-insensitive duplicate-target rejection; validation
before ID or target reservation; coexistence with VariableAbsorptance and
PhaseChangeHysteresis; typed coverage; and object-count inclusion. The
support-boundary test attaches PhaseChange to both used and unused materials and
requires one explicit all-definition `UnsupportedSurfaceBoundary` run block. No
test claims source lazy no-CondFD validation, repeated-target overwrite,
internal F/C-factor targets, `MaterialFD` or sentinel state, CondFD consumption,
one/two-point execution, interpolation or property equations, Hysteresis/VTC
precedence, EIO/output reporting, runtime numerics, or conformance.

This checkpoint does not port the IRT paired-interzone surface-use semantics
or non-interzone warnings, the CondFD prohibition and algorithm behavior, or
dynamic AirGap/IRT heat transfer or EcoRoof execution. It also does not claim exact EnergyPlus
diagnostic text, all input-processor default behavior beyond the standalone
spectral-dataset, variable-absorptance, phase-change-hysteresis, and phase-change
temperature-enthalpy contracts, internal F/C-factor
material injection, EMS mutation, broad material EIO formatting, or any of the
remaining 8 deferred overlay/dataset families.

## Routine Inventory

| Routine | Completion status | Inventory obligation |
|---|---|---|
| `GetWindowGlassSpectralData` | `state_mapped` | owns the complete bounded pre-material standalone dataset read, positional zero-fill, transmittance floor, point validation, separate typed arena/name map, and valid-unused runtime-inert boundary; active glazing consumers remain blocked |
| `MaterialGlass::SetupSimpleWindowGlazingSystem` | `state_mapped` | its complete material-owned performance-index block model, optional-visible branch, reversed intermediate-U film-resistance interpolation, and materializing high-U resistance clamp are typed; construction, angular/hemispherical optics, reporting, runtime, and conformance remain outside the mapping |
| `GetMaterialData` | `source_mapped` | owns all 22 base families and the tail variable-absorptance call; its Regular/NoMass/AirGap/InfraredTransparent, RefractionExtinctionMethod, glazing EquivalentLayer, Gas, gap EquivalentLayer, GasMixture, ordinary Shade, shade EquivalentLayer, drape EquivalentLayer, ordinary Screen, screen EquivalentLayer, ordinary Blind, blind EquivalentLayer, RoofVegetation, Thermochromic glazing-group, SimpleGlazingSystem, and complex-fenestration Gap and ComplexShade objects plus only the regular Glazing `SpectralAverage` branch are implemented; RoofVegetation, SimpleGlazingSystem, and complex-fenestration Gap and ComplexShade have bounded generic-definition CLI comparisons, while Thermochromic EIO remains unclaimed |
| `CalcScreenTransmittance` | `source_mapped` | the Screen fixture comparator reproduces only its normal-incidence A/Z paths and the values required by the bounded static EIO row |
| `CalcWindowScreenProperties` | `source_mapped` | the Screen fixture comparator reproduces only its reverse-order 18 by 18 initialization integration and fixture activation boundary |
| `ReportGlass` | `source_mapped` | owns the bounded Blind specialized header, raw seven-field row serialization, construction-occurrence order, and post-`CalcNominalWindowCond` skip behavior |
| `CalcNominalWindowCond` | `source_mapped` | owns the exact-bare-companion search and the missing-bare/between-glass error flags that make `ReportGlass` omit those construction rows; Rust fail-closes rather than reproducing this calculation |
| `GetVariableAbsorptanceInput` | `state_mapped` | owns the complete bounded post-base overlay read: exact Regular/NoMass target gate, defaulted four-way control, source-null unresolved dependency names, selected/opposite dependency rules, separate typed arena/name map, one-overlay-per-target fail-close boundary, and universal runtime block |
| `GetVariableAbsorptanceSurfaceList` | `source_mapped` | owns exterior-first-layer surface activation and interior-layer warnings; no Rust execution state is added by the typed-input checkpoint |
| `UpdateVariableAbsorptances` | `source_mapped` | owns schedule/function trigger evaluation, exterior thermal/solar mutation, clamping, and the EnergyPlus 26.1 scheduled-solar pointer defect; all runtime behavior remains unsupported |
| `GetHysteresisData` | `state_mapped` | owns the complete bounded post-base hysteresis attachment read: public Material/NoMass target gate, thirteen required strict-positive inputs, grouped typed state and source-derived specific heats, duplicate-target fail-close boundary, and universal runtime block |
| `GetCondFDInput` | `state_mapped` | owns the complete bounded PhaseChange typed-input pass: public Material/NoMass target gate, defaulted finite conductivity-temperature coefficient, unbounded complete ordered point vector, duplicate-target fail-close boundary, deliberate eager validation, and universal runtime block; its CondFD settings and VariableThermalConductivity passes and all numerical state remain unsupported |
| `GetMoistureBalanceEMPDInput` | `source_mapped` | owns the EMPD settings overlay |
| `GetHeatBalHAMTInput` | `source_mapped` | owns the six ordered HAMT objects |

All fourteen routine records have `required_for_full_domain = false`. The
bounded implementation slice does not promote the whole `GetMaterialData`,
`CalcScreenTransmittance`, `CalcWindowScreenProperties`, `ReportGlass`, or
`CalcNominalWindowCond` routines beyond `source_mapped`. Only the declared
standalone `GetWindowGlassSpectralData` input boundary and the material-owned
`SetupSimpleWindowGlazingSystem` calculation plus the declared
`GetVariableAbsorptanceInput` overlay boundary, `GetHysteresisData` attachment
boundary, and the declared `GetCondFDInput` PhaseChange pass are `state_mapped`
within their bounded input domains.

## Evidence And Promotion Boundary

The existing `construction_materials_001` case remains nonblocking smoke
evidence for selected static EIO fields of its existing regular and no-mass
inputs. `material_opaque_variants_001` separately contributes nonblocking
diagnostic grouped-EIO evidence for the exact four-construction fixture. Its
gate compares all 10 emitted material-layer rows, including two adjacent
AirGap layers in both directions and the sole IRT layer, without promoting a
runtime or conformance claim.

Typed model/compiler tests additionally prove that the four original opaque states,
the partial regular-glazing state, and the complete RefractionExtinction,
glazing EquivalentLayer, Gas, gap EquivalentLayer, GasMixture, ordinary Shade,
shade EquivalentLayer, drape EquivalentLayer, ordinary Screen, and screen
EquivalentLayer plus ordinary Blind, blind EquivalentLayer, RoofVegetation,
Thermochromic glazing-group, SimpleGlazingSystem, and complex-fenestration Gap
and ComplexShade states are represented separately; their required fields, defaults,
exclusive/inclusive bounds, regular-glazing energy sums, shared names, source
order, formulas, 26.1 quirks, Autocalculate states, uppercase equivalent-gap
gas tokens, required vent modes, standard/custom gas resolution, ordered
mixture prefix semantics, equivalent-layer shade five-sum constraints and
front-only visible storage, equivalent-layer drape three-sum constraints,
omitted back-side checks, front-infrared equality, front-only visible storage,
source-effective pleats, Screen solid-fraction optical derivations, screen
EquivalentLayer sentinel/blank-geometry/visible-storage quirks, Blind optical
sums/equalities/geometry/slot projections, blind EquivalentLayer
blank-group/index quirks, four optical sums, warning-only geometry recovery,
infrared defaults, RoofVegetation defaults/bounds/moisture clamp and dry-soil
projections, Thermochromic ordered temperature/MaterialId references and
minimum-one safety gate, SimpleGlazing optional-visible identity, complete
block-model formulas, reversed intermediate-U interpolation, and high-U
warning/clamp, complex-gap copied gas/helper state and provenance-only source
identity, ComplexShade fixed Rough/R-only projections, back-emissivity scalar
mapping, Venetian-only curve relationship with half-width equality accepted,
and family boundaries are compiled; and the bounded
AirGap/IRT construction invariants, equivalent-layer construction exclusion,
ordinary Glass/Gas-or-GasMixture alternation, and safe exterior, interior, and
between-glass Shade/Blind patterns plus the exterior-only Screen pattern and
universal SimpleGlazing and complex-fenestration material construction
rejection are rejected or accepted as declared.
`window_glazing_spectral_average_001` adds an external exact-EIO smoke gate
for every field EnergyPlus emits from the bounded `SpectralAverage` material
slice, together with oracle-only proof that the fixture uses that construction
on a detailed window. The gate is explicitly nonblocking, diagnostic-only,
and does not compile or execute the fenestration surface in Rust.
`window_glazing_refraction_extinction_001` applies the same boundary to the
alternative-input object while comparing every normalized EIO field and
retaining raw n/k as typed-only evidence.
`window_glazing_equivalent_layer_001` compares the dedicated emitted identity
and all 14 solar/infrared numeric fields, while retaining 11 visible inputs
and thermal resistance as typed-only evidence and treating the
equivalent-layer construction and surface rows as oracle-only fixture locks.
`window_material_gas_001` compares every ordinary-window gas-layer occurrence
as a duplicate-aware multiset across all five gas types. Multiplicity, name,
and canonical gas type are exact, and thickness is exact after source
`{:.3R}` normalization. Custom coefficients/properties and nominal resistance
remain typed-only because the EIO row omits them.
`window_material_gap_equivalent_layer_001` compares the exact fixture-ordered
seven-row equivalent-layer gap sequence across all five gas types and all
three vent modes, including one reused material and excluding one unused
definition. Name, canonical gas/vent type, and source-formatted thickness are
exact; arbitrary IDF construction declaration order remains unclaimed.
`window_material_gas_mixture_001` compares all six fixture definitions by
name against their generic `Material Details` echoes, including the unused
mixture, while component and occurrence semantics remain typed-only because
the generic report omits them.
`window_material_shade_001` compares two generic Shade definition rows,
including the unused definition, plus the two specialized construction-layer
occurrences created by exterior/interior reuse. Its exact construction and
surface rows remain oracle-only integrity locks.
`window_material_shade_equivalent_layer_001` compares all three equivalent-layer
shade definitions against their generic `MediumRough`/all-zero rows, including
the unused definition. Its specialized exact-header sequence contains one
defaulted occurrence and two high-precision reused occurrences, proving that
surfaces do not multiply construction rows and that a surface-unused,
fixture-EMS-referenced construction still emits its layer; visible inputs
remain typed-only because EIO omits them.
`window_material_drape_equivalent_layer_001` compares all five fixture drape
definitions against their generic `MediumRough`/all-zero rows, including the
definition unused by every construction. Its specialized exact malformed
header and A,Z,Z,P,Q occurrence sequence lock reuse, exclusion of the unused
definition, zeroed one-sided pleats, and source `{:.4R}`/`{:.5R}` formatting;
visible inputs remain typed-only because EIO omits them.
`window_material_screen_001` compares every Screen definition against its
generic row, including unused M, and locks the exact specialized-header A,Z,Z
construction-occurrence sequence. For fixture-activated A and Z only, the
comparator reproduces the source normal-incidence calculation and reverse
18 by 18 diffuse integration needed by those EIO fields. The three clean
reporting lanes independently prove generic/specialized activation; runtime
window, control, surface, map, and conformance behavior remains unclaimed.
`window_material_screen_equivalent_layer_001` compares every equivalent-layer
Screen definition against its generic all-zero row, including unused M, and
locks the exact malformed-header A,Z,Z construction-occurrence sequence. Its A
row preserves raw `Autocalculate` and blank geometry, while its duplicated
front/back solar and infrared values plus wire geometry use source
`{:.4R}`/`{:.5R}` formatting. The three clean reporting lanes independently
prove generic/specialized activation; construction packing, ASHWAT behavior,
surfaces, EMS, runtime, and conformance remain unclaimed.

`window_material_blind_001` compares every ordinary Blind definition against
its generic all-zero row, including unused M, and locks the exact specialized
header plus A,Z,Z construction-occurrence sequence. This static evidence does
not promote `CalcBlindProperties`, window optics/thermal execution,
daylighting, shading control, surface behavior, or conformance.

`window_material_blind_equivalent_layer_001` compares every equivalent-layer
Blind definition against its generic `Rough`/all-zero row, including unused M,
and locks the exact malformed header plus logical A,Z,Z construction-occurrence
sequence. Its 18-token header names angle control while its 17-token payload
omits it; all 14 numeric payload fields use source `{:.5R}`. The gate preserves
the EnergyPlus 26.1 absence of a trailing newline and the exact following
Construction/header suffixes. A locks defaults and source-effective zero
states; duplicate Z locks a negative high-precision angle and reuse. The three
clean reporting lanes independently prove generic/specialized activation.
Equivalent-layer construction packing, ASHWAT behavior,
optics/thermal/control execution, surfaces, daylighting, ratings, EIO
serialization, runtime, and conformance remain unclaimed.

The `material_roof_vegetation_001` smoke gate locks the exact generic 11-token
definition rows, including their `{:.4R}`/`{:.3R}` numeric lexemes, in fixture
source order Z,M,A. Z is used by the sole vegetated construction; defaulted M
and explicit A are unused, but all three definitions appear exactly once. The
bounded CLI separately locks normalized identity, roughness, source-rounded
numeric values, and the dry-input resistance derivation; its numeric matching
makes no row-order or textual-serialization claim. Its primary, Materials-only,
and Constructions-only lanes isolate
generic-report activation from the shared CTF report. The used-Z CTF summary
and construction row remain oracle-only fixture locks, and there is no
dedicated RoofVegetation EIO row. This evidence excludes plant, soil-label,
moisture, and method fields; CTF/construction behavior; EcoRoof runtime and
water balance; the one-used-material rule; broad order or diagnostics; and
conformance.

Thermochromic glazing-group evidence is typed-only. No generic or specialized
EIO checkpoint is claimed because the parent generic row depends on an
upstream negative roughness index and the construction rows require the still
deferred master/child generation algorithm.

`window_material_simple_glazing_system_001` adds a bounded generic-definition
diagnostic for all-unused Z,M,A in fixture source IDF order. Both-selector and
Materials-only runs each contain one exact header and three exact 11-token
rows; Constructions-only and blank/default runs contain none. Z and M share
U-factor 2.7 but differ in SHGC and visible input, so their identical generic
numeric payloads lock the report's non-exposure of those optical inputs; A at
U-factor 5 locks a distinct thermal payload. All rows are `VerySmooth`, use the
source `{:.4R}`/`{:.3R}` descriptors, and retain zero density, specific heat,
solar absorptance, and visible absorptance plus 0.84 thermal absorptance. The
fixture has no specialized glazing or window-construction header/data. SHGC,
visible and derived optics, use/reuse occurrences, `ReportGlass`, construction
ratings, angular/hemispherical optics, runtime, Rust EIO serialization, broad
diagnostic/declaration-order parity, and conformance remain outside the
boundary; neither parent source routine is promoted wholesale.

`window_material_gap_001` adds a bounded generic-definition diagnostic for
three unused Gap definitions in a warning-free no-zone source-IDF fixture.
Both-selector and Materials-only runs each contain one exact 11-token header
and the exact Z,M,A target rows; Constructions-only and blank/default contain
no generic header or data. Every target is `Rough`, locks source `{:.4R}` own
thickness, and has zero resistance, conductivity, density, specific heat, and
absorptances. Z and M both report 0.0127 m despite different referenced gas,
gap pressure, deflection, and pillar state; A reports 0.006 m. Two unrelated
Gas/GasMixture generic rows and an empty `Material:Air` header are allowed in
Materials-enabled lanes. Empty generic CTF headers in Constructions-enabled
lanes are not window-specific evidence, and no dedicated Gap, Glazing, or
WindowConstruction header/data appears. Converted epJSON emits A,M,Z, so only
the source-IDF fixture-local sequence is claimed. Gas/mixture/helper state,
pressure, use, complex-fenestration algorithms, optics/thermal behavior,
surfaces, ratings, daylighting, Rust EIO serialization, runtime, broad
diagnostics, conformance, and whole-routine promotion remain outside the
boundary.

`window_material_complex_shade_001` adds a bounded generic-definition
diagnostic for seven unused ComplexShade definitions in a warning-free
no-zone source-IDF fixture. Both-selector and Materials-only runs contain one
exact 11-token generic header and the exact Z,Y,X,W,V,U,T target rows;
Constructions-only and blank/default contain no generic header or target data.
Every target is `Rough`, reports its own thickness and conductivity, maps
BackEmissivity to thermal absorptance, and has zero resistance, density,
specific heat, solar absorptance, and visible absorptance. Empty
`Material:Air` and generic CTF headers remain outside target evidence, and no
dedicated ComplexShade, Glazing, or WindowConstruction data row appears.
Converted epJSON emits T,U,V,W,X,Y,Z, so only the source-IDF fixture-local
sequence is claimed. Layer type, infrared transmittance, front emissivity,
openings, slats, curve, use, complex-fenestration-state packing, TARCOG/WCE,
BSDF, thermal behavior, Rust EIO serialization, runtime, broad diagnostics,
conformance, and whole-routine promotion remain outside the boundary.

These tests and static EIO smokes remain bounded evidence, not an EnergyPlus
material-family or window gate.

These material EIO smokes do not promote numerical window behavior, runtime
execution, or conformance.

CP58 remains incomplete until, at minimum:

- the three deferred `WindowMaterial:Glazing` optical branches have
  schema-complete typed variants
- the remaining 8 overlays/datasets have typed attachment and validation
  models
- source-order attachment, duplicate/reference diagnostics, generated
  F/C-factor materials, reporting, EMS, and algorithm-specific consumers are
  mapped and implemented
- declared blocking families cover opaque, window, equivalent-layer,
  complex-fenestration, eco-roof, EMPD, CondFD/PCM, and HAMT behavior

No material-family, construction, conduction, window, moisture, phase-change,
or heat-balance conformance claim is added by this checkpoint.
