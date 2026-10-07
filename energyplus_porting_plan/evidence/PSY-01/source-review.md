# PSY-01 source closure

The locked source is EnergyPlus 26.1.0 commit
`6f2e40d10250a105b49966baa24d843711e61048`. Exact file and line-range
hashes, immutable contracts, and the six observer insertion patches are in
`contracts/PSY-01-source.json`. The executable includes the complete original
header; removing the documented observer insertions restores its original bytes.
No property formula, conditional, cache assignment, or mathematical helper is
rewritten as the reference.

Selected inline bodies are `PsyRhoAirFnPbTdbW` state overload 512–547 and
constexpr overload 549–574, its fast variant 576–591, `PsyHFnTdbW` 648–666
and fast 668–677, `PsyCpAirFnW` 679–716 and fast 718–741, and
`PsyTdbFnHW` 743–762 in `Psychrometrics.hh`. The sole induced integration helper
is original `PsyWFnTdbH` 962–998, added to keep existing H-to-W consumers compatible
with canonical H's original J-based evaluation grouping. This does not change the
named four-function card scope or promote PSY-02. Direct mathematical dependencies
are `Real64` (typedef double), `Constant::Kelvin` (273.15), and the original
ObjexxFCL double `max` (`a < b ? b : a`). Density uses 287.0 and 1.6077687;
enthalpy, heat capacity and inverse temperature use 1004.84, 2500940.0, 1858.95.
No saturation or relative-humidity helper is part of this card.

The induced W helper computes `(H - 1004.84*T)/(2500940 + 1858.95*T)` and
returns 1e-5 only when that computed W is negative. Exact zero, positive subfloor
W and NaN remain unchanged; this is not an unconditional humidity floor. Its
optional warning/statistics aggregation is excluded, leaving no mutable state.
`PurchasedAirManager.cc` source boundaries pin the positive-flow sensible cooling
capacity H/T consumer at 2191–2204, ConstantSensibleHeatRatio H/W consumer at
2213–2227, dehumidifying capacity control branches 2263–2306, and post-saturation
W/H consumer at 2313–2325. Those existing CSHR/dehumidification consumers explain
the compatibility correction; they remain outside CON-01 B's None controls.
The upstream saturation calls at 2314/2316 are explicitly not certified here.

Normal functions apply the 1e-5 humidity floor at their original locations.
The `max` helper preserves NaN, while negative and zero W select the floor.
Fast functions require already adjusted W>=1e-5 and retain their assertions.
`api/func.cc` 165–199 dispatches density and enthalpy to fast functions but Cp
and inverse temperature to normal functions. Using that API without accounting
for its fast precondition would fail to test the card's normal subfloor branches.

Normal and fast Cp each own independent static `(dwSave,cpaSave)` state, initially
(-100,-100). They test raw W equality before flooring. A cold first W=-100 returns
the sentinel -100; the same W after another input computes the floor Cp and then
can hit that computed cache. Equal signed zeros hit; NaN cannot hit itself.
The reference captures actual cache states before the condition and before each
hit/miss return without mutating them. Sequence IDs don't reset state; only a
fresh process does. The bounded production mapping is one execution thread with
independent safe thread-local normal/fast cache owners. Original cross-thread
static data races are outside this card.

The whole-header C++ probe compiled and linked with portable clang++ using source,
third-party, ObjexxFCL, fmt and valijson include paths. Genuine native state is
created/deleted by the pinned original DLL's C exports; the diagnostics-disabled
inline density functions don't inspect state fields, avoiding C++ structure-layout
interop. The receipt records the DLL hash and actual compile dependency hashes.
File I/O, `EP_psych_errors` warnings and `EP_psych_stats` aggregation remain
explicitly excluded.

The frozen unit contract has 394 ordered calls in two processes: normal physical
inputs, dry/moist states, zero-temperature neighbors, humidity floor neighbors,
pressure changes, own-result h-to-T chains, independent inverse enthalpy, cache
sequences, separately identified IEEE helper-domain probes, and 31 induced H-to-W
consumer/guard probes. Per-output units,
atol and rtol are fixed before any Rust comparison. IEEE probes do not expand the
CON-01 admitted A/B input scope.

Production replay accepts actual recorded Rust tuple/result/cache bits and retains
caller, order and pipeline-phase evidence. It reports omissions explicitly and
cannot substitute a helper rerun for observed production results. No downstream
zone/HVAC numerical or card gate is promoted by this source review.

The production v2 protocol stores exact event dictionaries and ordered u32 indices.
The original body executes for every index in order, including its own cache-hit
branch. Only storage is deduplicated; the reference key includes source input ID,
computed result bits and actual Cp before/after/hit fields, so an input ID may
produce multiple original-state variants. Comparison checks every ordered input
identity and weights unique actual/reference pair errors and RMSE by actual repeat
counts. Opaque real Rust zone/system invocation context is preserved without
claiming it is an EnergyPlus stage/timeline. The reference-only compression check
replays the 394 original unit results and a seven-event repeated-input sequence
with five distinct original cache-state variants; this is protocol verification,
not Rust numerical or production gate evidence. The original six header observers
and v1 unit contracts remain unchanged.
