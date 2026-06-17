[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-outdoor-air-sum-conformance\26.1.0"
$CaseId = "ideal_loads_outdoor_air_sum_conformance_candidate_001"
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

function Assert-NearlyEqual {
    param(
        [Parameter(Mandatory = $true)][double]$Actual,
        [Parameter(Mandatory = $true)][double]$Expected,
        [Parameter(Mandatory = $true)][double]$Tolerance,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ([math]::Abs($Actual - $Expected) -gt $Tolerance) {
        throw "Unexpected $Description`: expected $Expected, got $Actual"
    }
    Write-Host "OK $Description`: $Actual"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required IdealLoads outdoor-air Sum conformance input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads outdoor-air Sum conformance candidate artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-outdoor-air-design-flow-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads outdoor-air Sum conformance candidate comparison failed."
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

Assert-FileExists -Path $summaryPath -Description "IdealLoads outdoor-air Sum conformance compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads outdoor-air Sum conformance markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads outdoor-air Sum conformance selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads outdoor-air Sum conformance Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads outdoor-air Sum conformance variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads outdoor-air Sum conformance first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads outdoor-air Sum conformance tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads outdoor-air Sum conformance stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "IdealLoads outdoor-air Sum conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads outdoor-air Sum conformance status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads outdoor-air Sum conformance should have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.series_count -ne 22) {
    throw "Unexpected IdealLoads outdoor-air series count: $($summary.series_count)"
}
if ($summary.samples -ne 96) {
    throw "Unexpected IdealLoads outdoor-air sample count: $($summary.samples)"
}
Assert-NearlyEqual -Actual $summary.design_volume_flow_rate_m3_per_s -Expected 0.05 -Tolerance 0.000000000001 -Description "outdoor-air Sum final design volume flow"
if ($summary.zone_volume_m3 -ne 1.0) {
    throw "Unexpected outdoor-air zone volume: $($summary.zone_volume_m3)"
}
Assert-NearlyEqual -Actual $summary.outdoor_air_flow_per_person_m3_per_s -Expected 0.0 -Tolerance 0.000000000001 -Description "outdoor-air Flow/Person component"
Assert-NearlyEqual -Actual $summary.outdoor_air_flow_per_area_m3_per_s -Expected 0.015 -Tolerance 0.000000000001 -Description "outdoor-air Flow/Area component"
Assert-NearlyEqual -Actual $summary.outdoor_air_flow_per_zone_m3_per_s -Expected 0.025 -Tolerance 0.000000000001 -Description "outdoor-air Flow/Zone component"
Assert-NearlyEqual -Actual $summary.outdoor_air_air_changes_m3_per_s -Expected 0.01 -Tolerance 0.000000000001 -Description "outdoor-air AirChanges/Hour component"
$summarySum = $summary.outdoor_air_flow_per_person_m3_per_s + $summary.outdoor_air_flow_per_area_m3_per_s + $summary.outdoor_air_flow_per_zone_m3_per_s + $summary.outdoor_air_air_changes_m3_per_s
Assert-NearlyEqual -Actual $summarySum -Expected $summary.design_volume_flow_rate_m3_per_s -Tolerance 0.000000000001 -Description "outdoor-air Sum component total"

$rows = @($summary.series)
if ($rows.Count -ne 22) {
    throw "Expected twenty-two outdoor-air rows, found $($rows.Count)"
}
$conformanceRows = @($rows | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 14) {
    throw "Expected fourteen outdoor-air Sum conformance rows, found $($conformanceRows.Count)"
}
if (@($conformanceRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All outdoor-air Sum conformance rows must pass"
}
$diagnosticRows = @($rows | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 8) {
    throw "Expected eight outdoor-air inactive proof diagnostic rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All outdoor-air inactive proof diagnostic rows must pass"
}
foreach ($expected in @(
    @("Zone Ideal Loads Outdoor Air Mass Flow Rate", "conformance"),
    @("Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate", "conformance"),
    @("Zone Ideal Loads Outdoor Air Sensible Heating Rate", "conformance"),
    @("Zone Ideal Loads Outdoor Air Sensible Cooling Rate", "conformance"),
    @("Zone Ideal Loads Outdoor Air Latent Heating Rate", "conformance"),
    @("Zone Ideal Loads Outdoor Air Latent Cooling Rate", "conformance"),
    @("Zone Ideal Loads Outdoor Air Total Heating Rate", "conformance"),
    @("Zone Ideal Loads Outdoor Air Total Cooling Rate", "conformance"),
    @("Zone Ideal Loads Supply Air Mass Flow Rate", "conformance"),
    @("Zone Ideal Loads Supply Air Standard Density Volume Flow Rate", "conformance"),
    @("Zone Ideal Loads Supply Air Temperature", "conformance"),
    @("Zone Ideal Loads Supply Air Humidity Ratio", "conformance"),
    @("Zone Ideal Loads Mixed Air Temperature", "conformance"),
    @("Zone Ideal Loads Mixed Air Humidity Ratio", "conformance"),
    @("Zone Ideal Loads Heat Recovery Sensible Heating Rate", "diagnostic"),
    @("Zone Ideal Loads Heat Recovery Latent Heating Rate", "diagnostic"),
    @("Zone Ideal Loads Heat Recovery Total Heating Rate", "diagnostic"),
    @("Zone Ideal Loads Heat Recovery Sensible Cooling Rate", "diagnostic"),
    @("Zone Ideal Loads Heat Recovery Latent Cooling Rate", "diagnostic"),
    @("Zone Ideal Loads Heat Recovery Total Cooling Rate", "diagnostic"),
    @("Zone Ideal Loads Economizer Active Time", "diagnostic"),
    @("Zone Ideal Loads Heat Recovery Active Time", "diagnostic")
)) {
    $row = @($rows | Where-Object { $_.variable -eq $expected[0] })
    if ($row.Count -ne 1) {
        throw "Expected one row for $($expected[0]), found $($row.Count)"
    }
    if ($row[0].level -ne $expected[1]) {
        throw "Unexpected level for $($expected[0]): $($row[0].level)"
    }
}

$massRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Mass Flow Rate" })
if ($massRow.Count -ne 1 -or $massRow[0].units -ne "kg/s" -or $massRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-design-flow") {
    throw "Missing kg/s outdoor-air mass-flow row"
}
$volumeRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate" })
if ($volumeRow.Count -ne 1 -or $volumeRow[0].units -ne "m3/s" -or $volumeRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-design-flow") {
    throw "Missing m3/s outdoor-air standard-density volume-flow row"
}
$sensibleHeatingRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Sensible Heating Rate" })
if ($sensibleHeatingRow.Count -ne 1 -or $sensibleHeatingRow[0].units -ne "W" -or $sensibleHeatingRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-sensible-report") {
    throw "Missing W outdoor-air sensible heating row"
}
if ($sensibleHeatingRow[0].max_abs_delta -gt 1.0 -or $sensibleHeatingRow[0].rmse_delta -gt 1.0) {
    throw "Outdoor-air sensible heating row exceeded diagnostic tolerance: max_abs=$($sensibleHeatingRow[0].max_abs_delta) rmse=$($sensibleHeatingRow[0].rmse_delta)"
}
$sensibleCoolingRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Sensible Cooling Rate" })
if ($sensibleCoolingRow.Count -ne 1 -or $sensibleCoolingRow[0].units -ne "W" -or $sensibleCoolingRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-sensible-report") {
    throw "Missing W outdoor-air sensible cooling row"
}
$latentHeatingRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Latent Heating Rate" })
if ($latentHeatingRow.Count -ne 1 -or $latentHeatingRow[0].units -ne "W" -or $latentHeatingRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-latent-report") {
    throw "Missing W outdoor-air latent heating row"
}
if ($latentHeatingRow[0].max_abs_delta -gt 0.000000001 -or $latentHeatingRow[0].rmse_delta -gt 0.000000001) {
    throw "Outdoor-air latent heating row exceeded diagnostic tolerance: max_abs=$($latentHeatingRow[0].max_abs_delta) rmse=$($latentHeatingRow[0].rmse_delta)"
}
$latentCoolingRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Latent Cooling Rate" })
if ($latentCoolingRow.Count -ne 1 -or $latentCoolingRow[0].units -ne "W" -or $latentCoolingRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-latent-report") {
    throw "Missing W outdoor-air latent cooling row"
}
if ($latentCoolingRow[0].max_abs_delta -gt 0.000000001 -or $latentCoolingRow[0].rmse_delta -gt 0.000000001) {
    throw "Outdoor-air latent cooling row exceeded diagnostic tolerance: max_abs=$($latentCoolingRow[0].max_abs_delta) rmse=$($latentCoolingRow[0].rmse_delta)"
}
$totalHeatingRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Total Heating Rate" })
if ($totalHeatingRow.Count -ne 1 -or $totalHeatingRow[0].units -ne "W" -or $totalHeatingRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-total-report") {
    throw "Missing W outdoor-air total heating row"
}
if ($totalHeatingRow[0].max_abs_delta -gt 1.0 -or $totalHeatingRow[0].rmse_delta -gt 1.0) {
    throw "Outdoor-air total heating row exceeded diagnostic tolerance: max_abs=$($totalHeatingRow[0].max_abs_delta) rmse=$($totalHeatingRow[0].rmse_delta)"
}
$totalCoolingRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Total Cooling Rate" })
if ($totalCoolingRow.Count -ne 1 -or $totalCoolingRow[0].units -ne "W" -or $totalCoolingRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-total-report") {
    throw "Missing W outdoor-air total cooling row"
}
if ($totalCoolingRow[0].max_abs_delta -gt 1.0 -or $totalCoolingRow[0].rmse_delta -gt 1.0) {
    throw "Outdoor-air total cooling row exceeded diagnostic tolerance: max_abs=$($totalCoolingRow[0].max_abs_delta) rmse=$($totalCoolingRow[0].rmse_delta)"
}
$supplyMassRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Supply Air Mass Flow Rate" })
if ($supplyMassRow.Count -ne 1 -or $supplyMassRow[0].units -ne "kg/s" -or $supplyMassRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-supply-state") {
    throw "Missing kg/s supply-air mass-flow row"
}
if ($supplyMassRow[0].max_abs_delta -gt 0.000001 -or $supplyMassRow[0].rmse_delta -gt 0.000001) {
    throw "Supply-air mass-flow row exceeded diagnostic tolerance: max_abs=$($supplyMassRow[0].max_abs_delta) rmse=$($supplyMassRow[0].rmse_delta)"
}
$supplyVolumeRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Supply Air Standard Density Volume Flow Rate" })
if ($supplyVolumeRow.Count -ne 1 -or $supplyVolumeRow[0].units -ne "m3/s" -or $supplyVolumeRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-supply-state") {
    throw "Missing m3/s supply-air standard-density volume-flow row"
}
if ($supplyVolumeRow[0].max_abs_delta -gt 0.000001 -or $supplyVolumeRow[0].rmse_delta -gt 0.000001) {
    throw "Supply-air standard-density volume-flow row exceeded diagnostic tolerance: max_abs=$($supplyVolumeRow[0].max_abs_delta) rmse=$($supplyVolumeRow[0].rmse_delta)"
}
$supplyTemperatureRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Supply Air Temperature" })
if ($supplyTemperatureRow.Count -ne 1 -or $supplyTemperatureRow[0].units -ne "C" -or $supplyTemperatureRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-supply-state") {
    throw "Missing C supply-air temperature row"
}
if ($supplyTemperatureRow[0].max_abs_delta -gt 0.02 -or $supplyTemperatureRow[0].rmse_delta -gt 0.02) {
    throw "Supply-air temperature row exceeded diagnostic tolerance: max_abs=$($supplyTemperatureRow[0].max_abs_delta) rmse=$($supplyTemperatureRow[0].rmse_delta)"
}
$supplyHumidityRatioRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Supply Air Humidity Ratio" })
if ($supplyHumidityRatioRow.Count -ne 1 -or $supplyHumidityRatioRow[0].units -ne "kgWater/kgDryAir" -or $supplyHumidityRatioRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-supply-state") {
    throw "Missing kgWater/kgDryAir supply-air humidity-ratio row"
}
if ($supplyHumidityRatioRow[0].max_abs_delta -gt 0.000001 -or $supplyHumidityRatioRow[0].rmse_delta -gt 0.000001) {
    throw "Supply-air humidity-ratio row exceeded diagnostic tolerance: max_abs=$($supplyHumidityRatioRow[0].max_abs_delta) rmse=$($supplyHumidityRatioRow[0].rmse_delta)"
}
$mixedAirTemperatureRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Mixed Air Temperature" })
if ($mixedAirTemperatureRow.Count -ne 1 -or $mixedAirTemperatureRow[0].units -ne "C" -or $mixedAirTemperatureRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-mixed-air") {
    throw "Missing C mixed-air temperature row"
}
if ($mixedAirTemperatureRow[0].max_abs_delta -gt 0.02 -or $mixedAirTemperatureRow[0].rmse_delta -gt 0.02) {
    throw "Mixed-air temperature row exceeded diagnostic tolerance: max_abs=$($mixedAirTemperatureRow[0].max_abs_delta) rmse=$($mixedAirTemperatureRow[0].rmse_delta)"
}
$mixedAirHumidityRatioRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Mixed Air Humidity Ratio" })
if ($mixedAirHumidityRatioRow.Count -ne 1 -or $mixedAirHumidityRatioRow[0].units -ne "kgWater/kgDryAir" -or $mixedAirHumidityRatioRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-mixed-air") {
    throw "Missing kgWater/kgDryAir mixed-air humidity-ratio row"
}
if ($mixedAirHumidityRatioRow[0].max_abs_delta -gt 0.000001 -or $mixedAirHumidityRatioRow[0].rmse_delta -gt 0.000001) {
    throw "Mixed-air humidity-ratio row exceeded diagnostic tolerance: max_abs=$($mixedAirHumidityRatioRow[0].max_abs_delta) rmse=$($mixedAirHumidityRatioRow[0].rmse_delta)"
}

foreach ($heatRecoveryVariable in @(
    "Zone Ideal Loads Heat Recovery Sensible Heating Rate",
    "Zone Ideal Loads Heat Recovery Latent Heating Rate",
    "Zone Ideal Loads Heat Recovery Total Heating Rate",
    "Zone Ideal Loads Heat Recovery Sensible Cooling Rate",
    "Zone Ideal Loads Heat Recovery Latent Cooling Rate",
    "Zone Ideal Loads Heat Recovery Total Cooling Rate"
)) {
    $heatRecoveryRow = @($rows | Where-Object { $_.variable -eq $heatRecoveryVariable })
    if ($heatRecoveryRow.Count -ne 1 -or $heatRecoveryRow[0].units -ne "W" -or $heatRecoveryRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-inactive-heat-recovery") {
        throw "Missing W inactive heat-recovery row: $heatRecoveryVariable"
    }
    if ($heatRecoveryRow[0].max_abs_delta -gt 0.000000001 -or $heatRecoveryRow[0].rmse_delta -gt 0.000000001) {
        throw "Inactive heat-recovery row exceeded zero diagnostic tolerance: variable=$heatRecoveryVariable max_abs=$($heatRecoveryRow[0].max_abs_delta) rmse=$($heatRecoveryRow[0].rmse_delta)"
    }
}
$economizerActiveRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Economizer Active Time" })
if ($economizerActiveRow.Count -ne 1 -or $economizerActiveRow[0].units -ne "hr" -or $economizerActiveRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-inactive-economizer") {
    throw "Missing hr inactive economizer active-time row"
}
if ($economizerActiveRow[0].max_abs_delta -gt 0.000000001 -or $economizerActiveRow[0].rmse_delta -gt 0.000000001) {
    throw "Inactive economizer active-time row exceeded zero diagnostic tolerance: max_abs=$($economizerActiveRow[0].max_abs_delta) rmse=$($economizerActiveRow[0].rmse_delta)"
}
$heatRecoveryActiveRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Heat Recovery Active Time" })
if ($heatRecoveryActiveRow.Count -ne 1 -or $heatRecoveryActiveRow[0].units -ne "hr" -or $heatRecoveryActiveRow[0].rust_source -ne "rust-ideal-loads-outdoor-air-inactive-heat-recovery") {
    throw "Missing hr inactive heat-recovery active-time row"
}
if ($heatRecoveryActiveRow[0].max_abs_delta -gt 0.000000001 -or $heatRecoveryActiveRow[0].rmse_delta -gt 0.000000001) {
    throw "Inactive heat-recovery active-time row exceeded zero diagnostic tolerance: max_abs=$($heatRecoveryActiveRow[0].max_abs_delta) rmse=$($heatRecoveryActiveRow[0].rmse_delta)"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) rows"
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 22 -or $resultStore.sample_count -ne $summary.samples) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw | ConvertFrom-Json
if (@($selectedOutputs.series).Count -ne 22) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "outdoor-air-design-flow") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.outdoor_air -ne $true) {
    throw "Stage summary must record outdoor_air=true"
}
if ($stageSummary.outdoor_air_method -ne "Sum") {
    throw "Stage summary must record outdoor_air_method=Sum, got $($stageSummary.outdoor_air_method)"
}
if ($stageSummary.zone_volume_m3 -ne 1.0) {
    throw "Stage summary must record zone_volume_m3=1, got $($stageSummary.zone_volume_m3)"
}
Assert-NearlyEqual -Actual $stageSummary.outdoor_air_flow_per_person_m3_per_s -Expected 0.0 -Tolerance 0.000000000001 -Description "stage-summary Flow/Person component"
Assert-NearlyEqual -Actual $stageSummary.outdoor_air_flow_per_area_m3_per_s -Expected 0.015 -Tolerance 0.000000000001 -Description "stage-summary Flow/Area component"
Assert-NearlyEqual -Actual $stageSummary.outdoor_air_flow_per_zone_m3_per_s -Expected 0.025 -Tolerance 0.000000000001 -Description "stage-summary Flow/Zone component"
Assert-NearlyEqual -Actual $stageSummary.outdoor_air_air_changes_m3_per_s -Expected 0.01 -Tolerance 0.000000000001 -Description "stage-summary AirChanges/Hour component"
$stageSum = $stageSummary.outdoor_air_flow_per_person_m3_per_s + $stageSummary.outdoor_air_flow_per_area_m3_per_s + $stageSummary.outdoor_air_flow_per_zone_m3_per_s + $stageSummary.outdoor_air_air_changes_m3_per_s
Assert-NearlyEqual -Actual $stageSum -Expected $stageSummary.design_volume_flow_rate_m3_per_s -Tolerance 0.000000000001 -Description "stage-summary Sum component total"

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance IdealLoads outdoor-air Sum branch for declared variables only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_outdoor_air_compat" -Description "markdown source-order wrapper"
Assert-Contains -Text $reportText -Pattern "ideal_loads_invocation_path: zone-equipment-validated source-order PurchasedAir wrapper" -Description "markdown IdealLoads invocation path"
Assert-Contains -Text $reportText -Pattern "direct_calc_helper_invocation: false" -Description "markdown direct calc helper invocation"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_execution_boundary: validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper" -Description "markdown zone-equipment execution boundary"
Assert-Contains -Text $reportText -Pattern "ideal_loads_runtime_binding_source: compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding" -Description "markdown typed-ID binding source"
Assert-Contains -Text $reportText -Pattern "purchased_air_name_lookup_policy: PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs" -Description "markdown runtime string lookup policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_flags:" -Description "markdown IdealLoads feature flags"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "selected_purchased_air_branch: outdoor_air" -Description "markdown PurchasedAir branch"
Assert-Contains -Text $reportText -Pattern "declared_ideal_loads_branch:" -Description "markdown declared branch"
Assert-Contains -Text $reportText -Pattern "inactive_branches:" -Description "markdown inactive branches"
Assert-Contains -Text $reportText -Pattern "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md" -Description "markdown source-map anchor"
Assert-Contains -Text $reportText -Pattern "node_output_timestamp_alignment: timestamp" -Description "markdown node timestamp alignment"
Assert-Contains -Text $reportText -Pattern "outdoor_air_source: DesignSpecification:OutdoorAir Sum with blank OA schedule, EnergyPlus StdRhoAir from Site:Location, and source-order zone/OA/mixed-air state proof rows" -Description "markdown OA source"
Assert-Contains -Text $reportText -Pattern "outdoor_air_schedule: blank-always-1.0" -Description "markdown OA schedule guard"
Assert-Contains -Text $reportText -Pattern "zone_volume_m3: 1.000000000000000" -Description "markdown zone volume"
Assert-Contains -Text $reportText -Pattern "outdoor_air_flow_per_person_m3_per_s: 0.000000000000000" -Description "markdown Flow/Person component"
Assert-Contains -Text $reportText -Pattern "outdoor_air_flow_per_area_m3_per_s: 0.015000000000000" -Description "markdown Flow/Area component"
Assert-Contains -Text $reportText -Pattern "outdoor_air_flow_per_zone_m3_per_s: 0.025000000000000" -Description "markdown Flow/Zone component"
Assert-Contains -Text $reportText -Pattern "outdoor_air_air_changes_m3_per_s: 0.010000000000000" -Description "markdown AirChanges/Hour component"
Assert-Contains -Text $reportText -Pattern "design_volume_flow_rate_m3_per_s: 0.050000000000000" -Description "markdown Sum final design volume"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Mass Flow Rate | conformance" -Description "markdown OA mass row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate | conformance" -Description "markdown OA volume row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Sensible Heating Rate | conformance" -Description "markdown OA sensible heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Sensible Cooling Rate | conformance" -Description "markdown OA sensible cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Latent Heating Rate | conformance" -Description "markdown OA latent heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Latent Cooling Rate | conformance" -Description "markdown OA latent cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Total Heating Rate | conformance" -Description "markdown OA total heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Total Cooling Rate | conformance" -Description "markdown OA total cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Mass Flow Rate | conformance" -Description "markdown supply-air mass row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Standard Density Volume Flow Rate | conformance" -Description "markdown supply-air volume row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Temperature | conformance" -Description "markdown supply-air temperature row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Humidity Ratio | conformance" -Description "markdown supply-air humidity-ratio row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Mixed Air Temperature | conformance" -Description "markdown mixed-air temperature row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Mixed Air Humidity Ratio | conformance" -Description "markdown mixed-air humidity-ratio row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Heat Recovery Sensible Heating Rate | diagnostic" -Description "markdown inactive heat-recovery sensible heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Heat Recovery Total Cooling Rate | diagnostic" -Description "markdown inactive heat-recovery total cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Economizer Active Time | diagnostic" -Description "markdown inactive economizer active-time row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Heat Recovery Active Time | diagnostic" -Description "markdown inactive heat-recovery active-time row"

Write-Host "IdealLoads outdoor-air Sum conformance candidate artifacts generated."

