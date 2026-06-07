[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-thermostat\26.1.0"
$ReportRoot = Join-Path $OutputRoot "report-skeleton"
$CaseId = "ideal_loads_thermostat_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$EpJsonPath = Join-Path $CaseOutputRoot "input.epJSON"

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
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required IdealLoads thermostat smoke input: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Validating v0.10 IdealLoads thermostat case manifest."
$validateOutput = & $cargo.Source run -p ep_cli --quiet -- conformance validate-case $CasePath 2>&1
if ($LASTEXITCODE -ne 0) {
    $validateOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads thermostat case manifest validation failed."
}
$validateText = ($validateOutput -join "`n")
Assert-Contains -Text $validateText -Pattern "comparison_class: smoke" -Description "manifest smoke class"
Assert-Contains -Text $validateText -Pattern "conformance_claim: false" -Description "manifest claim boundary"
Assert-Contains -Text $validateText -Pattern "outputs: 4" -Description "manifest output count"

Write-Host "Generating v0.10 IdealLoads thermostat oracle baseline."
$baselineOutput = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $baselineOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads thermostat baseline generation failed."
}
$baselineText = ($baselineOutput -join "`n")
Assert-Contains -Text $baselineText -Pattern "Conformance Baseline" -Description "baseline header"
Assert-Contains -Text $baselineText -Pattern "id: $CaseId" -Description "baseline case id"
Assert-Contains -Text $baselineText -Pattern "comparison_class: smoke" -Description "baseline smoke class"
Assert-Contains -Text $baselineText -Pattern "conformance_claim: false" -Description "baseline claim boundary"
Assert-Contains -Text $baselineText -Pattern "status: generated" -Description "baseline status"
Assert-FileExists -Path $EpJsonPath -Description "converted v0.10 epJSON"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "eplusout.eso") -Description "v0.10 EnergyPlus ESO"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "case-expanded.toml") -Description "v0.10 expanded manifest"

Write-Host "Compiling v0.10 IdealLoads thermostat typed model."
$compileOutput = & $cargo.Source run -p ep_cli --quiet -- compile $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $compileOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads thermostat typed compile failed."
}
$compileText = ($compileOutput -join "`n")
Assert-Contains -Text $compileText -Pattern "TypedModel" -Description "compile header"
Assert-Contains -Text $compileText -Pattern "thermostat_dual_setpoints: 1" -Description "dual setpoint typed count"
Assert-Contains -Text $compileText -Pattern "zone_thermostats: 1" -Description "zone thermostat typed count"
Assert-Contains -Text $compileText -Pattern "ideal_loads_air_systems: 1" -Description "IdealLoads typed count"
Assert-Contains -Text $compileText -Pattern "zone_equipment_lists: 1" -Description "equipment list typed count"
Assert-Contains -Text $compileText -Pattern "zone_equipment_connections: 1" -Description "equipment connection typed count"
Assert-Contains -Text $compileText -Pattern "ThermostatSetpoint:DualSetpoint: 1 [typed]" -Description "dual setpoint coverage"
Assert-Contains -Text $compileText -Pattern "ZoneControl:Thermostat: 1 [typed]" -Description "thermostat coverage"
Assert-Contains -Text $compileText -Pattern "ZoneHVAC:IdealLoadsAirSystem: 1 [typed]" -Description "IdealLoads coverage"
Assert-Contains -Text $compileText -Pattern "ZoneHVAC:EquipmentList: 1 [typed]" -Description "equipment list coverage"
Assert-Contains -Text $compileText -Pattern "ZoneHVAC:EquipmentConnections: 1 [typed]" -Description "equipment connection coverage"

Write-Host "Planning v0.10 IdealLoads thermostat graph."
$planOutput = & $cargo.Source run -p ep_cli --quiet -- model plan $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $planOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads thermostat execution-plan smoke failed."
}
$planText = ($planOutput -join "`n")
Assert-Contains -Text $planText -Pattern "ExecutionPlan" -Description "plan header"
Assert-Contains -Text $planText -Pattern "zone_thermostat_edges: 1" -Description "zone thermostat graph edge"
Assert-Contains -Text $planText -Pattern "thermostat_setpoint_edges: 1" -Description "thermostat setpoint graph edge"
Assert-Contains -Text $planText -Pattern "zone_ideal_loads_edges: 1" -Description "zone IdealLoads graph edge"
Assert-Contains -Text $planText -Pattern "steps: 8" -Description "plan step count"
Assert-Contains -Text $planText -Pattern "zone: 3" -Description "zone stage step count"

Write-Host "Writing v0.10 IdealLoads thermostat baseline-only report skeleton."
$reportOutput = & $cargo.Source run -p ep_cli --quiet -- conformance report-skeleton $CasePath $CaseOutputRoot $ReportRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $reportOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads thermostat report skeleton failed."
}
$reportText = ($reportOutput -join "`n")
Assert-Contains -Text $reportText -Pattern "Conformance Report Skeleton" -Description "report header"
Assert-Contains -Text $reportText -Pattern "id: $CaseId" -Description "report case id"
Assert-Contains -Text $reportText -Pattern "series: 4" -Description "report series count"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: none" -Description "report tolerance boundary"
Assert-Contains -Text $reportText -Pattern "status: baseline-only" -Description "report status"

$ReportCaseRoot = Join-Path $ReportRoot $CaseId
$MarkdownReport = Join-Path $ReportCaseRoot "compare-report.md"
$SummaryReport = Join-Path $ReportCaseRoot "compare-summary.json"
Assert-FileExists -Path $MarkdownReport -Description "v0.10 baseline-only markdown report"
Assert-FileExists -Path $SummaryReport -Description "v0.10 baseline-only summary report"

$markdown = Get-Content -LiteralPath $MarkdownReport -Raw
Assert-Contains -Text $markdown -Pattern "comparison_class: smoke" -Description "markdown smoke class"
Assert-Contains -Text $markdown -Pattern "conformance_claim: false" -Description "markdown claim boundary"
Assert-Contains -Text $markdown -Pattern "tolerance_policy: none" -Description "markdown tolerance boundary"
Assert-Contains -Text $markdown -Pattern "status: baseline-only" -Description "markdown baseline-only status"
Assert-Contains -Text $markdown -Pattern "Zone Thermostat Heating Setpoint Temperature" -Description "markdown thermostat heating setpoint"
Assert-Contains -Text $markdown -Pattern "Zone Thermostat Cooling Setpoint Temperature" -Description "markdown thermostat cooling setpoint"
Assert-Contains -Text $markdown -Pattern "Zone Ideal Loads Zone Total Heating Rate" -Description "markdown IdealLoads heating rate"
Assert-Contains -Text $markdown -Pattern "Zone Ideal Loads Zone Total Cooling Rate" -Description "markdown IdealLoads cooling rate"
Assert-Contains -Text $markdown -Pattern "hvac-state" -Description "markdown HVAC state class"

$summary = Get-Content -LiteralPath $SummaryReport -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected v0.10 summary case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "smoke") {
    throw "Unexpected v0.10 summary comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "v0.10 smoke summary must keep conformance_claim=false"
}
if ($summary.tolerance_policy -ne "none") {
    throw "v0.10 smoke summary must keep tolerance_policy=none"
}
if ($summary.status -ne "baseline-only") {
    throw "Unexpected v0.10 summary status: $($summary.status)"
}
if ($summary.requested_outputs.Count -ne 4) {
    throw "Unexpected v0.10 summary requested output count: $($summary.requested_outputs.Count)"
}
if (-not ($summary.requested_outputs | Where-Object { $_.class -eq "hvac-state" -and $_.variable -eq "Zone Ideal Loads Zone Total Heating Rate" })) {
    throw "Missing v0.10 hvac-state heating-rate summary series"
}
if (-not ($summary.requested_outputs | Where-Object { $_.class -eq "zone-state" -and $_.variable -eq "Zone Thermostat Cooling Setpoint Temperature" })) {
    throw "Missing v0.10 thermostat cooling setpoint summary series"
}

Write-Host "IdealLoads thermostat smoke passed."
