# Diagnostic Probe

Use this template when the PR adds instrumentation or diagnostic-only probes without promoting compatibility evidence.

## Change Type

- [ ] Runtime algorithm port
- [x] Diagnostic/reporting
- [ ] Conformance claim
- [ ] Refactor only

## Algorithm Port Ticket

- [ ] Not an algorithm/source-order change
- [ ] Compatibility port ticket completed
- [x] Diagnostic probe only; no conformance claim
- Ticket path or PR section:
- Algorithm ID:
- Port type: diagnostic_probe
- EnergyPlus version: 26.1.0
- EnergyPlus source file:
- EnergyPlus routine:
- EnergyPlus source-order stage:
- Rust target module:
- Rust target function:
- ExecutionStageKind:
- Compatibility path: false
- Diagnostic probe used: true
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
- Conformance claim: no
- Not-claimed branches:
- Partial run allowed: yes / no

## Guardrails

- [ ] Diagnostic output remains outside conformance manifests
- [ ] No broad compatibility claim
- [ ] Report text marks diagnostic-only evidence
- [ ] Blocking diagnostic gate added or updated