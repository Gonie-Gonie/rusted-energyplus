---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-05
---

# Performance Optimization Boundaries

Optimization is allowed only within the compatibility contract.

Allowed optimization areas:

- memory layout
- cache locality
- scheduling and execution planning
- deterministic tracing
- parser and compiler structure
- algorithm-preserving numerical implementation

Not allowed:

- replacing EnergyPlus physical algorithms without evidence
- changing numerical behavior without declared comparison and tolerance policy
- using performance results as a substitute for conformance evidence

