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

For source-order work, prefer the dedicated compatibility or diagnostic probe PR template in `.github/PULL_REQUEST_TEMPLATE/`. The PR gate classifies the base-to-head diff and rejects sensitive changes without a completed ticket; the non-algorithm checkbox cannot override that classification. Embed every field below even when linking to another ticket, and split source-order work so each PR carries one Algorithm ID.

- [ ] Not an algorithm/source-order change
- [ ] Compatibility port ticket completed
- [ ] Diagnostic probe only; no conformance claim
- Ticket path or PR section:
- Algorithm ID:
- Domain:
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
- Inactive branches:
- Unsupported active branches:
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

- If yes, case IDs:
- Variables/meters:
- Boundary tolerance notes:
- Additional boundary notes:

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
