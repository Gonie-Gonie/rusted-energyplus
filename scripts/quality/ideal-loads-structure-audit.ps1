[CmdletBinding()]
param()

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
$calcCoolingEconomizerConditionInitializationValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\initialization_validation.rs"
$calcCoolingEconomizerConditionPredecessorValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\predecessor_validation.rs"
$calcCoolingEconomizerConditionRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition\release\runtime_validation.rs"
$calcCoolingEconomizerConditionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_tests.rs"
$calcCoolingEconomizerConditionReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_release_tests.rs"
$calcCoolingEconomizerConditionReleaseProvenanceTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_release_tests\provenance_tests.rs"
$calcCoolingEconomizerConditionReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_economizer_condition_release_tests\corruption_tests.rs"
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
$idealLoadsCoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$idealLoadsCoupledMinimumOaValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\minimum_oa_validation.rs"
$idealLoadsCoupledCoolingEntryValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_entry_validation.rs"
$idealLoadsCoupledCoolingOaMaxFlowValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_oa_max_flow_validation.rs"
$idealLoadsCoupledCoolingOaMaxFlowBodyValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_oa_max_flow_body_validation.rs"
$idealLoadsCoupledCoolingEconomizerGuardValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_economizer_guard_validation.rs"
$idealLoadsCoupledCoolingEconomizerConditionValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_economizer_condition_validation.rs"
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
Assert-FileExists -Path $calcCoolingEconomizerConditionInitializationValidation -Description "PurchasedAir Calc cooling economizer condition retained initialization validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionPredecessorValidation -Description "PurchasedAir Calc cooling economizer condition predecessor validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionRuntimeValidation -Description "PurchasedAir Calc cooling economizer condition runtime validation"
Assert-FileExists -Path $calcCoolingEconomizerConditionTests -Description "PurchasedAir Calc cooling economizer condition characterization tests"
Assert-FileExists -Path $calcCoolingEconomizerConditionReleaseTests -Description "PurchasedAir Calc cooling economizer condition public release tests"
Assert-FileExists -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Description "PurchasedAir Calc cooling economizer condition provenance tests"
Assert-FileExists -Path $calcCoolingEconomizerConditionReleaseCorruptionTests -Description "PurchasedAir Calc cooling economizer condition corruption tests"
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
Assert-FileExists -Path $idealLoadsCoupledRuntime -Description "IdealLoads coupled release runtime"
Assert-FileExists -Path $idealLoadsCoupledMinimumOaValidation -Description "IdealLoads minimum-OA release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingEntryValidation -Description "IdealLoads cooling-entry release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Description "IdealLoads cooling OA maximum-flow release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Description "IdealLoads cooling OA maximum-flow true-body release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Description "IdealLoads cooling economizer guard release validator"
Assert-FileExists -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Description "IdealLoads cooling economizer condition release validator"
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

Assert-LineLimit -Path $calcRoot -Limit 80 -Description "IdealLoads calc module root"
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
Assert-LineLimit -Path $calcCoolingEconomizerConditionInitializationValidation -Limit 120 -Description "PurchasedAir Calc cooling economizer condition retained initialization validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionPredecessorValidation -Limit 340 -Description "PurchasedAir Calc cooling economizer condition predecessor validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionRuntimeValidation -Limit 480 -Description "PurchasedAir Calc cooling economizer condition runtime validation"
Assert-LineLimit -Path $calcCoolingEconomizerConditionTests -Limit 340 -Description "PurchasedAir Calc cooling economizer condition characterization tests"
Assert-LineLimit -Path $calcCoolingEconomizerConditionReleaseTests -Limit 340 -Description "PurchasedAir Calc cooling economizer condition public release tests"
Assert-LineLimit -Path $calcCoolingEconomizerConditionReleaseProvenanceTests -Limit 160 -Description "PurchasedAir Calc cooling economizer condition provenance tests"
Assert-LineLimit -Path $calcCoolingEconomizerConditionReleaseCorruptionTests -Limit 260 -Description "PurchasedAir Calc cooling economizer condition corruption tests"
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
Assert-LineLimit -Path $idealLoadsInitState -Limit 260 -Description "IdealLoads persistent initialization state"
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
Assert-LineLimit -Path $idealLoadsCoupledMinimumOaValidation -Limit 240 -Description "IdealLoads minimum-OA release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingEntryValidation -Limit 240 -Description "IdealLoads cooling-entry release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingOaMaxFlowValidation -Limit 280 -Description "IdealLoads cooling OA maximum-flow release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingOaMaxFlowBodyValidation -Limit 280 -Description "IdealLoads cooling OA maximum-flow true-body release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingEconomizerGuardValidation -Limit 240 -Description "IdealLoads cooling economizer guard release validator"
Assert-LineLimit -Path $idealLoadsCoupledCoolingEconomizerConditionValidation -Limit 280 -Description "IdealLoads cooling economizer condition release validator"
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

Assert-Contains -Path $calcRoot -Pattern 'mod humidity;' -Description "IdealLoads calc humidity submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_entry_gate;' -Description "PurchasedAir Calc cooling-entry gate submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_entry_gate_tests;' -Description "PurchasedAir Calc cooling-entry gate test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_entry_gate::\*;' -Description "PurchasedAir Calc cooling-entry gate public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_entry_gate\s*\(' -Description "cooling-entry transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_gate;' -Description "PurchasedAir Calc cooling OA maximum-flow gate submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_gate_tests;' -Description "PurchasedAir Calc cooling OA maximum-flow gate test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_oa_max_flow_gate::\*;' -Description "PurchasedAir Calc cooling OA maximum-flow gate public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_cooling_oa_max_flow_gate\s*\(' -Description "cooling OA maximum-flow transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_body;' -Description "PurchasedAir Calc cooling OA maximum-flow true-body submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod cooling_oa_max_flow_body_tests;' -Description "PurchasedAir Calc cooling OA maximum-flow true-body test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_oa_max_flow_body::\*;' -Description "PurchasedAir Calc cooling OA maximum-flow true-body public re-export"
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
Assert-Contains -Path $calcRoot -Pattern 'mod lifecycle;' -Description "PurchasedAir Calc-entry lifecycle submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod lifecycle_tests;' -Description "PurchasedAir Calc-entry lifecycle test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use lifecycle::\*;' -Description "PurchasedAir Calc-entry lifecycle public re-export"
Assert-Contains -Path $calcRoot -Pattern 'mod minimum_oa_prefix;' -Description "PurchasedAir Calc minimum-OA prefix submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod minimum_oa_prefix_tests;' -Description "PurchasedAir Calc minimum-OA prefix test module declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use minimum_oa_prefix::\*;' -Description "PurchasedAir Calc minimum-OA prefix public re-export"
Assert-NotContains -Path $calcRoot -Pattern 'pub fn advance_direct_no_oa_calc_minimum_oa_prefix\s*\(' -Description "minimum-OA transition implementation in calc module root"
Assert-Contains -Path $calcRoot -Pattern 'mod limits;' -Description "IdealLoads calc limits submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod mass_flow;' -Description "IdealLoads calc mass-flow submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod no_oa;' -Description "no-OA calc submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod psychrometrics;' -Description "IdealLoads calc psychrometrics submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'mod types;' -Description "IdealLoads calc shared types submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use limits::IdealLoadsSensibleLimitContext;' -Description "IdealLoads calc limits public re-export"
Assert-Contains -Path $calcRoot -Pattern 'pub use no_oa::\*;' -Description "no-OA calc public re-export"
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
Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)#\[derive\(Clone, Debug, Default, PartialEq\)\]\s*pub struct PurchasedAirRuntimeState\s*\{.*?cooling_economizer_condition_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingEconomizerConditionSnapshot>,\s*\}' -Description "runtime-root default-empty per-system CP316 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_economizer_condition_latest_witnesses:' -Description "public runtime-root CP316 witness map"
Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)pub\(in crate::ideal_loads\) fn cooling_economizer_condition_latest_witness\s*\(\s*&self,\s*system:\s*IdealLoadsAirSystemId,\s*\)\s*->\s*Option<PurchasedAirCalcCoolingEconomizerConditionSnapshot>\s*\{\s*self\.cooling_economizer_condition_latest_witnesses\s*\.get\(&system\)\s*\.copied\(\)\s*\}' -Description "ideal_loads-scoped runtime-root CP316 witness getter"
Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)pub\(in crate::ideal_loads\) fn set_cooling_economizer_condition_latest_witness\s*\(\s*&mut self,\s*system:\s*IdealLoadsAirSystemId,\s*snapshot:\s*PurchasedAirCalcCoolingEconomizerConditionSnapshot,\s*\)\s*\{\s*self\.cooling_economizer_condition_latest_witnesses\s*\.insert\(system, snapshot\);\s*\}' -Description "ideal_loads-scoped runtime-root CP316 witness setter"
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
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'pub calculation_cooling_economizer_guard: PurchasedAirCalcCoolingEconomizerGuardSnapshot' -Description "Calc cooling economizer scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_condition_tests\.rs"\]' -Description "Calc cooling economizer condition binding test module path"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_economizer_condition_integrity_tests\.rs"\]' -Description "Calc cooling economizer condition retained-state integrity test module path"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionTests -Pattern 'scheduled_binding_orders_cooling_economizer_condition_after_cp315_before_numerical_calc' -Description "Calc cooling economizer condition scheduled binding order regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionTests -Pattern 'public_cooling_economizer_condition_rejects_forgery_replay_and_overflow_without_mutation' -Description "Calc cooling economizer condition forgery, replay, and overflow transaction regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionTests -Pattern 'public_cooling_economizer_condition_rejects_economizer_configuration_without_mutation' -Description "Calc cooling economizer condition non-release enum rejection regression"
Assert-Contains -Path $idealLoadsBindingCoolingEconomizerConditionIntegrityTests -Pattern 'public_condition_rejects_retained_identity_and_route_forgery_without_mutation' -Description "Calc cooling economizer condition retained identity and route forgery regression"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'CalculationCoolingEconomizerCondition\(PurchasedAirCalcCoolingEconomizerConditionError\)' -Description "Calc cooling economizer condition scheduled binding error boundary"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'pub calculation_cooling_economizer_condition:\s*[\r\n]+\s*PurchasedAirCalcCoolingEconomizerConditionSnapshot' -Description "Calc cooling economizer condition scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'advance_direct_no_oa_calc_cooling_economizer_condition\s*\(' -Description "Calc cooling economizer condition scheduled release call"
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
$bindingCalcIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
$bindingCoolingEconomizerConditionCall = [regex]::Match(
    $bindingText,
    '(?s)let calculation_cooling_economizer_condition =\s*advance_direct_no_oa_calc_cooling_economizer_condition\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_economizer_guard,\s*\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingEconomizerCondition,\s*\)\?;'
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
    $bindingCalcIndex -le $bindingCoolingEconomizerConditionIndex
) {
    throw "InitPurchasedAir must precede the Calc-entry prefix, minimum-OA prefix, cooling-entry gate, cooling OA maximum-flow gate, cooling OA maximum-flow true body, cooling economizer guard, cooling economizer condition, and bounded numerical Calc coupling"
}
if (-not $bindingCoolingEconomizerConditionCall.Success) {
    throw "CP316 binding must call the exact no-node release wrapper with only runtime, system, and CP315 predecessor"
}
$bindingCoolingEconomizerConditionCallEnd =
    $bindingCoolingEconomizerConditionCall.Index + $bindingCoolingEconomizerConditionCall.Length
if ($bindingCalcIndex -le $bindingCoolingEconomizerConditionCallEnd) {
    throw "CP316 exact release call must complete before the bounded numerical Calc coupling"
}
$bindingCoolingEconomizerConditionToCalcWindow = $bindingText.Substring(
    $bindingCoolingEconomizerConditionCall.Index,
    $bindingCalcIndex - $bindingCoolingEconomizerConditionCall.Index
)
foreach ($forbiddenBehavior in $coolingEconomizerConditionForbiddenBehaviorPatterns) {
    if ($bindingCoolingEconomizerConditionToCalcWindow -match $forbiddenBehavior.Pattern) {
        throw "$($forbiddenBehavior.Description) unexpectedly present between CP316 and bounded numerical Calc coupling"
    }
}
$bindingPostCoolingEconomizerConditionWindow = $bindingText.Substring(
    $bindingCoolingEconomizerConditionCallEnd,
    $bindingCalcIndex - $bindingCoolingEconomizerConditionCallEnd
)
if ($bindingPostCoolingEconomizerConditionWindow -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP316 and before the bounded numerical Calc coupling"
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

Write-Host "IdealLoads structure audit complete."
