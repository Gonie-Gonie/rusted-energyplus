---
status: active
claim_level: source-mapped
owner: compiler
last_reviewed: 2026-07-15
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
| typed Rust variants | 4 / 34 | `Material`, `Material:NoMass`, `Material:AirGap`, and `Material:InfraredTransparent` belong to this bounded migration. |
| deferred typed variants | 30 / 34 | The other 18 base definitions and all 12 overlays/datasets remain unported as variants. |

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
| 5 | `WindowMaterial:Glazing` | detailed glazing definition | deferred |
| 6 | `WindowMaterial:Glazing:RefractionExtinctionMethod` | glazing using refraction/extinction input | deferred |
| 7 | `WindowMaterial:Glazing:EquivalentLayer` | equivalent-layer glazing | deferred |
| 8 | `WindowMaterial:Gas` | single-gas window gap | deferred |
| 9 | `WindowMaterial:Gap:EquivalentLayer` | equivalent-layer gap | deferred |
| 10 | `WindowMaterial:GasMixture` | multi-gas window gap | deferred |
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

## Bounded Four-Variant Contract

This checkpoint migrates only the first four base definitions from a single
option-heavy record to discriminated material definitions under a shared
identity envelope.

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

The compiler preserves EnergyPlus family order by compiling all `Material`
objects, then all `Material:NoMass`, `Material:AirGap`, and
`Material:InfraredTransparent` objects in that order, and keeps their names
in the shared material registry. `material_opaque_variants_001` adds
nonblocking diagnostic EnergyPlus 26.1 grouped-EIO evidence for its exact
static fixture: construction and layer counts plus every outside-to-inside
material name, order, and thermal resistance are compared for the regular,
two-adjacent-AirGap, reversed, and sole-layer IRT construction groups. The
generic `Material CTF Summary` row emitted for IRT is treated as a row shape,
not as an independent object-type discriminator.

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
| `GetMaterialData` | `source_mapped` | owns all 22 base families and the tail variable-absorptance call; only its Regular/NoMass/AirGap/InfraredTransparent typed slice is implemented |
| `GetVariableAbsorptanceInput` | `source_mapped` | owns the post-base variable-absorptance overlay |
| `GetHysteresisData` | `source_mapped` | owns the post-base hysteresis overlay |
| `GetCondFDInput` | `source_mapped` | owns PhaseChange then VariableThermalConductivity |
| `GetMoistureBalanceEMPDInput` | `source_mapped` | owns the EMPD settings overlay |
| `GetHeatBalHAMTInput` | `source_mapped` | owns the six ordered HAMT objects |

All seven routine records have `required_for_full_domain = false`. The
four-variant implementation slice does not promote the whole
`GetMaterialData` routine beyond `source_mapped`.

## Evidence And Promotion Boundary

The existing `construction_materials_001` case remains nonblocking smoke
evidence for selected static EIO fields of its existing regular and no-mass
inputs. `material_opaque_variants_001` separately contributes nonblocking
diagnostic grouped-EIO evidence for the exact four-construction fixture. Its
gate compares all 10 emitted material-layer rows, including two adjacent
AirGap layers in both directions and the sole IRT layer, without promoting a
runtime or conformance claim.

Typed model/compiler tests additionally prove that four object-specific states
are represented separately, required fields and fixed defaults are compiled,
and the bounded AirGap/IRT construction invariants are rejected or accepted as
declared. Those tests and the static EIO smoke remain bounded evidence, not an
EnergyPlus material-family gate.

CP58 remains incomplete until, at minimum:

- the other 18 base definitions have schema-complete typed variants
- all 12 overlays/datasets have typed attachment and validation models
- source-order attachment, duplicate/reference diagnostics, generated
  F/C-factor materials, reporting, EMS, and algorithm-specific consumers are
  mapped and implemented
- declared blocking families cover opaque, window, equivalent-layer,
  complex-fenestration, eco-roof, EMPD, CondFD/PCM, and HAMT behavior

No material-family, construction, conduction, window, moisture, phase-change,
or heat-balance conformance claim is added by this checkpoint.
