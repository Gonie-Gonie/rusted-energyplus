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
