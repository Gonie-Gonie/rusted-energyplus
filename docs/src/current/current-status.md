---
status: active
claim_level: none
owner: core
last_reviewed: 2026-11-15
---

# Current Status

## Fixed Porting Scope

The [fixed CON-01 inputs](../../validation/contracts/scope.json) have completed
their scoped validation: A is an opaque unconditioned one-zone model; B adds
direct no-outdoor-air sensible IdealLoads with no limit or hard-sized flow,
capacity, or combined limits. The original work covered 54 applicable units for
A and 64 for B; their accepted adjudications remain in the validation evidence.

The fifteen cases cover 24 hours, 72 hours and a year for A and each of B's
four limit modes. They use 2013 weather-only simulation at four zone timesteps
per hour. B's hard-sized limits are 0.02 m3/s and 120 W; its availability is Off
for the first three hours each day and On for the remaining twenty-one. The
input contract excludes windows, active outdoor-air paths, infiltration/AFN,
Plant, EMS, equipment autosizing, and multiple zones/equipment.

The [final validation](../../validation/evidence/SRC-06-final-current-source-validation-60/summary.json)
records fifteen normal runs, 16,427,880 reported values with zero tolerance or
RMSE failures, real Surface/warmup handoffs, and Rust JSON/CSV output rereads.
The [SUR-02 supplement](../../validation/evidence/SUR-02-isolated-history-impulse-61/summary.json)
adds the previously missing executed impulse/current-row unit checks.
File-wide byte equality is not established; headers, elapsed times and some
inactive registry counters differ. Original abnormal afterstates remain
unavailable. These finite results do not establish general EnergyPlus parity.
The [validation assets](../../validation/README.md) retain the current inputs,
policies and accepted evidence. The completed task packet is historical material
at Git commit `b7907009cefd7372727672c6f1ecc211d897a383`.

## Running And Comparing

The [example-object union](../generated/example-object-union.md) contains 116
object types: the original 110-type corpus plus accepted ComfortViewFactorAngles,
ElectricEquipment, Space, SpaceList, ZoneGroup and ZoneList additions.
Legal-value completion remains tracked per object. The retained
[input-domain evidence](../../validation/evidence/object-input-domains.json)
contains 269 paired physical cases and 12,149,416 reported values with zero
tolerance or RMSE failures. These cases span recorded Source builds; they do
not establish arbitrary legal object/value combinations.

Unused SimpleGlazing materials now have a bounded Source B direct-epJSON route: the real getter and Setup own per-material runtime properties, and a new Materials report request is checked by the unchanged whole-EIO comparison. One fixed U-factor4.2/SHGC0.57/omitted-VT scene retains the existing158-series numeric and science profile. Every Construction reference, positive window physics, Source A/IDF and full-object completion remain held; typed properties are not substituted for runtime state.

Unused named and Custom Gas materials have a bounded Source B direct-epJSON route through the real input/getter and per-material runtime row. Original schema values, Native signed-i64-effective operands, separate NominalR and constructor-zero Materials properties remain distinct. Custom retains all eleven runtime writes, omitted ratio zero and Native diagnostic/IO ordering; Source-exact names and total Raw/typed loader identity remain checked. Same-row owner checks and the fixed unused-Argon and unused-Custom comparisons retain the unchanged numerical/science, whole EIO, topology and geometry-export requirements, with no new report request or time series. Every Construction reference, active window physics, Source A/IDF, Legacy changes and full-object completion remain held.

Unused GasMixture materials have a bounded Source B direct-epJSON route through the actual five-slot MaterialGasMix owner, real5A6N/min7 input effects and exact compiler/raw/typed identities. Integer counts1..4, required Gas2 even at count1, optional active fraction zero, repeated species and non-unit sums retain Native behavior. First-gas-only NominalR is unconditional and separate from constructor-zero report Resistance; Thickness diagnostics retain their no-error-flag rule. The fixed unused Air/Argon comparison preserves the existing numerical/science, whole EIO, topology and geometry-export gates with no new output request or series. Missing active type with undefined Native indexing, every Construction/window consumer, Source A/IDF, Legacy changes and full-object completion remain held.

Unused SpectralAverage Glazing has a bounded Source B direct-epJSON route through the real7A14N/min14 ordinary MaterialGlass getter and its own GroupGlass row, with SGS Setup unchanged and uninvoked. Original schema and signed-effective numeric operands, defaults/used state, six joint optical checks and their state/diagnostic IO prefixes remain distinct. The independently fixed Float0.003/default scene retains every inherited report,158-series/75120-value, whole-EIO, topology and export criterion with no new output request or series. Public Material Details and Source geometry can establish only the accepted unused-row scope; every Construction/window consumer, Source A/IDF, Legacy, other optical modes and full-object completion remain held.

Unused RefractionExtinction Glazing has an additional bounded Source B direct-epJSON route through the real2A9N/no-min_fields MaterialGlass owner and its own GroupGlass row; the Average and SGS owners remain unchanged. Original schema and signed-effective operands, default/used effects, the one infrared >=1 check, nonpositive-conductivity no-error rule and diagnostic IO prefixes remain distinct. The independently fixed five-Float/default scene preserves every inherited158-series/75120-value, whole-EIO, topology, clock and export criterion with no new request or series. Fixed safe exp transport is a static System32 UCRT boundary on WindowsGNU and Rust exp elsewhere, not Native private-optical parity evidence. Public Material Details/Source geometry establish only the accepted unused-row scope; every Construction/window consumer, Source A/IDF, Legacy changes, further optical-owner expansion and full-object completion remain held.

Positive glazing spectral datasets and linked unused Spectral Glass have an additional bounded Source B direct-epJSON route through the actual ordered dataset getter before MaterialGlass. Zero allocation, fixed/extensible argument counts, signed-effective Tau clamp, first matching uppercase dataset name and diagnostic IO prefixes remain actual owner state; the older Average, Refraction, SGS and Gas routes stay unchanged. The independently fixed two-dataset/one-unused-Glass input retains all773 parent numeric tokens/kinds/bits and adds eight Float records plus one original u64 integer, without new requests or series. Full inherited158-series/75120-value, whole-EIO/BND,136-event topology, clock and export criteria remain mandatory. Public linked Material Details/Source geometry establish only this bounded unused-row scope; no Native private optics, every Construction/window, Source A/IDF, Legacy change, positive Curve/Table or full-object support is promoted.

Five unused analytic2D Curve getters have a bounded Source B direct-epJSON route: Biquadratic,QuadraticLinear,CubicLinear,Bicubic and FanPressureRise retain original field/name/number custody and one actual SelectedCurves owner through post-Setup Curve reporting and live Which references. No typed compiler Curve arena or evaluator is substituted. One fixed required52-number input preserves all782 parent numeric records and every inherited158-series/75120-value,whole-EIO/BND,136-event topology,clock and export criterion with no new request or series. The five-owner existing-body plan distinguishes dynamic registration/read/System output from static upstream-adapter/initializer coverage. This qualifies earlier positive-Curve holds only for these unused definitions; CurveValue,evaluation,active consumers/pressure,AFN/Duct/EMS,Table,other Curve types,Source A/IDF,Legacy changes and full-object completion remain held. Every legal field/value combination is not yet demonstrated; evaluator and active-consumer ownership are the next gaps.

Unused SpectralAndAngle Glazing has a bounded Source B direct-epJSON lookup route through the same real SelectedCurves owner and ordinary MaterialGlass GroupGlass row. Three genuine ordinal references and the Native N2..N8 skip, ordered diagnostics and BACK-reads-FRONT bounds are retained without a Curve evaluator or typed Curve arena. The fixed unused Glass/Biquadratic scene adds11 Float operands over834 held parent records with no new request or series; all inherited158/75120, full current MaterialDetails EIO, BND topology, clocks and exports remain mandatory. Four existing bodies with authored41 assertions,8 loops and4 runtime contexts provide bounded owner/lookup evidence only after actual reviewed QA and paired acceptance. Every active Construction/window consumer, CurveValue/Table/evaluation, Source A/IDF, Legacy change, private Native pointer/optics and full-object completion remain held.

Unused WindowMaterial:Shade has a bounded Source B direct-epJSON route through the genuine ordered Shade constructor/getter and public material report. All14 numeric fields retain original schema, signed-effective number, default/blank/used and total Raw/IP/name/typed-ID/loader custody; scratch field names use the original JSON keys. ROnly, constructor state, conductivity-dependent NominalR and ordered diagnostic IO remain owner state. One unused Shade adds14 Float operands over845 held parent records, without a new request or series; all158-series/75120-value, whole-current MaterialDetails EIO, BND136-event topology, clock and export criteria stay mandatory. Four existing bodies provide authored42 assertion sites,5 for loops and4 runtime owners; those counts are static coverage, not a QA or paired-result claim. Every active Construction/window/shading consumer, Source A/IDF, Legacy change, private Native state and full-object/every-valid-combination completion remains held.

Unused WindowMaterial:Screen has a bounded Source B direct-epJSON route through the genuine2A12N/min9 getter and public material report. Its13 consumed body keys,JSON-key scratch names,defaults/blanks/used state,signed-effective numbers and total Raw/IP/name/typed-ID/loader custody stay explicit. One unused Screen adds12 Float operands and1 method String over859 held numeric records,without a request or series change. Four existing bodies provide authored45 assertion sites,5 loops and4 fresh Material owners;Window unit-owner credit is0 and glass.rs is a static dependency only. A full normal supported Source completion with nonzero exported Surfaces and authenticated compiled Source annotations is mandatory for the positive Screen BeginSim/BuiltIn/W5/all-false shading-control suffix. That SOURCE_CAUSAL_INFERENCE does not export private flags or prove Native optics. Full ordered11x11 public MaterialDetails,the Source material geometry identity,and all158/75120/BND136-event/clock/export gates remain mandatory after actual acceptance. Active Construction/Screen/shading consumers,Source A/IDF,Legacy changes,Native private fields/TAR/maps and full-object completion remain held.

Unused WindowMaterial:Blind has a bounded Source B direct epJSON route through the real 2A27N getter and same Material-owned CalcWindowBlindProperties. All 181 slat angles, 37 profile angles and both bands remain required for unused Fixed input. Constructor/raw TAR1, defaults, JSON-key scratch, identity custody and diagnostic/IO order stay explicit. One unused Blind adds 27 Float operands and one Horizontal String over 871 held numeric records (898 total), without new requests. Authored 55 assertions, 6 loops and four Material owners in four existing bodies are static CODE facts, with zero Window unit owners. Actual QA/history, full normal Source completion, whole public 12x11 MaterialDetails/Source geometry, compiled producer causal evidence and all inherited 158/75120/BND/clock/export gates require separate genuine evidence. Native-private computed TAR/pointer parity, active window/Construction/control/EMS/variable-slat/daylighting/rating/thermal/EQL consumption, Source A/IDF/Legacy changes and full-object completion remain held.

Unused WindowMaterial:Shade:EquivalentLayer now has a bounded Source B direct epJSON route through the genuine1A11N getter and owned TAR1/common Material row. Defaults and blanks beyond legal NumNums5..11, original scalar kinds, total identity custody and five Native joint checks retain their own boundaries. One11-Float unused scene preserves898 parent numeric records and all requests; all158series/75120values pass unchanged policies, the whole13x11 public MaterialDetails table matches exactly and all12parent rows remain. Same-row Source geometry and six real physics surfaces are observed. Actual zero EQL Construction count skips positive unavailable EQL initialization, while ordinary same-owner Blind precomputation remains. Active EQL/window/Construction/control, ASHWAT, Native privateTAR, Source A/IDF/Legacy expansion and full-object or arbitrary-combination parity remain unclaimed.

Unused WindowMaterial:Drape:EquivalentLayer now has a bounded Source B direct epJSON route through the genuine1A13N/min4 getter and same owned TAR1/pleat/common Material row. Legal NumNums5..13, all13 default/blank slots, original versus signed-effective operands, total identity custody and three ordered joint checks retain their own boundaries. One13-Float unused scene preserves909 parent numbers and all requests; all158series/75120values pass unchanged policies, whole14x11 public MaterialDetails matches exactly and all13parent rows remain. Same-row Source geometry and six actual physics surfaces are observed. Authored47assert sites/6for/4Material contexts are separate from actual8-command QA with4current bodies and0fail/history/reuse; there are0Window unit owners. Actual zero EQL Construction count skips positive unavailable EQL work while ordinary Blind precomputation remains. Active EQL/windows/control/ASHWAT, Native privateTAR/optics, Source A/IDF/Legacy expansion and full-object/all-combination parity remain unclaimed.

Unused WindowMaterial:Screen:EquivalentLayer now has a bounded Source B direct epJSON route through the genuine 1A10N/min11 getter and inherited TAR1/common Material row. Required inputs, Auto versus blank state, all 10 scratch slots, signed-effective operands, identity custody and Native wire/openness recovery retain their boundaries. One 10-Float unused scene preserves 922 parent numbers and all requests. One Native/Source pair with zero reruns passes all 158 series/75,120 values and unchanged gates; the whole 15x11 public MaterialDetails table matches exactly and all 14 parent rows remain. Source geometry binds Group17/id13/typed12/Rough2/common zeros with six actual physics surfaces. Static 47 assert sites/6 for loops/4 Material owners are separate from actual 8-command QA/4 current bodies/0 failure-history-reuse and zero getter-body credit. Actual zero EQL Construction count skips positive EQL work while ordinary Blind precomputation remains. Active EQL/windows/control/ASHWAT, Native private TAR/optics, Source A/IDF/Legacy expansion and full-object/every-valid-combination parity remain unclaimed.

Direct Source epJSON retains the first comparator-equivalent member spelling
and last whole subtree, checked by two Native convert-only observations and
existing loader assertions. `Schedule:File` comparisons cover hourly and
20-minute data on February 29, distinct leap-day values, per-file daylight-saving
controls and the next-day handoff. That controlled weather copies February 28
meteorology; full schedule-domain completion remains open.

Within the bounded one-Zone A/B route, `Zone.multiplier` accepts the Native
signed owner's positive range, `1..=2147483647`. The actual getter, aggregate
demand, air balance, meters and reports retain the live factor; per-Zone
intensive state keeps its original meaning. Those two witnesses retain Group/list factors of one.
Two multiplier-2 models compare their complete original profiles: 15,144 values
have zero tolerance or RMSE failures, with exact Zone Information records,
dictionaries and clocks. This finite evidence retains the other model limits.

The opaque Source route accepts all four `BuildingSurface:Detailed` class
choices: Wall, Floor, Roof and Ceiling. Raw/typed Ceiling remains visible while
the actual getter assigns its Roof class. Surface/full tracing now exports
the retained current Source geometry after successful completion. Two full-profile
Ceiling comparisons pass the original policies; their EIO bodies and DXF geometry
agree within the explicitly recorded version-header and line-ending boundaries.

Explicit Source A also accepts multiple closed opaque Zones, each with its
implicit Space. Direct OtherEquipment can target each Zone with all thirteen
non-Water fuels or thermal-only None and genuine built-in or authored schedules.
Direct People supports all three sizing
methods with live count, sensible/latent/radiant gains and humidity consumers.
Source typed OtherEquipment names preserve authored whitespace with ASCII case
folding. A direct-epJSON public-output witness distinguishes "A", " A ",
and "A " under unchanged numeric policies. Constant's missing-type-limit
warning uses the input format's field title.
Positive ZoneLists expand People and OtherEquipment in authored member order
into real instances on each implicit Space. People uses its selected count or
area operand; unused fields retain schema-valid positive values. Its live design
occupants reach OtherEquipment per-person sizing before simulation. Public Count
retains its original tolerance even when nominal EIO uses scientific notation.
The four genuine System list load outputs are connected; their uncontrolled
physical witness is zero, while existing owner tests cover nonzero reductions,
reset, order and Zone factors.
Positive ZoneGroups now reach the bounded Source A route with implicit Spaces.
The real getter retains the referenced ZoneList identity, multiplier/default and
ordered diagnostics; checked signed products feed the existing Zone consumers.
Local gain rates, design occupants and sizing stay per physical Zone. Facility
meters apply each actual nominal owner's public Zone and list factors; ordered
System list reductions use Zone factors, then four genuine Group outputs apply
the Group factor once. One factor-2 case compares 408 series, 38,232 values and
195 public joins with unchanged limits. Public EIO retains Zone factors 1/1 and
list factors 2/2, 36 nominal owners, 175 records and 28 actual warmup days; whole
ERR, DXF and CTF checks pass. All 16 System streams and 1,536 samples are genuine
zeros. Passed existing owner tests separately establish nonzero reductions,
reset/order and actual ListNum versus Group ordinal. Private Native Group/gain
allocations and physical Source private Group/gain allocations are unobserved.
Group compositions with authored Spaces or nonempty SpaceLists, optional comfort,
Source B, Space heat balance, HVAC and arbitrary combinations remain unvalidated.
Distinct People and OtherEquipment instances in different implicit Spaces can
retain equal public report names with separate arena rows, Which bindings and
ReportIDs. One alias case keeps 391 requests but emits 406 variable series and
13 Hourly meter series: 39,288 values and 200 public joins pass the original
limits. Completed-file parsing preserves each actual ID; occurrence bindings
use public nominal Name/Zone order without sorting values. Whole EIO, ERR and
DXF comparisons remain strict. Other collision and input-value domains remain open.
Source A connects People, OtherEquipment and ElectricEquipment on bounded closed
opaque Zones with one whole-Zone Space each. Zone, Space, ordered ZoneList and
SpaceList targets retain distinct real owners, member order and singleton
prefixes. Empty SpaceLists keep zero-length bindings without gain objects,
object outputs or meters; structural schema checks always run. Positive explicit
Zone floor adjusts its sole Space before sizing: initial raw Zulu Space6 becomes
final12, with actual Count/schedule contributions feeding per-person designs.

The Electric case compares 968 series, 91,992 values and 773 public joins,
including 79 nominal owners and all three sizing methods, under unchanged
limits. Defaults, retaincase subcategories and ignored retained JSON metadata
reach genuine getters and consumers. The legal convective-fraction snap keeps
object Q-Lost separate from Space/Zone component totals and real generic heat
reports. Whole EIO, ERR, DXF, CTF and 30-day warmup checks pass. Source exports
actual enclosure floors; private gain allocations remain separate owner evidence.
The original high-load Native model and two earlier Source attempts remain
unaccepted; the accepted pair uses corrected Build104 and the sole retained
low-load Native reference. Four existing test executions are retained without
rerun or new test functions; actual formatting/lint and physical regression
validate the consumer corrections. There are 187 recorded Source builds,
165 with accepted pairs and 22 without accepted pairs, including 8 builds
without Source execution. B/legacy Electric,
Group/Space compositions, multiple/remainder Spaces, Space heat balance/HVAC
and arbitrary/full-object combinations remain unvalidated.

Explicit Source A and bounded Source B execute active DesignDays with sizing
off, either beside real weather or DD-only without a RunPeriod or EPW. Retained
A witnesses cover all eight humidity selectors, four dry-bulb and five solar
choices. Earlier rain wet-film, raw/effective dewpoint, pressure, DayType/DST and
TS1 evidence remains retained. Snow uses unity reflectance and zero solar in
snowy DDs; nontrivial snow-ground-solar change remains unobserved.

Two B inputs preserve the original 102 output rules, nine Day owners, two Site
solar rules and two separately declared Facility meter rules. NoLimit reports
120 DD hours; hard flow/capacity limits report 120 DD hours plus 24 weather hours.
All 119,856 values pass the original limits, complete ordered streams, whole EIO,
ERR/END, dictionaries, SHD and actual warmup checks. CTF coefficient reports, DXF
and tabular CSV are unrequested; paired absence is checked with zero coefficient
rows or climatic cells credited. Actual CTF surface physics remains compared.
Those two B cases retain six NoSun/NoWind surfaces; their positive Site solar
alone does not establish surface/HVAC coupling. A further NoLimit case changes
only ROOF and WALL Y0 to SunExposed/WindExposed. All 115 series and 54,480 values
pass the unchanged policy. Both engines retain all 15 accepted108 forcing streams
exactly, while all 16 exposed-face and 15 demand-or-supply series change. This
establishes joint exposure-to-surface/HVAC behavior for that declared case.
Whole EIO/SHD and five warmup reports agree. Unrequested coefficient, DXF and
tabular reports receive no credit. Separate Sun/Wind effects, private film/local
wind/HAT, arbitrary values/combinations, OA/latent control, sizing and legacy
active DDs remain outside this evidence. No capability or frozen CON-01 identity
is promoted.

A separate two-Zone opaque weather case observes cross-Zone geometric shading.
Ten SunExposed/NoWind walls and roofs surround two closed masses; two Ground
floors use twelve authored 18C monthly values. Each engine reports 37 partial
sunlit-fraction samples on the declared ZULU wall and roof. All 115 series and
11,040 values pass unchanged limits, with complete streams and Source JSON/CSV
joins. Whole requested reports, CTF, DXF, tables and six warmup days agree
within their recorded identity boundaries. This finite case does not establish
reciprocal shading, baseline thermal causality, positive shadow-shadow area or
arbitrary object combinations.

The opaque Source route now calculates exterior reflections. One retained
two-Zone case changes the Building solar selection and adds 30 named irradiance
requests. All 145 series and 13,920 values pass unchanged limits; all 30 new series
match exactly. Beam-to-diffuse and sky-diffuse obstruction irradiance have 84
and 185 paired positive samples; all 960 opaque specular samples per engine are
zero. Whole requested reports, geometry and six warmup days agree. Windows,
authored reflective shades, private Native hit ownership, baseline thermal
causality and arbitrary/full-object combinations remain unestablished.

The bounded inside-convection route now runs ASTMC1340 with actual uncapped
Zone system flow and exposes six live inside heat-transfer coefficients. One
Source B case uses five DesignDays and a Ground floor; all 121 series and
57,360 values pass, including both genuine Hourly meter streams. The six new
coefficient rules use explicit 1e-9 W/m2-K point and RMSE limits; all inherited
rules remain exact. Positive public node flow confirms activation. Public
averages do not instrument private Native convection-entry ACH or call history.
Adaptive, surface-local/user/EMS/Kiva and reached window children remain bounded.

Bounded Source A/B admits AdaptiveASH55 and AdaptiveCEN15251 for DesignDay-only
execution. Two B cases with five DesignDays compare 125 series and 59,280 values
for ASH55, and 130 series and 61,680 values for ASH55 plus CEN15251, under the
preserved policies. Their adaptive outputs and complete AdaptiveComfortSummary
tables use actual occupancy, the retained DesignDay mean and accumulated occupied
hours. Weather/STAT/EPW history, sizing, non-Mixing RoomAir, Pierce/KSU/CoolingEffectASH55
and arbitrary compositions remain unvalidated. These finite cases do not prove
all threshold regions or full-object support; public samples do not expose
private state or call histories. No capability or frozen conformance promotion
is implied.

The bounded Source B direct-epJSON Humidistat producer uses one real Zone/control, RH schedules and normal-mixing ThirdOrder, Analytical or Euler prediction. Analytical single50 with the omitted dehumidifying schedule and Euler crossed65/45 qualify under all161-series rules. ThirdOrder dual40/60 remains unaccepted: 14 new dehumidifying-report points exceed the unchanged1e-12 kgWater/s limit (maximum1.1494137e-12); all original155 streams and response gates pass.

Bounded Source B direct epJSON retains its established outdoor-air, hard-limit, DCV/economizer/recovery and non-Humidistat humidity evidence. On basic positive-OA paths, named or present exact-empty humidity choices still require both actual OA references and every existing prerequisite. Coherent authored OutdoorAir:Node singles and nameless OutdoorAir:NodeList objects reach the bounded ordinary no-AFN owner under the same full positive-OA prerequisites. Each single permits finite Native-effective height and omitted, exact-empty or owned supported DB, WB and wind schedule fields, including both named temperature schedules. On this ordinary no-AFN route, the curve-name field is any String; symmetry is exactly empty/No/Yes and angle is exactly empty/Absolute/Relative. Direct epJSON enum case is significant. Omitted and exact-empty fields retain the actual defaults and blank metadata. Actual A2 then A3 lookup, nullable pointers and used state determine IsLocal; positional AnyLocal alone does not. The genuine eight-alpha getter still marks the object used, fills all fields and preserves positional counts, counted default Height and AnyLocal effects; these do not create IsLocal without a temperature pointer. Ordinary weather handling does not resolve the WPC curve or write AFN pressure state. OutdoorAir:NodeList instance keys are input-order and used-object identities, never an A1 node name. Omitted or empty member arrays perform the real zero-alpha input fill; each nodes array member is an Object with a required String node_or_nodelist_name field. An exact-empty field value returns zero nodes, while whitespace remains a real name. All OA-list objects are filled before all singles in actual IP order. Every returned membership preserves the exact Native allocation/reference effects: newly assigned names begin at ref0 and references to existing names increment. OutsideAir registrations and advancing fluid streams are retained per membership; only the final outdoor-node registry deduplicates node numbers. Single registration still precedes Height and schedule writes. Success alone commits the final list/count and clears the consumed input flag. The selected IdealLoads member consumer uses the existing registry path without invented reference or registration effects. True InitCall preserves global/point weather, DB-before-WB/wind and the local outer WB>DB clamp. A present WB pointer selects the existing backend-aware W(Tdb,Twb,P) owner, including the combined route. With only DB present, global W is written BEFORE cached Twb and nested TsatPb/Psat; computed WB commits only on child success. Earlier weather/clamp writes, failure prefixes and later H/P/Quality retain their order. False local calls retain real W without schedule refresh or outer clamp; nonlocal nodes retain global W. The same live owners, cache, saved pressure and default interpolation=false remain through three cold callers/HB. Fixed inherited scenes and the selected coherent multi/list scene retain unchanged158-series policies, full original science, clocks, ancillary and independent nine-report OA/mixed response. The new scene preserves every inherited active OA climate field and all numeric values. A complete source-derived topology prediction, sealed before actors from the empty node seed and actual report cutoff, is compared against the whole BND body, including node references, ordered registrations, streams and conditional blocks. Complete general-NodeList and Scheduled-manager exports retain public name/order custody; neither export is private OA getter telemetry. Prediction changes cannot be fitted to observed output. Distinct DB/WB schedule names with equal declared values establish one normal boundary, not arbitrary order, private clamp or call-count evidence. Active AFN/Curve/Table objects, defaulted multizone AFN, duct-loss routes, EMS, Source A/IDF, crossed Humidistat and arbitrary/full-object compositions stay held. Direct epJSON OutdoorAir:Node singles also retain unknown body members of every JSON kind, including name and idf_* bookkeeping, while Native ignores them as getter operands. A1 remains the actual instance map key; only the eight declared body properties affect this getter. Their existing type, enum, numeric and reference checks remain strict. The fixed metadata child preserves all755 inherited numeric records and adds two ignored Float records, not two physical operands. Weather math, node topology, reports and the frozen comparison criteria remain unchanged. Legacy, Source A/IDF, AFN and all other getter guards retain their boundaries.

Actual remaining moisture demands reach IdealLoads Humidistat consumers. Only the absent-control six-report zero path qualifies under every161-series rule. The three authored-control scenes retain zero credit after16/4/6 point failures at the unchanged1e-12 kgWater/s limit; all RMSE and original155 checks pass. The finite-cooling scene also lacks its Winter humidification witness in both engines. Broader combinations, Source A/IDF, OA, EMS/faults, AFN/Space/sizing and full-object completion remain open.

A separate passive diagnostic aligns all5,573 API-visible prediction frames by full clock, phase and occurrence order; 15 additional Source kickoff frames have no Native callback. The first observable difference is upstream temperature at warmup frame4, one binary64 step. Near-cap demand differences straddle18C and have different temperature-derived cache tags; actual Native cache entries and the underlying correction cause remain unobserved. The complete ordered Native ESO/MTR profile matches the original CLI run after report-ID identity and program-header normalization. Published production Source is restored unchanged. This diagnostic adds no qualifying case or value and changes no policy.

The bounded Source no-outdoor-air owners retain heating-constant/cooling-capacity coupling and now implement constant sensible heat ratio (SHR) cooling. Eligibility permits named or omitted-default SHR (.7) with heating humidity explicitly None or ConstantSupplyHumidityRatio through the four cooling limit selectors, retaining all active numeric, topology, schedule and Legacy guards. Six prospectively frozen cases completed once under Source141; 6 meet every original numeric, export, clock, ancillary and science check plus their applicable public response witness. Qualifying cases: HEAT_CONST_COOL_NONE_CAPACITY_COUPLING, HEAT_CONST_COOL_NONE_FLOW_CAPACITY, HEAT_CONST_COOL_CONST_CAPACITY, HEAT_CONST_COOL_CONST_FLOW_CAPACITY, SHR_OMITTED_DEFAULTS_NO_LIMIT, SHR_NAMED_HEAT_CONST_FLOW_CAPACITY. The four prescribed heating-constant capacity/combined scenes include one designated Winter simultaneous heating/dehumidification witness; the two SHR scenes have their own Summer sensible/latent cooling witnesses. Those finite public interval responses imply no exact-quarter capacity, global SHR/humidity or private-mode law. Exact-empty humidity selectors now use the bounded Source B non-Humidistat NoOA/basic positive-OA paths described above; omitted defaults remain distinct. The four Source139 comparisons from Unit137 remain unaccepted. Broader HVAC Humidistat and OA/DCV/economizer/recovery compositions, sizing, remaining combinations and full-object conformance remain open.

93 bounded Source B Scheduled/NodeList pairs retain 6,888,240 values. The unused Source A pair adds59,520, giving 94 pairs and 6,947,760 values. Qualifying scenes retain eighteen Scheduled joins, exhaust/returns, ordered NodeList/UTF-8, weather/fuel and comfort under every applicable frozen policy. Finite-route, zero-boundary and current-occupancy evidence have separate scope; no private state, exact averaged capacity/mixing or full-object claim follows.

All three Zone air solution algorithms now reach bounded Source A/B consumers
with Space heat balance disabled. Four one-Zone Euler and Analytical cases
compare 360 series and 126,394 values under unchanged scalar limits, including
both controlled Detailed node-temperature arrays. Their complete public
intervals and Source JSON/CSV joins pass; shortened intervals are reporting
evidence. Private solver histories and broader combinations remain unestablished.

HVAC and remaining gain families remain outside this increment. Reciprocal interzone Wall–Wall
and Roof–Floor pairs accept normal opaque stacks of one to ten layers, including
the getter's Ceiling alias. Exact reverse Material
identity retains the Native fast path; differing counts or layer identities use
an actual Source nominal-U difference of at most 0.001. Equal-count mismatches
retain Native's warning sequence. Infrared-transparent constructions retain
their sole-layer exact-reverse branch on Wall pairs. Pairs require equal Zone
and list factors and opposite coincident vertices. Twenty-seven 24-hour two-Zone
cases cover 674,544 derived values with zero tolerance or RMSE failures;
the maximum absolute difference is 3.8198777474462986e-11. Actual Zone/Space and enclosure links,
EIO, diagnostics and DXF agree within their recorded boundaries. This separate
class inherits no one-Zone capabilities or conformance promotion. Multiple/remainder Spaces, Space heat balance, Source B authored Spaces,
Group compositions with authored Spaces or nonempty SpaceLists, Source B groups
and multi-Zone HVAC remain open, as do broader interzone combinations.
Source closure can complete collinear edge points in a temporary copy while
retaining authored Surface vertices and volume inputs. Open-zone fallbacks remain held.
Actual device and Zone W/J joins and Facility sums use their recorded public factors;
None contributes heat without a resource report or meter.

The reversed two-layer and unequal two/three-layer cases also match all emitted
CTF records. The latter reads real nominal-U array slots by Construction number.
The reversed two-layer case's original 30-day warmup allowance failed in Native;
a separate input changes only that allowance to 100 days and converges after 32,
with unchanged convergence tolerances. The original failed reference remains
unaccepted.

Paired Walls accept the legal Sun/Wind exposure enums, omission and empty defaults.
IDF and direct epJSON comparisons preserve their distinct warning labels and
surface order; non-external Sun warnings and paired Wind clearing agree with
Native. Those two exposure cases finish with both flags false; raw authored fields
retain their presence and values. The existing input table covers all sixteen
presence/value combinations without adding test functions.

The two-pair construction-warning case also checks repeated warning suppression.
Both real Construction order-warning flags and the invocation-local hint follow
Native's record order. Reusing the warned constructions skips the later pair's
exposure clearing, so its actual WindExposed flags remain set. Internal Rust
path bookkeeping is separate from the observed scientific owner fields.

`Material:InfraredTransparent` now reaches the ordered Source A/B getter and
single-layer CTF on Ground or Adiabatic boundaries, preserving the Native
warnings and Invalid roughness. Ground and Adiabatic B comparisons plus proper
reciprocal interzone A pairs cover 49,824 values with zero tolerance or RMSE
failures. Adiabatic values are raw-exact; Ground's maximum absolute difference
is 4.66e-10. Exterior roughness lookups, Materials reports and used-IRT
Constructions reports remain excluded. Multi-Zone Ground/Adiabatic IRT uses,
CondFD and broader object combinations remain open.

Reporting key matching uses the [Rust RE2 port](../../../crates/ep_re2/README.md)
at pre-scan, request construction and live registration. Eager OnePass analysis,
a persistent anchored-longest DFA and the original-context NFA fallback use
independently checked official-binary accounting. Component comparisons retain
exact wire results, including resource failures and cache resets. Physical A/B
regex cases cover 160 series and 15,144 values under the original policy.
Two further 24-hour cases with a 100,000-byte Zone name cover 192 values and
reproduce the failed-DFA diagnostics before successful matching. Their complete
dictionaries and ordered timestamps agree; full-file equality is not claimed.

`MINREPORTFREQUENCY` now reaches the shared variable/meter frequency parser
through real reporting owners. It preserves first-call ordering, raw input
spelling, abbreviation warnings and EIO records. Two same-input 24-hour cases
with `Hourly` and `hour` compare 192 values across one variable and three meters;
the maximum absolute difference is 8.66e-15 with zero tolerance or RMSE failures.
Their dictionaries, clocks, override records and frequency diagnostics agree.
Other frequency overrides have source and existing-table coverage; this physical
comparison covers Hourly only. Unbound producers, unsupported output backends
and general legal-value completion remain open.

The current compatibility-mode arbitrary runtime covers the official
`1ZoneUncontrolled` heat-balance path and the declared direct-Zone no-OA
sensible PurchasedAir binding. The completed porting route is selected with
`--porting-scope A|B` and writes its own ESO/MTR and JSON/CSV results.

**Automatic oracle comparison is not connected to this Source route.**
Adding `--oracle-baseline` or `--compare-oracle` retains the Rust output but
returns `SourceReportedOracleComparisonUnsupported` (exit 4). Run the two
engines separately and use the fixed-case checker described in
[Verify the port](../guides/verify-porting.md).

Ad-hoc and launcher runs keep `conformance_claim=false`. Unsupported inputs
are blocked; explicitly allowed diagnostic partial runs remain diagnostic.
See [Run an IDF](../guides/run-arbitrary-idf.md) for artifacts and exit codes,
and [Roadmap](roadmap.md) for the remaining user-facing work.

## Current Conformance Claims

Current conformance evidence is limited to promoted manifests and declared
variables/meters. The exact case list is generated in
`docs/src/generated/conformance-case-index.md`. Algorithm and coverage boundaries
are generated from `specs/algorithm_ledger.toml`, `specs/object_coverage.toml`,
and `specs/variable_coverage.toml`.

- official `1ZoneUncontrolled` dynamic source-order compatibility rows are
  declared-variable compatibility only.
- IdealLoads no-OA/no-limit sensible, numeric capacity limit, numeric flow
  limit, flow-and-capacity limit, ConstantSensibleHeatRatio,
  ConstantSupplyHumidityRatio, selected Humidistat, selected outdoor-air,
  economizer, heat-recovery and reporting evidence remain case-scoped;
  they do not establish full IdealLoads compatibility.

{{#include ../generated/current-status-classification.md}}

{{#include ../generated/variable-coverage.md:current-status-variable-summary}}

Diagnostic-only evidence remains useful for source-order porting but does not
create compatibility claims. Historical diagnostic selectors and replay
wrappers are tracked in `specs/diagnostic_probe_ledger.toml`, outside the
default test suite. They are not additional finite-port production evidence.

## Not Claimed

General EnergyPlus replacement, broad heat-balance compatibility,
broad HVAC/node/meter compatibility, Plant, sizing, EMS, PythonPlugin, AFN,
fenestration, infiltration and broad ExampleFiles coverage are not claimed.
There is no broad IdealLoads branch coverage beyond the declared cases.
IdealLoads outdoor-air, economizer, heat-recovery and broader humidity branches
remain outside arbitrary-run compatibility even when individual release cases
have separate evidence.

Routine/domain readiness is owned by the algorithm ledger and project contract;
input support is owned by the capability registry. A finite unit pass, parser
support or successful demo does not promote those registries. The
[project contract](project-contract.md) defines the promotion requirements.
