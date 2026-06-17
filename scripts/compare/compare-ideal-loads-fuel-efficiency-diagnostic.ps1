[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-fuel-efficiency\26.1.0"
$CaseId = "ideal_loads_fuel_efficiency_diagnostic_001"
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
    Assert-FileExists -Path $path -Description "required IdealLoads compare input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads fuel-efficiency diagnostic comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads fuel-efficiency diagnostic comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 12" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 110" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_failures_count: 0" -Description "zero diagnostic tolerance failures"
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
$oracleMtrPath = Join-Path (Join-Path $CaseOutputRoot "oracle") "eplusout.mtr"

Assert-FileExists -Path $summaryPath -Description "IdealLoads compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads oracle selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads stage summary"
Assert-FileExists -Path $oracleMtrPath -Description "IdealLoads oracle MTR"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "IdealLoads fuel-efficiency diagnostic summary must set conformance_claim=false"
}
if ($summary.status -ne "diagnostic") {
    throw "Unexpected IdealLoads fuel-efficiency diagnostic status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads fuel-efficiency diagnostic should have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.samples -ne 110) {
    throw "Unexpected IdealLoads sample count: $($summary.samples)"
}
if ($summary.series_count -ne 12) {
    throw "Unexpected IdealLoads series count: $($summary.series_count)"
}
if ([Math]::Abs([double]$summary.heating_fuel_efficiency - 0.8) -gt 1.0e-12) {
    throw "Unexpected heating fuel efficiency: $($summary.heating_fuel_efficiency)"
}
if ([Math]::Abs([double]$summary.cooling_fuel_efficiency - 0.75) -gt 1.0e-12) {
    throw "Unexpected cooling fuel efficiency: $($summary.cooling_fuel_efficiency)"
}
if ($summary.fuel_energy_rate_source -ne "EnergyPlus ReportPurchasedAir constant Schedule:Constant fuel-efficiency schedule branch; diagnostic-only") {
    throw "Unexpected fuel energy rate source: $($summary.fuel_energy_rate_source)"
}
if ($summary.fuel_energy_rate_rust_source -ne "rust-ideal-loads-constant-fuel-efficiency") {
    throw "Unexpected fuel energy rate Rust source: $($summary.fuel_energy_rate_rust_source)"
}
if ($summary.fuel_energy_rust_source -ne "rust-ideal-loads-constant-fuel-efficiency-time-step-energy") {
    throw "Unexpected fuel energy Rust source: $($summary.fuel_energy_rust_source)"
}
if ($summary.zone_demand_synthetic_rc_model -ne $false) {
    throw "IdealLoads fuel-efficiency diagnostic must not synthesize zone demand from an RC shortcut"
}
if ($summary.requested_meter_count -ne 2) {
    throw "Expected 2 requested diagnostic meter rows, found $($summary.requested_meter_count)"
}
if ($summary.rust_meter_time_series_comparison -ne $true) {
    throw "IdealLoads fuel-efficiency diagnostic must compare Rust hourly facility meter diagnostics"
}
if ($summary.meter_aggregation_source -ne "ep_runtime::RuntimeMeterRegistry") {
    throw "Unexpected meter aggregation source: $($summary.meter_aggregation_source)"
}
if ($summary.meter_fuel_energy_binding_source -ne "ep_runtime::ideal_loads_facility_meter_binding") {
    throw "Unexpected meter fuel-energy binding source: $($summary.meter_fuel_energy_binding_source)"
}
$meterRows = @($summary.meter_series)
if ($summary.meter_series_count -ne 2 -or $meterRows.Count -ne 2) {
    throw "Expected 2 compared meter series, found count=$($summary.meter_series_count) rows=$($meterRows.Count)"
}
if ($summary.meter_tolerance_failures -ne 0) {
    throw "Expected zero meter tolerance failures, found $($summary.meter_tolerance_failures)"
}
if (@($meterRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads fuel-efficiency facility meter diagnostics must pass"
}
if (@($meterRows | Where-Object { $_.name -eq "DistrictHeatingWater:Facility" -and $_.rust_source -eq "rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -and $_.alignment -eq "timestamp" -and $_.expected_samples -eq 24 -and $_.observed_samples -eq 24 -and $_.units -eq "J" }).Count -ne 1) {
    throw "Missing passing hourly heating facility meter diagnostic row"
}
if (@($meterRows | Where-Object { $_.name -eq "DistrictCooling:Facility" -and $_.rust_source -eq "rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -and $_.alignment -eq "timestamp" -and $_.expected_samples -eq 24 -and $_.observed_samples -eq 24 -and $_.units -eq "J" }).Count -ne 1) {
    throw "Missing passing hourly cooling facility meter diagnostic row"
}

$diagnosticRows = @($summary.series | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 12) {
    throw "Expected 12 diagnostic-level output rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads fuel-efficiency diagnostic rows must pass"
}
$rateRows = @($summary.series | Where-Object { $_.variable -like "*Fuel Energy Rate" })
if ($rateRows.Count -ne 4) {
    throw "Expected 4 fuel energy-rate rows, found $($rateRows.Count)"
}
if (@($rateRows | Where-Object { $_.rust_source -ne "rust-ideal-loads-constant-fuel-efficiency" }).Count -ne 0) {
    throw "Fuel energy-rate rows must use the constant fuel-efficiency Rust source"
}
$fuelEnergyRows = @($summary.series | Where-Object { $_.variable -like "*Fuel Energy" -and $_.variable -notlike "*Energy Rate" })
if ($fuelEnergyRows.Count -ne 4) {
    throw "Expected 4 fuel energy rows, found $($fuelEnergyRows.Count)"
}
if (@($fuelEnergyRows | Where-Object { $_.rust_source -ne "rust-ideal-loads-constant-fuel-efficiency-time-step-energy" }).Count -ne 0) {
    throw "Fuel energy rows must use the constant fuel-efficiency TimeStepSysSec source"
}
if (@($fuelEnergyRows | Where-Object { $_.units -ne "J" }).Count -ne 0) {
    throw "Fuel energy rows must use joule units"
}
$rawRateRows = @($summary.series | Where-Object { $_.variable -notlike "*Fuel Energy*" })
if ($rawRateRows.Count -ne 4) {
    throw "Expected 4 raw IdealLoads rate rows, found $($rawRateRows.Count)"
}
if (@($rawRateRows | Where-Object { $_.rust_source -ne "rust-ideal-loads-no-oa-sensible-calc" }).Count -ne 0) {
    throw "Raw IdealLoads rate rows must use the no-OA sensible Rust source"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 12 -or $resultStore.sample_count -ne 110) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw | ConvertFrom-Json
if (@($selectedOutputs.series).Count -ne 12) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "no-oa-no-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ([Math]::Abs([double]$stageSummary.heating_fuel_efficiency - 0.8) -gt 1.0e-12) {
    throw "Unexpected stage heating fuel efficiency: $($stageSummary.heating_fuel_efficiency)"
}
if ([Math]::Abs([double]$stageSummary.cooling_fuel_efficiency - 0.75) -gt 1.0e-12) {
    throw "Unexpected stage cooling fuel efficiency: $($stageSummary.cooling_fuel_efficiency)"
}
if ($stageSummary.meter_aggregation_source -ne "ep_runtime::RuntimeMeterRegistry") {
    throw "Unexpected stage meter aggregation source: $($stageSummary.meter_aggregation_source)"
}
if ($stageSummary.meter_fuel_energy_binding_source -ne "ep_runtime::ideal_loads_facility_meter_binding") {
    throw "Unexpected stage meter fuel-energy binding source: $($stageSummary.meter_fuel_energy_binding_source)"
}

$oracleMtrText = Get-Content -LiteralPath $oracleMtrPath -Raw
Assert-Contains -Text $oracleMtrText -Pattern "DistrictHeatingWater:Facility" -Description "oracle MTR heating meter"
Assert-Contains -Text $oracleMtrText -Pattern "DistrictCooling:Facility" -Description "oracle MTR cooling meter"

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: diagnostic-only no-OA/no-limit sensible IdealLoads branch" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "fuel_energy_rate_source: EnergyPlus ReportPurchasedAir constant Schedule:Constant fuel-efficiency schedule branch; diagnostic-only" -Description "markdown fuel source"
Assert-Contains -Text $reportText -Pattern "fuel_efficiency: heating=0.800000000000 cooling=0.750000000000" -Description "markdown fuel efficiency values"
Assert-Contains -Text $reportText -Pattern "meter_source: EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy diagnostic; rust_meter_time_series_comparison=true requested_meters=2" -Description "markdown meter source"
Assert-Contains -Text $reportText -Pattern "meter_aggregation_source: ep_runtime::RuntimeMeterRegistry" -Description "markdown meter aggregation source"
Assert-Contains -Text $reportText -Pattern "meter_fuel_energy_binding_source: ep_runtime::ideal_loads_facility_meter_binding" -Description "markdown meter fuel-energy binding source"
Assert-Contains -Text $reportText -Pattern "| DistrictHeatingWater:Facility | diagnostic | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown heating meter row"
Assert-Contains -Text $reportText -Pattern "| DistrictCooling:Facility | diagnostic | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown cooling meter row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy Rate | diagnostic" -Description "markdown zone heating fuel rate row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy | diagnostic" -Description "markdown zone heating fuel energy row"

Write-Host "IdealLoads fuel-efficiency diagnostic comparison artifacts generated."
