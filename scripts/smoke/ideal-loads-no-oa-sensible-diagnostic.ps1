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
$ReportRoot = Join-Path $OutputRoot "report-skeleton"
$CaseId = "ideal_loads_no_oa_sensible_conformance_001"
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

function Assert-CleanEnergyPlusErr {
    param([Parameter(Mandatory = $true)][string]$Path)
    $errText = Get-Content -LiteralPath $Path -Raw
    $warningLines = @($errText -split "`r?`n" | Where-Object { $_.Contains("** Warning **") })
    $severeLines = @($errText -split "`r?`n" | Where-Object { $_.Contains("** Severe") })
    $fatalLines = @($errText -split "`r?`n" | Where-Object { $_.Contains("** Fatal") })
    if ($warningLines.Count -gt 0 -or $severeLines.Count -gt 0 -or $fatalLines.Count -gt 0) {
        Write-Host $errText
        throw "IdealLoads no-OA sensible diagnostic baseline must not rely on EnergyPlus warning/severe/fatal auto-fixes."
    }
    Write-Host "OK clean EnergyPlus ERR: warnings=0 severes=0 fatals=0"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required IdealLoads no-OA sensible diagnostic input: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Validating IdealLoads no-OA sensible diagnostic case manifest."
$validateOutput = & $cargo.Source run -p ep_cli --quiet -- conformance validate-case-v2 $CasePath 2>&1
if ($LASTEXITCODE -ne 0) {
    $validateOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA sensible diagnostic case manifest validation failed."
}
$validateText = ($validateOutput -join "`n")
Assert-Contains -Text $validateText -Pattern "comparison_class: diagnostic-only" -Description "manifest diagnostic class"
Assert-Contains -Text $validateText -Pattern "conformance_claim: false" -Description "manifest claim boundary"
Assert-Contains -Text $validateText -Pattern "outputs: 16" -Description "manifest output count"
Assert-Contains -Text $validateText -Pattern "level=diagnostic" -Description "manifest diagnostic output level"
Assert-Contains -Text $validateText -Pattern "System Node Mass Flow Rate / detailed / node-state / eso" -Description "manifest node flow output"

Write-Host "Generating IdealLoads no-OA sensible oracle baseline."
$baselineOutput = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $baselineOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA sensible baseline generation failed."
}
$baselineText = ($baselineOutput -join "`n")
Assert-Contains -Text $baselineText -Pattern "Conformance Baseline" -Description "baseline header"
Assert-Contains -Text $baselineText -Pattern "id: $CaseId" -Description "baseline case id"
Assert-Contains -Text $baselineText -Pattern "comparison_class: diagnostic-only" -Description "baseline diagnostic class"
Assert-Contains -Text $baselineText -Pattern "conformance_claim: false" -Description "baseline claim boundary"
Assert-Contains -Text $baselineText -Pattern "status: generated" -Description "baseline status"
Assert-FileExists -Path $EpJsonPath -Description "converted diagnostic epJSON"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "eplusout.eso") -Description "diagnostic EnergyPlus ESO"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "eplusout.err") -Description "diagnostic EnergyPlus ERR"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "case-expanded.toml") -Description "diagnostic expanded manifest"
Assert-CleanEnergyPlusErr -Path (Join-Path $CaseOutputRoot "eplusout.err")

Write-Host "Compiling IdealLoads no-OA sensible typed model."
$compileOutput = & $cargo.Source run -p ep_cli --quiet -- compile $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $compileOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA sensible typed compile failed."
}
$compileText = ($compileOutput -join "`n")
Assert-Contains -Text $compileText -Pattern "TypedModel" -Description "compile header"
Assert-Contains -Text $compileText -Pattern "thermostat_dual_setpoints: 1" -Description "dual setpoint typed count"
Assert-Contains -Text $compileText -Pattern "zone_thermostats: 1" -Description "zone thermostat typed count"
Assert-Contains -Text $compileText -Pattern "ideal_loads_air_systems: 1" -Description "IdealLoads typed count"
Assert-Contains -Text $compileText -Pattern "zone_equipment_lists: 1" -Description "equipment list typed count"
Assert-Contains -Text $compileText -Pattern "zone_equipment_connections: 1" -Description "equipment connection typed count"
Assert-Contains -Text $compileText -Pattern "nodes: 3" -Description "node registry typed count"
Assert-Contains -Text $compileText -Pattern "node_lists: 1" -Description "NodeList typed count"

Write-Host "Planning IdealLoads no-OA sensible graph."
$planOutput = & $cargo.Source run -p ep_cli --quiet -- model plan $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $planOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA sensible execution-plan smoke failed."
}
$planText = ($planOutput -join "`n")
Assert-Contains -Text $planText -Pattern "ExecutionPlan" -Description "plan header"
Assert-Contains -Text $planText -Pattern "zone_ideal_loads_edges: 1" -Description "zone IdealLoads graph edge"
Assert-Contains -Text $planText -Pattern "ideal_loads_supply_node_edges: 1" -Description "IdealLoads supply node graph edge"
Assert-Contains -Text $planText -Pattern "EvaluateZoneThermostat(0)" -Description "thermostat execution step"
Assert-Contains -Text $planText -Pattern "SolveZone(0)" -Description "zone solve execution step"
Assert-Contains -Text $planText -Pattern "EvaluateIdealLoadsAirSystem(0)" -Description "IdealLoads execution step"

Write-Host "Writing IdealLoads no-OA sensible baseline report skeleton."
$reportOutput = & $cargo.Source run -p ep_cli --quiet -- conformance report-skeleton $CasePath $CaseOutputRoot $ReportRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $reportOutput | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA sensible report skeleton failed."
}
$reportText = ($reportOutput -join "`n")
Assert-Contains -Text $reportText -Pattern "Conformance Report Skeleton" -Description "report header"
Assert-Contains -Text $reportText -Pattern "id: $CaseId" -Description "report case id"
Assert-Contains -Text $reportText -Pattern "series: 16" -Description "report series count"
Assert-Contains -Text $reportText -Pattern "energyplus_warnings: 0" -Description "report warning count"
Assert-Contains -Text $reportText -Pattern "energyplus_severes: 0" -Description "report severe count"
Assert-Contains -Text $reportText -Pattern "energyplus_fatals: 0" -Description "report fatal count"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: none" -Description "report non-claim tolerance boundary"
Assert-Contains -Text $reportText -Pattern "status: baseline-only" -Description "report status"

$ReportCaseRoot = Join-Path $ReportRoot $CaseId
$MarkdownReport = Join-Path $ReportCaseRoot "compare-report.md"
$SummaryReport = Join-Path $ReportCaseRoot "compare-summary.json"
Assert-FileExists -Path $MarkdownReport -Description "diagnostic candidate markdown report"
Assert-FileExists -Path $SummaryReport -Description "diagnostic candidate summary report"

$summary = Get-Content -LiteralPath $SummaryReport -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected IdealLoads no-OA summary case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected IdealLoads no-OA summary comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "IdealLoads no-OA diagnostic summary must keep conformance_claim=false"
}
if ($summary.tolerance_policy -ne "none") {
    throw "IdealLoads no-OA diagnostic summary must keep tolerance_policy=none until compare artifacts exist"
}
if ($summary.status -ne "baseline-only") {
    throw "Unexpected IdealLoads no-OA summary status: $($summary.status)"
}
if ($summary.requested_outputs.Count -ne 16) {
    throw "Unexpected IdealLoads no-OA requested output count: $($summary.requested_outputs.Count)"
}

Write-Host "IdealLoads no-OA sensible diagnostic passed."
