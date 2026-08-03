[CmdletBinding()]
param()

# Windows PowerShell 5.1 caps visible variables at 4096 by default. The
# source-order checkpoint audits intentionally share a large master scope.
if (Get-Variable -Name MaximumVariableCount -ErrorAction SilentlyContinue) {
    $MaximumVariableCount = [Math]::Max([int]$MaximumVariableCount, 16384)
}

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description missing: $Path"
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -notmatch $Pattern) {
        throw "$Description missing in $Path"
    }
}

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -match $Pattern) {
        throw "$Description unexpectedly present in $Path"
    }
}

function Assert-LineLimit {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][int]$Limit,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $lineCount = (Get-Content -Encoding UTF8 -LiteralPath $Path | Measure-Object -Line).Lines
    if ($lineCount -gt $Limit) {
        throw "$Description exceeds $Limit LOC: $Path has $lineCount LOC"
    }
}

function Assert-ExactStringArray {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string[]]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    $namePattern = [regex]::Escape($Name)
    $arrayMatch = [regex]::Match(
        $text,
        "(?s)pub const\s+$namePattern\s*:\s*&\[\&str\]\s*=\s*&\[(?<body>.*?)\];"
    )
    if (-not $arrayMatch.Success) {
        throw "$Description declaration missing in $Path"
    }

    [string[]]$actual = @(
        [regex]::Matches($arrayMatch.Groups["body"].Value, '"(?<value>[^"]+)"') |
            ForEach-Object { $_.Groups["value"].Value }
    )
    if ($actual.Count -ne $Expected.Count) {
        throw "$Description expected $($Expected.Count) entries, found $($actual.Count) in $Path"
    }
    for ($index = 0; $index -lt $Expected.Count; $index += 1) {
        if ($actual[$index] -cne $Expected[$index]) {
            throw "$Description entry $($index + 1) expected '$($Expected[$index])', found '$($actual[$index])' in $Path"
        }
    }
}

function Assert-PatternsInOrder {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string[]]$Patterns,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    $cursor = 0
    for ($index = 0; $index -lt $Patterns.Count; $index += 1) {
        $remaining = $text.Substring($cursor)
        $match = [regex]::Match($remaining, $Patterns[$index])
        if (-not $match.Success) {
            throw "$Description pattern $($index + 1) missing or out of order in $Path"
        }
        $cursor += $match.Index + $match.Length
    }
}

$calcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$calcLifecycle = "crates\ep_runtime\src\ideal_loads\calc\lifecycle.rs"
$calcLifecycleTests = "crates\ep_runtime\src\ideal_loads\calc\lifecycle_tests.rs"
$calcMinimumOaPrefix = "crates\ep_runtime\src\ideal_loads\calc\minimum_oa_prefix.rs"
$calcMinimumOaPrefixTests = "crates\ep_runtime\src\ideal_loads\calc\minimum_oa_prefix_tests.rs"
$calcCoolingEntryGate = "crates\ep_runtime\src\ideal_loads\calc\cooling_entry_gate.rs"
$calcCoolingEntryGateRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_entry_gate\release.rs"
$calcCoolingEntryGateTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_entry_gate_tests.rs"
$calcCoolingOaMaxFlowGate = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_gate.rs"
$calcCoolingOaMaxFlowGateRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_gate\release.rs"
$calcCoolingOaMaxFlowGateReleaseValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_gate\release\validation.rs"
$calcCoolingOaMaxFlowGateTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_gate_tests.rs"
$calcCoolingOaMaxFlowBody = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_body.rs"
$calcCoolingOaMaxFlowBodyTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_body\transition.rs"
$calcCoolingOaMaxFlowBodyRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_body\release.rs"
$calcCoolingOaMaxFlowBodyReleaseValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_body\release\validation.rs"
$calcCoolingOaMaxFlowBodyTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_oa_max_flow_body_tests.rs"
$calcCoolingEconomizerGuard = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_guard.rs"
$calcCoolingEconomizerGuardTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_guard\transition.rs"
$calcCoolingEconomizerGuardRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_guard\release.rs"
$calcCoolingEconomizerGuardReleaseValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_guard\release\validation.rs"
$calcCoolingEconomizerGuardTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_guard_tests.rs"
$calcCoolingEconomizerCondition = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition.rs"
$calcCoolingEconomizerConditionTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\transition.rs"
$calcCoolingEconomizerConditionRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release.rs"
$calcCoolingEconomizerConditionEntryPrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\entry_prefix_validation.rs"
$calcCoolingEconomizerConditionCompletedConditionValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\completed_condition_validation.rs"
$calcCoolingEconomizerConditionInitializationValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\initialization_validation.rs"
$calcCoolingEconomizerConditionPredecessorValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\predecessor_validation.rs"
$calcCoolingEconomizerConditionRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\runtime_validation.rs"
$calcCoolingEconomizerConditionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_tests.rs"
$calcCoolingEconomizerConditionReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_release_tests.rs"
$calcCoolingEconomizerConditionReleaseProvenanceTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_release_tests\provenance_tests.rs"
$calcCoolingEconomizerConditionReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_release_tests\corruption_tests.rs"
$calcCoolingEconomizerBody = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body.rs"
$calcCoolingEconomizerBodyState = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\state.rs"
$calcCoolingEconomizerBodyTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\transition.rs"
$calcCoolingEconomizerBodyRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\release.rs"
$calcCoolingEconomizerBodyEntryPrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\release\entry_prefix_validation.rs"
$calcCoolingEconomizerBodyInitializationValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\release\initialization_validation.rs"
$calcCoolingEconomizerBodyPredecessorValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\release\predecessor_validation.rs"
$calcCoolingEconomizerBodyRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\release\runtime_validation.rs"
$calcCoolingEconomizerBodyTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_tests.rs"
$calcCoolingEconomizerBodyGateAndAssignmentTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_tests\gate_and_assignment_tests.rs"
$calcCoolingEconomizerBodyIeeeTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_tests\ieee_tests.rs"
$calcCoolingEconomizerBodySkipTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_tests\skip_tests.rs"
$calcCoolingEconomizerBodySourceOrderTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_tests\source_order_tests.rs"
$calcCoolingEconomizerBodyReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_release_tests.rs"
$calcCoolingEconomizerBodyReleaseProvenanceTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_release_tests\provenance_tests.rs"
$calcCoolingEconomizerBodyReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body_release_tests\corruption_tests.rs"
$calcCoolingEconomizerBodyCompletedBodyValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_body\release\completed_body_validation.rs"
$calcCoolingSensibleFlow = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow.rs"
$calcCoolingSensibleFlowState = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow\state.rs"
$calcCoolingSensibleFlowTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow\transition.rs"
$calcCoolingSensibleFlowRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow\release.rs"
$calcCoolingSensibleFlowCompletedStateValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow\release\completed_state_validation.rs"
$calcCoolingSensibleFlowPredecessorValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow\release\predecessor_validation.rs"
$calcCoolingSensibleFlowRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow\release\runtime_validation.rs"
$calcCoolingSensibleFlowSnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow\release\snapshot_validation.rs"
$calcCoolingSensibleFlowTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow_tests.rs"
$calcCoolingSensibleFlowSourceOrderTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow_tests\source_order_tests.rs"
$calcCoolingSensibleFlowSkipTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow_tests\skip_tests.rs"
$calcCoolingSensibleFlowGateTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow_tests\gate_tests.rs"
$calcCoolingSensibleFlowReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow_release_tests.rs"
$calcCoolingSensibleFlowReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_sensible_flow_release_tests\corruption_tests.rs"
$calcCoolingDehumidificationFlow = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow.rs"
$calcCoolingDehumidificationFlowState = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow\state.rs"
$calcCoolingDehumidificationFlowTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow\transition.rs"
$calcCoolingDehumidificationFlowRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow\release.rs"
$calcCoolingDehumidificationFlowCompletedStateValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow\release\completed_state_validation.rs"
$calcCoolingDehumidificationFlowPredecessorValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow\release\predecessor_validation.rs"
$calcCoolingDehumidificationFlowRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow\release\runtime_validation.rs"
$calcCoolingDehumidificationFlowSnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow\release\snapshot_validation.rs"
$calcCoolingDehumidificationFlowTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow_tests.rs"
$calcCoolingDehumidificationFlowSourceOrderTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow_tests\source_order_tests.rs"
$calcCoolingDehumidificationFlowSkipTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow_tests\skip_tests.rs"
$calcCoolingDehumidificationFlowGateTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow_tests\gate_tests.rs"
$calcCoolingDehumidificationFlowReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow_release_tests.rs"
$calcCoolingDehumidificationFlowReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_dehumidification_flow_release_tests\corruption_tests.rs"
$calcCoolingHumidificationFlow = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow.rs"
$calcCoolingHumidificationFlowState = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow\state.rs"
$calcCoolingHumidificationFlowTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow\transition.rs"
$calcCoolingHumidificationFlowRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow\release.rs"
$calcCoolingHumidificationFlowPredecessorValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow\release\predecessor_validation.rs"
$calcCoolingHumidificationFlowRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow\release\runtime_validation.rs"
$calcCoolingHumidificationFlowSnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow\release\snapshot_validation.rs"
$calcCoolingHumidificationFlowTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow_tests.rs"
$calcCoolingHumidificationFlowReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow_release_tests.rs"
$calcHumidity = "crates\ep_runtime\src\ideal_loads\calc\humidity.rs"
$calcLimits = "crates\ep_runtime\src\ideal_loads\calc\limits.rs"
$calcMassFlow = "crates\ep_runtime\src\ideal_loads\calc\mass_flow.rs"
$calcMoistureDemand = "crates\ep_runtime\src\ideal_loads\calc\moisture_demand.rs"
$calcMoistureDemandTests = "crates\ep_runtime\src\ideal_loads\calc\moisture_demand_tests.rs"
$noOaCalc = "crates\ep_runtime\src\ideal_loads\calc\no_oa.rs"
$calcPsychrometrics = "crates\ep_runtime\src\ideal_loads\calc\psychrometrics.rs"
$calcTypes = "crates\ep_runtime\src\ideal_loads\calc\types.rs"
$noOaTests = "crates\ep_runtime\src\ideal_loads\calc\no_oa_tests.rs"
$idealLoadsMod = "crates\ep_runtime\src\ideal_loads\mod.rs"
$idealLoadsHumidistat = "crates\ep_runtime\src\ideal_loads\humidistat.rs"
$idealLoadsHumidistatTests = "crates\ep_runtime\src\ideal_loads\humidistat_tests.rs"
$idealLoadsSizing = "crates\ep_runtime\src\ideal_loads\sizing.rs"
$idealLoadsSizingTests = "crates\ep_runtime\src\ideal_loads\sizing_tests.rs"
$idealLoadsInit = "crates\ep_runtime\src\ideal_loads\init.rs"
$idealLoadsInitManagerPlan = "crates\ep_runtime\src\ideal_loads\init\manager_plan.rs"
$idealLoadsInitManagerPlanTests = "crates\ep_runtime\src\ideal_loads\init\manager_plan_tests.rs"
$idealLoadsInitManagerScanTests = "crates\ep_runtime\src\ideal_loads\init\manager_scan_tests.rs"
$idealLoadsInitSummary = "crates\ep_runtime\src\ideal_loads\init\summary.rs"
$idealLoadsInitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$idealLoadsInitWitnesses = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$idealLoadsInitSupplyTemperatureDiagnostic = "crates\ep_runtime\src\ideal_loads\init\supply_temperature_diagnostic.rs"
$idealLoadsInitSupplyTemperatureDiagnosticTests = "crates\ep_runtime\src\ideal_loads\init\supply_temperature_diagnostic_tests.rs"
$idealLoadsInitTopologyPlan = "crates\ep_runtime\src\ideal_loads\init\topology_plan.rs"
$idealLoadsInitTopologyPlanTests = "crates\ep_runtime\src\ideal_loads\init\topology_plan_tests.rs"
$idealLoadsInitTopologyTransition = "crates\ep_runtime\src\ideal_loads\init\topology_transition.rs"
$idealLoadsInitTopologyTransitionTests = "crates\ep_runtime\src\ideal_loads\init\topology_transition_tests.rs"
$idealLoadsInitTransition = "crates\ep_runtime\src\ideal_loads\init\transition.rs"
$idealLoadsInitTests = "crates\ep_runtime\src\ideal_loads\init\lifecycle_tests.rs"
$idealLoadsInitWarningTests = "crates\ep_runtime\src\ideal_loads\init\warning_tests.rs"
$idealLoadsBindingMinimumOaTests = "crates\ep_runtime\src\ideal_loads\binding\minimum_oa_prefix_tests.rs"
$idealLoadsBindingCoolingEntryGateTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_entry_gate_tests.rs"
$idealLoadsBindingCoolingOaMaxFlowGateTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_oa_max_flow_gate_tests.rs"
$idealLoadsBindingCoolingOaMaxFlowBodyTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_oa_max_flow_body_tests.rs"
$idealLoadsBindingCoolingEconomizerGuardTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_economizer_guard_tests.rs"
$idealLoadsBindingCoolingEconomizerGuardIntegrityTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_economizer_guard_integrity_tests.rs"
$idealLoadsBindingCoolingEconomizerConditionTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_economizer_condition_tests.rs"
$idealLoadsBindingCoolingEconomizerConditionIntegrityTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_economizer_condition_integrity_tests.rs"
$idealLoadsBindingCoolingEconomizerBodyTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_economizer_body_tests.rs"
$idealLoadsBindingCoolingEconomizerBodyIntegrityTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_economizer_body_integrity_tests.rs"
$idealLoadsBindingCoolingSensibleFlowTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_sensible_flow_tests.rs"
$idealLoadsBindingCoolingDehumidificationFlowTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_dehumidification_flow_tests.rs"
$idealLoadsBindingCoolingHumidificationFlowTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_humidification_flow_tests.rs"
$idealLoadsCoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$idealLoadsCoupledMinimumOaValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\minimum_oa_validation.rs"
$idealLoadsCoupledCoolingEntryValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_entry_validation.rs"
$idealLoadsCoupledCoolingOaMaxFlowValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_oa_max_flow_validation.rs"
$idealLoadsCoupledCoolingOaMaxFlowBodyValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_oa_max_flow_body_validation.rs"
$idealLoadsCoupledCoolingEconomizerGuardValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_economizer_guard_validation.rs"
$idealLoadsCoupledCoolingEconomizerConditionValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_economizer_condition_validation.rs"
$idealLoadsCoupledCoolingEconomizerBodyValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_economizer_body_validation.rs"
$idealLoadsCoupledCoolingSensibleFlowValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_sensible_flow_validation.rs"
$idealLoadsCoupledCoolingDehumidificationFlowValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_dehumidification_flow_validation.rs"
$idealLoadsCoupledCoolingHumidificationFlowValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_humidification_flow_validation.rs"
$idealLoadsCoupledOutputCoolingHumidificationFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_humidification_flow_fixture.rs"
$idealLoadsCouplingValidation = "crates\ep_runtime\src\ideal_loads\coupling\validation.rs"
$idealLoadsInput = "crates\ep_runtime\src\ideal_loads\input.rs"
$idealLoadsMeters = "crates\ep_runtime\src\ideal_loads\meters.rs"
$idealLoadsReport = "crates\ep_runtime\src\ideal_loads\report.rs"
$idealLoadsReportTests = "crates\ep_runtime\src\ideal_loads\report_tests.rs"
$idealLoadsReportSemantics = "crates\ep_runtime\src\ideal_loads\report\semantics.rs"
$idealLoadsRuntime = "crates\ep_runtime\src\ideal_loads\runtime.rs"
$idealLoadsUpdate = "crates\ep_runtime\src\ideal_loads\update.rs"
$outdoorAir = "crates\ep_runtime\src\ideal_loads\outdoor_air.rs"
$outdoorAirTests = "crates\ep_runtime\src\ideal_loads\outdoor_air_tests.rs"
$outdoorAirDcvTests = "crates\ep_runtime\src\ideal_loads\outdoor_air_dcv_tests.rs"
$outdoorAirWrapperTests = "crates\ep_runtime\src\ideal_loads\outdoor_air_wrapper_tests.rs"
$outdoorAirDesignFlow = "crates\ep_runtime\src\ideal_loads\outdoor_air\design_flow.rs"
$outdoorAirDcv = "crates\ep_runtime\src\ideal_loads\outdoor_air\dcv.rs"
$outdoorAirMinimumFlow = "crates\ep_runtime\src\ideal_loads\outdoor_air\minimum_flow.rs"
$outdoorAirEconomizer = "crates\ep_runtime\src\ideal_loads\outdoor_air\economizer.rs"
$outdoorAirMixedAir = "crates\ep_runtime\src\ideal_loads\outdoor_air\mixed_air.rs"
$outdoorAirPsychrometrics = "crates\ep_runtime\src\ideal_loads\outdoor_air\psychrometrics.rs"
$outdoorAirSupply = "crates\ep_runtime\src\ideal_loads\outdoor_air\supply.rs"
$dispatch = "crates\ep_runtime\src\ideal_loads\dispatch.rs"
$idealLoadsCli = "crates\ep_cli\src\ideal_loads.rs"
$idealLoadsCliCaseAdapter = "crates\ep_cli\src\ideal_loads\case_adapter\mod.rs"
$idealLoadsCliTimeAxisAdapter = "crates\ep_cli\src\ideal_loads\case_adapter\time_axis.rs"
$idealLoadsCliTimeAxisAdapterTests = "crates\ep_cli\src\ideal_loads\case_adapter\time_axis_tests.rs"
$idealLoadsCliCommands = "crates\ep_cli\src\ideal_loads\commands\mod.rs"
$idealLoadsCliReports = "crates\ep_cli\src\ideal_loads\reports\mod.rs"
$idealLoadsCliOutdoorAirReports = "crates\ep_cli\src\ideal_loads\reports\outdoor_air\mod.rs"
$idealLoadsCliOutdoorAirMarkdown = "crates\ep_cli\src\ideal_loads\reports\outdoor_air\markdown.rs"
$idealLoadsCliOutdoorAirJson = "crates\ep_cli\src\ideal_loads\reports\outdoor_air\json.rs"
$idealLoadsCliOutdoorAirCsv = "crates\ep_cli\src\ideal_loads\reports\outdoor_air\csv.rs"
$idealLoadsCliPhysicsBoundaryFiles = @($idealLoadsCli)
$idealLoadsCliPhysicsBoundaryFiles += @(
    Get-ChildItem -LiteralPath "crates\ep_cli\src\ideal_loads" -Recurse -File -Filter "*.rs" |
        ForEach-Object { $_.FullName }
)
$outdoorAirSumCompare = "scripts\compare\compare-ideal-loads-outdoor-air-sum-conformance-candidate.ps1"
$outdoorAirMaximumCompare = "scripts\compare\compare-ideal-loads-outdoor-air-maximum-conformance-candidate.ps1"
$conformanceManifest = "crates\ep_conformance\src\conformance.rs"
$runtime = "crates\ep_runtime\src\runtime.rs"
$executionPlan = "crates\ep_runtime\src\execution_plan.rs"
$runSupport = "crates\ep_run\src\support.rs"
$runPipeline = "crates\ep_run\src\pipeline.rs"
$runPurchasedAirMinimumOa = "crates\ep_run\src\pipeline\purchased_air_minimum_oa.rs"
$runPurchasedAirCoolingEntryGate = "crates\ep_run\src\pipeline\purchased_air_cooling_entry_gate.rs"
$runPurchasedAirCoolingOaMaxFlow = "crates\ep_run\src\pipeline\purchased_air_cooling_oa_max_flow.rs"
$runPurchasedAirCoolingOaMaxFlowBody = "crates\ep_run\src\pipeline\purchased_air_cooling_oa_max_flow_body.rs"
$runPurchasedAirCoolingOaMaxFlowBodySerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_oa_max_flow_body\serialization.rs"
$runPurchasedAirCoolingEconomizerGuard = "crates\ep_run\src\pipeline\purchased_air_cooling_economizer_guard.rs"
$runPurchasedAirCoolingEconomizerCondition = "crates\ep_run\src\pipeline\purchased_air_cooling_economizer_condition.rs"
$runPurchasedAirCoolingEconomizerConditionSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_economizer_condition\serialization.rs"
$runPurchasedAirCoolingEconomizerBody = "crates\ep_run\src\pipeline\purchased_air_cooling_economizer_body.rs"
$runPurchasedAirCoolingEconomizerBodySerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_economizer_body\serialization.rs"
$runPurchasedAirCoolingEconomizerBodySnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_economizer_body\serialization\snapshot.rs"
$runPurchasedAirCoolingEconomizerBodyValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_economizer_body\validation.rs"
$runPurchasedAirCoolingSensibleFlow = "crates\ep_run\src\pipeline\purchased_air_cooling_sensible_flow.rs"
$runPurchasedAirCoolingSensibleFlowSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_sensible_flow\serialization.rs"
$runPurchasedAirCoolingSensibleFlowSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_sensible_flow\serialization\snapshot.rs"
$runPurchasedAirCoolingSensibleFlowValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_sensible_flow\validation.rs"
$runPurchasedAirCoolingDehumidificationFlow = "crates\ep_run\src\pipeline\purchased_air_cooling_dehumidification_flow.rs"
$runPurchasedAirCoolingDehumidificationFlowSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_dehumidification_flow\serialization.rs"
$runPurchasedAirCoolingDehumidificationFlowSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_dehumidification_flow\serialization\snapshot.rs"
$runPurchasedAirCoolingDehumidificationFlowValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_dehumidification_flow\validation.rs"
$runPurchasedAirCoolingDehumidificationFlowSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_dehumidification_flow\validation\snapshot.rs"
$runPurchasedAirCoolingHumidificationFlow = "crates\ep_run\src\pipeline\purchased_air_cooling_humidification_flow.rs"
$runPurchasedAirCoolingHumidificationFlowSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_humidification_flow\serialization.rs"
$runPurchasedAirCoolingHumidificationFlowSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_humidification_flow\serialization\snapshot.rs"
$runPurchasedAirCoolingHumidificationFlowValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_humidification_flow\validation.rs"
$runPurchasedAirCoolingHumidificationFlowSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_humidification_flow\validation\snapshot.rs"
$runDirectZoneCoupledTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$runRuntimeBoundaries = "crates\ep_run\src\support\runtime_boundaries.rs"
$runtimeOutputTests = "crates\ep_runtime\src\runtime\tests\part08.rs"
$lib = "crates\ep_runtime\src\lib.rs"
$output = "crates\ep_runtime\src\output.rs"
$resultStore = "crates\ep_runtime\src\output\result_store.rs"
$node = "crates\ep_runtime\src\node\mod.rs"
$nodeProjection = "crates\ep_runtime\src\node\projection.rs"
$nodeState = "crates\ep_runtime\src\node\state.rs"
$plant = "crates\ep_runtime\src\plant\mod.rs"
$plantState = "crates\ep_runtime\src\plant\state.rs"
$zoneEquipment = "crates\ep_runtime\src\zone_equipment\mod.rs"
$zoneEquipmentDemand = "crates\ep_runtime\src\zone_equipment\demand.rs"
$zoneEquipmentDispatch = "crates\ep_runtime\src\zone_equipment\dispatch.rs"
$zoneEquipmentTests = "crates\ep_runtime\src\zone_equipment\tests.rs"

Assert-FileExists -Path $calcRoot -Description "IdealLoads calc module root"
Assert-FileExists -Path $calcLifecycle -Description "PurchasedAir Calc-entry lifecycle module"
Assert-FileExists -Path $calcLifecycleTests -Description "PurchasedAir Calc-entry lifecycle tests"
Assert-FileExists -Path $calcMinimumOaPrefix -Description "PurchasedAir Calc minimum-OA prefix module"
Assert-FileExists -Path $calcMinimumOaPrefixTests -Description "PurchasedAir Calc minimum-OA prefix tests"
Assert-FileExists -Path $calcCoolingEntryGate -Description "PurchasedAir Calc cooling-entry gate module"
Assert-FileExists -Path $calcCoolingEntryGateRelease -Description "PurchasedAir Calc cooling-entry release boundary"
Assert-FileExists -Path $calcCoolingEntryGateTests -Description "PurchasedAir Calc cooling-entry characterization tests"
Assert-FileExists -Path $calcCoolingOaMaxFlowGate -Description "PurchasedAir Calc cooling OA maximum-flow gate module"
Assert-FileExists -Path $calcCoolingOaMaxFlowGateRelease -Description "PurchasedAir Calc cooling OA maximum-flow release boundary"
Assert-FileExists -Path $calcCoolingOaMaxFlowGateReleaseValidation -Description "PurchasedAir Calc cooling OA maximum-flow release validation helpers"
Assert-FileExists -Path $calcCoolingOaMaxFlowGateTests -Description "PurchasedAir Calc cooling OA maximum-flow characterization tests"
Assert-FileExists -Path $calcCoolingOaMaxFlowBody -Description "PurchasedAir Calc cooling OA maximum-flow true-body module"
Assert-FileExists -Path $calcCoolingOaMaxFlowBodyTransition -Description "PurchasedAir Calc cooling OA maximum-flow true-body transition"
Assert-FileExists -Path $calcCoolingOaMaxFlowBodyRelease -Description "PurchasedAir Calc cooling OA maximum-flow true-body release boundary"
Assert-FileExists -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Description "PurchasedAir Calc cooling OA maximum-flow true-body release validation helpers"
Assert-FileExists -Path $calcCoolingOaMaxFlowBodyTests -Description "PurchasedAir Calc cooling OA maximum-flow true-body characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerGuard -Description "PurchasedAir Calc cooling economizer guard module"
Assert-FileExists -Path $calcCoolingEconomizerGuardTransition -Description "PurchasedAir Calc cooling economizer guard transition"
Assert-FileExists -Path $calcCoolingEconomizerGuardRelease -Description "PurchasedAir Calc cooling economizer guard release boundary"
Assert-FileExists -Path $calcCoolingEconomizerGuardReleaseValidation -Description "PurchasedAir Calc cooling economizer guard release validation helpers"
Assert-FileExists -Path $calcCoolingEconomizerGuardTests -Description "PurchasedAir Calc cooling economizer guard characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerCondition -Description "PurchasedAir Calc cooling economizer condition module"
Assert-FileExists -Path $calcCoolingEconomizerConditionTransition -Description "PurchasedAir Calc cooling economizer condition transition"
Assert-FileExists -Path $calcCoolingEconomizerConditionRelease -Description "PurchasedAir Calc cooling economizer condition release boundary"
Assert-FileExists -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Description "PurchasedAir Calc cooling economizer condition retained entry-prefix validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionCompletedConditionValidation -Description "PurchasedAir Calc cooling economizer completed-condition validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionInitializationValidation -Description "PurchasedAir Calc cooling economizer condition retained initialization validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionPredecessorValidation -Description "PurchasedAir Calc cooling economizer condition predecessor validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionRuntimeValidation -Description "PurchasedAir Calc cooling economizer condition runtime validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionTests -Description "PurchasedAir Calc cooling economizer condition characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerConditionReleaseTests -Description "PurchasedAir Calc cooling economizer condition public release tests"
Assert-FileExists -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Description "PurchasedAir Calc cooling economizer condition provenance tests"
Assert-FileExists -Path $calcCoolingEconomizerConditionReleaseCorruptionTests -Description "PurchasedAir Calc cooling economizer condition corruption tests"
Assert-FileExists -Path $calcCoolingEconomizerBody -Description "PurchasedAir Calc cooling economizer true-body module"
Assert-FileExists -Path $calcCoolingEconomizerBodyState -Description "PurchasedAir Calc cooling economizer true-body persistent state"
Assert-FileExists -Path $calcCoolingEconomizerBodyTransition -Description "PurchasedAir Calc cooling economizer true-body transition"
Assert-FileExists -Path $calcCoolingEconomizerBodyRelease -Description "PurchasedAir Calc cooling economizer true-body release boundary"
Assert-FileExists -Path $calcCoolingEconomizerBodyEntryPrefixValidation -Description "PurchasedAir Calc cooling economizer true-body retained entry-prefix validation"
Assert-FileExists -Path $calcCoolingEconomizerBodyInitializationValidation -Description "PurchasedAir Calc cooling economizer true-body retained initialization validation"
Assert-FileExists -Path $calcCoolingEconomizerBodyPredecessorValidation -Description "PurchasedAir Calc cooling economizer true-body predecessor validation"
Assert-FileExists -Path $calcCoolingEconomizerBodyRuntimeValidation -Description "PurchasedAir Calc cooling economizer true-body runtime validation"
Assert-FileExists -Path $calcCoolingEconomizerBodyTests -Description "PurchasedAir Calc cooling economizer true-body characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerBodyGateAndAssignmentTests -Description "PurchasedAir Calc cooling economizer gate and assignment characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerBodyIeeeTests -Description "PurchasedAir Calc cooling economizer true-body IEEE characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerBodySkipTests -Description "PurchasedAir Calc cooling economizer true-body skip characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerBodySourceOrderTests -Description "PurchasedAir Calc cooling economizer true-body source-order characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerBodyReleaseTests -Description "PurchasedAir Calc cooling economizer true-body public release tests"
Assert-FileExists -Path $calcCoolingEconomizerBodyReleaseProvenanceTests -Description "PurchasedAir Calc cooling economizer true-body provenance tests"
Assert-FileExists -Path $calcCoolingEconomizerBodyReleaseCorruptionTests -Description "PurchasedAir Calc cooling economizer true-body corruption tests"
Assert-FileExists -Path $calcCoolingEconomizerBodyCompletedBodyValidation -Description "PurchasedAir Calc completed cooling economizer body validator for CP318"
Assert-FileExists -Path $calcCoolingSensibleFlow -Description "PurchasedAir Calc cooling sensible-flow module"
Assert-FileExists -Path $calcCoolingSensibleFlowState -Description "PurchasedAir Calc cooling sensible-flow persistent state"
Assert-FileExists -Path $calcCoolingSensibleFlowTransition -Description "PurchasedAir Calc cooling sensible-flow transition"
Assert-FileExists -Path $calcCoolingSensibleFlowRelease -Description "PurchasedAir Calc cooling sensible-flow release boundary"
Assert-FileExists -Path $calcCoolingSensibleFlowCompletedStateValidation -Description "PurchasedAir Calc completed cooling sensible-flow validator for CP319"
Assert-FileExists -Path $calcCoolingSensibleFlowPredecessorValidation -Description "PurchasedAir Calc cooling sensible-flow predecessor validation"
Assert-FileExists -Path $calcCoolingSensibleFlowRuntimeValidation -Description "PurchasedAir Calc cooling sensible-flow runtime validation"
Assert-FileExists -Path $calcCoolingSensibleFlowSnapshotValidation -Description "PurchasedAir Calc cooling sensible-flow exact snapshot validation"
Assert-FileExists -Path $calcCoolingSensibleFlowTests -Description "PurchasedAir Calc cooling sensible-flow characterization tests"
Assert-FileExists -Path $calcCoolingSensibleFlowSourceOrderTests -Description "PurchasedAir Calc cooling sensible-flow source-order tests"
Assert-FileExists -Path $calcCoolingSensibleFlowSkipTests -Description "PurchasedAir Calc cooling sensible-flow skip tests"
Assert-FileExists -Path $calcCoolingSensibleFlowGateTests -Description "PurchasedAir Calc cooling sensible-flow gate and IEEE tests"
Assert-FileExists -Path $calcCoolingSensibleFlowReleaseTests -Description "PurchasedAir Calc cooling sensible-flow public release tests"
Assert-FileExists -Path $calcCoolingSensibleFlowReleaseCorruptionTests -Description "PurchasedAir Calc cooling sensible-flow corruption tests"
Assert-FileExists -Path $calcCoolingDehumidificationFlow -Description "PurchasedAir Calc cooling dehumidification-flow module"
Assert-FileExists -Path $calcCoolingDehumidificationFlowState -Description "PurchasedAir Calc cooling dehumidification-flow persistent state"
Assert-FileExists -Path $calcCoolingDehumidificationFlowTransition -Description "PurchasedAir Calc cooling dehumidification-flow transition"
Assert-FileExists -Path $calcCoolingDehumidificationFlowRelease -Description "PurchasedAir Calc cooling dehumidification-flow release boundary"
Assert-FileExists -Path $calcCoolingDehumidificationFlowCompletedStateValidation -Description "PurchasedAir Calc completed cooling dehumidification-flow validator for CP320"
Assert-FileExists -Path $calcCoolingDehumidificationFlowPredecessorValidation -Description "PurchasedAir Calc cooling dehumidification-flow predecessor validation"
Assert-FileExists -Path $calcCoolingDehumidificationFlowRuntimeValidation -Description "PurchasedAir Calc cooling dehumidification-flow runtime validation"
Assert-FileExists -Path $calcCoolingDehumidificationFlowSnapshotValidation -Description "PurchasedAir Calc cooling dehumidification-flow exact snapshot validation"
Assert-FileExists -Path $calcCoolingDehumidificationFlowTests -Description "PurchasedAir Calc cooling dehumidification-flow characterization tests"
Assert-FileExists -Path $calcCoolingDehumidificationFlowSourceOrderTests -Description "PurchasedAir Calc cooling dehumidification-flow source-order tests"
Assert-FileExists -Path $calcCoolingDehumidificationFlowSkipTests -Description "PurchasedAir Calc cooling dehumidification-flow skip tests"
Assert-FileExists -Path $calcCoolingDehumidificationFlowGateTests -Description "PurchasedAir Calc cooling dehumidification-flow gate and IEEE tests"
Assert-FileExists -Path $calcCoolingDehumidificationFlowReleaseTests -Description "PurchasedAir Calc cooling dehumidification-flow public release tests"
Assert-FileExists -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Description "PurchasedAir Calc cooling dehumidification-flow corruption tests"
Assert-FileExists -Path $calcCoolingHumidificationFlow -Description "PurchasedAir Calc cooling humidification-flow module"
Assert-FileExists -Path $calcCoolingHumidificationFlowState -Description "PurchasedAir Calc cooling humidification-flow persistent state"
Assert-FileExists -Path $calcCoolingHumidificationFlowTransition -Description "PurchasedAir Calc cooling humidification-flow transition"
Assert-FileExists -Path $calcCoolingHumidificationFlowRelease -Description "PurchasedAir Calc cooling humidification-flow release boundary"
Assert-FileExists -Path $calcCoolingHumidificationFlowPredecessorValidation -Description "PurchasedAir Calc cooling humidification-flow predecessor validation"
Assert-FileExists -Path $calcCoolingHumidificationFlowRuntimeValidation -Description "PurchasedAir Calc cooling humidification-flow runtime validation"
Assert-FileExists -Path $calcCoolingHumidificationFlowSnapshotValidation -Description "PurchasedAir Calc cooling humidification-flow exact snapshot validation"
Assert-FileExists -Path $calcCoolingHumidificationFlowTests -Description "PurchasedAir Calc cooling humidification-flow characterization tests"
Assert-FileExists -Path $calcCoolingHumidificationFlowReleaseTests -Description "PurchasedAir Calc cooling humidification-flow public release tests"
Assert-FileExists -Path $calcHumidity -Description "IdealLoads calc humidity module"
Assert-FileExists -Path $calcLimits -Description "IdealLoads calc limits module"
Assert-FileExists -Path $calcMassFlow -Description "IdealLoads calc mass-flow module"
Assert-FileExists -Path $calcMoistureDemand -Description "IdealLoads ThirdOrder moisture-demand module"
Assert-FileExists -Path $calcMoistureDemandTests -Description "IdealLoads ThirdOrder moisture-demand tests"
Assert-FileExists -Path $noOaCalc -Description "IdealLoads no-OA calc module"
Assert-FileExists -Path $calcPsychrometrics -Description "IdealLoads calc psychrometrics module"
Assert-FileExists -Path $calcTypes -Description "IdealLoads calc shared types module"
Assert-FileExists -Path $noOaTests -Description "IdealLoads no-OA calc tests"
Assert-FileExists -Path $idealLoadsMod -Description "IdealLoads module root"
Assert-FileExists -Path $idealLoadsHumidistat -Description "IdealLoads Humidistat closed-loop transition module"
Assert-FileExists -Path $idealLoadsHumidistatTests -Description "IdealLoads Humidistat closed-loop transition tests"
Assert-FileExists -Path $idealLoadsSizing -Description "IdealLoads direct hard-size sizing module"
Assert-FileExists -Path $idealLoadsSizingTests -Description "IdealLoads direct hard-size sizing tests"
Assert-FileExists -Path $idealLoadsInit -Description "IdealLoads initialization module"
Assert-FileExists -Path $idealLoadsInitManagerPlan -Description "IdealLoads immutable initialization manager plan"
Assert-FileExists -Path $idealLoadsInitManagerPlanTests -Description "IdealLoads initialization manager-plan tests"
Assert-FileExists -Path $idealLoadsInitManagerScanTests -Description "IdealLoads manager-wide initialization sweep tests"
Assert-FileExists -Path $idealLoadsInitSummary -Description "IdealLoads initialization lifecycle summary"
Assert-FileExists -Path $idealLoadsInitState -Description "IdealLoads persistent initialization state"
Assert-FileExists -Path $idealLoadsInitWitnesses -Description "IdealLoads private calculation witness accessors"
Assert-FileExists -Path $idealLoadsInitSupplyTemperatureDiagnostic -Description "IdealLoads supply-temperature diagnostic registry"
Assert-FileExists -Path $idealLoadsInitSupplyTemperatureDiagnosticTests -Description "IdealLoads supply-temperature diagnostic tests"
Assert-FileExists -Path $idealLoadsInitTopologyPlan -Description "IdealLoads immutable selected-unit topology plan"
Assert-FileExists -Path $idealLoadsInitTopologyPlanTests -Description "IdealLoads selected-unit topology-plan tests"
Assert-FileExists -Path $idealLoadsInitTopologyTransition -Description "IdealLoads selected-unit topology transition"
Assert-FileExists -Path $idealLoadsInitTopologyTransitionTests -Description "IdealLoads selected-unit topology-transition tests"
Assert-FileExists -Path $idealLoadsInitTransition -Description "IdealLoads initialization transitions"
Assert-FileExists -Path $idealLoadsInitTests -Description "IdealLoads initialization tests"
Assert-FileExists -Path $idealLoadsInitWarningTests -Description "IdealLoads initialization warning tests"
Assert-FileExists -Path $idealLoadsBindingMinimumOaTests -Description "IdealLoads binding minimum-OA transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingEntryGateTests -Description "IdealLoads binding cooling-entry transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingOaMaxFlowGateTests -Description "IdealLoads binding cooling OA maximum-flow transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingOaMaxFlowBodyTests -Description "IdealLoads binding cooling OA maximum-flow true-body transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingEconomizerGuardTests -Description "IdealLoads binding cooling economizer guard transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingEconomizerGuardIntegrityTests -Description "IdealLoads binding cooling economizer guard retained-state integrity tests"
Assert-FileExists -Path $idealLoadsBindingCoolingEconomizerConditionTests -Description "IdealLoads binding cooling economizer condition transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingEconomizerConditionIntegrityTests -Description "IdealLoads binding cooling economizer condition retained-state integrity tests"
Assert-FileExists -Path $idealLoadsBindingCoolingEconomizerBodyTests -Description "IdealLoads binding cooling economizer true-body transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingEconomizerBodyIntegrityTests -Description "IdealLoads binding cooling economizer true-body retained-state integrity tests"
Assert-FileExists -Path $idealLoadsBindingCoolingSensibleFlowTests -Description "IdealLoads binding cooling sensible-flow transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingDehumidificationFlowTests -Description "IdealLoads binding cooling dehumidification-flow transaction tests"
Assert-FileExists -Path $idealLoadsBindingCoolingHumidificationFlowTests -Description "IdealLoads binding cooling humidification-flow transaction tests"
Assert-FileExists -Path $idealLoadsCoupledRuntime -Description "IdealLoads coupled release runtime"
Assert-FileExists -Path $idealLoadsCoupledMinimumOaValidation -Description "IdealLoads minimum-OA release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingEntryValidation -Description "IdealLoads cooling-entry release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Description "IdealLoads cooling OA maximum-flow release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Description "IdealLoads cooling OA maximum-flow true-body release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Description "IdealLoads cooling economizer guard release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Description "IdealLoads cooling economizer condition release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Description "IdealLoads cooling economizer true-body release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Description "IdealLoads cooling sensible-flow release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Description "IdealLoads cooling dehumidification-flow release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Description "IdealLoads cooling humidification-flow release validator"
Assert-FileExists -Path $idealLoadsCoupledOutputCoolingHumidificationFixture -Description "IdealLoads coupled-output cooling humidification-flow fixture"
Assert-FileExists -Path $idealLoadsCouplingValidation -Description "IdealLoads release coupling validation"
Assert-FileExists -Path $idealLoadsInput -Description "IdealLoads input boundary module"
Assert-FileExists -Path $idealLoadsMeters -Description "IdealLoads meter binding module"
Assert-FileExists -Path $idealLoadsReport -Description "IdealLoads report module"
Assert-FileExists -Path $idealLoadsReportTests -Description "IdealLoads report tests"
Assert-FileExists -Path $idealLoadsReportSemantics -Description "IdealLoads report semantics module"
Assert-FileExists -Path $idealLoadsRuntime -Description "IdealLoads compatibility runtime"
Assert-FileExists -Path $idealLoadsUpdate -Description "IdealLoads node-update module"
Assert-FileExists -Path $outdoorAir -Description "IdealLoads outdoor-air module"
Assert-FileExists -Path $outdoorAirTests -Description "IdealLoads outdoor-air tests module"
Assert-FileExists -Path $outdoorAirDcvTests -Description "IdealLoads outdoor-air DCV tests module"
Assert-FileExists -Path $outdoorAirWrapperTests -Description "IdealLoads outdoor-air wrapper tests module"
Assert-FileExists -Path $outdoorAirDesignFlow -Description "IdealLoads outdoor-air design-flow module"
Assert-FileExists -Path $outdoorAirDcv -Description "IdealLoads outdoor-air DCV module"
Assert-FileExists -Path $outdoorAirMinimumFlow -Description "IdealLoads minimum outdoor-air flow resolver module"
Assert-FileExists -Path $outdoorAirEconomizer -Description "IdealLoads outdoor-air economizer module"
Assert-FileExists -Path $outdoorAirMixedAir -Description "IdealLoads outdoor-air mixed-air module"
Assert-FileExists -Path $outdoorAirPsychrometrics -Description "IdealLoads outdoor-air psychrometrics module"
Assert-FileExists -Path $outdoorAirSupply -Description "IdealLoads outdoor-air supply module"
Assert-FileExists -Path $dispatch -Description "IdealLoads source-order dispatch module"
Assert-FileExists -Path $idealLoadsCli -Description "IdealLoads CLI report generator"
Assert-FileExists -Path $idealLoadsCliCaseAdapter -Description "IdealLoads CLI case-adapter module"
Assert-FileExists -Path $idealLoadsCliTimeAxisAdapter -Description "IdealLoads CLI time-axis adapter"
Assert-FileExists -Path $idealLoadsCliTimeAxisAdapterTests -Description "IdealLoads CLI time-axis adapter tests"
Assert-FileExists -Path $idealLoadsCliCommands -Description "IdealLoads CLI command entry points"
Assert-FileExists -Path $idealLoadsCliReports -Description "IdealLoads CLI report modules"
Assert-FileExists -Path $idealLoadsCliOutdoorAirReports -Description "IdealLoads CLI outdoor-air artifact writer"
Assert-FileExists -Path $idealLoadsCliOutdoorAirMarkdown -Description "IdealLoads CLI outdoor-air Markdown renderer"
Assert-FileExists -Path $idealLoadsCliOutdoorAirJson -Description "IdealLoads CLI outdoor-air JSON renderers"
Assert-FileExists -Path $idealLoadsCliOutdoorAirCsv -Description "IdealLoads CLI outdoor-air CSV renderers"
Assert-FileExists -Path $outdoorAirSumCompare -Description "IdealLoads outdoor-air Sum comparison script"
Assert-FileExists -Path $outdoorAirMaximumCompare -Description "IdealLoads outdoor-air Maximum comparison script"
Assert-FileExists -Path $conformanceManifest -Description "Conformance manifest schema"
Assert-FileExists -Path $runtime -Description "Runtime root"
Assert-FileExists -Path $executionPlan -Description "Runtime execution plan"
Assert-FileExists -Path $runSupport -Description "ep_run support assessment"
Assert-FileExists -Path $runPipeline -Description "ep_run pipeline"
Assert-FileExists -Path $runPurchasedAirMinimumOa -Description "ep_run PurchasedAir minimum-OA pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingEntryGate -Description "ep_run PurchasedAir cooling-entry pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingOaMaxFlow -Description "ep_run PurchasedAir cooling OA maximum-flow pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingOaMaxFlowBody -Description "ep_run PurchasedAir cooling OA maximum-flow true-body pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingOaMaxFlowBodySerialization -Description "ep_run PurchasedAir cooling OA maximum-flow true-body JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingEconomizerGuard -Description "ep_run PurchasedAir cooling economizer guard pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingEconomizerCondition -Description "ep_run PurchasedAir cooling economizer condition pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Description "ep_run PurchasedAir cooling economizer condition JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingEconomizerBody -Description "ep_run PurchasedAir cooling economizer true-body pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingEconomizerBodySerialization -Description "ep_run PurchasedAir cooling economizer true-body JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingEconomizerBodySnapshotSerialization -Description "ep_run PurchasedAir cooling economizer true-body snapshot JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingEconomizerBodyValidation -Description "ep_run PurchasedAir cooling economizer true-body exact release validator"
Assert-FileExists -Path $runPurchasedAirCoolingSensibleFlow -Description "ep_run PurchasedAir cooling sensible-flow pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingSensibleFlowSerialization -Description "ep_run PurchasedAir cooling sensible-flow JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingSensibleFlowSnapshotSerialization -Description "ep_run PurchasedAir cooling sensible-flow snapshot JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingSensibleFlowValidation -Description "ep_run PurchasedAir cooling sensible-flow exact release validator"
Assert-FileExists -Path $runPurchasedAirCoolingDehumidificationFlow -Description "ep_run PurchasedAir cooling dehumidification-flow pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingDehumidificationFlowSerialization -Description "ep_run PurchasedAir cooling dehumidification-flow JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotSerialization -Description "ep_run PurchasedAir cooling dehumidification-flow snapshot JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingDehumidificationFlowValidation -Description "ep_run PurchasedAir cooling dehumidification-flow exact release validator"
Assert-FileExists -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotValidation -Description "ep_run PurchasedAir cooling dehumidification-flow snapshot validator"
Assert-FileExists -Path $runPurchasedAirCoolingHumidificationFlow -Description "ep_run PurchasedAir cooling humidification-flow pipeline module"
Assert-FileExists -Path $runPurchasedAirCoolingHumidificationFlowSerialization -Description "ep_run PurchasedAir cooling humidification-flow JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingHumidificationFlowSnapshotSerialization -Description "ep_run PurchasedAir cooling humidification-flow snapshot JSON serializer"
Assert-FileExists -Path $runPurchasedAirCoolingHumidificationFlowValidation -Description "ep_run PurchasedAir cooling humidification-flow exact release validator"
Assert-FileExists -Path $runPurchasedAirCoolingHumidificationFlowSnapshotValidation -Description "ep_run PurchasedAir cooling humidification-flow snapshot validator"
Assert-FileExists -Path $runDirectZoneCoupledTests -Description "ep_run direct-Zone coupled integration tests"
Assert-FileExists -Path $runRuntimeBoundaries -Description "ep_run runtime boundary assessment"
Assert-FileExists -Path $runtimeOutputTests -Description "Runtime output registry tests"
Assert-FileExists -Path $lib -Description "Runtime crate facade"
Assert-FileExists -Path $output -Description "Runtime output registry"
Assert-FileExists -Path $resultStore -Description "Runtime result store"
Assert-FileExists -Path $node -Description "Node compatibility facade"
Assert-FileExists -Path $nodeProjection -Description "Node projection module"
Assert-FileExists -Path $nodeState -Description "Node state module"
Assert-FileExists -Path $plant -Description "Plant compatibility facade"
Assert-FileExists -Path $plantState -Description "Plant state module"
Assert-FileExists -Path $zoneEquipment -Description "Zone equipment compatibility facade"
Assert-FileExists -Path $zoneEquipmentDemand -Description "Zone equipment demand module"
Assert-FileExists -Path $zoneEquipmentDispatch -Description "Zone equipment dispatch module"
Assert-FileExists -Path $zoneEquipmentTests -Description "Zone equipment tests module"

Assert-LineLimit -Path $calcRoot -Limit 90 -Description "IdealLoads calc module root"
Assert-LineLimit -Path $calcLifecycle -Limit 520 -Description "PurchasedAir Calc-entry lifecycle module"
Assert-LineLimit -Path $calcLifecycleTests -Limit 240 -Description "PurchasedAir Calc-entry lifecycle tests"
Assert-LineLimit -Path $calcMinimumOaPrefix -Limit 380 -Description "PurchasedAir Calc minimum-OA prefix module"
Assert-LineLimit -Path $calcMinimumOaPrefixTests -Limit 220 -Description "PurchasedAir Calc minimum-OA prefix tests"
Assert-LineLimit -Path $calcCoolingEntryGate -Limit 280 -Description "PurchasedAir Calc cooling-entry gate module"
Assert-LineLimit -Path $calcCoolingEntryGateRelease -Limit 360 -Description "PurchasedAir Calc cooling-entry release boundary"
Assert-LineLimit -Path $calcCoolingEntryGateTests -Limit 300 -Description "PurchasedAir Calc cooling-entry characterization tests"
Assert-LineLimit -Path $calcCoolingOaMaxFlowGate -Limit 320 -Description "PurchasedAir Calc cooling OA maximum-flow gate module"
Assert-LineLimit -Path $calcCoolingOaMaxFlowGateRelease -Limit 380 -Description "PurchasedAir Calc cooling OA maximum-flow release boundary"
Assert-LineLimit -Path $calcCoolingOaMaxFlowGateReleaseValidation -Limit 240 -Description "PurchasedAir Calc cooling OA maximum-flow release validation helpers"
Assert-LineLimit -Path $calcCoolingOaMaxFlowGateTests -Limit 320 -Description "PurchasedAir Calc cooling OA maximum-flow characterization tests"
Assert-LineLimit -Path $calcCoolingOaMaxFlowBody -Limit 340 -Description "PurchasedAir Calc cooling OA maximum-flow true-body module"
Assert-LineLimit -Path $calcCoolingOaMaxFlowBodyTransition -Limit 220 -Description "PurchasedAir Calc cooling OA maximum-flow true-body transition"
Assert-LineLimit -Path $calcCoolingOaMaxFlowBodyRelease -Limit 380 -Description "PurchasedAir Calc cooling OA maximum-flow true-body release boundary"
Assert-LineLimit -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Limit 300 -Description "PurchasedAir Calc cooling OA maximum-flow true-body release validation helpers"
Assert-LineLimit -Path $calcCoolingOaMaxFlowBodyTests -Limit 340 -Description "PurchasedAir Calc cooling OA maximum-flow true-body characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerGuard -Limit 340 -Description "PurchasedAir Calc cooling economizer guard module"
Assert-LineLimit -Path $calcCoolingEconomizerGuardTransition -Limit 220 -Description "PurchasedAir Calc cooling economizer guard transition"
Assert-LineLimit -Path $calcCoolingEconomizerGuardRelease -Limit 380 -Description "PurchasedAir Calc cooling economizer guard release boundary"
Assert-LineLimit -Path $calcCoolingEconomizerGuardReleaseValidation -Limit 560 -Description "PurchasedAir Calc cooling economizer guard release validation helpers"
Assert-LineLimit -Path $calcCoolingEconomizerGuardTests -Limit 340 -Description "PurchasedAir Calc cooling economizer guard characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerCondition -Limit 300 -Description "PurchasedAir Calc cooling economizer condition module"
Assert-LineLimit -Path $calcCoolingEconomizerConditionTransition -Limit 240 -Description "PurchasedAir Calc cooling economizer condition transition"
Assert-LineLimit -Path $calcCoolingEconomizerConditionRelease -Limit 260 -Description "PurchasedAir Calc cooling economizer condition release boundary"
Assert-LineLimit -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Limit 360 -Description "PurchasedAir Calc cooling economizer condition retained entry-prefix validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionCompletedConditionValidation -Limit 100 -Description "PurchasedAir Calc cooling economizer completed-condition validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionInitializationValidation -Limit 120 -Description "PurchasedAir Calc cooling economizer condition retained initialization validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionPredecessorValidation -Limit 340 -Description "PurchasedAir Calc cooling economizer condition predecessor validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionRuntimeValidation -Limit 480 -Description "PurchasedAir Calc cooling economizer condition runtime validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionTests -Limit 340 -Description "PurchasedAir Calc cooling economizer condition characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerConditionReleaseTests -Limit 340 -Description "PurchasedAir Calc cooling economizer condition public release tests"
Assert-LineLimit -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Limit 160 -Description "PurchasedAir Calc cooling economizer condition provenance tests"
Assert-LineLimit -Path $calcCoolingEconomizerConditionReleaseCorruptionTests -Limit 260 -Description "PurchasedAir Calc cooling economizer condition corruption tests"
Assert-LineLimit -Path $calcCoolingEconomizerBody -Limit 420 -Description "PurchasedAir Calc cooling economizer true-body module"
Assert-LineLimit -Path $calcCoolingEconomizerBodyState -Limit 220 -Description "PurchasedAir Calc cooling economizer true-body persistent state"
Assert-LineLimit -Path $calcCoolingEconomizerBodyTransition -Limit 400 -Description "PurchasedAir Calc cooling economizer true-body transition"
Assert-LineLimit -Path $calcCoolingEconomizerBodyRelease -Limit 280 -Description "PurchasedAir Calc cooling economizer true-body release boundary"
Assert-LineLimit -Path $calcCoolingEconomizerBodyEntryPrefixValidation -Limit 360 -Description "PurchasedAir Calc cooling economizer true-body retained entry-prefix validation"
Assert-LineLimit -Path $calcCoolingEconomizerBodyInitializationValidation -Limit 120 -Description "PurchasedAir Calc cooling economizer true-body retained initialization validation"
Assert-LineLimit -Path $calcCoolingEconomizerBodyPredecessorValidation -Limit 420 -Description "PurchasedAir Calc cooling economizer true-body predecessor validation"
Assert-LineLimit -Path $calcCoolingEconomizerBodyRuntimeValidation -Limit 520 -Description "PurchasedAir Calc cooling economizer true-body runtime validation"
Assert-LineLimit -Path $calcCoolingEconomizerBodyTests -Limit 400 -Description "PurchasedAir Calc cooling economizer true-body characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodyGateAndAssignmentTests -Limit 220 -Description "PurchasedAir Calc cooling economizer gate and assignment characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodyIeeeTests -Limit 160 -Description "PurchasedAir Calc cooling economizer true-body IEEE characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodySkipTests -Limit 240 -Description "PurchasedAir Calc cooling economizer true-body skip characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodySourceOrderTests -Limit 340 -Description "PurchasedAir Calc cooling economizer true-body source-order characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodyReleaseTests -Limit 360 -Description "PurchasedAir Calc cooling economizer true-body public release tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodyReleaseProvenanceTests -Limit 200 -Description "PurchasedAir Calc cooling economizer true-body provenance tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodyReleaseCorruptionTests -Limit 280 -Description "PurchasedAir Calc cooling economizer true-body corruption tests"
Assert-LineLimit -Path $calcCoolingEconomizerBodyCompletedBodyValidation -Limit 180 -Description "PurchasedAir Calc completed cooling economizer body validator for CP318"
Assert-LineLimit -Path $calcCoolingSensibleFlow -Limit 420 -Description "PurchasedAir Calc cooling sensible-flow module"
Assert-LineLimit -Path $calcCoolingSensibleFlowState -Limit 220 -Description "PurchasedAir Calc cooling sensible-flow persistent state"
Assert-LineLimit -Path $calcCoolingSensibleFlowTransition -Limit 320 -Description "PurchasedAir Calc cooling sensible-flow transition"
Assert-LineLimit -Path $calcCoolingSensibleFlowRelease -Limit 380 -Description "PurchasedAir Calc cooling sensible-flow release boundary"
Assert-LineLimit -Path $calcCoolingSensibleFlowCompletedStateValidation -Limit 180 -Description "PurchasedAir Calc completed cooling sensible-flow validator for CP319"
Assert-LineLimit -Path $calcCoolingSensibleFlowPredecessorValidation -Limit 180 -Description "PurchasedAir Calc cooling sensible-flow predecessor validation"
Assert-LineLimit -Path $calcCoolingSensibleFlowRuntimeValidation -Limit 560 -Description "PurchasedAir Calc cooling sensible-flow runtime validation"
Assert-LineLimit -Path $calcCoolingSensibleFlowSnapshotValidation -Limit 360 -Description "PurchasedAir Calc cooling sensible-flow exact snapshot validation"
Assert-LineLimit -Path $calcCoolingSensibleFlowTests -Limit 400 -Description "PurchasedAir Calc cooling sensible-flow characterization tests"
Assert-LineLimit -Path $calcCoolingSensibleFlowSourceOrderTests -Limit 300 -Description "PurchasedAir Calc cooling sensible-flow source-order tests"
Assert-LineLimit -Path $calcCoolingSensibleFlowSkipTests -Limit 240 -Description "PurchasedAir Calc cooling sensible-flow skip tests"
Assert-LineLimit -Path $calcCoolingSensibleFlowGateTests -Limit 300 -Description "PurchasedAir Calc cooling sensible-flow gate and IEEE tests"
Assert-LineLimit -Path $calcCoolingSensibleFlowReleaseTests -Limit 420 -Description "PurchasedAir Calc cooling sensible-flow public release tests"
Assert-LineLimit -Path $calcCoolingSensibleFlowReleaseCorruptionTests -Limit 320 -Description "PurchasedAir Calc cooling sensible-flow corruption tests"
Assert-LineLimit -Path $calcCoolingDehumidificationFlow -Limit 460 -Description "PurchasedAir Calc cooling dehumidification-flow module"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowState -Limit 240 -Description "PurchasedAir Calc cooling dehumidification-flow persistent state"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowTransition -Limit 400 -Description "PurchasedAir Calc cooling dehumidification-flow transition"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowRelease -Limit 360 -Description "PurchasedAir Calc cooling dehumidification-flow release boundary"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowCompletedStateValidation -Limit 200 -Description "PurchasedAir Calc completed cooling dehumidification-flow validator for CP320"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowPredecessorValidation -Limit 240 -Description "PurchasedAir Calc cooling dehumidification-flow predecessor validation"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowRuntimeValidation -Limit 600 -Description "PurchasedAir Calc cooling dehumidification-flow runtime validation"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowSnapshotValidation -Limit 500 -Description "PurchasedAir Calc cooling dehumidification-flow exact snapshot validation"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowTests -Limit 420 -Description "PurchasedAir Calc cooling dehumidification-flow characterization tests"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowSourceOrderTests -Limit 320 -Description "PurchasedAir Calc cooling dehumidification-flow source-order tests"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowSkipTests -Limit 260 -Description "PurchasedAir Calc cooling dehumidification-flow skip tests"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowGateTests -Limit 360 -Description "PurchasedAir Calc cooling dehumidification-flow gate and IEEE tests"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowReleaseTests -Limit 460 -Description "PurchasedAir Calc cooling dehumidification-flow public release tests"
Assert-LineLimit -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Limit 360 -Description "PurchasedAir Calc cooling dehumidification-flow corruption tests"
Assert-LineLimit -Path $calcCoolingHumidificationFlow -Limit 500 -Description "PurchasedAir Calc cooling humidification-flow module"
Assert-LineLimit -Path $calcCoolingHumidificationFlowState -Limit 280 -Description "PurchasedAir Calc cooling humidification-flow persistent state"
Assert-LineLimit -Path $calcCoolingHumidificationFlowTransition -Limit 460 -Description "PurchasedAir Calc cooling humidification-flow transition"
Assert-LineLimit -Path $calcCoolingHumidificationFlowRelease -Limit 380 -Description "PurchasedAir Calc cooling humidification-flow release boundary"
Assert-LineLimit -Path $calcCoolingHumidificationFlowPredecessorValidation -Limit 260 -Description "PurchasedAir Calc cooling humidification-flow predecessor validation"
Assert-LineLimit -Path $calcCoolingHumidificationFlowRuntimeValidation -Limit 650 -Description "PurchasedAir Calc cooling humidification-flow runtime validation"
Assert-LineLimit -Path $calcCoolingHumidificationFlowSnapshotValidation -Limit 560 -Description "PurchasedAir Calc cooling humidification-flow exact snapshot validation"
Assert-LineLimit -Path $calcCoolingHumidificationFlowTests -Limit 440 -Description "PurchasedAir Calc cooling humidification-flow characterization tests"
Assert-LineLimit -Path $calcCoolingHumidificationFlowReleaseTests -Limit 500 -Description "PurchasedAir Calc cooling humidification-flow public release tests"
Assert-LineLimit -Path $calcHumidity -Limit 220 -Description "IdealLoads calc humidity module"
Assert-LineLimit -Path $calcLimits -Limit 180 -Description "IdealLoads calc limits module"
Assert-LineLimit -Path $calcMassFlow -Limit 150 -Description "IdealLoads calc mass-flow module"
Assert-LineLimit -Path $calcMoistureDemand -Limit 280 -Description "IdealLoads ThirdOrder moisture-demand module"
Assert-LineLimit -Path $calcMoistureDemandTests -Limit 240 -Description "IdealLoads ThirdOrder moisture-demand tests"
Assert-LineLimit -Path $noOaCalc -Limit 700 -Description "IdealLoads no-OA calc module"
Assert-LineLimit -Path $calcPsychrometrics -Limit 120 -Description "IdealLoads calc psychrometrics module"
Assert-LineLimit -Path $calcTypes -Limit 140 -Description "IdealLoads calc shared types module"
Assert-LineLimit -Path $noOaTests -Limit 650 -Description "IdealLoads no-OA calc tests"
Assert-LineLimit -Path $idealLoadsHumidistat -Limit 300 -Description "IdealLoads Humidistat closed-loop transition module"
Assert-LineLimit -Path $idealLoadsHumidistatTests -Limit 500 -Description "IdealLoads Humidistat closed-loop transition tests"
Assert-LineLimit -Path $idealLoadsSizing -Limit 380 -Description "IdealLoads direct hard-size sizing module"
Assert-LineLimit -Path $idealLoadsSizingTests -Limit 240 -Description "IdealLoads direct hard-size sizing tests"
Assert-LineLimit -Path $idealLoadsInit -Limit 120 -Description "IdealLoads initialization module"
Assert-LineLimit -Path $idealLoadsInitManagerPlan -Limit 140 -Description "IdealLoads immutable initialization manager plan"
Assert-LineLimit -Path $idealLoadsInitManagerPlanTests -Limit 250 -Description "IdealLoads initialization manager-plan tests"
Assert-LineLimit -Path $idealLoadsInitManagerScanTests -Limit 420 -Description "IdealLoads manager-wide initialization sweep tests"
Assert-LineLimit -Path $idealLoadsInitSummary -Limit 100 -Description "IdealLoads initialization lifecycle summary"
Assert-LineLimit -Path $idealLoadsInitState -Limit 380 -Description "IdealLoads persistent initialization state"
Assert-LineLimit -Path $idealLoadsInitWitnesses -Limit 241 -Description "IdealLoads private calculation witness accessors"
Assert-LineLimit -Path $idealLoadsInitSupplyTemperatureDiagnostic -Limit 340 -Description "IdealLoads supply-temperature diagnostic registry"
Assert-LineLimit -Path $idealLoadsInitSupplyTemperatureDiagnosticTests -Limit 340 -Description "IdealLoads supply-temperature diagnostic tests"
Assert-LineLimit -Path $idealLoadsInitTopologyPlan -Limit 480 -Description "IdealLoads immutable selected-unit topology plan"
Assert-LineLimit -Path $idealLoadsInitTopologyPlanTests -Limit 340 -Description "IdealLoads selected-unit topology-plan tests"
Assert-LineLimit -Path $idealLoadsInitTopologyTransition -Limit 100 -Description "IdealLoads selected-unit topology transition"
Assert-LineLimit -Path $idealLoadsInitTopologyTransitionTests -Limit 400 -Description "IdealLoads selected-unit topology-transition tests"
Assert-LineLimit -Path $idealLoadsInitTransition -Limit 540 -Description "IdealLoads initialization transitions"
Assert-LineLimit -Path $idealLoadsInitTests -Limit 380 -Description "IdealLoads initialization tests"
Assert-LineLimit -Path $idealLoadsInitWarningTests -Limit 100 -Description "IdealLoads initialization warning tests"
Assert-LineLimit -Path $idealLoadsBindingMinimumOaTests -Limit 180 -Description "IdealLoads binding minimum-OA transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingEntryGateTests -Limit 320 -Description "IdealLoads binding cooling-entry transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingOaMaxFlowGateTests -Limit 320 -Description "IdealLoads binding cooling OA maximum-flow transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingOaMaxFlowBodyTests -Limit 220 -Description "IdealLoads binding cooling OA maximum-flow true-body transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingEconomizerGuardTests -Limit 280 -Description "IdealLoads binding cooling economizer guard transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingEconomizerGuardIntegrityTests -Limit 220 -Description "IdealLoads binding cooling economizer guard retained-state integrity tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingEconomizerConditionTests -Limit 340 -Description "IdealLoads binding cooling economizer condition transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingEconomizerConditionIntegrityTests -Limit 260 -Description "IdealLoads binding cooling economizer condition retained-state integrity tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingEconomizerBodyTests -Limit 280 -Description "IdealLoads binding cooling economizer true-body transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingEconomizerBodyIntegrityTests -Limit 220 -Description "IdealLoads binding cooling economizer true-body retained-state integrity tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingSensibleFlowTests -Limit 320 -Description "IdealLoads binding cooling sensible-flow transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingDehumidificationFlowTests -Limit 360 -Description "IdealLoads binding cooling dehumidification-flow transaction tests"
Assert-LineLimit -Path $idealLoadsBindingCoolingHumidificationFlowTests -Limit 400 -Description "IdealLoads binding cooling humidification-flow transaction tests"
Assert-LineLimit -Path $idealLoadsCoupledMinimumOaValidation -Limit 240 -Description "IdealLoads minimum-OA release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingEntryValidation -Limit 240 -Description "IdealLoads cooling-entry release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Limit 280 -Description "IdealLoads cooling OA maximum-flow release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Limit 280 -Description "IdealLoads cooling OA maximum-flow true-body release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Limit 240 -Description "IdealLoads cooling economizer guard release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Limit 280 -Description "IdealLoads cooling economizer condition release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Limit 320 -Description "IdealLoads cooling economizer true-body release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Limit 360 -Description "IdealLoads cooling sensible-flow release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Limit 400 -Description "IdealLoads cooling dehumidification-flow release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Limit 440 -Description "IdealLoads cooling humidification-flow release validator"
Assert-LineLimit -Path $idealLoadsCoupledOutputCoolingHumidificationFixture -Limit 120 -Description "IdealLoads coupled-output cooling humidification-flow fixture"
Assert-LineLimit -Path $idealLoadsCouplingValidation -Limit 260 -Description "IdealLoads release coupling validation"
Assert-LineLimit -Path $idealLoadsInput -Limit 260 -Description "IdealLoads input boundary module"
Assert-LineLimit -Path $idealLoadsMeters -Limit 120 -Description "IdealLoads meter binding module"
Assert-LineLimit -Path $idealLoadsReportSemantics -Limit 80 -Description "IdealLoads report semantics module"
Assert-LineLimit -Path $idealLoadsReportTests -Limit 100 -Description "IdealLoads report tests"
Assert-LineLimit -Path $idealLoadsUpdate -Limit 120 -Description "IdealLoads node-update module"
Assert-LineLimit -Path $outdoorAir -Limit 450 -Description "IdealLoads outdoor-air module"
Assert-LineLimit -Path $outdoorAirTests -Limit 450 -Description "IdealLoads outdoor-air tests module"
Assert-LineLimit -Path $outdoorAirDcvTests -Limit 240 -Description "IdealLoads outdoor-air DCV tests module"
Assert-LineLimit -Path $outdoorAirWrapperTests -Limit 500 -Description "IdealLoads outdoor-air wrapper tests module"
Assert-LineLimit -Path $outdoorAirDesignFlow -Limit 220 -Description "IdealLoads outdoor-air design-flow module"
Assert-LineLimit -Path $outdoorAirDcv -Limit 120 -Description "IdealLoads outdoor-air DCV module"
Assert-LineLimit -Path $outdoorAirMinimumFlow -Limit 240 -Description "IdealLoads minimum outdoor-air flow resolver module"
Assert-LineLimit -Path $outdoorAirEconomizer -Limit 150 -Description "IdealLoads outdoor-air economizer module"
Assert-LineLimit -Path $outdoorAirMixedAir -Limit 260 -Description "IdealLoads outdoor-air mixed-air module"
Assert-LineLimit -Path $outdoorAirPsychrometrics -Limit 160 -Description "IdealLoads outdoor-air psychrometrics module"
Assert-LineLimit -Path $outdoorAirSupply -Limit 150 -Description "IdealLoads outdoor-air supply module"
Assert-LineLimit -Path $idealLoadsCliCaseAdapter -Limit 40 -Description "IdealLoads CLI case-adapter module"
Assert-LineLimit -Path $idealLoadsCliTimeAxisAdapter -Limit 160 -Description "IdealLoads CLI time-axis adapter"
Assert-LineLimit -Path $idealLoadsCliTimeAxisAdapterTests -Limit 160 -Description "IdealLoads CLI time-axis adapter tests"
Assert-LineLimit -Path $idealLoadsCliCommands -Limit 180 -Description "IdealLoads CLI command entry points"
Assert-LineLimit -Path $idealLoadsCliReports -Limit 40 -Description "IdealLoads CLI report module root"
Assert-LineLimit -Path $idealLoadsCliOutdoorAirReports -Limit 120 -Description "IdealLoads CLI outdoor-air artifact writer"
Assert-LineLimit -Path $idealLoadsCliOutdoorAirMarkdown -Limit 450 -Description "IdealLoads CLI outdoor-air Markdown renderer"
Assert-LineLimit -Path $idealLoadsCliOutdoorAirJson -Limit 1000 -Description "IdealLoads CLI outdoor-air JSON renderers"
Assert-LineLimit -Path $idealLoadsCliOutdoorAirCsv -Limit 120 -Description "IdealLoads CLI outdoor-air CSV renderers"
Assert-LineLimit -Path $nodeProjection -Limit 500 -Description "Node projection module"
Assert-LineLimit -Path $nodeState -Limit 220 -Description "Node state module"
Assert-LineLimit -Path $plantState -Limit 900 -Description "Plant state module"
Assert-LineLimit -Path $zoneEquipment -Limit 80 -Description "Zone equipment compatibility facade"
Assert-LineLimit -Path $zoneEquipmentDemand -Limit 120 -Description "Zone equipment demand module"
Assert-LineLimit -Path $zoneEquipmentDispatch -Limit 360 -Description "Zone equipment dispatch module"
Assert-LineLimit -Path $zoneEquipmentTests -Limit 320 -Description "Zone equipment tests module"
Assert-LineLimit -Path $resultStore -Limit 220 -Description "Runtime result store"
Assert-LineLimit -Path $runPurchasedAirMinimumOa -Limit 300 -Description "ep_run PurchasedAir minimum-OA pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingEntryGate -Limit 330 -Description "ep_run PurchasedAir cooling-entry pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingOaMaxFlow -Limit 440 -Description "ep_run PurchasedAir cooling OA maximum-flow pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingOaMaxFlowBody -Limit 420 -Description "ep_run PurchasedAir cooling OA maximum-flow true-body pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingOaMaxFlowBodySerialization -Limit 200 -Description "ep_run PurchasedAir cooling OA maximum-flow true-body JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingEconomizerGuard -Limit 420 -Description "ep_run PurchasedAir cooling economizer guard pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingEconomizerCondition -Limit 400 -Description "ep_run PurchasedAir cooling economizer condition pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Limit 220 -Description "ep_run PurchasedAir cooling economizer condition JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingEconomizerBody -Limit 460 -Description "ep_run PurchasedAir cooling economizer true-body pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingEconomizerBodySerialization -Limit 260 -Description "ep_run PurchasedAir cooling economizer true-body JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingEconomizerBodySnapshotSerialization -Limit 220 -Description "ep_run PurchasedAir cooling economizer true-body snapshot JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingEconomizerBodyValidation -Limit 320 -Description "ep_run PurchasedAir cooling economizer true-body exact release validator"
Assert-LineLimit -Path $runPurchasedAirCoolingSensibleFlow -Limit 460 -Description "ep_run PurchasedAir cooling sensible-flow pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingSensibleFlowSerialization -Limit 260 -Description "ep_run PurchasedAir cooling sensible-flow JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingSensibleFlowSnapshotSerialization -Limit 260 -Description "ep_run PurchasedAir cooling sensible-flow snapshot JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingSensibleFlowValidation -Limit 360 -Description "ep_run PurchasedAir cooling sensible-flow exact release validator"
Assert-LineLimit -Path $runPurchasedAirCoolingDehumidificationFlow -Limit 500 -Description "ep_run PurchasedAir cooling dehumidification-flow pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingDehumidificationFlowSerialization -Limit 320 -Description "ep_run PurchasedAir cooling dehumidification-flow JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotSerialization -Limit 320 -Description "ep_run PurchasedAir cooling dehumidification-flow snapshot JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingDehumidificationFlowValidation -Limit 400 -Description "ep_run PurchasedAir cooling dehumidification-flow exact release validator"
Assert-LineLimit -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotValidation -Limit 360 -Description "ep_run PurchasedAir cooling dehumidification-flow snapshot validator"
Assert-LineLimit -Path $runPurchasedAirCoolingHumidificationFlow -Limit 540 -Description "ep_run PurchasedAir cooling humidification-flow pipeline module"
Assert-LineLimit -Path $runPurchasedAirCoolingHumidificationFlowSerialization -Limit 360 -Description "ep_run PurchasedAir cooling humidification-flow JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingHumidificationFlowSnapshotSerialization -Limit 360 -Description "ep_run PurchasedAir cooling humidification-flow snapshot JSON serializer"
Assert-LineLimit -Path $runPurchasedAirCoolingHumidificationFlowValidation -Limit 440 -Description "ep_run PurchasedAir cooling humidification-flow exact release validator"
Assert-LineLimit -Path $runPurchasedAirCoolingHumidificationFlowSnapshotValidation -Limit 400 -Description "ep_run PurchasedAir cooling humidification-flow snapshot validator"

Assert-Contains -Path $calcRoot -Pattern 'mod humidity;' -Description "IdealLoads calc humidity submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_entry_gate;' -Description "PurchasedAir Calc cooling-entry gate submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_entry_gate_tests;' -Description "PurchasedAir Calc cooling-entry gate test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_entry_gate::\*;' -Description "PurchasedAir Calc cooling-entry gate public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_entry_gate\s*\(' -Description "cooling-entry transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_gate;' -Description "PurchasedAir Calc cooling OA maximum-flow gate submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_gate_tests;' -Description "PurchasedAir Calc cooling OA maximum-flow gate test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use (?:cooling_oa_max_flow_gate::\*;|\{[^}]*cooling_oa_max_flow_gate::\*)' -Description "PurchasedAir Calc cooling OA maximum-flow gate public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_oa_max_flow_gate\s*\(' -Description "cooling OA maximum-flow transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_body;' -Description "PurchasedAir Calc cooling OA maximum-flow true-body submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_body_tests;' -Description "PurchasedAir Calc cooling OA maximum-flow true-body test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use (?:cooling_oa_max_flow_body::\*;|\{[^}]*cooling_oa_max_flow_body::\*)' -Description "PurchasedAir Calc cooling OA maximum-flow true-body public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_oa_max_flow_body\s*\(' -Description "cooling OA maximum-flow true-body transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_guard;' -Description "PurchasedAir Calc cooling economizer guard submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_guard_tests;' -Description "PurchasedAir Calc cooling economizer guard test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_economizer_guard::\*;' -Description "PurchasedAir Calc cooling economizer guard public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_economizer_guard\s*\(' -Description "cooling economizer guard transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_condition;' -Description "PurchasedAir Calc cooling economizer condition submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_condition_tests;' -Description "PurchasedAir Calc cooling economizer condition test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_condition_release_tests;' -Description "PurchasedAir Calc cooling economizer condition release test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_economizer_condition::\*;' -Description "PurchasedAir Calc cooling economizer condition public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_economizer_condition\s*\(' -Description "cooling economizer condition transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_body;' -Description "PurchasedAir Calc cooling economizer true-body submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_body_tests;' -Description "PurchasedAir Calc cooling economizer true-body test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_economizer_body_release_tests;' -Description "PurchasedAir Calc cooling economizer true-body release test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_economizer_body::\*;' -Description "PurchasedAir Calc cooling economizer true-body public re-export"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_sensible_flow;' -Description "PurchasedAir Calc cooling sensible-flow submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_sensible_flow_tests;' -Description "PurchasedAir Calc cooling sensible-flow test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_sensible_flow_release_tests;' -Description "PurchasedAir Calc cooling sensible-flow release test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_sensible_flow::\*;' -Description "PurchasedAir Calc cooling sensible-flow public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_sensible_flow\s*\(' -Description "cooling sensible-flow transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_dehumidification_flow;' -Description "PurchasedAir Calc cooling dehumidification-flow submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_dehumidification_flow_tests;' -Description "PurchasedAir Calc cooling dehumidification-flow test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_dehumidification_flow_release_tests;' -Description "PurchasedAir Calc cooling dehumidification-flow release test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_dehumidification_flow::\*;' -Description "PurchasedAir Calc cooling dehumidification-flow public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_dehumidification_flow\s*\(' -Description "cooling dehumidification-flow transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_humidification_flow;' -Description "PurchasedAir Calc cooling humidification-flow submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_humidification_flow_tests;' -Description "PurchasedAir Calc cooling humidification-flow test module declaration"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern '#\[path = "cooling_humidification_flow_release_tests\.rs"\]\s*[\r\n]+\s*mod release_tests;' -Description "PurchasedAir Calc cooling humidification-flow nested release test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_humidification_flow::\*;' -Description "PurchasedAir Calc cooling humidification-flow public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_humidification_flow\s*\(' -Description "cooling humidification-flow transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod lifecycle;' -Description "PurchasedAir Calc-entry lifecycle submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod lifecycle_tests;' -Description "PurchasedAir Calc-entry lifecycle test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use (?:lifecycle::\*;|\{[^}]*lifecycle::\*)' -Description "PurchasedAir Calc-entry lifecycle public re-export"
Assert-Contains -Path $calcRoot -Pattern 'mod minimum_oa_prefix;' -Description "PurchasedAir Calc minimum-OA prefix submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod minimum_oa_prefix_tests;' -Description "PurchasedAir Calc minimum-OA prefix test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use (?:minimum_oa_prefix::\*;|\{[^}]*minimum_oa_prefix::\*)' -Description "PurchasedAir Calc minimum-OA prefix public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_minimum_oa_prefix\s*\(' -Description "minimum-OA transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod limits;' -Description "IdealLoads calc limits submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod mass_flow;' -Description "IdealLoads calc mass-flow submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod no_oa;' -Description "no-OA calc submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod psychrometrics;' -Description "IdealLoads calc psychrometrics submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod types;' -Description "IdealLoads calc shared types submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use limits::IdealLoadsSensibleLimitContext;' -Description "IdealLoads calc limits public re-export"
Assert-Contains -Path $calcRoot -Pattern 'pub use (?:no_oa::\*;|\{[^}]*no_oa::\*)' -Description "no-OA calc public re-export"
Assert-Contains -Path $calcRoot -Pattern 'pub use psychrometrics::' -Description "IdealLoads calc psychrometrics public re-export"
Assert-Contains -Path $calcRoot -Pattern 'pub use types::\*;' -Description "IdealLoads calc shared types public re-export"
Assert-Contains -Path $calcRoot -Pattern 'mod no_oa_tests;' -Description "no-OA calc test module declaration"
Assert-NotContains -Path $calcRoot -Pattern 'calc_no_oa_no_limit_sensible_compat\s*\(' -Description "branch formula in calc module root"
Assert-NotContains -Path $calcRoot -Pattern 'fn heating_result_with_limits\s*\(' -Description "heating branch helper in calc module root"
Assert-NotContains -Path $calcRoot -Pattern 'fn cooling_result_with_limits\s*\(' -Description "cooling branch helper in calc module root"

Assert-Contains -Path $idealLoadsMod -Pattern 'mod init;' -Description "IdealLoads init submodule declaration"
Assert-Contains -Path $idealLoadsMod -Pattern 'mod sizing;' -Description "IdealLoads direct hard-size sizing submodule declaration"
Assert-Contains -Path $idealLoadsMod -Pattern 'mod sizing_tests;' -Description "IdealLoads direct hard-size sizing test module"
Assert-Contains -Path $idealLoadsMod -Pattern 'mod update;' -Description "IdealLoads update submodule declaration"
Assert-Contains -Path $idealLoadsMod -Pattern 'pub use init::\*;' -Description "IdealLoads init public re-export"
Assert-Contains -Path $idealLoadsMod -Pattern 'pub use sizing::\*;' -Description "IdealLoads direct hard-size sizing public re-export"
Assert-Contains -Path $idealLoadsMod -Pattern 'pub use update::\*;' -Description "IdealLoads update public re-export"

Assert-Contains -Path $idealLoadsSizing -Pattern 'pub struct PurchasedAirSizedLimits' -Description "runtime-owned four-field PurchasedAir sizing overlay"
Assert-Contains -Path $idealLoadsSizing -Pattern 'pub struct PurchasedAirHardSizeLegacyOutcome' -Description "direct hard-size sizing outcome"
Assert-Contains -Path $idealLoadsSizing -Pattern 'pub fn size_purchased_air_direct_hard_sized_legacy_route\s*\(' -Description "direct hard-size SizePurchasedAir child"
Assert-Contains -Path $idealLoadsSizing -Pattern 'NoCurrentZoneEquipment' -Description "current Zone-equipment source branch"
Assert-Contains -Path $idealLoadsSizing -Pattern 'CustomZoneHvacSizingNotImplemented' -Description "custom ZoneHVAC sizing guard"
Assert-Contains -Path $idealLoadsSizing -Pattern 'ZoneSizingRunNotImplemented' -Description "Zone-sizing-run guard"
Assert-Contains -Path $idealLoadsSizing -Pattern 'AutosizingNotImplemented' -Description "Autosize guard"
Assert-Contains -Path $idealLoadsSizing -Pattern 'MaximumHeatingAirFlowRate[\s\S]*MaximumSensibleHeatingCapacity[\s\S]*MaximumCoolingAirFlowRate[\s\S]*MaximumTotalCoolingCapacity' -Description "four-field source-order sizing outcome"
Assert-Contains -Path $idealLoadsSizingTests -Pattern 'positive_no_limit_fields_call_all_children_in_source_order' -Description "inactive-limit positive child-call regression"
Assert-Contains -Path $idealLoadsSizingTests -Pattern 'zero_and_blank_fields_skip_children_and_small_heat_capacity_outer_report' -Description "zero/blank and heating-capacity report regression"
Assert-Contains -Path $idealLoadsSizingTests -Pattern 'missing_current_zone_equipment_suppresses_the_entire_field_body' -Description "missing current Zone-equipment suppression regression"
Assert-Contains -Path $idealLoadsSizingTests -Pattern 'unported_routes_and_unresolved_values_fail_closed' -Description "unported sizing guard regression"

Assert-Contains -Path $noOaCalc -Pattern 'pub fn calc_no_oa_no_limit_sensible_compat\s*\(' -Description "no-OA/no-limit sensible calc"
Assert-Contains -Path $noOaCalc -Pattern 'pub fn calc_no_oa_sensible_with_limits_compat\s*\(' -Description "finite-limit sensible calc"
Assert-Contains -Path $noOaCalc -Pattern 'fn heating_result_with_limits\s*\(' -Description "heating branch helper"
Assert-Contains -Path $noOaCalc -Pattern 'fn cooling_result_with_limits\s*\(' -Description "cooling branch helper"
Assert-Contains -Path $calcHumidity -Pattern 'fn humidistat_dehumidification_mass_flow_rate_kg_per_s\s*\(' -Description "dehumidification diagnostic branch helper"
Assert-Contains -Path $calcHumidity -Pattern 'fn humidistat_humidification_mass_flow_rate_kg_per_s\s*\(' -Description "humidification diagnostic branch helper"
Assert-Contains -Path $calcHumidity -Pattern 'fn heating_supply_humidity_ratio\s*\(' -Description "heating supply humidity helper"
Assert-Contains -Path $calcHumidity -Pattern 'fn cooling_supply_humidity_ratio\s*\(' -Description "cooling supply humidity helper"
Assert-Contains -Path $calcLimits -Pattern 'pub struct IdealLoadsSensibleLimitContext' -Description "IdealLoads sensible limit context"
Assert-Contains -Path $calcLimits -Pattern 'fn flow_limit_kg_per_s\s*\(' -Description "IdealLoads flow limit helper"
Assert-Contains -Path $calcLimits -Pattern 'fn capacity_limit_w\s*\(' -Description "IdealLoads capacity limit helper"
Assert-Contains -Path $calcMassFlow -Pattern 'fn limited_heating_mass_flow_rate_kg_per_s\s*\(' -Description "IdealLoads limited heating mass-flow helper"
Assert-Contains -Path $calcMassFlow -Pattern 'fn limited_cooling_mass_flow_rate_kg_per_s\s*\(' -Description "IdealLoads limited cooling mass-flow helper"
Assert-Contains -Path $calcPsychrometrics -Pattern 'pub fn moist_air_enthalpy_j_per_kg\s*\(' -Description "IdealLoads moist-air enthalpy helper"
Assert-Contains -Path $calcPsychrometrics -Pattern 'pub fn energyplus_standard_air_density_kg_per_m3\s*\(' -Description "IdealLoads standard air density helper"
Assert-Contains -Path $calcPsychrometrics -Pattern 'fn humidity_ratio_from_enthalpy_and_dry_bulb\s*\(' -Description "IdealLoads humidity-from-enthalpy helper"
Assert-Contains -Path $calcTypes -Pattern 'pub enum IdealLoadsSensibleMode' -Description "IdealLoads sensible mode type"
Assert-Contains -Path $calcTypes -Pattern 'pub struct IdealLoadsZoneState' -Description "IdealLoads zone state type"
Assert-Contains -Path $calcTypes -Pattern 'pub struct IdealLoadsSensibleResult' -Description "IdealLoads sensible result type"
Assert-NotContains -Path $noOaCalc -Pattern '#\[test\]' -Description "unit tests in no-OA implementation module"
Assert-NotContains -Path $noOaCalc -Pattern 'pub fn moist_air_enthalpy_j_per_kg\s*\(' -Description "moist-air enthalpy helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'pub fn energyplus_standard_air_density_kg_per_m3\s*\(' -Description "standard air density helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn humidity_ratio_from_enthalpy_and_dry_bulb\s*\(' -Description "humidity-from-enthalpy helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'pub struct IdealLoadsSensibleLimitContext' -Description "limit context in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'pub enum IdealLoadsSensibleMode' -Description "sensible mode type in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'pub struct IdealLoadsZoneState' -Description "zone state type in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'pub struct IdealLoadsSensibleResult' -Description "sensible result type in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn humidistat_dehumidification_mass_flow_rate_kg_per_s\s*\(' -Description "dehumidification diagnostic helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn humidistat_humidification_mass_flow_rate_kg_per_s\s*\(' -Description "humidification diagnostic helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn heating_supply_humidity_ratio\s*\(' -Description "heating supply humidity helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn cooling_supply_humidity_ratio\s*\(' -Description "cooling supply humidity helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn limited_heating_mass_flow_rate_kg_per_s\s*\(' -Description "limited heating mass-flow helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn limited_cooling_mass_flow_rate_kg_per_s\s*\(' -Description "limited cooling mass-flow helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn flow_limit_kg_per_s\s*\(' -Description "flow limit helper in no-OA calc module"
Assert-NotContains -Path $noOaCalc -Pattern 'fn capacity_limit_w\s*\(' -Description "capacity limit helper in no-OA calc module"
Assert-Contains -Path $noOaTests -Pattern '#\[test\]' -Description "unit tests in no-OA test module"

Assert-Contains -Path $calcLifecycle -Pattern 'pub const PURCHASED_AIR_CALC_ENTRY_SOURCE' -Description "Calc-entry EnergyPlus source provenance"
Assert-Contains -Path $calcLifecycle -Pattern 'pub const PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER' -Description "Calc-entry exact source order"
Assert-Contains -Path $calcLifecycle -Pattern 'pub const PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS' -Description "Calc-entry 12-target reset evidence"
Assert-Contains -Path $calcLifecycle -Pattern 'PurchasedAirManager\.cc:1967,1971-2022' -Description "Calc-entry alias and executable-prefix provenance"
Assert-ExactStringArray -Path $calcLifecycle -Name "PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER" -Expected @(
    "resolve-supply-node",
    "resolve-zone-node",
    "resolve-outdoor-air-node",
    "resolve-recirculation-node",
    "reset-12-entry-values",
    "default-unit-on",
    "default-economizer-off",
    "read-heating-setpoint-demand",
    "read-cooling-setpoint-demand",
    "availability-manager-zone-write-if-allocated",
    "availability-manager-status-copy-if-allocated",
    "availability-manager-force-off-check-if-allocated",
    "read-overall-availability",
    "default-heating-on",
    "read-heating-availability",
    "default-cooling-on",
    "read-cooling-availability",
    "gate-unit-body"
) -Description "Calc-entry exact 18-step source order"
Assert-ExactStringArray -Path $calcLifecycle -Name "PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS" -Expected @(
    "SupplyMassFlowRate",
    "OAMassFlowRate",
    "PurchAir.MinOAMassFlowRate",
    "PurchAir.TimeEconoActive",
    "PurchAir.TimeHtRecActive",
    "SysOutputProvided",
    "MoistOutputProvided",
    "CoolSensOutput",
    "CoolLatOutput",
    "CoolTotOutput",
    "HeatSensOutput",
    "LatOutput"
) -Description "Calc-entry exact ordered 12-target reset snapshot"
Assert-NotContains -Path $calcLifecycle -Pattern '"SensOutput"' -Description "non-source bare SensOutput reset target"
Assert-Contains -Path $calcLifecycle -Pattern 'PurchAir\.MinOAMassFlowRate' -Description "Calc-entry retained minimum OA reset target"
Assert-Contains -Path $calcLifecycle -Pattern 'PurchAir\.TimeEconoActive' -Description "Calc-entry retained economizer-time reset target"
Assert-Contains -Path $calcLifecycle -Pattern 'PurchAir\.TimeHtRecActive' -Description "Calc-entry retained heat-recovery-time reset target"
Assert-Contains -Path $calcLifecycle -Pattern 'pub enum PurchasedAirAvailabilityStatus' -Description "Calc-entry source availability status"
Assert-Contains -Path $calcLifecycle -Pattern 'pub struct PurchasedAirCalcEntryDemandSnapshot' -Description "Calc-entry narrow sensible-demand snapshot"
Assert-Contains -Path $calcLifecycle -Pattern 'pub struct PurchasedAirCalcEntryContext' -Description "Calc-entry call context"
Assert-Contains -Path $calcLifecycle -Pattern 'pub struct PurchasedAirCalcEntrySnapshot' -Description "Calc-entry source-ordered snapshot"
Assert-Contains -Path $calcLifecycle -Pattern 'pub struct PurchasedAirCalcEntryRuntimeState' -Description "Calc-entry bounded persistent state"
Assert-Contains -Path $calcLifecycle -Pattern 'pub struct PurchasedAirCalcEntryLifecycleSummary' -Description "Calc-entry lifecycle summary"
Assert-Contains -Path $calcLifecycle -Pattern 'pub enum PurchasedAirCalcEntryError' -Description "Calc-entry fail-closed error"
Assert-Contains -Path $calcLifecycle -Pattern 'pub fn advance_purchased_air_calc_entry\s*\(' -Description "Calc-entry persistent transition"
Assert-Contains -Path $calcLifecycle -Pattern 'pub fn purchased_air_calc_entry_lifecycle_summary\s*\(' -Description "Calc-entry persistent summary"
Assert-Contains -Path $calcLifecycle -Pattern 'checked_add\(1\) != Some\(unit\.init_call_count\)' -Description "Init-to-Calc entry lockstep guard"
Assert-Contains -Path $calcLifecycle -Pattern 'state\.minimum_outdoor_air_mass_flow_rate_kg_per_s = 0\.0' -Description "retained minimum OA reset mutation"
Assert-Contains -Path $calcLifecycle -Pattern 'state\.economizer_active_time_hours = 0\.0' -Description "retained economizer-time reset mutation"
Assert-Contains -Path $calcLifecycle -Pattern 'state\.heat_recovery_active_time_hours = 0\.0' -Description "retained heat-recovery-time reset mutation"
Assert-Contains -Path $calcLifecycle -Pattern 'value > 0\.0 \|\| value\.is_nan\(\)' -Description "source <=0 off and NaN-on predicate"
Assert-Contains -Path $calcLifecycleTests -Pattern 'entry_resets_then_reads_demand_manager_and_all_schedules' -Description "Calc-entry reset, demand, manager, and schedule regression"
Assert-Contains -Path $calcLifecycleTests -Pattern 'schedule_gates_are_independent_and_nan_is_nominally_on' -Description "Calc-entry independent gates and NaN regression"
Assert-Contains -Path $calcLifecycleTests -Pattern 'direct_entry_retains_mismatched_zone_and_aliased_nodes_without_validation' -Description "Calc-entry direct source-characterization regression"
Assert-Contains -Path $calcLifecycleTests -Pattern 'unknown_public_unit_rejects_advance_and_summary_without_mutation' -Description "Calc-entry unknown-unit transaction regression"

Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub const PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE' -Description "Calc minimum-OA parent source provenance"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub const PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE' -Description "Calc minimum-OA child source provenance"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub const PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER' -Description "Calc minimum-OA exact source order"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2023-2040' -Description "Calc minimum-OA exact parent boundary"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2762-2810; bounded no-OA route 2781,2783,2785,2806-2809' -Description "Calc minimum-OA exact bounded child boundary"
Assert-ExactStringArray -Path $calcMinimumOaPrefix -Name "PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER" -Expected @(
    "resolve-zone-heat-balance-reference",
    "call-calc-purch-air-min-oa-mass-flow",
    "child-zero-no-outdoor-air-working-flow",
    "child-write-retained-minimum-outdoor-air-flow",
    "read-ems-outdoor-air-override-flag",
    "apply-ems-outdoor-air-flow-if-enabled",
    "read-outdoor-air-enabled",
    "calculate-outdoor-air-specific-heat-if-enabled",
    "calculate-or-zero-minimum-outdoor-air-sensible-output",
    "calculate-or-zero-minimum-outdoor-air-moisture-output"
) -Description "Calc minimum-OA exact 10-step parent and bounded-child source order"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub struct PurchasedAirCalcMinimumOaPrefixSnapshot' -Description "Calc minimum-OA source-ordered snapshot"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub struct PurchasedAirCalcMinimumOaPrefixRuntimeState' -Description "Calc minimum-OA bounded persistent state"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub struct PurchasedAirCalcMinimumOaPrefixLifecycleSummary' -Description "Calc minimum-OA lifecycle summary"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub enum PurchasedAirCalcMinimumOaPrefixError' -Description "Calc minimum-OA fail-closed error"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub fn advance_direct_no_oa_calc_minimum_oa_prefix\s*\(' -Description "Calc minimum-OA release transition"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub fn purchased_air_calc_minimum_oa_prefix_lifecycle_summary\s*\(' -Description "Calc minimum-OA lifecycle summary accessor"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'pub\(super\) fn advance_minimum_oa_prefix_state\s*\(' -Description "Calc minimum-OA direct characterization transition"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'CalculationEntryCallOrder' -Description "CP310-to-CP311 one-for-one source-order guard"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'CalculationEntrySnapshotMismatch' -Description "CP310-to-CP311 retained snapshot guard"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'calculation_entry_snapshots_bitwise_equal' -Description "CP310 NaN-compatible exact snapshot guard"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'OutdoorAirOutsideBoundedSubset' -Description "active outdoor-air release rejection"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'calculation_entry\.outdoor_air_node\.is_none\(\)' -Description "release absent outdoor-air node guard"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'design_specification_outdoor_air_object_name\s*\.is_none\(\)' -Description "release absent DSOA binding guard"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'outdoor_air_inlet_node_name\.is_none\(\)' -Description "release absent OA inlet declaration guard"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'ems_override_enabled: body_entered\.then_some\(false\)' -Description "release EMS override disabled snapshot"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'ems_override_applied: false' -Description "release EMS override application forbidden"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'outdoor_air_enabled: body_entered\.then_some\(false\)' -Description "release outdoor-air branch disabled snapshot"
Assert-Contains -Path $calcMinimumOaPrefix -Pattern 'psychrometric_call_count: 0' -Description "release psychrometric branch not executed"
Assert-Contains -Path $calcMinimumOaPrefixTests -Pattern 'active_no_oa_prefix_rewrites_retained_minimum_and_zeros_both_effects' -Description "Calc minimum-OA active no-OA regression"
Assert-Contains -Path $calcMinimumOaPrefixTests -Pattern 'unit_off_skips_child_ems_predicate_and_outdoor_air_branch' -Description "Calc minimum-OA UnitOff skip regression"
Assert-Contains -Path $calcMinimumOaPrefixTests -Pattern 'heat_and_cool_off_do_not_block_the_unit_on_prefix' -Description "Calc minimum-OA independent heat/cool gate regression"

$minimumOaForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'PurchasedAirManager\.cc:2046'; Description = "line-2046-and-later Calc provenance in minimum-OA boundary" },
    [pscustomobject]@{ Pattern = 'IdealLoadsSensibleMode|OperatingMode|TempControlType|SingleHeat|SingleCool'; Description = "line-2046-and-later operating-mode behavior in minimum-OA boundary" },
    [pscustomobject]@{ Pattern = 'CalcPurchAirMixedAir|calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s'; Description = "later Calc mixed-air or economizer behavior in minimum-OA boundary" },
    [pscustomobject]@{ Pattern = 'resolve_minimum_outdoor_air_compat\s*\(|IdealLoadsMinimumOutdoorAirCompatInput|calcDesignSpecificationOutdoorAir'; Description = "active DSOA child execution in minimum-OA boundary" },
    [pscustomobject]@{ Pattern = 'DemandControlledVentilationType|OccupancySchedule|Co2Setpoint|CO2Setpoint'; Description = "active DCV selector behavior in minimum-OA boundary" },
    [pscustomobject]@{ Pattern = 'current_people_count|co2_setpoint_required_mass_flow_rate_kg_per_s'; Description = "active occupancy or CO2 signal consumption in minimum-OA boundary" },
    [pscustomobject]@{ Pattern = 'PsyCpAirFnW\s*\(|energyplus_moist_air_specific_heat_j_per_kg_k\s*\('; Description = "active outdoor-air psychrometric calculation in minimum-OA boundary" }
)
foreach ($minimumOaBoundaryFile in @(
        $calcMinimumOaPrefix,
        $idealLoadsCoupledMinimumOaValidation,
        $runPurchasedAirMinimumOa
    )) {
    foreach ($forbiddenBehavior in $minimumOaForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $minimumOaBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
Assert-NotContains -Path $calcMinimumOaPrefix -Pattern 'state\.(ems_override_apply_count|outdoor_air_effect_count|psychrometric_call_count)\s*\+=' -Description "active OA or EMS counter mutation in no-OA release transition"

Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE' -Description "Calc cooling-entry gate source provenance"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling-entry first excluded source"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER' -Description "Calc cooling-entry exact source order"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2046-2047' -Description "Calc cooling-entry exact parent boundary"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2056' -Description "Calc cooling-entry exact first excluded executable"
Assert-ExactStringArray -Path $calcCoolingEntryGate -Name "PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER" -Expected @(
    "read-minimum-outdoor-air-sensible-output",
    "read-cooling-setpoint-demand",
    "compare-inclusive-greater-equal",
    "read-zone-temperature-control-type-after-short-circuit",
    "exclude-exact-single-heating-control",
    "assign-cooling-operating-mode-if-admitted"
) -Description "Calc cooling-entry exact six-step source order"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub enum PurchasedAirTemperatureControlType' -Description "Calc cooling-entry source temperature-control type"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub struct PurchasedAirCalcCoolingEntryGateSnapshot' -Description "Calc cooling-entry source-ordered snapshot"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub struct PurchasedAirCalcCoolingEntryGateRuntimeState' -Description "Calc cooling-entry bounded persistent state"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub struct PurchasedAirCalcCoolingEntryGateLifecycleSummary' -Description "Calc cooling-entry lifecycle summary"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub fn purchased_air_calc_cooling_entry_gate_lifecycle_summary\s*\(' -Description "Calc cooling-entry lifecycle summary accessor"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'pub\(super\) fn advance_cooling_entry_gate_state\s*\(' -Description "Calc cooling-entry source-characterization transition"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'minimum_oa_sensible_output_w >= cooling_setpoint_demand_w' -Description "Calc cooling-entry inclusive sensible comparison"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'sensible_comparison_satisfied == Some\(true\)' -Description "Calc cooling-entry thermostat-read short circuit"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'value != PurchasedAirTemperatureControlType::SingleHeat' -Description "Calc cooling-entry exact SingleHeat exclusion"
Assert-Contains -Path $calcCoolingEntryGate -Pattern 'cooling_body_entered\.then_some\(IdealLoadsSensibleMode::Cooling\)' -Description "Calc cooling-entry local Cooling assignment"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'pub enum PurchasedAirCalcCoolingEntryGatePredicateInput' -Description "Calc cooling-entry finite-input identity"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'pub enum PurchasedAirCalcCoolingEntryGateError' -Description "Calc cooling-entry fail-closed error"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'pub fn advance_direct_no_oa_calc_cooling_entry_gate\s*\(' -Description "Calc cooling-entry release transition"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'CalculationEntrySnapshotMismatch' -Description "CP310-to-CP312 retained snapshot guard"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'MinimumOaPrefixSnapshotMismatch' -Description "CP311-to-CP312 retained snapshot guard"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'PredecessorLinkMismatch' -Description "CP310/CP311 predecessor-link guard"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'PredecessorCallOrder' -Description "CP310/CP311/CP312 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'MinimumOaPrefixOutsideDirectSubset' -Description "CP312 no-OA/no-EMS predecessor-shape guard"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'TemperatureControlTypeOutsideDirectSubset' -Description "CP312 release thermostat-subset guard"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'NonFinitePredicateInput' -Description "CP312 active nonfinite predicate guard"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'calc_cooling_entry_gate\.transition_count\.checked_add\(1\)' -Description "CP312 checked predecessor call-order increment"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'temperature_control_type != PurchasedAirTemperatureControlType::DualHeatCool' -Description "CP312 exact DualHeatCool release validation"
Assert-Contains -Path $calcCoolingEntryGateRelease -Pattern 'remaining_output_req_to_cool_sp_w\s*[\r\n]+\s*\.is_finite\(\)' -Description "CP312 finite active cooling-setpoint validation"
Assert-Contains -Path $calcCoolingEntryGateTests -Pattern 'unit_off_skips_every_cooling_entry_site' -Description "Calc cooling-entry UnitOff skip regression"
Assert-Contains -Path $calcCoolingEntryGateTests -Pattern 'negative_and_both_zero_cooling_thresholds_enter_inclusively' -Description "Calc cooling-entry negative and signed-zero regression"
Assert-Contains -Path $calcCoolingEntryGateTests -Pattern 'positive_and_nan_thresholds_short_circuit_before_thermostat_read' -Description "Calc cooling-entry positive and NaN short-circuit regression"
Assert-Contains -Path $calcCoolingEntryGateTests -Pattern 'exact_single_heat_alone_blocks_a_satisfied_numeric_gate' -Description "Calc cooling-entry exact SingleHeat regression"
Assert-Contains -Path $calcCoolingEntryGateTests -Pattern 'heating_and_cooling_availability_do_not_gate_the_line_2046_predicate' -Description "Calc cooling-entry availability-independence regression"

$coolingEntryForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'PurchasedAirManager\.cc:2348'; Description = "later Heat/DeadBand selector provenance in cooling-entry boundary" },
    [pscustomobject]@{ Pattern = 'IdealLoadsSensibleMode::Heating|IdealLoadsSensibleMode::Deadband'; Description = "later Heating or DeadBand selection in cooling-entry boundary" },
    [pscustomobject]@{ Pattern = 'CoolingLimit|MaxCoolMassFlowRate|EconomizerType|CalcPurchAirMixedAir|SupplyMassFlowRate'; Description = "line-2056-and-later cooling-body behavior in cooling-entry boundary" }
)
foreach ($coolingEntryBoundaryFile in @($calcCoolingEntryGate, $calcCoolingEntryGateRelease)) {
    foreach ($forbiddenBehavior in $coolingEntryForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingEntryBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}

Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE' -Description "Calc cooling OA maximum-flow source provenance"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling OA maximum-flow first excluded source"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER' -Description "Calc cooling OA maximum-flow exact source order"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2056-2057' -Description "Calc cooling OA maximum-flow exact guard boundary"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2058' -Description "Calc cooling OA maximum-flow exact first excluded executable"
Assert-ExactStringArray -Path $calcCoolingOaMaxFlowGate -Name "PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER" -Expected @(
    "compare-cooling-limit-to-flow-rate",
    "compare-cooling-limit-to-flow-rate-and-capacity-after-short-circuit",
    "read-outdoor-air-mass-flow-after-limit-short-circuit",
    "read-maximum-cooling-air-mass-flow-after-limit-short-circuit",
    "compare-strict-outdoor-air-above-maximum-cooling-flow",
    "enter-maximum-cooling-flow-body-if-satisfied"
) -Description "Calc cooling OA maximum-flow exact six-step source order"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub struct PurchasedAirCalcCoolingOaMaxFlowGateSnapshot' -Description "Calc cooling OA maximum-flow source-ordered snapshot"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub struct PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState' -Description "Calc cooling OA maximum-flow bounded persistent state"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub struct PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary' -Description "Calc cooling OA maximum-flow lifecycle summary"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub fn purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary\s*\(' -Description "Calc cooling OA maximum-flow lifecycle summary accessor"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'pub\(super\) fn advance_cooling_oa_max_flow_gate_state\s*\(' -Description "Calc cooling OA maximum-flow source-characterization transition"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'cooling_limit == IdealLoadsLimit::LimitFlowRate' -Description "Calc cooling OA maximum-flow first selector comparison"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'cooling_limit_flow_rate_comparison_satisfied == Some\(false\)' -Description "Calc cooling OA maximum-flow OR short circuit"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity' -Description "Calc cooling OA maximum-flow second selector comparison"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'cooling_flow_limit_active == Some\(true\)' -Description "Calc cooling OA maximum-flow AND short circuit"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'outdoor_air_mass_flow_rate_kg_per_s\s*>\s*maximum_cooling_air_mass_flow_rate_kg_per_s' -Description "Calc cooling OA maximum-flow strict comparison"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'outdoor_air_mass_flow_above_maximum == Some\(true\)' -Description "Calc cooling OA maximum-flow excluded-body entry decision"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'strict_mass_flow_comparison_count' -Description "Calc cooling OA maximum-flow strict-comparison counter"
Assert-Contains -Path $calcCoolingOaMaxFlowGate -Pattern 'strict_mass_flow_comparison_satisfied_count' -Description "Calc cooling OA maximum-flow satisfied-comparison counter"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'pub enum PurchasedAirCalcCoolingOaMaxFlowGateError' -Description "Calc cooling OA maximum-flow fail-closed error"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'pub fn advance_direct_no_oa_calc_cooling_oa_max_flow_gate\s*\(' -Description "Calc cooling OA maximum-flow release transition"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'mod validation;' -Description "Calc cooling OA maximum-flow release validation helper declaration"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'use validation::\*;' -Description "Calc cooling OA maximum-flow release validation helper ownership"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'InitializationMaximumCoolingMassFlowCacheMismatch' -Description "CP313 retained maximum cooling-flow cache identity guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'InvalidMaximumCoolingMassFlowCache' -Description "CP313 finite nonnegative maximum cooling-flow cache guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'MinimumOaPrefixSnapshotMismatch' -Description "CP311-to-CP313 retained minimum-OA snapshot guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'CoolingEntryGateSnapshotMismatch' -Description "CP312-to-CP313 retained cooling-entry snapshot guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'PredecessorLinkMismatch' -Description "CP311/CP312 predecessor-link guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP313 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'MinimumOaPrefixOutsideDirectSubset' -Description "CP313 no-OA predecessor-shape guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'CoolingEntryGateOutsideDirectSubset' -Description "CP313 cooling-parent predecessor-shape guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'RuntimeStateInvariantViolation' -Description "CP313 retained lifecycle-state guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'ExactReleaseReductionViolated' -Description "CP313 excluded-body release guard"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'calc_cooling_oa_max_flow_gate\s*[\r\n]+\s*\.transition_count\s*[\r\n]+\s*\.checked_add\(1\)' -Description "CP313 checked predecessor call-order increment"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern '!supplied_maximum\.is_finite\(\) \|\| supplied_maximum < 0\.0' -Description "CP313 supplied maximum cooling-flow validation"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern '!retained_maximum\.is_finite\(\) \|\| retained_maximum < 0\.0' -Description "CP313 retained maximum cooling-flow validation"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'working_outdoor_air_mass_flow_rate_kg_per_s' -Description "CP313 minimum-OA local mass-flow input"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'IdealLoadsLimit::LimitFlowRate \| IdealLoadsLimit::LimitFlowRateAndCapacity' -Description "CP313 exact release flow-limit selector"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'outdoor_air_mass_flow_rate_kg_per_s > retained_maximum' -Description "CP313 exact strict release comparison"
Assert-Contains -Path $calcCoolingOaMaxFlowGateRelease -Pattern 'debug_assert!\(!snapshot\.maximum_cooling_flow_body_entered\)' -Description "CP313 release never enters excluded line-2058 body"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern 'pub\(super\) fn minimum_oa_snapshot_is_direct_release\s*\(' -Description "CP313 minimum-OA retained-snapshot validator"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern 'pub\(super\) fn cooling_entry_snapshot_is_direct_release\s*\(' -Description "CP313 cooling-entry retained-snapshot validator"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern 'pub\(super\) fn cooling_oa_max_flow_runtime_state_is_consistent\s*\(' -Description "CP313 lifecycle-state consistency validator"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern 'IdealLoadsLimit::LimitFlowRateAndCapacity' -Description "CP313 combined-limit retained-history shape"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern 'IdealLoadsLimit::NoLimit \| IdealLoadsLimit::LimitCapacity' -Description "CP313 no-flow-selector retained-history shape"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern '\.checked_add\(state\.unit_off_skip_count\)' -Description "CP313 checked transition partition"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern '\.checked_add\(state\.cooling_limit_flow_rate_and_capacity_match_count\)' -Description "CP313 checked selected-flow partition"
Assert-Contains -Path $calcCoolingOaMaxFlowGateReleaseValidation -Pattern 'state\.strict_mass_flow_comparison_satisfied_count\s*[\r\n]+\s*== state\.maximum_cooling_flow_body_entry_count' -Description "CP313 satisfied-comparison/body-entry reconciliation"
Assert-Contains -Path $calcCoolingOaMaxFlowGateTests -Pattern 'strict_greater_than_characterizes_nan_signed_zero_and_infinity' -Description "Calc cooling OA maximum-flow strict-comparison edge-case regression"
Assert-Contains -Path $calcCoolingOaMaxFlowGateTests -Pattern 'selector_or_and_mass_flow_and_short_circuits_follow_source_order' -Description "Calc cooling OA maximum-flow OR/AND short-circuit regression"
Assert-Contains -Path $calcCoolingOaMaxFlowGateTests -Pattern 'unit_off_and_active_non_cooling_skip_every_site_in_distinct_partitions' -Description "Calc cooling OA maximum-flow parent-skip partition regression"

$coolingOaMaxFlowForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'PurchasedAirManager\.cc:(2082|2348)'; Description = "later economizer or Heat/DeadBand provenance in cooling OA maximum-flow boundary" },
    [pscustomobject]@{ Pattern = 'OAVolFlowRate\s*='; Description = "line-2058 outdoor-air volume-flow clamp in cooling OA maximum-flow boundary" },
    [pscustomobject]@{ Pattern = 'OAFlowMaxCoolOutput(Error|Index)|ShowWarningError|ShowContinueError|ShowRecurringWarningErrorAtEnd'; Description = "line-2058-and-later warning behavior in cooling OA maximum-flow boundary" },
    [pscustomobject]@{ Pattern = 'StdRhoAir|MaxCoolVolFlowRate'; Description = "line-2058-and-later volume-to-mass-flow behavior in cooling OA maximum-flow boundary" },
    [pscustomobject]@{ Pattern = 'OAMassFlowRate\s*=\s*PurchAir\.MaxCoolMassFlowRate|outdoor_air_mass_flow_rate_kg_per_s\s*=\s*(retained_maximum|maximum_cooling_air_mass_flow_rate_kg_per_s)'; Description = "line-2078 outdoor-air mass-flow clamp in cooling OA maximum-flow boundary" },
    [pscustomobject]@{ Pattern = 'EconomizerType|TimeEconoActive|CalcPurchAirMixedAir|SupplyMassFlowRate'; Description = "later economizer, mixed-air, or supply-flow behavior in cooling OA maximum-flow boundary" }
)
foreach ($coolingOaMaxFlowBoundaryFile in @($calcCoolingOaMaxFlowGate, $calcCoolingOaMaxFlowGateRelease, $calcCoolingOaMaxFlowGateReleaseValidation)) {
    foreach ($forbiddenBehavior in $coolingOaMaxFlowForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingOaMaxFlowBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}

Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE' -Description "Calc cooling OA maximum-flow true-body source provenance"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling OA maximum-flow true-body first excluded source"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE' -Description "Calc cooling OA maximum-flow recurring-warning child provenance"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER' -Description "Calc cooling OA maximum-flow true-body exact source order"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2058-2078' -Description "Calc cooling OA maximum-flow exact true-body boundary"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2082' -Description "Calc cooling OA maximum-flow exact lexical first excluded executable"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'EnergyPlus 26\.1 UtilityRoutines\.cc:1146-1194,1293-1379; max-only optional argument' -Description "Calc cooling OA maximum-flow exact recurring-warning child boundary"
Assert-ExactStringArray -Path $calcCoolingOaMaxFlowBody -Name "PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER" -Expected @(
    "read-outdoor-air-mass-flow-for-volume-conversion",
    "read-standard-air-density-for-volume-conversion",
    "calculate-outdoor-air-volume-flow",
    "read-first-warning-counter",
    "compare-first-warning-counter-below-one",
    "enter-first-warning-branch-if-satisfied",
    "increment-first-warning-counter",
    "reach-first-warning-call-site",
    "read-maximum-cooling-air-volume-flow-for-continue-warning",
    "reach-continue-warning-call-site",
    "reach-continue-warning-timestamp-call-site",
    "enter-recurring-warning-branch-otherwise",
    "reach-recurring-warning-call-site-with-max-only-value",
    "characterize-recurring-warning-index-allocation-or-reuse",
    "characterize-recurring-warning-report-maximum",
    "read-maximum-cooling-air-mass-flow-for-clamp",
    "assign-clamped-outdoor-air-mass-flow"
) -Description "Calc cooling OA maximum-flow exact 17-step true-body source order"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'mod release;' -Description "Calc cooling OA maximum-flow true-body release submodule declaration"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'mod transition;' -Description "Calc cooling OA maximum-flow true-body transition submodule declaration"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub use release::\*;' -Description "Calc cooling OA maximum-flow true-body release re-export"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub\(super\) use transition::advance_cooling_oa_max_flow_body_state;' -Description "Calc cooling OA maximum-flow bounded internal transition visibility"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub struct PurchasedAirCalcCoolingOaMaxFlowBodySnapshot' -Description "Calc cooling OA maximum-flow true-body source-ordered snapshot"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub struct PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState' -Description "Calc cooling OA maximum-flow true-body bounded persistent state"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub struct PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary' -Description "Calc cooling OA maximum-flow true-body lifecycle summary"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub fn purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary\s*\(' -Description "Calc cooling OA maximum-flow true-body lifecycle summary accessor"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub outdoor_air_flow_max_cooling_output_error_count: usize' -Description "Calc cooling OA maximum-flow retained first-warning counter"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub outdoor_air_flow_max_cooling_output_index: usize' -Description "Calc cooling OA maximum-flow retained recurring index"
Assert-Contains -Path $calcCoolingOaMaxFlowBody -Pattern 'pub characterized_recurring_warning_report_maximum_m3_per_s: Option<f64>' -Description "Calc cooling OA maximum-flow max-only recurring value"
Assert-NotContains -Path $calcCoolingOaMaxFlowBody -Pattern 'fn advance_cooling_oa_max_flow_body_state\s*\(' -Description "true-body transition implementation in module facade"
Assert-NotContains -Path $calcCoolingOaMaxFlowBody -Pattern '#\[test\]' -Description "unit test body in cooling OA maximum-flow true-body facade"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_oa_max_flow_body_state\s*\(' -Description "Calc cooling OA maximum-flow source-characterization transition"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'outdoor_air_mass_flow_rate_value_kg_per_s / standard_air_density_kg_per_m3' -Description "Calc cooling OA maximum-flow volume conversion"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'warning_counter_before\.map\(\|counter\| counter < 1\)' -Description "Calc cooling OA maximum-flow first-warning predicate"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'outdoor_air_flow_max_cooling_output_error_count \+= 1' -Description "Calc cooling OA maximum-flow first-warning counter increment"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'outdoor_air_flow_max_cooling_output_index = 1' -Description "Calc cooling OA maximum-flow relative recurring index allocation"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'source_max\(value, retained\)' -Description "Calc cooling OA maximum-flow max-only recurring aggregation"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'characterized_total_warning_error_increment_count \+= 1' -Description "Calc cooling OA maximum-flow one characterized warning increment per body"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'active_guard_false_economizer_fallthrough_count \+= 1' -Description "Calc cooling OA maximum-flow active false-path continuation partition"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTransition -Pattern 'outdoor_air_mass_flow_rate_after_clamp_kg_per_s:\s*[\r\n]+\s*maximum_cooling_air_mass_flow_rate_kg_per_s' -Description "Calc cooling OA maximum-flow final mass-flow clamp"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'pub enum PurchasedAirCalcCoolingOaMaxFlowBodyError' -Description "Calc cooling OA maximum-flow true-body fail-closed error"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'pub fn advance_direct_no_oa_calc_cooling_oa_max_flow_body\s*\(' -Description "Calc cooling OA maximum-flow true-body release transition"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'mod validation;' -Description "Calc cooling OA maximum-flow true-body release validation helper declaration"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'use validation::\*;' -Description "Calc cooling OA maximum-flow true-body release validation helper ownership"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'InitializationSnapshotMismatch' -Description "CP314 retained initialization and density identity guard"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'InitializationMaximumCoolingMassFlowCacheMismatch' -Description "CP314 retained maximum cooling-flow cache identity guard"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'CoolingOaMaxFlowGateSnapshotMismatch' -Description "CP313-to-CP314 retained snapshot guard"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'PredecessorLinkMismatch' -Description "CP313-to-CP314 predecessor-link guard"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP314 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'PredecessorOutsideDirectSubset' -Description "CP314 exact release predecessor fallthrough guard"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'RuntimeStateInvariantViolation' -Description "CP314 retained zero-effect release-state guard"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'calc_cooling_oa_max_flow_body\s*[\r\n]+\s*\.transition_count\s*[\r\n]+\s*\.checked_add\(1\)' -Description "CP314 checked predecessor call-order increment"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'supplied_density\.is_some_and\(\|density\| density\.is_finite\(\) && density > 0\.0\)' -Description "CP314 finite positive retained density validation"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'predecessor_is_exact_direct_fallthrough\s*\(' -Description "CP314 exact release predecessor fallthrough validation"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'debug_assert!\(snapshot\.body_skipped\)' -Description "CP314 exact release complete-skip result"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'debug_assert!\(!snapshot\.standard_air_density_read\)' -Description "CP314 exact release no mapped density read"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyRelease -Pattern 'debug_assert!\(!snapshot\.outdoor_air_mass_flow_clamp_assignment_performed\)' -Description "CP314 exact release no mass-flow clamp"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'pub\(super\) fn predecessor_is_exact_direct_fallthrough\s*\(' -Description "CP314 exact CP313 predecessor validator"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'pub\(super\) fn direct_runtime_states_are_consistent\s*\(' -Description "CP314 retained CP313/CP314 lifecycle validator"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'predecessor\.maximum_cooling_flow_body_entered' -Description "CP314 exact release body-entry rejection"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'predecessor\.outdoor_air_mass_flow_above_maximum == Some\(false\)' -Description "CP314 exact no-OA strict-comparison fallthrough"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'body\.transition_count == body\.body_skip_count' -Description "CP314 exact release complete-skip invariant"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'body\.body_entry_count == 0' -Description "CP314 exact release zero-body-entry invariant"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'body\.standard_air_density_read_count == 0' -Description "CP314 exact release zero mapped density-read invariant"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'body\.outdoor_air_flow_max_cooling_output_index == 0' -Description "CP314 exact release zero recurring-index invariant"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyReleaseValidation -Pattern 'body\.outdoor_air_mass_flow_clamp_assignment_count == 0' -Description "CP314 exact release zero-clamp invariant"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTests -Pattern 'body_skip_partitions_expose_no_sites_or_nonfinite_values' -Description "Calc cooling OA maximum-flow complete-skip regression"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTests -Pattern 'first_body_entry_reaches_first_warning_sites_and_clamps' -Description "Calc cooling OA maximum-flow first-warning and clamp regression"
Assert-Contains -Path $calcCoolingOaMaxFlowBodyTests -Pattern 'later_entries_allocate_then_reuse_recurring_index_and_update_max_only' -Description "Calc cooling OA maximum-flow recurring max-only regression"

$coolingOaMaxFlowBodyForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'PurchasedAirManager\.cc:(2109|2348)'; Description = "later cooling reset or Heat/DeadBand provenance in cooling OA maximum-flow true-body boundary" },
    [pscustomobject]@{ Pattern = 'EconomizerType|TimeEconoActive|CalcPurchAirMixedAir|SupplyMassFlowRateForCool'; Description = "later economizer, mixed-air, or supply-flow behavior in cooling OA maximum-flow true-body boundary" },
    [pscustomobject]@{ Pattern = 'ShowWarningError|ShowContinueError|ShowRecurringWarningErrorAtEnd|StoreRecurringErrorMessage|TotalWarningErrors'; Description = "actual warning service or process-global registry in cooling OA maximum-flow true-body boundary" },
    [pscustomobject]@{ Pattern = 'ReportMinOf|ReportSumOf|reported_minimum|reported_sum'; Description = "unmapped recurring minimum or sum behavior in max-only warning characterization" },
    [pscustomobject]@{ Pattern = 'calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s|CalcPurchAirMinOAMassFlow|PsyCpAirFnW'; Description = "unmapped OA, economizer, or psychrometric numerical behavior in cooling OA maximum-flow true-body boundary" }
)
foreach ($coolingOaMaxFlowBodyBoundaryFile in @($calcCoolingOaMaxFlowBody, $calcCoolingOaMaxFlowBodyTransition, $calcCoolingOaMaxFlowBodyRelease, $calcCoolingOaMaxFlowBodyReleaseValidation)) {
    foreach ($forbiddenBehavior in $coolingOaMaxFlowBodyForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingOaMaxFlowBodyBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}

Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE' -Description "Calc cooling economizer guard source provenance"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling economizer guard first excluded source"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER' -Description "Calc cooling economizer guard exact source order"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2082' -Description "Calc cooling economizer exact outer-guard boundary"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2083' -Description "Calc cooling economizer exact lexical first excluded executable"
Assert-ExactStringArray -Path $calcCoolingEconomizerGuard -Name "PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER" -Expected @(
    "read-economizer-type",
    "compare-economizer-type-not-equal-to-no-economizer",
    "enter-economizer-body-if-satisfied"
) -Description "Calc cooling economizer exact three-step outer-guard source order"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'mod release;' -Description "Calc cooling economizer guard release submodule declaration"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'mod transition;' -Description "Calc cooling economizer guard transition submodule declaration"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub use release::\*;' -Description "Calc cooling economizer guard release re-export"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub\(super\) use transition::advance_cooling_economizer_guard_state;' -Description "Calc cooling economizer bounded internal transition visibility"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerGuardSnapshot' -Description "Calc cooling economizer source-ordered snapshot"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerGuardRuntimeState' -Description "Calc cooling economizer bounded persistent state"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary' -Description "Calc cooling economizer lifecycle summary"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub fn purchased_air_calc_cooling_economizer_guard_lifecycle_summary\s*\(' -Description "Calc cooling economizer lifecycle summary accessor"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub economizer_type: Option<OutdoorAirEconomizerType>' -Description "Calc cooling economizer typed enum read evidence"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub economizer_not_no_economizer: Option<bool>' -Description "Calc cooling economizer comparison-result evidence"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub economizer_body_entry_count: usize' -Description "Calc cooling economizer excluded-body entry counter"
Assert-Contains -Path $calcCoolingEconomizerGuard -Pattern 'pub no_economizer_fallthrough_count: usize' -Description "Calc cooling economizer false-result continuation counter"
Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)#\[derive\(Clone, Debug, Default, PartialEq\)\]\s*pub struct PurchasedAirRuntimeState\s*\{.*?cooling_economizer_condition_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingEconomizerConditionSnapshot>,' -Description "runtime-root default-empty per-system CP316 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_economizer_condition_latest_witnesses:' -Description "public runtime-root CP316 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern '(?s)pub\(in crate::ideal_loads\) fn cooling_economizer_condition_latest_witness\s*\(\s*&self,\s*system:\s*IdealLoadsAirSystemId,\s*\)\s*->\s*Option<PurchasedAirCalcCoolingEconomizerConditionSnapshot>\s*\{\s*self\.cooling_economizer_condition_latest_witnesses\s*\.get\(&system\)\s*\.copied\(\)\s*\}' -Description "ideal_loads-scoped runtime-root CP316 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern '(?s)pub\(in crate::ideal_loads\) fn set_cooling_economizer_condition_latest_witness\s*\(\s*&mut self,\s*system:\s*IdealLoadsAirSystemId,\s*snapshot:\s*PurchasedAirCalcCoolingEconomizerConditionSnapshot,\s*\)\s*\{\s*self\.cooling_economizer_condition_latest_witnesses\s*\.insert\(system, snapshot\);\s*\}' -Description "ideal_loads-scoped runtime-root CP316 witness setter"
Assert-NotContains -Path $calcCoolingEconomizerGuard -Pattern '\b(?:cooling_economizer_condition_latest_witness(?:es)?|condition_consumer_latest_witness|PurchasedAirCalcCoolingEconomizerConditionSnapshot)\b' -Description "CP316 witness ownership, accessor, setter, or snapshot import in CP315 guard state"
Assert-NotContains -Path $calcCoolingEconomizerGuard -Pattern 'fn advance_cooling_economizer_guard_state\s*\(' -Description "cooling economizer transition implementation in module facade"
Assert-NotContains -Path $calcCoolingEconomizerGuard -Pattern '#\[test\]' -Description "unit test body in cooling economizer guard facade"
Assert-NotContains -Path $calcCoolingEconomizerGuardTransition -Pattern '\b(?:cooling_economizer_condition_latest_witness(?:es)?|condition_consumer_latest_witness)\b' -Description "CP315 transition mutation of the runtime-root CP316 witness"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_economizer_guard_state\s*\(' -Description "Calc cooling economizer source-characterization transition"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'predecessor\.active_guard_false_economizer_fallthrough' -Description "Calc cooling economizer exact CP314 fallthrough gate"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern '\.map\(\|value\| value != OutdoorAirEconomizerType::NoEconomizer\)' -Description "Calc cooling economizer exact typed inequality comparison"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'economizer_body_entered = economizer_not_no_economizer == Some\(true\)' -Description "Calc cooling economizer true-result excluded-body decision"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'no_economizer_fallthrough = economizer_not_no_economizer == Some\(false\)' -Description "Calc cooling economizer false-result continuation"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'state\.guard_evaluation_count \+= 1' -Description "Calc cooling economizer guard evaluation counter"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'state\.economizer_type_read_count \+= 1' -Description "Calc cooling economizer typed enum read counter"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'state\.no_economizer_comparison_count \+= 1' -Description "Calc cooling economizer comparison counter"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'state\.economizer_body_entry_count \+= 1' -Description "Calc cooling economizer characterized true-result counter"
Assert-Contains -Path $calcCoolingEconomizerGuardTransition -Pattern 'state\.no_economizer_fallthrough_count \+= 1' -Description "Calc cooling economizer false-result counter"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'pub enum PurchasedAirCalcCoolingEconomizerGuardError' -Description "Calc cooling economizer fail-closed error"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'pub fn advance_direct_no_oa_calc_cooling_economizer_guard\s*\(' -Description "Calc cooling economizer release transition"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'mod validation;' -Description "Calc cooling economizer release validation helper declaration"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'use validation::\*;' -Description "Calc cooling economizer release validation helper ownership"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'CoolingOaMaxFlowBodySnapshotMismatch' -Description "CP314-to-CP315 retained snapshot guard"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'PredecessorLinkMismatch' -Description "CP313/CP314-to-CP315 predecessor-link guard"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP315 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'PredecessorOutsideDirectSubset' -Description "CP315 exact release predecessor-shape guard"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'RuntimeStateInvariantViolation' -Description "CP315 retained release-state guard"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'classify_no_oa_sensible_subset\(system\)\.is_supported\(\)' -Description "CP315 exact no-OA release subset validation"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'calc_cooling_economizer_guard\s*[\r\n]+\s*\.transition_count\s*[\r\n]+\s*\.checked_add\(1\)' -Description "CP315 checked predecessor call-order increment"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'predecessor_is_exact_direct_release\s*\(' -Description "CP315 exact CP314 predecessor validation"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'direct_runtime_states_are_consistent\s*\(' -Description "CP315 retained CP313-through-CP315 lifecycle validation"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'system\.outdoor_air_economizer_type' -Description "CP315 typed selected-unit economizer read input"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'debug_assert!\(!snapshot\.economizer_body_entered\)' -Description "CP315 exact release zero inner-body entry"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'debug_assert_eq!\(snapshot\.economizer_not_no_economizer, Some\(false\)\)' -Description "CP315 exact release false outer-guard result"
Assert-Contains -Path $calcCoolingEconomizerGuardRelease -Pattern 'debug_assert!\(snapshot\.no_economizer_fallthrough\)' -Description "CP315 exact release line-2109 continuation decision"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'pub\(super\) fn cooling_oa_max_flow_body_snapshots_bitwise_equal\s*\(' -Description "CP315 exact retained CP314 snapshot identity validator"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'pub\(super\) fn predecessor_links_to_gate\s*\(' -Description "CP315 exact CP313/CP314 link validator"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'pub\(super\) fn predecessor_is_exact_direct_release\s*\(' -Description "CP315 exact CP314 release-shape validator"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'pub\(super\) fn direct_runtime_states_are_consistent\s*\(' -Description "CP315 retained CP313-through-CP315 history validator"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'predecessor\.active_guard_false_economizer_fallthrough' -Description "CP315 active CP313-false/CP314-fallthrough predecessor shape"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'guard\s*[\r\n]+\s*\.guard_evaluation_count\s*[\r\n]+\s*\.checked_add\(guard\.unit_off_skip_count\)' -Description "CP315 checked transition partition"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'guard\.guard_evaluation_count == guard\.economizer_type_read_count' -Description "CP315 one typed enum read per evaluated guard"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'guard\.economizer_type_read_count == guard\.no_economizer_comparison_count' -Description "CP315 one comparison per typed enum read"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'guard\.economizer_body_entry_count == 0' -Description "CP315 exact release zero excluded-body entries"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'guard\.no_economizer_fallthrough_count == guard\.guard_evaluation_count' -Description "CP315 exact release all evaluated guards fall through"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'snapshot\.economizer_type == Some\(OutdoorAirEconomizerType::NoEconomizer\)' -Description "CP315 exact release retained NoEconomizer value"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'snapshot\.economizer_not_no_economizer == Some\(false\)' -Description "CP315 exact release retained false comparison"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern '!snapshot\.economizer_body_entered' -Description "CP315 exact release retained zero inner-body entry"
Assert-Contains -Path $calcCoolingEconomizerGuardReleaseValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "CP315 bitwise CP314 floating-state identity"
Assert-Contains -Path $calcCoolingEconomizerGuardTests -Pattern 'line_2082_characterizes_all_typed_economizer_values_without_body_effects' -Description "Calc cooling economizer typed enum characterization regression"
Assert-Contains -Path $calcCoolingEconomizerGuardTests -Pattern 'unit_off_non_cooling_and_true_sibling_body_are_distinct_complete_skips' -Description "Calc cooling economizer complete parent-skip partition regression"

$coolingEconomizerGuardForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'PurchasedAirManager\.cc:2348'; Description = "later Heat/DeadBand provenance in cooling economizer guard boundary" },
    [pscustomobject]@{ Pattern = 'SupplyMassFlowRateForCool\s*=|OAMassFlowRate\s*=|EconoOn\s*=|TimeEconoActive'; Description = "excluded cooling-flow or economizer-body mutation in cooling economizer guard boundary" },
    [pscustomobject]@{ Pattern = 'PsyCpAirFnW|DeltaT\b|CalcPurchAirMixedAir'; Description = "excluded inner psychrometric or mixed-air calculation in cooling economizer guard boundary" },
    [pscustomobject]@{ Pattern = 'calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s'; Description = "separate outdoor-air economizer helper in cooling economizer guard boundary" },
    [pscustomobject]@{ Pattern = '(OutAir|OutdoorAir|ZoneExhaust).*Node.*(Temp|Enthalpy)'; Description = "excluded inner node temperature or enthalpy read in cooling economizer guard boundary" }
)
foreach ($coolingEconomizerGuardBoundaryFile in @($calcCoolingEconomizerGuard, $calcCoolingEconomizerGuardTransition, $calcCoolingEconomizerGuardRelease, $calcCoolingEconomizerGuardReleaseValidation)) {
    foreach ($forbiddenBehavior in $coolingEconomizerGuardForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingEconomizerGuardBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}

Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE' -Description "Calc cooling economizer condition source provenance"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling economizer condition first excluded source"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER' -Description "Calc cooling economizer condition exact source order"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2083-2086' -Description "Calc cooling economizer exact compound-condition boundary"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2089' -Description "Calc cooling economizer condition exact lexical first excluded executable"
Assert-ExactStringArray -Path $calcCoolingEconomizerCondition -Name "PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER" -Expected @(
    "read-economizer-type-for-differential-dry-bulb",
    "compare-economizer-type-equal-to-differential-dry-bulb",
    "read-outdoor-air-node-temperature-after-dry-bulb-match",
    "read-zone-recirculation-air-node-temperature-after-dry-bulb-match",
    "compare-strict-outdoor-temperature-below-zone-recirculation-temperature",
    "read-economizer-type-for-differential-enthalpy-after-dry-bulb-arm-false",
    "compare-economizer-type-equal-to-differential-enthalpy",
    "read-outdoor-air-node-enthalpy-after-enthalpy-match",
    "read-zone-recirculation-air-node-enthalpy-after-enthalpy-match",
    "compare-strict-outdoor-enthalpy-below-zone-recirculation-enthalpy",
    "select-excluded-line-2089-if-compound-condition-satisfied",
    "select-excluded-line-2109-if-compound-condition-false"
) -Description "Calc cooling economizer exact twelve-step short-circuit source order"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'mod release;' -Description "Calc cooling economizer condition release submodule declaration"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'mod transition;' -Description "Calc cooling economizer condition transition submodule declaration"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub use release::\*;' -Description "Calc cooling economizer condition release re-export"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub\(super\) use transition::advance_cooling_economizer_condition_state;' -Description "Calc cooling economizer condition bounded internal transition visibility"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerConditionSnapshot' -Description "Calc cooling economizer condition source-ordered snapshot"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerConditionRuntimeState' -Description "Calc cooling economizer condition bounded persistent state"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary' -Description "Calc cooling economizer condition lifecycle summary"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub fn purchased_air_calc_cooling_economizer_condition_lifecycle_summary\s*\(' -Description "Calc cooling economizer condition lifecycle summary accessor"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub\(in crate::ideal_loads::calc\) struct PurchasedAirCalcCoolingEconomizerConditionInput' -Description "Calc cooling economizer internal pre-sampled characterization input"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern '(?s)enum PurchasedAirCalcCoolingEconomizerConditionRetainedRoute\s*\{\s*UnitOff,\s*NonCooling,\s*MaximumCoolingFlowBodySibling,\s*NoEconomizerOuterGuardFallthrough,\s*Evaluated,\s*\}' -Description "Calc cooling economizer exact private retained-route tags"
Assert-NotContains -Path $calcCoolingEconomizerCondition -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+enum PurchasedAirCalcCoolingEconomizerConditionRetainedRoute\b' -Description "public cooling economizer retained-route enum"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern '(?m)^\s*latest_route:\s*Option<PurchasedAirCalcCoolingEconomizerConditionRetainedRoute>,\s*$' -Description "Calc cooling economizer private latest-route evidence"
Assert-NotContains -Path $calcCoolingEconomizerCondition -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+latest_route:' -Description "public cooling economizer latest-route evidence"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'latest_route:\s*None' -Description "Calc cooling economizer retained-route empty initialization"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern '(?m)^\s*latest_transition_ordinal:\s*Option<usize>,\s*$' -Description "Calc cooling economizer private latest-transition generation evidence"
Assert-NotContains -Path $calcCoolingEconomizerCondition -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+latest_transition_ordinal:' -Description "public cooling economizer latest-transition generation evidence"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'latest_transition_ordinal:\s*None' -Description "Calc cooling economizer latest-transition generation empty initialization"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub differential_dry_bulb_economizer_type_read_count: usize' -Description "Calc cooling economizer first enum-read counter"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub differential_enthalpy_economizer_type_read_count: usize' -Description "Calc cooling economizer repeated enum-read counter"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub outdoor_air_temperature_read_count: usize' -Description "Calc cooling economizer conditional outdoor-temperature counter"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub recirculation_air_temperature_read_count: usize' -Description "Calc cooling economizer conditional recirculation-temperature counter"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub outdoor_air_enthalpy_read_count: usize' -Description "Calc cooling economizer conditional outdoor-enthalpy counter"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub recirculation_air_enthalpy_read_count: usize' -Description "Calc cooling economizer conditional recirculation-enthalpy counter"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub economizer_calculation_body_entry_count: usize' -Description "Calc cooling economizer excluded line-2089 body-entry counter"
Assert-Contains -Path $calcCoolingEconomizerCondition -Pattern 'pub economizer_condition_fallthrough_count: usize' -Description "Calc cooling economizer line-2109 fallthrough counter"
Assert-NotContains -Path $calcCoolingEconomizerCondition -Pattern 'fn advance_cooling_economizer_condition_state\s*\(' -Description "cooling economizer condition transition implementation in module facade"
Assert-NotContains -Path $calcCoolingEconomizerCondition -Pattern '#\[test\]' -Description "unit test body in cooling economizer condition facade"

Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_economizer_condition_state\s*\(' -Description "Calc cooling economizer source-characterization transition"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'economizer_condition_evaluated = predecessor\.economizer_body_entered' -Description "Calc cooling economizer exact CP315 true-guard gate"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern '\.map\(\|value\| value == OutdoorAirEconomizerType::DifferentialDryBulb\)' -Description "Calc cooling economizer dry-bulb selector equality"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'dry_bulb_operands_read = differential_dry_bulb_selector_matched == Some\(true\)' -Description "Calc cooling economizer conditional temperature reads"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'input\.outdoor_air_temperature_c < input\.recirculation_air_temperature_c' -Description "Calc cooling economizer strict dry-bulb comparison"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'outdoor_air_temperature_below_recirculation_temperature != Some\(true\)' -Description "Calc cooling economizer source OR short-circuit fallthrough"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'differential_enthalpy_selector_comparison_evaluated\.then_some\(input\.economizer_type\)' -Description "Calc cooling economizer repeated typed enum read"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern '\.map\(\|value\| value == OutdoorAirEconomizerType::DifferentialEnthalpy\)' -Description "Calc cooling economizer enthalpy selector equality"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'enthalpy_operands_read = differential_enthalpy_selector_matched == Some\(true\)' -Description "Calc cooling economizer conditional stored-enthalpy reads"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'input\.outdoor_air_enthalpy_j_per_kg < input\.recirculation_air_enthalpy_j_per_kg' -Description "Calc cooling economizer strict stored-enthalpy comparison"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'outdoor_air_temperature_below_recirculation_temperature == Some\(true\)\s*[\r\n]+\s*\|\| outdoor_air_enthalpy_below_recirculation_enthalpy == Some\(true\)' -Description "Calc cooling economizer source disjunction result"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.differential_dry_bulb_economizer_type_read_count \+= 1' -Description "Calc cooling economizer first enum-read counter update"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.differential_enthalpy_economizer_type_read_count \+= 1' -Description "Calc cooling economizer repeated enum-read counter update"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.dry_bulb_temperature_comparison_count \+= 1' -Description "Calc cooling economizer dry-bulb comparison counter update"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.enthalpy_comparison_count \+= 1' -Description "Calc cooling economizer enthalpy comparison counter update"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.economizer_calculation_body_entry_count \+= 1' -Description "Calc cooling economizer true continuation counter update"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.economizer_condition_fallthrough_count \+= 1' -Description "Calc cooling economizer false continuation counter update"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.latest_route = Some\(retained_route\);' -Description "Calc cooling economizer retained-route recording"
Assert-Contains -Path $calcCoolingEconomizerConditionTransition -Pattern 'state\.latest_transition_ordinal = Some\(state\.transition_count\);' -Description "Calc cooling economizer current-generation recording"
Assert-PatternsInOrder -Path $calcCoolingEconomizerConditionTransition -Patterns @(
    'pub\(in crate::ideal_loads::calc\) fn advance_cooling_economizer_condition_state\s*\(',
    'let economizer_condition_evaluated = predecessor\.economizer_body_entered;',
    'let retained_route = if economizer_condition_evaluated',
    'let dry_bulb_economizer_type = economizer_condition_evaluated\.then_some\(input\.economizer_type\);',
    '\.map\(\|value\| value == OutdoorAirEconomizerType::DifferentialDryBulb\);',
    'let dry_bulb_operands_read = differential_dry_bulb_selector_matched == Some\(true\);',
    '(?s)let \(outdoor_air_temperature_c, recirculation_air_temperature_c\) = if dry_bulb_operands_read \{.*?Some\(input\.recirculation_air_temperature_c\)',
    'Some\(input\.outdoor_air_temperature_c < input\.recirculation_air_temperature_c\)',
    '(?s)let differential_enthalpy_selector_comparison_evaluated = economizer_condition_evaluated\s*&& outdoor_air_temperature_below_recirculation_temperature != Some\(true\);',
    'differential_enthalpy_selector_comparison_evaluated\.then_some\(input\.economizer_type\);',
    '\.map\(\|value\| value == OutdoorAirEconomizerType::DifferentialEnthalpy\);',
    'let enthalpy_operands_read = differential_enthalpy_selector_matched == Some\(true\);',
    '(?s)let \(outdoor_air_enthalpy_j_per_kg, recirculation_air_enthalpy_j_per_kg\) =\s*if enthalpy_operands_read \{.*?Some\(input\.recirculation_air_enthalpy_j_per_kg\)',
    'Some\(input\.outdoor_air_enthalpy_j_per_kg < input\.recirculation_air_enthalpy_j_per_kg\)',
    '(?s)let economizer_condition_satisfied = economizer_condition_evaluated\.then_some\(\s*outdoor_air_temperature_below_recirculation_temperature == Some\(true\)\s*\|\| outdoor_air_enthalpy_below_recirculation_enthalpy == Some\(true\),\s*\);',
    'let snapshot = PurchasedAirCalcCoolingEconomizerConditionSnapshot \{',
    'state\.latest = Some\(snapshot\);',
    'state\.latest_route = Some\(retained_route\);',
    'state\.latest_transition_ordinal = Some\(state\.transition_count\);'
) -Description "Calc cooling economizer executable selector, short-circuit, snapshot, and retained-route flow"

Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'pub enum PurchasedAirCalcCoolingEconomizerConditionError' -Description "Calc cooling economizer condition fail-closed error"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_economizer_condition\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingEconomizerGuardSnapshot,\s*\)\s*->\s*Result<\s*PurchasedAirCalcCoolingEconomizerConditionSnapshot,\s*PurchasedAirCalcCoolingEconomizerConditionError,\s*>\s*\{' -Description "Calc cooling economizer exact public no-node release signature"
Assert-NotContains -Path $calcCoolingEconomizerConditionRelease -Pattern '\b(?:NodeId|NodeStateStore|AirNodeState)\b' -Description "live Node dependency in public cooling economizer release boundary"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'mod entry_prefix_validation;' -Description "Calc cooling economizer retained entry-prefix validation helper declaration"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'mod initialization_validation;' -Description "Calc cooling economizer retained initialization validation helper declaration"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'mod predecessor_validation;' -Description "Calc cooling economizer predecessor validation helper declaration"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'mod runtime_validation;' -Description "Calc cooling economizer runtime validation helper declaration"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'use entry_prefix_validation::completed_cp310_through_cp313_prefix_is_consistent;' -Description "Calc cooling economizer retained entry-prefix helper import"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'use initialization_validation::initialization_state_is_exact_direct_release;' -Description "Calc cooling economizer retained initialization helper import"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'use predecessor_validation::\*;' -Description "Calc cooling economizer predecessor validation ownership"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'use runtime_validation::\*;' -Description "Calc cooling economizer runtime validation ownership"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'CoolingEconomizerGuardSnapshotMismatch' -Description "CP315-to-CP316 retained snapshot guard"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP316 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'PredecessorOutsideDirectSubset' -Description "CP316 exact release predecessor-shape guard"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'RuntimeStateInvariantViolation' -Description "CP316 retained release-state guard"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'classify_no_oa_sensible_subset\(system\)\.is_supported\(\)' -Description "CP316 exact no-OA release subset validation"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'system\.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer' -Description "CP316 exact NoEconomizer release guard"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'calc_state_identities_match\s*\(' -Description "CP316 retained CP310-through-CP316 identity validation"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'economizer_guard_snapshot_is_exact_direct_release\s*\(' -Description "CP316 exact CP315 predecessor validation"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'call_order_is_pending_condition\s*\(' -Description "CP316 pending one-for-one call-order validation"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'completed_cp310_through_cp313_prefix_is_consistent\(unit, system\)' -Description "CP316 retained CP310-through-CP313 entry-prefix validation call"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'completed_cp313_through_cp315_prefix_is_consistent\s*\(' -Description "CP316 retained predecessor-history validation"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'pending_condition_state_is_consistent\s*\(' -Description "CP316 retained condition-state validation"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'initialization_state_is_exact_direct_release\(runtime, unit, system\)' -Description "CP316 exact retained initialization validation call"
Assert-PatternsInOrder -Path $calcCoolingEconomizerConditionRelease -Patterns @(
    'pub fn advance_direct_no_oa_calc_cooling_economizer_condition\s*\(',
    'let selected = predecessor\.system;',
    'let unit = runtime\.units\.get\(&selected\)\.ok_or\(',
    'let condition_consumer_latest_witness =\s*runtime\.cooling_economizer_condition_latest_witness\(selected\);',
    'if system\.id != selected',
    'initialization_state_is_exact_direct_release\(runtime, unit, system\)',
    'calc_state_identities_match\(unit, selected\)',
    'economizer_guard_snapshot_is_exact_direct_release\(predecessor\)',
    'call_order_is_pending_condition\(unit, predecessor\)',
    'completed_cp310_through_cp313_prefix_is_consistent\(unit, system\)',
    'completed_cp313_through_cp315_prefix_is_consistent\(unit, system, predecessor\)',
    '(?s)pending_condition_state_is_consistent\(\s*unit,\s*predecessor,\s*condition_consumer_latest_witness,\s*\)',
    'let snapshot = \{',
    'let unit = runtime\.units\.get_mut\(&selected\)\.ok_or\(',
    'advance_cooling_economizer_condition_state\s*\(',
    'runtime\.set_cooling_economizer_condition_latest_witness\(selected, snapshot\);',
    'debug_assert!\(!snapshot\.economizer_condition_evaluated\);',
    'Ok\(snapshot\)'
) -Description "CP316 runtime-root witness read, immutable validation, scoped unit transition, root publication, and return order"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern '(?s)runtime\.set_cooling_economizer_condition_latest_witness\(selected, snapshot\);\s*debug_assert!\(!snapshot\.economizer_condition_evaluated\);\s*debug_assert!\(!snapshot\.differential_dry_bulb_economizer_type_read\);\s*debug_assert!\(!snapshot\.differential_enthalpy_economizer_type_read\);\s*debug_assert!\(!snapshot\.outdoor_air_temperature_read\);\s*debug_assert!\(!snapshot\.outdoor_air_enthalpy_read\);\s*debug_assert!\(!snapshot\.economizer_calculation_body_entered\);\s*Ok\(snapshot\)\s*\}\s*$' -Description "CP316 runtime-root witness publication followed only by debug assertions and successful return"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'debug_assert!\(!snapshot\.economizer_condition_evaluated\)' -Description "CP316 exact release complete condition skip"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'debug_assert!\(!snapshot\.differential_dry_bulb_economizer_type_read\)' -Description "CP316 exact release zero dry-bulb enum reads"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'debug_assert!\(!snapshot\.differential_enthalpy_economizer_type_read\)' -Description "CP316 exact release zero enthalpy enum reads"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'debug_assert!\(!snapshot\.outdoor_air_temperature_read\)' -Description "CP316 exact release zero temperature reads"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'debug_assert!\(!snapshot\.outdoor_air_enthalpy_read\)' -Description "CP316 exact release zero stored-enthalpy reads"
Assert-Contains -Path $calcCoolingEconomizerConditionRelease -Pattern 'debug_assert!\(!snapshot\.economizer_calculation_body_entered\)' -Description "CP316 exact release zero line-2089 entries"

Assert-Contains -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Pattern 'pub\(super\) fn completed_cp310_through_cp313_prefix_is_consistent\s*\(' -Description "CP316 private retained CP310-through-CP313 entry-prefix helper"
Assert-Contains -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Pattern '(?s)\(\s*unit\.calc_entry\.latest,\s*unit\.calc_minimum_oa_prefix\.latest,\s*unit\.calc_cooling_entry_gate\.latest,\s*unit\.calc_cooling_oa_max_flow_gate\.latest,\s*\)' -Description "CP316 retained entry-prefix latest-snapshot tuple"
Assert-Contains -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Pattern 'entry_snapshot_is_exact_direct_release\(unit, entry\)' -Description "CP316 retained Calc-entry snapshot consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Pattern 'minimum_oa_snapshot_is_exact_direct_release\(minimum_oa\)' -Description "CP316 retained minimum-OA snapshot consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Pattern 'cooling_entry_snapshot_is_exact_direct_release\(cooling_entry, entry\)' -Description "CP316 retained cooling-entry snapshot consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionEntryPrefixValidation -Pattern 'cooling_gate_snapshot_is_exact_direct_release\s*\(' -Description "CP316 retained cooling-gate snapshot consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern 'pub\(super\) fn initialization_state_is_exact_direct_release\s*\(' -Description "CP316 private retained initialization helper"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern 'let topology_outcome = topology_plan\.resolve\(\);' -Description "CP316 retained topology-plan consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern 'let flags = unit\.flags\(runtime\.equipment_list_checked\);' -Description "CP316 retained initialization-flags consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern 'density\.is_finite\(\)\s*[\r\n]+\s*&& density > 0\.0' -Description "CP316 finite-positive initialized density cache"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern '(?s)initialized_mass_flow_has_expected_bits\(\s*system\.heating_limit,\s*expected_sized_limits\.maximum_heating_air_flow_rate_m3_per_s,\s*density,\s*unit\.maximum_heating_air_mass_flow_rate_kg_per_s,\s*\)' -Description "CP316 exact initialized heating mass-flow cache validation"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern '(?s)initialized_mass_flow_has_expected_bits\(\s*system\.cooling_limit,\s*expected_sized_limits\.maximum_cooling_air_flow_rate_m3_per_s,\s*density,\s*unit\.maximum_cooling_air_mass_flow_rate_kg_per_s,\s*\)' -Description "CP316 exact initialized cooling mass-flow cache validation"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern '(?m)^fn initialized_mass_flow_has_expected_bits\s*\(' -Description "CP316 private initialized mass-flow bit validator"
Assert-NotContains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+fn initialized_mass_flow_has_expected_bits\s*\(' -Description "public initialized mass-flow bit validator"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern '(?s)let expected_mass_flow = if matches!\(\s*limit,\s*IdealLoadsLimit::LimitFlowRate \| IdealLoadsLimit::LimitFlowRateAndCapacity\s*\).*?volume_flow \* density\s*\}\s*else\s*\{\s*0\.0\s*\};' -Description "CP316 flow hard-size-times-density and non-flow positive-zero mass-flow expectation"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern 'expected_mass_flow\.is_finite\(\) && actual_mass_flow\.to_bits\(\) == expected_mass_flow\.to_bits\(\)' -Description "CP316 bitwise exact initialized mass-flow cache comparison"
Assert-Contains -Path $calcCoolingEconomizerConditionInitializationValidation -Pattern 'topology_ready && flags_ready && counts_ready && sizing_ready && caches_ready' -Description "CP316 exact retained initialization conjunction"

Assert-Contains -Path $calcCoolingEconomizerConditionPredecessorValidation -Pattern 'pub\(super\) fn economizer_guard_snapshot_is_exact_direct_release\s*\(' -Description "CP316 exact retained CP315 snapshot validator"
Assert-Contains -Path $calcCoolingEconomizerConditionPredecessorValidation -Pattern 'pub\(super\) fn economizer_guard_links_to_body\s*\(' -Description "CP316 exact CP315-to-CP314 link validator"
Assert-Contains -Path $calcCoolingEconomizerConditionPredecessorValidation -Pattern 'pub\(super\) fn cooling_body_snapshot_is_exact_direct_release\s*\(' -Description "CP316 retained CP314 snapshot validator"
Assert-Contains -Path $calcCoolingEconomizerConditionPredecessorValidation -Pattern 'pub\(super\) fn cooling_body_links_to_gate\s*\(' -Description "CP316 retained CP314-to-CP313 link validator"
Assert-Contains -Path $calcCoolingEconomizerConditionPredecessorValidation -Pattern 'pub\(super\) fn cooling_gate_snapshot_is_exact_direct_release\s*\(' -Description "CP316 retained CP313 snapshot validator"
Assert-Contains -Path $calcCoolingEconomizerConditionPredecessorValidation -Pattern 'snapshot\.economizer_type == Some\(OutdoorAirEconomizerType::NoEconomizer\)' -Description "CP316 exact retained NoEconomizer predecessor"

Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'pub\(super\) fn calc_state_identities_match\s*\(' -Description "CP316 retained lifecycle identity validator"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'pub\(super\) fn call_order_is_pending_condition\s*\(' -Description "CP316 one-for-one pending-call validator"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'pub\(super\) fn completed_cp313_through_cp315_prefix_is_consistent\s*\(' -Description "CP316 predecessor-history validator"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern '(?s)pub\(super\) fn pending_condition_state_is_consistent\s*\(\s*unit:\s*&PurchasedAirUnitRuntimeState,\s*predecessor:\s*PurchasedAirCalcCoolingEconomizerGuardSnapshot,\s*condition_consumer_latest_witness:\s*Option<PurchasedAirCalcCoolingEconomizerConditionSnapshot>,\s*\)\s*->\s*bool' -Description "CP316 pending condition-state validator accepting the runtime-root witness"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern '\.transition_count\s*[\r\n]+\s*\.checked_add\(1\)\s*[\r\n]+\s*== Some\(unit\.calc_cooling_economizer_guard\.transition_count\)' -Description "CP316 checked pending predecessor order"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'state\s*[\r\n]+\s*\.condition_evaluation_count\s*[\r\n]+\s*\.checked_add\(state\.unit_off_skip_count\)' -Description "CP316 checked transition partition"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'state\.condition_evaluation_count == 0' -Description "CP316 exact release zero condition evaluations"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'state\.maximum_cooling_flow_body_sibling_skip_count == 0' -Description "CP316 exact release zero internal sibling skips"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'state\s*[\r\n]+\s*\.no_economizer_outer_guard_fallthrough_skip_count\s*[\r\n]+\s*\.checked_add\(usize::from\(predecessor\.no_economizer_fallthrough\)\)' -Description "CP316 outer-false skip links to CP315 fallthrough"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'condition_snapshot_sites_are_skipped\(snapshot\)' -Description "CP316 exact release snapshot-site helper consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'condition_source_counters_are_zero\(state\)' -Description "CP316 exact release source-counter helper consumption"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern '(?s)match \(\s*state\.transition_count,\s*state\.latest,\s*state\.latest_route,\s*state\.latest_transition_ordinal,\s*condition_consumer_latest_witness,\s*\)\s*\{\s*\(0, None, None, None, None\) => true,\s*\(\s*count,\s*Some\(latest\),\s*Some\(retained_route\),\s*Some\(latest_transition_ordinal\),\s*Some\(consumer_witness\),\s*\) if count > 0 => \{\s*latest_transition_ordinal == count\s*&& consumer_witness == latest\s*&& latest\.parent_call_ordinal == count.*?condition_snapshot_route\(latest\) == Some\(retained_route\)' -Description "CP316 five-part latest-generation, route, and exact runtime-root witness validation"
Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern 'fn condition_snapshot_route\s*\(' -Description "CP316 private snapshot-to-route decoder"

foreach ($coolingEconomizerWitnessExclusionFile in @(
    $runPurchasedAirCoolingEconomizerGuard,
    $runPurchasedAirCoolingEconomizerConditionSerialization,
    "specs\algorithm_ledger.toml",
    "specs\capabilities.toml"
)) {
    Assert-NotContains -Path $coolingEconomizerWitnessExclusionFile -Pattern '\b(?:cooling_economizer_condition_latest_witness(?:es)?|condition_consumer_latest_witness|consumer_witness)\b' -Description "private CP315/CP316 runtime-root witness outside runtime validation"
}

$coolingEconomizerConditionZeroCounterNames = @(
    "differential_dry_bulb_economizer_type_read_count",
    "differential_dry_bulb_selector_comparison_count",
    "differential_dry_bulb_selector_match_count",
    "outdoor_air_temperature_read_count",
    "recirculation_air_temperature_read_count",
    "dry_bulb_temperature_comparison_count",
    "dry_bulb_temperature_comparison_satisfied_count",
    "differential_enthalpy_economizer_type_read_count",
    "differential_enthalpy_selector_comparison_count",
    "differential_enthalpy_selector_match_count",
    "outdoor_air_enthalpy_read_count",
    "recirculation_air_enthalpy_read_count",
    "enthalpy_comparison_count",
    "enthalpy_comparison_satisfied_count",
    "economizer_calculation_body_entry_count",
    "economizer_condition_fallthrough_count"
)
foreach ($counterName in $coolingEconomizerConditionZeroCounterNames) {
    $escapedCounterName = [regex]::Escape($counterName)
    Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern ("state\." + $escapedCounterName + "\s*==\s*0") -Description "CP316 runtime zero source counter $counterName"
    Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern ("count!\(" + $escapedCounterName + ",\s*0\)") -Description "CP316 coupled zero source counter $counterName"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern ("\(\s*`"" + $escapedCounterName + "`",\s*0") -Description "CP316 pipeline zero source counter $counterName"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern ("`"" + $escapedCounterName + "`"") -Description "CP316 serialized source counter $counterName"
}

$coolingEconomizerConditionSkippedBooleanFields = @(
    "economizer_condition_evaluated",
    "differential_dry_bulb_economizer_type_read",
    "differential_dry_bulb_selector_comparison_evaluated",
    "outdoor_air_temperature_read",
    "recirculation_air_temperature_read",
    "dry_bulb_temperature_comparison_evaluated",
    "differential_enthalpy_economizer_type_read",
    "differential_enthalpy_selector_comparison_evaluated",
    "outdoor_air_enthalpy_read",
    "recirculation_air_enthalpy_read",
    "enthalpy_comparison_evaluated",
    "economizer_calculation_body_entered",
    "economizer_condition_fallthrough"
)
foreach ($fieldName in $coolingEconomizerConditionSkippedBooleanFields) {
    $escapedFieldName = [regex]::Escape($fieldName)
    Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern ("!snapshot\s*\." + $escapedFieldName) -Description "CP316 runtime skipped Boolean site $fieldName"
    Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern ($escapedFieldName + ":\s*false") -Description "CP316 coupled skipped Boolean site $fieldName"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern ("!condition\s*\." + $escapedFieldName) -Description "CP316 pipeline skipped Boolean site $fieldName"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern ("`"" + $escapedFieldName + "`"") -Description "CP316 serialized Boolean site $fieldName"
}

$coolingEconomizerConditionSkippedOptionFields = @(
    "differential_dry_bulb_economizer_type",
    "differential_dry_bulb_selector_matched",
    "outdoor_air_temperature_c",
    "recirculation_air_temperature_c",
    "outdoor_air_temperature_below_recirculation_temperature",
    "differential_enthalpy_economizer_type",
    "differential_enthalpy_selector_matched",
    "outdoor_air_enthalpy_j_per_kg",
    "recirculation_air_enthalpy_j_per_kg",
    "outdoor_air_enthalpy_below_recirculation_enthalpy",
    "economizer_condition_satisfied"
)
foreach ($fieldName in $coolingEconomizerConditionSkippedOptionFields) {
    $escapedFieldName = [regex]::Escape($fieldName)
    Assert-Contains -Path $calcCoolingEconomizerConditionRuntimeValidation -Pattern ("snapshot\s*\." + $escapedFieldName + "\s*\.is_none\(\)") -Description "CP316 runtime skipped optional site $fieldName"
    Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern ($escapedFieldName + ":\s*None") -Description "CP316 coupled skipped optional site $fieldName"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern ("condition\s*\." + $escapedFieldName + "\s*\.is_none\(\)") -Description "CP316 pipeline skipped optional site $fieldName"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern ("`"" + $escapedFieldName + "`"") -Description "CP316 serialized optional site $fieldName"
}

Assert-Contains -Path $calcCoolingEconomizerConditionTests -Pattern 'compound_condition_preserves_selector_and_or_short_circuit_order' -Description "Calc cooling economizer compound selector and OR short-circuit regression"
Assert-Contains -Path $calcCoolingEconomizerConditionTests -Pattern 'raw_strict_less_than_preserves_nan_signed_zero_and_infinity' -Description "Calc cooling economizer raw IEEE strict-comparison regression"
Assert-Contains -Path $calcCoolingEconomizerConditionTests -Pattern 'unit_off_non_cooling_sibling_and_outer_false_are_four_complete_skips' -Description "Calc cooling economizer four parent-skip partitions regression"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseTests -Pattern 'mod corruption_tests;' -Description "Calc cooling economizer corruption-test child module declaration"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseTests -Pattern 'mod provenance_tests;' -Description "Calc cooling economizer provenance-test child module declaration"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseTests -Pattern 'public_no_oa_condition_never_accepts_or_reads_node_values' -Description "Calc cooling economizer public no-OA zero-node-read regression"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseTests -Pattern 'public_condition_rejects_forgery_replay_overflow_and_prefix_corruption_transactionally' -Description "Calc cooling economizer public forgery, replay, overflow, and prefix transaction regression"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Pattern 'public_condition_rejects_stale_mixed_route_latest_transactionally' -Description "Calc cooling economizer private retained-route stale-mix regression"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Pattern 'public_condition_rejects_whole_state_generation_replay_transactionally' -Description "Calc cooling economizer whole-state generation replay regression"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Pattern '(?s)fn public_condition_rejects_alternate_history_guard_and_condition_splice_transactionally\s*\(\)\s*\{.*?target_unit\.calc_cooling_economizer_guard\s*=\s*donor_unit\.calc_cooling_economizer_guard\.clone\(\);.*?target_unit\.calc_cooling_economizer_condition\s*=\s*donor_unit\.calc_cooling_economizer_condition\.clone\(\);.*?assert_runtime_invariant_without_mutation\(target, &system, target_pending\);\s*\}' -Description "Calc cooling economizer joint guard-and-condition alternate-history splice rejection"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Pattern '(?s)fn public_condition_rejects_alternate_history_whole_unit_transplant_transactionally\s*\(\)\s*\{.*?let donor_unit = donor\.units\.remove\(&SYSTEM\).*?[t]arget\.units\.insert\(SYSTEM, donor_unit\);.*?assert_runtime_invariant_without_mutation\(target, &system, target_pending\);\s*\}' -Description "Calc cooling economizer whole-unit alternate-history transplant rejection"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Pattern '(?s)assert_eq!\(\s*donor_pending, target_pending,\s*"both histories must expose the same third pending NoEconomizer predecessor"\s*\);' -Description "Calc cooling economizer alternate histories expose the exact same pending predecessor"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Pattern 'PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation' -Description "Calc cooling economizer alternate-history rejection marker"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Pattern 'assert_eq!\(runtime, before\);' -Description "Calc cooling economizer alternate-history transactionality marker"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseCorruptionTests -Pattern 'public_condition_rejects_entry_prefix_and_initialization_corruption_transactionally' -Description "Calc cooling economizer retained entry-prefix and initialization corruption regression"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseCorruptionTests -Pattern 'unit\.maximum_heating_air_mass_flow_rate_kg_per_s = 1\.0' -Description "Calc cooling economizer finite-positive heating mass-flow cache corruption regression"
Assert-Contains -Path $calcCoolingEconomizerConditionReleaseCorruptionTests -Pattern 'unit\.maximum_cooling_air_mass_flow_rate_kg_per_s = 1\.0' -Description "Calc cooling economizer finite-positive cooling mass-flow cache corruption regression"

$coolingEconomizerConditionForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'PsyCpAirFnW|PsyHFnTdbW|moist_air_enthalpy_j_per_kg|psychrometric_[A-Za-z0-9_]*enthalpy'; Description = "excluded psychrometric or enthalpy-recomputation behavior in cooling economizer condition boundary" },
    [pscustomobject]@{ Pattern = '(?i)(?:SupplyMassFlowRate|MaxCoolMassFlowRate|OAMassFlowRate|EconoOn|TimeEconoActive)\s*(?:=(?!=)|\+=|-=|\*=|/=)'; Description = "excluded C++ cooling-flow or economizer-body mutation in cooling economizer condition boundary" },
    [pscustomobject]@{ Pattern = '(?i)(?:outdoor_air|oa|supply|economizer)[A-Za-z0-9_\.]*mass_flow_rate[A-Za-z0-9_\.]*\s*(?:=(?!=)|\+=|-=|\*=|/=)'; Description = "excluded Rust cooling-flow or economizer-body mutation in cooling economizer condition boundary" },
    [pscustomobject]@{ Pattern = 'CalcPurchAirMixedAir|calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s|\b(?!set_cooling_economizer_condition_latest_witness\b)(?:calc|apply|update|set|adjust|compute|resolve)_[A-Za-z0-9_]*(?:outdoor_air|economizer|mass_flow)[A-Za-z0-9_]*\s*\('; Description = "separate mixed-air, outdoor-air, economizer, or flow helper in cooling economizer condition boundary" },
    [pscustomobject]@{ Pattern = '\.(?:clamp|min|max)\s*\(|\b(?:DeltaT|delta_t)\b'; Description = "unmapped numerical limiting or delta-temperature behavior in cooling economizer condition boundary" }
)
foreach ($coolingEconomizerConditionBoundaryFile in @(
    $calcCoolingEconomizerCondition,
    $calcCoolingEconomizerConditionTransition,
    $calcCoolingEconomizerConditionRelease,
    $calcCoolingEconomizerConditionEntryPrefixValidation,
    $calcCoolingEconomizerConditionCompletedConditionValidation,
    $calcCoolingEconomizerConditionInitializationValidation,
    $calcCoolingEconomizerConditionPredecessorValidation,
    $calcCoolingEconomizerConditionRuntimeValidation
)) {
    foreach ($forbiddenBehavior in $coolingEconomizerConditionForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingEconomizerConditionBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
foreach ($coolingEconomizerConditionIntegrationEvidenceFile in @(
    $idealLoadsCoupledCoolingEconomizerConditionValidation,
    $runPurchasedAirCoolingEconomizerCondition,
    $runPurchasedAirCoolingEconomizerConditionSerialization
)) {
    foreach ($forbiddenBehavior in $coolingEconomizerConditionForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingEconomizerConditionIntegrationEvidenceFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
Assert-NotContains -Path $calcCoolingEconomizerConditionTransition -Pattern '\.is_finite\(\)' -Description "finite validation in pure cooling economizer condition characterization"

Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE' -Description "Calc cooling economizer true-body source provenance"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling economizer true-body first excluded source"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER' -Description "Calc cooling economizer true-body exact source order"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2089-2101' -Description "Calc cooling economizer exact true-body boundary"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2109' -Description "Calc cooling economizer true-body exact first excluded executable"
Assert-ExactStringArray -Path $calcCoolingEconomizerBody -Name "PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER" -Expected @(
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
    "read-outdoor-air-node-temperature",
    "read-zone-node-temperature",
    "subtract-zone-temperature-from-outdoor-air-temperature",
    "assign-local-delta-temperature",
    "read-delta-temperature-for-small-temperature-difference-gate",
    "compare-strict-delta-temperature-below-negative-small-temperature-difference",
    "enter-delta-temperature-body-if-satisfied",
    "read-zone-cooling-setpoint-load-after-delta-temperature-match",
    "read-local-cp-air-for-first-division",
    "calculate-zone-cooling-setpoint-load-divided-by-cp-air",
    "read-local-delta-temperature-for-second-division",
    "calculate-first-division-intermediate-divided-by-delta-temperature",
    "assign-initial-supply-mass-flow-rate",
    "read-cooling-limit-for-flow-rate",
    "compare-cooling-limit-equal-to-flow-rate",
    "read-cooling-limit-for-flow-rate-and-capacity-after-short-circuit",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-cooling-air-mass-flow-after-selector-match",
    "compare-strict-maximum-cooling-air-mass-flow-above-zero",
    "enter-maximum-flow-clamp-body-if-satisfied",
    "read-supply-mass-flow-rate-for-inner-maximum",
    "apply-source-shaped-maximum-with-zero",
    "reread-maximum-cooling-air-mass-flow-as-clamp-upper-bound",
    "apply-source-shaped-minimum-with-maximum-cooling-air-mass-flow",
    "assign-clamped-supply-mass-flow-rate",
    "read-resulting-supply-mass-flow-rate",
    "read-current-outdoor-air-mass-flow-rate",
    "compare-strict-supply-mass-flow-above-outdoor-air-mass-flow",
    "enter-economizer-activation-body-if-satisfied",
    "assign-economizer-on-true-after-mass-flow-match",
    "reread-supply-mass-flow-for-outdoor-air-mass-flow-assignment",
    "assign-outdoor-air-mass-flow-from-supply-mass-flow",
    "read-system-time-step",
    "assign-economizer-active-time"
) -Description "Calc cooling economizer true-body exact thirty-seven-site source order"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SMALL_TEMP_DIFF_C: f64 = 1\.0e-5' -Description "Calc cooling economizer exact HVAC SmallTempDiff"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'mod release;' -Description "Calc cooling economizer true-body release submodule declaration"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'mod state;' -Description "Calc cooling economizer true-body state submodule declaration"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'mod transition;' -Description "Calc cooling economizer true-body transition submodule declaration"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub use release::\*;' -Description "Calc cooling economizer true-body release re-export"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub use state::PurchasedAirCalcCoolingEconomizerBodyRuntimeState;' -Description "Calc cooling economizer true-body state re-export"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub\(super\) use transition::advance_cooling_economizer_body_state;' -Description "Calc cooling economizer bounded internal true-body transition visibility"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerBodySnapshot' -Description "Calc cooling economizer source-ordered true-body snapshot"
Assert-Contains -Path $calcCoolingEconomizerBodyState -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerBodyRuntimeState' -Description "Calc cooling economizer bounded true-body persistent state"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub struct PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary' -Description "Calc cooling economizer true-body lifecycle summary"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub fn purchased_air_calc_cooling_economizer_body_lifecycle_summary\s*\(' -Description "Calc cooling economizer true-body lifecycle summary accessor"
Assert-Contains -Path $calcCoolingEconomizerBodyState -Pattern 'pub body_execution_count: usize' -Description "Calc cooling economizer true-body execution counter"
Assert-Contains -Path $calcCoolingEconomizerBody -Pattern 'pub economizer_calculation_body_executed: bool' -Description "Calc cooling economizer true-body execution evidence"
Assert-Contains -Path $calcCoolingEconomizerBodyState -Pattern 'pub economizer_condition_fallthrough_skip_count: usize' -Description "Calc cooling economizer CP316-false complete-skip counter"
Assert-Contains -Path $calcCoolingEconomizerBodyState -Pattern 'pub maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count: usize' -Description "Calc cooling economizer repeated clamp-upper-bound read counter"
Assert-Contains -Path $calcCoolingEconomizerBodyState -Pattern 'pub supply_mass_flow_rate_for_outdoor_air_assignment_read_count: usize' -Description "Calc cooling economizer repeated supply-assignment read counter"
Assert-NotContains -Path $calcCoolingEconomizerBody -Pattern 'fn advance_cooling_economizer_body_state\s*\(' -Description "cooling economizer true-body transition implementation in module facade"
Assert-NotContains -Path $calcCoolingEconomizerBody -Pattern '#\[test\]' -Description "unit test body in cooling economizer true-body facade"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)#\[derive\(Clone, Debug, Default, PartialEq\)\]\s*pub struct PurchasedAirRuntimeState\s*\{.*?cooling_economizer_body_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingEconomizerBodySnapshot>,' -Description "runtime-root default-empty per-system CP317 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_economizer_body_latest_witnesses:' -Description "public runtime-root CP317 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern '(?s)pub\(in crate::ideal_loads\) fn cooling_economizer_body_latest_witness\s*\(\s*&self,\s*system:\s*IdealLoadsAirSystemId,\s*\)\s*->\s*Option<PurchasedAirCalcCoolingEconomizerBodySnapshot>\s*\{\s*self\.cooling_economizer_body_latest_witnesses\s*\.get\(&system\)\s*\.copied\(\)\s*\}' -Description "ideal_loads-scoped runtime-root CP317 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern '(?s)pub\(in crate::ideal_loads\) fn set_cooling_economizer_body_latest_witness\s*\(\s*&mut self,\s*system:\s*IdealLoadsAirSystemId,\s*snapshot:\s*PurchasedAirCalcCoolingEconomizerBodySnapshot,\s*\)\s*\{\s*self\.cooling_economizer_body_latest_witnesses\s*\.insert\(system, snapshot\);\s*\}' -Description "ideal_loads-scoped runtime-root CP317 witness setter"
Assert-NotContains -Path $calcCoolingEconomizerCondition -Pattern '\b(?:cooling_economizer_body_latest_witness(?:es)?|body_consumer_latest_witness|PurchasedAirCalcCoolingEconomizerBodySnapshot)\b' -Description "CP317 witness ownership, accessor, setter, or snapshot import in CP316 condition state"
Assert-NotContains -Path $calcCoolingEconomizerConditionTransition -Pattern '\b(?:cooling_economizer_body_latest_witness(?:es)?|body_consumer_latest_witness)\b' -Description "CP316 transition mutation of the runtime-root CP317 witness"

Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_economizer_body_state\s*\(' -Description "Calc cooling economizer source-characterization true-body transition"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern 'predecessor\.economizer_calculation_body_entered' -Description "Calc cooling economizer exact CP316 body-entry gate"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '(?:map\s*\(\s*energyplus_psy_cp_air_fn_w\s*\)|energyplus_psy_cp_air_fn_w\s*\()' -Description "Calc cooling economizer canonical PsyCpAirFnW implementation"
Assert-NotContains -Path $calcCoolingEconomizerBodyTransition -Pattern 'energyplus_moist_air_specific_heat_j_per_kg_k' -Description "legacy NaN-normalizing moist-air specific-heat helper in CP317"
Assert-NotContains -Path $calcCoolingEconomizerBodyTransition -Pattern '\.then_some\s*\(\s*input\.' -Description "eager conditional CP317 source input read"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '\.map\(\|\(outdoor_air, zone\)\| outdoor_air - zone\)' -Description "Calc cooling economizer captured outdoor-minus-Zone delta"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern 'delta\s*<\s*-PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SMALL_TEMP_DIFF_C' -Description "Calc cooling economizer strict negative-small-difference gate"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '\.map\(\|\(load, cp_air\)\| load / cp_air\)' -Description "Calc cooling economizer first source supply-flow division"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '\.map\(\|\(load_over_cp, delta_temperature\)\| load_over_cp / delta_temperature\)' -Description "Calc cooling economizer left-associated second supply-flow division"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '== IdealLoadsLimit::LimitFlowRate' -Description "Calc cooling economizer first flow-limit selector comparison"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '== IdealLoadsLimit::LimitFlowRateAndCapacity' -Description "Calc cooling economizer second flow-limit selector comparison"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern 'maximum\s*>\s*0\.0' -Description "Calc cooling economizer strict positive maximum-flow comparison"
Assert-PatternsInOrder -Path $calcCoolingEconomizerBodyTransition -Patterns @(
    'let nonnegative = source_max\(supply_for_clamp, 0\.0\);',
    'let clamp_upper_bound = input\.maximum_cooling_air_mass_flow_rate_kg_per_s;',
    'let clamped = source_min\(nonnegative, clamp_upper_bound\);'
) -Description "Calc cooling economizer source-ordered supply reread, inner maximum, maximum reread, and outer minimum"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '(?s)fn source_min\s*\(\s*left:\s*f64,\s*right:\s*f64\s*\)\s*->\s*f64\s*\{\s*if left < right \{\s*left\s*\}\s*else\s*\{\s*right\s*\}\s*\}' -Description "Calc cooling economizer Objexx min NaN operand-order semantics"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '(?s)fn source_max\s*\(\s*left:\s*f64,\s*right:\s*f64\s*\)\s*->\s*f64\s*\{\s*if left < right \{\s*right\s*\}\s*else\s*\{\s*left\s*\}\s*\}' -Description "Calc cooling economizer Objexx max NaN operand-order semantics"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern '\.map\(\|\(supply, outdoor_air\)\| supply > outdoor_air\)' -Description "Calc cooling economizer strict supply-above-outdoor-air comparison"
Assert-PatternsInOrder -Path $calcCoolingEconomizerBodyTransition -Patterns @(
    'let resulting_supply_mass_flow_rate_kg_per_s =',
    'let outdoor_air_mass_flow_rate_kg_per_s =',
    'let supply_mass_flow_above_outdoor_air_mass_flow =',
    'let economizer_activation_body_entered =',
    'let economizer_on =',
    'let supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s =',
    'let assigned_outdoor_air_mass_flow_rate_kg_per_s =',
    'let system_time_step_hours =',
    'let assigned_economizer_active_time_hours ='
) -Description "Calc cooling economizer exact final comparison and conditional assignment read order"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern 'economizer_on_assigned' -Description "Calc cooling economizer conditional EconoOn assignment evidence"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern 'assigned_outdoor_air_mass_flow_rate_kg_per_s' -Description "Calc cooling economizer conditional outdoor-air flow assignment evidence"
Assert-Contains -Path $calcCoolingEconomizerBodyTransition -Pattern 'assigned_economizer_active_time_hours' -Description "Calc cooling economizer conditional active-time assignment evidence"
Assert-NotContains -Path $calcCoolingEconomizerBodyTransition -Pattern '\.(?:clamp|min|max)\s*\(' -Description "Rust f64 clamp/min/max substitution in CP317 Objexx-shaped clamp"
Assert-NotContains -Path $calcCoolingEconomizerBodyTransition -Pattern '\.is_finite\(\)' -Description "finite validation in pure cooling economizer true-body characterization"

Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'pub enum PurchasedAirCalcCoolingEconomizerBodyError' -Description "Calc cooling economizer true-body fail-closed error"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_economizer_body\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingEconomizerConditionSnapshot,\s*\)\s*->\s*Result<\s*PurchasedAirCalcCoolingEconomizerBodySnapshot,\s*PurchasedAirCalcCoolingEconomizerBodyError,?\s*>\s*\{' -Description "Calc cooling economizer true-body exact public no-node release signature"
Assert-NotContains -Path $calcCoolingEconomizerBodyRelease -Pattern '\b(?:NodeId|NodeStateStore|AirNodeState)\b' -Description "live Node dependency in public cooling economizer true-body release boundary"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP317 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'PredecessorOutsideDirectSubset' -Description "CP317 exact release predecessor-shape guard"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'RuntimeStateInvariantViolation' -Description "CP317 retained release-state guard"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'classify_no_oa_sensible_subset\(system\)\.is_supported\(\)' -Description "CP317 exact no-OA release subset validation"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'cooling_economizer_body_latest_witness\(selected\)' -Description "CP317 runtime-root witness read"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'set_cooling_economizer_body_latest_witness\(selected, snapshot\)' -Description "CP317 runtime-root witness publication"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'debug_assert!\(!snapshot\.economizer_calculation_body_executed\)' -Description "CP317 exact release zero true-body executions"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'debug_assert!\(!snapshot\.psychrometric_cp_air_evaluated\)' -Description "CP317 exact release zero psychrometric calls"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'debug_assert!\(!snapshot\.economizer_on_assigned\)' -Description "CP317 exact release zero EconoOn assignments"

Assert-Contains -Path $calcCoolingEconomizerBodySourceOrderTests -Pattern 'source_order_preserves_delta_limit_clamp_and_assignment_short_circuits' -Description "Calc cooling economizer true-body source-order characterization regression"
Assert-Contains -Path $calcCoolingEconomizerBodyTests -Pattern 'mod gate_and_assignment_tests;' -Description "Calc cooling economizer gate and assignment test module declaration"
Assert-Contains -Path $calcCoolingEconomizerBodyGateAndAssignmentTests -Pattern 'strict_delta_temperature_gate_preserves_boundary_nan_and_infinity_behavior' -Description "Calc cooling economizer strict delta-temperature gate characterization regression"
Assert-Contains -Path $calcCoolingEconomizerBodyGateAndAssignmentTests -Pattern 'strict_final_comparison_controls_all_four_economizer_assignments' -Description "Calc cooling economizer strict final comparison and assignment characterization regression"
Assert-Contains -Path $calcCoolingEconomizerBodyIeeeTests -Pattern 'objexx_min_max_preserve_nan_and_signed_zero_operand_order' -Description "Calc cooling economizer Objexx min/max edge-semantics regression"
Assert-Contains -Path $calcCoolingEconomizerBodySkipTests -Pattern 'unit_off_non_cooling_outer_false_and_condition_false_are_complete_skips' -Description "Calc cooling economizer true-body complete parent-skip partition regression"
Assert-Contains -Path $calcCoolingEconomizerBodyReleaseTests -Pattern 'public_no_oa_body_never_reads_or_mutates_calculation_sites' -Description "Calc cooling economizer public no-OA zero-site regression"
Assert-Contains -Path $calcCoolingEconomizerBodyReleaseTests -Pattern 'exact_release_snapshot_rejects_impossible_skip_route_predecessor_flags' -Description "Calc cooling economizer impossible predecessor-route shape regression"
Assert-Contains -Path $calcCoolingEconomizerBodyReleaseTests -Pattern 'public_body_rejects_forgery_replay_overflow_and_prefix_corruption_transactionally' -Description "Calc cooling economizer true-body forgery, replay, overflow, and prefix transaction regression"
Assert-Contains -Path $calcCoolingEconomizerBodyReleaseProvenanceTests -Pattern 'public_body_rejects_alternate_history_condition_and_body_splice_transactionally' -Description "Calc cooling economizer true-body alternate-history splice rejection"
Assert-Contains -Path $calcCoolingEconomizerBodyReleaseProvenanceTests -Pattern 'public_body_rejects_alternate_history_whole_unit_transplant_transactionally' -Description "Calc cooling economizer true-body whole-unit transplant rejection"
Assert-Contains -Path $calcCoolingEconomizerBodyReleaseCorruptionTests -Pattern 'public_body_rejects_entry_prefix_and_initialization_corruption_transactionally' -Description "Calc cooling economizer true-body retained prefix corruption regression"

$coolingEconomizerBodyForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'PurchasedAirManager\.cc:(2110|2111|2348)'; Description = "later cooling or Heat/DeadBand provenance in cooling economizer true-body boundary" },
    [pscustomobject]@{ Pattern = 'SupplyMassFlowRateForCool|MinCoolSuppAirTemp|CalcPurchAirMixedAir'; Description = "excluded common cooling-flow reset or mixed-air behavior in cooling economizer true-body boundary" },
    [pscustomobject]@{ Pattern = 'PsyHFnTdbW|moist_air_enthalpy_j_per_kg|psychrometric_[A-Za-z0-9_]*enthalpy'; Description = "excluded enthalpy calculation in cooling economizer true-body boundary" },
    [pscustomobject]@{ Pattern = 'calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s'; Description = "separate outdoor-air economizer helper in cooling economizer true-body boundary" },
    [pscustomobject]@{ Pattern = 'OutdoorAirEconomizerType::(?:DifferentialDryBulb|DifferentialEnthalpy)'; Description = "re-evaluation of CP316 economizer selectors in CP317" }
)
foreach ($coolingEconomizerBodyBoundaryFile in @(
    $calcCoolingEconomizerBody,
    $calcCoolingEconomizerBodyState,
    $calcCoolingEconomizerBodyTransition,
    $calcCoolingEconomizerBodyRelease,
    $calcCoolingEconomizerBodyEntryPrefixValidation,
    $calcCoolingEconomizerBodyInitializationValidation,
    $calcCoolingEconomizerBodyPredecessorValidation,
    $calcCoolingEconomizerBodyRuntimeValidation
)) {
    foreach ($forbiddenBehavior in $coolingEconomizerBodyForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingEconomizerBodyBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}

Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE' -Description "Calc cooling sensible-flow source provenance"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling sensible-flow first excluded source"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER' -Description "Calc cooling sensible-flow exact source order"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2109-2116' -Description "Calc cooling sensible-flow exact CP318 boundary"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2119' -Description "Calc cooling sensible-flow exact first excluded executable"
Assert-ExactStringArray -Path $calcCoolingSensibleFlow -Name "PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER" -Expected @(
    "assign-supply-mass-flow-rate-for-cool-zero",
    "read-cooling-on",
    "enter-cooling-on-body-if-true",
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
    "read-minimum-cooling-supply-air-temperature",
    "read-zone-node-temperature",
    "subtract-zone-temperature-from-minimum-cooling-supply-air-temperature",
    "assign-local-delta-temperature",
    "read-delta-temperature-for-small-temperature-difference-gate",
    "compare-strict-delta-temperature-below-negative-small-temperature-difference",
    "enter-delta-temperature-body-if-satisfied",
    "read-zone-cooling-setpoint-load-after-delta-temperature-match",
    "read-local-cp-air-for-first-division",
    "calculate-zone-cooling-setpoint-load-divided-by-cp-air",
    "read-local-delta-temperature-for-second-division",
    "calculate-first-division-intermediate-divided-by-delta-temperature",
    "assign-supply-mass-flow-rate-for-cool"
) -Description "Calc cooling sensible-flow exact nineteen-site source order"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C: f64 = 1\.0e-5' -Description "Calc cooling sensible-flow exact HVAC SmallTempDiff"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub\(in crate::ideal_loads::calc\) mod release;' -Description "Calc cooling sensible-flow bounded release submodule"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'mod state;' -Description "Calc cooling sensible-flow state submodule"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'mod transition;' -Description "Calc cooling sensible-flow transition submodule"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub use release::\*;' -Description "Calc cooling sensible-flow release re-export"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub use state::PurchasedAirCalcCoolingSensibleFlowRuntimeState;' -Description "Calc cooling sensible-flow state re-export"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub struct PurchasedAirCalcCoolingSensibleFlowSnapshot' -Description "Calc cooling sensible-flow source-ordered snapshot"
Assert-Contains -Path $calcCoolingSensibleFlowState -Pattern 'pub struct PurchasedAirCalcCoolingSensibleFlowRuntimeState' -Description "Calc cooling sensible-flow persistent state"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub struct PurchasedAirCalcCoolingSensibleFlowLifecycleSummary' -Description "Calc cooling sensible-flow lifecycle summary"
Assert-Contains -Path $calcCoolingSensibleFlow -Pattern 'pub fn purchased_air_calc_cooling_sensible_flow_lifecycle_summary\s*\(' -Description "Calc cooling sensible-flow lifecycle summary accessor"
Assert-Contains -Path $calcCoolingSensibleFlowState -Pattern 'pub cooling_body_entry_count: usize' -Description "Calc cooling sensible-flow common Cooling reconvergence counter"
Assert-Contains -Path $calcCoolingSensibleFlowState -Pattern 'pub supply_mass_flow_rate_for_cool_reset_assignment_count: usize' -Description "Calc cooling sensible-flow positive-zero reset counter"
Assert-Contains -Path $calcCoolingSensibleFlowState -Pattern 'pub cooling_on_fallthrough_count: usize' -Description "Calc cooling sensible-flow CoolOn-false counter"
Assert-Contains -Path $calcCoolingSensibleFlowState -Pattern 'pub delta_temperature_fallthrough_count: usize' -Description "Calc cooling sensible-flow strict-gate fallthrough counter"
Assert-Contains -Path $calcCoolingSensibleFlowState -Pattern 'pub\(super\) latest_route:' -Description "Calc cooling sensible-flow private retained route"
Assert-Contains -Path $calcCoolingSensibleFlowState -Pattern 'pub\(super\) latest_transition_ordinal:' -Description "Calc cooling sensible-flow private retained generation"
Assert-NotContains -Path $calcCoolingSensibleFlow -Pattern '#\[test\]' -Description "unit test body in cooling sensible-flow facade"

Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_sensible_flow_state\s*\(' -Description "Calc cooling sensible-flow pure source transition"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern 'let cooling_body_entered = predecessor\.predecessor_cooling_body_entered;' -Description "Calc cooling sensible-flow common Cooling reconvergence criterion"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern 'cooling_body_entered\.then_some\(0\.0_f64\)' -Description "Calc cooling sensible-flow positive-zero reset"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '(?s)let cooling_on = if cooling_body_entered \{\s*Some\(input\.cooling_on\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling sensible-flow lazy CoolOn read"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '(?s)let zone_humidity_ratio = if cooling_on_body_entered \{\s*Some\(input\.zone_humidity_ratio\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling sensible-flow lazy Zone humidity read"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '\.map\(energyplus_psy_cp_air_fn_w\)' -Description "Calc cooling sensible-flow canonical PsyCpAirFnW scalar"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '(?s)let minimum_cooling_supply_air_temperature_c = if cooling_on_body_entered \{\s*Some\(input\.minimum_cooling_supply_air_temperature_c\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling sensible-flow lazy minimum supply-temperature read"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '(?s)let zone_temperature_c = if cooling_on_body_entered \{\s*Some\(input\.zone_temperature_c\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling sensible-flow lazy Zone-temperature read"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '\.map\(\|\(minimum_supply, zone\)\| minimum_supply - zone\)' -Description "Calc cooling sensible-flow captured minimum-supply-minus-Zone delta"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern 'delta < -PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C' -Description "Calc cooling sensible-flow strict negative-small-difference gate"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '(?s)let zone_cooling_setpoint_load_w = if delta_temperature_body_entered \{\s*Some\(input\.zone_cooling_setpoint_load_w\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling sensible-flow lazy QZnCoolSP read"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '\.map\(\|\(load, cp_air\)\| load / cp_air\)' -Description "Calc cooling sensible-flow first left-associated division"
Assert-Contains -Path $calcCoolingSensibleFlowTransition -Pattern '\.map\(\|\(first_division, delta\)\| first_division / delta\)' -Description "Calc cooling sensible-flow second left-associated division"
Assert-NotContains -Path $calcCoolingSensibleFlowTransition -Pattern '\.then_some\s*\(\s*input\.' -Description "eager conditional CP318 source input read"
Assert-NotContains -Path $calcCoolingSensibleFlowTransition -Pattern '\.is_finite\(\)' -Description "finite normalization in pure CP318 characterization"
Assert-NotContains -Path $calcCoolingSensibleFlowTransition -Pattern '(?:cp_air|delta(?:_temperature)?)\s*\*\s*(?:delta(?:_temperature)?|cp_air)' -Description "denominator-product rewrite in CP318"
Assert-NotContains -Path $calcCoolingSensibleFlowTransition -Pattern '\.(?:abs|clamp|min|max)\s*\(' -Description "normalizing or limiting rewrite in CP318"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_sensible_flow_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSensibleFlowSnapshot>' -Description "runtime-root default-empty per-system CP318 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_sensible_flow_latest_witnesses:' -Description "public runtime-root CP318 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_sensible_flow_latest_witness\s*\(' -Description "ideal_loads-scoped runtime-root CP318 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_sensible_flow_latest_witness\s*\(' -Description "ideal_loads-scoped runtime-root CP318 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_sensible_flow: PurchasedAirCalcCoolingSensibleFlowRuntimeState' -Description "per-unit CP318 persistent state"
Assert-Contains -Path $calcCoolingEconomizerBodyCompletedBodyValidation -Pattern 'pub\(super\) fn completed_body_state_is_consistent\s*\(' -Description "CP317 completed-state validator for CP318"
Assert-Contains -Path $calcCoolingEconomizerBodyCompletedBodyValidation -Pattern 'body_consumer_latest_witness' -Description "CP317 exact consumer-witness validation"
Assert-Contains -Path $calcCoolingEconomizerBodyRelease -Pattern 'pub\(in crate::ideal_loads::calc\) fn completed_direct_cooling_economizer_body_is_consistent\s*\(' -Description "CP317 narrow completed-state export for CP318"

Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'pub enum PurchasedAirCalcCoolingSensibleFlowError' -Description "Calc cooling sensible-flow fail-closed error"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_sensible_flow\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingEconomizerBodySnapshot,\s*zone_state:\s*&ZoneHeatBalanceState,\s*\)' -Description "Calc cooling sensible-flow exact public release signature"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'ZoneIdentityMismatch' -Description "CP318 bound Zone identity guard"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'NonFiniteActiveInput' -Description "CP318 active finite-input guard"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP318 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'PredecessorOutsideDirectSubset' -Description "CP318 exact predecessor-shape guard"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'classify_no_oa_sensible_subset\(system\)\.is_supported\(\)' -Description "CP318 exact no-OA release subset validation"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'cooling_on: entry\.cooling_on' -Description "CP318 retained CP310 CoolOn derivation"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'zone_cooling_setpoint_load_w: entry\.demand\.remaining_output_req_to_cool_sp_w' -Description "CP318 retained CP310 QZnCoolSP derivation"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'zone_humidity_ratio: zone_state\.air_humidity_ratio' -Description "CP318 bound live Zone humidity input"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'zone_temperature_c: zone_state\.mean_air_temperature_c' -Description "CP318 bound live Zone temperature input"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'if predecessor\.predecessor_cooling_body_entered \{\s*validate_active_input\(selected, input\)\?;' -Description "CP318 active-only pre-mutation finite validation"
Assert-PatternsInOrder -Path $calcCoolingSensibleFlowRelease -Patterns @(
    'if predecessor\.predecessor_cooling_body_entered',
    'validate_active_input\(selected, input\)\?;',
    '\.get_mut\(&selected\)',
    'advance_cooling_sensible_flow_state',
    'set_cooling_sensible_flow_latest_witness\(selected, snapshot\)'
) -Description "CP318 validation-before-mutation and witness publication order"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'completed_direct_cooling_economizer_body_is_consistent' -Description "CP318 exact completed CP317 state consumption"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'pending_sensible_flow_state_is_consistent' -Description "CP318 pending runtime-state validation"
Assert-Contains -Path $calcCoolingSensibleFlowSnapshotValidation -Pattern 'pub\(in crate::ideal_loads\) fn cooling_sensible_flow_snapshot_is_exact_direct_release\s*\(' -Description "CP318 crate-private exact direct snapshot validator"
Assert-Contains -Path $calcCoolingSensibleFlowSnapshotValidation -Pattern 'snapshot\.cooling_on == Some\(true\)' -Description "CP318 exact direct retained CoolOn true"
Assert-Contains -Path $calcCoolingSensibleFlowSnapshotValidation -Pattern 'let first_division = load / cp_air;' -Description "CP318 exact direct first division"
Assert-Contains -Path $calcCoolingSensibleFlowSnapshotValidation -Pattern 'let supply_flow = first_division / delta_temperature;' -Description "CP318 exact direct left-associated second division"
Assert-Contains -Path $calcCoolingSensibleFlowRuntimeValidation -Pattern 'call_order_is_pending_sensible_flow' -Description "CP318 pending one-for-one call-order validator"
Assert-Contains -Path $calcCoolingSensibleFlowRuntimeValidation -Pattern 'cooling_on_fallthrough_count == 0' -Description "CP318 direct CoolOn-true lifecycle invariant"
Assert-Contains -Path $calcCoolingSensibleFlowRuntimeValidation -Pattern 'cooling_sensible_flow_snapshot_route\(latest\) == Some\(retained_route\)' -Description "CP318 private retained-route validation"

$coolingSensibleFlowForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'SupplyMassFlowRateForDehum|SupplyMassFlowRateForHum'; Description = "excluded humidity-flow reset in CP318 boundary" },
    [pscustomobject]@{ Pattern = 'CalcPurchAirMixedAir|VerySmallMassFlow'; Description = "excluded mixed-air or later flow cutoff in CP318 boundary" },
    [pscustomobject]@{ Pattern = '\b(?:EMS|Ems)\b|ems_override'; Description = "excluded EMS behavior in CP318 boundary" },
    [pscustomobject]@{ Pattern = 'energyplus_moist_air_specific_heat_j_per_kg_k'; Description = "legacy NaN-normalizing specific-heat helper in CP318 boundary" },
    [pscustomobject]@{ Pattern = '\b(?:calc_no_oa|cooling_mass_flow_candidate|sensible_mass_flow)\b'; Description = "pre-existing numerical helper reuse in CP318 boundary" },
    [pscustomobject]@{ Pattern = '\b(?:dwSave|cpaSave)\b|-100\.0'; Description = "excluded PsyCpAirFnW static cache lifecycle in CP318 boundary" },
    [pscustomobject]@{ Pattern = '0\.001'; Description = "legacy numerical mass-flow threshold in CP318 boundary" }
)
foreach ($coolingSensibleFlowBoundaryFile in @(
    $calcCoolingSensibleFlow,
    $calcCoolingSensibleFlowState,
    $calcCoolingSensibleFlowTransition,
    $calcCoolingSensibleFlowRelease,
    $calcCoolingSensibleFlowPredecessorValidation,
    $calcCoolingSensibleFlowRuntimeValidation,
    $calcCoolingSensibleFlowSnapshotValidation
)) {
    foreach ($forbiddenBehavior in $coolingSensibleFlowForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingSensibleFlowBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
Assert-Contains -Path $calcCoolingSensibleFlowTests -Pattern 'mod source_order_tests;' -Description "CP318 source-order test split"
Assert-Contains -Path $calcCoolingSensibleFlowTests -Pattern 'mod skip_tests;' -Description "CP318 skip test split"
Assert-Contains -Path $calcCoolingSensibleFlowTests -Pattern 'mod gate_tests;' -Description "CP318 gate and IEEE test split"
Assert-Contains -Path $calcCoolingSensibleFlowSourceOrderTests -Pattern 'source_boundary_and_all_nineteen_sites_are_stable' -Description "CP318 exact source-boundary regression"
Assert-Contains -Path $calcCoolingSensibleFlowSourceOrderTests -Pattern 'active_route_uses_canonical_psychrometrics_and_left_associated_divisions' -Description "CP318 canonical Cp and division-order regression"
Assert-Contains -Path $calcCoolingSensibleFlowSkipTests -Pattern 'unit_off_skips_every_source_site_with_poisoned_inputs' -Description "CP318 UnitOff complete-skip regression"
Assert-Contains -Path $calcCoolingSensibleFlowSkipTests -Pattern 'active_non_cooling_skips_every_source_site_with_poisoned_inputs' -Description "CP318 non-cooling complete-skip regression"
Assert-Contains -Path $calcCoolingSensibleFlowGateTests -Pattern 'strict_small_temperature_gate_falls_through_at_exact_negative_threshold' -Description "CP318 strict threshold equality regression"
Assert-Contains -Path $calcCoolingSensibleFlowGateTests -Pattern 'delta_just_below_negative_threshold_enters_assignment_body' -Description "CP318 strict threshold next-value regression"
Assert-Contains -Path $calcCoolingSensibleFlowGateTests -Pattern 'false_cooling_availability_only_executes_reset_and_cooling_on_read' -Description "CP318 CoolOn-false lazy-read regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseTests -Pattern 'public_active_cooling_executes_exact_left_associated_source_route' -Description "CP318 exact public active-release regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseTests -Pattern 'public_non_cooling_route_skips_poisoned_source_inputs' -Description "CP318 public non-cooling skip regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseTests -Pattern 'public_active_delta_fallthrough_retains_the_positive_zero_reset' -Description "CP318 public reset-only fallthrough regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseTests -Pattern 'repeated_release_calls_preserve_one_for_one_history_across_route_changes' -Description "CP318 public history-order regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseCorruptionTests -Pattern 'forged_predecessor_and_replay_are_rejected_without_mutation' -Description "CP318 forgery and replay transaction regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseCorruptionTests -Pattern 'zone_identity_and_active_nonfinite_inputs_fail_transactionally' -Description "CP318 bound-Zone and active-input transaction regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseCorruptionTests -Pattern 'retained_count_corruption_fails_before_any_cp318_mutation' -Description "CP318 retained-count corruption regression"
Assert-Contains -Path $calcCoolingSensibleFlowReleaseCorruptionTests -Pattern 'exact_release_validator_rejects_forged_arithmetic_and_provenance' -Description "CP318 exact snapshot forgery regression"
Assert-Contains -Path $idealLoadsBindingCoolingSensibleFlowTests -Pattern 'scheduled_binding_executes_cooling_sensible_flow_before_numerical_coupling' -Description "CP318 scheduled positive-cooling binding regression"
Assert-Contains -Path $idealLoadsBindingCoolingSensibleFlowTests -Pattern 'scheduled_binding_records_unit_off_and_non_cooling_cp318_skips' -Description "CP318 scheduled skip binding regression"
Assert-Contains -Path $idealLoadsBindingCoolingSensibleFlowTests -Pattern 'public_cooling_sensible_flow_replay_is_whole_state_transactional' -Description "CP318 scheduled replay transaction regression"

Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE' -Description "Calc cooling dehumidification-flow source provenance"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling dehumidification-flow first excluded source"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER' -Description "Calc cooling dehumidification-flow exact source order"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2119-2128' -Description "Calc cooling dehumidification-flow exact CP319 boundary"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2133' -Description "Calc cooling dehumidification-flow exact first excluded executable"
Assert-ExactStringArray -Path $calcCoolingDehumidificationFlow -Name "PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER" -Expected @(
    "assign-supply-mass-flow-rate-for-dehumidification-zero",
    "read-cooling-on-for-dehumidification",
    "enter-cooling-on-dehumidification-body-if-true",
    "read-dehumidification-control-type",
    "compare-dehumidification-control-type-equal-to-humidistat",
    "enter-humidistat-dehumidification-body-if-matched",
    "read-zone-dehumidifying-setpoint-moisture-demand",
    "assign-local-zone-dehumidifying-setpoint-moisture-demand",
    "read-minimum-cooling-supply-air-humidity-ratio",
    "read-zone-node-humidity-ratio",
    "subtract-zone-humidity-ratio-from-minimum-cooling-supply-air-humidity-ratio",
    "assign-local-delta-humidity-ratio",
    "read-delta-humidity-ratio-for-small-difference-gate",
    "compare-strict-delta-humidity-ratio-below-negative-small-delta-humidity-ratio",
    "read-zone-dehumidifying-setpoint-moisture-demand-after-delta-match",
    "compare-strict-zone-dehumidifying-setpoint-moisture-demand-below-zero",
    "enter-dehumidification-flow-body-if-compound-condition-satisfied",
    "reread-zone-dehumidifying-setpoint-moisture-demand-for-division",
    "reread-delta-humidity-ratio-for-division",
    "calculate-zone-dehumidifying-setpoint-moisture-demand-divided-by-delta-humidity-ratio",
    "assign-supply-mass-flow-rate-for-dehumidification"
) -Description "Calc cooling dehumidification-flow exact twenty-one-site source order"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO: f64 =\s*[\r\n]+\s*0\.00025' -Description "Calc cooling dehumidification-flow exact SmallDeltaHumRat"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub\(in crate::ideal_loads::calc\) mod release;' -Description "Calc cooling dehumidification-flow bounded release submodule"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'mod state;' -Description "Calc cooling dehumidification-flow state submodule"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'mod transition;' -Description "Calc cooling dehumidification-flow transition submodule"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub use release::\*;' -Description "Calc cooling dehumidification-flow release re-export"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub use state::PurchasedAirCalcCoolingDehumidificationFlowRuntimeState;' -Description "Calc cooling dehumidification-flow state re-export"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub struct PurchasedAirCalcCoolingDehumidificationFlowSnapshot' -Description "Calc cooling dehumidification-flow source-ordered snapshot"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub struct PurchasedAirCalcCoolingDehumidificationFlowRuntimeState' -Description "Calc cooling dehumidification-flow persistent state"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub struct PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary' -Description "Calc cooling dehumidification-flow lifecycle summary"
Assert-Contains -Path $calcCoolingDehumidificationFlow -Pattern 'pub fn purchased_air_calc_cooling_dehumidification_flow_lifecycle_summary\s*\(' -Description "Calc cooling dehumidification-flow lifecycle summary accessor"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub cooling_body_entry_count: usize' -Description "Calc cooling dehumidification-flow common Cooling reconvergence counter"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub supply_mass_flow_rate_for_dehumidification_reset_assignment_count: usize' -Description "Calc cooling dehumidification-flow positive-zero reset counter"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub dehumidification_control_type_fallthrough_count: usize' -Description "Calc cooling dehumidification-flow selector fallthrough counter"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub delta_humidity_ratio_fallthrough_count: usize' -Description "Calc cooling dehumidification-flow delta gate fallthrough counter"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub zone_dehumidifying_setpoint_moisture_demand_fallthrough_count: usize' -Description "Calc cooling dehumidification-flow moisture gate fallthrough counter"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub\(super\) latest_route:' -Description "Calc cooling dehumidification-flow private retained route"
Assert-Contains -Path $calcCoolingDehumidificationFlowState -Pattern 'pub\(super\) latest_transition_ordinal:' -Description "Calc cooling dehumidification-flow private retained generation"
Assert-NotContains -Path $calcCoolingDehumidificationFlow -Pattern '#\[test\]' -Description "unit test body in cooling dehumidification-flow facade"

Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_dehumidification_flow_state\s*\(' -Description "Calc cooling dehumidification-flow pure source transition"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern 'let cooling_body_entered = predecessor\.cooling_body_entered;' -Description "Calc cooling dehumidification-flow common Cooling reconvergence criterion"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern 'cooling_body_entered\.then_some\(0\.0_f64\)' -Description "Calc cooling dehumidification-flow positive-zero reset"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '(?s)let cooling_on = if cooling_body_entered \{\s*Some\(input\.cooling_on\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling dehumidification-flow lazy CoolOn read"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '(?s)let dehumidification_control_type = if cooling_on_body_entered \{\s*Some\(input\.dehumidification_control_type\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling dehumidification-flow lazy selector read"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '(?s)let zone_dehumidifying_setpoint_moisture_demand_kg_per_s =\s*if dehumidification_control_body_entered \{\s*Some\(input\.zone_dehumidifying_setpoint_moisture_demand_kg_per_s\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling dehumidification-flow lazy moisture-demand read"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '(?s)let minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air =\s*if dehumidification_control_body_entered \{\s*Some\(\s*input\.minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air\s*\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling dehumidification-flow lazy minimum supply humidity read"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '(?s)let zone_humidity_ratio_kg_water_per_kg_dry_air =\s*if dehumidification_control_body_entered \{\s*Some\(input\.zone_humidity_ratio_kg_water_per_kg_dry_air\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling dehumidification-flow lazy Zone humidity read"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '\.map\(\|\(minimum_supply, zone\)\| minimum_supply - zone\)' -Description "Calc cooling dehumidification-flow source humidity subtraction"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern 'delta\s*< -PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO' -Description "Calc cooling dehumidification-flow strict negative-small-difference gate"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '(?s)if delta_humidity_ratio_comparison_satisfied \{\s*assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s\s*\} else \{\s*None\s*\}' -Description "Calc cooling dehumidification-flow left-to-right compound short circuit"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '\.map\(\|demand\| demand < 0\.0\)' -Description "Calc cooling dehumidification-flow strict negative moisture-demand gate"
Assert-Contains -Path $calcCoolingDehumidificationFlowTransition -Pattern '\.map\(\|\(demand, delta\)\| demand / delta\)' -Description "Calc cooling dehumidification-flow raw source division"
Assert-NotContains -Path $calcCoolingDehumidificationFlowTransition -Pattern '\.then_some\s*\(\s*input\.' -Description "eager conditional CP319 source input read"
Assert-NotContains -Path $calcCoolingDehumidificationFlowTransition -Pattern '\.is_finite\(\)' -Description "finite normalization in pure CP319 characterization"
Assert-NotContains -Path $calcCoolingDehumidificationFlowTransition -Pattern '\.(?:abs|clamp|min|max)\s*\(' -Description "normalizing or limiting rewrite in CP319"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_dehumidification_flow_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingDehumidificationFlowSnapshot>' -Description "runtime-root default-empty per-system CP319 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_dehumidification_flow_latest_witnesses:' -Description "public runtime-root CP319 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_dehumidification_flow_latest_witness\s*\(' -Description "ideal_loads-scoped runtime-root CP319 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_dehumidification_flow_latest_witness\s*\(' -Description "ideal_loads-scoped runtime-root CP319 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_dehumidification_flow:\s*PurchasedAirCalcCoolingDehumidificationFlowRuntimeState' -Description "per-unit CP319 persistent state"
Assert-Contains -Path $calcCoolingSensibleFlowCompletedStateValidation -Pattern 'pub\(super\) fn completed_sensible_flow_state_is_consistent\s*\(' -Description "CP318 completed-state validator for CP319"
Assert-Contains -Path $calcCoolingSensibleFlowCompletedStateValidation -Pattern 'flow_consumer_latest_witness' -Description "CP318 exact CP319 consumer-witness validation"
Assert-Contains -Path $calcCoolingSensibleFlowRelease -Pattern 'pub\(in crate::ideal_loads::calc\) fn completed_direct_cooling_sensible_flow_is_consistent\s*\(' -Description "CP318 narrow completed-state export for CP319"

Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'pub enum PurchasedAirCalcCoolingDehumidificationFlowError' -Description "Calc cooling dehumidification-flow fail-closed error"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_dehumidification_flow\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingSensibleFlowSnapshot,\s*\)' -Description "Calc cooling dehumidification-flow exact no-service public release signature"
Assert-NotContains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'ZoneHeatBalanceState' -Description "live Zone state argument in CP319 public release"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP319 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'PredecessorOutsideDirectSubset' -Description "CP319 exact predecessor-shape guard"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'classify_no_oa_sensible_subset\(system\)\.is_supported\(\)' -Description "CP319 exact no-OA release subset validation"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'system\.dehumidification_control_type != DehumidificationControlType::None' -Description "CP319 explicit ConstantSHR and humidity-selector rejection"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'cooling_on: entry\.cooling_on' -Description "CP319 retained CP310 CoolOn derivation"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'dehumidification_control_type: system\.dehumidification_control_type' -Description "CP319 static selector input"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64::NAN' -Description "CP319 poisoned skipped live moisture input"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'zone_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN' -Description "CP319 poisoned skipped live Zone humidity input"
Assert-PatternsInOrder -Path $calcCoolingDehumidificationFlowRelease -Patterns @(
    'completed_direct_cooling_sensible_flow_is_consistent',
    'pending_dehumidification_flow_state_is_consistent',
    '\.get_mut\(&selected\)',
    'advance_cooling_dehumidification_flow_state',
    'set_cooling_dehumidification_flow_latest_witness\(selected, snapshot\)'
) -Description "CP319 validation-before-mutation and witness publication order"
Assert-Contains -Path $calcCoolingDehumidificationFlowSnapshotValidation -Pattern 'pub\(in crate::ideal_loads\) fn cooling_dehumidification_flow_snapshot_is_exact_direct_release\s*\(' -Description "CP319 crate-private exact direct snapshot validator"
Assert-Contains -Path $calcCoolingDehumidificationFlowSnapshotValidation -Pattern 'snapshot\.dehumidification_control_type == Some\(DehumidificationControlType::None\)' -Description "CP319 exact direct selector fallthrough"
Assert-Contains -Path $calcCoolingDehumidificationFlowSnapshotValidation -Pattern 'value\.to_bits\(\) == expected\.to_bits\(\)' -Description "CP319 signed-zero bitwise validation"
Assert-Contains -Path $calcCoolingDehumidificationFlowRuntimeValidation -Pattern 'call_order_is_pending_dehumidification_flow' -Description "CP319 pending one-for-one call-order validator"
Assert-Contains -Path $calcCoolingDehumidificationFlowRuntimeValidation -Pattern 'dehumidification_control_type_humidistat_count == 0' -Description "CP319 direct selector-false lifecycle invariant"
Assert-Contains -Path $calcCoolingDehumidificationFlowRuntimeValidation -Pattern 'downstream_source_counters_are_zero' -Description "CP319 direct no-live-service counter invariant"
Assert-Contains -Path $calcCoolingDehumidificationFlowRuntimeValidation -Pattern 'cooling_dehumidification_flow_snapshot_route\(latest\) == Some\(retained_route\)' -Description "CP319 private retained-route validation"

$coolingDehumidificationFlowForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'SupplyMassFlowRateForHumid'; Description = "excluded humidification-flow reset in CP319 boundary" },
    [pscustomobject]@{ Pattern = 'CalcPurchAirMixedAir|VerySmallMassFlow'; Description = "excluded mixed-air or later flow cutoff in CP319 boundary" },
    [pscustomobject]@{ Pattern = '\b(?:EMS|Ems)\b|ems_override'; Description = "excluded EMS behavior in CP319 boundary" },
    [pscustomobject]@{ Pattern = 'humidistat_dehumidification_mass_flow_rate_kg_per_s'; Description = "later capacity-zero and normalized humidity helper reuse in CP319 boundary" },
    [pscustomobject]@{ Pattern = '\b(?:calc_no_oa|cooling_mass_flow_candidate|sensible_mass_flow)\b'; Description = "pre-existing numerical helper reuse in CP319 boundary" },
    [pscustomobject]@{ Pattern = 'energyplus_psy_|psychrometric'; Description = "psychrometric work outside CP319 boundary" },
    [pscustomobject]@{ Pattern = '\.max\s*\(\s*0\.0'; Description = "positive normalization in CP319 boundary" },
    [pscustomobject]@{ Pattern = '\bstatic\s+mut\b|\bOnceLock\b'; Description = "mutable static cache ownership in CP319 boundary" }
)
foreach ($coolingDehumidificationFlowBoundaryFile in @(
    $calcCoolingDehumidificationFlow,
    $calcCoolingDehumidificationFlowState,
    $calcCoolingDehumidificationFlowTransition,
    $calcCoolingDehumidificationFlowRelease,
    $calcCoolingDehumidificationFlowPredecessorValidation,
    $calcCoolingDehumidificationFlowRuntimeValidation,
    $calcCoolingDehumidificationFlowSnapshotValidation
)) {
    foreach ($forbiddenBehavior in $coolingDehumidificationFlowForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingDehumidificationFlowBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
Assert-Contains -Path $calcCoolingDehumidificationFlowTests -Pattern 'mod source_order_tests;' -Description "CP319 source-order test split"
Assert-Contains -Path $calcCoolingDehumidificationFlowTests -Pattern 'mod skip_tests;' -Description "CP319 skip test split"
Assert-Contains -Path $calcCoolingDehumidificationFlowTests -Pattern 'mod gate_tests;' -Description "CP319 gate and IEEE test split"
Assert-Contains -Path $calcCoolingDehumidificationFlowSourceOrderTests -Pattern 'source_boundary_and_all_twenty_one_sites_are_stable' -Description "CP319 exact source-boundary regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowSourceOrderTests -Pattern 'humidistat_route_preserves_source_reads_and_single_division' -Description "CP319 Humidistat source-order and division regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowSkipTests -Pattern 'unit_off_skips_every_source_site_with_poisoned_inputs' -Description "CP319 UnitOff complete-skip regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowSkipTests -Pattern 'active_non_cooling_skips_every_source_site_with_poisoned_inputs' -Description "CP319 non-cooling complete-skip regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowGateTests -Pattern 'false_cooling_availability_only_executes_reset_and_cooling_on_read' -Description "CP319 CoolOn-false lazy-read regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowGateTests -Pattern 'non_humidistat_control_skips_all_live_humidity_inputs' -Description "CP319 selector-fallthrough lazy-read regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowGateTests -Pattern 'exact_negative_delta_threshold_short_circuits_second_gate_read' -Description "CP319 strict delta threshold and left-to-right short-circuit regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowGateTests -Pattern 'negative_zero_moisture_demand_falls_through_second_strict_gate' -Description "CP319 strict moisture threshold signed-zero regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowGateTests -Pattern 'raw_ieee_division_is_not_clamped_or_reassociated' -Description "CP319 raw IEEE division regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseTests -Pattern 'public_active_cooling_proves_none_control_and_skips_live_humidity_sites' -Description "CP319 exact public active-release regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseTests -Pattern 'public_non_cooling_route_skips_the_entire_cp319_slice' -Description "CP319 public non-cooling skip regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Pattern 'forged_predecessor_and_replay_are_rejected_without_mutation' -Description "CP319 forgery and replay transaction regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Pattern 'humidistat_model_is_rejected_transactionally_without_live_service_input' -Description "CP319 Humidistat rejection transaction regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Pattern 'constant_sensible_heat_ratio_model_is_rejected_transactionally_by_none_guard' -Description "CP319 explicit ConstantSHR rejection transaction regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Pattern 'retained_count_corruption_fails_before_any_cp319_mutation' -Description "CP319 retained-count corruption regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Pattern 'completed_cp318_source_counter_corruption_fails_transactionally' -Description "CP319 completed-predecessor corruption regression"
Assert-Contains -Path $calcCoolingDehumidificationFlowReleaseCorruptionTests -Pattern 'exact_release_validator_rejects_forged_reset_and_provenance' -Description "CP319 exact snapshot forgery regression"
Assert-Contains -Path $idealLoadsBindingCoolingDehumidificationFlowTests -Pattern 'scheduled_binding_executes_cooling_dehumidification_flow_before_numerical_coupling' -Description "CP319 scheduled active-cooling binding regression"
Assert-Contains -Path $idealLoadsBindingCoolingDehumidificationFlowTests -Pattern 'scheduled_binding_records_unit_off_and_non_cooling_cp319_skips' -Description "CP319 scheduled skip binding regression"
Assert-Contains -Path $idealLoadsBindingCoolingDehumidificationFlowTests -Pattern 'public_cooling_dehumidification_flow_replay_is_whole_state_transactional' -Description "CP319 scheduled replay transaction regression"

Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE' -Description "Calc cooling humidification-flow source provenance"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE' -Description "Calc cooling humidification-flow first excluded source"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub const PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER' -Description "Calc cooling humidification-flow exact source order"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2133-2144' -Description "Calc cooling humidification-flow exact CP320 boundary"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2147' -Description "Calc cooling humidification-flow exact first excluded executable"
Assert-ExactStringArray -Path $calcCoolingHumidificationFlow -Name "PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER" -Expected @(
    "assign-supply-mass-flow-rate-for-humidification-zero",
    "read-heating-on",
    "enter-heating-on-body-if-true",
    "read-humidification-control-type",
    "compare-humidification-control-type-equal-to-humidistat",
    "enter-humidistat-control-body-if-matched",
    "read-dehumidification-control-type-for-humidistat-comparison",
    "compare-dehumidification-control-type-equal-to-humidistat",
    "read-dehumidification-control-type-for-none-comparison-after-first-false",
    "compare-dehumidification-control-type-equal-to-none",
    "enter-admitted-humidification-body-if-control-condition-satisfied",
    "read-zone-humidifying-setpoint-moisture-demand",
    "assign-local-zone-humidifying-setpoint-moisture-demand",
    "read-maximum-heating-supply-air-humidity-ratio",
    "read-zone-node-humidity-ratio",
    "subtract-zone-humidity-ratio-from-maximum-heating-supply-air-humidity-ratio",
    "assign-local-delta-humidity-ratio",
    "read-delta-humidity-ratio-for-small-difference-gate",
    "compare-strict-delta-humidity-ratio-above-small-delta-humidity-ratio",
    "read-zone-humidifying-setpoint-moisture-demand-after-delta-match",
    "compare-strict-zone-humidifying-setpoint-moisture-demand-above-zero",
    "enter-humidification-flow-body-if-compound-condition-satisfied",
    "reread-zone-humidifying-setpoint-moisture-demand-for-division",
    "reread-delta-humidity-ratio-for-division",
    "calculate-zone-humidifying-setpoint-moisture-demand-divided-by-delta-humidity-ratio",
    "assign-supply-mass-flow-rate-for-humidification"
) -Description "Calc cooling humidification-flow exact twenty-six-site source order"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO: f64\s*=\s*0\.00025' -Description "Calc cooling humidification-flow exact SmallDeltaHumRat"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub\(in crate::ideal_loads::calc\) mod release;' -Description "Calc cooling humidification-flow bounded release submodule"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'mod state;' -Description "Calc cooling humidification-flow state submodule"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'mod transition;' -Description "Calc cooling humidification-flow transition submodule"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub use release::\*;' -Description "Calc cooling humidification-flow release re-export"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub use state::PurchasedAirCalcCoolingHumidificationFlowRuntimeState;' -Description "Calc cooling humidification-flow state re-export"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub struct PurchasedAirCalcCoolingHumidificationFlowSnapshot' -Description "Calc cooling humidification-flow source-ordered snapshot"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub struct PurchasedAirCalcCoolingHumidificationFlowRuntimeState' -Description "Calc cooling humidification-flow persistent state"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub struct PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary' -Description "Calc cooling humidification-flow lifecycle summary"
Assert-Contains -Path $calcCoolingHumidificationFlow -Pattern 'pub fn purchased_air_calc_cooling_humidification_flow_lifecycle_summary\s*\(' -Description "Calc cooling humidification-flow lifecycle summary accessor"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub cooling_body_entry_count: usize' -Description "Calc cooling humidification-flow common Cooling reconvergence counter"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub reset_assignment_count: usize' -Description "Calc cooling humidification-flow positive-zero reset counter"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub heating_on_fallthrough_count: usize' -Description "Calc cooling humidification-flow HeatOn fallthrough counter"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub dehumidification_control_type_second_read_count: usize' -Description "Calc cooling humidification-flow repeated selector read counter"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub delta_fallthrough_count: usize' -Description "Calc cooling humidification-flow delta gate fallthrough counter"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub moisture_demand_fallthrough_count: usize' -Description "Calc cooling humidification-flow moisture gate fallthrough counter"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub\(super\) latest_route:' -Description "Calc cooling humidification-flow private retained route"
Assert-Contains -Path $calcCoolingHumidificationFlowState -Pattern 'pub\(super\) latest_transition_ordinal:' -Description "Calc cooling humidification-flow private retained generation"
Assert-NotContains -Path $calcCoolingHumidificationFlow -Pattern '#\[test\]' -Description "unit test body in cooling humidification-flow facade"

Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_humidification_flow_state\s*\(' -Description "Calc cooling humidification-flow pure source transition"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'predecessor: PurchasedAirCalcCoolingDehumidificationFlowSnapshot' -Description "Calc cooling humidification-flow immediate CP319 predecessor type"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'let cooling = predecessor\.cooling_body_entered;' -Description "Calc cooling humidification-flow immediate CP319 Cooling reconvergence"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'let reset = cooling\.then_some\(0\.0_f64\);' -Description "Calc cooling humidification-flow positive-zero reset"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let heating_on = if cooling \{\s*Some\(input\.heating_on\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow lazy retained HeatOn read"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let humid_control = if heating_body \{\s*Some\(input\.humidification_control_type\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow lazy outer selector read"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'HumidificationControlType::Humidistat' -Description "Calc cooling humidification-flow outer Humidistat comparison"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let first_dehumid_control = if humid_body \{\s*Some\(input\.dehumidification_control_type\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow lazy first nested selector read"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let second_dehumid_control = if dehumid_is_humidistat == Some\(false\) \{\s*Some\(input\.dehumidification_control_type\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow repeated nested selector read after first false"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'second_dehumid_control\.map\(\|value\| value == DehumidificationControlType::None\)' -Description "Calc cooling humidification-flow second nested None comparison"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'dehumid_is_humidistat == Some\(true\) \|\| dehumid_is_none == Some\(true\)' -Description "Calc cooling humidification-flow left-to-right nested selector OR"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let demand = if controls_admitted \{\s*Some\(input\.zone_humidifying_setpoint_moisture_demand_kg_per_s\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow lazy moisture-demand read"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let max_supply = if controls_admitted \{\s*Some\(input\.maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow lazy maximum supply humidity read"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let zone = if controls_admitted \{\s*Some\(input\.zone_humidity_ratio_kg_water_per_kg_dry_air\)\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow lazy Zone humidity read"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'max_supply\.zip\(zone\)\.map\(\|\(supply, zone\)\| supply - zone\)' -Description "Calc cooling humidification-flow source humidity subtraction"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'value > PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO' -Description "Calc cooling humidification-flow strict positive SmallDeltaHumRat gate"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '(?s)let demand_gate = if delta_above == Some\(true\) \{\s*demand\s*\} else \{\s*None\s*\};' -Description "Calc cooling humidification-flow left-to-right compound short circuit"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'demand_gate\.map\(\|value\| value > 0\.0\)' -Description "Calc cooling humidification-flow strict positive moisture-demand gate"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern '\.map\(\|\(demand, delta\)\| demand / delta\)' -Description "Calc cooling humidification-flow raw source division"
Assert-Contains -Path $calcCoolingHumidificationFlowTransition -Pattern 'predecessor_cooling_body_entered: predecessor\.cooling_body_entered' -Description "Calc cooling humidification-flow immediate CP319 snapshot link"
Assert-NotContains -Path $calcCoolingHumidificationFlowTransition -Pattern 'predecessor_cooling_body_entered: predecessor\.predecessor_cooling_body_entered' -Description "stale transitive CP318 link in CP320 snapshot"
Assert-NotContains -Path $calcCoolingHumidificationFlowTransition -Pattern '\.then_some\s*\(\s*input\.' -Description "eager conditional CP320 source input read"
Assert-NotContains -Path $calcCoolingHumidificationFlowTransition -Pattern '\.is_finite\(\)' -Description "finite normalization in pure CP320 characterization"
Assert-NotContains -Path $calcCoolingHumidificationFlowTransition -Pattern '\.(?:abs|clamp|min|max)\s*\(' -Description "normalizing or limiting rewrite in CP320"
Assert-NotContains -Path $calcCoolingHumidificationFlowTransition -Pattern 'input\.cooling_on|zone_dehumidifying_setpoint|minimum_cooling_supply|below_negative|demand < 0\.0' -Description "stale CP319 semantics in CP320 transition"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_humidification_flow_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidificationFlowSnapshot>' -Description "runtime-root default-empty per-system CP320 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_humidification_flow_latest_witnesses:' -Description "public runtime-root CP320 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_humidification_flow_latest_witness\s*\(' -Description "ideal_loads-scoped runtime-root CP320 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_humidification_flow_latest_witness\s*\(' -Description "ideal_loads-scoped runtime-root CP320 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_humidification_flow:\s*PurchasedAirCalcCoolingHumidificationFlowRuntimeState' -Description "per-unit CP320 persistent state"
Assert-Contains -Path $calcCoolingDehumidificationFlowCompletedStateValidation -Pattern 'pub\(super\) fn completed_dehumidification_flow_state_is_consistent\s*\(' -Description "CP319 completed-state validator for CP320"
Assert-Contains -Path $calcCoolingDehumidificationFlowCompletedStateValidation -Pattern 'consumer_witness' -Description "CP319 exact CP320 consumer-witness validation"
Assert-Contains -Path $calcCoolingDehumidificationFlowRelease -Pattern 'pub\(in crate::ideal_loads::calc\) fn completed_direct_cooling_dehumidification_flow_is_consistent\s*\(' -Description "CP319 narrow completed-state export for CP320"

Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'pub enum PurchasedAirCalcCoolingHumidificationFlowError' -Description "Calc cooling humidification-flow fail-closed error"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_humidification_flow\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp319:\s*PurchasedAirCalcCoolingDehumidificationFlowSnapshot,\s*\)' -Description "Calc cooling humidification-flow exact no-service public release signature"
Assert-NotContains -Path $calcCoolingHumidificationFlowRelease -Pattern 'ZoneHeatBalanceState' -Description "live Zone state argument in CP320 public release"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'PredecessorCallOrder' -Description "CP310-through-CP320 one-for-one source-order guard"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'PredecessorOutsideDirectSubset' -Description "CP320 exact predecessor-shape guard"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'classify_no_oa_sensible_subset\(system\)\.is_supported\(\)' -Description "CP320 exact no-OA release subset validation"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'system\.humidification_control_type != HumidificationControlType::None' -Description "CP320 explicit outer HumidificationControl None guard"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'system\.dehumidification_control_type != DehumidificationControlType::None' -Description "CP320 explicit nested DehumidificationControl None guard"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern '(?s)let heating_on = unit\s*\.calc_entry\s*\.latest.*?\.heating_on;' -Description "CP320 retained same-call HeatOn derivation"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'zone_humidifying_setpoint_moisture_demand_kg_per_s: f64::NAN' -Description "CP320 poisoned skipped live moisture input"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN' -Description "CP320 poisoned skipped maximum supply humidity input"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'zone_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN' -Description "CP320 poisoned skipped live Zone humidity input"
Assert-PatternsInOrder -Path $calcCoolingHumidificationFlowRelease -Patterns @(
    'completed_direct_cooling_dehumidification_flow_is_consistent',
    'pending_humidification_flow_state_is_consistent',
    '\.get_mut\(&selected\)',
    'advance_cooling_humidification_flow_state',
    'set_cooling_humidification_flow_latest_witness\(selected, snapshot\)'
) -Description "CP320 validation-before-mutation and witness publication order"
Assert-Contains -Path $calcCoolingHumidificationFlowPredecessorValidation -Pattern 'pub\(super\) fn humidification_flow_links_to_dehumidification_flow\s*\(' -Description "CP320 immediate predecessor-link validator"
Assert-Contains -Path $calcCoolingHumidificationFlowSnapshotValidation -Pattern 'pub\(in crate::ideal_loads\) fn cooling_humidification_flow_snapshot_is_exact_direct_release\s*\(' -Description "CP320 crate-private exact direct snapshot validator"
Assert-Contains -Path $calcCoolingHumidificationFlowSnapshotValidation -Pattern 'snapshot\.humidification_control_type == Some\(HumidificationControlType::None\)' -Description "CP320 exact direct outer selector fallthrough"
Assert-Contains -Path $calcCoolingHumidificationFlowSnapshotValidation -Pattern 'downstream_sites_are_skipped\(snapshot\)' -Description "CP320 exact direct no-live-service sites"
Assert-Contains -Path $calcCoolingHumidificationFlowSnapshotValidation -Pattern 'value\.to_bits\(\) == expected\.to_bits\(\)' -Description "CP320 signed-zero bitwise validation"
Assert-Contains -Path $calcCoolingHumidificationFlowRuntimeValidation -Pattern 'call_order_is_pending_humidification_flow' -Description "CP320 pending one-for-one call-order validator"
Assert-Contains -Path $calcCoolingHumidificationFlowRuntimeValidation -Pattern 'humidification_control_type_humidistat_count == 0' -Description "CP320 direct outer selector-false lifecycle invariant"
Assert-Contains -Path $calcCoolingHumidificationFlowRuntimeValidation -Pattern 'downstream_counts_are_zero' -Description "CP320 direct no-live-service counter invariant"
Assert-Contains -Path $calcCoolingHumidificationFlowRuntimeValidation -Pattern 'cooling_humidification_flow_snapshot_route\(latest\) == Some\(route\)' -Description "CP320 private retained-route validation"

$coolingHumidificationFlowForbiddenBehaviorPatterns = @(
    [pscustomobject]@{ Pattern = 'SupplyMassFlowRateForCool|SupplyMassFlowRateForDehum'; Description = "prior cooling-flow assignment in CP320 boundary" },
    [pscustomobject]@{ Pattern = 'CalcPurchAirMixedAir|VerySmallMassFlow'; Description = "excluded mixed-air or later flow cutoff in CP320 boundary" },
    [pscustomobject]@{ Pattern = '\b(?:EMS|Ems)\b|ems_override'; Description = "excluded EMS behavior in CP320 boundary" },
    [pscustomobject]@{ Pattern = 'humidistat_(?:humidification|dehumidification)_mass_flow_rate_kg_per_s'; Description = "later normalized humidity helper reuse in CP320 boundary" },
    [pscustomobject]@{ Pattern = '\b(?:calc_no_oa|cooling_mass_flow_candidate|sensible_mass_flow)\b'; Description = "pre-existing numerical helper reuse in CP320 boundary" },
    [pscustomobject]@{ Pattern = 'energyplus_psy_|psychrometric'; Description = "psychrometric work outside CP320 boundary" },
    [pscustomobject]@{ Pattern = '\.max\s*\(\s*0\.0'; Description = "positive normalization in CP320 boundary" },
    [pscustomobject]@{ Pattern = '\bstatic\s+mut\b|\bOnceLock\b'; Description = "mutable static cache ownership in CP320 boundary" }
)
foreach ($coolingHumidificationFlowBoundaryFile in @(
    $calcCoolingHumidificationFlow,
    $calcCoolingHumidificationFlowState,
    $calcCoolingHumidificationFlowTransition,
    $calcCoolingHumidificationFlowRelease,
    $calcCoolingHumidificationFlowPredecessorValidation,
    $calcCoolingHumidificationFlowRuntimeValidation,
    $calcCoolingHumidificationFlowSnapshotValidation
)) {
    foreach ($forbiddenBehavior in $coolingHumidificationFlowForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingHumidificationFlowBoundaryFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
foreach ($coolingHumidificationFlowPostBoundaryFile in @(
    $calcCoolingHumidificationFlowState,
    $calcCoolingHumidificationFlowTransition,
    $calcCoolingHumidificationFlowRelease,
    $calcCoolingHumidificationFlowPredecessorValidation,
    $calcCoolingHumidificationFlowRuntimeValidation,
    $calcCoolingHumidificationFlowSnapshotValidation
)) {
    Assert-NotContains -Path $coolingHumidificationFlowPostBoundaryFile -Pattern 'PurchasedAirManager\.cc:2147|PurchasedAirManager\.cc:2155|capacity.*zero' -Description "post-CP320 source behavior in CP320 implementation"
}
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'source_boundary_and_all_twenty_six_sites_are_stable' -Description "CP320 exact source-boundary regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'cp320_links_to_the_immediate_cp319_cooling_body' -Description "CP320 immediate-predecessor link regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'humidistat_dehumidification_short_circuits_second_or_read' -Description "CP320 nested Humidistat OR short-circuit regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'none_dehumidification_requires_the_second_repeated_read' -Description "CP320 repeated None selector read regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'non_humidistat_outer_control_skips_every_live_input' -Description "CP320 outer selector lazy-read regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'false_heating_availability_stops_before_control_reads' -Description "CP320 HeatOn-false lazy-read regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'other_dehumidification_control_repeats_read_then_rejects_body' -Description "CP320 nested selector reject regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'strict_positive_delta_gate_short_circuits_demand_gate_at_equality' -Description "CP320 strict delta threshold short-circuit regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'positive_zero_demand_falls_through_strict_second_gate' -Description "CP320 strict moisture threshold signed-zero regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'raw_ieee_division_is_not_normalized' -Description "CP320 raw IEEE division regression"
Assert-Contains -Path $calcCoolingHumidificationFlowTests -Pattern 'unit_off_and_non_cooling_skip_all_sites' -Description "CP320 complete skip regression"
Assert-Contains -Path $calcCoolingHumidificationFlowReleaseTests -Pattern 'active_direct_route_uses_retained_heating_on_and_skips_live_services' -Description "CP320 exact public active-release regression"
Assert-Contains -Path $calcCoolingHumidificationFlowReleaseTests -Pattern 'non_cooling_release_skips_all_twenty_six_sites' -Description "CP320 public non-cooling skip regression"
Assert-Contains -Path $calcCoolingHumidificationFlowReleaseTests -Pattern 'failure_is_transactional' -Description "CP320 unsupported-selector transaction regression"
Assert-Contains -Path $calcCoolingHumidificationFlowReleaseTests -Pattern 'corrupted_pending_state_fails_without_partial_mutation' -Description "CP320 corrupted-state transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingHumidificationFlowTests -Pattern 'scheduled_binding_executes_cooling_humidification_flow_before_numerical_coupling' -Description "CP320 scheduled active-cooling binding regression"
Assert-Contains -Path $idealLoadsBindingCoolingHumidificationFlowTests -Pattern 'scheduled_binding_records_unit_off_and_non_cooling_cp320_skips' -Description "CP320 scheduled skip binding regression"
Assert-Contains -Path $idealLoadsBindingCoolingHumidificationFlowTests -Pattern 'public_cooling_humidification_flow_replay_is_whole_state_transactional' -Description "CP320 scheduled replay transaction regression"

Assert-Contains -Path $idealLoadsInit -Pattern 'pub struct IdealLoadsInitFlags' -Description "IdealLoads init flags type"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod manager_plan;' -Description "IdealLoads immutable manager-plan module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod manager_plan_tests;' -Description "IdealLoads manager-plan test module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod manager_scan_tests;' -Description "IdealLoads manager-sweep test module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod summary;' -Description "IdealLoads lifecycle-summary module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod state;' -Description "IdealLoads persistent init-state module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod supply_temperature_diagnostic;' -Description "IdealLoads supply-temperature diagnostic module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod supply_temperature_diagnostic_tests;' -Description "IdealLoads supply-temperature diagnostic test module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod topology_plan;' -Description "IdealLoads selected-unit topology-plan module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod topology_plan_tests;' -Description "IdealLoads selected-unit topology-plan test module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod topology_transition;' -Description "IdealLoads selected-unit topology-transition module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod topology_transition_tests;' -Description "IdealLoads selected-unit topology-transition test module"
Assert-Contains -Path $idealLoadsInit -Pattern 'mod transition;' -Description "IdealLoads init-transition module"
Assert-Contains -Path $idealLoadsInit -Pattern 'pub use summary::\*;' -Description "IdealLoads lifecycle-summary public re-export"
Assert-Contains -Path $idealLoadsInit -Pattern 'pub use supply_temperature_diagnostic::\*;' -Description "IdealLoads supply-temperature diagnostic public re-export"
Assert-Contains -Path $idealLoadsInit -Pattern 'pub use topology_plan::\*;' -Description "IdealLoads selected-unit topology-plan public re-export"
Assert-Contains -Path $idealLoadsInit -Pattern 'pub const PURCHASED_AIR_INIT_LIFECYCLE_SOURCE' -Description "IdealLoads persistent init provenance"
Assert-Contains -Path $idealLoadsInit -Pattern 'pub const fn diagnostic_adapter_assumed_ready\s*\(' -Description "IdealLoads diagnostic-only assumed-ready flags"
Assert-Contains -Path $idealLoadsInit -Pattern 'pub topology_ready: bool' -Description "IdealLoads selected-unit topology-ready flag"
Assert-NotContains -Path $idealLoadsInit -Pattern 'source_order_candidate\s*\(' -Description "obsolete all-true source-order constructor"
Assert-Contains -Path $idealLoadsInitManagerPlan -Pattern 'pub struct PurchasedAirInitManagerPlan' -Description "immutable PurchasedAir manager plan"
Assert-Contains -Path $idealLoadsInitManagerPlan -Pattern 'pub struct PurchasedAirInitManagerPlanRow' -Description "declaration-order PurchasedAir manager-plan row"
Assert-Contains -Path $idealLoadsInitManagerPlan -Pattern 'pub fn from_model\s*\(' -Description "typed-model PurchasedAir manager-plan builder"
Assert-Contains -Path $idealLoadsInitManagerPlan -Pattern 'pub fn system_order\s*\(' -Description "PurchasedAir manager-plan declaration-order iterator"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub struct PurchasedAirInitLifecycleSummary' -Description "PurchasedAir selected-unit lifecycle summary"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub recirculation_source: Option<PurchasedAirRecirculationSource>' -Description "PurchasedAir summary recirculation branch"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub topology_diagnostics: Vec<PurchasedAirInitTopologyDiagnostic>' -Description "PurchasedAir summary ordered topology diagnostics"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub topology_failure: Option<PurchasedAirInitTopologyError>' -Description "PurchasedAir summary retained topology failure"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub topology_completion_count: usize' -Description "PurchasedAir summary topology completion count"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub sizing_attempt_count: usize' -Description "PurchasedAir summary sizing attempt count"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub sized_limits: Option<PurchasedAirSizedLimits>' -Description "PurchasedAir summary sized overlay"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub sizing_outcome: Option<PurchasedAirHardSizeLegacyOutcome>' -Description "PurchasedAir summary sizing outcome"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub supply_temperature_registered_recurring_diagnostic_count: usize' -Description "PurchasedAir summary registered recurring count"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub supply_temperature_diagnostic_event_count: usize' -Description "PurchasedAir summary recurring event count"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub supply_temperature_characterized_severe_error_count_increment: usize' -Description "PurchasedAir summary characterized severe increment"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub cooling_supply_temperature_error_index: usize' -Description "PurchasedAir summary cooling recurring index"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub heating_supply_temperature_error_index: usize' -Description "PurchasedAir summary heating recurring index"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub supply_temperature_diagnostics: Vec<PurchasedAirSupplyTemperatureDiagnostic>' -Description "PurchasedAir summary global recurring identities"
Assert-Contains -Path $idealLoadsInitSummary -Pattern 'pub economizer_flow_limit_warning_count: usize' -Description "PurchasedAir summary economizer advisory count"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub struct PurchasedAirRuntimeState' -Description "persistent PurchasedAir manager state"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub struct PurchasedAirUnitRuntimeState' -Description "persistent PurchasedAir unit state"
Assert-Contains -Path $idealLoadsInitState -Pattern 'declared_system_order' -Description "retained PurchasedAir declaration order"
Assert-Contains -Path $idealLoadsInitState -Pattern 'equipment_list_diagnostics' -Description "ordered PurchasedAir manager-sweep diagnostics"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub one_time_latched: bool' -Description "source one-time topology latch"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub topology_completed: bool' -Description "selected-unit topology completion state"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub topology_plan: Option<PurchasedAirInitTopologyPlan>' -Description "retained immutable selected-unit topology plan"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub recirculation_source: Option<PurchasedAirRecirculationSource>' -Description "retained PurchasedAir recirculation branch"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_entry: PurchasedAirCalcEntryRuntimeState' -Description "retained bounded Calc-entry state"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixRuntimeState' -Description "retained bounded Calc minimum-OA state"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub topology_diagnostics: Vec<PurchasedAirInitTopologyDiagnostic>' -Description "retained ordered topology diagnostics"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub topology_failure: Option<PurchasedAirInitTopologyError>' -Description "retained fatal topology outcome"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub topology_completion_count: usize' -Description "historical topology completion count"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub sized_limits: Option<PurchasedAirSizedLimits>' -Description "persistent four-field sizing overlay"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub sizing_outcome: Option<PurchasedAirHardSizeLegacyOutcome>' -Description "persistent source-ordered sizing outcome"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub sizing_attempt_count: usize' -Description "historical sizing attempt count"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub supply_temperature_diagnostic_registry: PurchasedAirSupplyTemperatureDiagnosticRegistry' -Description "global PurchasedAir recurring registry state"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub cooling_supply_temperature_error_index: usize' -Description "per-unit cooling recurring index"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub heating_supply_temperature_error_index: usize' -Description "per-unit heating recurring index"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub economizer_flow_limit_warning_count: usize' -Description "historical economizer advisory count"
Assert-Contains -Path $idealLoadsInitState -Pattern 'environment_initialization_needed' -Description "source MyEnvrnFlag state"
Assert-Contains -Path $idealLoadsInitState -Pattern 'environment_initialization_count' -Description "historical begin-environment count"
Assert-Contains -Path $idealLoadsInitTopologyPlan -Pattern 'pub struct PurchasedAirInitTopologyPlan' -Description "immutable PurchasedAir selected-unit topology plan"
Assert-Contains -Path $idealLoadsInitTopologyPlan -Pattern 'pub struct PurchasedAirInitTopologyEvaluation' -Description "PurchasedAir source-order topology evaluation"
Assert-Contains -Path $idealLoadsInitTopologyPlan -Pattern 'pub enum PurchasedAirRecirculationSource' -Description "PurchasedAir source-shaped recirculation branch"
Assert-Contains -Path $idealLoadsInitTopologyPlan -Pattern 'pub enum PurchasedAirInitTopologyDiagnosticKind' -Description "PurchasedAir topology diagnostic categories"
Assert-Contains -Path $idealLoadsInitTopologyPlan -Pattern 'pub fn from_model\s*\(' -Description "typed-model selected-unit topology-plan builder"
Assert-Contains -Path $idealLoadsInitTopologyPlan -Pattern 'pub\(crate\) fn evaluate\s*\(' -Description "source-order selected-unit topology evaluator"
Assert-Contains -Path $idealLoadsInitTopologyTransition -Pattern 'fn advance_selected_unit_topology\s*\(' -Description "persistent selected-unit topology transition"
Assert-Contains -Path $idealLoadsInitTopologyTransition -Pattern 'LatchedTopologyChanged' -Description "immutable selected-unit topology replay guard"
Assert-Contains -Path $idealLoadsInitTopologyTransition -Pattern 'unit\.one_time_latched = true' -Description "source pre-validation selected-unit latch commit"
Assert-Contains -Path $idealLoadsInitTopologyTransition -Pattern 'unit\.topology_plan = Some\(plan\.clone\(\)\)' -Description "retained immutable selected-unit topology plan"
Assert-Contains -Path $idealLoadsInitTopologyTransition -Pattern 'unit\.topology_failure = Some\(failure\)' -Description "retained selected-unit topology failure"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'pub fn init_purchased_air_runtime\s*\(' -Description "persistent InitPurchasedAir transition"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'pub fn purchased_air_init_lifecycle_summary\s*\(' -Description "persistent init lifecycle summary"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'SelectedSystemMissingFromManagerPlan' -Description "selected-system manager-plan guard"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'DeclaredSystemOrderChanged' -Description "immutable PurchasedAir declaration-order guard"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'ManagerPlanMembershipChanged' -Description "immutable PurchasedAir membership-plan guard"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'equipment_list_scan_order' -Description "declaration-order PurchasedAir manager sweep"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'EquipmentListMembershipMissing' -Description "non-fail-fast PurchasedAir membership diagnostic"
Assert-NotContains -Path $idealLoadsInitTransition -Pattern 'UnsupportedDeclaredSystemCount' -Description "obsolete single-unit manager arena guard"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'size_purchased_air_direct_hard_sized_legacy_route' -Description "InitPurchasedAir calls bounded SizePurchasedAir child"
Assert-Contains -Path $idealLoadsInitTransition -Pattern 'advance_supply_temperature_diagnostics' -Description "InitPurchasedAir calls bounded supply-temperature suffix"
Assert-NotContains -Path $idealLoadsInitTransition -Pattern 'fn validate_hard_sizes\s*\(' -Description "obsolete standalone hard-size validator"
$initTransitionText = Read-RepoText -Path $idealLoadsInitTransition
$sizeChildIndex = $initTransitionText.IndexOf("let sizing_outcome =")
$sizeLatchClearIndex = $initTransitionText.IndexOf("unit.sizing_needed = false")
if ($sizeChildIndex -lt 0 -or $sizeLatchClearIndex -le $sizeChildIndex) {
    throw "SizePurchasedAir child must precede the Init sizing-latch clear"
}
$environmentIndex = $initTransitionText.IndexOf("initialize_environment(unit")
$supplyTemperatureIndex = $initTransitionText.IndexOf("let supply_temperature = advance_supply_temperature_diagnostics(")
if ($environmentIndex -lt 0 -or $supplyTemperatureIndex -le $environmentIndex) {
    throw "Begin-environment initialization must precede the supply-temperature diagnostic suffix"
}
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'pub struct PurchasedAirSupplyTemperatureDiagnosticRegistry' -Description "bounded PurchasedAir recurring registry"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'pub struct PurchasedAirSupplyTemperatureDiagnostic' -Description "PurchasedAir recurring identity evidence"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'pub struct PurchasedAirSupplyTemperatureGateTrace' -Description "PurchasedAir availability read-site trace"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'ShowSevereError' -Description "cooling first-detail API evidence"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'ShowSevereMessage' -Description "heating first-detail API evidence"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'diagnostic\.recurring_minimum_c\.min\(supply_temperature_c\)' -Description "recurring minimum aggregation"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'diagnostic\.recurring_maximum_c\.max\(supply_temperature_c\)' -Description "recurring maximum aggregation"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnostic -Pattern 'pub const PURCHASED_AIR_SUPPLY_TEMPERATURE_UNIT_C' -Description "recurring diagnostic C unit"
$supplyTemperatureText = Read-RepoText -Path $idealLoadsInitSupplyTemperatureDiagnostic
$coolingDiagnosticIndex = $supplyTemperatureText.IndexOf("CoolingMinimumAboveSetpoint")
$heatingDiagnosticIndex = $supplyTemperatureText.IndexOf("HeatingMaximumBelowSetpoint")
if ($coolingDiagnosticIndex -lt 0 -or $heatingDiagnosticIndex -le $coolingDiagnosticIndex) {
    throw "Cooling supply-temperature diagnostic must precede heating"
}
Assert-Contains -Path $idealLoadsInitTests -Pattern 'environment_latch_rearms_and_recomputes_on_the_next_environment' -Description "begin-environment latch regression"
Assert-Contains -Path $idealLoadsInitTests -Pattern 'environment_reinitialization_uses_the_persistent_sizing_overlay' -Description "begin-environment retained-overlay regression"
Assert-Contains -Path $idealLoadsInitTests -Pattern 'unported_sizing_routes_leave_the_size_latch_armed' -Description "unported sizing latch regression"
Assert-Contains -Path $idealLoadsInitTests -Pattern 'absent_current_zone_equipment_completes_the_source_suppression_path' -Description "no-current-equipment normal-return regression"
Assert-Contains -Path $idealLoadsInitTests -Pattern 'deferred_gates_replay_topology_and_invalid_density_fail_closed' -Description "deferred and fail-closed initialization regression"
Assert-Contains -Path $idealLoadsInitManagerPlanTests -Pattern 'model_plan_preserves_non_sorted_system_declaration_order' -Description "manager-plan declaration-order regression"
Assert-Contains -Path $idealLoadsInitManagerPlanTests -Pattern 'model_plan_uses_first_matching_list_and_scans_past_earlier_entries' -Description "manager-plan first-match regression"
Assert-Contains -Path $idealLoadsInitManagerPlanTests -Pattern 'model_plan_retains_missing_equipment_list_membership' -Description "manager-plan missing-membership regression"
Assert-Contains -Path $idealLoadsInitManagerPlanTests -Pattern 'model_and_row_constructors_reject_duplicate_system_ids' -Description "manager-plan duplicate-ID regression"
Assert-Contains -Path $idealLoadsInitManagerPlanTests -Pattern 'model_and_row_constructors_reject_any_active_return_plenum' -Description "manager-plan return-plenum guard regression"
Assert-Contains -Path $idealLoadsInitManagerScanTests -Pattern 'manager_sweep_preserves_declaration_order_and_initializes_only_selected_unit' -Description "manager-sweep declaration-order regression"
Assert-Contains -Path $idealLoadsInitManagerScanTests -Pattern 'deferred_sweep_runs_once_and_is_not_repeated_across_selected_units' -Description "deferred manager-sweep replay regression"
Assert-Contains -Path $idealLoadsInitManagerScanTests -Pattern 'missing_equipment_memberships_emit_ordered_diagnostics_without_fail_fast' -Description "manager-sweep ordered diagnostic regression"
Assert-Contains -Path $idealLoadsInitManagerScanTests -Pattern 'changed_manager_plan_is_rejected_before_selected_unit_mutation' -Description "immutable manager-plan replay regression"
Assert-Contains -Path $idealLoadsInitManagerScanTests -Pattern 'selected_system_missing_from_plan_is_rejected_before_allocation' -Description "selected-system manager-plan regression"
Assert-Contains -Path $idealLoadsInitManagerScanTests -Pattern 'completed_manager_scan_and_sizing_overlay_survive_failed_retry' -Description "manager-sweep retained sizing-error prefix regression"
Assert-Contains -Path $idealLoadsInitTopologyPlanTests -Pattern 'blank_exhaust_with_one_return_assigns_the_return_node' -Description "blank-exhaust single-return topology regression"
Assert-Contains -Path $idealLoadsInitTopologyPlanTests -Pattern 'invalid_exhaust_multiple_returns_and_economizer_keep_source_order' -Description "selected-unit topology diagnostic-order regression"
Assert-Contains -Path $idealLoadsInitTopologyPlanTests -Pattern 'supply_fatal_precedes_exhaust_return_and_economizer_work' -Description "selected-unit supply fatal precedence regression"
Assert-Contains -Path $idealLoadsInitTopologyPlanTests -Pattern 'blank_exhaust_and_zero_returns_fatal_without_a_rejected_node' -Description "blank-exhaust zero-return fatal regression"
Assert-Contains -Path $idealLoadsInitTopologyPlanTests -Pattern 'rust_node_zero_remains_a_valid_single_return_identity' -Description "Rust NodeId zero identity regression"
Assert-Contains -Path $idealLoadsInitTopologyPlanTests -Pattern 'economizer_advisory_requires_resolved_oa_and_a_missing_flow_limit' -Description "selected-unit economizer advisory gate regression"
Assert-Contains -Path $idealLoadsInitTopologyPlanTests -Pattern 'model_plan_preserves_node_list_order_and_resolved_oa_state' -Description "typed-model topology-plan order regression"
Assert-Contains -Path $idealLoadsInitTopologyTransitionTests -Pattern 'selected_latch_precedes_supply_fatal_and_is_not_replayed' -Description "selected-unit pre-fatal latch regression"
Assert-Contains -Path $idealLoadsInitTopologyTransitionTests -Pattern 'invalid_exhaust_diagnostic_precedes_single_return_fallback' -Description "invalid-exhaust transition-order regression"
Assert-Contains -Path $idealLoadsInitTopologyTransitionTests -Pattern 'multiple_returns_warn_and_leave_recirculation_unassigned' -Description "multiple-return source quirk transition regression"
Assert-Contains -Path $idealLoadsInitTopologyTransitionTests -Pattern 'zero_returns_fatal_after_latch_and_retains_diagnostic_prefix' -Description "zero-return retained fatal-prefix regression"
Assert-Contains -Path $idealLoadsInitTopologyTransitionTests -Pattern 'economizer_advisory_is_one_time_and_follows_topology_diagnostics' -Description "economizer one-time diagnostic-order regression"
Assert-Contains -Path $idealLoadsInitTopologyTransitionTests -Pattern 'manager_sweep_survives_selected_topology_fatal' -Description "manager-sweep selected-topology-fatal prefix regression"
Assert-Contains -Path $idealLoadsInitTopologyTransitionTests -Pattern 'environment_validation_never_commits_half_cache' -Description "begin-environment cache atomicity regression"
Assert-Contains -Path $idealLoadsInitWarningTests -Pattern 'warning_predicates_preserve_strict_setpoint_limit_and_availability_gates' -Description "source warning-predicate regression"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnosticTests -Pattern 'recurring_registry_preserves_first_detail_asymmetry_and_cooling_first_order' -Description "recurring detail/order regression"
Assert-Contains -Path $idealLoadsInitSupplyTemperatureDiagnosticTests -Pattern 'recurring_registry_allocates_globally_and_reuses_each_units_indices' -Description "global allocation and per-unit reuse regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupling.rs" -Pattern 'complete_direct_zone_purchased_air_coupling' -Description "post-Init direct-Zone PurchasedAir completion"
Assert-Contains -Path $idealLoadsCouplingValidation -Pattern 'initialized_limit_context' -Description "persistent initialization cache validation"
Assert-Contains -Path $idealLoadsCouplingValidation -Pattern 'initialized_recirculation_node != input\.recirculation_node' -Description "initialized recirculation-node identity validation"
Assert-Contains -Path $idealLoadsCouplingValidation -Pattern 'with_purchased_air_sized_limits' -Description "Init sizing overlay threaded into Calc context"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupling.rs" -Pattern 'calc_uses_the_persistent_sizing_overlay_after_model_values_change' -Description "Calc retained sizing-overlay regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern 'predictor_failure_precedes_and_preserves_purchased_air_initialization' -Description "predictor-before-Init observable regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern 'initialization_failure_precedes_calc_only_input_validation' -Description "Init-before-Calc error precedence regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern 'public_calc_entry_replay_and_identity_errors_do_not_mutate_lifecycle' -Description "Calc-entry public replay and identity transaction regression"
Assert-Contains -Path $idealLoadsBindingMinimumOaTests -Pattern 'exact_nan_calc_entry_snapshot_advances_minimum_oa_prefix_transactionally' -Description "Calc minimum-OA exact-NaN snapshot transaction regression"
Assert-Contains -Path $idealLoadsBindingMinimumOaTests -Pattern 'advance_direct_no_oa_calc_minimum_oa_prefix\s*\(' -Description "Calc minimum-OA replay transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingEntryGateTests -Pattern 'public_cooling_entry_gate_rejects_forgery_subset_and_replay_without_mutation' -Description "Calc cooling-entry forgery, subset, and replay transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingEntryGateTests -Pattern 'public_active_release_rejects_nonfinite_cooling_predicate_transactionally' -Description "Calc cooling-entry active nonfinite transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingOaMaxFlowGateTests -Pattern 'scheduled_binding_orders_cooling_oa_max_flow_gate_before_numerical_calc' -Description "Calc cooling OA maximum-flow scheduled binding order regression"
Assert-Contains -Path $idealLoadsBindingCoolingOaMaxFlowGateTests -Pattern 'public_cooling_oa_max_flow_gate_rejects_forgery_replay_and_overflow_without_mutation' -Description "Calc cooling OA maximum-flow forgery, replay, and overflow transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingOaMaxFlowGateTests -Pattern 'public_cooling_oa_max_flow_gate_rejects_nonfinite_cache_while_unit_off' -Description "Calc cooling OA maximum-flow UnitOff cache-validation regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_oa_max_flow_body_tests\.rs"\]' -Description "Calc cooling OA maximum-flow true-body binding test module path"
Assert-Contains -Path $idealLoadsBindingCoolingOaMaxFlowBodyTests -Pattern 'scheduled_binding_orders_cooling_oa_max_flow_body_after_gate_before_numerical_calc' -Description "Calc cooling OA maximum-flow true-body scheduled binding order regression"
Assert-Contains -Path $idealLoadsBindingCoolingOaMaxFlowBodyTests -Pattern 'public_cooling_oa_max_flow_body_rejects_forgery_and_replay_without_mutation' -Description "Calc cooling OA maximum-flow true-body forgery and replay transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingOaMaxFlowBodyTests -Pattern 'public_cooling_oa_max_flow_body_rejects_retained_and_supplied_negative_zero_gate_forgery' -Description "Calc cooling OA maximum-flow true-body bitwise negative-zero forgery regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_guard_tests\.rs"\]' -Description "Calc cooling economizer guard binding test module path"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerGuardTests -Pattern 'scheduled_binding_orders_cooling_economizer_guard_after_cp314_before_numerical_calc' -Description "Calc cooling economizer scheduled binding order regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerGuardTests -Pattern 'public_cooling_economizer_guard_rejects_forgery_replay_and_overflow_without_mutation' -Description "Calc cooling economizer forgery, replay, and overflow transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerGuardTests -Pattern 'public_cooling_economizer_guard_rejects_economizer_configuration_without_mutation' -Description "Calc cooling economizer non-release enum rejection regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_guard_integrity_tests\.rs"\]' -Description "Calc cooling economizer guard retained-state integrity test module path"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerGuardIntegrityTests -Pattern 'public_guard_rejects_retained_identity_and_route_forgery_without_mutation' -Description "Calc cooling economizer retained identity and route forgery regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'CalculationCoolingEconomizerGuard\(PurchasedAirCalcCoolingEconomizerGuardError\)' -Description "Calc cooling economizer scheduled binding error boundary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs" -Pattern 'pub calculation_cooling_economizer_guard: PurchasedAirCalcCoolingEconomizerGuardSnapshot' -Description "Calc cooling economizer scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_condition_tests\.rs"\]' -Description "Calc cooling economizer condition binding test module path"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_condition_integrity_tests\.rs"\]' -Description "Calc cooling economizer condition retained-state integrity test module path"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionTests -Pattern 'scheduled_binding_orders_cooling_economizer_condition_after_cp315_before_numerical_calc' -Description "Calc cooling economizer condition scheduled binding order regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionTests -Pattern 'public_cooling_economizer_condition_rejects_forgery_replay_and_overflow_without_mutation' -Description "Calc cooling economizer condition forgery, replay, and overflow transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionTests -Pattern 'public_cooling_economizer_condition_rejects_economizer_configuration_without_mutation' -Description "Calc cooling economizer condition non-release enum rejection regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionIntegrityTests -Pattern 'public_condition_rejects_retained_identity_and_route_forgery_without_mutation' -Description "Calc cooling economizer condition retained identity and route forgery regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'CalculationCoolingEconomizerCondition\(PurchasedAirCalcCoolingEconomizerConditionError\)' -Description "Calc cooling economizer condition scheduled binding error boundary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs" -Pattern 'pub calculation_cooling_economizer_condition:\s*[\r\n]+\s*PurchasedAirCalcCoolingEconomizerConditionSnapshot' -Description "Calc cooling economizer condition scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'advance_direct_no_oa_calc_cooling_economizer_condition\s*\(' -Description "Calc cooling economizer condition scheduled release call"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_body_tests\.rs"\]' -Description "Calc cooling economizer true-body binding test module path"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_body_integrity_tests\.rs"\]' -Description "Calc cooling economizer true-body retained-state integrity test module path"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerBodyTests -Pattern 'scheduled_binding_orders_cooling_economizer_body_after_cp316_before_numerical_calc' -Description "Calc cooling economizer true-body scheduled binding order regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerBodyTests -Pattern 'public_cooling_economizer_body_rejects_forgery_replay_and_overflow_without_mutation' -Description "Calc cooling economizer true-body forgery, replay, and overflow transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerBodyIntegrityTests -Pattern 'public_body_rejects_retained_identity_and_route_forgery_without_mutation' -Description "Calc cooling economizer true-body retained identity and route forgery regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'CalculationCoolingEconomizerBody\(PurchasedAirCalcCoolingEconomizerBodyError\)' -Description "Calc cooling economizer true-body scheduled binding error boundary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs" -Pattern 'pub calculation_cooling_economizer_body:\s*PurchasedAirCalcCoolingEconomizerBodySnapshot' -Description "Calc cooling economizer true-body scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'advance_direct_no_oa_calc_cooling_economizer_body\s*\(' -Description "Calc cooling economizer true-body scheduled release call"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'CalculationCoolingSensibleFlow\(PurchasedAirCalcCoolingSensibleFlowError\)' -Description "Calc cooling sensible-flow scheduled binding error boundary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs" -Pattern 'pub calculation_cooling_sensible_flow:\s*PurchasedAirCalcCoolingSensibleFlowSnapshot' -Description "Calc cooling sensible-flow scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'advance_direct_no_oa_calc_cooling_sensible_flow\s*\(' -Description "Calc cooling sensible-flow scheduled release call"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'CalculationCoolingDehumidificationFlow\(PurchasedAirCalcCoolingDehumidificationFlowError\)' -Description "Calc cooling dehumidification-flow scheduled binding error boundary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs" -Pattern 'pub calculation_cooling_dehumidification_flow:\s*[\r\n]+\s*PurchasedAirCalcCoolingDehumidificationFlowSnapshot' -Description "Calc cooling dehumidification-flow scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'advance_direct_no_oa_calc_cooling_dehumidification_flow\s*\(' -Description "Calc cooling dehumidification-flow scheduled release call"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_humidification_flow_tests\.rs"\]' -Description "Calc cooling humidification-flow binding test module path"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'CalculationCoolingHumidificationFlow\(PurchasedAirCalcCoolingHumidificationFlowError\)' -Description "Calc cooling humidification-flow scheduled binding error boundary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs" -Pattern 'pub calculation_cooling_humidification_flow:\s*PurchasedAirCalcCoolingHumidificationFlowSnapshot' -Description "Calc cooling humidification-flow scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'advance_direct_no_oa_calc_cooling_humidification_flow\s*\(' -Description "Calc cooling humidification-flow scheduled release call"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'zone_component_availability:\s*Some\(PurchasedAirAvailabilityStatus::NoAction\)' -Description "release allocated ZoneComp NoAction visit"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'PurchasedAirTemperatureControlType::DualHeatCool' -Description "release prevalidated DualHeatCool cooling-entry input"
$bindingText = Read-RepoText -Path "crates\ep_runtime\src\ideal_loads\binding.rs"
$bindingInitIndex = $bindingText.IndexOf("let initialization = init_purchased_air_runtime(")
$bindingCalcEntryIndex = $bindingText.IndexOf("let calculation_entry = advance_purchased_air_calc_entry(")
$bindingMinimumOaIndex = $bindingText.IndexOf("let calculation_minimum_outdoor_air = advance_direct_no_oa_calc_minimum_oa_prefix(")
$bindingCoolingEntryIndex = $bindingText.IndexOf("let calculation_cooling_entry_gate = advance_direct_no_oa_calc_cooling_entry_gate(")
$bindingCoolingOaMaxFlowIndex = $bindingText.IndexOf("let calculation_cooling_oa_max_flow_gate = advance_direct_no_oa_calc_cooling_oa_max_flow_gate(")
$bindingCoolingOaMaxFlowBodyIndex = $bindingText.IndexOf("let calculation_cooling_oa_max_flow_body = advance_direct_no_oa_calc_cooling_oa_max_flow_body(")
$bindingCoolingEconomizerGuardIndex = $bindingText.IndexOf("let calculation_cooling_economizer_guard =")
$bindingCoolingEconomizerConditionIndex = $bindingText.IndexOf("let calculation_cooling_economizer_condition =")
$bindingCoolingEconomizerBodyIndex = $bindingText.IndexOf("let calculation_cooling_economizer_body =")
$bindingCoolingSensibleFlowIndex = $bindingText.IndexOf("let calculation_cooling_sensible_flow =")
$bindingCoolingDehumidificationFlowIndex = $bindingText.IndexOf("let calculation_cooling_dehumidification_flow =")
$bindingCoolingHumidificationFlowIndex = $bindingText.IndexOf("let calculation_cooling_humidification_flow =")
$bindingCoolingCapacityZeroFlowResetIndex = $bindingText.IndexOf("let calculation_cooling_capacity_zero_flow_reset =")
$bindingCoolingSupplyMassFlowMaximumIndex = $bindingText.IndexOf("let calculation_cooling_supply_mass_flow_maximum =")
$bindingCoolingSupplyMassFlowEmsOverrideGuardIndex = $bindingText.IndexOf("let calculation_cooling_supply_mass_flow_ems_override_guard =")
$bindingCoolingSupplyMassFlowEmsOverrideBodyIndex = $bindingText.IndexOf("let calculation_cooling_supply_mass_flow_ems_override_body =")
$bindingCoolingSupplyMassFlowLimitGuardIndex = $bindingText.IndexOf("let calculation_cooling_supply_mass_flow_limit_guard =")
$bindingCalcIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
$bindingCoolingEconomizerConditionCall = [regex]::Match(
    $bindingText,
    '(?s)let calculation_cooling_economizer_condition =\s*advance_direct_no_oa_calc_cooling_economizer_condition\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_economizer_guard,\s*\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingEconomizerCondition,\s*\)\?;'
)
$bindingCoolingEconomizerBodyCall = [regex]::Match(
    $bindingText,
    '(?s)let calculation_cooling_economizer_body =\s*advance_direct_no_oa_calc_cooling_economizer_body\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_economizer_condition,\s*\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingEconomizerBody,?\s*\)\?;'
)
$bindingCoolingSensibleFlowCall = [regex]::Match(
    $bindingText,
    '(?s)let calculation_cooling_sensible_flow =\s*advance_direct_no_oa_calc_cooling_sensible_flow\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_economizer_body,\s*&\*input\.zone_state,\s*\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingSensibleFlow,?\s*\)\?;'
)
$bindingCoolingDehumidificationFlowCall = [regex]::Match(
    $bindingText,
    '(?s)let calculation_cooling_dehumidification_flow =\s*advance_direct_no_oa_calc_cooling_dehumidification_flow\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_sensible_flow,\s*\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::\s*CalculationCoolingDehumidificationFlow,?\s*\)\?;'
)
$bindingCoolingHumidificationFlowCall = [regex]::Match(
    $bindingText,
    '(?s)let calculation_cooling_humidification_flow =\s*advance_direct_no_oa_calc_cooling_humidification_flow\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_dehumidification_flow,\s*\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::\s*CalculationCoolingHumidificationFlow,?\s*\)\?;'
)
$bindingCoolingCapacityZeroFlowResetCall = [regex]::Match(
    $bindingText,
    '(?s)let calculation_cooling_capacity_zero_flow_reset =\s*advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_humidification_flow,\s*\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::\s*CalculationCoolingCapacityZeroFlowReset,?\s*\)\?;'
)
if (
    $bindingInitIndex -lt 0 -or
    $bindingCalcEntryIndex -le $bindingInitIndex -or
    $bindingMinimumOaIndex -le $bindingCalcEntryIndex -or
    $bindingCoolingEntryIndex -le $bindingMinimumOaIndex -or
    $bindingCoolingOaMaxFlowIndex -le $bindingCoolingEntryIndex -or
    $bindingCoolingOaMaxFlowBodyIndex -le $bindingCoolingOaMaxFlowIndex -or
    $bindingCoolingEconomizerGuardIndex -le $bindingCoolingOaMaxFlowBodyIndex -or
    $bindingCoolingEconomizerConditionIndex -le $bindingCoolingEconomizerGuardIndex -or
    $bindingCoolingEconomizerBodyIndex -le $bindingCoolingEconomizerConditionIndex -or
    $bindingCoolingSensibleFlowIndex -le $bindingCoolingEconomizerBodyIndex -or
    $bindingCoolingDehumidificationFlowIndex -le $bindingCoolingSensibleFlowIndex -or
    $bindingCoolingHumidificationFlowIndex -le $bindingCoolingDehumidificationFlowIndex -or
    $bindingCoolingCapacityZeroFlowResetIndex -le $bindingCoolingHumidificationFlowIndex -or
    $bindingCoolingSupplyMassFlowMaximumIndex -le $bindingCoolingCapacityZeroFlowResetIndex -or
    $bindingCoolingSupplyMassFlowEmsOverrideGuardIndex -le $bindingCoolingSupplyMassFlowMaximumIndex -or
    $bindingCoolingSupplyMassFlowEmsOverrideBodyIndex -le $bindingCoolingSupplyMassFlowEmsOverrideGuardIndex -or
    $bindingCoolingSupplyMassFlowLimitGuardIndex -le $bindingCoolingSupplyMassFlowEmsOverrideBodyIndex -or
    $bindingCalcIndex -le $bindingCoolingSupplyMassFlowLimitGuardIndex
) {
    throw "InitPurchasedAir must precede the Calc-entry prefix, minimum-OA prefix, cooling-entry gate, cooling OA maximum-flow gate, cooling OA maximum-flow true body, cooling economizer guard, cooling economizer condition, cooling economizer true body, cooling sensible flow, cooling dehumidification flow, cooling humidification flow, cooling capacity-zero flow reset, cooling supply-mass-flow maximum, cooling supply-mass-flow EMS override guard, cooling supply-mass-flow EMS override body, cooling supply-mass-flow limit guard, and bounded numerical Calc coupling"
}
if (-not $bindingCoolingEconomizerConditionCall.Success) {
    throw "CP316 binding must call the exact no-node release wrapper with only runtime, system, and CP315 predecessor"
}
if (-not $bindingCoolingEconomizerBodyCall.Success) {
    throw "CP317 binding must call the exact no-node release wrapper with only runtime, system, and CP316 predecessor"
}
if (-not $bindingCoolingSensibleFlowCall.Success) {
    throw "CP318 binding must call the exact bound-Zone release wrapper with runtime, system, CP317 predecessor, and Zone state"
}
if (-not $bindingCoolingDehumidificationFlowCall.Success) {
    throw "CP319 binding must call the exact no-service release wrapper with only runtime, system, and CP318 predecessor"
}
if (-not $bindingCoolingHumidificationFlowCall.Success) {
    throw "CP320 binding must call the exact no-service release wrapper with only runtime, system, and CP319 predecessor"
}
if (-not $bindingCoolingCapacityZeroFlowResetCall.Success) {
    throw "CP321 binding must call the exact no-service release wrapper with only runtime, system, and CP320 predecessor"
}
$bindingCoolingEconomizerConditionCallEnd =
    $bindingCoolingEconomizerConditionCall.Index + $bindingCoolingEconomizerConditionCall.Length
if ($bindingCoolingEconomizerBodyIndex -lt $bindingCoolingEconomizerConditionCallEnd) {
    throw "CP316 exact release call must complete before CP317"
}
$bindingCoolingEconomizerConditionToBodyWindow = $bindingText.Substring(
    $bindingCoolingEconomizerConditionCall.Index,
    $bindingCoolingEconomizerBodyIndex - $bindingCoolingEconomizerConditionCall.Index
)
foreach ($forbiddenBehavior in $coolingEconomizerConditionForbiddenBehaviorPatterns) {
    if ($bindingCoolingEconomizerConditionToBodyWindow -match $forbiddenBehavior.Pattern) {
        throw "$($forbiddenBehavior.Description) unexpectedly present between CP316 entry and CP317"
    }
}
$bindingPostCoolingEconomizerConditionWindow = $bindingText.Substring(
    $bindingCoolingEconomizerConditionCallEnd,
    $bindingCoolingEconomizerBodyIndex - $bindingCoolingEconomizerConditionCallEnd
)
if ($bindingPostCoolingEconomizerConditionWindow -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP316 and before CP317"
}
$bindingCoolingEconomizerBodyCallEnd =
    $bindingCoolingEconomizerBodyCall.Index + $bindingCoolingEconomizerBodyCall.Length
if ($bindingCoolingSensibleFlowIndex -lt $bindingCoolingEconomizerBodyCallEnd) {
    throw "CP317 exact release call must complete before CP318"
}
$bindingCoolingEconomizerBodyToSensibleFlowWindow = $bindingText.Substring(
    $bindingCoolingEconomizerBodyCall.Index,
    $bindingCoolingSensibleFlowIndex - $bindingCoolingEconomizerBodyCall.Index
)
foreach ($forbiddenBehavior in $coolingEconomizerBodyForbiddenBehaviorPatterns) {
    if ($bindingCoolingEconomizerBodyToSensibleFlowWindow -match $forbiddenBehavior.Pattern) {
        throw "$($forbiddenBehavior.Description) unexpectedly present between CP317 and CP318"
    }
}
$bindingPostCoolingEconomizerBodyWindow = $bindingText.Substring(
    $bindingCoolingEconomizerBodyCallEnd,
    $bindingCoolingSensibleFlowIndex - $bindingCoolingEconomizerBodyCallEnd
)
if ($bindingPostCoolingEconomizerBodyWindow -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP317 and before CP318"
}
$bindingCoolingSensibleFlowCallEnd =
    $bindingCoolingSensibleFlowCall.Index + $bindingCoolingSensibleFlowCall.Length
if ($bindingCoolingDehumidificationFlowIndex -lt $bindingCoolingSensibleFlowCallEnd) {
    throw "CP318 exact release call must complete before CP319"
}
$bindingCoolingSensibleFlowToDehumidificationFlowWindow = $bindingText.Substring(
    $bindingCoolingSensibleFlowCall.Index,
    $bindingCoolingDehumidificationFlowIndex - $bindingCoolingSensibleFlowCall.Index
)
foreach ($forbiddenBehavior in $coolingSensibleFlowForbiddenBehaviorPatterns) {
    if ($bindingCoolingSensibleFlowToDehumidificationFlowWindow -match $forbiddenBehavior.Pattern) {
        throw "$($forbiddenBehavior.Description) unexpectedly present between CP318 and CP319"
    }
}
$bindingPostCoolingSensibleFlowWindow = $bindingText.Substring(
    $bindingCoolingSensibleFlowCallEnd,
    $bindingCoolingDehumidificationFlowIndex - $bindingCoolingSensibleFlowCallEnd
)
if ($bindingPostCoolingSensibleFlowWindow -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP318 and before CP319"
}
$bindingCoolingDehumidificationFlowCallEnd =
    $bindingCoolingDehumidificationFlowCall.Index +
    $bindingCoolingDehumidificationFlowCall.Length
if ($bindingCoolingHumidificationFlowIndex -lt $bindingCoolingDehumidificationFlowCallEnd) {
    throw "CP319 exact release call must complete before CP320"
}
$bindingCoolingDehumidificationFlowToHumidificationFlowWindow = $bindingText.Substring(
    $bindingCoolingDehumidificationFlowCall.Index,
    $bindingCoolingHumidificationFlowIndex - $bindingCoolingDehumidificationFlowCall.Index
)
foreach ($forbiddenBehavior in $coolingDehumidificationFlowForbiddenBehaviorPatterns) {
    if ($bindingCoolingDehumidificationFlowToHumidificationFlowWindow -match $forbiddenBehavior.Pattern) {
        throw "$($forbiddenBehavior.Description) unexpectedly present between CP319 and CP320"
    }
}
$bindingPostCoolingDehumidificationFlowWindow = $bindingText.Substring(
    $bindingCoolingDehumidificationFlowCallEnd,
    $bindingCoolingHumidificationFlowIndex - $bindingCoolingDehumidificationFlowCallEnd
)
if ($bindingPostCoolingDehumidificationFlowWindow -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP319 and before CP320"
}
$bindingCoolingHumidificationFlowCallEnd =
    $bindingCoolingHumidificationFlowCall.Index +
    $bindingCoolingHumidificationFlowCall.Length
if ($bindingCoolingCapacityZeroFlowResetIndex -lt $bindingCoolingHumidificationFlowCallEnd) {
    throw "CP320 exact release call must complete before CP321"
}
$bindingCoolingHumidificationFlowToCapacityZeroWindow = $bindingText.Substring(
    $bindingCoolingHumidificationFlowCall.Index,
    $bindingCoolingCapacityZeroFlowResetIndex - $bindingCoolingHumidificationFlowCall.Index
)
foreach ($forbiddenBehavior in $coolingHumidificationFlowForbiddenBehaviorPatterns) {
    if ($bindingCoolingHumidificationFlowToCapacityZeroWindow -match $forbiddenBehavior.Pattern) {
        throw "$($forbiddenBehavior.Description) unexpectedly present between CP320 and CP321"
    }
}
$bindingPostCoolingHumidificationFlowWindow = $bindingText.Substring(
    $bindingCoolingHumidificationFlowCallEnd,
    $bindingCoolingCapacityZeroFlowResetIndex - $bindingCoolingHumidificationFlowCallEnd
)
if ($bindingPostCoolingHumidificationFlowWindow -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP320 and before CP321"
}
$bindingCoolingCapacityZeroFlowResetCallEnd =
    $bindingCoolingCapacityZeroFlowResetCall.Index +
    $bindingCoolingCapacityZeroFlowResetCall.Length
if ($bindingCoolingSupplyMassFlowMaximumIndex -le $bindingCoolingCapacityZeroFlowResetCallEnd) {
    throw "CP321 exact release call must complete before CP322"
}
$bindingPostCoolingCapacityZeroFlowResetWindow = $bindingText.Substring(
    $bindingCoolingCapacityZeroFlowResetCallEnd,
    $bindingCoolingSupplyMassFlowMaximumIndex - $bindingCoolingCapacityZeroFlowResetCallEnd
)
if ($bindingPostCoolingCapacityZeroFlowResetWindow -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP321 and before CP322"
}
Assert-Contains -Path $calcLimits -Pattern 'initialized_heating_air_mass_flow_limit_kg_per_s' -Description "initialized heating flow cache input"
Assert-Contains -Path $calcLimits -Pattern 'initialized_cooling_air_mass_flow_limit_kg_per_s' -Description "initialized cooling flow cache input"
Assert-Contains -Path $calcLimits -Pattern 'purchased_air_sized_limits' -Description "Calc four-field PurchasedAir sizing overlay"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_init_lifecycle' -Description "release lifecycle JSON evidence"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_entry_lifecycle: PurchasedAirCalcEntryLifecycleSummary' -Description "coupled runtime Calc-entry summary"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'calc_entry_snapshot_matches_release' -Description "per-timestep Calc-entry reconciliation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'validate_calc_entry_lifecycle' -Description "final Calc-entry lifecycle reconciliation"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_entry_lifecycle' -Description "release Calc-entry lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern 'validate_direct_purchased_air_calc_entry_lifecycle' -Description "release Calc-entry lifecycle firewall"
Assert-Contains -Path $runPipeline -Pattern 'persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime' -Description "non-direct Calc-entry evidence rejection"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_entry_lifecycle' -Description "direct run Calc-entry lifecycle JSON assertion"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod minimum_oa_validation;' -Description "coupled runtime minimum-OA validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "per-timestep minimum-OA release validator"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "final minimum-OA lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'prefix\.source == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE' -Description "coupled minimum-OA parent provenance validation"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'prefix\.minimum_oa_child_source == PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE' -Description "coupled minimum-OA child provenance validation"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'prefix\.source_order == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER' -Description "coupled minimum-OA exact source-order validation"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'prefix\.ems_override_enabled == Some\(false\)' -Description "coupled minimum-OA EMS-disabled release validation"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'prefix\.outdoor_air_enabled == Some\(false\)' -Description "coupled minimum-OA no-OA release validation"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'state\.outdoor_air_effect_count' -Description "coupled minimum-OA active-OA zero-count validation"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern 'state\.psychrometric_call_count' -Description "coupled minimum-OA psychrometric zero-count validation"
Assert-Contains -Path $idealLoadsCoupledMinimumOaValidation -Pattern '\.checked_add\(state\.unit_off_skip_count\)' -Description "coupled minimum-OA checked source/skip partition"
Assert-NotContains -Path $idealLoadsCoupledRuntime -Pattern 'fn snapshot_matches_release\s*\(' -Description "minimum-OA validator implementation in coupled-runtime root"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'minimum_oa_validation::snapshot_matches_release' -Description "coupled runtime per-timestep minimum-OA validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'minimum_oa_validation::validate_lifecycle' -Description "coupled runtime final minimum-OA validation"
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_minimum_oa;' -Description "pipeline minimum-OA evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirMinimumOa -Pattern 'pub\(super\) fn lifecycle_json\s*\(' -Description "pipeline minimum-OA JSON ownership"
Assert-Contains -Path $runPurchasedAirMinimumOa -Pattern 'pub\(super\) fn validate_direct_lifecycle\s*\(' -Description "pipeline minimum-OA firewall ownership"
Assert-Contains -Path $runPurchasedAirMinimumOa -Pattern 'prefix\.source_order == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER' -Description "pipeline minimum-OA exact source-order validation"
Assert-Contains -Path $runPurchasedAirMinimumOa -Pattern 'prefix\.ems_override_enabled == Some\(false\)' -Description "pipeline minimum-OA EMS-disabled release validation"
Assert-Contains -Path $runPurchasedAirMinimumOa -Pattern 'prefix\.outdoor_air_enabled == Some\(false\)' -Description "pipeline minimum-OA no-OA release validation"
Assert-Contains -Path $runPurchasedAirMinimumOa -Pattern '\.checked_add\(state\.unit_off_skip_count\)' -Description "pipeline minimum-OA checked source/skip partition"
Assert-NotContains -Path $runPipeline -Pattern 'fn latest_matches_release\s*\(' -Description "minimum-OA pipeline validator implementation in pipeline root"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_minimum_oa_prefix_lifecycle' -Description "release minimum-OA lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_minimum_oa::validate_direct_lifecycle' -Description "release minimum-OA pipeline firewall"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_minimum_oa_prefix_lifecycle: PurchasedAirCalcMinimumOaPrefixLifecycleSummary' -Description "coupled runtime minimum-OA lifecycle summary"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_minimum_oa_prefix_lifecycle' -Description "direct run minimum-OA lifecycle JSON assertion"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_entry_validation;' -Description "coupled runtime cooling-entry validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "per-timestep cooling-entry release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "final cooling-entry lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'gate\.source == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE' -Description "coupled cooling-entry provenance validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'gate\.first_excluded_source\s*[\r\n]+\s*== PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE' -Description "coupled cooling-entry first-excluded validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'gate\.source_order == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER' -Description "coupled cooling-entry exact source-order validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'source_skip_partition' -Description "coupled cooling-entry checked source/skip partition"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'cooling_fallthrough_partition' -Description "coupled cooling-entry checked cooling/fallthrough partition"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'sensible_comparison_satisfied_count' -Description "coupled cooling-entry satisfied-comparison reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'temperature_control_type_read_count' -Description "coupled cooling-entry thermostat-read reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'single_heat_block_count", 0' -Description "coupled cooling-entry release SingleHeat zero-count guard"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'operating_mode_assignment_count' -Description "coupled cooling-entry mode-assignment reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'pub\(super\) fn numerical_mode_matches_release\s*\(' -Description "cooling-entry gate to distinct numerical DTO reconciler"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'actual == IdealLoadsSensibleMode::Cooling' -Description "cooling-entry gate reconciled with distinct numerical Cooling DTO"
Assert-Contains -Path $idealLoadsCoupledCoolingEntryValidation -Pattern 'actual == IdealLoadsSensibleMode::Off' -Description "UnitOff cooling-entry skip reconciled with numerical Off DTO"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_entry_validation::snapshot_matches_release' -Description "coupled runtime per-timestep cooling-entry validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_entry_validation::validate_lifecycle' -Description "coupled runtime final cooling-entry validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_entry_gate_lifecycle: PurchasedAirCalcCoolingEntryGateLifecycleSummary' -Description "coupled runtime cooling-entry lifecycle summary"
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_entry_gate;' -Description "pipeline cooling-entry evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'pub\(super\) fn lifecycle_json\s*\(' -Description "pipeline cooling-entry JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'pub\(super\) fn validate_direct_lifecycle\s*\(' -Description "pipeline cooling-entry firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'gate\.source_order == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER' -Description "pipeline cooling-entry exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'gate\.first_excluded_source\s*[\r\n]+\s*== PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling-entry first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'cooling_demand_w\.is_finite\(\)' -Description "pipeline cooling-entry finite release predicate validation"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'source_skip_partition' -Description "pipeline cooling-entry checked source/skip partition"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'cooling_fallthrough_partition' -Description "pipeline cooling-entry checked cooling/fallthrough partition"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'sensible_comparison_satisfied_count' -Description "pipeline cooling-entry satisfied-comparison evidence"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'temperature_control_type_read_count' -Description "pipeline cooling-entry thermostat-read evidence"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'single_heat_block_count", 0' -Description "pipeline cooling-entry release SingleHeat zero-count guard"
Assert-Contains -Path $runPurchasedAirCoolingEntryGate -Pattern 'operating_mode_assignment_count' -Description "pipeline cooling-entry mode-assignment evidence"
Assert-NotContains -Path $runPipeline -Pattern 'fn validate_direct_lifecycle\s*\(' -Description "cooling-entry pipeline validator implementation in pipeline root"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_cooling_entry_gate_lifecycle' -Description "release cooling-entry lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_entry_gate::validate_direct_lifecycle' -Description "release cooling-entry pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_entry_gate_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling-entry evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_entry_gate_validation_rejects_disconnected_evidence' -Description "pipeline cooling-entry disconnected-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_entry_gate_json_exposes_inclusive_cooling_route' -Description "pipeline cooling-entry inclusive-route JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_entry_gate_lifecycle' -Description "direct run cooling-entry lifecycle JSON assertion"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_oa_max_flow_gate: PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState' -Description "persistent cooling OA maximum-flow lifecycle state"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_oa_max_flow_validation;' -Description "coupled runtime cooling OA maximum-flow validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "per-timestep cooling OA maximum-flow release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "final cooling OA maximum-flow lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE' -Description "coupled cooling OA maximum-flow snapshot provenance validation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'first_excluded_source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE' -Description "coupled cooling OA maximum-flow snapshot first-excluded validation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER' -Description "coupled cooling OA maximum-flow exact source-order validation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'p\.cooling_body_entry_count' -Description "coupled cooling OA maximum-flow predecessor body-entry reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'numerical_cooling_count' -Description "coupled cooling OA maximum-flow numerical Cooling reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'transition_partition' -Description "coupled cooling OA maximum-flow checked source/skip partition"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'let first_matches = usize::from\(limit == IdealLoadsLimit::LimitFlowRate\)' -Description "coupled cooling OA maximum-flow FlowRate route reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'let second_matches = usize::from\(limit == IdealLoadsLimit::LimitFlowRateAndCapacity\)' -Description "coupled cooling OA maximum-flow combined-limit route reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'strict_mass_flow_comparison_count' -Description "coupled cooling OA maximum-flow strict-comparison reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'count!\(strict_mass_flow_comparison_satisfied_count, 0\)' -Description "coupled cooling OA maximum-flow release false-result guard"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Pattern 'count!\(maximum_cooling_flow_body_entry_count, 0\)' -Description "coupled cooling OA maximum-flow excluded-body zero-count guard"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_oa_max_flow_validation::snapshot_matches_release' -Description "coupled runtime per-timestep cooling OA maximum-flow validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_oa_max_flow_validation::validate_lifecycle' -Description "coupled runtime final cooling OA maximum-flow validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_oa_max_flow_gate_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary' -Description "coupled runtime cooling OA maximum-flow lifecycle summary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_oa_max_flow_count_arithmetic_rejects_overflow_and_underflow' -Description "coupled cooling OA maximum-flow checked-arithmetic regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_oa_max_flow_gate_reconciles_every_release_limit_shape' -Description "coupled cooling OA maximum-flow four-limit release regression"
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_oa_max_flow;' -Description "pipeline cooling OA maximum-flow evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'pub\(super\) fn lifecycle_json\s*\(' -Description "pipeline cooling OA maximum-flow JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'pub\(super\) fn validate_direct_lifecycle\s*\(' -Description "pipeline cooling OA maximum-flow firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'gate\.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER' -Description "pipeline cooling OA maximum-flow exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'gate\.first_excluded_source\s*[\r\n]+\s*== PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling OA maximum-flow first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'expected_second_comparisons' -Description "pipeline cooling OA maximum-flow OR short-circuit reconciliation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'selected_flow_count' -Description "pipeline cooling OA maximum-flow AND short-circuit reconciliation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'strict_mass_flow_comparison_satisfied_count' -Description "pipeline cooling OA maximum-flow false-result evidence"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'maximum_cooling_flow_body_entry_count' -Description "pipeline cooling OA maximum-flow excluded-body evidence"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern '!maximum_cooling_mass_flow\.is_finite\(\)' -Description "pipeline cooling OA maximum-flow finite maximum validation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'maximum_cooling_mass_flow < 0\.0' -Description "pipeline cooling OA maximum-flow nonnegative maximum validation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlow -Pattern 'active_shape_matches\s*\(' -Description "pipeline cooling OA maximum-flow four-limit latest-shape validation"
Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_oa_max_flow_gate_lifecycle"' -Description "release cooling OA maximum-flow lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_oa_max_flow::validate_direct_lifecycle' -Description "release cooling OA maximum-flow pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_oa_max_flow_gate_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling OA maximum-flow evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_oa_max_flow_validation_rejects_disconnected_evidence' -Description "pipeline cooling OA maximum-flow disconnected-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_oa_max_flow_json_exposes_limit_short_circuit_routes' -Description "pipeline cooling OA maximum-flow four-limit JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_oa_max_flow_gate_lifecycle' -Description "direct run cooling OA maximum-flow lifecycle JSON assertion"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_oa_max_flow_body: PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState' -Description "persistent cooling OA maximum-flow true-body lifecycle state"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_oa_max_flow_body_validation;' -Description "coupled runtime cooling OA maximum-flow true-body validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "per-timestep cooling OA maximum-flow true-body release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "final cooling OA maximum-flow true-body lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE' -Description "coupled cooling OA maximum-flow true-body snapshot provenance validation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'first_excluded_source:\s*PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE' -Description "coupled cooling OA maximum-flow true-body first-excluded validation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'recurring_warning_child_source:\s*PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE' -Description "coupled cooling OA maximum-flow true-body recurring-child validation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER' -Description "coupled cooling OA maximum-flow true-body exact source-order validation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'body_entry_count,\s*[\r\n]+\s*predecessor\.maximum_cooling_flow_body_entry_count' -Description "coupled cooling OA maximum-flow true-body predecessor-entry reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'active_guard_false_economizer_fallthrough_count,\s*[\r\n]+\s*predecessor\.active_fallthrough_count' -Description "coupled cooling OA maximum-flow true-body active false-path partition"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'count!\(standard_air_density_read_count, 0\)' -Description "coupled cooling OA maximum-flow true-body zero mapped density reads"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'count!\(characterized_recurring_warning_occurrence_count, 0\)' -Description "coupled cooling OA maximum-flow true-body zero recurring warnings"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'count!\(outdoor_air_flow_max_cooling_output_index, 0\)' -Description "coupled cooling OA maximum-flow true-body zero recurring index"
Assert-Contains -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Pattern 'count!\(outdoor_air_mass_flow_clamp_assignment_count, 0\)' -Description "coupled cooling OA maximum-flow true-body zero release clamps"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_oa_max_flow_body_validation::snapshot_matches_release' -Description "coupled runtime per-timestep cooling OA maximum-flow true-body validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_oa_max_flow_body_validation::validate_lifecycle' -Description "coupled runtime final cooling OA maximum-flow true-body validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_oa_max_flow_body_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary' -Description "coupled runtime cooling OA maximum-flow true-body lifecycle summary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_oa_max_flow_body_partition_overflow_fails_closed' -Description "coupled cooling OA maximum-flow true-body checked-arithmetic regression"
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_oa_max_flow_body;' -Description "pipeline cooling OA maximum-flow true-body evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'mod serialization;' -Description "pipeline cooling OA maximum-flow true-body serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline cooling OA maximum-flow true-body JSON re-export"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBodySerialization -Pattern 'pub\(crate\) fn lifecycle_json\s*\(' -Description "pipeline cooling OA maximum-flow true-body JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBodySerialization -Pattern '"recurring_warning_child_source"' -Description "pipeline cooling OA maximum-flow recurring-child JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBodySerialization -Pattern '"characterized_recurring_warning_report_maximum_m3_per_s"' -Description "pipeline cooling OA maximum-flow max-only JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBodySerialization -Pattern '"outdoor_air_mass_flow_clamp_assignment_performed"' -Description "pipeline cooling OA maximum-flow clamp-site JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'pub\(super\) fn validate_direct_lifecycle\s*\(' -Description "pipeline cooling OA maximum-flow true-body firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER' -Description "pipeline cooling OA maximum-flow true-body exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling OA maximum-flow true-body first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE' -Description "pipeline cooling OA maximum-flow true-body recurring-child validation"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'let skip_partition = checked_add\s*\(' -Description "pipeline cooling OA maximum-flow true-body checked skip partition"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'let transition_partition = checked_add\s*\(' -Description "pipeline cooling OA maximum-flow true-body checked transition partition"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern '\("direct_body_entry_count", 0, state\.body_entry_count\)' -Description "pipeline cooling OA maximum-flow true-body zero release entry guard"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern '"standard_air_density_read_count",\s*[\r\n]+\s*0' -Description "pipeline cooling OA maximum-flow true-body zero mapped density guard"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern '"characterized_total_warning_error_increment_count",\s*[\r\n]+\s*0' -Description "pipeline cooling OA maximum-flow true-body zero warning increment guard"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern '"outdoor_air_mass_flow_clamp_assignment_count",\s*[\r\n]+\s*0' -Description "pipeline cooling OA maximum-flow true-body zero clamp guard"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'state\s*[\r\n]+\s*\.characterized_recurring_warning_report_maximum_m3_per_s\s*[\r\n]+\s*\.is_some\(\)' -Description "pipeline cooling OA maximum-flow true-body max-only state firewall"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'fn latest_matches_release\s*\(' -Description "pipeline cooling OA maximum-flow true-body latest-state firewall"
Assert-Contains -Path $runPurchasedAirCoolingOaMaxFlowBody -Pattern 'fn skipped_shape\s*\(' -Description "pipeline cooling OA maximum-flow true-body complete-skip firewall"
Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_oa_max_flow_body_lifecycle"' -Description "release cooling OA maximum-flow true-body lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_oa_max_flow_body::validate_direct_lifecycle' -Description "release cooling OA maximum-flow true-body pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_oa_max_flow_body_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling OA maximum-flow true-body evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_oa_max_flow_body_validation_rejects_malformed_evidence' -Description "pipeline cooling OA maximum-flow true-body malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_oa_max_flow_body_json_exposes_zero_effect_skip' -Description "pipeline cooling OA maximum-flow true-body zero-effect JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_oa_max_flow_body_lifecycle' -Description "direct run cooling OA maximum-flow true-body lifecycle JSON assertion"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_economizer_guard: PurchasedAirCalcCoolingEconomizerGuardRuntimeState' -Description "persistent cooling economizer guard lifecycle state"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_economizer_guard_validation;' -Description "coupled runtime cooling economizer guard validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "per-timestep cooling economizer guard release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "final cooling economizer guard lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE' -Description "coupled cooling economizer snapshot provenance validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'first_excluded_source:\s*PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE' -Description "coupled cooling economizer first-excluded validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER' -Description "coupled cooling economizer exact source-order validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'guard\.economizer_guard_evaluated == numerical_cooling' -Description "coupled cooling economizer numerical Cooling reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'guard_evaluation_count,\s*[\r\n]+\s*predecessor\.active_guard_false_economizer_fallthrough_count' -Description "coupled cooling economizer predecessor-fallthrough reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'maximum_cooling_flow_body_sibling_skip_count,\s*[\r\n]+\s*predecessor\.body_entry_count' -Description "coupled cooling economizer CP314 true-body skip reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'count!\(maximum_cooling_flow_body_sibling_skip_count, 0\)' -Description "coupled cooling economizer exact-release sibling zero-count guard"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'count!\(economizer_type_read_count, state\.guard_evaluation_count\)' -Description "coupled cooling economizer one enum read per evaluation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'count!\(no_economizer_comparison_count, state\.guard_evaluation_count\)' -Description "coupled cooling economizer one comparison per evaluation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'count!\(economizer_body_entry_count, 0\)' -Description "coupled cooling economizer zero excluded-body entry guard"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'no_economizer_fallthrough_count,\s*[\r\n]+\s*state\.guard_evaluation_count' -Description "coupled cooling economizer all evaluated guards fall through"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Pattern 'OutdoorAirEconomizerType::NoEconomizer' -Description "coupled cooling economizer exact release enum guard"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_economizer_guard_validation::snapshot_matches_release' -Description "coupled runtime per-timestep cooling economizer validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_economizer_guard_validation::validate_lifecycle' -Description "coupled runtime final cooling economizer validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_economizer_guard_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary' -Description "coupled runtime cooling economizer lifecycle summary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_economizer_guard_partition_overflow_fails_closed' -Description "coupled cooling economizer checked-arithmetic regression"
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_economizer_guard;' -Description "pipeline cooling economizer evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'pub\(super\) fn lifecycle_json\s*\(' -Description "pipeline cooling economizer JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern '"economizer_type"' -Description "pipeline cooling economizer typed enum JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern '"economizer_not_no_economizer"' -Description "pipeline cooling economizer comparison-result JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern '"economizer_body_entered"' -Description "pipeline cooling economizer excluded-body JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'pub\(super\) fn validate_direct_lifecycle\s*\(' -Description "pipeline cooling economizer firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER' -Description "pipeline cooling economizer exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling economizer first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'let skip_partition = checked_add\s*\(' -Description "pipeline cooling economizer checked skip partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'let transition_partition = checked_add\s*\(' -Description "pipeline cooling economizer checked transition partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'let guard_result_partition = checked_add\s*\(' -Description "pipeline cooling economizer checked guard-result partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern '\(\s*"direct_sibling_skip_count",\s*0' -Description "pipeline cooling economizer direct sibling zero-count guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern '\(\s*"economizer_body_entry_count",\s*0' -Description "pipeline cooling economizer zero release body-entry guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'guard\.economizer_type == Some\(OutdoorAirEconomizerType::NoEconomizer\)' -Description "pipeline cooling economizer exact NoEconomizer latest state"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'guard\.economizer_not_no_economizer == Some\(false\)' -Description "pipeline cooling economizer exact false latest result"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerGuard -Pattern 'fn skipped_shape\s*\(' -Description "pipeline cooling economizer complete-skip firewall"
Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_economizer_guard_lifecycle"' -Description "release cooling economizer lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_economizer_guard::validate_direct_lifecycle' -Description "release cooling economizer pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_economizer_guard_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling economizer evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_economizer_guard_validation_rejects_malformed_evidence' -Description "pipeline cooling economizer malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_economizer_guard_json_exposes_no_economizer_fallthrough' -Description "pipeline cooling economizer false-fallthrough JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_economizer_guard_lifecycle' -Description "direct run cooling economizer lifecycle JSON assertion"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_economizer_condition: PurchasedAirCalcCoolingEconomizerConditionRuntimeState' -Description "persistent cooling economizer condition lifecycle state"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_economizer_condition_validation;' -Description "coupled runtime cooling economizer condition validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern '(?s)pub\(super\) fn snapshot_matches_release\s*\(\s*output:\s*&DirectZonePurchasedAirScheduledCouplingOutput,\s*call_ordinal:\s*usize,\s*binding:\s*&DirectZonePurchasedAirModelBinding<''_>,\s*\)\s*->\s*bool' -Description "exact evidence-only per-timestep cooling economizer condition release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern '(?s)pub\(super\) fn validate_lifecycle\s*\(\s*lifecycle:\s*&PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,\s*predecessor_lifecycle:\s*&PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,\s*timestep_count:\s*usize,\s*latest_output:\s*&DirectZonePurchasedAirScheduledCouplingOutput,\s*binding:\s*&DirectZonePurchasedAirModelBinding<''_>,\s*\)\s*->\s*Result<\(\), Error>' -Description "exact evidence-only final cooling economizer condition lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE' -Description "coupled cooling economizer condition snapshot provenance validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'first_excluded_source:\s*PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE' -Description "coupled cooling economizer condition first-excluded validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER' -Description "coupled cooling economizer condition exact source-order validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'let skip_partition = checked_add\s*\(' -Description "coupled cooling economizer condition checked complete-skip partition"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'let transition_partition = checked_add\s*\(' -Description "coupled cooling economizer condition checked transition partition"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'let condition_result_partition = checked_add\s*\(' -Description "coupled cooling economizer condition checked result partition"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'condition_evaluation_count,\s*[\r\n]+\s*predecessor\.economizer_body_entry_count' -Description "coupled cooling economizer condition CP315-entry reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(condition_evaluation_count, 0\)' -Description "coupled cooling economizer condition exact-release total evaluation skip"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'no_economizer_outer_guard_fallthrough_skip_count,\s*[\r\n]+\s*predecessor\.no_economizer_fallthrough_count' -Description "coupled cooling economizer condition CP315-false skip reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(differential_dry_bulb_economizer_type_read_count, 0\)' -Description "coupled cooling economizer condition zero dry-bulb enum reads"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(outdoor_air_temperature_read_count, 0\)' -Description "coupled cooling economizer condition zero temperature reads"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(dry_bulb_temperature_comparison_count, 0\)' -Description "coupled cooling economizer condition zero temperature comparisons"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(differential_enthalpy_economizer_type_read_count, 0\)' -Description "coupled cooling economizer condition zero enthalpy enum reads"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(outdoor_air_enthalpy_read_count, 0\)' -Description "coupled cooling economizer condition zero stored-enthalpy reads"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(enthalpy_comparison_count, 0\)' -Description "coupled cooling economizer condition zero enthalpy comparisons"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Pattern 'count!\(economizer_calculation_body_entry_count, 0\)' -Description "coupled cooling economizer condition zero line-2089 body entries"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_economizer_condition_validation::snapshot_matches_release\(\s*output,\s*timestep_index \+ 1,\s*&binding,\s*\)' -Description "coupled runtime exact evidence-only per-timestep cooling economizer condition validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_economizer_condition_validation::validate_lifecycle\(\s*&calc_cooling_economizer_condition_lifecycle,\s*&calc_cooling_economizer_guard_lifecycle,\s*timestep_outputs\.len\(\),\s*latest_output,\s*&binding,\s*\)' -Description "coupled runtime exact evidence-only final cooling economizer condition validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_economizer_condition_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary' -Description "coupled runtime cooling economizer condition lifecycle summary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_economizer_condition_partition_overflow_fails_closed' -Description "coupled cooling economizer condition checked-arithmetic regression"
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_economizer_condition;' -Description "pipeline cooling economizer condition evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'mod serialization;' -Description "pipeline cooling economizer condition serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline cooling economizer condition serializer re-export"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern '(?s)pub\(in crate::pipeline\) fn lifecycle_json\s*\(\s*lifecycle:\s*&PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,\s*\)\s*->\s*Value' -Description "pipeline evidence-only cooling economizer condition JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern '"differential_dry_bulb_economizer_type_read_count"' -Description "pipeline cooling economizer condition dry-bulb read-count JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern '"differential_enthalpy_economizer_type_read_count"' -Description "pipeline cooling economizer condition enthalpy read-count JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern '"economizer_calculation_body_entry_count"' -Description "pipeline cooling economizer condition line-2089 entry JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern '"economizer_condition_fallthrough_count"' -Description "pipeline cooling economizer condition line-2109 fallthrough JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerConditionSerialization -Pattern 'fn snapshot_json\s*\(' -Description "pipeline cooling economizer condition snapshot JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern '(?s)pub\(super\) fn validate_direct_lifecycle\s*\(\s*lifecycle:\s*Option<&PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary>,\s*predecessor_lifecycle:\s*Option<&PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary>,\s*init_lifecycle:\s*Option<&PurchasedAirInitLifecycleSummary>,\s*coupling_call_count:\s*Option<usize>,\s*\)\s*->\s*Result<\(\), String>' -Description "pipeline exact evidence-only cooling economizer condition firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER' -Description "pipeline cooling economizer condition exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling economizer condition first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'let skip_partition = checked_add\s*\(' -Description "pipeline cooling economizer condition checked complete-skip partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'let transition_partition = checked_add\s*\(' -Description "pipeline cooling economizer condition checked transition partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'let result_partition = checked_add\s*\(' -Description "pipeline cooling economizer condition checked result partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern '\(\s*"direct_condition_evaluation_count",\s*0' -Description "pipeline cooling economizer condition total evaluation zero guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern '\(\s*"differential_dry_bulb_economizer_type_read_count",\s*0' -Description "pipeline cooling economizer condition dry-bulb enum-read zero guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern '\(\s*"outdoor_air_temperature_read_count",\s*0' -Description "pipeline cooling economizer condition temperature-read zero guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern '\(\s*"differential_enthalpy_economizer_type_read_count",\s*0' -Description "pipeline cooling economizer condition enthalpy enum-read zero guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern '\(\s*"outdoor_air_enthalpy_read_count",\s*0' -Description "pipeline cooling economizer condition stored-enthalpy-read zero guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern '\(\s*"economizer_calculation_body_entry_count",\s*0' -Description "pipeline cooling economizer condition line-2089 body-entry zero guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerCondition -Pattern 'fn skipped_shape\s*\(' -Description "pipeline cooling economizer condition complete source-site skip firewall"
Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_economizer_condition_lifecycle"' -Description "release cooling economizer condition lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern '(?s)purchased_air_cooling_economizer_condition::validate_direct_lifecycle\(\s*result\s*\.purchased_air_calc_cooling_economizer_condition_lifecycle\s*\.as_ref\(\),\s*result\s*\.purchased_air_calc_cooling_economizer_guard_lifecycle\s*\.as_ref\(\),\s*init_lifecycle,\s*result\.purchased_air_coupling_call_count,\s*\)\?;' -Description "release exact evidence-only cooling economizer condition pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_economizer_condition_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling economizer condition evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_economizer_condition_validation_rejects_malformed_evidence' -Description "pipeline cooling economizer condition malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_economizer_condition_json_exposes_zero_evidence_skip' -Description "pipeline cooling economizer condition zero-evidence JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_economizer_condition_lifecycle' -Description "direct run cooling economizer condition lifecycle JSON assertion"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_economizer_body: PurchasedAirCalcCoolingEconomizerBodyRuntimeState' -Description "persistent cooling economizer true-body lifecycle state"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_economizer_body_validation;' -Description "coupled runtime cooling economizer true-body validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern '(?s)pub\(super\) fn snapshot_matches_release\s*\(\s*output:\s*&DirectZonePurchasedAirScheduledCouplingOutput,\s*call_ordinal:\s*usize,\s*binding:\s*&DirectZonePurchasedAirModelBinding<''_>,\s*\)\s*->\s*bool' -Description "exact evidence-only per-timestep cooling economizer true-body release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern '(?s)pub\(super\) fn validate_lifecycle\s*\(\s*lifecycle:\s*&PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,\s*predecessor_lifecycle:\s*&PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,\s*timestep_count:\s*usize,\s*latest_output:\s*&DirectZonePurchasedAirScheduledCouplingOutput,\s*binding:\s*&DirectZonePurchasedAirModelBinding<''_>,\s*\)\s*->\s*Result<\(\), Error>' -Description "exact evidence-only final cooling economizer true-body lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern 'source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE' -Description "coupled cooling economizer true-body snapshot provenance validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern 'first_excluded_source:\s*PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE' -Description "coupled cooling economizer true-body first-excluded validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern 'source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER' -Description "coupled cooling economizer true-body exact source-order validation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern 'let skip_partition = checked_add\s*\(' -Description "coupled cooling economizer true-body checked complete-skip partition"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern 'let transition_partition = checked_add\s*\(' -Description "coupled cooling economizer true-body checked transition partition"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern 'body_execution_count,\s*[\r\n]+\s*predecessor\.economizer_calculation_body_entry_count' -Description "coupled cooling economizer true-body CP316-entry reconciliation"
Assert-Contains -Path $idealLoadsCoupledCoolingEconomizerBodyValidation -Pattern 'count!\(body_execution_count, 0\)' -Description "coupled cooling economizer exact-release zero true-body executions"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_economizer_body_validation::snapshot_matches_release\(\s*output,\s*timestep_index \+ 1,\s*&binding,\s*\)' -Description "coupled runtime exact evidence-only per-timestep cooling economizer true-body validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_economizer_body_validation::validate_lifecycle\(\s*&calc_cooling_economizer_body_lifecycle,\s*&calc_cooling_economizer_condition_lifecycle,\s*timestep_outputs\.len\(\),\s*latest_output,\s*&binding,\s*\)' -Description "coupled runtime exact evidence-only final cooling economizer true-body validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_economizer_body_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary' -Description "coupled runtime cooling economizer true-body lifecycle summary"
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_economizer_body;' -Description "pipeline cooling economizer true-body evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern 'mod serialization;' -Description "pipeline cooling economizer true-body serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern 'mod validation;' -Description "pipeline cooling economizer true-body validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline cooling economizer true-body serializer re-export"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySerialization -Pattern '(?s)pub\(in crate::pipeline\) fn lifecycle_json\s*\(\s*lifecycle:\s*&PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,\s*\)\s*->\s*Value' -Description "pipeline evidence-only cooling economizer true-body JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySerialization -Pattern 'mod snapshot;' -Description "pipeline cooling economizer true-body snapshot serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySerialization -Pattern '"body_execution_count"' -Description "pipeline cooling economizer true-body execution-count JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySerialization -Pattern '"psychrometric_cp_air_evaluation_count"' -Description "pipeline cooling economizer true-body psychrometric-count JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySerialization -Pattern '"economizer_active_time_assignment_count"' -Description "pipeline cooling economizer true-body active-time JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySnapshotSerialization -Pattern 'pub\(super\) fn snapshot_json\s*\(' -Description "pipeline cooling economizer true-body snapshot JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern '(?s)pub\(super\) fn validate_direct_lifecycle\s*\(\s*lifecycle:\s*Option<&PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary>,\s*predecessor_lifecycle:\s*Option<&PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary>,\s*init_lifecycle:\s*Option<&PurchasedAirInitLifecycleSummary>,\s*coupling_call_count:\s*Option<usize>,\s*\)\s*->\s*Result<\(\), String>' -Description "pipeline exact evidence-only cooling economizer true-body firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern 'PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER' -Description "pipeline cooling economizer true-body exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern 'PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling economizer true-body first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern '(?s)let skip_partition = \[.*?\.try_fold\(0usize,.*?checked_add\(partial, value, "skip partition"\)' -Description "pipeline cooling economizer true-body checked complete-skip partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern 'let transition_partition = checked_add\s*\(' -Description "pipeline cooling economizer true-body checked transition partition"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBody -Pattern '\(\s*"direct_body_execution_count",\s*0' -Description "pipeline cooling economizer true-body total execution zero guard"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodyValidation -Pattern 'pub\(super\) fn skipped_shape\s*\(' -Description "pipeline cooling economizer true-body complete source-site skip firewall"
Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodyValidation -Pattern 'pub\(super\) fn validate_zero_source_counters\s*\(' -Description "pipeline cooling economizer true-body exhaustive zero-counter firewall"

$coolingEconomizerBodyExpandedSnapshotFields = @(
    "psychrometric_cp_air_result_j_per_kg_k",
    "cp_air_assigned",
    "delta_temperature_assigned",
    "assigned_delta_temperature_c",
    "delta_temperature_for_gate_read",
    "delta_temperature_for_gate_c",
    "delta_temperature_body_entered",
    "cp_air_for_first_division_read",
    "cp_air_for_first_division_j_per_kg_k",
    "zone_cooling_setpoint_load_over_cp_air_calculated",
    "zone_cooling_setpoint_load_over_cp_air_kg_k_per_s",
    "delta_temperature_for_second_division_read",
    "delta_temperature_for_second_division_c",
    "initial_supply_mass_flow_rate_assigned",
    "initial_supply_mass_flow_rate_kg_per_s",
    "maximum_flow_clamp_body_entered",
    "supply_mass_flow_rate_for_clamp_read",
    "supply_mass_flow_rate_for_clamp_kg_per_s",
    "inner_max_evaluated",
    "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read",
    "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s",
    "outer_min_evaluated",
    "clamped_supply_mass_flow_rate_assigned",
    "economizer_activation_body_entered",
    "supply_mass_flow_rate_for_outdoor_air_assignment_read",
    "supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s"
)
foreach ($field in $coolingEconomizerBodyExpandedSnapshotFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodyValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling economizer expanded snapshot validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySnapshotSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling economizer expanded snapshot JSON for $field"
}

$coolingEconomizerBodyExpandedCounterFields = @(
    "cp_air_assignment_count",
    "delta_temperature_assignment_count",
    "delta_temperature_for_gate_read_count",
    "delta_temperature_body_entry_count",
    "cp_air_for_first_division_read_count",
    "zone_cooling_setpoint_load_over_cp_air_calculation_count",
    "delta_temperature_for_second_division_read_count",
    "initial_supply_mass_flow_rate_assignment_count",
    "cooling_limit_flow_rate_read_count",
    "cooling_limit_flow_rate_and_capacity_read_count",
    "maximum_flow_clamp_body_entry_count",
    "supply_mass_flow_rate_for_clamp_read_count",
    "inner_max_evaluation_count",
    "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count",
    "outer_min_evaluation_count",
    "clamped_supply_mass_flow_rate_assignment_count",
    "economizer_activation_body_entry_count",
    "supply_mass_flow_rate_for_outdoor_air_assignment_read_count"
)
foreach ($field in $coolingEconomizerBodyExpandedCounterFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodyValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling economizer expanded zero-counter validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingEconomizerBodySerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling economizer expanded counter JSON for $field"
}
Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_economizer_body_lifecycle"' -Description "release cooling economizer true-body lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern '(?s)purchased_air_cooling_economizer_body::validate_direct_lifecycle\(\s*result\s*\.purchased_air_calc_cooling_economizer_body_lifecycle\s*\.as_ref\(\),\s*result\s*\.purchased_air_calc_cooling_economizer_condition_lifecycle\s*\.as_ref\(\),\s*init_lifecycle,\s*result\.purchased_air_coupling_call_count,\s*\)\?;' -Description "release exact evidence-only cooling economizer true-body pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_economizer_body_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling economizer true-body evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_economizer_body_validation_rejects_malformed_evidence' -Description "pipeline cooling economizer true-body malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_economizer_body_json_exposes_zero_evidence_skip' -Description "pipeline cooling economizer true-body zero-evidence JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_economizer_body_lifecycle' -Description "direct run cooling economizer true-body lifecycle JSON assertion"

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_sensible_flow_validation;' -Description "coupled runtime cooling sensible-flow validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "exact per-timestep cooling sensible-flow release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "exact final cooling sensible-flow lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Pattern 'cooling_sensible_flow_snapshot_is_exact_direct_release\(flow\)' -Description "coupled cooling sensible-flow exact direct snapshot validator consumption"
Assert-Contains -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Pattern 'flow\.cooling_body_entered == predecessor\.predecessor_cooling_body_entered' -Description "coupled cooling sensible-flow common reconvergence criterion"
Assert-Contains -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Pattern 'count!\(\s*cooling_body_entry_count,\s*numerical_cooling_count\s*\)' -Description "coupled cooling sensible-flow mode-count reconciliation without value coupling"
Assert-Contains -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Pattern 'count!\(cooling_on_fallthrough_count, 0\)' -Description "coupled cooling sensible-flow exact direct CoolOn-true invariant"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_sensible_flow_validation::snapshot_matches_release\(\s*output,\s*timestep_index \+ 1,\s*&binding,\s*\)' -Description "coupled runtime per-timestep CP318 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_sensible_flow_validation::validate_lifecycle\(\s*&calc_cooling_sensible_flow_lifecycle,\s*&calc_cooling_economizer_body_lifecycle,\s*timestep_outputs\.len\(\),\s*numerical_cooling_count,\s*latest_output,\s*&binding,\s*\)' -Description "coupled runtime final CP318 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_sensible_flow_lifecycle:\s*PurchasedAirCalcCoolingSensibleFlowLifecycleSummary' -Description "coupled runtime cooling sensible-flow lifecycle summary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_sensible_flow_partition_overflow_fails_closed' -Description "coupled cooling sensible-flow checked-arithmetic regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_sensible_flow_lifecycle_records_unit_off_without_source_execution' -Description "coupled cooling sensible-flow UnitOff skip regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs" -Pattern 'calculation_cooling_sensible_flow' -Description "coupled output CP318 fixture ownership"

Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_sensible_flow;' -Description "pipeline cooling sensible-flow evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern 'mod serialization;' -Description "pipeline cooling sensible-flow serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern 'mod validation;' -Description "pipeline cooling sensible-flow validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline cooling sensible-flow serializer re-export"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowSerialization -Pattern '(?s)pub\(in crate::pipeline\) fn lifecycle_json\s*\(\s*lifecycle:\s*&PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,\s*\)\s*->\s*Value' -Description "pipeline cooling sensible-flow JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowSerialization -Pattern 'mod snapshot;' -Description "pipeline cooling sensible-flow snapshot serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowSnapshotSerialization -Pattern 'pub\(super\) fn snapshot_json\s*\(' -Description "pipeline cooling sensible-flow snapshot JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern '(?s)pub\(super\) fn validate_direct_lifecycle\s*\(\s*lifecycle:\s*Option<&PurchasedAirCalcCoolingSensibleFlowLifecycleSummary>,\s*predecessor_lifecycle:\s*Option<&PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary>,\s*init_lifecycle:\s*Option<&PurchasedAirInitLifecycleSummary>,\s*coupling_call_count:\s*Option<usize>,\s*\)\s*->\s*Result<\(\), String>' -Description "pipeline exact cooling sensible-flow firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern 'PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER' -Description "pipeline cooling sensible-flow exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern 'PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling sensible-flow first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern 'let transition_partition = checked_add\s*\(' -Description "pipeline cooling sensible-flow checked transition partition"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowValidation -Pattern 'pub\(super\) fn validate_source_counters\s*\(' -Description "pipeline cooling sensible-flow exhaustive source-counter firewall"
Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowValidation -Pattern 'pub\(super\) fn snapshot_shape\s*\(' -Description "pipeline cooling sensible-flow exact snapshot firewall"

$coolingSensibleFlowSnapshotFields = @(
    "cooling_body_entered",
    "supply_mass_flow_rate_for_cool_reset_assigned",
    "reset_supply_mass_flow_rate_for_cool_kg_per_s",
    "cooling_on_read",
    "cooling_on",
    "cooling_on_body_entered",
    "zone_humidity_ratio_read",
    "zone_humidity_ratio",
    "psychrometric_cp_air_evaluated",
    "psychrometric_cp_air_result_j_per_kg_k",
    "cp_air_assigned",
    "cp_air_j_per_kg_k",
    "minimum_cooling_supply_air_temperature_read",
    "minimum_cooling_supply_air_temperature_c",
    "zone_temperature_read",
    "zone_temperature_c",
    "delta_temperature_calculated",
    "delta_temperature_c",
    "delta_temperature_assigned",
    "assigned_delta_temperature_c",
    "delta_temperature_for_gate_read",
    "delta_temperature_for_gate_c",
    "delta_temperature_comparison_evaluated",
    "delta_temperature_below_negative_small_temp_diff",
    "delta_temperature_body_entered",
    "zone_cooling_setpoint_load_read",
    "zone_cooling_setpoint_load_w",
    "cp_air_for_first_division_read",
    "cp_air_for_first_division_j_per_kg_k",
    "zone_cooling_setpoint_load_over_cp_air_calculated",
    "zone_cooling_setpoint_load_over_cp_air_kg_k_per_s",
    "delta_temperature_for_second_division_read",
    "delta_temperature_for_second_division_c",
    "supply_mass_flow_rate_for_cool_calculated",
    "calculated_supply_mass_flow_rate_for_cool_kg_per_s",
    "supply_mass_flow_rate_for_cool_assigned",
    "assigned_supply_mass_flow_rate_for_cool_kg_per_s",
    "resulting_supply_mass_flow_rate_for_cool_kg_per_s"
)
foreach ($field in $coolingSensibleFlowSnapshotFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling sensible-flow snapshot validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowSnapshotSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling sensible-flow snapshot JSON for $field"
}

$coolingSensibleFlowPredecessorSnapshotFields = @(
    "predecessor_cooling_body_entered",
    "predecessor_maximum_cooling_flow_body_sibling_skipped",
    "predecessor_no_economizer_outer_guard_fallthrough_skipped",
    "predecessor_economizer_condition_fallthrough_skipped",
    "predecessor_economizer_calculation_body_executed"
)
foreach ($field in $coolingSensibleFlowPredecessorSnapshotFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingSensibleFlow -Pattern "\b$fieldPattern\b" -Description "pipeline cooling sensible-flow predecessor validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowSnapshotSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling sensible-flow predecessor JSON for $field"
}

$coolingSensibleFlowCounterFields = @(
    "cooling_body_entry_count",
    "supply_mass_flow_rate_for_cool_reset_assignment_count",
    "cooling_on_read_count",
    "cooling_on_body_entry_count",
    "cooling_on_fallthrough_count",
    "zone_humidity_ratio_read_count",
    "psychrometric_cp_air_evaluation_count",
    "cp_air_assignment_count",
    "minimum_cooling_supply_air_temperature_read_count",
    "zone_temperature_read_count",
    "delta_temperature_calculation_count",
    "delta_temperature_assignment_count",
    "delta_temperature_for_gate_read_count",
    "delta_temperature_comparison_count",
    "delta_temperature_comparison_satisfied_count",
    "delta_temperature_body_entry_count",
    "delta_temperature_fallthrough_count",
    "zone_cooling_setpoint_load_read_count",
    "cp_air_for_first_division_read_count",
    "zone_cooling_setpoint_load_over_cp_air_calculation_count",
    "delta_temperature_for_second_division_read_count",
    "supply_mass_flow_rate_for_cool_calculation_count",
    "supply_mass_flow_rate_for_cool_assignment_count"
)
foreach ($field in $coolingSensibleFlowCounterFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling sensible-flow counter validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingSensibleFlowSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling sensible-flow counter JSON for $field"
}

Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_sensible_flow_lifecycle"' -Description "release cooling sensible-flow lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern '(?s)purchased_air_cooling_sensible_flow::validate_direct_lifecycle\(\s*result\s*\.purchased_air_calc_cooling_sensible_flow_lifecycle\s*\.as_ref\(\),\s*result\s*\.purchased_air_calc_cooling_economizer_body_lifecycle\s*\.as_ref\(\),\s*init_lifecycle,\s*result\.purchased_air_coupling_call_count,\s*\)\?;' -Description "release exact cooling sensible-flow pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_sensible_flow_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling sensible-flow evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_sensible_flow_validation_rejects_malformed_evidence' -Description "pipeline cooling sensible-flow malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_sensible_flow_json_exposes_all_source_sites' -Description "pipeline cooling sensible-flow all-site JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_sensible_flow_lifecycle' -Description "direct run cooling sensible-flow lifecycle JSON assertion"
Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads.rs" -Pattern 'purchased_air_calc_cooling_sensible_flow_lifecycle' -Description "diagnostic run cooling sensible-flow null-evidence assertion"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'support_boundary_addenda = \[\s*[\r\n]+\s*"CP318 supersedes CP317' -Description "CP318 algorithm support-boundary addendum"
Assert-Contains -Path "specs\capabilities.toml" -Pattern 'claim_boundary_addenda = \[\s*[\r\n]+\s*"CP318 additionally requires' -Description "CP318 capability claim-boundary addendum"
Assert-Contains -Path "tools\docs\generate_docs.py" -Pattern 'item\.get\("support_boundary_addenda", \[\]\)' -Description "generated algorithm support-boundary addenda"
Assert-Contains -Path "tools\docs\generate_docs.py" -Pattern 'item\.get\("claim_boundary_addenda", \[\]\)' -Description "generated capability claim-boundary addenda"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP318 supersedes CP317' -Description "generated CP318 algorithm boundary"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP318 additionally requires' -Description "generated CP318 capability boundary"
foreach ($coolingSensibleFlowIntegrationFile in @(
    $idealLoadsCoupledCoolingSensibleFlowValidation,
    $runPurchasedAirCoolingSensibleFlow,
    $runPurchasedAirCoolingSensibleFlowSerialization,
    $runPurchasedAirCoolingSensibleFlowSnapshotSerialization,
    $runPurchasedAirCoolingSensibleFlowValidation
)) {
    foreach ($forbiddenBehavior in $coolingSensibleFlowForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingSensibleFlowIntegrationFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
Assert-NotContains -Path $idealLoadsCoupledCoolingSensibleFlowValidation -Pattern '(?i)(?:resulting|assigned|calculated)_supply_mass_flow_rate_for_cool.*\bcoupling\b|\bcoupling\b.*(?:resulting|assigned|calculated)_supply_mass_flow_rate_for_cool' -Description "CP318 candidate bitwise reconciliation with unchanged numerical DTO"

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_dehumidification_flow_validation;' -Description "coupled runtime cooling dehumidification-flow validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "exact per-timestep cooling dehumidification-flow release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "exact final cooling dehumidification-flow lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Pattern 'cooling_dehumidification_flow_snapshot_is_exact_direct_release\(flow\)' -Description "coupled cooling dehumidification-flow exact direct snapshot validator consumption"
Assert-Contains -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Pattern 'flow\.predecessor_cooling_body_entered == predecessor\.cooling_body_entered' -Description "coupled cooling dehumidification-flow CP318 predecessor link"
Assert-Contains -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Pattern 'count!\(\s*cooling_body_entry_count,\s*numerical_cooling_count\s*\)' -Description "coupled cooling dehumidification-flow mode-count reconciliation without value coupling"
Assert-Contains -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Pattern 'count!\(dehumidification_control_type_humidistat_count, 0\)' -Description "coupled cooling dehumidification-flow exact direct selector invariant"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_dehumidification_flow_validation::snapshot_matches_release\(\s*output,\s*timestep_index \+ 1,\s*&binding,\s*\)' -Description "coupled runtime per-timestep CP319 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_dehumidification_flow_validation::validate_lifecycle\(\s*&calc_cooling_dehumidification_flow_lifecycle,\s*&calc_cooling_sensible_flow_lifecycle,\s*timestep_outputs\.len\(\),\s*numerical_cooling_count,\s*latest_output,\s*&binding,\s*\)' -Description "coupled runtime final CP319 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_dehumidification_flow_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary' -Description "coupled runtime cooling dehumidification-flow lifecycle summary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_dehumidification_flow_partition_overflow_fails_closed' -Description "coupled cooling dehumidification-flow checked-arithmetic regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern '(?s)cooling_sensible_flow_lifecycle_records_unit_off_without_source_execution.*calc_cooling_dehumidification_flow_lifecycle' -Description "coupled cooling dehumidification-flow UnitOff skip regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs" -Pattern 'calculation_cooling_dehumidification_flow' -Description "coupled output CP319 fixture ownership"

Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_dehumidification_flow;' -Description "pipeline cooling dehumidification-flow evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern 'mod serialization;' -Description "pipeline cooling dehumidification-flow serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern 'mod validation;' -Description "pipeline cooling dehumidification-flow validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline cooling dehumidification-flow serializer re-export"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSerialization -Pattern '(?s)pub\(in crate::pipeline\) fn lifecycle_json\s*\(\s*lifecycle:\s*&PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,\s*\)\s*->\s*Value' -Description "pipeline cooling dehumidification-flow JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSerialization -Pattern 'mod snapshot;' -Description "pipeline cooling dehumidification-flow snapshot serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotSerialization -Pattern 'pub\(super\) fn snapshot_json\s*\(' -Description "pipeline cooling dehumidification-flow snapshot JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowValidation -Pattern 'mod snapshot;' -Description "pipeline cooling dehumidification-flow snapshot validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotValidation -Pattern 'pub\(in crate::pipeline\) fn snapshot_shape\s*\(' -Description "pipeline cooling dehumidification-flow exact snapshot firewall"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "pipeline cooling dehumidification-flow signed-zero firewall"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern '(?s)pub\(super\) fn validate_direct_lifecycle\s*\(\s*lifecycle:\s*Option<&PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary>,\s*predecessor_lifecycle:\s*Option<&PurchasedAirCalcCoolingSensibleFlowLifecycleSummary>,\s*init_lifecycle:\s*Option<&PurchasedAirInitLifecycleSummary>,\s*coupling_call_count:\s*Option<usize>,\s*\)\s*->\s*Result<\(\), String>' -Description "pipeline exact cooling dehumidification-flow firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern 'PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER' -Description "pipeline cooling dehumidification-flow exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern 'PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling dehumidification-flow first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern 'let transition_partition = checked_add\s*\(' -Description "pipeline cooling dehumidification-flow checked transition partition"
Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowValidation -Pattern 'pub\(super\) fn validate_source_counters\s*\(' -Description "pipeline cooling dehumidification-flow exhaustive source-counter firewall"

$coolingDehumidificationFlowLinkFields = @(
    "source",
    "first_excluded_source",
    "source_order",
    "system",
    "parent_call_ordinal",
    "controlled_zone",
    "unit_body_entered",
    "predecessor_cooling_body_entered",
    "predecessor_cooling_on_body_entered",
    "predecessor_delta_temperature_body_entered",
    "predecessor_supply_mass_flow_rate_for_cool_assigned",
    "unit_off_skipped",
    "non_cooling_skipped",
    "cooling_body_entered"
)
foreach ($field in $coolingDehumidificationFlowLinkFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern "flow\.$fieldPattern\b" -Description "pipeline cooling dehumidification-flow predecessor/provenance validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling dehumidification-flow snapshot JSON for $field"
}

$coolingDehumidificationFlowSourceSnapshotFields = @(
    "supply_mass_flow_rate_for_dehumidification_reset_assigned",
    "reset_supply_mass_flow_rate_for_dehumidification_kg_per_s",
    "cooling_on_read",
    "cooling_on",
    "cooling_on_body_entered",
    "dehumidification_control_type_read",
    "dehumidification_control_type",
    "dehumidification_control_type_humidistat",
    "dehumidification_control_body_entered",
    "zone_dehumidifying_setpoint_moisture_demand_read",
    "zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
    "zone_dehumidifying_setpoint_moisture_demand_assigned",
    "assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
    "minimum_cooling_supply_air_humidity_ratio_read",
    "minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air",
    "zone_humidity_ratio_read",
    "zone_humidity_ratio_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_calculated",
    "delta_humidity_ratio_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_assigned",
    "assigned_delta_humidity_ratio_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_for_gate_read",
    "delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_comparison_evaluated",
    "delta_humidity_ratio_below_negative_small_delta",
    "zone_dehumidifying_setpoint_moisture_demand_for_gate_read",
    "zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s",
    "zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated",
    "zone_dehumidifying_setpoint_moisture_demand_below_zero",
    "dehumidification_flow_body_entered",
    "zone_dehumidifying_setpoint_moisture_demand_for_division_read",
    "zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s",
    "delta_humidity_ratio_for_division_read",
    "delta_humidity_ratio_for_division_kg_water_per_kg_dry_air",
    "supply_mass_flow_rate_for_dehumidification_calculated",
    "calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s",
    "supply_mass_flow_rate_for_dehumidification_assigned",
    "assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s",
    "resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s"
)
if (
    $coolingDehumidificationFlowLinkFields.Count +
    $coolingDehumidificationFlowSourceSnapshotFields.Count -ne 53
) {
    throw "CP319 pipeline snapshot audit must enumerate all 53 public fields"
}
foreach ($field in $coolingDehumidificationFlowSourceSnapshotFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling dehumidification-flow snapshot validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSnapshotSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling dehumidification-flow snapshot JSON for $field"
}

$coolingDehumidificationFlowCommonLifecycleFields = @(
    "system",
    "transition_count",
    "cooling_body_entry_count",
    "unit_off_skip_count",
    "non_cooling_skip_count",
    "latest"
)
foreach ($field in $coolingDehumidificationFlowCommonLifecycleFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlow -Pattern "\b$fieldPattern\b" -Description "pipeline cooling dehumidification-flow lifecycle validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling dehumidification-flow lifecycle JSON for $field"
}

$coolingDehumidificationFlowCounterFields = @(
    "supply_mass_flow_rate_for_dehumidification_reset_assignment_count",
    "cooling_on_read_count",
    "cooling_on_body_entry_count",
    "cooling_on_fallthrough_count",
    "dehumidification_control_type_read_count",
    "dehumidification_control_type_humidistat_count",
    "dehumidification_control_type_fallthrough_count",
    "dehumidification_control_body_entry_count",
    "zone_dehumidifying_setpoint_moisture_demand_read_count",
    "zone_dehumidifying_setpoint_moisture_demand_assignment_count",
    "minimum_cooling_supply_air_humidity_ratio_read_count",
    "zone_humidity_ratio_read_count",
    "delta_humidity_ratio_calculation_count",
    "delta_humidity_ratio_assignment_count",
    "delta_humidity_ratio_for_gate_read_count",
    "delta_humidity_ratio_comparison_count",
    "delta_humidity_ratio_comparison_satisfied_count",
    "delta_humidity_ratio_fallthrough_count",
    "zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count",
    "zone_dehumidifying_setpoint_moisture_demand_comparison_count",
    "zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count",
    "zone_dehumidifying_setpoint_moisture_demand_fallthrough_count",
    "dehumidification_flow_body_entry_count",
    "zone_dehumidifying_setpoint_moisture_demand_for_division_read_count",
    "delta_humidity_ratio_for_division_read_count",
    "supply_mass_flow_rate_for_dehumidification_calculation_count",
    "supply_mass_flow_rate_for_dehumidification_assignment_count"
)
if (
    $coolingDehumidificationFlowCommonLifecycleFields.Count +
    $coolingDehumidificationFlowCounterFields.Count -ne 33
) {
    throw "CP319 pipeline lifecycle audit must enumerate all 33 public state fields"
}
foreach ($field in $coolingDehumidificationFlowCounterFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling dehumidification-flow counter validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingDehumidificationFlowSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling dehumidification-flow counter JSON for $field"
}

Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_dehumidification_flow_lifecycle"' -Description "release cooling dehumidification-flow lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern '(?s)purchased_air_cooling_dehumidification_flow::validate_direct_lifecycle\(\s*result\s*\.purchased_air_calc_cooling_dehumidification_flow_lifecycle\s*\.as_ref\(\),\s*result\s*\.purchased_air_calc_cooling_sensible_flow_lifecycle\s*\.as_ref\(\),\s*init_lifecycle,\s*result\.purchased_air_coupling_call_count,\s*\)\?;' -Description "release exact cooling dehumidification-flow pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_dehumidification_flow_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling dehumidification-flow evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_dehumidification_flow_validation_rejects_malformed_evidence' -Description "pipeline cooling dehumidification-flow malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_dehumidification_flow_json_exposes_all_source_sites' -Description "pipeline cooling dehumidification-flow all-site JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_dehumidification_flow_lifecycle' -Description "direct run cooling dehumidification-flow lifecycle JSON assertion"
Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads.rs" -Pattern 'purchased_air_calc_cooling_dehumidification_flow_lifecycle' -Description "diagnostic run cooling dehumidification-flow null-evidence assertion"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern '"CP319 supersedes CP318' -Description "CP319 algorithm support-boundary addendum"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_dehumidification_flow/release\.rs::advance_direct_no_oa_calc_cooling_dehumidification_flow' -Description "CP319 algorithm Rust release target"
Assert-Contains -Path "specs\capabilities.toml" -Pattern '"CP319 additionally requires' -Description "CP319 capability claim-boundary addendum"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP319 supersedes CP318' -Description "generated CP319 algorithm boundary"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP319 additionally requires' -Description "generated CP319 capability boundary"
foreach ($coolingDehumidificationFlowIntegrationFile in @(
    $idealLoadsCoupledCoolingDehumidificationFlowValidation,
    $runPurchasedAirCoolingDehumidificationFlow,
    $runPurchasedAirCoolingDehumidificationFlowSerialization,
    $runPurchasedAirCoolingDehumidificationFlowSnapshotSerialization,
    $runPurchasedAirCoolingDehumidificationFlowValidation,
    $runPurchasedAirCoolingDehumidificationFlowSnapshotValidation
)) {
    foreach ($forbiddenBehavior in $coolingDehumidificationFlowForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingDehumidificationFlowIntegrationFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
Assert-NotContains -Path $idealLoadsCoupledCoolingDehumidificationFlowValidation -Pattern '(?i)(?:resulting|assigned|calculated)_supply_mass_flow_rate_for_dehumidification.*\bcoupling\b|\bcoupling\b.*(?:resulting|assigned|calculated)_supply_mass_flow_rate_for_dehumidification' -Description "CP319 candidate bitwise reconciliation with unchanged numerical DTO"

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_humidification_flow_validation;' -Description "coupled runtime cooling humidification-flow validator submodule declaration"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'CalcCoolingHumidificationFlowLifecycle\(PurchasedAirCalcCoolingHumidificationFlowError\)' -Description "coupled runtime cooling humidification-flow summary error boundary"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'CalcCoolingHumidificationFlowLifecycleInvariant' -Description "coupled runtime cooling humidification-flow lifecycle invariant error"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'UnexpectedCalculationCoolingHumidificationFlow' -Description "coupled runtime cooling humidification-flow per-step error"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "exact per-timestep cooling humidification-flow release validator"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "exact final cooling humidification-flow lifecycle validator"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'pub\(super\) fn checked_add\s*\(' -Description "cooling humidification-flow checked counter arithmetic"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'cooling_humidification_flow_snapshot_is_exact_direct_release\(flow\)' -Description "coupled cooling humidification-flow exact direct snapshot validator consumption"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'flow\.predecessor_cooling_body_entered == predecessor\.cooling_body_entered' -Description "coupled cooling humidification-flow immediate CP319 predecessor link"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'count!\(cooling_body_entry_count, numerical_cooling_count\)' -Description "coupled cooling humidification-flow mode-count reconciliation without value coupling"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'count!\(humidification_control_type_humidistat_count, 0\)' -Description "coupled cooling humidification-flow exact direct outer selector invariant"
Assert-Contains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern 'count!\(dehumidification_control_type_second_read_count, 0\)' -Description "coupled cooling humidification-flow exact direct nested selector no-read invariant"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_humidification_flow_validation::snapshot_matches_release\(\s*output,\s*timestep_index \+ 1,\s*&binding,\s*\)' -Description "coupled runtime per-timestep CP320 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_humidification_flow_validation::validate_lifecycle\(\s*&calc_cooling_humidification_flow_lifecycle,\s*&calc_cooling_dehumidification_flow_lifecycle,\s*timestep_outputs\.len\(\),\s*numerical_cooling_count,\s*latest_output,\s*&binding,\s*\)' -Description "coupled runtime final CP320 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_humidification_flow_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary' -Description "coupled runtime cooling humidification-flow lifecycle summary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern 'cooling_humidification_flow_partition_overflow_fails_closed' -Description "coupled cooling humidification-flow checked-arithmetic regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs" -Pattern '(?s)cooling_sensible_flow_lifecycle_records_unit_off_without_source_execution.*calc_cooling_humidification_flow_lifecycle' -Description "coupled cooling humidification-flow UnitOff skip regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs" -Pattern 'calculation_cooling_humidification_flow' -Description "coupled output CP320 fixture ownership"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs" -Pattern '#\[path = "coupled_output_tests/cooling_humidification_flow_fixture\.rs"\]' -Description "coupled output CP320 fixture module path"
Assert-Contains -Path $idealLoadsCoupledOutputCoolingHumidificationFixture -Pattern 'pub\(super\) fn calculation_cooling_humidification_flow_snapshot\s*\(' -Description "coupled output CP320 snapshot fixture"

Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_humidification_flow;' -Description "pipeline cooling humidification-flow evidence submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern 'mod serialization;' -Description "pipeline cooling humidification-flow serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern 'mod validation;' -Description "pipeline cooling humidification-flow validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline cooling humidification-flow serializer re-export"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSerialization -Pattern '(?s)pub\(in crate::pipeline\) fn lifecycle_json\s*\(\s*lifecycle:\s*&PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,\s*\)\s*->\s*Value' -Description "pipeline cooling humidification-flow JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSerialization -Pattern 'mod snapshot;' -Description "pipeline cooling humidification-flow snapshot serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSnapshotSerialization -Pattern 'pub\(super\) fn snapshot_json\s*\(' -Description "pipeline cooling humidification-flow snapshot JSON ownership"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowValidation -Pattern 'pub\(super\) fn validate_source_counters\s*\(' -Description "pipeline cooling humidification-flow exhaustive source-counter firewall"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowValidation -Pattern 'mod snapshot;' -Description "pipeline cooling humidification-flow snapshot validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSnapshotValidation -Pattern 'pub\(in crate::pipeline\) fn snapshot_shape\s*\(' -Description "pipeline cooling humidification-flow exact snapshot firewall"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSnapshotValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "pipeline cooling humidification-flow signed-zero firewall"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern '(?s)pub\(super\) fn validate_direct_lifecycle\s*\(\s*lifecycle:\s*Option<&PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary>,\s*predecessor_lifecycle:\s*Option<&PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary>,\s*init_lifecycle:\s*Option<&PurchasedAirInitLifecycleSummary>,\s*coupling_call_count:\s*Option<usize>,\s*\)\s*->\s*Result<\(\), String>' -Description "pipeline exact cooling humidification-flow firewall ownership"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern 'PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER' -Description "pipeline cooling humidification-flow exact source-order validation"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern 'PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE' -Description "pipeline cooling humidification-flow first-excluded validation"
Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern 'let transition_partition = checked_add\s*\(' -Description "pipeline cooling humidification-flow checked transition partition"

$coolingHumidificationFlowLinkFields = @(
    "source",
    "first_excluded_source",
    "source_order",
    "system",
    "parent_call_ordinal",
    "controlled_zone",
    "unit_body_entered",
    "predecessor_cooling_body_entered",
    "unit_off_skipped",
    "non_cooling_skipped",
    "cooling_body_entered"
)
foreach ($field in $coolingHumidificationFlowLinkFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern "flow\.$fieldPattern\b" -Description "pipeline cooling humidification-flow predecessor/provenance validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSnapshotSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling humidification-flow snapshot JSON for $field"
}

$coolingHumidificationFlowSourceSnapshotFields = @(
    "supply_mass_flow_rate_for_humidification_reset_assigned",
    "reset_supply_mass_flow_rate_for_humidification_kg_per_s",
    "heating_on_read",
    "heating_on",
    "heating_on_body_entered",
    "humidification_control_type_read",
    "humidification_control_type",
    "humidification_control_type_humidistat",
    "humidification_control_body_entered",
    "dehumidification_control_type_first_read",
    "first_dehumidification_control_type",
    "dehumidification_control_type_humidistat",
    "dehumidification_control_type_second_read",
    "second_dehumidification_control_type",
    "dehumidification_control_type_none",
    "humidification_control_condition_admitted",
    "zone_humidifying_setpoint_moisture_demand_read",
    "zone_humidifying_setpoint_moisture_demand_kg_per_s",
    "zone_humidifying_setpoint_moisture_demand_assigned",
    "assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s",
    "maximum_heating_supply_air_humidity_ratio_read",
    "maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air",
    "zone_humidity_ratio_read",
    "zone_humidity_ratio_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_calculated",
    "delta_humidity_ratio_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_assigned",
    "assigned_delta_humidity_ratio_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_for_gate_read",
    "delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air",
    "delta_humidity_ratio_comparison_evaluated",
    "delta_humidity_ratio_above_small_delta",
    "zone_humidifying_setpoint_moisture_demand_for_gate_read",
    "zone_humidifying_setpoint_moisture_demand_for_gate_kg_per_s",
    "zone_humidifying_setpoint_moisture_demand_comparison_evaluated",
    "zone_humidifying_setpoint_moisture_demand_above_zero",
    "humidification_flow_body_entered",
    "zone_humidifying_setpoint_moisture_demand_for_division_read",
    "zone_humidifying_setpoint_moisture_demand_for_division_kg_per_s",
    "delta_humidity_ratio_for_division_read",
    "delta_humidity_ratio_for_division_kg_water_per_kg_dry_air",
    "supply_mass_flow_rate_for_humidification_calculated",
    "calculated_supply_mass_flow_rate_for_humidification_kg_per_s",
    "supply_mass_flow_rate_for_humidification_assigned",
    "assigned_supply_mass_flow_rate_for_humidification_kg_per_s",
    "resulting_supply_mass_flow_rate_for_humidification_kg_per_s"
)
if (
    $coolingHumidificationFlowLinkFields.Count +
    $coolingHumidificationFlowSourceSnapshotFields.Count -ne 57
) {
    throw "CP320 pipeline snapshot audit must enumerate all 57 public fields"
}
foreach ($field in $coolingHumidificationFlowSourceSnapshotFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSnapshotValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling humidification-flow snapshot validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSnapshotSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling humidification-flow snapshot JSON for $field"
}

$coolingHumidificationFlowCommonLifecycleFields = @(
    "system",
    "transition_count",
    "cooling_body_entry_count",
    "unit_off_skip_count",
    "non_cooling_skip_count",
    "latest"
)
foreach ($field in $coolingHumidificationFlowCommonLifecycleFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlow -Pattern "\b$fieldPattern\b" -Description "pipeline cooling humidification-flow lifecycle validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling humidification-flow lifecycle JSON for $field"
}

$coolingHumidificationFlowCounterFields = @(
    "reset_assignment_count",
    "heating_on_read_count",
    "heating_on_body_entry_count",
    "heating_on_fallthrough_count",
    "humidification_control_type_read_count",
    "humidification_control_type_humidistat_count",
    "humidification_control_type_fallthrough_count",
    "humidification_control_body_entry_count",
    "dehumidification_control_type_first_read_count",
    "dehumidification_control_type_humidistat_count",
    "dehumidification_control_type_second_read_count",
    "dehumidification_control_type_none_count",
    "dehumidification_control_type_rejected_count",
    "admitted_control_body_entry_count",
    "moisture_demand_read_count",
    "moisture_demand_assignment_count",
    "maximum_heating_supply_humidity_ratio_read_count",
    "zone_humidity_ratio_read_count",
    "delta_calculation_count",
    "delta_assignment_count",
    "delta_gate_read_count",
    "delta_comparison_count",
    "delta_comparison_satisfied_count",
    "delta_fallthrough_count",
    "moisture_demand_gate_read_count",
    "moisture_demand_comparison_count",
    "moisture_demand_comparison_satisfied_count",
    "moisture_demand_fallthrough_count",
    "humidification_flow_body_entry_count",
    "moisture_demand_division_read_count",
    "delta_division_read_count",
    "calculation_count",
    "assignment_count"
)
if (
    $coolingHumidificationFlowCommonLifecycleFields.Count +
    $coolingHumidificationFlowCounterFields.Count -ne 39
) {
    throw "CP320 pipeline lifecycle audit must enumerate all 39 public state fields"
}
foreach ($field in $coolingHumidificationFlowCounterFields) {
    $fieldPattern = [regex]::Escape($field)
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowValidation -Pattern "\b$fieldPattern\b" -Description "pipeline cooling humidification-flow counter validation for $field"
    Assert-Contains -Path $runPurchasedAirCoolingHumidificationFlowSerialization -Pattern "`"$fieldPattern`"" -Description "pipeline cooling humidification-flow counter JSON for $field"
}

Assert-Contains -Path $runPipeline -Pattern '"purchased_air_calc_cooling_humidification_flow_lifecycle"' -Description "release cooling humidification-flow lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern '(?s)purchased_air_cooling_humidification_flow::validate_direct_lifecycle\(\s*result\s*\.purchased_air_calc_cooling_humidification_flow_lifecycle\s*\.as_ref\(\),\s*result\s*\.purchased_air_calc_cooling_dehumidification_flow_lifecycle\s*\.as_ref\(\),\s*init_lifecycle,\s*result\.purchased_air_coupling_call_count,\s*\)\?;' -Description "release exact cooling humidification-flow pipeline firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_humidification_flow_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct cooling humidification-flow evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_humidification_flow_validation_rejects_malformed_evidence' -Description "pipeline cooling humidification-flow malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_humidification_flow_json_exposes_all_source_sites' -Description "pipeline cooling humidification-flow all-site JSON regression"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'fn assert_cooling_humidification_flow\s*\(' -Description "direct run cooling humidification-flow lifecycle JSON assertion helper"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'purchased_air_calc_cooling_humidification_flow_lifecycle' -Description "direct run cooling humidification-flow lifecycle JSON assertion"
Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads.rs" -Pattern 'purchased_air_calc_cooling_humidification_flow_lifecycle' -Description "diagnostic run cooling humidification-flow null-evidence assertion"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern '"CP320 supersedes CP319' -Description "CP320 algorithm support-boundary addendum"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_humidification_flow\.rs::PurchasedAirCalcCoolingHumidificationFlowRuntimeState' -Description "CP320 algorithm Rust state target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_humidification_flow\.rs::PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary' -Description "CP320 algorithm Rust lifecycle target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_humidification_flow\.rs::purchased_air_calc_cooling_humidification_flow_lifecycle_summary' -Description "CP320 algorithm Rust lifecycle accessor target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_humidification_flow/release\.rs::advance_direct_no_oa_calc_cooling_humidification_flow' -Description "CP320 algorithm Rust release target"
Assert-Contains -Path "specs\capabilities.toml" -Pattern '"CP320 additionally requires' -Description "CP320 capability claim-boundary addendum"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP320 supersedes CP319' -Description "generated CP320 algorithm boundary"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP320 additionally requires' -Description "generated CP320 capability boundary"
foreach ($coolingHumidificationFlowIntegrationFile in @(
    $idealLoadsCoupledCoolingHumidificationFlowValidation,
    $runPurchasedAirCoolingHumidificationFlow,
    $runPurchasedAirCoolingHumidificationFlowSerialization,
    $runPurchasedAirCoolingHumidificationFlowSnapshotSerialization,
    $runPurchasedAirCoolingHumidificationFlowValidation,
    $runPurchasedAirCoolingHumidificationFlowSnapshotValidation
)) {
    foreach ($forbiddenBehavior in $coolingHumidificationFlowForbiddenBehaviorPatterns) {
        Assert-NotContains -Path $coolingHumidificationFlowIntegrationFile -Pattern $forbiddenBehavior.Pattern -Description $forbiddenBehavior.Description
    }
}
Assert-NotContains -Path $idealLoadsCoupledCoolingHumidificationFlowValidation -Pattern '(?i)(?:resulting|assigned|calculated)_supply_mass_flow_rate_for_humidification.*\bcoupling\b|\bcoupling\b.*(?:resulting|assigned|calculated)_supply_mass_flow_rate_for_humidification' -Description "CP320 candidate bitwise reconciliation with unchanged numerical DTO"

Assert-Contains -Path $runPipeline -Pattern 'topology_ready' -Description "release topology-ready JSON and validation evidence"
Assert-Contains -Path $runPipeline -Pattern 'topology_diagnostics' -Description "release ordered topology diagnostic evidence"
Assert-Contains -Path $runPipeline -Pattern 'topology_failure' -Description "release retained topology failure evidence"
Assert-Contains -Path $runPipeline -Pattern 'topology_completion_count' -Description "release topology completion-count evidence"
Assert-Contains -Path $runPipeline -Pattern 'sizing_attempt_count' -Description "release sizing attempt-count evidence"
Assert-Contains -Path $runPipeline -Pattern 'sized_limits' -Description "release four-field sizing-overlay evidence"
Assert-Contains -Path $runPipeline -Pattern 'sizing_outcome' -Description "release source-ordered sizing-outcome evidence"
Assert-Contains -Path $runPipeline -Pattern 'economizer_flow_limit_warning_count' -Description "release economizer advisory-count evidence"
Assert-Contains -Path $runPipeline -Pattern 'supply_temperature_diagnostic_registry' -Description "release supply-temperature registry JSON"
Assert-Contains -Path $runPipeline -Pattern '"identities": supply_temperature_diagnostics' -Description "release recurring identity JSON"
Assert-Contains -Path $runPipeline -Pattern 'supply_temperature_diagnostics_clear' -Description "release diagnostic-empty validator"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'lifecycle\.supply_temperature_registered_recurring_diagnostic_count == 0' -Description "coupled runtime recurring registry firewall"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'lifecycle\.supply_temperature_diagnostics\.is_empty\(\)' -Description "coupled runtime recurring identity firewall"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'registered_recurring_diagnostic_count' -Description "direct run zero recurring registry assertion"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'supply_temperature_diagnostic_registry' -Description "direct run diagnostic registry JSON assertion"
Assert-Contains -Path $idealLoadsUpdate -Pattern 'pub const fn supply_node_update_from_result\s*\(' -Description "IdealLoads supply-node update helper"

Assert-Contains -Path $idealLoadsInput -Pattern 'pub struct IdealLoadsFeatureFlags' -Description "IdealLoads compile feature flags"
Assert-Contains -Path $idealLoadsInput -Pattern 'pub fn from_system\s*\(' -Description "IdealLoads feature flag builder"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_outdoor_air' -Description "IdealLoads outdoor-air feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_economizer' -Description "IdealLoads economizer feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_heat_recovery' -Description "IdealLoads heat-recovery feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_dcv' -Description "IdealLoads DCV feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_humidistat' -Description "IdealLoads humidistat feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_constant_supply_humidity' -Description "IdealLoads constant supply humidity feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_flow_limit' -Description "IdealLoads flow-limit feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_capacity_limit' -Description "IdealLoads capacity-limit feature flag"
Assert-Contains -Path $idealLoadsInput -Pattern 'has_autosize' -Description "IdealLoads autosize feature flag"
Assert-Contains -Path $conformanceManifest -Pattern 'pub trace: Option<TraceContract>' -Description "case-level trace contract"
Assert-Contains -Path $conformanceManifest -Pattern 'pub struct TraceContract' -Description "trace contract schema type"
Assert-Contains -Path $conformanceManifest -Pattern 'require_non_empty\("trace\.level"' -Description "trace level validation"

Assert-Contains -Path $idealLoadsMod -Pattern 'mod meters;' -Description "IdealLoads meter submodule declaration"
Assert-Contains -Path $idealLoadsMod -Pattern 'pub use meters::\*;' -Description "IdealLoads meter public re-export"
Assert-Contains -Path $idealLoadsMeters -Pattern 'pub struct IdealLoadsFacilityMeterBinding' -Description "IdealLoads facility-meter binding type"
Assert-Contains -Path $idealLoadsMeters -Pattern 'pub const IDEAL_LOADS_METER_AGGREGATION_SOURCE' -Description "IdealLoads meter aggregation source label"
Assert-Contains -Path $idealLoadsMeters -Pattern 'pub fn ideal_loads_facility_meter_binding\s*\(' -Description "IdealLoads facility-meter binding helper"
Assert-Contains -Path $resultStore -Pattern 'pub struct ResultStore' -Description "runtime result store"
Assert-Contains -Path $resultStore -Pattern 'pub fn diagnostics\s*\(&self\)\s*->\s*RuntimeDiagnosticStore' -Description "ResultStore duplicate diagnostics"
Assert-Contains -Path $resultStore -Pattern 'pub fn profile\s*\(&self\)\s*->\s*ResultStoreProfile' -Description "ResultStore profile snapshot"
Assert-Contains -Path $resultStore -Pattern 'pub struct ResultStoreProfile' -Description "ResultStore profile type"
Assert-Contains -Path $resultStore -Pattern 'RuntimeDiagnosticCode::DuplicateOutputHandle' -Description "duplicate output handle diagnostic"
Assert-Contains -Path $resultStore -Pattern 'RuntimeDiagnosticCode::DuplicateOutputSeries' -Description "duplicate output series diagnostic"
Assert-Contains -Path $output -Pattern 'pub struct RuntimeOutputRegistry' -Description "runtime output registry"
Assert-Contains -Path $output -Pattern 'pub fn from_model\s*\(' -Description "runtime output registry model binding"
Assert-Contains -Path $output -Pattern 'pub fn resolve_output_requests\s*\(' -Description "runtime output request resolver"
Assert-Contains -Path $output -Pattern 'RuntimeDiagnosticCode::DuplicateOutputRequest' -Description "duplicate output request diagnostic"
Assert-Contains -Path $output -Pattern 'RuntimeDiagnosticCode::OutputVariableUnavailable' -Description "unavailable output variable diagnostic"
Assert-Contains -Path $runtimeOutputTests -Pattern 'runtime_output_registry_diagnoses_unavailable_output' -Description "unavailable output registry test"
Assert-Contains -Path $runtimeOutputTests -Pattern 'result_store_diagnostics_report_duplicate_system_node_handles' -Description "system-node duplicate result-store handle test"
Assert-Contains -Path $output -Pattern 'ideal_loads_facility_meter_binding' -Description "Runtime meter registry uses IdealLoads meter binding helper"
Assert-NotContains -Path $output -Pattern 'pub struct IdealLoadsFacilityMeterBinding' -Description "IdealLoads facility-meter binding type in generic output registry"
Assert-NotContains -Path $output -Pattern 'pub fn ideal_loads_facility_meter_binding\s*\(' -Description "IdealLoads facility-meter binding helper in generic output registry"

Assert-Contains -Path $idealLoadsReport -Pattern 'mod semantics;' -Description "IdealLoads report semantics submodule declaration"
Assert-Contains -Path $idealLoadsReport -Pattern 'pub use semantics::\*;' -Description "IdealLoads report semantics public re-export"
Assert-Contains -Path $idealLoadsReportSemantics -Pattern 'pub const IDEAL_LOADS_RATE_OUTPUT_SOURCE' -Description "IdealLoads ReportPurchasedAir rate source metadata"
Assert-Contains -Path $idealLoadsReport -Pattern 'pub struct IdealLoadsReportSnapshot' -Description "IdealLoads ReportPurchasedAir snapshot"
Assert-Contains -Path $idealLoadsReport -Pattern 'mod report_tests;' -Description "IdealLoads report test module declaration"
Assert-Contains -Path $idealLoadsReport -Pattern 'pub supply_air_latent_heating_rate_w: f64' -Description "IdealLoads report snapshot latent rate"
Assert-Contains -Path $idealLoadsReport -Pattern 'pub supply_mass_flow_rate_kg_per_s: f64' -Description "IdealLoads report snapshot supply mass flow"
Assert-Contains -Path $idealLoadsReport -Pattern 'pub supply_temperature_c: f64' -Description "IdealLoads report snapshot supply temperature"
Assert-Contains -Path $idealLoadsReport -Pattern 'pub supply_humidity_ratio: f64' -Description "IdealLoads report snapshot supply humidity"
Assert-Contains -Path $idealLoadsReportTests -Pattern 'fn report_snapshot_copies_every_reported_calculation_field\s*\(' -Description "IdealLoads report snapshot field-copy test"
Assert-Contains -Path $idealLoadsReportSemantics -Pattern 'pub const IDEAL_LOADS_ENERGY_OUTPUT_TIMESTEP_SOURCE' -Description "IdealLoads report-energy timestep metadata"
Assert-Contains -Path $idealLoadsReportSemantics -Pattern 'pub const IDEAL_LOADS_FUEL_ENERGY_OUTPUT_LEVEL_POLICY' -Description "IdealLoads fuel-energy level policy metadata"
Assert-NotContains -Path $idealLoadsReport -Pattern 'pub const IDEAL_LOADS_RATE_OUTPUT_SOURCE' -Description "ReportPurchasedAir semantics in report module root"

Assert-Contains -Path $outdoorAir -Pattern 'mod dcv;' -Description "outdoor-air DCV submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod design_flow;' -Description "outdoor-air design-flow submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod minimum_flow;' -Description "minimum outdoor-air flow resolver submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod economizer;' -Description "outdoor-air economizer submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod mixed_air;' -Description "outdoor-air mixed-air submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod psychrometrics;' -Description "outdoor-air psychrometrics submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern 'mod supply;' -Description "outdoor-air supply submodule declaration"
Assert-Contains -Path $outdoorAir -Pattern '#\[path = "outdoor_air_tests\.rs"\]' -Description "outdoor-air test module path declaration"
Assert-Contains -Path $outdoorAir -Pattern '#\[path = "outdoor_air_wrapper_tests\.rs"\]' -Description "outdoor-air wrapper test module path declaration"
Assert-Contains -Path $outdoorAir -Pattern 'pub use dcv::\*;' -Description "outdoor-air DCV public re-export"
Assert-Contains -Path $outdoorAir -Pattern 'pub use design_flow::\*;' -Description "outdoor-air design-flow public re-export"
Assert-Contains -Path $outdoorAir -Pattern 'pub use minimum_flow::\*;' -Description "minimum outdoor-air flow public types"
Assert-Contains -Path $outdoorAirTests -Pattern '#\[test\]' -Description "outdoor-air root unit tests"
Assert-Contains -Path $outdoorAirDesignFlow -Pattern 'pub fn design_outdoor_air_volume_flow_components_m3_per_s\s*\(' -Description "outdoor-air design-flow component helper"
Assert-Contains -Path $outdoorAirDesignFlow -Pattern 'pub fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air scheduled mass-flow helper"
Assert-Contains -Path $outdoorAirDesignFlow -Pattern 'fn nonnegative_product\s*\(' -Description "outdoor-air design-flow scalar helper"
Assert-Contains -Path $outdoorAirTests -Pattern 'fn sum_combines_supported_terms\s*\(' -Description "outdoor-air Sum runtime test"
Assert-Contains -Path $outdoorAirTests -Pattern 'fn maximum_selects_largest_supported_term\s*\(' -Description "outdoor-air Maximum runtime test"
Assert-Contains -Path $outdoorAirDcv -Pattern 'pub fn calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "OccupancySchedule DCV helper"
Assert-Contains -Path $outdoorAirDcv -Pattern 'pub fn calc_co2_setpoint_dcv_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "CO2Setpoint DCV helper"
Assert-Contains -Path $outdoorAirDcvTests -Pattern 'fn co2_setpoint_dcv_selects_required_flow_above_minimum\s*\(' -Description "CO2Setpoint controlling-demand runtime test"
Assert-Contains -Path $outdoorAirDcvTests -Pattern 'fn co2_setpoint_dcv_preserves_minimum_for_noncontrolling_requirements\s*\(' -Description "CO2Setpoint minimum-flow runtime test"
Assert-Contains -Path $outdoorAirMinimumFlow -Pattern 'fn resolve_minimum_outdoor_air_compat\s*\(' -Description "source-order minimum outdoor-air resolver"
Assert-Contains -Path $outdoorAirMinimumFlow -Pattern 'pub enum SimPurchasedAirOutdoorAirCompatError' -Description "minimum outdoor-air typed error boundary"
Assert-Contains -Path $outdoorAir -Pattern 'resolve_minimum_outdoor_air_compat\s*\(' -Description "outdoor-air wrapper owns minimum-flow resolution"
Assert-Contains -Path $outdoorAirWrapperTests -Pattern 'fn wrapper_resolves_design_flow_before_calc_update_and_trace\s*\(' -Description "outdoor-air wrapper design-flow ownership test"
Assert-Contains -Path $outdoorAirWrapperTests -Pattern 'fn wrapper_occupancy_dcv_recomputes_sum_before_schedule_and_density\s*\(' -Description "outdoor-air wrapper OccupancySchedule source-order test"
Assert-Contains -Path $outdoorAirWrapperTests -Pattern 'fn wrapper_co2_dcv_applies_max_then_explicit_nonfinite_guard\s*\(' -Description "outdoor-air wrapper CO2Setpoint source-order test"
Assert-Contains -Path $outdoorAirWrapperTests -Pattern 'fn wrapper_applies_energyplus_very_small_mass_flow_cutoff\s*\(' -Description "outdoor-air wrapper VerySmallMassFlow cutoff test"
Assert-Contains -Path $outdoorAirWrapperTests -Pattern 'fn wrapper_reports_missing_or_unsupported_minimum_flow_inputs\s*\(' -Description "outdoor-air wrapper typed error tests"
Assert-Contains -Path $outdoorAirEconomizer -Pattern 'fn calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "economizer OA flow helper"
Assert-Contains -Path $outdoorAirMixedAir -Pattern 'fn mixed_air_state\s*\(' -Description "mixed-air state helper"
Assert-Contains -Path $outdoorAirMixedAir -Pattern 'fn heat_recovery_allows_outdoor_air_tempering\s*\(' -Description "heat recovery activation helper"
Assert-Contains -Path $outdoorAirPsychrometrics -Pattern 'fn heat_recovery_saturation_adjusted_state\s*\(' -Description "heat recovery saturation helper"
Assert-Contains -Path $outdoorAirSupply -Pattern 'fn outdoor_air_supply_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air supply mass-flow helper"
Assert-Contains -Path $outdoorAirSupply -Pattern 'fn supply_air_state\s*\(' -Description "outdoor-air supply state helper"
Assert-Contains -Path $outdoorAir -Pattern 'pub fn sim_purchased_air_outdoor_air_compat\s*\(' -Description "outdoor-air source-order wrapper"
Assert-NotContains -Path $outdoorAir -Pattern 'pub fn design_outdoor_air_volume_flow_components_m3_per_s\s*\(' -Description "outdoor-air design-flow component helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'pub fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air scheduled mass-flow helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn nonnegative_product\s*\(' -Description "outdoor-air design-flow scalar helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "OccupancySchedule DCV helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn calc_co2_setpoint_dcv_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "CO2Setpoint DCV helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s\s*\(' -Description "economizer OA flow helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn mixed_air_state\s*\(' -Description "mixed-air state helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn heat_recovery_allows_outdoor_air_tempering\s*\(' -Description "heat recovery activation helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn heat_recovery_saturation_adjusted_state\s*\(' -Description "heat recovery saturation helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn outdoor_air_supply_mass_flow_rate_kg_per_s\s*\(' -Description "outdoor-air supply mass-flow helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'fn supply_air_state\s*\(' -Description "outdoor-air supply state helper in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern '#\[test\]' -Description "unit test body in outdoor-air root"
Assert-NotContains -Path $outdoorAir -Pattern 'energyplus_psychrometric_humidity_ratio_from_rh' -Description "psychrometric humidity-ratio helper import in outdoor-air root"
Assert-NotContains -Path $outdoorAirSumCompare -Pattern '\$summarySum\b|\$stageSum\b' -Description "script-owned outdoor-air Sum reconstruction"
Assert-NotContains -Path $outdoorAirMaximumCompare -Pattern '\$summaryMaximum\b|\$stageMaximum\b' -Description "script-owned outdoor-air Maximum reconstruction"

Assert-Contains -Path $dispatch -Pattern 'pub fn sim_purchased_air_compat\s*\(' -Description "SimPurchasedAir source-order wrapper"
Assert-Contains -Path $dispatch -Pattern 'pub struct IdealLoadsCompiledBranchFlags' -Description "IdealLoads cached branch flags"
Assert-Contains -Path $dispatch -Pattern 'pub fn sim_purchased_air_compat_with_branch_flags\s*\(' -Description "SimPurchasedAir wrapper accepts cached branch flags"
Assert-Contains -Path $dispatch -Pattern 'purchased_air_source_order_stages\s*\(' -Description "PurchasedAir source-order stage summary"
foreach ($routine in @(
        "GetPurchasedAir",
        "InitPurchasedAir",
        "CalcPurchAirLoads",
        "UpdatePurchasedAir",
        "ReportPurchasedAir"
    )) {
    Assert-Contains -Path $dispatch -Pattern "source_routine: `"$routine`"" -Description "PurchasedAirManager routine $routine"
}
Assert-Contains -Path $dispatch -Pattern 'pub const IDEAL_LOADS_RUNTIME_BINDING_SOURCE' -Description "IdealLoads runtime typed-ID binding metadata"
Assert-Contains -Path $dispatch -Pattern 'pub const IDEAL_LOADS_RUNTIME_STRING_LOOKUP_POLICY' -Description "IdealLoads runtime string lookup policy metadata"
Assert-Contains -Path $dispatch -Pattern 'pub const IDEAL_LOADS_SIZE_PURCHASED_AIR_POLICY' -Description "IdealLoads SizePurchasedAir autosize policy metadata"
Assert-Contains -Path $dispatch -Pattern 'executes the persistent direct hard-sized SizePurchasedAir legacy route' -Description "IdealLoads bounded SizePurchasedAir policy"
Assert-Contains -Path $dispatch -Pattern 'IdealLoadsFeatureFlags::from_system' -Description "PurchasedAir branch dispatch uses compile feature flags"
Assert-Contains -Path $idealLoadsRuntime -Pattern 'IdealLoadsCompiledBranchFlags::from_system' -Description "IdealLoads runtime caches branch flags per system"
Assert-Contains -Path $idealLoadsRuntime -Pattern 'sim_purchased_air_compat_with_branch_flags' -Description "IdealLoads runtime uses cached branch flags"
Assert-Contains -Path $dispatch -Pattern 'calc_no_oa_sensible_with_limits_and_recirculation_compat\s*\(' -Description "finite-limit branch dispatch"
Assert-Contains -Path $dispatch -Pattern 'calc_no_oa_no_limit_sensible_with_recirculation_context_compat\s*\(' -Description "no-limit branch dispatch"
Assert-NotContains -Path $dispatch -Pattern 'calc_outdoor_air_sensible_report_rates_compat\s*\(' -Description "outdoor-air diagnostic calculation in source-order conformance wrapper"

$runtimeFormulaPatterns = @(
    'fn calc_no_oa_',
    'fn calc_outdoor_air_',
    'fn heating_result_with_limits\s*\(',
    'fn cooling_result_with_limits\s*\(',
    'fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s\s*\('
)
foreach ($pattern in $runtimeFormulaPatterns) {
    Assert-NotContains -Path $runtime -Pattern $pattern -Description "IdealLoads branch formula in runtime root"
}

Assert-Contains -Path $node -Pattern 'mod state;' -Description "node state submodule declaration"
Assert-Contains -Path $node -Pattern 'pub use state::\*;' -Description "node state public re-export"
Assert-Contains -Path $node -Pattern 'mod projection;' -Description "node projection submodule declaration"
Assert-Contains -Path $node -Pattern 'pub use projection::\*;' -Description "node projection public re-export"
Assert-Contains -Path $node -Pattern 'pub struct IdealLoadsSupplyNodeUpdate' -Description "IdealLoads node update transfer struct"
Assert-Contains -Path $node -Pattern 'IDEAL_LOADS_NODE_OUTPUT_STORE_TYPE' -Description "IdealLoads node output store metadata"
Assert-Contains -Path $nodeProjection -Pattern 'pub struct NodeStateProjectionOptions' -Description "node projection options"
Assert-Contains -Path $nodeProjection -Pattern 'pub struct NodeStateProjectionEvidencePolicy' -Description "node projection evidence policy"
Assert-Contains -Path $nodeProjection -Pattern 'pub fn simulate_ideal_loads_node_state_projection\s*\(' -Description "node projection runtime function"
Assert-Contains -Path $nodeState -Pattern 'pub struct NodeStateStore' -Description "NodeStateStore implementation"
Assert-Contains -Path $nodeState -Pattern 'pub struct AirNodeState' -Description "AirNodeState implementation"
Assert-Contains -Path $nodeState -Pattern 'pub enum NodeStateRole' -Description "NodeStateRole implementation"
Assert-Contains -Path $nodeState -Pattern 'pub const NODE_STATE_SOURCE_MAP_PATH' -Description "node-state source-map metadata"
Assert-Contains -Path $nodeState -Pattern 'pub fn node_temperature_setpoint_from_energyplus\s*\(' -Description "EnergyPlus node setpoint sentinel adapter"
Assert-NotContains -Path $nodeState -Pattern 'IdealLoadsSupplyNodeUpdate' -Description "IdealLoads supply-node transfer struct in generic NodeStateStore"
Assert-NotContains -Path $nodeState -Pattern 'ideal_loads::' -Description "IdealLoads module dependency in generic NodeStateStore"
Assert-NotContains -Path $runtime -Pattern 'pub struct NodeStateProjectionOptions' -Description "node projection options in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct NodeStateProjectionEvidencePolicy' -Description "node projection evidence policy in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub fn simulate_ideal_loads_node_state_projection\s*\(' -Description "node projection runtime function in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct NodeStateStore' -Description "NodeStateStore implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct AirNodeState' -Description "AirNodeState implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub enum NodeStateRole' -Description "NodeStateRole implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub const NODE_STATE_SOURCE_MAP_PATH' -Description "node-state source-map metadata in runtime root"

Assert-Contains -Path $lib -Pattern 'pub mod plant;' -Description "plant module public declaration"
Assert-Contains -Path $lib -Pattern 'pub use plant::\*;' -Description "plant module public re-export"
Assert-Contains -Path $plant -Pattern 'mod state;' -Description "plant state submodule declaration"
Assert-Contains -Path $plant -Pattern 'pub use state::\*;' -Description "plant state public re-export"
Assert-Contains -Path $plantState -Pattern 'pub struct PlantStateStore' -Description "PlantStateStore implementation"
Assert-Contains -Path $plantState -Pattern 'pub struct PlantStateProjectionEvidencePolicy' -Description "plant projection evidence policy"
Assert-Contains -Path $plantState -Pattern 'pub const PLANT_STATE_SOURCE_MAP_PATH' -Description "plant-state source-map metadata"
Assert-Contains -Path $plantState -Pattern 'pub fn simulate_plant_state_projection\s*\(' -Description "plant projection runtime function"
Assert-NotContains -Path $runtime -Pattern 'pub struct PlantStateStore' -Description "PlantStateStore implementation in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub struct PlantStateProjectionEvidencePolicy' -Description "plant projection evidence policy in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub const PLANT_STATE_SOURCE_MAP_PATH' -Description "plant-state source-map metadata in runtime root"
Assert-NotContains -Path $runtime -Pattern 'pub fn simulate_plant_state_projection\s*\(' -Description "plant projection runtime function in runtime root"

Assert-Contains -Path $zoneEquipment -Pattern 'mod demand;' -Description "zone equipment demand submodule declaration"
Assert-Contains -Path $zoneEquipment -Pattern 'pub use demand::\*;' -Description "zone equipment demand public re-export"
Assert-Contains -Path $zoneEquipment -Pattern 'mod dispatch;' -Description "zone equipment dispatch submodule declaration"
Assert-Contains -Path $zoneEquipment -Pattern 'pub use dispatch::\*;' -Description "zone equipment dispatch public re-export"
Assert-Contains -Path $zoneEquipment -Pattern '#\[path = "tests\.rs"\]' -Description "zone equipment test module path declaration"
Assert-Contains -Path $zoneEquipmentDemand -Pattern 'pub struct ZoneSysEnergyDemand' -Description "ZoneSysEnergyDemand source-order demand struct"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'pub const fn ideal_loads_zone_equipment_stages\s*\(' -Description "Zone equipment compatibility stages"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'pub fn validate_ideal_loads_zone_equipment_dispatch\s*\(' -Description "IdealLoads zone equipment dispatch validation"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'SupplyNodeNotInZoneInletList' -Description "IdealLoads supply node inlet-list validation"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'SequenceAmbiguity' -Description "IdealLoads sequence ambiguity validation"
Assert-Contains -Path $zoneEquipmentDispatch -Pattern 'MultipleZoneEquipmentDiagnosticOnly' -Description "IdealLoads multiple-equipment diagnostic boundary"
Assert-NotContains -Path $zoneEquipment -Pattern 'pub struct ZoneSysEnergyDemand' -Description "ZoneSysEnergyDemand implementation in zone equipment facade"
Assert-NotContains -Path $zoneEquipment -Pattern 'pub fn validate_ideal_loads_zone_equipment_dispatch\s*\(' -Description "dispatch validation implementation in zone equipment facade"
Assert-NotContains -Path $zoneEquipment -Pattern '#\[test\]' -Description "unit tests in zone equipment facade"
Assert-Contains -Path $executionPlan -Pattern 'ExecutionStep::ManageZoneEquipment' -Description "ZoneEquipmentManager stage in execution plan"
Assert-Contains -Path $executionPlan -Pattern 'ExecutionStep::SimZoneEquipment' -Description "SimZoneEquipment stage in execution plan"
Assert-Contains -Path $executionPlan -Pattern 'ExecutionStep::EvaluateIdealLoadsAirSystem' -Description "IdealLoads evaluation stage in execution plan"
Assert-Contains -Path $executionPlan -Pattern 'energyplus_ideal_loads_compatibility_stages' -Description "IdealLoads PurchasedAirManager source stage list"
Assert-Contains -Path $executionPlan -Pattern 'source_routine: "SimPurchasedAir"' -Description "SimPurchasedAir execution-plan source routine"
Assert-Contains -Path $executionPlan -Pattern 'source_routine: "GetPurchasedAir"' -Description "GetPurchasedAir execution-plan source routine"
Assert-Contains -Path $executionPlan -Pattern 'source_routine: "InitPurchasedAir"' -Description "InitPurchasedAir execution-plan source routine"
Assert-Contains -Path $executionPlan -Pattern 'source_routine: "CalcPurchAirLoads"' -Description "CalcPurchAirLoads execution-plan source routine"
Assert-Contains -Path $executionPlan -Pattern 'source_routine: "UpdatePurchasedAir"' -Description "UpdatePurchasedAir execution-plan source routine"
Assert-Contains -Path $executionPlan -Pattern 'source_routine: "ReportPurchasedAir"' -Description "ReportPurchasedAir execution-plan source routine"
Assert-Contains -Path $runSupport -Pattern 'active_ideal_loads_branches' -Description "run support active IdealLoads branch list"
Assert-Contains -Path $runSupport -Pattern 'inactive_ideal_loads_branches' -Description "run support inactive IdealLoads branch list"
Assert-Contains -Path $runPipeline -Pattern '"active_ideal_loads_branches"' -Description "run-summary active IdealLoads branch list"
Assert-Contains -Path $runPipeline -Pattern '"inactive_ideal_loads_branches"' -Description "run-summary inactive IdealLoads branch list"
Assert-Contains -Path $runRuntimeBoundaries -Pattern 'UnsupportedHeatBalanceBranch' -Description "unsupported active IdealLoads branch fails support assessment"
Assert-Contains -Path $runRuntimeBoundaries -Pattern 'unsupported_features_for_selected_branch' -Description "selected branch unsupported feature guard"
Assert-NotContains -Path $runtime -Pattern 'calc_no_oa_no_limit_sensible_compat\s*\(' -Description "IdealLoads no-limit branch formula call in runtime root"
Assert-NotContains -Path $runtime -Pattern 'calc_no_oa_sensible_with_limits_compat\s*\(' -Description "IdealLoads finite-limit branch formula call in runtime root"
Assert-NotContains -Path $runtime -Pattern 'calc_outdoor_air_sensible_report_rates_compat\s*\(' -Description "IdealLoads outdoor-air report calc call in runtime root"
Assert-NotContains -Path $runtime -Pattern 'sim_purchased_air_outdoor_air_compat\s*\(' -Description "IdealLoads outdoor-air diagnostic wrapper call in runtime root"
Assert-Contains -Path $idealLoadsMod -Pattern 'mod humidistat;' -Description "IdealLoads Humidistat transition module declaration"
Assert-Contains -Path $idealLoadsMod -Pattern 'pub use humidistat::\*;' -Description "IdealLoads Humidistat transition public re-export"
Assert-Contains -Path $idealLoadsHumidistat -Pattern 'pub struct NoOaHumidistatClosedLoopState' -Description "Humidistat closed-loop runtime state"
Assert-Contains -Path $idealLoadsHumidistat -Pattern 'pub struct NoOaHumidistatZoneTimestepInput' -Description "Humidistat closed-loop typed zone-timestep input"
Assert-Contains -Path $idealLoadsHumidistat -Pattern 'pub struct NoOaHumidistatZoneTimestepOutput' -Description "Humidistat closed-loop typed zone-timestep output"
Assert-Contains -Path $idealLoadsHumidistat -Pattern 'pub enum NoOaHumidistatZoneTimestepError' -Description "Humidistat closed-loop typed zone-timestep error"
Assert-Contains -Path $idealLoadsHumidistat -Pattern 'pub fn advance_no_oa_humidistat_zone_timestep_compat\s*\(' -Description "Humidistat closed-loop runtime zone-timestep transition"
Assert-Contains -Path $idealLoadsHumidistatTests -Pattern 'fn no_oa_humidistat_closed_loop_advances_corrected_humidity_histories\s*\(' -Description "Humidistat history transition runtime test"
Assert-Contains -Path $idealLoadsHumidistatTests -Pattern 'fn no_oa_humidistat_closed_loop_reuses_committed_history_on_next_step\s*\(' -Description "Humidistat committed-history runtime test"
Assert-Contains -Path $idealLoadsHumidistatTests -Pattern 'fn no_oa_humidistat_closed_loop_divides_supply_flow_by_zone_multiplier_before_correction\s*\(' -Description "Humidistat zone-multiplier runtime test"
Assert-Contains -Path $idealLoadsHumidistatTests -Pattern 'fn no_oa_humidistat_closed_loop_preserves_state_when_predictor_rejects_input\s*\(' -Description "Humidistat atomic predictor-error runtime test"
Assert-Contains -Path $idealLoadsHumidistatTests -Pattern 'fn no_oa_humidistat_closed_loop_rejects_non_humidistat_branch_without_advancing_state\s*\(' -Description "Humidistat branch-ownership runtime test"
Assert-Contains -Path $idealLoadsHumidistatTests -Pattern 'fn no_oa_humidistat_closed_loop_preserves_state_when_purchased_air_rejects_input\s*\(' -Description "Humidistat atomic PurchasedAir-error runtime test"
Assert-Contains -Path $idealLoadsHumidistatTests -Pattern 'fn no_oa_humidistat_closed_loop_preserves_state_when_corrector_rejects_input\s*\(' -Description "Humidistat atomic corrector-error runtime test"
Assert-NotContains -Path $idealLoadsHumidistat -Pattern 'ep_compare|ep_conformance|IdealLoadsInputTrace|LoadedSeries|std::path::Path' -Description "case/report adaptation in Humidistat runtime transition"
Assert-NotContains -Path $idealLoadsHumidistat -Pattern '#\[test\]' -Description "inline tests in Humidistat runtime transition"
Assert-NotContains -Path $idealLoadsHumidistat -Pattern 'pub timestep_seconds:' -Description "ambiguous Humidistat system-substep input"
Assert-Contains -Path $idealLoadsCli -Pattern 'mod case_adapter;' -Description "IdealLoads CLI case-adapter module declaration"
Assert-Contains -Path $idealLoadsCliCaseAdapter -Pattern 'mod time_axis;' -Description "IdealLoads CLI time-axis adapter module declaration"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapter -Pattern 'time_axis_timestep_profile\s*\(' -Description "IdealLoads adapter uses runtime TimeAxis timestep profile"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapter -Pattern 'pub\(in crate::ideal_loads\) fn ideal_loads_timestep_context\s*\(' -Description "IdealLoads nominal timestep adapter"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapter -Pattern 'pub\(in crate::ideal_loads\) fn ideal_loads_sample_timestep_seconds\s*\(' -Description "IdealLoads exact sample-seconds adapter"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapter -Pattern 'pub\(in crate::ideal_loads\) fn ideal_loads_sample_timestep_hours\s*\(' -Description "IdealLoads exact sample-hours adapter"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapterTests -Pattern 'fn ideal_loads_timestep_context_uses_runtime_time_axis_nominal_values\s*\(' -Description "IdealLoads TimeAxis nominal metadata test"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapterTests -Pattern 'fn ideal_loads_timestep_context_does_not_validate_unrelated_run_period_dates\s*\(' -Description "IdealLoads timestep-only adapter run-period decoupling test"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapterTests -Pattern 'fn ideal_loads_sample_timestep_uses_valid_duration_and_timestamp_precision_normalization\s*\(' -Description "IdealLoads sample duration and timestamp-precision test"
Assert-Contains -Path $idealLoadsCliTimeAxisAdapterTests -Pattern 'fn ideal_loads_sample_timestep_falls_back_for_missing_or_invalid_timestamp\s*\(' -Description "IdealLoads nominal timestamp fallback test"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_sample_timestep_seconds\s*\(' -Description "IdealLoads report-energy uses exact sample duration adapter"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_sample_timestep_hours\s*\(' -Description "IdealLoads outdoor-air active time uses exact sample duration adapter"
Assert-Contains -Path $idealLoadsCli -Pattern 'timestep_source: \{\}' -Description "IdealLoads report exposes TimeAxis source"
Assert-Contains -Path $idealLoadsCli -Pattern 'adaptive_system_timestep_claim' -Description "IdealLoads report preserves adaptive timestep non-claim"
Assert-NotContains -Path $idealLoadsCli -Pattern 'IDEAL_LOADS_(NO_OA_ENERGY|OUTDOOR_AIR)_SYSTEM_SUBSTEPS' -Description "fixed IdealLoads system-substep constant"
Assert-NotContains -Path $idealLoadsCli -Pattern 'fn ideal_loads_(system_timestep_seconds|energy_report_interval_seconds|outdoor_air_sample_timestep_hours)\s*\(' -Description "legacy IdealLoads fixed-timestep helper"
Assert-NotContains -Path $idealLoadsCli -Pattern 'fn energy_report_seconds_from_timestamp\s*\(' -Description "duplicate IdealLoads report-energy timestamp heuristic"
Assert-NotContains -Path $idealLoadsCli -Pattern 'fixed_system_substeps|fixed 8-substep fixture branch' -Description "fixed IdealLoads substep report metadata"
Assert-NotContains -Path $idealLoadsCli -Pattern '(?<!nominal_)system_timestep_substeps|(?<!nominal_)system_timestep_seconds|energy_report_interval_seconds' -Description "ambiguous legacy IdealLoads timestep JSON key"
Assert-Contains -Path $idealLoadsCli -Pattern 'IdealLoadsReportSnapshot' -Description "IdealLoads CLI ReportPurchasedAir snapshot consumer"
Assert-NotContains -Path $idealLoadsCli -Pattern 'IdealLoadsSensibleResult' -Description "IdealLoads calculation result type in CLI"
Assert-NotContains -Path $idealLoadsCliOutdoorAirJson -Pattern '(?<!nominal_)system_timestep_substeps|(?<!nominal_)system_timestep_seconds|energy_report_interval_seconds|fixed 8-substep' -Description "legacy IdealLoads outdoor-air timestep metadata"
Assert-NotContains -Path $idealLoadsCliOutdoorAirMarkdown -Pattern 'fixed_system_substeps|fixed 8-substep|112\.5' -Description "fixed IdealLoads outdoor-air timestep claim"
Assert-Contains -Path $outdoorAirSumCompare -Pattern 'timestep_source -ne "ep_runtime::TimeAxis"' -Description "outdoor-air TimeAxis report assertion"
Assert-Contains -Path $outdoorAirSumCompare -Pattern 'adaptive_system_timestep_claim -ne \$false' -Description "outdoor-air adaptive timestep non-claim assertion"
Assert-Contains -Path $outdoorAirSumCompare -Pattern 'sample_timestep_source -ne "ESO timestamp duration with ep_runtime::TimeAxis integer-substep normalization and nominal fallback"' -Description "outdoor-air sample timestep source assertion"
Assert-Contains -Path $idealLoadsCli -Pattern 'mod commands;' -Description "IdealLoads CLI commands module declaration"
Assert-Contains -Path $idealLoadsCli -Pattern 'pub\(crate\) use commands::' -Description "IdealLoads CLI command facade re-export"
Assert-Contains -Path $idealLoadsCli -Pattern 'mod reports;' -Description "IdealLoads CLI reports module declaration"
Assert-Contains -Path $idealLoadsCliReports -Pattern 'mod outdoor_air;' -Description "IdealLoads CLI outdoor-air reports module declaration"
Assert-Contains -Path $idealLoadsCliReports -Pattern 'pub\(super\) use outdoor_air::write_outdoor_air_artifacts;' -Description "IdealLoads CLI outdoor-air writer facade"
Assert-Contains -Path $idealLoadsCliOutdoorAirReports -Pattern 'mod csv;' -Description "IdealLoads CLI outdoor-air CSV module declaration"
Assert-Contains -Path $idealLoadsCliOutdoorAirReports -Pattern 'mod json;' -Description "IdealLoads CLI outdoor-air JSON module declaration"
Assert-Contains -Path $idealLoadsCliOutdoorAirReports -Pattern 'mod markdown;' -Description "IdealLoads CLI outdoor-air Markdown module declaration"
Assert-Contains -Path $idealLoadsCliOutdoorAirReports -Pattern 'pub\(in crate::ideal_loads\) fn write_outdoor_air_artifacts\s*\(' -Description "IdealLoads CLI outdoor-air artifact writer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirMarkdown -Pattern 'pub\(super\) fn render_outdoor_air_markdown\s*\(' -Description "IdealLoads CLI outdoor-air Markdown renderer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirJson -Pattern 'pub\(super\) fn render_outdoor_air_summary_json\s*\(' -Description "IdealLoads CLI outdoor-air summary JSON renderer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirJson -Pattern 'pub\(super\) fn render_outdoor_air_selected_outputs_json\s*\(' -Description "IdealLoads CLI outdoor-air selected-outputs JSON renderer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirJson -Pattern 'pub\(super\) fn render_outdoor_air_result_store_json\s*\(' -Description "IdealLoads CLI outdoor-air result-store JSON renderer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirJson -Pattern 'pub\(super\) fn render_outdoor_air_stage_summary_json\s*\(' -Description "IdealLoads CLI outdoor-air stage-summary JSON renderer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirCsv -Pattern 'pub\(super\) fn render_outdoor_air_variable_deltas_csv\s*\(' -Description "IdealLoads CLI outdoor-air variable-deltas CSV renderer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirCsv -Pattern 'pub\(super\) fn render_outdoor_air_first_divergence_csv\s*\(' -Description "IdealLoads CLI outdoor-air first-divergence CSV renderer ownership"
Assert-Contains -Path $idealLoadsCliOutdoorAirCsv -Pattern 'pub\(super\) fn render_outdoor_air_tolerance_failures_csv\s*\(' -Description "IdealLoads CLI outdoor-air tolerance-failures CSV renderer ownership"
Assert-NotContains -Path $idealLoadsCli -Pattern 'fn (write_outdoor_air_artifacts|render_outdoor_air_(markdown|summary_json|selected_outputs_json|result_store_json|variable_deltas_csv|first_divergence_csv|tolerance_failures_csv|stage_summary_json))\s*\(' -Description "IdealLoads outdoor-air report renderer implementation in facade"
Assert-Contains -Path $idealLoadsCliCommands -Pattern 'pub\(crate\) struct IdealLoadsDiagnosticReportSummary' -Description "IdealLoads CLI command artifact summary"
Assert-Contains -Path $idealLoadsCliCommands -Pattern 'pub\(crate\) fn generate_ideal_loads_no_oa_sensible_report\s*\(' -Description "IdealLoads no-OA command entry point"
Assert-Contains -Path $idealLoadsCliCommands -Pattern 'pub\(crate\) fn generate_ideal_loads_outdoor_air_design_flow_report\s*\(' -Description "IdealLoads outdoor-air command entry point"
Assert-NotContains -Path $idealLoadsCli -Pattern 'pub\(crate\) struct IdealLoadsDiagnosticReportSummary' -Description "IdealLoads CLI command artifact summary in facade"
Assert-NotContains -Path $idealLoadsCli -Pattern 'pub\(crate\) fn generate_ideal_loads_(no_oa_sensible|outdoor_air_design_flow)_report\s*\(' -Description "IdealLoads command implementation in facade"
Assert-Contains -Path $idealLoadsCli -Pattern 'zone_equipment_dispatch_path' -Description "IdealLoads report zone-equipment dispatch path metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_invocation_path' -Description "IdealLoads report source-order invocation path metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'direct_calc_helper_invocation' -Description "IdealLoads report direct-helper invocation metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'zone_equipment_dispatch_execution_boundary' -Description "IdealLoads report zone-equipment execution boundary metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_trace_level\s*\(' -Description "IdealLoads report manifest trace-level helper"
Assert-Contains -Path $idealLoadsCli -Pattern 'trace_level_source' -Description "IdealLoads report trace-level source metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'trace_result_invariance_policy' -Description "IdealLoads report trace-result invariance metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'trace_overhead_accounting' -Description "IdealLoads report trace overhead accounting metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'zone_equipment_dispatch_validation' -Description "IdealLoads report zone-equipment dispatch validation metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'zone_equipment_conformance_candidate' -Description "IdealLoads report zone-equipment conformance-candidate metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_feature_flags' -Description "IdealLoads report feature flag metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_feature_dispatch_policy' -Description "IdealLoads report feature-dispatch policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_prebound_id_contract' -Description "IdealLoads report prebound-ID contract metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_psychrometric_evaluation_policy' -Description "IdealLoads report psychrometric evaluation policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_psychrometric_cache_policy' -Description "IdealLoads report psychrometric cache policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'resolve_ideal_loads_output_handles' -Description "IdealLoads setup-time output handle resolver"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_output_handle_registration_policy' -Description "IdealLoads report output handle registration policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_output_handle_write_policy' -Description "IdealLoads report output handle write policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_diagnostic_output_request_policy' -Description "IdealLoads report diagnostic output request policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_report_export_order_policy' -Description "IdealLoads report export order policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_detailed_output_lookup_policy' -Description "IdealLoads report Detailed output lookup policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_duplicate_output_handle_policy' -Description "IdealLoads report duplicate output handle policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'ideal_loads_runtime_binding_source' -Description "IdealLoads report typed-ID binding metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'purchased_air_name_lookup_policy' -Description "IdealLoads report string lookup policy metadata"
Assert-Contains -Path $idealLoadsCli -Pattern 'sim_purchased_air_compat' -Description "IdealLoads no-OA report generator uses source-order wrapper"
Assert-Contains -Path $idealLoadsCli -Pattern 'sim_purchased_air_outdoor_air_compat' -Description "IdealLoads outdoor-air report generator uses source-order wrapper"
Assert-Contains -Path $idealLoadsCli -Pattern 'IdealLoadsMinimumOutdoorAirCompatInput' -Description "IdealLoads outdoor-air reporter passes raw minimum-flow inputs to runtime"
foreach ($minimumFlowHelper in @(
        'design_outdoor_air_volume_flow_components_m3_per_s',
        'calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s',
        'calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s',
        'calc_co2_setpoint_dcv_outdoor_air_mass_flow_rate_kg_per_s'
    )) {
    foreach ($idealLoadsCliPhysicsFile in $idealLoadsCliPhysicsBoundaryFiles) {
        Assert-NotContains -Path $idealLoadsCliPhysicsFile -Pattern $minimumFlowHelper -Description "minimum outdoor-air physics helper in CLI IdealLoads tree"
    }
    Assert-NotContains -Path $idealLoadsRuntime -Pattern $minimumFlowHelper -Description "minimum outdoor-air physics helper in compatibility runtime root"
}
foreach ($idealLoadsCliPhysicsFile in $idealLoadsCliPhysicsBoundaryFiles) {
    Assert-NotContains -Path $idealLoadsCliPhysicsFile -Pattern 'IdealLoadsSensibleResult' -Description "IdealLoads calculation result type in CLI IdealLoads tree"
}
Assert-Contains -Path $idealLoadsCli -Pattern 'meter_rate_to_energy_j\s*\(' -Description "IdealLoads report energy uses runtime rate-to-energy helper"
Assert-Contains -Path $calcMoistureDemand -Pattern 'pub fn third_order_humidity_history_term\s*\(' -Description "ThirdOrder humidity history runtime helper"
Assert-Contains -Path $calcMoistureDemandTests -Pattern 'fn third_order_humidity_history_term_matches_energyplus_coefficients\s*\(' -Description "ThirdOrder humidity history runtime test"
Assert-Contains -Path $idealLoadsCli -Pattern 'third_order_humidity_history_term\s*\(' -Description "IdealLoads residual diagnostic uses runtime ThirdOrder helper"
Assert-Contains -Path $idealLoadsCli -Pattern 'advance_no_oa_humidistat_zone_timestep_compat\s*\(' -Description "IdealLoads CLI uses runtime Humidistat zone-timestep state transition"
$cliForbiddenPhysicsPatterns = @(
    [pscustomobject]@{ Pattern = 'outdoor_air_design_mass_flow_rate_kg_per_s\s*\.max\s*\('; Description = "CO2Setpoint DCV physics in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'rate\([^\r\n]*\)\s*\*\s*interval_seconds'; Description = "IdealLoads rate-to-energy physics in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'fn third_order_humidity_history_term\s*\('; Description = "ThirdOrder humidity history helper in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = '3\.0\s*\*\s*history\[0\].*1\.5\s*\*\s*history\[1\]'; Description = "raw ThirdOrder humidity history coefficients in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'NoOaThirdOrderHumidityCorrectorInput|correct_no_oa_third_order_humidity_ratio_compat'; Description = "Humidistat corrector physics in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'zone_(air|mean)_humidity_history\s*=\s*\['; Description = "Humidistat history mutation in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'remaining_output_req_to_humid_sp_kg_per_s\s*=\s*predicted'; Description = "Humidistat predicted-demand injection in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'remaining_output_req_to_dehumid_sp_kg_per_s\s*=\s*predicted'; Description = "Humidistat predicted-demand injection in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'calc_no_oa_no_limit_sensible_compat'; Description = "direct no-limit no-OA calc helper call in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'calc_no_oa_sensible_with_limits_compat'; Description = "direct finite-limit no-OA calc helper call in CLI IdealLoads tree" },
    [pscustomobject]@{ Pattern = 'calc_outdoor_air_sensible_report_rates_compat'; Description = "direct outdoor-air calc helper call in CLI IdealLoads tree" }
)
foreach ($idealLoadsCliPhysicsFile in $idealLoadsCliPhysicsBoundaryFiles) {
    foreach ($forbiddenPhysics in $cliForbiddenPhysicsPatterns) {
        Assert-NotContains -Path $idealLoadsCliPhysicsFile -Pattern $forbiddenPhysics.Pattern -Description $forbiddenPhysics.Description
    }
}

# CP321 maps only the exact cooling-capacity-zero flow reset at
# PurchasedAirManager.cc lines 2147-2152.
$calcCoolingCapacityZeroFlowReset = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset.rs"
$calcCoolingCapacityZeroFlowResetState = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\state.rs"
$calcCoolingCapacityZeroFlowResetTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\transition.rs"
$calcCoolingCapacityZeroFlowResetRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\release.rs"
$calcCoolingCapacityZeroFlowResetPredecessorValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\release\predecessor_validation.rs"
$calcCoolingCapacityZeroFlowResetRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\release\runtime_validation.rs"
$calcCoolingCapacityZeroFlowResetSnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\release\snapshot_validation.rs"
$calcCoolingCapacityZeroFlowResetTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\tests\mod.rs"
$idealLoadsBindingCoolingCapacityZeroFlowResetTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_capacity_zero_flow_reset_tests.rs"
$idealLoadsCoupledCoolingCapacityZeroFlowResetValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_capacity_zero_flow_reset_validation.rs"
$runPurchasedAirCoolingCapacityZeroFlowReset = "crates\ep_run\src\pipeline\purchased_air_cooling_capacity_zero_flow_reset.rs"
$runPurchasedAirCoolingCapacityZeroFlowResetSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_capacity_zero_flow_reset\serialization.rs"
$runPurchasedAirCoolingCapacityZeroFlowResetSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_capacity_zero_flow_reset\serialization\snapshot.rs"
$runPurchasedAirCoolingCapacityZeroFlowResetValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_capacity_zero_flow_reset\validation.rs"
$runPurchasedAirCoolingCapacityZeroFlowResetSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_capacity_zero_flow_reset\validation\snapshot.rs"

foreach ($cp321RequiredFile in @(
        $calcCoolingCapacityZeroFlowReset,
        $calcCoolingCapacityZeroFlowResetState,
        $calcCoolingCapacityZeroFlowResetTransition,
        $calcCoolingCapacityZeroFlowResetRelease,
        $calcCoolingCapacityZeroFlowResetPredecessorValidation,
        $calcCoolingCapacityZeroFlowResetRuntimeValidation,
        $calcCoolingCapacityZeroFlowResetSnapshotValidation,
        $calcCoolingCapacityZeroFlowResetTests,
        $idealLoadsBindingCoolingCapacityZeroFlowResetTests,
        $idealLoadsCoupledCoolingCapacityZeroFlowResetValidation,
        $runPurchasedAirCoolingCapacityZeroFlowReset,
        $runPurchasedAirCoolingCapacityZeroFlowResetSerialization,
        $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotSerialization,
        $runPurchasedAirCoolingCapacityZeroFlowResetValidation,
        $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotValidation
    )) {
    Assert-FileExists -Path $cp321RequiredFile -Description "CP321 cooling capacity-zero flow-reset structure"
}

Assert-Contains -Path $calcRoot -Pattern 'mod cooling_capacity_zero_flow_reset;' -Description "CP321 calc submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_capacity_zero_flow_reset::\*;' -Description "CP321 calc public re-export"
Assert-Contains -Path $calcCoolingCapacityZeroFlowReset -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2147-2152' -Description "CP321 exact source boundary"
Assert-Contains -Path $calcCoolingCapacityZeroFlowReset -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2155' -Description "CP321 first excluded executable"
Assert-ExactStringArray -Path $calcCoolingCapacityZeroFlowReset -Name "PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER" -Expected @(
    "read-cooling-limit-for-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-total-cooling-capacity-after-limit-condition-true",
    "compare-maximum-total-cooling-capacity-equal-to-zero",
    "enter-zero-cooling-capacity-body-if-compound-condition-satisfied",
    "assign-supply-mass-flow-rate-for-cooling-zero",
    "assign-supply-mass-flow-rate-for-dehumidification-zero",
    "assign-supply-mass-flow-rate-for-humidification-zero"
) -Description "CP321 exact ten source-order sites"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetState -Pattern 'pub struct PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState' -Description "CP321 persistent public state"
Assert-Contains -Path $calcCoolingCapacityZeroFlowReset -Pattern 'pub struct PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary' -Description "CP321 public lifecycle summary"
Assert-Contains -Path $calcCoolingCapacityZeroFlowReset -Pattern 'pub fn purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary\s*\(' -Description "CP321 lifecycle accessor"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetRelease -Pattern 'pub fn advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset\s*\(' -Description "CP321 exact direct release wrapper"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_capacity_zero_flow_reset_state\s*\(' -Description "CP321 pure transition"

Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'first_limit\.map\(\|limit\| limit == IdealLoadsLimit::LimitCapacity\)' -Description "CP321 first exact Capacity comparison"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'if is_capacity == Some\(false\)' -Description "CP321 lazy second selector read"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'second_limit\.map\(\|limit\| limit == IdealLoadsLimit::LimitFlowRateAndCapacity\)' -Description "CP321 second exact FlowRateAndCapacity comparison"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'if limit_satisfied == Some\(true\)' -Description "CP321 lazy maximum-capacity read"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'maximum_capacity\.map\(\|capacity\| capacity == 0\.0\)' -Description "CP321 exact zero equality"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern '(?s)let assigned_cool = zero_body\.then_some\(0\.0_f64\);.*let assigned_dehumidification = zero_body\.then_some\(0\.0_f64\);.*let assigned_humidification = zero_body\.then_some\(0\.0_f64\);' -Description "CP321 ordered positive-zero assignment values"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'assigned_supply_mass_flow_rate_for_cool_kg_per_s: assigned_cool' -Description "CP321 cooling candidate zero assignment"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s: assigned_dehumidification' -Description "CP321 dehumidification candidate zero assignment"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'assigned_supply_mass_flow_rate_for_humidification_kg_per_s: assigned_humidification' -Description "CP321 humidification candidate zero assignment"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'assigned_(?:cool|dehumidification|humidification)\.unwrap_or\(prior\)' -Description "CP321 false-path candidate preservation"
Assert-NotContains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern 'cooling_capacity_limit_is_zero|<=\s*0\.0|0\.0\s*>=' -Description "broader-than-source capacity-zero predicate in CP321 transition"
Assert-NotContains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern '\.is_finite\(\)|\.(?:abs|clamp|min|max)\s*\(' -Description "CP321 normalization or clamp"
Assert-NotContains -Path $calcCoolingCapacityZeroFlowResetTransition -Pattern '\.then_some\s*\(\s*input\.' -Description "eager conditional CP321 input read"

$cp321TransitionText = Get-Content -LiteralPath $calcCoolingCapacityZeroFlowResetTransition -Raw
$cp321CoolAssignmentIndex = $cp321TransitionText.IndexOf("supply_mass_flow_rate_for_cool_zero_assignment_count += 1")
$cp321DehumidAssignmentIndex = $cp321TransitionText.IndexOf("supply_mass_flow_rate_for_dehumidification_zero_assignment_count += 1")
$cp321HumidAssignmentIndex = $cp321TransitionText.IndexOf("supply_mass_flow_rate_for_humidification_zero_assignment_count += 1")
if (
    $cp321CoolAssignmentIndex -lt 0 -or
    $cp321DehumidAssignmentIndex -le $cp321CoolAssignmentIndex -or
    $cp321HumidAssignmentIndex -le $cp321DehumidAssignmentIndex
) {
    throw "CP321 must retain cooling, dehumidification, then humidification positive-zero assignment order"
}

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_capacity_zero_flow_reset_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>' -Description "runtime-root private CP321 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_capacity_zero_flow_reset_latest_witnesses:' -Description "public runtime-root CP321 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_capacity_zero_flow_reset_latest_witness\s*\(' -Description "runtime-root CP321 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_capacity_zero_flow_reset_latest_witness\s*\(' -Description "runtime-root CP321 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_capacity_zero_flow_reset:\s*[\r\n]+\s*PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState' -Description "per-unit CP321 persistent state"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'pub\(in crate::ideal_loads::calc\) fn completed_direct_cooling_humidification_flow_is_consistent\s*\(' -Description "CP320 narrow completed-state export for CP321"
Assert-Contains -Path $calcCoolingHumidificationFlowRelease -Pattern 'mod completed_state_validation;' -Description "CP320 completed-state validator for CP321"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetPredecessorValidation -Pattern 'pub\(super\) fn predecessor_chain_and_candidates_are_consistent\s*\(' -Description "CP321 immediate predecessor chain validation"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetRelease -Pattern 'completed_direct_cooling_humidification_flow_is_consistent' -Description "CP321 consumes completed CP320 state"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetRelease -Pattern 'exact_direct_initialization_is_consistent' -Description "CP321 revalidates retained exact initialization before mutation"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetRelease -Pattern 'completed_direct_prefix_through_economizer_guard_is_consistent' -Description "CP321 revalidates the complete retained Calc prefix before mutation"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetRelease -Pattern '(?s)sized_limits != expected_sized_limits.*sizing_outcome.*outcome\.sized_limits != sized_limits' -Description "CP321 validates system, sizing overlay, and sizing outcome together"

Assert-Contains -Path $idealLoadsBindingCoolingCapacityZeroFlowResetTests -Pattern 'scheduled_binding_covers_all_limit_routes_and_zero_or_positive_capacity' -Description "CP321 binding limit-route and capacity regression"
Assert-Contains -Path $idealLoadsBindingCoolingCapacityZeroFlowResetTests -Pattern 'scheduled_binding_records_unit_off_and_non_cooling_skips' -Description "CP321 scheduled skip regression"
Assert-Contains -Path $idealLoadsBindingCoolingCapacityZeroFlowResetTests -Pattern 'public_release_replay_and_corrupt_state_fail_without_mutation' -Description "CP321 replay and corruption transaction regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'source_boundary_and_exact_ten_sites_are_stable' -Description "CP321 exact ten-site regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'unit_off_and_non_cooling_skip_every_site' -Description "CP321 complete skip regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'capacity_limit_short_circuits_second_read_and_assigns_three_positive_zeros' -Description "CP321 Capacity short-circuit and signed-zero equality regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'input\(IdealLoadsLimit::LimitCapacity, -0\.0\)' -Description "CP321 exact negative-zero equality regression input"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'combined_limit_repeats_read_before_capacity_comparison' -Description "CP321 FlowRateAndCapacity lazy-read regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'rejected_limit_short_circuits_poisoned_capacity_and_preserves_candidate_bits' -Description "CP321 rejected-selector bit preservation regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'public_release_rejects_post_init_cooling_limit_selector_mutation_transactionally' -Description "CP321 post-init CoolingLimit mutation regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'public_release_rejects_coordinated_system_and_sized_overlay_mutation_transactionally' -Description "CP321 coordinated sizing-overlay mutation regression"
Assert-Contains -Path $runPipeline -Pattern 'mixed_capacity_and_combined' -Description "CP321 pipeline mixed-selector history rejection regression"
Assert-Contains -Path $runPipeline -Pattern 'latest_selector_disagrees_with_cumulative' -Description "CP321 pipeline latest-selector history rejection regression"
Assert-Contains -Path $calcCoolingCapacityZeroFlowResetTests -Pattern 'every_nonzero_or_nonfinite_capacity_preserves_all_three_candidate_bits' -Description "CP321 nonzero and nonfinite false-path regression"

$cp321BindingText = Get-Content -LiteralPath "crates\ep_runtime\src\ideal_loads\binding.rs" -Raw
$cp320BindingIndex = $cp321BindingText.IndexOf("let calculation_cooling_humidification_flow =")
$cp321BindingIndex = $cp321BindingText.IndexOf("let calculation_cooling_capacity_zero_flow_reset =")
$numericalBindingIndex = $cp321BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp320BindingIndex -lt 0 -or
    $cp321BindingIndex -le $cp320BindingIndex -or
    $numericalBindingIndex -le $cp321BindingIndex
) {
    throw "Binding must retain exact CP320 -> CP321 -> numerical Calc order"
}
$betweenCp320AndCp321 = $cp321BindingText.Substring(
    $cp320BindingIndex,
    $cp321BindingIndex - $cp320BindingIndex
)
$betweenCp321AndNumerical = $cp321BindingText.Substring(
    $cp321BindingIndex,
    $numericalBindingIndex - $cp321BindingIndex
)
foreach ($cp321Intermediary in @(
        [pscustomobject]@{ Pattern = 'cooling_capacity_limit_is_zero'; Description = "broader capacity-zero helper" },
        [pscustomobject]@{ Pattern = '\.(?:abs|clamp|min|max)\s*\('; Description = "normalization or clamp" },
        [pscustomobject]@{ Pattern = '(?i)(?:ems|psychrometric|diagnostic|schedule_service|node_service)\s*\('; Description = "excluded live service" }
    )) {
    if ($betweenCp320AndCp321 -match $cp321Intermediary.Pattern) {
        throw "$($cp321Intermediary.Description) unexpectedly present between CP320 and CP321"
    }
    if ($betweenCp321AndNumerical -match $cp321Intermediary.Pattern) {
        throw "$($cp321Intermediary.Description) unexpectedly present between CP321 and numerical Calc"
    }
}

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_capacity_zero_flow_reset_validation;' -Description "coupled runtime CP321 validator declaration"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_capacity_zero_flow_reset_validation::snapshot_matches_release\(\s*output,\s*timestep_index \+ 1,\s*&binding,\s*\)' -Description "coupled runtime per-timestep CP321 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_capacity_zero_flow_reset_validation::validate_lifecycle\(\s*&calc_cooling_capacity_zero_flow_reset_lifecycle,\s*&calc_cooling_humidification_flow_lifecycle,' -Description "coupled runtime final CP321 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_capacity_zero_flow_reset_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary' -Description "coupled runtime CP321 lifecycle"
Assert-Contains -Path $idealLoadsCoupledCoolingCapacityZeroFlowResetValidation -Pattern 'cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release' -Description "coupled CP321 exact snapshot validator"

Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_capacity_zero_flow_reset;' -Description "pipeline CP321 evidence module declaration"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern 'mod serialization;' -Description "pipeline CP321 serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern 'mod validation;' -Description "pipeline CP321 validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline CP321 lifecycle serializer wiring"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern 'pub\(super\) fn validate_direct_lifecycle\s*\(' -Description "pipeline CP321 direct validator entry"
foreach ($cp321SourceOrderConstant in @(
        'PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER',
        'PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER',
        'PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER',
        'PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER'
    )) {
    Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern $cp321SourceOrderConstant -Description "pipeline CP318-CP321 SOURCE_ORDER lineage"
}
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern '(?s)fn latest_matches_release\s*\(.*cp320: &PurchasedAirCalcCoolingHumidificationFlowSnapshot,.*cp319: &PurchasedAirCalcCoolingDehumidificationFlowSnapshot,.*cp318: &PurchasedAirCalcCoolingSensibleFlowSnapshot,' -Description "pipeline CP321 retained CP318-CP321 snapshot lineage"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern '(?s)reset\.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,\s*cp318\.resulting_supply_mass_flow_rate_for_cool_kg_per_s' -Description "pipeline CP321 cooling candidate lineage"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern '(?s)reset\.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,\s*cp319\.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s' -Description "pipeline CP321 dehumidification candidate lineage"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowReset -Pattern '(?s)reset\.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,\s*cp320\.resulting_supply_mass_flow_rate_for_humidification_kg_per_s' -Description "pipeline CP321 humidification candidate lineage"

Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSerialization -Pattern 'mod snapshot;' -Description "pipeline CP321 snapshot serializer declaration"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSerialization -Pattern 'pub\(in crate::pipeline\) fn lifecycle_json\s*\(' -Description "pipeline CP321 lifecycle JSON serializer"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP321 latest snapshot serializer wiring"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotSerialization -Pattern 'pub\(super\) fn snapshot_json\s*\(' -Description "pipeline CP321 snapshot JSON serializer"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP321 SOURCE_ORDER JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotSerialization -Pattern '"resulting_supply_mass_flow_rate_for_cool_kg_per_s"' -Description "pipeline CP321 cooling-result JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotSerialization -Pattern '"resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s"' -Description "pipeline CP321 dehumidification-result JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotSerialization -Pattern '"resulting_supply_mass_flow_rate_for_humidification_kg_per_s"' -Description "pipeline CP321 humidification-result JSON evidence"

Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetValidation -Pattern 'mod snapshot;' -Description "pipeline CP321 snapshot validator declaration"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetValidation -Pattern 'pub\(super\) use snapshot::\{same_option, snapshot_shape\};' -Description "pipeline CP321 snapshot validator wiring"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetValidation -Pattern 'pub\(super\) fn validate_source_counters\s*\(' -Description "pipeline CP321 source-counter validator"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotValidation -Pattern 'pub\(in crate::pipeline\) fn snapshot_shape\s*\(' -Description "pipeline CP321 direct snapshot shape validator"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotValidation -Pattern 'limit == IdealLoadsLimit::LimitCapacity' -Description "pipeline CP321 Capacity selector validation"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotValidation -Pattern 'limit == IdealLoadsLimit::LimitFlowRateAndCapacity' -Description "pipeline CP321 FlowRateAndCapacity selector validation"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotValidation -Pattern 'let is_zero = capacity == 0\.0;' -Description "pipeline CP321 exact zero validation"
Assert-Contains -Path $runPurchasedAirCoolingCapacityZeroFlowResetSnapshotValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "pipeline CP321 bitwise candidate validation"

Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle' -Description "pipeline CP321 lifecycle JSON evidence"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_capacity_zero_flow_reset::validate_direct_lifecycle' -Description "pipeline CP321 direct-only firewall"
Assert-Contains -Path $runPipeline -Pattern '\.purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle\s*[\r\n]+\s*\.is_some\(\)' -Description "non-direct CP321 evidence rejection"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_capacity_zero_reset_rejects_malformed_evidence' -Description "pipeline CP321 malformed-evidence regression"
Assert-Contains -Path $runPipeline -Pattern 'direct_release_cooling_capacity_zero_reset_json_exposes_all_source_sites' -Description "pipeline CP321 all-site JSON regression"

Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern '"CP321 supersedes CP320' -Description "CP321 algorithm support-boundary addendum"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_capacity_zero_flow_reset\.rs::PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState' -Description "CP321 algorithm Rust state target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_capacity_zero_flow_reset\.rs::PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary' -Description "CP321 algorithm Rust lifecycle target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_capacity_zero_flow_reset/release\.rs::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset' -Description "CP321 algorithm Rust release target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_capacity_zero_flow_reset\.rs::purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary' -Description "CP321 algorithm lifecycle accessor target"
Assert-Contains -Path "specs\capabilities.toml" -Pattern '"CP321 additionally requires' -Description "CP321 capability claim-boundary addendum"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP321 supersedes CP320' -Description "generated CP321 algorithm boundary"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP321 additionally requires' -Description "generated CP321 capability boundary"
foreach ($cp321Doc in @(
        "docs\src\current\current-status.md",
        "docs\src\current\project-contract.md",
        "docs\src\porting-map\ideal-loads-source-map.md",
        "docs\src\porting-map\heat-balance-source-map.md",
        "docs\src\porting-map\zone-air-update-map.md"
    )) {
    Assert-Contains -Path $cp321Doc -Pattern 'CP321' -Description "CP321 documentation boundary"
    Assert-Contains -Path $cp321Doc -Pattern '2155' -Description "CP321 first excluded executable documentation"
}

. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp322-cooling-supply-mass-flow-maximum.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp323-cooling-supply-mass-flow-ems-override-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp324-cooling-supply-mass-flow-ems-override-body.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp325-cooling-supply-mass-flow-limit-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp326-cooling-supply-mass-flow-limit-body.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp327-cooling-supply-mass-flow-very-small-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp328-cooling-supply-mass-flow-very-small-guard-body.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp329-cooling-mixed-air-call.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp330-cooling-supply-mass-flow-positive-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp331-cooling-positive-supply-cp-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp332-cooling-positive-supply-temperature-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp333-cooling-positive-supply-temperature-minimum-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp349-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp350-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp351-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-total-output-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp352-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-enthalpy-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp353-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp354-cooling-constant-shr-supply-humidity-ratio-overdrying-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp355-cooling-constant-shr-supply-humidity-ratio-minimum-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp356-cooling-constant-shr-supply-humidity-ratio-mixed-air-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp357-cooling-constant-shr-case-break.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp358-cooling-humidistat-case-entry.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp359-cooling-humidistat-moisture-demand-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp360-cooling-humidistat-supply-humidity-ratio-for-dehumidification-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp361-cooling-humidistat-supply-humidity-ratio-for-dehumidification-minimum-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp363-cooling-humidistat-case-break.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp364-cooling-constant-supply-humidity-ratio-case-entry.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp365-cooling-constant-supply-humidity-ratio-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp366-cooling-constant-supply-humidity-ratio-case-break.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp367-cooling-default-supply-humidity-ratio-mixed-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp368-cooling-default-supply-humidity-ratio-case-break.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp369-cooling-supply-humidity-ratio-humidification-heating-availability-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp370-cooling-supply-humidity-ratio-humidification-control-humidistat-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp371-cooling-supply-humidity-ratio-humidification-dehumidification-control-humidistat-or-none-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp372-cooling-supply-humidity-ratio-humidification-moisture-demand-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp373-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp374-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-maximum-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp375-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-maximum-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp376-cooling-supply-humidity-ratio-pre-saturation-original-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp377-cooling-supply-humidity-ratio-saturation-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp378-cooling-supply-humidity-ratio-saturation-limit-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp379-cooling-supply-enthalpy-post-saturation-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp380-cooling-post-saturation-capacity-limit-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp381-cooling-post-saturation-capacity-limit-dehumidification-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp382-cooling-post-saturation-capacity-limit-dehumidification-total-output-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp383-cooling-post-saturation-capacity-limit-dehumidification-total-output-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp384-cooling-post-saturation-capacity-limit-dehumidification-total-output-maximum-capacity-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp385-cooling-post-saturation-capacity-limit-dehumidification-total-output-supply-enthalpy-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp386-cooling-post-saturation-capacity-limit-dehumidification-control-switch.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp387-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp393-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-break.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp394-cooling-post-saturation-capacity-limit-dehumidification-control-humidistat-case-entry.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp395-cooling-post-saturation-capacity-limit-dehumidification-control-humidistat-supply-humidity-ratio-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp396-cooling-post-saturation-capacity-limit-dehumidification-control-humidistat-case-break.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp397-cooling-post-saturation-capacity-limit-dehumidification-control-none-case-entry.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp398-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-case-entry.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp399-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-cp-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp400-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-sensible-output-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp401-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp402-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-guard.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp403-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp404-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-humidity-ratio-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp405-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-maximum-capacity-assignment.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp406-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-capacity-guard-else-branch-entry.ps1")
. (Join-Path $PSScriptRoot "ideal-loads-structure-audit\cp407-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-assignment.ps1")

Write-Host "IdealLoads structure audit complete."
