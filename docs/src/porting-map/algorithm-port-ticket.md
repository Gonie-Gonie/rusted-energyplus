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
ticket can live in an issue, design note, or PR body, but the PR must embed the
required field snapshot even when it also links to another ticket. Pull
requests may use the default template, the compatibility algorithm-port
template, or the diagnostic-probe template under `.github/PULL_REQUEST_TEMPLATE/`.
The pull-request workflow compares the PR base and head revisions, runs
`pr-port-ticket-check`, and rejects algorithm or source-order changes that omit
the completed ticket fields. The
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
| `state_mapping.inactive_branches` | EnergyPlus branches that are inactive under the ticket's first target case or fixture assumptions. |
| `state_mapping.unsupported_active_branches` | EnergyPlus branches that can be active but remain unsupported or outside the compatibility claim. |
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

- `Not an algorithm/source-order change` is accepted only when the base-to-head
  merge-base changed-file set contains no source-order-sensitive path. A
  non-sensitive PR passes without requiring the checkbox. Rename/copy records
  check both the old and new paths, and deletions check the deleted path.
- The classifier treats production Rust under `ep_runtime`, `ep_compiler`, and
  `ep_run`, the `ep_cli` compatibility selectors/IdealLoads path, durable
  algorithm ledger/capability claims, every script selected by a case
  manifest's `[gate]`, case manifests, evidence-command catalog changes, and
  official dynamic heat-balance/probe lanes as sensitive. Command-catalog
  edits that do not touch a base/head evidence command remain non-sensitive.
  Test-only Rust files, docs, and unrelated governance-only changes are not
  sensitive by themselves.
- A PR with a source-order-sensitive path must pass `pr-port-ticket-check` with
  completed compatibility, diagnostic, or refactor-only ticket fields.
- A source-order-sensitive PR carries one Algorithm ID. Split work that changes
  production paths or durable claim blocks for multiple algorithms into
  separate PRs; the gate rejects unrelated paths and multi-algorithm ledger
  sections instead of letting one ticket cover the batch.
- The gate cross-checks the algorithm ID/domain/source/Rust target against the
  ledger. The selected ledger entry's `port_ticket_mappings` must link the
  EnergyPlus source file, routine, source-order stage, and
  `ExecutionStageKind` as one exact tuple, while its checked-in source map must
  name the routine and stage. When production Rust changes, every surviving
  sensitive Rust path must be a mapped target of the selected algorithm, and
  the ticket's named Rust module must itself change. Deleted and pre-rename
  paths are checked against the selected algorithm's merge-base ledger and
  gate mappings; added, modified, and post-rename paths are checked against
  the head mappings. Changed algorithm-ledger blocks must match the ticket ID,
  changed capability blocks must reference it, and capability-registry changes
  outside `[[capability]]` blocks are rejected until they have an explicit
  algorithm-link contract. Every changed evidence command and changed case/gate
  metadata must resolve through the ticket's base/head evidence boundary. Proof
  variables are checked against coverage and
  the selected evidence case, and report/full gate invocation values against
  that case manifest. EnergyPlus remains pinned to `26.1.0`; the PR check
  therefore runs from a clean checkout without the ignored local
  reference-source tree.
- `First target case` must be the ledger's first case/first evidence or an
  exact case ID declared in that algorithm's support boundary. This permits a
  later evidence case without allowing an unrelated manifest.
- Markdown fenced code, indented code, and HTML comments cannot supply ticket
  fields. Only the level-two `Algorithm Port Ticket` section is parsed, so
  summary fields in other PR sections do not create duplicates.
- A compatibility port must reference an EnergyPlus 26.1.0 source file and
  routine before code is moved into `heat_balance`, `ideal_loads`,
  `zone_equipment`, or `node` compatibility modules.
- A diagnostic probe must set `port_type = "diagnostic_probe"` and
  `claim_boundary.conformance_claim = false`.
- A compatibility ticket must keep `rust.diagnostic_probe_used = false` unless
  the change is explicitly diagnostic-only and cannot be promoted as
  conformance evidence.
- A conformance promotion must have a ledger entry, case manifest, proof
  variables, tolerance, report path, and blocking gate. If a case gate carries
  arguments such as `-CaseId`, `Blocking gate` must include that full
  invocation; command registration is checked against the command token.
- A refactor-only PR can mark the ticket as not applicable only when it does
  not change source-order behavior, output timing, runtime state ownership, or
  claim boundaries.
- Compatibility code must not call diagnostic probe functions. Diagnostic
  probes may call compatibility functions and add instrumentation.

## Relationship To The Ledger

The ticket is the review-time working contract. `specs/algorithm_ledger.toml`
is the durable project inventory. When the port lands, update the ledger in the
same change or explicitly state why the ticket is preparatory only.
