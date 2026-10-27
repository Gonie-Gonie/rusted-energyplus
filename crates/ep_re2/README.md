# Rust reporting regex port

This is an **unconnected, in-progress** Rust port of the RE2 constructor and
boolean `FullMatch` used by EnergyPlus reporting. Source reporting still blocks
regex keys. This crate adds no supported object or physical conformance case.

The primary source is EnergyPlus 26.1.0 commit
`6f2e40d10250a105b49966baa24d843711e61048`, `third_party/RE2`.
`LICENSE.RE2` retains its license. Production code uses Rust and the standard
library; C++ files under `tests/reference/` are test-only reference observers.

The default UTF8/LikePerl parser retains the Native AST: operation kinds, full
flags, class-builder versus finished-class stage, simple caches, capture-name
identity, ordered children and the three alternation-factoring rounds. An arena
and explicit stacks handle user depth without recursive ownership. Unicode14
groups/folding come from the pinned source, independent of host Unicode data.
The boolean matcher preserves byte operations, surrogate rune encodings,
Native broad-class noncanonical-byte acceptance, ASCII word boundaries and
newline assertions. Captures are parsed; reporting requests boolean membership.

Two table tests keep the comparison load small:

- 1,494 actual Native parse records compare every serialized field and ordered
  sharing identity: 1,445 successful ASTs, 49 parse errors and 3,597 reachable
  nodes. The reference observer does not simplify or construct a RE2 matcher.
- 10,175 actual Native constructor/match observations compare `ok`, error code,
  error bytes, error-argument bytes and full-match result. The seven added
  regressions close all five previously recorded AST-order/class-union
  differences, including `a|(?i:a)` against `A` and permissive broad UTF8 paths.

Fixtures and observation provenance live under `tests/data/`. These finite
comparisons establish regression coverage, not completion of every legal input.

```text
cargo test -p ep_re2 --locked
cargo clippy -p ep_re2 --all-targets --locked -- -D warnings
```

Required work before reporting admission:

1. Complete and compare Native simplification, required-prefix selection and
   exact constructor/program-memory accounting. The current boolean lowering
   does not establish Native program-graph or allocation identity.
2. Connect the pre-scan, request-construction and registration owners, retaining
   their different constructor, diagnostic and write order.
3. Verify those connections with same-input unit and physical EnergyPlus runs.

External RE2 Options and capture extraction are outside the reporting API target.
To reproduce reference observations, compile the relevant C++ observer with the
pinned CMake `RE2_SOURCES` list of 22 translation units. The constructor observer
accepts `pattern_hex<TAB>text_hex`; the parse observer accepts one pattern hex
per line. Both use explicit byte lengths. The constructor observer's Quiet option
changes logging only. The recorded GCC13.2 source-unit build is separate from
the pinned EnergyPlus DLL and physical oracle comparisons.
