## Change Type

- [ ] Refactor only
- [ ] Documentation cleanup
- [ ] Spec/schema change
- [ ] Input interpretation
- [ ] Runtime algorithm port
- [ ] Diagnostic/reporting
- [ ] Conformance claim
- [ ] Performance work

## Algorithm Port Ticket

- [ ] Not an algorithm/source-order change
- [ ] Compatibility port ticket completed
- [ ] Diagnostic probe only; no conformance claim
- Ticket path or PR section:
- Algorithm ID:
- Port type: compatibility / diagnostic_probe / refactor_only
- EnergyPlus version: 26.1.0
- EnergyPlus source file:
- EnergyPlus routine:
- EnergyPlus source-order stage:
- Rust target module:
- Rust target function:
- ExecutionStageKind:
- Compatibility path: true / false
- Diagnostic probe used: true / false
- Read state:
- Write state:
- History/state ownership:
- Unsupported state:
- Affected variables:
- Affected meters:
- Diagnostic-only variables:
- First target case:
- Proof variables:
- Tolerance candidate:
- Report path:
- Blocking gate:
- Conformance claim: yes / no
- Not-claimed branches:
- Partial run allowed: yes / no

## Claim Boundary

- Conformance claim: yes / no
- If yes, case IDs:
- Variables/meters:
- Tolerance:
- Report path:
- Blocking gate:

## Evidence Level

- [ ] Smoke
- [ ] Baseline-only
- [ ] Diagnostic
- [ ] Conformance
- [ ] Regression
- [ ] Performance

## Guardrails

- [ ] No new panic/unwrap/expect/todo
- [ ] No diagnostic result used as conformance
- [ ] No broad compatibility claim
- [ ] Specs updated if needed
- [ ] Generated docs updated if needed
- [ ] Reports regenerated if needed