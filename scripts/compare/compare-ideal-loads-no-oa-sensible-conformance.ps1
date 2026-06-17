[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-no-oa-sensible\26.1.0"
$CaseId = "ideal_loads_no_oa_sensible_conformance_001"
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

function Assert-PurchasedAirSourceOrder {
    param([Parameter(Mandatory = $true)]$StageSummary)

    $purchasedAirStages = @($StageSummary.purchased_air_stages)
    $expectedPurchasedAirRoutines = @(
        "GetPurchasedAir",
        "InitPurchasedAir",
        "CalcPurchAirLoads",
        "UpdatePurchasedAir",
        "ReportPurchasedAir"
    )
    if ($purchasedAirStages.Count -ne $expectedPurchasedAirRoutines.Count) {
        throw "Expected $($expectedPurchasedAirRoutines.Count) PurchasedAir stages, found $($purchasedAirStages.Count)"
    }
    for ($stageIndex = 0; $stageIndex -lt $expectedPurchasedAirRoutines.Count; $stageIndex++) {
        $actualRoutine = $purchasedAirStages[$stageIndex].source_routine
        if ($actualRoutine -ne $expectedPurchasedAirRoutines[$stageIndex]) {
            throw "Unexpected PurchasedAir stage at index ${stageIndex}: $actualRoutine"
        }
    }
    Write-Host "OK PurchasedAir source order: $($expectedPurchasedAirRoutines -join ' -> ')"
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

Write-Host "Generating IdealLoads no-OA sensible conformance comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA sensible conformance comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 28" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 110" -Description "detailed sample count"
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
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "IdealLoads conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads conformance status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads conformance comparison must have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.samples -ne 110) {
    throw "Unexpected IdealLoads sample count: $($summary.samples)"
}
if ($summary.series_count -ne 28) {
    throw "Unexpected IdealLoads series count: $($summary.series_count)"
}
if ($summary.zone_demand_synthetic_rc_model -ne $false) {
    throw "IdealLoads conformance must not synthesize zone demand from an RC shortcut"
}
$conformanceRows = @($summary.series | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 10) {
    throw "Expected 10 conformance-level output rows, found $($conformanceRows.Count)"
}
if (@($conformanceRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All conformance-level IdealLoads output rows must pass"
}
$nodeFlow = @($summary.series | Where-Object { $_.variable -eq "System Node Mass Flow Rate" })
if ($nodeFlow.Count -ne 1) {
    throw "Missing System Node Mass Flow Rate row"
}
if ($nodeFlow[0].alignment -ne "timestamp") {
    throw "System Node Mass Flow Rate must use timestamp alignment"
}
if ($nodeFlow[0].rust_source -ne "rust-ideal-loads-no-oa-sensible-calc") {
    throw "Unexpected node flow Rust source: $($nodeFlow[0].rust_source)"
}
if ($nodeFlow[0].level -ne "conformance") {
    throw "System Node Mass Flow Rate must be conformance-level in the promoted case"
}
if ($nodeFlow[0].status -ne "pass") {
    throw "System Node Mass Flow Rate must pass in conformance comparison"
}
$fuelRows = @($summary.series | Where-Object { $_.variable -like "Zone Ideal Loads *Fuel Energy Rate" })
if ($fuelRows.Count -ne 4) {
    throw "Expected 4 diagnostic fuel energy-rate rows, found $($fuelRows.Count)"
}
if (@($fuelRows | Where-Object { $_.level -ne "diagnostic" }).Count -ne 0) {
    throw "Fuel energy-rate rows must remain diagnostic-only"
}
if (@($fuelRows | Where-Object { $_.rust_source -ne "rust-ideal-loads-blank-fuel-efficiency" }).Count -ne 0) {
    throw "Fuel energy-rate rows must use the blank fuel-efficiency source"
}
if (@($fuelRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "Fuel energy-rate diagnostic rows must pass"
}
$energyRows = @($summary.series | Where-Object { $_.variable -like "Zone Ideal Loads *Energy" -and $_.variable -notlike "*Energy Rate" })
if ($energyRows.Count -ne 8) {
    throw "Expected 8 diagnostic IdealLoads energy rows, found $($energyRows.Count)"
}
if (@($energyRows | Where-Object { $_.level -ne "diagnostic" }).Count -ne 0) {
    throw "IdealLoads energy rows must remain diagnostic-only"
}
if (@($energyRows | Where-Object { $_.units -ne "J" }).Count -ne 0) {
    throw "IdealLoads energy rows must use joule units"
}
if (@($energyRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "IdealLoads energy diagnostic rows must pass"
}
$reportEnergyRows = @($energyRows | Where-Object { $_.variable -notlike "*Fuel Energy" })
if (@($reportEnergyRows | Where-Object { $_.rust_source -ne "rust-ideal-loads-report-time-step-energy" }).Count -ne 0) {
    throw "IdealLoads report energy rows must use the TimeStepSysSec source"
}
$fuelEnergyRows = @($energyRows | Where-Object { $_.variable -like "*Fuel Energy" })
if (@($fuelEnergyRows | Where-Object { $_.rust_source -ne "rust-ideal-loads-blank-fuel-efficiency-time-step-energy" }).Count -ne 0) {
    throw "IdealLoads fuel energy rows must use the blank fuel-efficiency TimeStepSysSec source"
}
if ([Math]::Abs([double]$summary.system_timestep_substeps - 8.0) -gt 1.0e-9) {
    throw "Unexpected IdealLoads system timestep substeps: $($summary.system_timestep_substeps)"
}
if ([Math]::Abs([double]$summary.system_timestep_seconds - 112.5) -gt 1.0e-9) {
    throw "Unexpected IdealLoads system timestep seconds: $($summary.system_timestep_seconds)"
}
if ([Math]::Abs([double]$summary.energy_report_interval_seconds - 900.0) -gt 1.0e-9) {
    throw "Unexpected IdealLoads energy report interval seconds: $($summary.energy_report_interval_seconds)"
}
if ($summary.rust_meter_time_series_comparison -ne $true) {
    throw "IdealLoads meter requests must compare Rust hourly facility meter diagnostics"
}
if ($summary.requested_meter_count -ne 2) {
    throw "Expected 2 requested diagnostic meter rows, found $($summary.requested_meter_count)"
}
$requestedMeters = @($summary.requested_meters)
if ($requestedMeters.Count -ne 2) {
    throw "Expected 2 requested_meters entries, found $($requestedMeters.Count)"
}
if (@($requestedMeters | Where-Object { $_.name -eq "DistrictHeatingWater:Facility" -and $_.source -eq "mtr" -and $_.level -eq "diagnostic" }).Count -ne 1) {
    throw "Missing diagnostic DistrictHeatingWater:Facility MTR request in summary"
}
if (@($requestedMeters | Where-Object { $_.name -eq "DistrictCooling:Facility" -and $_.source -eq "mtr" -and $_.level -eq "diagnostic" }).Count -ne 1) {
    throw "Missing diagnostic DistrictCooling:Facility MTR request in summary"
}
$meterRows = @($summary.meter_series)
if ($summary.meter_series_count -ne 2 -or $meterRows.Count -ne 2) {
    throw "Expected 2 compared meter series, found count=$($summary.meter_series_count) rows=$($meterRows.Count)"
}
if ($summary.meter_tolerance_failures -ne 0) {
    throw "Expected zero meter tolerance failures, found $($summary.meter_tolerance_failures)"
}
if (@($meterRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads facility meter diagnostics must pass"
}
if (@($meterRows | Where-Object { $_.name -eq "DistrictHeatingWater:Facility" -and $_.rust_source -eq "rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -and $_.alignment -eq "timestamp" -and $_.expected_samples -eq 24 -and $_.observed_samples -eq 24 -and $_.units -eq "J" }).Count -ne 1) {
    throw "Missing passing hourly heating facility meter diagnostic row"
}
if (@($meterRows | Where-Object { $_.name -eq "DistrictCooling:Facility" -and $_.rust_source -eq "rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -and $_.alignment -eq "timestamp" -and $_.expected_samples -eq 24 -and $_.observed_samples -eq 24 -and $_.units -eq "J" }).Count -ne 1) {
    throw "Missing passing hourly cooling facility meter diagnostic row"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 28 -or $resultStore.sample_count -ne 110) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw | ConvertFrom-Json
if (@($selectedOutputs.series).Count -ne 28) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$oracleMtrText = Get-Content -LiteralPath $oracleMtrPath -Raw
Assert-Contains -Text $oracleMtrText -Pattern "DistrictHeatingWater:Facility" -Description "oracle MTR heating meter"
Assert-Contains -Text $oracleMtrText -Pattern "DistrictCooling:Facility" -Description "oracle MTR cooling meter"

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "no-oa-no-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.zone_demand_synthetic_rc_model -ne $false) {
    throw "Stage summary must record that no RC demand shortcut is used"
}
Assert-PurchasedAirSourceOrder -StageSummary $stageSummary

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance no-OA/no-limit sensible IdealLoads branch for declared variables only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "zone_demand_synthetic_rc_model: false" -Description "markdown demand source guard"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat" -Description "markdown source-order wrapper"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "fuel_energy_rate_source: EnergyPlus ReportPurchasedAir blank fuel-efficiency schedule branch; diagnostic-only" -Description "markdown fuel source"
Assert-Contains -Text $reportText -Pattern "energy_source: EnergyPlus ReportPurchasedAir raw rate * TimeStepSysSec summed by OutputProcessor; diagnostic-only fixed_system_substeps=8 system_timestep_seconds=112.500000000000 energy_report_interval_seconds=900.000000000000" -Description "markdown energy source"
Assert-Contains -Text $reportText -Pattern "meter_source: EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy diagnostic; rust_meter_time_series_comparison=true requested_meters=2" -Description "markdown meter source"
Assert-Contains -Text $reportText -Pattern "meter_requests: DistrictHeatingWater:Facility, DistrictCooling:Facility" -Description "markdown meter requests"
Assert-Contains -Text $reportText -Pattern "| DistrictHeatingWater:Facility | diagnostic | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown heating meter row"
Assert-Contains -Text $reportText -Pattern "| DistrictCooling:Facility | diagnostic | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown cooling meter row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE INLET | System Node Mass Flow Rate | conformance" -Description "markdown node flow row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy Rate | diagnostic" -Description "markdown zone heating fuel row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy | diagnostic" -Description "markdown zone heating fuel energy row"

Write-Host "IdealLoads no-OA sensible conformance comparison artifacts generated."
