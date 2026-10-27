# Rust reporting regex port

This Rust port supplies constructor/status and boolean FullMatch to the Source
reporting pre-scan, request getter and registration. Two key-only derived A/B
examples verify 160 series and 15,144 reported values under the original policy,
with zero point or RMSE failures. Two further full-day comparisons retain a
100,000-byte Zone name and verify 192 values plus actual failed-DFA diagnostics
and fallback output. Full legal-domain completion remains open.

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
The official DLL's independent allocation/subtraction chain establishes its
Prog440/Inst8/DFA208/State16/atomic-pointer8/int4 accounting. The separate GCC
fixture keeps its own explicit Prog432/DFA200 scalars. Both paths share graph
passes. The default constructor eagerly retains OnePass analysis, nodes and
actual charge on the immutable compiled program. Its proven header4/cap gate
remains unchanged; lazy DFA accounting subtracts the actual flattened storage,
optional list heads and successful OnePass charge from the default allowance.
Boolean FullMatch uses a persistent forward anchored-longest DFA, with the
original text assertion context after prefix removal. Successful OnePass analysis
skips that cache for original texts of at most16 bytes. On budget/reset/slow
failure, logged matching emits the pinned line811 payload before the existing
NFA boolean fallback; quiet matching takes the same fallback without LOG.

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
two Build72 physical pairs.

Cloned expressions share serialized owned DFA cache state; immutable expression
equality does not inspect that state. Quiet and logged matching use the same
cache and fallback path, with diagnostics emitted after unlocking the cache.
Host allocation/internal invariant guards remain a separate fatal port/host
boundary, with no claim about Native exception or exit-code equivalence.
Minimum-report-frequency overrides, excluded positive pre-scan families,
unbound producers and output backends retain their existing boundaries. This is
finite reporting evidence within Source A/B.
External Options, capture extraction, unanchored/reverse search and CompileSet are
outside this reporting API. Finite comparisons do not prove full legal domains.

Reference observers compile with the pinned CMake RE2_SOURCES22-unit list.
Constructor input is pattern_hex<TAB>text_hex; parse/program input is pattern hex
per line. The phase observer also takes required_prefix or simplify. All use
explicit byte lengths.

Test-only exact-scalar accounting retains the same OnePass traversal and the
matched longest-DFA component.
The existing Native AST table checks 20 ordinary groups (861 ordered queries)
against actual public Native records, using explicit same-GCC DWARF layout data.
The same table also checks eager default-constructor OnePass flags against those
actual Native results; every selected program is within the proven small-size
gate. The same ordinary groups exercise clone persistence, equality after
matching and quiet/logged owner parity. An external current-source reread of all
2,362 groups agrees on every public field, complete wire bytes and stderr, with
no Native replay. The cfg(test) exact-scalar wire and reset tracer retain the same-GCC
component scope; they do not establish the independently measured official DLL
layout, a new physical case or a full-domain claim. Reverse/unanchored/capture
execution and prefix acceleration remain outside this boolean reporting API.
