[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\air-side-node-diagnostic\26.1.0"
$ReportRoot = Join-Path $OutputRoot "report-skeleton"
$ProjectionRoot = Join-Path $OutputRoot "node-state-projection"
$CaseId = "air_side_node_diagnostic_001"
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

function Assert-TextOrder {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string[]]$Patterns,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $offset = -1
    foreach ($pattern in $Patterns) {
        $index = $Text.IndexOf($pattern, $offset + 1, [System.StringComparison]::Ordinal)
        if ($index -lt 0) {
            Write-Host $Text
            throw "Missing ordered $Description step: $pattern"
        }
        $offset = $index
    }
    Write-Host "OK ordered $Description`: $($Patterns -join ' -> ')"
}

function Assert-CleanEnergyPlusErr {
    param([Parameter(Mandatory = $true)][string]$Path)
    $errText = Get-Content -LiteralPath $Path -Raw
    $warningLines = @($errText -split "`r?`n" | Where-Object { $_.Contains("** Warning **") })
    $severeLines = @($errText -split "`r?`n" | Where-Object { $_.Contains("** Severe") })
    $fatalLines = @($errText -split "`r?`n" | Where-Object { $_.Contains("** Fatal") })
    if ($warningLines.Count -gt 0 -or $severeLines.Count -gt 0 -or $fatalLines.Count -gt 0) {
        Write-Host $errText
        throw "v0.11 air-side node diagnostic baseline must not rely on EnergyPlus warning/severe/fatal auto-fixes."
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
        throw "Missing required air-side node diagnostic input: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Validating v0.11 air-side node diagnostic case manifest."
$validateOutput = & $cargo.Source run -p ep_cli --quiet -- conformance validate-case $CasePath 2>&1
if ($LASTEXITCODE -ne 0) {
    $validateOutput | ForEach-Object { Write-Host $_ }
    throw "Air-side node diagnostic case manifest validation failed."
}
$validateText = ($validateOutput -join "`n")
Assert-Contains -Text $validateText -Pattern "comparison_class: diagnostic-only" -Description "manifest diagnostic class"
Assert-Contains -Text $validateText -Pattern "conformance_claim: false" -Description "manifest claim boundary"
Assert-Contains -Text $validateText -Pattern "outputs: 13" -Description "manifest output count"
Assert-Contains -Text $validateText -Pattern "System Node Temperature / hourly / node-state / eso" -Description "manifest node temperature output"
Assert-Contains -Text $validateText -Pattern "System Node Mass Flow Rate / hourly / node-state / eso" -Description "manifest node flow output"
Assert-Contains -Text $validateText -Pattern "System Node Humidity Ratio / hourly / node-state / eso" -Description "manifest node humidity output"

Write-Host "Generating v0.11 air-side node diagnostic oracle baseline."
$baselineOutput = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $baselineOutput | ForEach-Object { Write-Host $_ }
    throw "Air-side node diagnostic baseline generation failed."
}
$baselineText = ($baselineOutput -join "`n")
Assert-Contains -Text $baselineText -Pattern "Conformance Baseline" -Description "baseline header"
Assert-Contains -Text $baselineText -Pattern "id: $CaseId" -Description "baseline case id"
Assert-Contains -Text $baselineText -Pattern "comparison_class: diagnostic-only" -Description "baseline diagnostic class"
Assert-Contains -Text $baselineText -Pattern "conformance_claim: false" -Description "baseline claim boundary"
Assert-Contains -Text $baselineText -Pattern "status: generated" -Description "baseline status"
Assert-FileExists -Path $EpJsonPath -Description "converted v0.11 epJSON"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "eplusout.eso") -Description "v0.11 EnergyPlus ESO"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "eplusout.err") -Description "v0.11 EnergyPlus ERR"
Assert-FileExists -Path (Join-Path $CaseOutputRoot "case-expanded.toml") -Description "v0.11 expanded manifest"
Assert-CleanEnergyPlusErr -Path (Join-Path $CaseOutputRoot "eplusout.err")

Write-Host "Compiling v0.11 air-side node diagnostic typed model."
$compileOutput = & $cargo.Source run -p ep_cli --quiet -- compile $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $compileOutput | ForEach-Object { Write-Host $_ }
    throw "Air-side node diagnostic typed compile failed."
}
$compileText = ($compileOutput -join "`n")
Assert-Contains -Text $compileText -Pattern "TypedModel" -Description "compile header"
Assert-Contains -Text $compileText -Pattern "nodes: 3" -Description "node registry typed count"
Assert-Contains -Text $compileText -Pattern "node_lists: 1" -Description "NodeList typed count"
Assert-Contains -Text $compileText -Pattern "ZoneHVAC:IdealLoadsAirSystem: 1 [typed]" -Description "IdealLoads coverage"
Assert-Contains -Text $compileText -Pattern "ZoneHVAC:EquipmentConnections: 1 [typed]" -Description "equipment connection coverage"
Assert-Contains -Text $compileText -Pattern "NodeList: 1 [typed]" -Description "NodeList coverage"

Write-Host "Planning v0.11 air-side node diagnostic graph."
$planOutput = & $cargo.Source run -p ep_cli --quiet -- model plan $EpJsonPath 2>&1
if ($LASTEXITCODE -ne 0) {
    $planOutput | ForEach-Object { Write-Host $_ }
    throw "Air-side node diagnostic execution-plan smoke failed."
}
$planText = ($planOutput -join "`n")
Assert-Contains -Text $planText -Pattern "ExecutionPlan" -Description "plan header"
Assert-Contains -Text $planText -Pattern "node_list_member_edges: 1" -Description "NodeList member graph edge"
Assert-Contains -Text $planText -Pattern "ideal_loads_supply_node_edges: 1" -Description "IdealLoads supply node graph edge"
Assert-Contains -Text $planText -Pattern "zone_air_node_edges: 1" -Description "zone air node graph edge"
Assert-TextOrder -Text $planText -Patterns @(
    "EvaluateZoneThermostat(0)",
    "SolveZone(0)",
    "EvaluateIdealLoadsAirSystem(0)"
) -Description "zone thermostat/solve/IdealLoads execution"

Write-Host "Writing v0.11 air-side node diagnostic baseline-only report skeleton."
$reportOutput = & $cargo.Source run -p ep_cli --quiet -- conformance report-skeleton $CasePath $CaseOutputRoot $ReportRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $reportOutput | ForEach-Object { Write-Host $_ }
    throw "Air-side node diagnostic report skeleton failed."
}
$reportText = ($reportOutput -join "`n")
Assert-Contains -Text $reportText -Pattern "Conformance Report Skeleton" -Description "report header"
Assert-Contains -Text $reportText -Pattern "id: $CaseId" -Description "report case id"
Assert-Contains -Text $reportText -Pattern "series: 13" -Description "report series count"
Assert-Contains -Text $reportText -Pattern "energyplus_warnings: 0" -Description "report warning count"
Assert-Contains -Text $reportText -Pattern "energyplus_severes: 0" -Description "report severe count"
Assert-Contains -Text $reportText -Pattern "energyplus_fatals: 0" -Description "report fatal count"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: none" -Description "report tolerance boundary"
Assert-Contains -Text $reportText -Pattern "status: baseline-only" -Description "report status"

$ReportCaseRoot = Join-Path $ReportRoot $CaseId
$MarkdownReport = Join-Path $ReportCaseRoot "compare-report.md"
$SummaryReport = Join-Path $ReportCaseRoot "compare-summary.json"
Assert-FileExists -Path $MarkdownReport -Description "v0.11 baseline-only markdown report"
Assert-FileExists -Path $SummaryReport -Description "v0.11 baseline-only summary report"

$markdown = Get-Content -LiteralPath $MarkdownReport -Raw
Assert-Contains -Text $markdown -Pattern "comparison_class: diagnostic-only" -Description "markdown diagnostic class"
Assert-Contains -Text $markdown -Pattern "conformance_claim: false" -Description "markdown claim boundary"
Assert-Contains -Text $markdown -Pattern "tolerance_policy: none" -Description "markdown tolerance boundary"
Assert-Contains -Text $markdown -Pattern "status: baseline-only" -Description "markdown baseline-only status"
Assert-Contains -Text $markdown -Pattern "node-state" -Description "markdown node state class"
Assert-Contains -Text $markdown -Pattern "System Node Temperature" -Description "markdown node temperature"
Assert-Contains -Text $markdown -Pattern "System Node Humidity Ratio" -Description "markdown node humidity"
Assert-Contains -Text $markdown -Pattern "System Node Mass Flow Rate" -Description "markdown node mass flow"

$summary = Get-Content -LiteralPath $SummaryReport -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected v0.11 summary case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected v0.11 summary comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "v0.11 node diagnostic summary must keep conformance_claim=false"
}
if ($summary.tolerance_policy -ne "none") {
    throw "v0.11 node diagnostic summary must keep tolerance_policy=none"
}
if ($summary.status -ne "baseline-only") {
    throw "Unexpected v0.11 summary status: $($summary.status)"
}
if ($summary.energyplus_err.warnings -ne 0) {
    throw "v0.11 node diagnostic summary must have zero EnergyPlus warnings"
}
if ($summary.energyplus_err.severes -ne 0) {
    throw "v0.11 node diagnostic summary must have zero EnergyPlus severes"
}
if ($summary.energyplus_err.fatals -ne 0) {
    throw "v0.11 node diagnostic summary must have zero EnergyPlus fatals"
}
if ($summary.requested_outputs.Count -ne 13) {
    throw "Unexpected v0.11 summary requested output count: $($summary.requested_outputs.Count)"
}

$nodeRows = @($summary.requested_outputs | Where-Object { $_.class -eq "node-state" })
if ($nodeRows.Count -ne 9) {
    throw "Expected 9 v0.11 node-state output rows, got $($nodeRows.Count)"
}
foreach ($node in @("ZONE ONE INLET", "ZONE ONE AIR NODE", "ZONE ONE RETURN")) {
    foreach ($variable in @("System Node Temperature", "System Node Humidity Ratio", "System Node Mass Flow Rate")) {
        $row = $nodeRows | Where-Object { $_.key -eq $node -and $_.variable -eq $variable }
        if (-not $row) {
            throw "Missing v0.11 node-state summary row for $node / $variable"
        }
        if ([int]$row.baseline_samples -ne 24) {
            throw "Unexpected sample count for $node / $variable`: $($row.baseline_samples)"
        }
        if ([int]$row.baseline_nonzero_count -le 0) {
            throw "Node-state baseline must include nonzero samples for $node / $variable"
        }
    }
}

Write-Host "Writing v0.11 Rust air-side node-state projection."
$projectionOutput = & $cargo.Source run -p ep_cli --quiet -- run node-state-projection $EpJsonPath $ProjectionRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $projectionOutput | ForEach-Object { Write-Host $_ }
    throw "Air-side node diagnostic Rust node-state projection failed."
}
$projectionText = ($projectionOutput -join "`n")
Assert-Contains -Text $projectionText -Pattern "Node State Projection" -Description "projection header"
Assert-Contains -Text $projectionText -Pattern "runtime_class: ideal-loads-node-state-projection" -Description "projection runtime class"
Assert-Contains -Text $projectionText -Pattern "comparison_class: diagnostic-only" -Description "projection diagnostic class"
Assert-Contains -Text $projectionText -Pattern "conformance_claim: false" -Description "projection claim boundary"
Assert-Contains -Text $projectionText -Pattern "algorithm_parity: false" -Description "projection algorithm boundary"
Assert-Contains -Text $projectionText -Pattern "tolerance_policy: none" -Description "projection tolerance boundary"
Assert-Contains -Text $projectionText -Pattern "nodes: 3" -Description "projection node count"
Assert-Contains -Text $projectionText -Pattern "state_nodes: 3" -Description "projection state node count"
Assert-Contains -Text $projectionText -Pattern "samples: 24" -Description "projection sample count"
Assert-Contains -Text $projectionText -Pattern "series: 9" -Description "projection series count"
Assert-Contains -Text $projectionText -Pattern "status: projected" -Description "projection status"

$ProjectionMarkdown = Join-Path $ProjectionRoot "node-state-summary.md"
$ProjectionSummary = Join-Path $ProjectionRoot "node-state-summary.json"
Assert-FileExists -Path $ProjectionMarkdown -Description "v0.11 Rust node-state markdown summary"
Assert-FileExists -Path $ProjectionSummary -Description "v0.11 Rust node-state JSON summary"

$projectionMarkdownText = Get-Content -LiteralPath $ProjectionMarkdown -Raw
Assert-Contains -Text $projectionMarkdownText -Pattern "comparison_class: diagnostic-only" -Description "projection markdown diagnostic class"
Assert-Contains -Text $projectionMarkdownText -Pattern "conformance_claim: false" -Description "projection markdown claim boundary"
Assert-Contains -Text $projectionMarkdownText -Pattern "status: projected" -Description "projection markdown status"
Assert-Contains -Text $projectionMarkdownText -Pattern "ZONE ONE INLET" -Description "projection markdown inlet node"
Assert-Contains -Text $projectionMarkdownText -Pattern "System Node Mass Flow Rate" -Description "projection markdown mass flow"

$projection = Get-Content -LiteralPath $ProjectionSummary -Raw | ConvertFrom-Json
if ($projection.comparison_class -ne "diagnostic-only") {
    throw "Unexpected v0.11 projection comparison_class: $($projection.comparison_class)"
}
if ($projection.conformance_claim -ne $false) {
    throw "v0.11 projection must keep conformance_claim=false"
}
if ($projection.algorithm_parity -ne $false) {
    throw "v0.11 projection must keep algorithm_parity=false"
}
if ($projection.tolerance_policy -ne "none") {
    throw "v0.11 projection must keep tolerance_policy=none"
}
if ($projection.status -ne "projected") {
    throw "Unexpected v0.11 projection status: $($projection.status)"
}
if ([int]$projection.nodes -ne 3) {
    throw "Unexpected v0.11 projection node count: $($projection.nodes)"
}
if ([int]$projection.state_nodes -ne 3) {
    throw "Unexpected v0.11 projection state node count: $($projection.state_nodes)"
}
if ([int]$projection.samples -ne 24) {
    throw "Unexpected v0.11 projection sample count: $($projection.samples)"
}
if ([int]$projection.series -ne 9) {
    throw "Unexpected v0.11 projection series count: $($projection.series)"
}
if ($projection.node_order.Count -ne 3) {
    throw "Unexpected v0.11 projection node order count: $($projection.node_order.Count)"
}
if ($projection.result_series.Count -ne 9) {
    throw "Unexpected v0.11 projection result series count: $($projection.result_series.Count)"
}

foreach ($node in @("ZONE ONE INLET", "ZONE ONE AIR NODE", "ZONE ONE RETURN")) {
    foreach ($variable in @("System Node Temperature", "System Node Humidity Ratio", "System Node Mass Flow Rate")) {
        $row = $projection.result_series | Where-Object { $_.key -eq $node -and $_.variable -eq $variable }
        if (-not $row) {
            throw "Missing v0.11 Rust projection row for $node / $variable"
        }
        if ([int]$row.samples -ne 24) {
            throw "Unexpected Rust projection sample count for $node / $variable`: $($row.samples)"
        }
        if ([int]$row.nonzero_count -ne 24) {
            throw "Rust projection must include nonzero hourly samples for $node / $variable"
        }
        if ($row.status -ne "projected") {
            throw "Unexpected Rust projection status for $node / $variable`: $($row.status)"
        }
    }
}

Write-Host "Air-side node diagnostic smoke passed."
