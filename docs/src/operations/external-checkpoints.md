# External Checkpoints

External facts must be checked at project start, after each milestone, and
before each release.

## 2026-06-04 Initial Check

- EnergyPlus 26.1.0 release exists and is the selected oracle.
- Release commit locked to `6f2e40d10250a105b49966baa24d843711e61048`.
- Official release assets include Windows x86_64 zip with SHA256 digest.
- Latest ReadTheDocs pages are 26.2, so docs should be treated as directional
  and release-specific behavior must be verified against the 26.1.0 oracle.
- EnergyPlus documentation confirms the epJSON/schema direction.
- EnergyPlus repository documents a regression-oriented development culture.
- Local 26.1.0 source tree contains schema generation tools under `idd/schema`
  and `idd/embedded`, while the packaged Windows runtime contains
  `Energy+.schema.epJSON`.
- Local 26.1.0 source tree contains `testfiles/1ZoneUncontrolled.idf`, matching
  the oracle smoke case family used by the packaged runtime.
- Issue-based tests remain useful for graph-first diagnostics:
  - #4787: omitted zone equipment list should become early typed diagnostic
  - #11599: variable speed pump/common pipe crash should become unsupported
    topology diagnostic
  - #11608: plant/condenser operation scheme priority should become an
    air/plant graph and operation scheme regression case
  - #11615: virtualenv/PythonEngine leakage reinforces isolated oracle runs

## Check Cadence

- Before changing oracle version
- Before adding a supported object family
- Before each release branch
- When an EnergyPlus issue is converted into a regression case
