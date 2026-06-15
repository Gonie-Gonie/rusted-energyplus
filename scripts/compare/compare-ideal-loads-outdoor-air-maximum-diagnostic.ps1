[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-outdoor-air-maximum\26.1.0"
$CaseId = "ideal_loads_outdoor_air_maximum_diagnostic_001"
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
    Assert-FileExists -Path $path -Description "required IdealLoads outdoor-air compare input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads outdoor-air Maximum diagnostic comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-outdoor-air-design-flow-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads outdoor-air Maximum diagnostic comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads Outdoor-Air Design-Flow Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 22" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 96" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_failures_count: 0" -Description "tolerance failures"
Assert-Contains -Text $text -Pattern "tolerance_policy: diagnostic-draft" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: diagnostic" -Description "diagnostic status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$selectedOutputsPath = Join-Path $CompareRoot "selected_outputs.json"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$variableDeltasPath = Join-Path $CompareRoot "variable-deltas.csv"
$firstDivergencePath = Join-Path $CompareRoot "first-divergence.csv"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads outdoor-air compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads outdoor-air markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads outdoor-air selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads outdoor-air Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads outdoor-air variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads outdoor-air first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads outdoor-air tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads outdoor-air stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "IdealLoads outdoor-air diagnostic summary must set conformance_claim=false"
}
if ($summary.status -ne "diagnostic") {
    throw "Unexpected IdealLoads outdoor-air diagnostic status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads outdoor-air diagnostic should have zero tolerance failures: $($summary.tolerance_failures)"
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
    throw "Expected twenty-two outdoor-air diagnostic rows, found $($rows.Count)"
}
foreach ($row in $rows) {
    if ($row.level -ne "diagnostic") {
        throw "Outdoor-air row must be diagnostic-level: $($row.variable)"
    }
    if ($row.status -ne "pass") {
        throw "Outdoor-air row should pass: $($row.variable)"
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
if ($stageSummary.outdoor_air_method -ne "Maximum") {
    throw "Stage summary must record outdoor_air_method=Maximum, got $($stageSummary.outdoor_air_method)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: diagnostic-only IdealLoads outdoor-air Flow/Zone, Flow/Area, AirChanges/Hour, Sum, and Maximum mass, standard-density volume, outdoor-air report rates, supply-air state, mixed-air state, and inactive economizer/heat recovery" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "outdoor_air_source: DesignSpecification:OutdoorAir Maximum with blank OA schedule, EnergyPlus StdRhoAir from Site:Location, and source-order zone/OA/mixed-air state proof rows" -Description "markdown OA source"
Assert-Contains -Text $reportText -Pattern "outdoor_air_schedule: blank-always-1.0" -Description "markdown OA schedule guard"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Mass Flow Rate | diagnostic" -Description "markdown OA mass row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate | diagnostic" -Description "markdown OA volume row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Sensible Heating Rate | diagnostic" -Description "markdown OA sensible heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Sensible Cooling Rate | diagnostic" -Description "markdown OA sensible cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Latent Heating Rate | diagnostic" -Description "markdown OA latent heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Latent Cooling Rate | diagnostic" -Description "markdown OA latent cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Total Heating Rate | diagnostic" -Description "markdown OA total heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Total Cooling Rate | diagnostic" -Description "markdown OA total cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Mass Flow Rate | diagnostic" -Description "markdown supply-air mass row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Standard Density Volume Flow Rate | diagnostic" -Description "markdown supply-air volume row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Temperature | diagnostic" -Description "markdown supply-air temperature row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Humidity Ratio | diagnostic" -Description "markdown supply-air humidity-ratio row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Mixed Air Temperature | diagnostic" -Description "markdown mixed-air temperature row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Mixed Air Humidity Ratio | diagnostic" -Description "markdown mixed-air humidity-ratio row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Heat Recovery Sensible Heating Rate | diagnostic" -Description "markdown inactive heat-recovery sensible heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Heat Recovery Total Cooling Rate | diagnostic" -Description "markdown inactive heat-recovery total cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Economizer Active Time | diagnostic" -Description "markdown inactive economizer active-time row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Heat Recovery Active Time | diagnostic" -Description "markdown inactive heat-recovery active-time row"

Write-Host "IdealLoads outdoor-air Maximum diagnostic comparison artifacts generated."
