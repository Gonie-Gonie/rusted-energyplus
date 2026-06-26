---
status: active
claim_level: conformance-boundary
owner: core
last_reviewed: 2026-06-25
---

# Algorithm Port Ticket

Every compatibility-mode algorithm port starts with an Algorithm Port Ticket.
The ticket is a small source-order contract for one EnergyPlus routine or one
explicit routine group. It prevents diagnostic probes, delta-reduction
experiments, and source-order compatibility work from sharing the same review
surface.

Use `specs/algorithm_port_ticket_template.toml` as the field contract. The
ticket can live in an issue, design note, or PR body, but the PR must link to
it or embed it when it changes source-order compatibility behavior. Pull
requests may use the default template, the compatibility algorithm-port
template, or the diagnostic-probe template under `.github/PULL_REQUEST_TEMPLATE/`.
The pull-request workflow runs `pr-port-ticket-check` and rejects algorithm or
source-order PR bodies that omit the completed ticket fields. The
`algorithm-ledger-check` gate validates that the template, PR prompt, and this
review policy stay aligned.

## Required Fields

| Field | Purpose |
|---|---|
| `algorithm_port_ticket.algorithm_id` | Stable ID matching or preparing an `algorithm_ledger.toml` entry. |
| `algorithm_port_ticket.domain` | Runtime domain such as `heat_balance`, `ideal_loads`, `zone_equipment`, or `node`. |
| `algorithm_port_ticket.port_type` | `compatibility`, `diagnostic_probe`, or `refactor_only`. |
| `energyplus.version` | Pinned EnergyPlus source version. Must remain `26.1.0`. |
| `energyplus.source_file` | EnergyPlus 26.1.0 file that owns the routine. |
| `energyplus.routine` | Routine or callback barrier being ported. |
| `energyplus.source_order_stage` | Source-order stage name used by the Rust execution barrier. |
| `rust.target_module` | Rust module expected to own the compatibility path. |
| `rust.target_function` | Rust function expected to map to the EnergyPlus routine. |
| `rust.execution_stage_kind` | `ExecutionStageKind` barrier that will hold the work. |
| `rust.compatibility_path` | Whether the work is allowed on the compatibility path. |
| `rust.diagnostic_probe_used` | Whether the ticket intentionally selects diagnostic instrumentation. |
| `state_mapping.input_state` | Runtime or model state read by the routine. |
| `state_mapping.output_state` | Runtime state written by the routine. |
| `state_mapping.history_state_ownership` | Timestep or system history slots owned by the routine. |
| `state_mapping.unsupported_state` | EnergyPlus state branches intentionally not implemented. |
| `outputs.affected_variables` | Output variables whose values may change. |
| `outputs.affected_meters` | Output meters whose values may change. |
| `outputs.diagnostic_only_variables` | Variables allowed only in diagnostic reports. |
| `evidence.first_target_case` | First conformance or diagnostic case affected. |
| `evidence.proof_variables` | Variables used to prove the port. |
| `evidence.tolerance_candidate` | Candidate tolerance or reason a tolerance is not yet proposed. |
| `evidence.report_path` | Report artifact expected to show the evidence. |
| `evidence.blocking_gate` | Gate that blocks promotion or release drift. |
| `claim_boundary.conformance_claim` | Must be false until a case, tolerance, report, and blocking gate exist. |
| `claim_boundary.not_claimed_branches` | EnergyPlus branches intentionally not claimed. |
| `claim_boundary.partial_run_allowed` | Whether arbitrary runs may continue as partial diagnostic runs. |

## Review Rules

- A source-order PR must either check `Not an algorithm/source-order change` or pass `pr-port-ticket-check` with completed compatibility, diagnostic, or refactor-only ticket fields.
- A compatibility port must reference an EnergyPlus 26.1.0 source file and
  routine before code is moved into `heat_balance`, `ideal_loads`,
  `zone_equipment`, or `node` compatibility modules.
- A diagnostic probe must set `port_type = "diagnostic_probe"` and
  `claim_boundary.conformance_claim = false`.
- A compatibility ticket must keep `rust.diagnostic_probe_used = false` unless
  the change is explicitly diagnostic-only and cannot be promoted as
  conformance evidence.
- A conformance promotion must have a ledger entry, case manifest, proof
  variables, tolerance, report path, and blocking gate.
- A refactor-only PR can mark the ticket as not applicable only when it does
  not change source-order behavior, output timing, runtime state ownership, or
  claim boundaries.
- Compatibility code must not call diagnostic probe functions. Diagnostic
  probes may call compatibility functions and add instrumentation.

## Relationship To The Ledger

The ticket is the review-time working contract. `specs/algorithm_ledger.toml`
is the durable project inventory. When the port lands, update the ledger in the
same change or explicitly state why the ticket is preparatory only.
