[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-outdoor-air-enthalpy-heat-recovery-conformance\26.1.0"
$CaseId = "ideal_loads_outdoor_air_enthalpy_heat_recovery_conformance_candidate_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside repository: $full"
    }
}

function Remove-RepoDirectory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (Test-Path -LiteralPath $Path) {
        Assert-RepoSubPath -Path $Path
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        Write-Host $Text
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required IdealLoads outdoor-air Enthalpy heat-recovery conformance input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads outdoor-air Enthalpy heat-recovery conformance candidate comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-outdoor-air-design-flow-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads outdoor-air design-flow Enthalpy heat-recovery conformance candidate comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads Outdoor-Air Design-Flow Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 22" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 96" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_failures_count: 0" -Description "tolerance failures"
Assert-Contains -Text $text -Pattern "tolerance_policy: conformance-gate" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "conformance status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$selectedOutputsPath = Join-Path $CompareRoot "selected_outputs.json"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$variableDeltasPath = Join-Path $CompareRoot "variable-deltas.csv"
$firstDivergencePath = Join-Path $CompareRoot "first-divergence.csv"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads outdoor-air Enthalpy heat-recovery conformance stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "IdealLoads outdoor-air conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads outdoor-air conformance status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads outdoor-air conformance should have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.series_count -ne 22) {
    throw "Unexpected IdealLoads outdoor-air series count: $($summary.series_count)"
}
if ($summary.samples -ne 96) {
    throw "Unexpected IdealLoads outdoor-air sample count: $($summary.samples)"
}
if ($summary.design_volume_flow_rate_m3_per_s -ne 0.05) {
    throw "Unexpected outdoor-air design volume flow: $($summary.design_volume_flow_rate_m3_per_s)"
}

$rows = @($summary.series)
if ($rows.Count -ne 22) {
    throw "Expected twenty-two outdoor-air rows, found $($rows.Count)"
}
$conformanceRows = @($rows | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 21) {
    throw "Expected twenty-one outdoor-air Enthalpy heat-recovery conformance rows, found $($conformanceRows.Count)"
}
if (@($conformanceRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All outdoor-air Enthalpy heat-recovery conformance rows must pass"
}
$diagnosticRows = @($rows | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 1) {
    throw "Expected one inactive economizer diagnostic row, found $($diagnosticRows.Count)"
}
if ($diagnosticRows[0].variable -ne "Zone Ideal Loads Economizer Active Time" -or $diagnosticRows[0].status -ne "pass") {
    throw "The only diagnostic row must be the passing inactive economizer active-time row"
}

$heatRecoverySource = "rust-ideal-loads-outdoor-air-enthalpy-heat-recovery"
$expectedRows = @(
    @{ Variable = "Zone Ideal Loads Outdoor Air Mass Flow Rate"; Level = "conformance"; Units = "kg/s"; Source = "rust-ideal-loads-outdoor-air-design-flow"; MaxAbs = 0.000000001; Rmse = 0.000000001 },
    @{ Variable = "Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate"; Level = "conformance"; Units = "m3/s"; Source = "rust-ideal-loads-outdoor-air-design-flow"; MaxAbs = 0.000000001; Rmse = 0.000000001 },
    @{ Variable = "Zone Ideal Loads Outdoor Air Sensible Heating Rate"; Level = "conformance"; Units = "W"; Source = "rust-ideal-loads-outdoor-air-sensible-report"; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Outdoor Air Sensible Cooling Rate"; Level = "conformance"; Units = "W"; Source = "rust-ideal-loads-outdoor-air-sensible-report"; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Outdoor Air Latent Heating Rate"; Level = "conformance"; Units = "W"; Source = "rust-ideal-loads-outdoor-air-latent-report"; MaxAbs = 0.000000001; Rmse = 0.000000001 },
    @{ Variable = "Zone Ideal Loads Outdoor Air Latent Cooling Rate"; Level = "conformance"; Units = "W"; Source = "rust-ideal-loads-outdoor-air-latent-report"; MaxAbs = 0.000000001; Rmse = 0.000000001 },
    @{ Variable = "Zone Ideal Loads Outdoor Air Total Heating Rate"; Level = "conformance"; Units = "W"; Source = "rust-ideal-loads-outdoor-air-total-report"; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Outdoor Air Total Cooling Rate"; Level = "conformance"; Units = "W"; Source = "rust-ideal-loads-outdoor-air-total-report"; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Supply Air Mass Flow Rate"; Level = "conformance"; Units = "kg/s"; Source = "rust-ideal-loads-outdoor-air-supply-state"; MaxAbs = 0.000001; Rmse = 0.000001 },
    @{ Variable = "Zone Ideal Loads Supply Air Standard Density Volume Flow Rate"; Level = "conformance"; Units = "m3/s"; Source = "rust-ideal-loads-outdoor-air-supply-state"; MaxAbs = 0.000001; Rmse = 0.000001 },
    @{ Variable = "Zone Ideal Loads Supply Air Temperature"; Level = "conformance"; Units = "C"; Source = "rust-ideal-loads-outdoor-air-supply-state"; MaxAbs = 0.02; Rmse = 0.02 },
    @{ Variable = "Zone Ideal Loads Supply Air Humidity Ratio"; Level = "conformance"; Units = "kgWater/kgDryAir"; Source = "rust-ideal-loads-outdoor-air-supply-state"; MaxAbs = 0.00005; Rmse = 0.00001 },
    @{ Variable = "Zone Ideal Loads Mixed Air Temperature"; Level = "conformance"; Units = "C"; Source = "rust-ideal-loads-outdoor-air-mixed-air"; MaxAbs = 0.02; Rmse = 0.02 },
    @{ Variable = "Zone Ideal Loads Mixed Air Humidity Ratio"; Level = "conformance"; Units = "kgWater/kgDryAir"; Source = "rust-ideal-loads-outdoor-air-mixed-air"; MaxAbs = 0.00005; Rmse = 0.00001 },
    @{ Variable = "Zone Ideal Loads Heat Recovery Sensible Heating Rate"; Level = "conformance"; Units = "W"; Source = $heatRecoverySource; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Heat Recovery Latent Heating Rate"; Level = "conformance"; Units = "W"; Source = $heatRecoverySource; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Heat Recovery Total Heating Rate"; Level = "conformance"; Units = "W"; Source = $heatRecoverySource; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Heat Recovery Sensible Cooling Rate"; Level = "conformance"; Units = "W"; Source = $heatRecoverySource; MaxAbs = 1.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Heat Recovery Latent Cooling Rate"; Level = "conformance"; Units = "W"; Source = $heatRecoverySource; MaxAbs = 6.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Heat Recovery Total Cooling Rate"; Level = "conformance"; Units = "W"; Source = $heatRecoverySource; MaxAbs = 6.0; Rmse = 1.0 },
    @{ Variable = "Zone Ideal Loads Economizer Active Time"; Level = "diagnostic"; Units = "hr"; Source = "rust-ideal-loads-outdoor-air-inactive-economizer"; MaxAbs = 0.000000001; Rmse = 0.000000001 },
    @{ Variable = "Zone Ideal Loads Heat Recovery Active Time"; Level = "conformance"; Units = "hr"; Source = $heatRecoverySource; MaxAbs = 0.000000001; Rmse = 0.000000001 }
)

foreach ($expected in $expectedRows) {
    $row = @($rows | Where-Object { $_.variable -eq $expected.Variable })
    if ($row.Count -ne 1) {
        throw "Expected one row for $($expected.Variable), found $($row.Count)"
    }
    if ($row[0].level -ne $expected.Level) {
        throw "Unexpected level for $($expected.Variable): $($row[0].level)"
    }
    if ($row[0].units -ne $expected.Units -or $row[0].rust_source -ne $expected.Source) {
        throw "Unexpected row metadata for $($expected.Variable): units=$($row[0].units) rust_source=$($row[0].rust_source)"
    }
    if ($row[0].max_abs_delta -gt [double]$expected.MaxAbs -or $row[0].rmse_delta -gt [double]$expected.Rmse) {
        throw "Outdoor-air Enthalpy heat-recovery row exceeded tolerance: variable=$($expected.Variable) max_abs=$($row[0].max_abs_delta) rmse=$($row[0].rmse_delta)"
    }
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath -Encoding UTF8)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) rows"
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($resultStore.series_count -ne 22 -or $resultStore.sample_count -ne $summary.samples) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}
$heatRecoveryActiveSeries = @($resultStore.series | Where-Object { $_.variable_name -eq "Zone Ideal Loads Heat Recovery Active Time" })
if ($heatRecoveryActiveSeries.Count -ne 1) {
    throw "Missing heat-recovery active-time result-store series"
}
$maxHeatRecoveryActiveTime = @($heatRecoveryActiveSeries[0].values | Measure-Object -Maximum)[0].Maximum
if ($maxHeatRecoveryActiveTime -le 0.0) {
    throw "Expected at least one active Enthalpy heat-recovery timestep"
}
$heatRecoverySensibleHeatingSeries = @($resultStore.series | Where-Object { $_.variable_name -eq "Zone Ideal Loads Heat Recovery Sensible Heating Rate" })
if ($heatRecoverySensibleHeatingSeries.Count -ne 1) {
    throw "Missing heat-recovery sensible-heating result-store series"
}
$maxHeatRecoverySensibleHeating = @($heatRecoverySensibleHeatingSeries[0].values | Measure-Object -Maximum)[0].Maximum
if ($maxHeatRecoverySensibleHeating -le 0.0) {
    throw "Expected Enthalpy heat recovery to report nonzero sensible heating"
}
$heatRecoveryLatentHeatingSeries = @($resultStore.series | Where-Object { $_.variable_name -eq "Zone Ideal Loads Heat Recovery Latent Heating Rate" })
if ($heatRecoveryLatentHeatingSeries.Count -ne 1) {
    throw "Missing heat-recovery latent-heating result-store series"
}
$maxHeatRecoveryLatentHeating = @($heatRecoveryLatentHeatingSeries[0].values | Measure-Object -Maximum)[0].Maximum
if ($maxHeatRecoveryLatentHeating -le 0.0) {
    throw "Expected Enthalpy heat recovery to report nonzero latent heating"
}

$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw -Encoding UTF8 | ConvertFrom-Json
if (@($selectedOutputs.series).Count -ne 22) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($stageSummary.branch -ne "outdoor-air-design-flow") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.outdoor_air -ne $true) {
    throw "Stage summary must record outdoor_air=true"
}
if ($stageSummary.outdoor_air_method -ne "Flow/Zone") {
    throw "Stage summary must record outdoor_air_method=Flow/Zone, got $($stageSummary.outdoor_air_method)"
}
if ($stageSummary.heat_recovery -ne "Enthalpy") {
    throw "Stage summary must record heat_recovery=Enthalpy, got $($stageSummary.heat_recovery)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance IdealLoads outdoor-air Enthalpy heat recovery branch for declared variables only; general heat-recovery saturation-limit branch parity remains outside the claim" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_outdoor_air_compat" -Description "markdown source-order wrapper"
Assert-Contains -Text $reportText -Pattern "ideal_loads_invocation_path: zone-equipment-validated source-order PurchasedAir wrapper" -Description "markdown IdealLoads invocation path"
Assert-Contains -Text $reportText -Pattern "direct_calc_helper_invocation: false" -Description "markdown direct calc helper invocation"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_execution_boundary: validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper" -Description "markdown zone-equipment execution boundary"
Assert-Contains -Text $reportText -Pattern "ideal_loads_runtime_binding_source: compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding" -Description "markdown typed-ID binding source"
Assert-Contains -Text $reportText -Pattern "purchased_air_name_lookup_policy: PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs" -Description "markdown runtime string lookup policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_flags:" -Description "markdown IdealLoads feature flags"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_dispatch_policy: compile feature flags select branch-specific source-order compat functions; unsupported active feature combinations emit diagnostics instead of approximate fallback" -Description "markdown IdealLoads feature dispatch policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_prebound_id_contract: compile-stage IdealLoadsAirSystemId, ZoneId, supply NodeId, return NodeId, zone air NodeId, optional outdoor air NodeId, availability ScheduleId, heating availability ScheduleId, and cooling availability ScheduleId" -Description "markdown IdealLoads prebound ID contract"
Assert-Contains -Text $reportText -Pattern "ideal_loads_psychrometric_evaluation_policy: compatibility reports use source-order direct psychrometric evaluation; no cross-timestep cache or reordering is enabled" -Description "markdown IdealLoads psychrometric evaluation policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_psychrometric_cache_policy: future compatibility cache must key exact temperature, humidity ratio, and pressure tuple and preserve EnergyPlus evaluation order" -Description "markdown IdealLoads psychrometric cache policy"
Assert-Contains -Text $reportText -Pattern "trace_level_source: case manifest [trace].level" -Description "markdown trace level source"
Assert-Contains -Text $reportText -Pattern "trace_result_invariance_policy: trace level selects evidence payload only; ResultStore values are computed before report serialization" -Description "markdown trace result invariance policy"
Assert-Contains -Text $reportText -Pattern "trace_overhead_accounting: trace/report serialization overhead is outside numerical conformance comparison and measured separately from simulation results" -Description "markdown trace overhead accounting"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "selected_purchased_air_branch: outdoor_air" -Description "markdown PurchasedAir branch"
Assert-Contains -Text $reportText -Pattern "declared_ideal_loads_branch:" -Description "markdown declared branch"
Assert-Contains -Text $reportText -Pattern "inactive_branches:" -Description "markdown inactive branches"
Assert-Contains -Text $reportText -Pattern "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md" -Description "markdown source-map anchor"
Assert-Contains -Text $reportText -Pattern "node_output_timestamp_alignment: timestamp" -Description "markdown node timestamp alignment"
Assert-Contains -Text $reportText -Pattern "outdoor_air_source: DesignSpecification:OutdoorAir Flow/Zone with blank OA schedule, EnergyPlus StdRhoAir from Site:Location, and source-order zone/OA/mixed-air state proof rows plus EnergyPlus Enthalpy heat recovery OA tempering when recirculation enthalpy can beneficially warm or cool outdoor air" -Description "markdown OA source"
Assert-Contains -Text $reportText -Pattern "outdoor_air_schedule: blank-always-1.0" -Description "markdown OA schedule guard"
foreach ($expected in $expectedRows) {
    Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | $($expected.Variable) | $($expected.Level)" -Description "markdown row $($expected.Variable)"
}

Write-Host "IdealLoads outdoor-air Enthalpy heat-recovery conformance candidate comparison artifacts generated."
