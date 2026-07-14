function New-PrPortTicketTestBody {
    param(
        [string]$PortType = "compatibility",
        [string]$CompatibilityPath = "true",
        [string]$DiagnosticProbeUsed = "false",
        [string]$ConformanceClaim = "no",
        [string]$CheckedLine = "- [x] Compatibility port ticket completed",
        [string]$TicketLocation = "PR body",
        [string]$AlgorithmId = "heat_balance_manager_source_order",
        [string]$Domain = "heat_balance",
        [string]$EnergyPlusVersion = "26.1.0",
        [string]$SourceFile = "src/EnergyPlus/HeatBalanceManager.cc",
        [string]$SourceRoutine = "ManageHeatBalance",
        [string]$SourceOrderStage = "GetHeatBalanceInput",
        [string]$RustModule = "crates/ep_runtime/src/heat_balance/manager.rs",
        [string]$RustFunction = "manage_heat_balance_source_order_stages",
        [string]$ExecutionStage = "GetHeatBalanceInput",
        [string]$FirstTargetCase = "official_1zone_uncontrolled_dynamic_conformance_candidate_001",
        [string]$ProofVariables = "Zone Mean Air Temperature",
        [string]$AffectedVariables = "Zone Mean Air Temperature",
        [string]$AffectedMeters = "none",
        [string]$DiagnosticOnlyVariables = "none",
        [string]$ToleranceCandidate = "0.01 C",
        [string]$ReportPath = ".runtime/official-dynamic-compat-candidate/26.1.0/official_1zone_uncontrolled_dynamic_conformance_candidate_001/compare/compare-report.md",
        [string]$BlockingGate = "official-dynamic-heat-balance-compat-candidate",
        [string]$PartialRunAllowed = "no",
        [string]$UnsupportedState = "fenestration"
    )
    return @"
## Algorithm Port Ticket

- [ ] Not an algorithm/source-order change
$CheckedLine
- Ticket path or PR section: $TicketLocation
- Algorithm ID: $AlgorithmId
- Domain: $Domain
- Port type: $PortType
- EnergyPlus version: $EnergyPlusVersion
- EnergyPlus source file: $SourceFile
- EnergyPlus routine: $SourceRoutine
- EnergyPlus source-order stage: $SourceOrderStage
- Rust target module: $RustModule
- Rust target function: $RustFunction
- ExecutionStageKind: $ExecutionStage
- Compatibility path: $CompatibilityPath
- Diagnostic probe used: $DiagnosticProbeUsed
- Read state: zone state
- Write state: result store
- History/state ownership: heat_balance::state
- Unsupported state: $UnsupportedState
- Inactive branches: no-HVAC branch disabled by fixture
- Unsupported active branches: active HVAC branch
- Affected variables: $AffectedVariables
- Affected meters: $AffectedMeters
- Diagnostic-only variables: $DiagnosticOnlyVariables
- First target case: $FirstTargetCase
- Proof variables: $ProofVariables
- Tolerance candidate: $ToleranceCandidate
- Report path: $ReportPath
- Blocking gate: $BlockingGate
- Conformance claim: $ConformanceClaim
- Not-claimed branches: HVAC, plant
- Partial run allowed: $PartialRunAllowed
"@
}

function Invoke-PrPortTicketSelfTest {
    $runtimeFile = "crates/ep_runtime/src/heat_balance/manager.rs"
    $conformanceRuntimeFile = "crates/ep_runtime/src/heat_balance/algorithm.rs"
    $diagnosticScript = "scripts/smoke/air-side-node-diagnostic-smoke.ps1"
    $headRevision = (& git -C $RepoRoot rev-parse HEAD).Trim()
    $validCompatibility = New-PrPortTicketTestBody
    $validDiagnostic = New-PrPortTicketTestBody `
        -PortType "diagnostic_probe" `
        -CompatibilityPath "false" `
        -DiagnosticProbeUsed "true" `
        -ConformanceClaim "no" `
        -CheckedLine "- [x] Diagnostic probe only; no conformance claim" `
        -AlgorithmId "air_side_node_state" `
        -Domain "hvac" `
        -SourceFile "src/EnergyPlus/OutputProcessor.cc" `
        -SourceRoutine "SetupOutputVariable" `
        -SourceOrderStage "SetupOutputVariable" `
        -RustModule "crates/ep_runtime/src/node/projection.rs" `
        -RustFunction "simulate_ideal_loads_node_state_projection" `
        -ExecutionStage "Output" `
        -FirstTargetCase "air_side_node_diagnostic_001" `
        -ProofVariables "System Node Temperature" `
        -AffectedVariables "System Node Temperature" `
        -ReportPath ".runtime/air-side-node-diagnostic/26.1.0/report-skeleton/air_side_node_diagnostic_001/compare-report.md" `
        -BlockingGate "air-side-node-diagnostic-smoke"
    $validConformance = New-PrPortTicketTestBody `
        -ConformanceClaim "yes" `
        -AlgorithmId "official_dynamic_heat_balance_compat_candidate" `
        -SourceFile "src/EnergyPlus/ZoneTempPredictorCorrector.cc" `
        -SourceRoutine "ManageZoneAirUpdates" `
        -SourceOrderStage "ManageZoneAirUpdates" `
        -RustModule "crates/ep_runtime/src/heat_balance/algorithm.rs" `
        -RustFunction "heat_balance_zone_air_algorithm_feature_base" `
        -ExecutionStage "ManageZoneAirUpdates"
    $validRefactor = New-PrPortTicketTestBody `
        -PortType "refactor_only" `
        -CheckedLine "" `
        -ConformanceClaim "no"
    $backtickFence = ((1..3 | ForEach-Object { [char]96 }) -join "")

    $cases = @(
        [pscustomobject]@{
            name = "docs_only_auto_pass"
            body = ""
            files = @("README.md")
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "non_algorithm_without_diff"
            body = "- [x] Not an algorithm/source-order change"
            files = @()
            changedFilesProvided = $false
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "runtime_source_order_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "algorithm_ledger_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @("specs/algorithm_ledger.toml")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "capabilities_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @("specs/capabilities.toml")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "evidence_command_catalog_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @("scripts/dev/commands.json")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "case_manifest_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @("data/conformance_cases/official_1zone_uncontrolled_dynamic_conformance_candidate_001/case.toml")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "ep_run_pipeline_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @("crates/ep_run/src/pipeline.rs")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "ideal_loads_evidence_gate_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @("scripts/compare/compare-ideal-loads-outdoor-air-flow-person-conformance-candidate.ps1")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "official_heat_balance_lane_cannot_opt_out"
            body = "- [x] Not an algorithm/source-order change"
            files = @("scripts/compare/official-dynamic-heat-balance-warmup-lanes.ps1")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "test_only_runtime_auto_pass"
            body = ""
            files = @("crates/ep_runtime/src/runtime/tests/part01.rs")
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "plural_test_file_auto_pass"
            body = ""
            files = @("crates/ep_runtime/src/ideal_loads/runtime_tests.rs")
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "singular_test_file_auto_pass"
            body = ""
            files = @("crates/ep_runtime/src/ideal_loads/test.rs")
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "governance_only_auto_pass"
            body = ""
            files = @(".github/workflows/pull-request.yml")
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "non_evidence_command_catalog_auto_pass"
            body = ""
            files = @("scripts/dev/commands.json")
            changedFilesProvided = $true
            baseRevision = $headRevision
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "valid_compatibility"
            body = $validCompatibility
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "valid_diagnostic"
            body = $validDiagnostic
            files = @($diagnosticScript)
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "valid_conformance"
            body = $validConformance
            files = @($conformanceRuntimeFile)
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "valid_refactor"
            body = $validRefactor
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "valid_case_manifest_change"
            body = $validCompatibility
            files = @("data/conformance_cases/official_1zone_uncontrolled_dynamic_conformance_candidate_001/case.toml")
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "unrelated_case_manifest_ticket"
            body = $validCompatibility
            files = @("data/conformance_cases/heat_balance_nomass_001/case.toml")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "missing_ticket"
            body = "## Algorithm Port Ticket`n`n- [ ] Not an algorithm/source-order change"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "fake_ticket_location"
            body = New-PrPortTicketTestBody -TicketLocation "definitely/not/a/checked/file.toml"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unknown_algorithm"
            body = New-PrPortTicketTestBody -AlgorithmId "missing_algorithm"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unrelated_algorithm_ticket"
            body = $validCompatibility
            files = @("crates/ep_runtime/src/ideal_loads/runtime.rs")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "mapped_file_cannot_cover_unrelated_rust"
            body = $validCompatibility
            files = @($runtimeFile, "crates/ep_runtime/src/ideal_loads/runtime.rs")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unrelated_gate_ticket"
            body = $validCompatibility
            files = @($diagnosticScript)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "mapped_deleted_rust_path"
            body = $validCompatibility
            files = @($runtimeFile)
            changedFilesProvided = $true
            records = @([pscustomobject]@{ status = "D"; side = "base"; path = $runtimeFile })
            baseRevision = $headRevision
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "unrelated_deleted_rust_path"
            body = $validCompatibility
            files = @("crates/ep_runtime/src/ideal_loads/deleted_algorithm.rs")
            changedFilesProvided = $true
            records = @([pscustomobject]@{ status = "D"; side = "base"; path = "crates/ep_runtime/src/ideal_loads/deleted_algorithm.rs" })
            baseRevision = $headRevision
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unrelated_deleted_gate_script"
            body = $validCompatibility
            files = @("scripts/compare/official-dynamic-heat-balance-deleted-lane.ps1")
            changedFilesProvided = $true
            records = @([pscustomobject]@{ status = "D"; side = "base"; path = "scripts/compare/official-dynamic-heat-balance-deleted-lane.ps1" })
            baseRevision = $headRevision
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "mapped_deleted_gate_script"
            body = $validConformance
            files = @("scripts/compare/official-dynamic-heat-balance-compat-candidate.ps1")
            changedFilesProvided = $true
            records = @([pscustomobject]@{ status = "D"; side = "base"; path = "scripts/compare/official-dynamic-heat-balance-compat-candidate.ps1" })
            baseRevision = $headRevision
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "unrelated_ledger_ticket_without_base_context"
            body = $validCompatibility
            files = @("specs/algorithm_ledger.toml")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unrelated_capability_ticket_without_base_context"
            body = $validCompatibility
            files = @("specs/capabilities.toml")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unrelated_command_ticket_without_base_context"
            body = $validCompatibility
            files = @("scripts/dev/commands.json")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "ticket_rust_module_must_change"
            body = $validDiagnostic
            files = @("crates/ep_runtime/src/node/state.rs")
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "domain_mismatch"
            body = New-PrPortTicketTestBody -Domain "hvac"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "energyplus_version_drift"
            body = New-PrPortTicketTestBody -EnergyPlusVersion "25.2.0"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unmapped_source_file"
            body = New-PrPortTicketTestBody -SourceFile "src/EnergyPlus/Fake.cc"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "invalid_source_order_stage"
            body = New-PrPortTicketTestBody -SourceOrderStage "DefinitelyNotAStage"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "missing_energyplus_routine"
            body = New-PrPortTicketTestBody -SourceRoutine "DefinitelyMissingRoutine"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "common_word_is_not_a_routine"
            body = New-PrPortTicketTestBody -SourceRoutine "state"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "routine_from_other_source"
            body = New-PrPortTicketTestBody -SourceRoutine "CalcHeatBalanceInsideSurf"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "stage_from_other_algorithm"
            body = New-PrPortTicketTestBody -SourceOrderStage "CalcHeatBalanceInsideSurf"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "missing_rust_function"
            body = New-PrPortTicketTestBody -RustFunction "missing_function"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unmapped_existing_rust_function"
            body = New-PrPortTicketTestBody -RustFunction "manage_heat_balance_source_order_path"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "invalid_execution_stage"
            body = New-PrPortTicketTestBody -ExecutionStage "MissingStage"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "missing_first_case"
            body = New-PrPortTicketTestBody -FirstTargetCase "missing_case"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "non_ledger_first_case"
            body = New-PrPortTicketTestBody -FirstTargetCase "heat_balance_nomass_001"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "uncovered_proof_variable"
            body = New-PrPortTicketTestBody -ProofVariables "Missing Output"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "uncovered_affected_variable"
            body = New-PrPortTicketTestBody -AffectedVariables "Zone Mean Air Temperature, Fake Output"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "report_mismatch"
            body = New-PrPortTicketTestBody -ReportPath ".runtime/fake-report.md"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "gate_mismatch"
            body = New-PrPortTicketTestBody -BlockingGate "missing-gate"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "invalid_partial_run_boolean"
            body = New-PrPortTicketTestBody -PartialRunAllowed "maybe"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "invalid_tolerance_candidate"
            body = New-PrPortTicketTestBody -ToleranceCandidate "banana"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "placeholder_state"
            body = New-PrPortTicketTestBody -UnsupportedState "TODO"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "diagnostic_claim"
            body = $validDiagnostic -replace "Conformance claim: no", "Conformance claim: yes"
            files = @($diagnosticScript)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "mutually_exclusive_classification"
            body = "$validCompatibility`n- [x] Diagnostic probe only; no conformance claim"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "duplicate_ticket_field"
            body = "$validCompatibility`n- Domain: heat_balance"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "other_section_may_repeat_summary_fields"
            body = "$validCompatibility`n## Claim Boundary`n`n- Report path: summary only`n- Blocking gate: summary only"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "code_fence_cannot_supply_opt_out"
            body = @'
```markdown
- [x] Not an algorithm/source-order change
```
'@
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "tilde_fence_cannot_supply_ticket"
            body = "~~~markdown`n$validCompatibility`n~~~"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "indented_fence_cannot_supply_ticket"
            body = "  ${backtickFence}markdown`n$validCompatibility`n  ${backtickFence}"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "html_comment_cannot_supply_ticket"
            body = "<!--`n$validCompatibility`n-->"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "unclosed_html_comment_cannot_supply_ticket"
            body = "<!--`n$validCompatibility"
            files = @($runtimeFile)
            changedFilesProvided = $true
            shouldPass = $false
        }
    )

    $defaultTemplate = Get-Content -Encoding UTF8 -Raw -LiteralPath (
        Join-Path $RepoRoot ".github\pull_request_template.md"
    )
    foreach ($field in @("Conformance claim", "Report path", "Blocking gate")) {
        [void](Get-TicketField -Text $defaultTemplate -Name $field)
    }
    Write-Host "OK PR port-ticket self-test: default_template_unique_ticket_fields"

    foreach ($case in $cases) {
        $passed = $true
        $failureMessage = ""
        try {
            $arguments = @{
                Text = $case.body
                ChangedFiles = $case.files
                ChangedFilesProvided = $case.changedFilesProvided
            }
            if ($case.PSObject.Properties.Name -contains "records") {
                $arguments.ChangedFileRecords = $case.records
            }
            if ($case.PSObject.Properties.Name -contains "baseRevision") {
                $arguments.BaseRevision = $case.baseRevision
            }
            [void](Test-AlgorithmPortTicketBody @arguments)
        }
        catch {
            $passed = $false
            $failureMessage = $_.Exception.Message
        }
        if ($passed -ne $case.shouldPass) {
            throw "Unexpected PR port-ticket self-test result for $($case.name): pass=$passed; error=$failureMessage"
        }
        Write-Host "OK PR port-ticket self-test: $($case.name)"
    }

    $nul = [char]0
    $renamePaths = @(
        ConvertFrom-GitNameStatusZ -Text "R100${nul}crates/ep_runtime/src/old.rs${nul}docs/old.rs${nul}"
    )
    if (
        $renamePaths.Count -ne 2 -or
        $renamePaths[0] -ne "crates/ep_runtime/src/old.rs" -or
        $renamePaths[1] -ne "docs/old.rs" -or
        -not (Test-AlgorithmSourceOrderPath -Path $renamePaths[0])
    ) {
        throw "Git rename parser self-test did not preserve and classify old and new paths."
    }
    Write-Host "OK PR port-ticket self-test: rename_old_and_new_paths"

    $renameRecords = @(
        ConvertFrom-GitNameStatusRecordsZ -Text "R100${nul}crates/ep_runtime/src/old.rs${nul}docs/old.rs${nul}"
    )
    if ($renameRecords[0].side -ne "base" -or $renameRecords[1].side -ne "head") {
        throw "Git rename parser self-test did not preserve base/head sides."
    }
    Write-Host "OK PR port-ticket self-test: rename_base_and_head_sides"

    $malformedRejected = $false
    try {
        [void](ConvertFrom-GitNameStatusZ -Text "R100${nul}missing-new-path${nul}")
    }
    catch {
        $malformedRejected = $true
    }
    if (-not $malformedRejected) {
        throw "Git name-status parser self-test accepted a malformed rename."
    }
    Write-Host "OK PR port-ticket self-test: malformed_rename"

    $deletePaths = @(ConvertFrom-GitNameStatusZ -Text "D${nul}${runtimeFile}${nul}")
    if ($deletePaths.Count -ne 1 -or -not (Test-AlgorithmSourceOrderPath -Path $deletePaths[0])) {
        throw "Git delete parser self-test did not preserve and classify the deleted path."
    }
    Write-Host "OK PR port-ticket self-test: deleted_sensitive_path"

    $newSensitiveRenamePaths = @(
        ConvertFrom-GitNameStatusZ -Text "R097${nul}docs/old.rs${nul}${runtimeFile}${nul}"
    )
    if (
        $newSensitiveRenamePaths.Count -ne 2 -or
        -not (Test-AlgorithmSourceOrderPath -Path $newSensitiveRenamePaths[1])
    ) {
        throw "Git rename parser self-test did not classify a new sensitive path."
    }
    Write-Host "OK PR port-ticket self-test: rename_new_sensitive_path"

    $gateWithArguments = Get-DevCommandName -Value (
        "scripts/dev.cmd compare-ideal-loads-humidity-annual-meter-conformance-candidate " +
        "-CaseId example"
    )
    if ($gateWithArguments -ne "compare-ideal-loads-humidity-annual-meter-conformance-candidate") {
        throw "Gate command parser did not separate the command name from arguments."
    }
    $normalizedGateWithArguments = Get-NormalizedDevCommandInvocation -Value (
        "scripts/dev.cmd compare-ideal-loads-humidity-annual-meter-conformance-candidate " +
        "-CaseId example"
    )
    $normalizedGateWithoutArguments = Get-NormalizedDevCommandInvocation -Value (
        "compare-ideal-loads-humidity-annual-meter-conformance-candidate"
    )
    if ($normalizedGateWithArguments -eq $normalizedGateWithoutArguments) {
        throw "Gate invocation normalization discarded case-selecting arguments."
    }
    Write-Host "OK PR port-ticket self-test: gate_command_with_arguments"

    $unrelatedEvidenceCommands = @(
        Get-UnrelatedEvidenceCommandNames `
            -ChangedCommands @("selected-gate", "unrelated-gate", "non-evidence-command") `
            -EvidenceCommands @("selected-gate", "unrelated-gate") `
            -AllowedCommands @("selected-gate")
    )
    if (
        $unrelatedEvidenceCommands.Count -ne 1 -or
        $unrelatedEvidenceCommands[0] -ne "unrelated-gate"
    ) {
        throw "Evidence-command subset self-test allowed an unrelated gate to hitchhike."
    }
    Write-Host "OK PR port-ticket self-test: evidence_command_subset"

    $selectedCaseId = "official_1zone_uncontrolled_dynamic_conformance_candidate_001"
    $boundaryCommands = @(
        Get-AllowedGateCommandBoundaryNames `
            -HeadCaseIds @($selectedCaseId) `
            -BaseCaseIds @($selectedCaseId) `
            -BaseRevision $headRevision
    )
    if (
        $boundaryCommands.Count -ne 1 -or
        $boundaryCommands[0] -ne "official-dynamic-heat-balance-compat-candidate"
    ) {
        throw "Gate-command boundary union concatenated scalar head/base command results."
    }
    Write-Host "OK PR port-ticket self-test: gate_command_boundary_union"

    $missingBaseCaseCommand = Get-GateCommandNameForCase `
        -CaseId "__definitely_new_case_001" `
        -Revision $headRevision
    if ($null -ne $missingBaseCaseCommand) {
        throw "New-case command transition self-test found a command at the missing base path."
    }
    Write-Host "OK PR port-ticket self-test: new_case_command_transition"

    $syntheticBaseLedger = "[[algorithm]]`nid = `"algorithm_a`"`nstatus = `"scaffold`"`n"
    $syntheticHeadLedger = "[[algorithm]]`nid = `"algorithm_a`"`nstatus = `"conformance`"`n"
    $syntheticChangedIds = @(
        Get-ChangedTomlBlockIds `
            -BaseText $syntheticBaseLedger `
            -HeadText $syntheticHeadLedger `
            -Table "algorithm"
    )
    if ($syntheticChangedIds.Count -ne 1 -or $syntheticChangedIds[0] -ne "algorithm_a") {
        throw "TOML block-diff self-test did not identify the changed algorithm ID."
    }
    Write-Host "OK PR port-ticket self-test: changed_algorithm_block_id"

    $gitLedgerText = Get-GitFileText -Revision $headRevision -Path "specs/algorithm_ledger.toml"
    $lfLedgerText = $gitLedgerText -replace "\r\n?", "`n"
    $newlineOnlyChangedIds = @(
        Get-ChangedTomlBlockIds `
            -BaseText $gitLedgerText `
            -HeadText $lfLedgerText `
            -Table "algorithm"
    )
    if ($newlineOnlyChangedIds.Count -ne 0) {
        throw "TOML block-diff self-test treated CRLF/LF-only ledger differences as changed."
    }
    Write-Host "OK PR port-ticket self-test: ledger_newline_normalization"

    $syntheticCapabilityBase = @"
schema = "test.v1"

[[capability]]
id = "capability_a"
algorithms = ["algorithm_a"]

[[unsupported_rule]]
id = "unsupported_a"
reason = "base reason"

[arbitrary_run]
status = "initial"
"@
    $syntheticCapabilityHead = $syntheticCapabilityBase.Replace("base reason", "changed reason")
    $changedCapabilityIds = @(
        Get-ChangedTomlBlockIds `
            -BaseText $syntheticCapabilityBase `
            -HeadText $syntheticCapabilityHead `
            -Table "capability"
    )
    $changedCapabilitySections = @(
        Get-ChangedTomlSectionKeys `
            -BaseText $syntheticCapabilityBase `
            -HeadText $syntheticCapabilityHead
    )
    if (
        $changedCapabilityIds.Count -ne 0 -or
        $changedCapabilitySections.Count -ne 1 -or
        $changedCapabilitySections[0] -ne "array:unsupported_rule:unsupported_a"
    ) {
        throw "Capability section self-test attributed an unsupported-rule mutation to a capability."
    }
    Write-Host "OK PR port-ticket self-test: capability_section_boundary"

    $syntheticCapabilityAndRootHead = $syntheticCapabilityBase.Replace(
        'schema = "test.v1"',
        'schema = "test.v2"'
    ).Replace(
        'algorithms = ["algorithm_a"]',
        'algorithms = ["algorithm_a", "algorithm_b"]'
    )
    $changedRootAndCapabilitySections = @(
        Get-ChangedTomlSectionKeys `
            -BaseText $syntheticCapabilityBase `
            -HeadText $syntheticCapabilityAndRootHead
    )
    if (
        $changedRootAndCapabilitySections -notcontains "root" -or
        $changedRootAndCapabilitySections -notcontains "array:capability:capability_a"
    ) {
        throw "Capability section self-test did not preserve a root change beside a capability mutation."
    }
    Write-Host "OK PR port-ticket self-test: capability_root_hitchhike"
}
