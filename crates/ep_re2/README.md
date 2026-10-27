# Rust reporting regex port

This Rust port supplies constructor/status and boolean FullMatch to the Source
reporting pre-scan, request getter and registration. Two key-only derived A/B
examples verify 160 series and 15,144 reported values under the original policy,
with zero point or RMSE failures. Full legal-domain completion remains open.

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

Two table functions compare 1,494 actual parse records, 100 rewrite records,
23 selected complete program records and 10,607 constructor/match rows. Adjacent
rows reuse one immutable expression: 1,505 constructions still check every row's
ok/code/error/error-argument/match fields. Separately retained complete comparisons
cover2,333 prefix/Simplify patterns each,2,352 programs and10,607 finalized-list
matches, all zero-difference. The existing constructor table also checks 15 actual
default-log rows, including byte truncation, split UTF8, NUL and embedded newlines.
Heavy allocation probes stay outside default tests.

Official budget models intentionally stop at a later All-No fatal after prescan.
They establish checked raw-constructor admission, never successful simulation,
physical conformance, private sizeof or separately observed CI-constructor ok.
Fixtures and observation/source provenance are under tests/data.

```text
cargo test -p ep_re2 --locked
cargo clippy -p ep_re2 --all-targets --locked -- -D warnings
```

Pre-scan constructs raw then CI, including duplicates, and checks only raw status.
The requested-variable getter constructs CI only. Literal/wildcard checks,
short-circuit matching, Used-before-dedup and genuine utility error effects retain
source order. Quiet APIs serve component tests; reporting uses explicit default
logging with observed Windows build-origin metadata. A separate default-options
source observer agrees on all 16 API records and 4,906 stderr bytes, including a
post-parse compile failure. These are component observations, separate from the
two successful physical pairs.

Valid-match OnePass/DFA cache accounting and resource stderr remain unported;
the anchored NFA supplies boolean matching. Minimum-report-frequency overrides,
excluded positive pre-scan families, unbound producers and output backends retain
their existing boundaries. This is finite reporting evidence within Source A/B.
External Options, capture extraction, unanchored/reverse search and CompileSet are
outside this reporting API. Finite comparisons do not prove full legal domains.

Reference observers compile with the pinned CMake RE2_SOURCES22-unit list.
Constructor input is pattern_hex<TAB>text_hex; parse/program input is pattern hex
per line. The phase observer also takes required_prefix or simplify. All use
explicit byte lengths.
