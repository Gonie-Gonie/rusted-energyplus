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
  `GetMaterialData`, and `GetVariableAbsorptanceInput`
- `src/EnergyPlus/Material.hh` for the base-material class hierarchy
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
| typed Rust material variants | 10 | Four complete opaque-object slices, the `WindowMaterial:Glazing` `SpectralAverage` branch, and the complete `RefractionExtinctionMethod`, glazing `EquivalentLayer`, `WindowMaterial:Gas`, gap `EquivalentLayer`, and `WindowMaterial:GasMixture` objects have distinct payloads. |
| complete bounded public-object slices | 9 / 34 | `Material`, `Material:NoMass`, `Material:AirGap`, `Material:InfraredTransparent`, `WindowMaterial:Glazing:RefractionExtinctionMethod`, `WindowMaterial:Glazing:EquivalentLayer`, `WindowMaterial:Gas`, `WindowMaterial:Gap:EquivalentLayer`, and `WindowMaterial:GasMixture` have their source-effective fields and bounded compiler contracts typed. |
| partial bounded public-object slices | 1 / 34 | Only `WindowMaterial:Glazing` with `Optical Data Type = SpectralAverage` is typed; `Spectral`, `SpectralAndAngle`, and `BSDF` remain explicitly unsupported. |
| wholly deferred public objects | 24 / 34 | The other 12 base definitions and all 12 overlays/datasets remain unported as variants. |

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
| 11 | `WindowMaterial:Shade` | window shade | deferred |
| 12 | `WindowMaterial:Shade:EquivalentLayer` | equivalent-layer shade | deferred |
| 13 | `WindowMaterial:Drape:EquivalentLayer` | equivalent-layer drape | deferred |
| 14 | `WindowMaterial:Screen` | exterior window screen | deferred |
| 15 | `WindowMaterial:Screen:EquivalentLayer` | equivalent-layer screen | deferred |
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
objects 6 through 8 complete bounded variants.

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

`MaterialFamily` and `ConstructionKind` separate opaque and fenestration
consumers. The two ordinary glazing variants, `WindowMaterial:Gas`, and
`WindowMaterial:GasMixture` use the ordinary fenestration family, while
equivalent-layer glazing and `WindowMaterial:Gap:EquivalentLayer` share the
separate equivalent-layer family. An ordinary `Construction` accepts exactly
`Glass ((Gas|GasMixture) Glass){0..3}`: one through four glazing panes,
beginning and ending with ordinary glazing and alternating with up to three
typed single-gas or gas-mixture gaps. It
rejects gas-only, trailing-gas, adjacent-glass, adjacent-gas, overlong, and
mixed opaque/window stacks. Shade, blind, and screen dependencies remain
untyped. A
`BuildingSurface:Detailed` cannot reference that fenestration construction.
The opaque runtime cache, execution plan, and construction-material CLI
comparison filter it out, while hand-built typed models that cross the family
boundary fail with dedicated runtime errors. Arbitrary-run support assessment
also counts every typed `WindowMaterial:Gas` and `WindowMaterial:GasMixture`
occurrence as explicitly unsupported and run-blocks it before execution; the
same explicit run block applies to every typed equivalent-layer gap. Glazing
thickness, conductivity, and asymmetric infrared emissivity, plus gap
thickness and resolved single-gas or ordered mixture properties, stay in
their fenestration payloads and are never projected through opaque material
accessors.

The compiler preserves EnergyPlus family order by compiling all `Material`
objects, then all `Material:NoMass`, `Material:AirGap`, and
`Material:InfraredTransparent` objects, followed by
`WindowMaterial:Glazing` and then
`WindowMaterial:Glazing:RefractionExtinctionMethod` and
`WindowMaterial:Glazing:EquivalentLayer` objects, followed by
`WindowMaterial:Gas`, `WindowMaterial:Gap:EquivalentLayer`, and
`WindowMaterial:GasMixture`, and keeps their names in the shared material
registry.
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
| `GetMaterialData` | `source_mapped` | owns all 22 base families and the tail variable-absorptance call; its Regular/NoMass/AirGap/InfraredTransparent, RefractionExtinctionMethod, glazing EquivalentLayer, Gas, gap EquivalentLayer, and GasMixture objects plus only the regular Glazing `SpectralAverage` branch are implemented |
| `GetVariableAbsorptanceInput` | `source_mapped` | owns the post-base variable-absorptance overlay |
| `GetHysteresisData` | `source_mapped` | owns the post-base hysteresis overlay |
| `GetCondFDInput` | `source_mapped` | owns PhaseChange then VariableThermalConductivity |
| `GetMoistureBalanceEMPDInput` | `source_mapped` | owns the EMPD settings overlay |
| `GetHeatBalHAMTInput` | `source_mapped` | owns the six ordered HAMT objects |

All seven routine records have `required_for_full_domain = false`. The
bounded implementation slice does not promote the whole
`GetMaterialData` routine beyond `source_mapped`.

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
glazing EquivalentLayer, Gas, gap EquivalentLayer, and GasMixture states are
represented separately; their required fields, defaults,
exclusive/inclusive bounds, regular-glazing energy sums, shared names, source
order, formulas, 26.1 quirks, Autocalculate states, uppercase equivalent-gap
gas tokens, required vent modes, standard/custom gas resolution, ordered
mixture prefix semantics, and family boundaries are compiled; and the bounded
AirGap/IRT construction invariants, equivalent-layer construction exclusion,
and ordinary Glass/Gas-or-GasMixture alternation are rejected or accepted as
declared.
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
These tests and static EIO smokes remain bounded evidence, not an EnergyPlus
material-family or window gate.

The gas EIO smokes do not promote numerical window behavior, runtime
execution, or conformance.

CP58 remains incomplete until, at minimum:

- the other 12 base definitions and the three deferred
  `WindowMaterial:Glazing` optical branches have schema-complete typed variants
- all 12 overlays/datasets have typed attachment and validation models
- source-order attachment, duplicate/reference diagnostics, generated
  F/C-factor materials, reporting, EMS, and algorithm-specific consumers are
  mapped and implemented
- declared blocking families cover opaque, window, equivalent-layer,
  complex-fenestration, eco-roof, EMPD, CondFD/PCM, and HAMT behavior

No material-family, construction, conduction, window, moisture, phase-change,
or heat-balance conformance claim is added by this checkpoint.
