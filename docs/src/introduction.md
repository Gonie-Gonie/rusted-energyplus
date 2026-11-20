# rusted-energyplus

A Rust engine for a bounded subset of EnergyPlus 26.1.0 inputs. Version 0.2.0
is a limited-scope prerelease with explicit input boundaries and a Windows
package. It is not a general replacement for EnergyPlus.

Start with [supported inputs](guides/supported-inputs.md), then
[run an input](guides/run-arbitrary-idf.md) and [inspect/compare results](guides/verify-porting.md).
The [complete object inventory](generated/example-object-union.md) lists all
types found in the selected examples, including blocked types.

Source A is bounded opaque unconditioned heat balance. Source B is bounded
direct-Zone IdealLoads with selected documented extensions. Use the CLI's
`--porting-scope A` or `B`; the launcher uses the older generic route.

[Current status](current/current-status.md) separates retained numerical
evidence from support inventories and package verification. Developer and
generated references retain detailed contracts without making them part of
the first-run workflow.
