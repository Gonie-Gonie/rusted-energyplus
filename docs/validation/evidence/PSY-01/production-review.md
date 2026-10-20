# PSY-01 production boundary

The selected normal kernels are `PsyRhoAirFnPbTdbW`, `PsyCpAirFnW`,
`PsyHFnTdbW` and `PsyTdbFnHW`. The existing runtime calls their Rust
implementations; C++ reference results are comparison outputs only. The tuple
example accepts arguments and context, never expected results. It is a test
entrypoint, separate from the ordinary `eplus-rs run` production entrypoint.

`ideal_loads/calc/psychrometrics.rs` delegates production enthalpy to the
canonical normal H kernel. This preserves the source humidity floor and J/kg
operation grouping. Its existing ConstantSensibleHeatRatio consumer also needs
the canonical `PsyWFnTdbH` grouping to keep strict CP378 numerical reconciliation
valid. This induced helper correction is covered by additional original-header
unit cases. It does not complete PSY-02 or admit humidity-control branches into
the CON-01 B scope.

The capacity/Both cooling consumer in `ideal_loads/calc/no_oa.rs` computes the
source enthalpy difference, tests the inclusive capacity guard, adjusts H by
capacity divided by mass flow, and uses the returned normal `PsyTdbFnHW`
temperature as the actual supply temperature before the mixed-air upper limit.
The original capacity-branch Cp call retains its observable saved-state effect.
The guard-false, equality and guard-true test verifies the real producer and
consumption of the returned temperature. A CP343 diagnostic snapshot alone
does not prove consumption by the physical calculation.

The regression diagnosis separated 150 strict CP378 failures from four fixture
failures. At T=22 C and W=0.02, the legacy regrouped H was
72943.21800000001 J/kg and canonical H is 72943.218 J/kg. In the tiny-capacity
fixture the source H-to-T chain consequently changes its temperature relative
to the mixed-air upper limit, while the old delta-T shortcut stays at 22 C.
The resulting saturation input differs and violates the existing bitwise
humidity reconciliation. Both the source capacity consumer and canonical
humidity inverse are repaired; the strict CP378 reconciliation remains intact.
The other fixtures are corrected to require canonical numerical H while
retaining evidence-corruption isolation, and to use inputs that overflow the
original H grouping while retaining their infinity assertions.

After those production corrections, 25 remaining failures were branch-fixture
coverage failures. The original C++ H-to-T chain was independently executed
before changing those fixtures. At W=0.02 it returns a temperature just below
22 C; at W=0.024 it returns a temperature just above 22 C, which the original
mixed-air minimum clamps to exactly 22 C. Changing 29 tiny-capacity fixture
humidity literals across 20 test files restores their intended zero-sensible,
latent-capacity equality branch. The existing assertions, route counters,
state checks and corruption checks remain; CP402 adds an explicit bitwise
latent-output/capacity equality assertion. Reversing those literal changes and
the additive assertion restores the 20 files to their previous contents.
The runtime library then passed all 2,908 tests, and the complete workspace
all-target test command also passed before the implementation checkpoint.
The preserved failed logs and original-header fixture probe remain diagnostic
evidence, rather than successful gate receipts.

The optional production collector records actual arguments, already computed
results, caller locations and Cp state. It supplies no arguments or numerical
results to the calculation. Compact dictionary entries retain exact IEEE bits,
context and cache state; their ordered IDs retain every recorded invocation.
Limits and omitted counts remain explicit. Replaying a truncated prefix cannot
certify the omitted part of a run. The original C++ body must execute for every
replayed invocation, including repeated cache hits.

The guarded density and Cp compatibility wrappers normalize humidity before
calling the canonical kernel. Their trace arguments are those actual kernel
arguments. These conditional kernel comparisons do not prove parity of the
upstream EnergyPlus caller's unnormalized arguments or invalid-input handling.
The source cache mapping covers one bounded execution thread and preserves
same-thread environment history; original cross-thread static data races are
outside the card.

The existing pipeline performs many snapshot/release validation recomputations.
They are actual observed calls and remain identifiable by caller and order.
They must be distinguished from the physical producers when assessing coverage.
The final report records the executed commits, complete-run counts, time
contexts, physical consumers, comparison errors and Full/Summary equality.

This card's helper replay does not certify ZON-02/HVAC-04/05 assembly, the CTF
implementation, adaptive system timesteps, warmup, or SYS source call order.
Those card gates remain separate obligations in the work plan.
