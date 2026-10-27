# Rust reporting regex port

This is an **unconnected, in-progress** Rust port of the RE2 constructor and
boolean `FullMatch` used by EnergyPlus reporting. The Source reporting paths
still reject regex keys. This crate adds no supported object, physical case,
or conformance claim.

The primary source is EnergyPlus 26.1.0 commit
`6f2e40d10250a105b49966baa24d843711e61048`, `third_party/RE2`.
`LICENSE.RE2` retains its license. Production code uses Rust and the standard
library; the C++ file under `tests/reference/` is a test-only reference observer.

The implementation includes an explicit-stack default UTF8/LikePerl parser,
pinned Unicode14 groups/folding, byte-range compilation and a byte-position NFA.
It preserves byte operations such as `\C`, surrogate rune encodings, and the
primary compiler's deliberate noncanonical-byte acceptance for broad classes.
Assertions use Native ASCII word bytes and newline rules. Captures are parsed;
these callers request only boolean membership.

The single table test compares 10,168 actual pinned-source constructor/match
observations: `ok`, error code, error bytes, error-argument bytes and full-match
result. The fixture includes normal/error grammar, Unicode properties and fold
boundaries, NUL/noncanonical text bytes, empty cycles, counted repetition and
deep groups. Its provenance is in `tests/data/provenance.json`.

```text
cargo test -p ep_re2 --locked
cargo clippy -p ep_re2 --all-targets --locked -- -D warnings
```

Required work before reporting admission:

1. Preserve Native AST kinds, flags, class-construction stage and simplification
   order. Native alternation/prefix factoring can change byte-range compilation.
   Empty classes must survive until the original simplification phase. The
   current semantic parser does not preserve all these choices.
2. Preserve required-prefix selection and exact constructor/program-memory
   accounting, including original compile-failure status behavior.
3. Connect the pre-scan, request-construction and registration owners, retaining
   their different constructor and diagnostic/write order.
4. Verify those connections with same-input unit and physical EnergyPlus runs.

The fixture is a finite unit comparison, not proof of those unfinished gates.
Additional AST probes already show an order-dependent difference: `a|(?i:a)`
against `A` is false in Native and true in the current semantic parser. Class
union/prefix probes also differ on noncanonical bytes. These observations are
retained in the provenance; they must be closed by the Native AST implementation.
External RE2 Options and capture extraction are outside the reporting API target.

To reproduce a reference observation, compile `tests/reference/re2_unit.cpp`
with the pinned CMake `RE2_SOURCES` list (22 translation units). It accepts
`pattern_hex<TAB>text_hex` and returns the five actual API fields above. Its
Quiet constructor changes logging only. The recorded reference used GCC13.2;
it is a source-derived unit build, separate from the pinned EnergyPlus DLL.
