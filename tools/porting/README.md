# Porting Reference and Verification Tools

Use the [verification guide](../../docs/src/guides/verify-porting.md) to run the
same fixed IDF through EnergyPlus and Rust, then compare their physical streams.
Inputs, frozen policies, accepted results and historical recovery details are
in [validation assets](../../docs/validation/README.md). Development test
selection is documented in [Developer Workflow](../../docs/src/guides/developer-workflow.md).

## Current Verification

- `check_completed_reported_streams.py`: validate successful paired receipts,
  fixed inputs, full reported dictionaries, timestamps and the unchanged
  per-variable tolerance policy. It launches neither engine.
- `source_input_example.py`: prepare deterministic synthetic weather and compare
  the separately identified composed B102 example using the unchanged physical
  stream kernel and original B policy; see the verification guide for recorded
  build/run commands. The checker launches neither engine.
- `prepare_con01.py --check`: verify the frozen input/metadata/contract bytes
  without regenerating them.
- `record_command.py`: execute an explicit command and record its exit,
  arguments, source snapshot and output. A receipt alone is not numerical proof.
- `freeze_rust_build.py`: bind a recorded build to its actual Cargo artifacts.
- `check_sur02_units.py`: the isolated history unit experiment for its pinned
  Source59 checkout, outside the default suite.

The remaining Native adapters and observers are focused reference tools for
specific source ranges. Their source pins, original receipts and required local
build artifacts remain explicit prerequisites. Reproducing a historical result
requires its tested checkout; do not substitute new source bytes into old
receipts. Retained JSON references use their original logical paths, resolved
through `validation_paths.py` to the current validation directory.

Use a fresh output folder under `.runtime/porting/<area>/` for an affected check.
Keep the accepted result and the command needed to reproduce it, rather than
accumulating task-card progress logs or rerunning unchanged annual cohorts.

## Frozen paired release

`paired/v1` runs the reviewed basic positive-OA profile from data-only
`paired.CaseConfig.v1` and `paired.SourceAuthority.v1` records. Invoke the file
directly so no parent package initializer runs:

```text
python -X utf8 -B tools/porting/paired/v1/__main__.py validate --bindings PATH#SHA256 --case ID --mode fresh
```

The external binding pins the noncircular `release.json`, every local Python
module, the case, Source authority and independent reviews. Normalize actual
Source evidence with `authority --release PATH#SHA256 --evidence PATH#SHA256
--mode fresh --output PATH`. Then use `act` for Native, `precheck`, `act` for
Source with that proof, `verify`, and `complete`. Outputs must be fresh paths;
Source launch requires a finite Native structural proof. Numerical comparisons
finish before all twelve engine gates, eight paired gates and two full exports.

Record `precheck` and `verify` with the trusted `record_command.py`, declaring
every Context pin plus the Native launch (and Source launch for `verify`) as
explicit inputs. Source `act` also requires `--native-precheck-receipt PATH#SHA256`.
`complete` requires `--verification-receipt PATH#SHA256`; the command, full input
custody and captured output pin must match the actual precheck or readback.
Shaped JSON without recorded execution cannot authorize Source or gain credit.

`--mode retained` validates explicit historical archives and original custody,
executes no actors and grants no new physical credit. Only Units158/159 have
full migration adapters; Unit155 is ledger/shared-kernel-only because its extra
humidity gate is outside this profile. Failed numeric, science and QA history
remain evidence. Unknown profiles and incomplete recorder execution fail.
