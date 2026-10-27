# Rust reporting regex port

This **unconnected, in-progress** Rust port supplies the constructor/status and
boolean FullMatch used by reporting. Source reporting still blocks regex keys.
It adds no supported object or physical conformance case.

Primary: EnergyPlus26.1.0 commit6f2e40d10250a105b49966baa24d843711e61048,
third_party/RE2. LICENSE.RE2 retains its license. Production uses Rust and std;
C++ files under tests/reference are test-only observers.

Native AST kinds, flags, class construction stage, caches, capture-name identity,
ordered factoring, Coalesce/Simplify and required-prefix selection are retained.
Unicode14 groups/folding use pinned tables. Compilation preserves allocation
order, patch lists, UTF8 suffix caches/tries, nullable repetition, bounded anchor
extraction, Optimize, ordered Flatten, list hints and bytemap batches. The old
semantic IR/compiler and separate byte-class matcher are removed. Execution uses
Native finalized lists, raw bytes and the original assertion context after prefix
removal. Captures are parsed; reporting requests no extracted captures.

Default forward cap698,995 comes from the pinned official binary's adjacent
checked-constructor boundary, with the exact2xcap walk budget. The separate GCC
unit cap is698,996; its observed Prog432-byte size is NOT the official DLL layout.
Runtime supplies no guessed C++ size or DFA metadata. Explicit-layout metadata is
test-only; both paths share graph passes. Boolean execution uses an anchored NFA,
without unanchored prefix caches, OnePass or DFA caches.

Two table functions compare1,494 actual parse records,100 rewrite records,
23 selected complete program records and10,607 constructor/match rows. Adjacent
rows reuse one immutable expression:1,505 constructions still check every row's
ok/code/error/error-argument/match fields. Separately retained complete comparisons
cover2,333 prefix/Simplify patterns each,2,352 programs and10,607 finalized-list
matches, all zero-difference. Heavy allocation probes stay outside default tests.

Official budget models intentionally stop at a later All-No fatal after prescan.
They establish checked raw-constructor admission, never successful simulation,
physical conformance, private sizeof or separately observed CI-constructor ok.
Fixtures and observation/source provenance are under tests/data.

```text
cargo test -p ep_re2 --locked
cargo clippy -p ep_re2 --all-targets --locked -- -D warnings
```

Before reporting admission, connect pre-scan, request construction and registration
with original writes/diagnostic order, including default RE2 stderr effects, then
verify same-input unit and physical runs. The C++ constructor observer uses Quiet
for logging only; status/match agreement does not establish production logging.
External Options, capture extraction, unanchored/reverse search and CompileSet are
outside this reporting API. Finite comparisons do not prove full legal domains.

Reference observers compile with the pinned CMake RE2_SOURCES22-unit list.
Constructor input is pattern_hex<TAB>text_hex; parse/program input is pattern hex
per line. The phase observer also takes required_prefix or simplify. All use
explicit byte lengths.
