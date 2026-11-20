# Support boundary — 0.2.0

This prerelease supports selected, bounded EnergyPlus 26.1.0 inputs.

| Select | Intended model |
| --- | --- |
| `--porting-scope A` | Opaque unconditioned heat balance, with reviewed geometry/gain combinations. |
| `--porting-scope B` | Direct-Zone IdealLoads with selected controls, limits and outdoor-air combinations. |

An object name does not establish support for all its fields, references or
combinations. The example inventory has 110 types: 47 have bounded A/B
admission, 27 bounded B admission and 36 are blocked. These are inventory
classifications, not counts of fully implemented objects.

Active windows, general HVAC/Plant, sizing, EMS, PythonPlugin and AirflowNetwork
are outside ordinary Source scope. The launcher selects the older generic
route; choose Source A/B through the CLI.

Follow [QUICKSTART.md](QUICKSTART.md) for input checking. The
[supported-input guide](docs/src/guides/supported-inputs.md),
[complete inventory](docs/src/generated/example-object-union.md) and
[detailed registry](docs/src/generated/object-coverage.md) state the boundaries.
