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
- `src/EnergyPlus/WindowManager.cc::CalcWindowScreenProperties` and the
  ordinary-window construction/material EIO writer
- `src/EnergyPlus/PhaseChangeModeling/HysteresisModel.cc::GetHysteresisData`
- `src/EnergyPlus/HeatBalFiniteDiffManager.cc::GetCondFDInput`
- `src/EnergyPlus/MoistureBalanceEMPDManager.cc::GetMoistureBalanceEMPDInput`
- `src/EnergyPlus/HeatBalanceHAMTManager.cc::GetHeatBalHAMTInput`

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
| typed Rust material variants | 15 | Four complete opaque-object slices, the `WindowMaterial:Glazing` `SpectralAverage` branch, and the complete `RefractionExtinctionMethod`, glazing `EquivalentLayer`, `WindowMaterial:Gas`, gap `EquivalentLayer`, `WindowMaterial:GasMixture`, ordinary `WindowMaterial:Shade`, shade `EquivalentLayer`, drape `EquivalentLayer`, ordinary `WindowMaterial:Screen`, and screen `EquivalentLayer` objects have distinct payloads. |
| complete bounded public-object slices | 14 / 34 | `Material`, `Material:NoMass`, `Material:AirGap`, `Material:InfraredTransparent`, `WindowMaterial:Glazing:RefractionExtinctionMethod`, `WindowMaterial:Glazing:EquivalentLayer`, `WindowMaterial:Gas`, `WindowMaterial:Gap:EquivalentLayer`, `WindowMaterial:GasMixture`, `WindowMaterial:Shade`, `WindowMaterial:Shade:EquivalentLayer`, `WindowMaterial:Drape:EquivalentLayer`, `WindowMaterial:Screen`, and `WindowMaterial:Screen:EquivalentLayer` have their source-effective fields and bounded compiler contracts typed. |
| partial bounded public-object slices | 1 / 34 | Only `WindowMaterial:Glazing` with `Optical Data Type = SpectralAverage` is typed; `Spectral`, `SpectralAndAngle`, and `BSDF` remain explicitly unsupported. |
| wholly deferred public objects | 19 / 34 | The other 7 base definitions and all 12 overlays/datasets remain unported as variants. |

This is a CP58 scaffold checkpoint. Complete inventory does not mean complete
schema, validation, runtime, optics, moisture, phase-change, or heat-transfer
behavior.

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
have been created. Hysteresis then upgrades an already-created regular
material.

## Base Definition Source Order

The following table is the public-object processing order inside
`Material::GetMaterialData`. It is not the schema presentation order.

| Source order | Public object | Source role | CP58 typed state |
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
| 16 | `WindowMaterial:Blind` | slatted blind | deferred |
| 17 | `WindowMaterial:Blind:EquivalentLayer` | equivalent-layer blind | deferred |
| 18 | `Material:RoofVegetation` | eco-roof material and vegetation state | deferred |
| 19 | `WindowMaterial:GlazingGroup:Thermochromic` | thermochromic glazing-group parent | deferred |
| 20 | `WindowMaterial:SimpleGlazingSystem` | derived simple glazing system | deferred |
| 21 | `WindowMaterial:Gap` | complex-fenestration gap, including optional deflection-state and support-pillar references | deferred |
| 22 | `WindowMaterial:ComplexShade` | complex-fenestration shade | deferred |

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

| Source sequence | Schema family number | Public object | Kind | CP58 typed state |
|---|---:|---|---|---|
| common HB 1 | 34 | `MaterialProperty:GlazingSpectralData` | standalone glazing dataset read by `GetWindowGlassSpectralData` | deferred |
| common HB 2 tail | 27 | `MaterialProperty:VariableAbsorptance` | base-material overlay read by `GetVariableAbsorptanceInput` after all 22 base families | deferred |
| common HB 3 | 25 | `MaterialProperty:PhaseChangeHysteresis` | regular-material overlay read by `GetHysteresisData` | deferred |
| CondFD 1 | 24 | `MaterialProperty:PhaseChange` | temperature/enthalpy overlay read first by `GetCondFDInput` | deferred |
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

This checkpoint migrates the first four base definitions from a single
option-heavy record to discriminated material definitions under a shared
identity envelope, adds one explicitly partial fifth variant for the
`SpectralAverage` branch of source-order object 5, and gives source-order
objects 6 through 15 complete bounded variants.

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

`Spectral` is not approximated with zero optical properties: it remains
blocked until `MaterialProperty:GlazingSpectralData` is typed in the earlier
`GetWindowGlassSpectralData` source stage. `SpectralAndAngle` remains blocked
on bivariate table/curve typing, and `BSDF` remains blocked on the complex
fenestration path.

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
The later `WindowMaterial:Gap` complex-fenestration reference path may also
consume a gas mixture, but that object remains deferred. Arbitrary-run
assessment explicitly blocks the typed mixture before execution.

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

`MaterialFamily` and `ConstructionKind` separate opaque and fenestration
consumers. The two ordinary glazing variants, `WindowMaterial:Gas`,
`WindowMaterial:GasMixture`, `WindowMaterial:Shade`, and
`WindowMaterial:Screen` use the ordinary fenestration family, while
equivalent-layer glazing, `WindowMaterial:Gap:EquivalentLayer`, and
`WindowMaterial:Shade:EquivalentLayer` plus
`WindowMaterial:Drape:EquivalentLayer` and
`WindowMaterial:Screen:EquivalentLayer` share the separate equivalent-layer
family. An ordinary `Construction` accepts the
unshaded `Glass ((Gas|GasMixture) Glass){0..3}` subset, the bounded exterior,
interior, double-between, and triple-between Shade patterns above, and one
exterior Screen directly before that plain window stack. It rejects gas-only,
trailing-gas, adjacent-glass, adjacent-gas, invalid shade or screen placement,
overlong, and mixed opaque/window stacks. Blind dependencies remain untyped. A
`BuildingSurface:Detailed` cannot reference that fenestration construction.
The opaque runtime cache, execution plan, and construction-material CLI
comparison filter it out, while hand-built typed models that cross the family
boundary fail with dedicated runtime errors. Arbitrary-run support assessment
also counts every typed `WindowMaterial:Gas` and `WindowMaterial:GasMixture`
occurrence as explicitly unsupported and run-blocks it before execution; the
same explicit run block applies to every typed equivalent-layer gap, shade,
drape, or screen and every ordinary screen definition. Glazing
thickness, conductivity, and
asymmetric infrared emissivity, gap thickness and resolved single-gas or
ordered mixture properties, and the equivalent-layer shade/drape TAR inputs
plus equivalent-layer screen sentinel, visible, infrared, and wire-geometry
state stay in their dedicated payloads and are never projected through opaque
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
`WindowMaterial:Screen:EquivalentLayer`, and keeps their names in the shared
material registry.
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

This checkpoint does not port the IRT paired-interzone surface-use semantics
or non-interzone warnings, the CondFD prohibition and algorithm behavior, or
dynamic AirGap/IRT heat transfer. It also does not claim exact EnergyPlus
diagnostic text, all input-processor default behavior, internal F/C-factor
material injection, EMS mutation, broad material EIO formatting, or any of
the deferred families.

## Routine Inventory

| Routine | Completion status | Inventory obligation |
|---|---|---|
| `GetWindowGlassSpectralData` | `source_mapped` | owns the pre-material spectral dataset read |
| `GetMaterialData` | `source_mapped` | owns all 22 base families and the tail variable-absorptance call; its Regular/NoMass/AirGap/InfraredTransparent, RefractionExtinctionMethod, glazing EquivalentLayer, Gas, gap EquivalentLayer, GasMixture, ordinary Shade, shade EquivalentLayer, drape EquivalentLayer, ordinary Screen, and screen EquivalentLayer objects plus only the regular Glazing `SpectralAverage` branch are implemented |
| `CalcScreenTransmittance` | `source_mapped` | the Screen fixture comparator reproduces only its normal-incidence A/Z paths and the values required by the bounded static EIO row |
| `CalcWindowScreenProperties` | `source_mapped` | the Screen fixture comparator reproduces only its reverse-order 18 by 18 initialization integration and fixture activation boundary |
| `GetVariableAbsorptanceInput` | `source_mapped` | owns the post-base variable-absorptance overlay |
| `GetHysteresisData` | `source_mapped` | owns the post-base hysteresis overlay |
| `GetCondFDInput` | `source_mapped` | owns PhaseChange then VariableThermalConductivity |
| `GetMoistureBalanceEMPDInput` | `source_mapped` | owns the EMPD settings overlay |
| `GetHeatBalHAMTInput` | `source_mapped` | owns the six ordered HAMT objects |

All nine routine records have `required_for_full_domain = false`. The
bounded implementation slice does not promote the whole
`GetMaterialData`, `CalcScreenTransmittance`, or
`CalcWindowScreenProperties` routines beyond `source_mapped`.

## Evidence And Promotion Boundary

The existing `construction_materials_001` case remains nonblocking smoke
evidence for selected static EIO fields of its existing regular and no-mass
inputs. `material_opaque_variants_001` separately contributes nonblocking
diagnostic grouped-EIO evidence for the exact four-construction fixture. Its
gate compares all 10 emitted material-layer rows, including two adjacent
AirGap layers in both directions and the sole IRT layer, without promoting a
runtime or conformance claim.

Typed model/compiler tests additionally prove that the four opaque states,
the partial regular-glazing state, and the complete RefractionExtinction,
glazing EquivalentLayer, Gas, gap EquivalentLayer, GasMixture, ordinary Shade,
shade EquivalentLayer, drape EquivalentLayer, ordinary Screen, and screen
EquivalentLayer
states are represented separately; their required fields, defaults,
exclusive/inclusive bounds, regular-glazing energy sums, shared names, source
order, formulas, 26.1 quirks, Autocalculate states, uppercase equivalent-gap
gas tokens, required vent modes, standard/custom gas resolution, ordered
mixture prefix semantics, equivalent-layer shade five-sum constraints and
front-only visible storage, equivalent-layer drape three-sum constraints,
omitted back-side checks, front-infrared equality, front-only visible storage,
source-effective pleats, Screen solid-fraction optical derivations, screen
EquivalentLayer sentinel/blank-geometry/visible-storage quirks, and family
boundaries are compiled; and the bounded
AirGap/IRT construction invariants, equivalent-layer construction exclusion,
ordinary Glass/Gas-or-GasMixture alternation, and safe exterior, interior, and
between-glass Shade patterns plus the exterior-only Screen pattern are
rejected or accepted as declared.
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
These tests and static EIO smokes remain bounded evidence, not an EnergyPlus
material-family or window gate.

These material EIO smokes do not promote numerical window behavior, runtime
execution, or conformance.

CP58 remains incomplete until, at minimum:

- the other 7 base definitions and the three deferred
  `WindowMaterial:Glazing` optical branches have schema-complete typed variants
- all 12 overlays/datasets have typed attachment and validation models
- source-order attachment, duplicate/reference diagnostics, generated
  F/C-factor materials, reporting, EMS, and algorithm-specific consumers are
  mapped and implemented
- declared blocking families cover opaque, window, equivalent-layer,
  complex-fenestration, eco-roof, EMPD, CondFD/PCM, and HAMT behavior

No material-family, construction, conduction, window, moisture, phase-change,
or heat-balance conformance claim is added by this checkpoint.
