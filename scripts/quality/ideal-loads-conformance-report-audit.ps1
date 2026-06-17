[CmdletBinding()]
param(
    [string]$CaseId = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Get-TomlString {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $pattern = '(?m)^\s*' + [regex]::Escape($Key) + '\s*=\s*"(?<value>[^"]+)"\s*$'
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        throw "$Description missing TOML string key: $Key"
    }
    return $match.Groups["value"].Value
}

function Get-TomlBool {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $pattern = '(?m)^\s*' + [regex]::Escape($Key) + '\s*=\s*(?<value>true|false)\s*$'
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        throw "$Description missing TOML bool key: $Key"
    }
    return ($match.Groups["value"].Value -eq "true")
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
}

function Get-RepoPath {
    param([Parameter(Mandatory = $true)][string]$Path)

    if ([System.IO.Path]::IsPathRooted($Path)) {
        return $Path
    }

    return Join-Path $RepoRoot ($Path -replace '/', '\')
}

$caseFiles = @(
    Get-ChildItem -LiteralPath "data\conformance_cases" -Directory |
        Where-Object { $_.Name -like "ideal_loads*" } |
        ForEach-Object { Join-Path $_.FullName "case.toml" } |
        Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } |
        Sort-Object
)

$promotedCases = @()
foreach ($caseFile in $caseFiles) {
    $text = Read-RepoText -Path $caseFile
    $id = Get-TomlString -Text $text -Key "id" -Description $caseFile
    if ($CaseId -ne "" -and $id -ne $CaseId) {
        continue
    }

    if (-not (Get-TomlBool -Text $text -Key "conformance_claim" -Description $id)) {
        continue
    }

    $comparisonClass = Get-TomlString -Text $text -Key "comparison_class" -Description $id
    if ($comparisonClass -ne "conformance") {
        throw "$id sets conformance_claim=true without comparison_class=conformance"
    }

    $gateScript = Get-TomlString -Text $text -Key "script" -Description "$id gate"
    if ($gateScript -notmatch '^scripts[\\/]dev\.cmd\s+(?<command>.+)$') {
        throw "$id gate script must use scripts/dev.cmd <command>"
    }
    $gateCommand = $Matches["command"]

    $blocking = Get-TomlBool -Text $text -Key "blocking" -Description "$id gate"
    if (-not $blocking) {
        throw "$id conformance gate must be blocking"
    }

    $reportPath = Get-TomlString -Text $text -Key "path" -Description "$id report"
    if ($reportPath -notmatch 'compare-report\.md$') {
        throw "$id report path must point to compare-report.md"
    }

    $promotedCases += [pscustomobject]@{
        Id = $id
        Command = $gateCommand
        ReportPath = $reportPath
    }
}

if ($CaseId -ne "" -and $promotedCases.Count -eq 0) {
    throw "No promoted IdealLoads conformance case matched CaseId: $CaseId"
}

if ($promotedCases.Count -eq 0) {
    throw "No promoted IdealLoads conformance cases found."
}

$devCmd = Join-Path $RepoRoot "scripts\dev.cmd"
$caseIndex = 0
foreach ($case in $promotedCases) {
    $caseIndex += 1
    Write-Host "[$caseIndex/$($promotedCases.Count)] Running $($case.Id): scripts/dev.cmd $($case.Command)"
    $commandArgs = @($case.Command -split '\s+' | Where-Object { $_ -ne "" })
    & $devCmd @commandArgs
    if ($LASTEXITCODE -ne 0) {
        throw "$($case.Id) gate failed with exit code $LASTEXITCODE"
    }

    $reportPath = Get-RepoPath -Path $case.ReportPath
    $compareRoot = Split-Path -Parent $reportPath
    $summaryPath = Join-Path $compareRoot "compare-summary.json"
    $stageSummaryPath = Join-Path $compareRoot "stage-summary.json"
    $toleranceFailuresPath = Join-Path $compareRoot "tolerance-failures.csv"

    Assert-FileExists -Path $reportPath -Description "$($case.Id) compare report"
    Assert-FileExists -Path $summaryPath -Description "$($case.Id) compare summary"
    Assert-FileExists -Path $stageSummaryPath -Description "$($case.Id) stage summary"
    Assert-FileExists -Path $toleranceFailuresPath -Description "$($case.Id) tolerance failures CSV"

    $summary = Get-Content -Encoding UTF8 -Raw -LiteralPath $summaryPath | ConvertFrom-Json
    if ($summary.case_id -ne $case.Id) {
        throw "$($case.Id) summary case_id mismatch: $($summary.case_id)"
    }
    if ($summary.comparison_class -ne "conformance") {
        throw "$($case.Id) summary comparison_class mismatch: $($summary.comparison_class)"
    }
    if ($summary.conformance_claim -ne $true) {
        throw "$($case.Id) summary must set conformance_claim=true"
    }
    if ($summary.status -ne "pass") {
        throw "$($case.Id) summary status must be pass, got $($summary.status)"
    }
    if ($summary.tolerance_failures -ne 0) {
        throw "$($case.Id) summary tolerance_failures must be 0, got $($summary.tolerance_failures)"
    }
    if (($summary.PSObject.Properties.Name -contains "meter_tolerance_failures") -and $summary.meter_tolerance_failures -ne 0) {
        throw "$($case.Id) summary meter_tolerance_failures must be 0, got $($summary.meter_tolerance_failures)"
    }

    $stageSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath $stageSummaryPath | ConvertFrom-Json
    foreach ($propertyName in @(
        "selected_purchased_air_branch",
        "declared_ideal_loads_branch",
        "inactive_branches",
        "source_map_anchor",
        "node_output_timestamp_alignment"
    )) {
        if (-not ($stageSummary.PSObject.Properties.Name -contains $propertyName)) {
            throw "$($case.Id) stage summary missing $propertyName"
        }
    }
}

Write-Host "IdealLoads conformance report audit complete."
Write-Host "  promoted_ideal_loads_cases_run: $($promotedCases.Count)"
