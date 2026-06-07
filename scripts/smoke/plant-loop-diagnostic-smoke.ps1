[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\plant-loop-diagnostic\26.1.0"
$ReportRoot = Join-Path $OutputRoot "report-skeleton"
$CaseId = "plant_loop_diagnostic_001"
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
        throw "v0.15 plant-loop diagnostic baseline must not rely on EnergyPlus warning/severe/fatal auto-fixes."
    }
    Write-Host "OK clean EnergyPlus ERR: warnings=0 severes=0 fatals=0"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_IL_Chicago-OHare.Intl.AP.725300_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required plant-loop diagnostic input: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Validating v0.15 plant-loop diagnostic case manifest."
$validateOutput = & $cargo.Source run -p ep_cli --quiet -- conformance validate-case $CasePath 2>&1
if ($LASTEXITCODE -ne 0) {
    $validateOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop diagnostic case manifest validation failed."
}
$validateText = ($validateOutput -join "`n")
Assert-Contains -Text $validateText -Pattern "comparison_class: diagnostic-only" -Description "manifest diagnostic class"
Assert-Contains -Text $validateText -Pattern "conformance_claim: false" -Description "manifest claim boundary"
Assert-Contains -Text $validateText -Pattern "outputs: 8" -Description "manifest output count"
Assert-Contains -Text $validateText -Pattern "Plant Supply Side Heating Demand Rate / hourly / plant-state / eso" -Description "manifest plant-state heating output"
Assert-Contains -Text $validateText -Pattern "Pump Electricity Rate / hourly / plant-equipment / eso" -Description "manifest pump output"
Assert-Contains -Text $validateText -Pattern "Plant Load Profile Heat Transfer Rate / hourly / plant-equipment / eso" -Description "manifest load-profile output"

Write-Host "Generating v0.15 plant-loop diagnostic oracle baseline."
$baselineOutput = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $baselineOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop diagnostic baseline generation failed."
}
$baselineText = ($baselineOutput -join "`n")
Assert-Contains -Text $baselineText -Pattern "Conformance Baseline" -Description "baseline header"
Assert-Contains -Text $baselineText -Pattern "id: $CaseId" -Description "baseline case id"
Assert-Contains -Text $baselineText -Pattern "comparison_class: diagnostic-only" -Description "baseline diagnostic class"
Assert-Contains -Text $baselineText -Pattern "conformance_claim: false" -Description "baseline claim boundary"
Assert-Contains -Text $baselineText -Pattern "status: generated" -Description "baseline status"
Assert-FileExists -Path $EpJsonPath -Description "converted v0.15 epJSON"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "eplusout.eso") -Description "v0.15 EnergyPlus ESO"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "eplusout.err") -Description "v0.15 EnergyPlus ERR"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "case-expanded.toml") -Description "v0.15 expanded manifest"
Assert-CleanEnergyPlusErr -Path (Join-Path $CaseOutputRoot "eplusout.err")

Write-Host "Compiling v0.15 plant-loop diagnostic typed graph."
$compileOutput = & $cargo.Source run -p ep_cli --quiet -- compile $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $compileOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop diagnostic typed compile failed."
}
$compileText = ($compileOutput -join "`n")
Assert-Contains -Text $compileText -Pattern "TypedModel" -Description "compile header"
Assert-Contains -Text $compileText -Pattern "plant_loops: 1" -Description "PlantLoop typed count"
Assert-Contains -Text $compileText -Pattern "plant_branches: 6" -Description "Branch typed count"
Assert-Contains -Text $compileText -Pattern "plant_branch_lists: 2" -Description "BranchList typed count"
Assert-Contains -Text $compileText -Pattern "plant_connectors: 4" -Description "connector typed count"
Assert-Contains -Text $compileText -Pattern "plant_connector_lists: 2" -Description "ConnectorList typed count"
Assert-Contains -Text $compileText -Pattern "nodes: 12" -Description "plant node registry count"
Assert-Contains -Text $compileText -Pattern "Output:Variable: 8 [raw-only]" -Description "explicit plant output request count"
Assert-Contains -Text $compileText -Pattern "Pump:VariableSpeed: 1 [raw-only]" -Description "variable-speed pump boundary"
Assert-Contains -Text $compileText -Pattern "DistrictHeating:Water: 1 [raw-only]" -Description "district heating boundary"
Assert-Contains -Text $compileText -Pattern "LoadProfile:Plant: 1 [raw-only]" -Description "load-profile plant boundary"

Write-Host "Planning v0.15 plant-loop diagnostic graph."
$planOutput = & $cargo.Source run -p ep_cli --quiet -- model plan $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $planOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop diagnostic execution-plan smoke failed."
}
$planText = ($planOutput -join "`n")
Assert-Contains -Text $planText -Pattern "ExecutionPlan" -Description "plan header"
Assert-Contains -Text $planText -Pattern "plant_loop_branch_list_edges: 2" -Description "plant loop branch-list graph edges"
Assert-Contains -Text $planText -Pattern "plant_branch_list_member_edges: 6" -Description "plant branch-list member graph edges"
Assert-Contains -Text $planText -Pattern "plant_connector_list_member_edges: 4" -Description "plant connector-list member graph edges"
Assert-Contains -Text $planText -Pattern "plant_branch_component_edges: 6" -Description "plant branch component graph edges"

Write-Host "Writing v0.15 plant-loop diagnostic baseline-only report skeleton."
$reportOutput = & $cargo.Source run -p ep_cli --quiet -- conformance report-skeleton $CasePath $CaseOutputRoot $ReportRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $reportOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop diagnostic report skeleton failed."
}
$reportText = ($reportOutput -join "`n")
Assert-Contains -Text $reportText -Pattern "Conformance Report Skeleton" -Description "report header"
Assert-Contains -Text $reportText -Pattern "id: $CaseId" -Description "report case id"
Assert-Contains -Text $reportText -Pattern "series: 8" -Description "report series count"
Assert-Contains -Text $reportText -Pattern "energyplus_warnings: 0" -Description "report warning count"
Assert-Contains -Text $reportText -Pattern "energyplus_severes: 0" -Description "report severe count"
Assert-Contains -Text $reportText -Pattern "energyplus_fatals: 0" -Description "report fatal count"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: none" -Description "report tolerance boundary"
Assert-Contains -Text $reportText -Pattern "status: baseline-only" -Description "report status"

$ReportCaseRoot = Join-Path $ReportRoot $CaseId
$MarkdownReport = Join-Path $ReportCaseRoot "compare-report.md"
$SummaryReport = Join-Path $ReportCaseRoot "compare-summary.json"
Assert-FileExists -Path $MarkdownReport -Description "v0.15 baseline-only markdown report"
Assert-FileExists -Path $SummaryReport -Description "v0.15 baseline-only summary report"

$markdown = Get-Content -LiteralPath $MarkdownReport -Raw
Assert-Contains -Text $markdown -Pattern "comparison_class: diagnostic-only" -Description "markdown diagnostic class"
Assert-Contains -Text $markdown -Pattern "conformance_claim: false" -Description "markdown claim boundary"
Assert-Contains -Text $markdown -Pattern "tolerance_policy: none" -Description "markdown tolerance boundary"
Assert-Contains -Text $markdown -Pattern "status: baseline-only" -Description "markdown baseline-only status"
Assert-Contains -Text $markdown -Pattern "plant-state" -Description "markdown plant state class"
Assert-Contains -Text $markdown -Pattern "plant-equipment" -Description "markdown plant equipment class"
Assert-Contains -Text $markdown -Pattern "Plant Supply Side Heating Demand Rate" -Description "markdown plant heating demand"
Assert-Contains -Text $markdown -Pattern "Pump Electricity Rate" -Description "markdown pump electricity"
Assert-Contains -Text $markdown -Pattern "Plant Load Profile Heat Transfer Rate" -Description "markdown load-profile heat transfer"

$summary = Get-Content -LiteralPath $SummaryReport -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected v0.15 summary case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected v0.15 summary comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "v0.15 plant diagnostic summary must keep conformance_claim=false"
}
if ($summary.tolerance_policy -ne "none") {
    throw "v0.15 plant diagnostic summary must keep tolerance_policy=none"
}
if ($summary.status -ne "baseline-only") {
    throw "Unexpected v0.15 summary status: $($summary.status)"
}
if ($summary.energyplus_err.warnings -ne 0) {
    throw "v0.15 plant diagnostic summary must have zero EnergyPlus warnings"
}
if ($summary.energyplus_err.severes -ne 0) {
    throw "v0.15 plant diagnostic summary must have zero EnergyPlus severes"
}
if ($summary.energyplus_err.fatals -ne 0) {
    throw "v0.15 plant diagnostic summary must have zero EnergyPlus fatals"
}
if ($summary.requested_outputs.Count -ne 8) {
    throw "Unexpected v0.15 summary requested output count: $($summary.requested_outputs.Count)"
}

$plantStateRows = @($summary.requested_outputs | Where-Object { $_.class -eq "plant-state" })
if ($plantStateRows.Count -ne 5) {
    throw "Expected 5 v0.15 plant-state output rows, got $($plantStateRows.Count)"
}
$plantEquipmentRows = @($summary.requested_outputs | Where-Object { $_.class -eq "plant-equipment" })
if ($plantEquipmentRows.Count -ne 3) {
    throw "Expected 3 v0.15 plant-equipment output rows, got $($plantEquipmentRows.Count)"
}

foreach ($row in $summary.requested_outputs) {
    if ([int]$row.baseline_samples -ne 48) {
        throw "Unexpected sample count for $($row.key) / $($row.variable): $($row.baseline_samples)"
    }
    if ($row.status -ne "baseline-only") {
        throw "Unexpected row status for $($row.key) / $($row.variable): $($row.status)"
    }
    if ([int]$row.baseline_nonzero_count -le 0) {
        throw "Plant diagnostic baseline must include nonzero samples for $($row.key) / $($row.variable)"
    }
}

foreach ($spec in @(
    @("MAIN LOOP", "Plant Supply Side Cooling Demand Rate", "plant-state"),
    @("MAIN LOOP", "Plant Supply Side Heating Demand Rate", "plant-state"),
    @("MAIN LOOP", "Plant Supply Side Inlet Mass Flow Rate", "plant-state"),
    @("MAIN LOOP", "Plant Supply Side Outlet Temperature", "plant-state"),
    @("PUMP", "Pump Electricity Rate", "plant-equipment"),
    @("PURCHASED HEATING", "District Heating Water Rate", "plant-equipment"),
    @("LOAD PROFILE 1", "Plant Load Profile Heat Transfer Rate", "plant-equipment")
)) {
    $key = $spec[0]
    $variable = $spec[1]
    $class = $spec[2]
    $row = $summary.requested_outputs | Where-Object {
        $_.key -eq $key -and $_.variable -eq $variable -and $_.class -eq $class
    }
    if (-not $row) {
        throw "Missing v0.15 summary row for $key / $variable / $class"
    }
}

Write-Host "Plant-loop diagnostic smoke passed."
