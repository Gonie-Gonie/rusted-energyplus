# Compatibility Algorithm Port

Use this template when the PR ports an EnergyPlus source routine into the compatibility runtime path.

## Change Type

- [x] Runtime algorithm port
- [ ] Diagnostic/reporting
- [ ] Conformance claim
- [ ] Refactor only

## Algorithm Port Ticket

- [ ] Not an algorithm/source-order change
- [x] Compatibility port ticket completed
- [ ] Diagnostic probe only; no conformance claim
- Ticket path or PR section:
- Algorithm ID:
- Port type: compatibility
- EnergyPlus version: 26.1.0
- EnergyPlus source file:
- EnergyPlus routine:
- EnergyPlus source-order stage:
- Rust target module:
- Rust target function:
- ExecutionStageKind:
- Compatibility path: true
- Diagnostic probe used: false
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

## Guardrails

- [ ] No diagnostic result used as conformance
- [ ] No broad compatibility claim
- [ ] Source map and algorithm ledger updated
- [ ] Blocking gate added or updated