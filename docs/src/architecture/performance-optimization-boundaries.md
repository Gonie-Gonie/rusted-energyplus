---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Performance Optimization Boundaries

Optimization is allowed only within the compatibility contract.

The detailed policy is in
[Performance, Stability, and Core Porting Philosophy](performance-stability-core-porting-philosophy.md).
This page is the short operational boundary used during reviews.

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

Performance measurements must name fixed inputs, stage timing, and artifact
paths. They may be included in release evidence documents for engineering
tracking, but they do not prove EnergyPlus compatibility.
