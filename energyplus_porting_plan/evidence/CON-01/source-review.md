# CON-01 independent source review

Reviewed on 2026-10-07. This review covers input admission, effective project
settings, state ownership contracts and the production admission entrypoint.
It does not certify heat-balance or HVAC numerical equivalence. The exact
executed Rust commit and artifact hashes belong to the command records and
paired comparison summaries; this review does not replace those records.

The reference is EnergyPlus 26.1.0 commit
`6f2e40d10250a105b49966baa24d843711e61048`. All 26 file/range hash pairs in
[source-boundaries.json](../../contracts/source-boundaries.json) were independently
recomputed and matched. Selected schema fields were checked against the local
locked `Energy+.schema.epJSON`, SHA256
`60b24d904e6fa553f773b38037a8a09730e6da1538647df552cd77db8d6704a9`.

## Reviewed source boundaries

| Pinned source | Selected boundary and result |
|---|---|
| `HeatBalanceManager.cc:243–327` | `GetHeatBalanceInput` calls project controls, site atmosphere, material/construction input and building geometry before radiative setup and `ManageInternalHeatGains(InitOnly=true)`. The contract records this source order. |
| `HeatBalanceManager.cc:530–722` | Building input: north axis uses signed C++ `mod(value,360)`, convergence tolerances must be positive, and minimum warmup raises maximum when larger. Invalid nonpositive day counts are rejected by the bounded guard. |
| `HeatBalanceManager.cc:724–893`; `DataHeatBalance.hh:1790–1838` | TARP inside, DOE-2 outside, CTF, surface temperature limit200C and convection bounds0.1/1000W/m2/K. Other algorithms and overrides are outside admission. |
| `HeatBalanceManager.cc:921–985`; `DataRoomAirModel.hh:170–176` | ThirdOrderBackwardDifference, no space heat-balance sizing/simulation, default Mixing with Direct coupling. Unsupported room-air models or dangling room-air zone names are rejected. |
| `HeatBalanceManager.cc:987–1250` | Absent contaminant and mass-flow-conservation declarations leave those paths inactive; absent HVAC root-finding object selects RegulaFalsi. Their nondefault objects are excluded by the whitelist. |
| `HeatBalanceManager.cc:566–585,1252–1317` | Effective atmosphere uses typed Terrain: Country0.14/270m, Suburbs or Urban0.22/370m, City0.33/460m, Ocean0.10/210m. Absent `Site:HeightVariation` fixes the temperature gradient0.0065K/m; every explicit override is outside admission. |
| `DataHeatBalance.hh:96–97`; selected schema | Blank warmup defaults are maximum25/minimum1; six days in the baseline IDFs is an explicit selected input. SimulationControl defaults are distinguished from the weather-only values selected in the cases. |
| `WeatherManager.cc:2067–2083`; `ScheduleManager.cc:2516–2532` | Civil year, EPW record year, weather ordinal and the always-leap366-day schedule ordinal have separate meanings. June30 nonleap civil/weather181 versus schedule182 is intentional. |

Active delegated input/setup helpers are `GetMaterialData` (only Material and
Material:NoMass), `GetConstructData` (opaque constructions), `CheckUsedConstructions`,
`GetBuildingData` (shadow input, zone input, opaque geometry), `InitSolarViewFactors`
and `ManageInternalHeatGains` input initialization. Their numeric bodies are not
certified here. InputProcessor object-count/retrieval and enum/YesNo conversion
boundaries are pinned in the source manifest.

Spectral glass, frame/divider, hysteresis, variable absorptance, incident-solar
multipliers, scheduled surface gains, thermochromic construction and Kiva paths
have no selected objects. Window factories still initialize BuiltIn/Simplified
defaults (`WindowManager.cc:8512–8518`, `WindowModel.cc:69–100,121–137`); absence of
windows does not count as validation of window calculations.

## Rust settings and admission

`ep_compiler::Compiler::parse_building` performs source-effective signed north
axis and warmup normalization. `porting_scope/settings.rs` projects actual typed
Building/Terrain, zone convection, calendar declarations and IdealLoads limits,
and explicitly reads raw project controls that the general compiler does not
implement. `settings.site_atmosphere` records all three effective site values.
The nine focused tests include all Terrain branches and rejection of a
default-valued `Site:HeightVariation` object.

The guard requires one opaque zone and six detailed surfaces, weather execution,
the selected algorithms and a fixed EPW. ScopeB requires one direct no-OA sensible
IdealLoads system with nonnegative hard-sized active limits. It rejects sizing,
Autosize, latent OtherEquipment, active DST/special days, non-Mixing room air,
windows, infiltration/AFN, air/plant loops, EMS and other undeclared object families.
Inactive DesignDays and Exterior:Lights are recorded separately. Exterior lighting
contributes no zone heat source; its meter calculation remains outside this gate.

Only the bounded invocation exempts the equivalent ThirdOrder/Mixing control
objects from raw-object support coverage. The strict guard checks those values,
the original compile report is retained, and ordinary support behavior is unchanged.
Admissible settings outside the hashed15-case matrix do not gain validated
numerical coverage. The14 state contracts preserve producer/consumer/timing and
units; MAT, ZT and ZTAV remain distinct. They are design contracts for later cards,
not assertions that all proposed Rust state types already exist.

## Gate interpretation and remaining work

For CON-01, production_gate means that CLI `--porting-scope A|B` reaches the real
`ep_run::run_bounded_porting` compile/support/admission path, writes the actual
trace, and rejects scope violations before graph/runtime construction. Accepted
dry runs may build the real graph and execution plan, but execute no Rust physics.
The pipeline receives original IDF/epJSON and EPW inputs; the observer harness
passes no expected-result files, actuators or physical-state setters to Rust.
The scope trace always keeps `conformance_claim=false`.

Closure requires replayable final-commit command results, matching input/artifact
hashes, the five executed24H reference branches and paired Rust settings checks,
two paired north/warmup probes, and negative production admission checks. Merely
preparing72H/annual rows does not establish execution or a numerical pass for
those durations. A gate update must retain this distinction and the execution
matrix's actual statuses.

GEO, CLK/SCH, PSY, CTF, SRC/RAD/SUR, ZON, HVAC and SYS cards still must establish
their own unit, state-transition, integration and production numerical evidence.
In particular, CTF generation/history, weather height/interpolation effects,
surface iteration, MAT/ZT/ZTAV and humidity histories, predictor/corrector loads,
IdealLoads branch internals/node/report updates, adaptive accepted system steps
and all warmup convergence conditions remain separate obligations. EP callback
observations do not certify these Rust calculations or identify every internal
HVAC branch. No downstream gate is promoted by this review.
