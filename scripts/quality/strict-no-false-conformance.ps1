[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-DoesNotContain {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for false-conformance guard: $Path"
    }

    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -ne $match) {
        $match | ForEach-Object { Write-Host "$($_.Path):$($_.LineNumber): $($_.Line)" }
        throw "Forbidden false-conformance wording found for $Description`: $Pattern"
    }
    Write-Host "OK no false-conformance wording for $Description"
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for false-conformance guard: $Path"
    }

    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -eq $match) {
        throw "Missing required compatibility boundary for $Description`: $Pattern"
    }
    Write-Host "OK compatibility boundary for $Description`: $Pattern"
}

function Assert-PathMissing {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (Test-Path -LiteralPath $Path) {
        throw "Forbidden retained documentation path for $Description`: $Path"
    }
    Write-Host "OK retained path absent for $Description`: $Path"
}

Assert-DoesNotContain -Path "README.md" -Pattern "first runtime path for an uncontrolled one-zone building subset" -Description "README scope"
Assert-DoesNotContain -Path "README.md" -Pattern "ResultStore output from the first uncontrolled one-zone simulation subset" -Description "README scope"
Assert-DoesNotContain -Path "README.md" -Pattern "zone temperature comparison passes" -Description "README scope"
Assert-DoesNotContain -Path "README.md" -Pattern "EnergyPlus simulation works" -Description "README scope"

Assert-DoesNotContain -Path "crates\ep_cli\src\main.rs" -Pattern "Zone Temperature Smoke Comparison" -Description "zone diagnostic CLI"
Assert-DoesNotContain -Path "crates\ep_cli\src\main.rs" -Pattern "exact_match: future" -Description "zone diagnostic CLI"
Assert-DoesNotContain -Path "scripts\compare\compare-zone-smoke.ps1" -Pattern "status: pass" -Description "zone diagnostic smoke"
Assert-DoesNotContain -Path "scripts\compare\compare-zone-smoke.ps1" -Pattern "exact_match: future" -Description "zone diagnostic smoke"
Assert-DoesNotContain -Path "scripts\compare\compare-regression.ps1" -Pattern "conformance-smoke" -Description "compare regression class names"

Assert-PathMissing -Path "docs\src\archive" -Description "mdBook archive tree"
Assert-Contains -Path "README.md" -Pattern "Old planning docs are not retained" -Description "retention policy"
Assert-Contains -Path "README.md" -Pattern "GitHub Release assets" -Description "release evidence retention"
Assert-Contains -Path "docs\src\adr\0001-docs-specs-and-evidence-retention.md" -Pattern "Old planning documents are not retained" -Description "ADR archive policy"
Assert-Contains -Path "docs\src\adr\0001-docs-specs-and-evidence-retention.md" -Pattern "uploaded to GitHub Releases as assets" -Description "ADR evidence asset policy"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.6"' -Description "v0.6 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "diagnostic-only"' -Description "diagnostic-only milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "first executable building simulation subset" -Description "v0.6 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.7"' -Description "v0.7 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "zone-temperature pass wording" -Description "v0.7 non-claim boundary"
Assert-Contains -Path "docs\src\porting-map\algorithm-porting-readiness.md" -Pattern "zone-temperature pass wording" -Description "v0.7 algorithm boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.8"' -Description "v0.8 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "heat_balance_nomass_001" -Description "v0.8 case boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "dynamic exterior heat-balance claim" -Description "v0.8 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.9"' -Description "v0.9 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "surface_temperature_nomass_001" -Description "v0.9 case boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "fenestration compatibility" -Description "v0.9 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "solar-radiation compatibility" -Description "v0.9 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.10"' -Description "v0.10 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "typed-graph-only" -Description "v0.10 graph-only boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "IdealLoads load conformance" -Description "v0.10 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.11"' -Description "v0.11 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "air_side_node_diagnostic_001" -Description "v0.11 case boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "node numerical conformance" -Description "v0.11 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.12"' -Description "v0.12 milestone boundary"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "planning guard" -Description "v0.12 source-map boundary"
Assert-Contains -Path "docs\src\porting-map\node-state-source-map.md" -Pattern "remain baseline-only or diagnostic-only evidence" -Description "v0.12 node non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.13"' -Description "v0.13 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "typed-graph-smoke" -Description "v0.13 graph smoke boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "plant loop simulation" -Description "v0.13 non-claim boundary"
Assert-Contains -Path "docs\src\porting-map\plant.md" -Pattern "typed graph smoke" -Description "v0.13 plant smoke boundary"
Assert-Contains -Path "docs\src\porting-map\plant.md" -Pattern "no plant loop simulation" -Description "v0.13 plant non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.14"' -Description "v0.14 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Plant Source Mapping" -Description "v0.14 planning boundary"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "planning guard" -Description "v0.14 source-map boundary"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "diagnostic-only evidence" -Description "v0.14 plant non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.15"' -Description "v0.15 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "plant_loop_diagnostic_001" -Description "v0.15 case boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "ExampleFiles numerical conformance" -Description "v0.15 non-claim boundary"
Assert-Contains -Path "docs\src\project-scope\versioning-reset-v2.md" -Pattern "v0.16 starts the Road to v1.0" -Description "v0.16 reset boundary"
Assert-Contains -Path "docs\src\project-scope\legacy-milestones.md" -Pattern "Historical Pre-Alpha Evidence Series" -Description "legacy milestone boundary"
Assert-Contains -Path "docs\src\project-scope\v1-scope.md" -Pattern "v1.0 must not claim" -Description "v1 non-full-compatibility boundary"
Assert-Contains -Path "docs\src\project-scope\v2-full-compatibility.md" -Pattern "Compatibility mode" -Description "v2 compatibility-mode boundary"
Assert-Contains -Path "docs\src\project-scope\v3-fast-successor.md" -Pattern "Fast-mode claims must not be mixed" -Description "v3 mode-specific boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.16"' -Description "v0.16 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Versioning and Evidence Cleanup" -Description "v0.16 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "new numerical conformance" -Description "v0.16 claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.17"' -Description "v0.17 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Case Manifest and Output Request Schema v2" -Description "v0.17 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "infrastructure-only"' -Description "v0.17 schema boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.18"' -Description "v0.18 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Output Request Injection and Oracle Baseline Pipeline" -Description "v0.18 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "baseline-only"' -Description "v0.18 baseline-only boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "official_1zone_uncontrolled_baseline_001" -Description "v0.18 official baseline case"
Assert-Contains -Path "specs\milestones.toml" -Pattern "ExampleFiles numerical conformance" -Description "v0.18 ExampleFiles non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.19"' -Description "v0.19 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Series Reader and Compare Engine v2" -Description "v0.19 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "comparison-infrastructure"' -Description "v0.19 comparison-infrastructure boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "meter conformance" -Description "v0.19 meter non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.20"' -Description "v0.20 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Conformance Report Generator" -Description "v0.20 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "reporting-infrastructure"' -Description "v0.20 reporting-infrastructure boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "new numerical conformance unless backed by generated evidence" -Description "v0.20 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.21"' -Description "v0.21 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Source Map and Algorithm Ledger v1" -Description "v0.21 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "planning-guard"' -Description "v0.21 planning-guard boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "algorithm completion without source map" -Description "v0.21 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.22"' -Description "v0.22 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Time, Weather, and Schedule Conformance Expansion" -Description "v0.22 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "declared-variables-only"' -Description "v0.22 declared-variable boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "general runtime compatibility" -Description "v0.22 non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.23"' -Description "v0.23 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Static Model Evidence Expansion" -Description "v0.23 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "static-evidence"' -Description "v0.23 static-evidence boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "dynamic heat-balance compatibility" -Description "v0.23 dynamic non-claim boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'version = "0.24"' -Description "v0.24 milestone boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "Runtime State and Output Registry Hardening" -Description "v0.24 active boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern 'claim_level = "runtime-infrastructure"' -Description "v0.24 runtime-infrastructure boundary"
Assert-Contains -Path "specs\milestones.toml" -Pattern "new numerical conformance" -Description "v0.24 numerical non-claim boundary"
Assert-Contains -Path "scripts\conformance\manifest-validate-all.ps1" -Pattern "Non-conformance case has conformance-level output" -Description "v0.17 false-claim schema gate"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.8 case class"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "conformance_claim = true" -Description "v0.8 manifest claim"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "max_abs = 0.000001" -Description "v0.8 tolerance"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "blocking = true" -Description "v0.8 blocking gate"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.9 case class"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern "conformance_claim = true" -Description "v0.9 manifest claim"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'variable_class = "surface-state"' -Description "v0.9 surface tolerance"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern "blocking = true" -Description "v0.9 blocking gate"
Assert-Contains -Path "data\conformance_cases\schedule_constant_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.22 schedule case class"
Assert-Contains -Path "data\conformance_cases\schedule_constant_001\case.toml" -Pattern "Schedule Value" -Description "v0.22 schedule variable"
Assert-Contains -Path "data\conformance_cases\schedule_constant_001\case.toml" -Pattern 'level = "conformance"' -Description "v0.22 schedule output level"
Assert-Contains -Path "data\conformance_cases\schedule_constant_001\case.toml" -Pattern "compare-schedule-conformance" -Description "v0.22 schedule blocking gate"
Assert-Contains -Path "data\conformance_cases\weather_fields_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.22 weather case class"
Assert-Contains -Path "data\conformance_cases\weather_fields_001\case.toml" -Pattern "Site Outdoor Air Drybulb Temperature" -Description "v0.22 weather conformance variable"
Assert-Contains -Path "data\conformance_cases\weather_fields_001\case.toml" -Pattern "remain diagnostic rows" -Description "v0.22 weather diagnostic boundary"
Assert-Contains -Path "data\conformance_cases\weather_fields_001\case.toml" -Pattern "compare-weather-conformance" -Description "v0.22 weather blocking gate"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern 'source_kind = "energy-plus-examplefile"' -Description "v0.23 official static source kind"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.23 static case class"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern "conformance_claim = true" -Description "v0.23 static conformance claim"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern "dynamic heat-balance, HVAC, plant, solar, fenestration, sizing, warmup, or meter conformance" -Description "v0.23 static non-claim boundary"
Assert-Contains -Path "data\conformance_cases\official_1zone_static_model_001\case.toml" -Pattern "compare-static-model-conformance" -Description "v0.23 static blocking gate"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern 'comparison_class = "smoke"' -Description "v0.10 case class"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.10 manifest claim boundary"
Assert-Contains -Path "data\conformance_cases\ideal_loads_thermostat_001\case.toml" -Pattern 'class = "hvac-state"' -Description "v0.10 HVAC output class"
Assert-Contains -Path "scripts\smoke\ideal-loads-thermostat-smoke.ps1" -Pattern "status: baseline-only" -Description "v0.10 smoke gate baseline-only status"
Assert-Contains -Path "scripts\smoke\ideal-loads-thermostat-smoke.ps1" -Pattern "baseline_nonzero_count" -Description "v0.10 nonzero baseline gate"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern 'comparison_class = "diagnostic-only"' -Description "v0.11 case class"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.11 manifest claim boundary"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern 'class = "node-state"' -Description "v0.11 node output class"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "status: baseline-only" -Description "v0.11 smoke gate baseline-only status"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "baseline_nonzero_count" -Description "v0.11 nonzero node baseline gate"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "algorithm_parity: false" -Description "v0.11 node projection algorithm boundary"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "state_nodes: 3" -Description "v0.11 node projection state count"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "status: projected" -Description "v0.11 node projection status"
Assert-Contains -Path "data\testcases\minimal\plant-loop-skeleton.epJSON" -Pattern '"PlantLoop"' -Description "v0.13 plant fixture"
Assert-Contains -Path "scripts\smoke\plant-loop-skeleton-smoke.ps1" -Pattern "Plant-loop skeleton smoke passed." -Description "v0.13 plant smoke gate"
Assert-Contains -Path "crates\ep_compiler\src\compiler.rs" -Pattern "parse_plant_loops" -Description "v0.13 PlantLoop parser"
Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "PlantLoopBranchListEdge" -Description "v0.13 PlantLoop graph edge"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "ManagePlantLoops" -Description "v0.14 plant manager source map"
Assert-Contains -Path "docs\src\porting-map\plant-source-map.md" -Pattern "SetComponentFlowRate" -Description "v0.14 plant flow source map"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'comparison_class = "diagnostic-only"' -Description "v0.15 case class"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.15 manifest claim boundary"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'class = "plant-state"' -Description "v0.15 plant-state output class"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'class = "plant-equipment"' -Description "v0.15 plant-equipment output class"
Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "status: baseline-only" -Description "v0.15 smoke gate baseline-only status"
Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "baseline_nonzero_count" -Description "v0.15 nonzero plant baseline gate"
Assert-Contains -Path "scripts\smoke\plant-loop-diagnostic-smoke.ps1" -Pattern "tolerance_policy: none" -Description "v0.15 tolerance boundary"
Assert-Contains -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Pattern "comparison_class: diagnostic-only" -Description "v0.16 projection diagnostic class"
Assert-Contains -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Pattern "conformance_claim: false" -Description "v0.16 projection claim boundary"
Assert-Contains -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Pattern "algorithm_parity: false" -Description "v0.16 projection algorithm boundary"
Assert-Contains -Path "scripts\smoke\plant-loop-projection-smoke.ps1" -Pattern "tolerance_policy: none" -Description "v0.16 projection tolerance boundary"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern 'level = "conformance"' -Description "v0.17 v0.8 conformance-level output"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'level = "conformance"' -Description "v0.17 v0.9 conformance-level output"
Assert-Contains -Path "data\conformance_cases\plant_loop_diagnostic_001\case.toml" -Pattern 'level = "baseline"' -Description "v0.17 plant non-conformance output level"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern 'source_kind = "energy-plus-examplefile"' -Description "v0.18 official source kind"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern 'comparison_class = "smoke"' -Description "v0.18 case class"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.18 manifest claim boundary"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml" -Pattern 'level = "baseline"' -Description "v0.18 baseline output level"
Assert-Contains -Path "crates\ep_cli\src\conformance_artifacts.rs" -Pattern "stage_idf_with_output_requests" -Description "v0.18 output injection staging"
Assert-Contains -Path "crates\ep_cli\src\conformance_artifacts.rs" -Pattern "rusted-energyplus.output-injection.v1" -Description "v0.18 output injection manifest schema"
Assert-Contains -Path "scripts\conformance\official-baseline-smoke.ps1" -Pattern "status: baseline-only" -Description "v0.18 report baseline-only status"
Assert-Contains -Path "scripts\conformance\official-baseline-smoke.ps1" -Pattern "conformance_claim: false" -Description "v0.18 smoke claim boundary"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "SeriesComparisonV2" -Description "v0.19 v2 comparison summary"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "SeriesAlignment::Timestamp" -Description "v0.19 timestamp alignment"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "rmse_delta" -Description "v0.19 RMSE metric"
Assert-Contains -Path "crates\ep_compare\src\series.rs" -Pattern "max_rel_delta" -Description "v0.19 relative delta metric"
Assert-Contains -Path "crates\ep_compare\src\eso.rs" -Pattern "parse_eso_time_series" -Description "v0.19 timestamp-aware ESO parser"
Assert-Contains -Path "crates\ep_compare\src\eso.rs" -Pattern "EsoTimeSeries" -Description "v0.19 ESO time-series type"
Assert-Contains -Path "scripts\compare\compare-series-v2-smoke.ps1" -Pattern "Series v2 smoke passed." -Description "v0.19 smoke gate"
Assert-Contains -Path "tools\reporting\conformance_index_report.py" -Pattern "build_conformance_index" -Description "v0.20 conformance index builder"
Assert-Contains -Path "tools\reporting\conformance_index_report.py" -Pattern "coverage_matrix" -Description "v0.20 coverage matrix"
Assert-Contains -Path "scripts\release\conformance-index-report.ps1" -Pattern "conformance_index_report.py" -Description "v0.20 report wrapper"
Assert-Contains -Path "docs\src\conformance\report-format.md" -Pattern "conformance-index-report.pdf" -Description "v0.20 report format artifact"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern "source_map =" -Description "v0.21 algorithm source-map field"
Assert-Contains -Path "tools\docs\validate_algorithm_ledger.py" -Pattern "No source map, no algorithm port." -Description "v0.21 source-map rule"
Assert-Contains -Path "scripts\quality\algorithm-ledger-check.ps1" -Pattern "validate_algorithm_ledger.py" -Description "v0.21 algorithm ledger gate"
Assert-Contains -Path "crates\ep_cli\src\time_weather_schedule.rs" -Pattern "timestamp_rule" -Description "v0.22 timestamp report boundary"
Assert-Contains -Path "crates\ep_cli\src\time_weather_schedule.rs" -Pattern "max_rmse_tolerance" -Description "v0.22 RMSE gate boundary"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern "compare-schedule-conformance" -Description "v0.22 evidence schedule gate"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern "compare-weather-conformance" -Description "v0.22 evidence weather gate"
Assert-Contains -Path "crates\ep_cli\src\static_model.rs" -Pattern "static EIO model evidence only" -Description "v0.23 static report claim boundary"
Assert-Contains -Path "crates\ep_cli\src\static_model.rs" -Pattern "surface_details_injected" -Description "v0.23 static surface detail marker"
Assert-Contains -Path "crates\ep_cli\src\conformance_artifacts.rs" -Pattern "Output:Surfaces:List,Details" -Description "v0.23 static surface detail injection"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "RuntimeOutputRegistry" -Description "v0.24 runtime output registry"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "RuntimeMeterRegistry" -Description "v0.24 runtime meter registry"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "OutputVariableUnavailable" -Description "v0.24 unavailable output diagnostic"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "MeterUnavailable" -Description "v0.24 unavailable meter diagnostic"
Assert-Contains -Path "crates\ep_runtime\src\output.rs" -Pattern "ResultStoreProfile" -Description "v0.24 result profile scaffold"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "RuntimeOutputRegistry::from_model" -Description "v0.24 execution-plan output registry"

Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "comparison_class: diagnostic-only" -Description "zone diagnostic CLI"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "conformance_claim: false" -Description "diagnostic CLI"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "Conformance Heat Balance Report" -Description "v0.8 conformance CLI"
Assert-Contains -Path "scripts\compare\compare-zone-smoke.ps1" -Pattern "status: extracted" -Description "zone diagnostic smoke"
Assert-Contains -Path "scripts\compare\compare-heat-balance-conformance.ps1" -Pattern "status: pass" -Description "v0.8 conformance gate"
Assert-Contains -Path "scripts\compare\compare-surface-temperature-conformance.ps1" -Pattern "status: pass" -Description "v0.9 conformance gate"
Assert-Contains -Path "scripts\compare\compare-schedule-conformance.ps1" -Pattern "Schedule conformance gate passed." -Description "v0.22 schedule conformance gate"
Assert-Contains -Path "scripts\compare\compare-weather-conformance.ps1" -Pattern "Weather conformance gate passed." -Description "v0.22 weather conformance gate"
Assert-Contains -Path "scripts\compare\compare-static-model-conformance.ps1" -Pattern "Static model conformance gate passed." -Description "v0.23 static conformance gate"
Assert-Contains -Path "scripts\smoke\runtime-registry-smoke.ps1" -Pattern "Runtime registry smoke passed." -Description "v0.24 runtime registry smoke gate"
Assert-Contains -Path "docs\src\releases\v0.24.0.md" -Pattern "runtime-infrastructure only" -Description "v0.24 release claim boundary"
Assert-Contains -Path "docs\src\operations\full-compatibility-reset.md" -Pattern "No conformance claim without case + variable list + tolerance + report + gate." -Description "reset policy"
Assert-Contains -Path "docs\src\architecture\performance-stability-core-porting-philosophy.md" -Pattern "default = SimulationMode::Compatibility" -Description "compatibility mode default"
Assert-Contains -Path "docs\src\architecture\performance-stability-core-porting-philosophy.md" -Pattern "No source map, no algorithm port." -Description "source-map algorithm boundary"
Assert-Contains -Path "docs\src\porting-map\algorithm-ledger.md" -Pattern "No source map, no algorithm port." -Description "algorithm ledger boundary"
Assert-Contains -Path "docs\src\operations\documentation-framework.md" -Pattern "PowerShell entry points are orchestration wrappers." -Description "documentation wrapper boundary"
Assert-Contains -Path "docs\src\operations\documentation-framework.md" -Pattern "oodocs" -Description "oodocs documentation framework"
Assert-Contains -Path "tools\reporting\README.md" -Pattern "structured HTML/PDF output" -Description "reporting generator policy"

Write-Host "False-conformance guard passed."
