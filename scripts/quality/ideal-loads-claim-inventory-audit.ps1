[CmdletBinding()]
param()

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

function Assert-TextMatches {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Text -notmatch $Pattern) {
        throw "$Description missing"
    }
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

function Assert-ConformanceBlocksHaveTolerances {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$CaseId
    )

    $blocks = [regex]::Matches($Text, '(?ms)^\[\[(?<kind>outputs|meters)\]\]\s*(?<body>.*?)(?=^\[|\z)')
    $conformanceBlockCount = 0
    foreach ($block in $blocks) {
        $body = $block.Groups["body"].Value
        if ($body -notmatch '(?m)^\s*level\s*=\s*"conformance"\s*$') {
            continue
        }

        $conformanceBlockCount += 1
        $kind = $block.Groups["kind"].Value
        if ($body -notmatch '(?m)^\s*abs_tol\s*=') {
            throw "$CaseId conformance $kind block is missing abs_tol"
        }
        if ($body -notmatch '(?m)^\s*rmse_tol\s*=') {
            throw "$CaseId conformance $kind block is missing rmse_tol"
        }
        if ($kind -eq "outputs" -and $body -notmatch '(?m)^\s*frequency\s*=\s*"detailed"\s*$') {
            throw "$CaseId conformance output must use detailed frequency"
        }
    }

    if ($conformanceBlockCount -eq 0) {
        throw "$CaseId claims conformance but has no conformance output or meter blocks"
    }

    return $conformanceBlockCount
}

function Get-DevCommandScriptPath {
    param(
        [Parameter(Mandatory = $true)][string]$DevText,
        [Parameter(Mandatory = $true)][string]$Command
    )

    $pattern = '(?ms)^\s*"' + [regex]::Escape($Command) + '"\s*=\s*@\{\s*Path\s*=\s*"(?<path>[^"]+)"'
    $match = [regex]::Match($DevText, $pattern)
    if (-not $match.Success) {
        throw "Dev command missing script path: $Command"
    }

    return Join-Path "scripts" $match.Groups["path"].Value
}

function Assert-ConformanceGateReportMetadataGuards {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$CaseId
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$CaseId gate script missing: $Path"
    }

    $scriptText = Read-RepoText -Path $Path
    $requiredMarkers = @(
        "compare-summary.json",
        "compare-report.md",
        "stage-summary.json",
        "tolerance-failures.csv",
        "comparison_class",
        "conformance_claim",
        "status",
        "tolerance_failures",
        "conformanceRows",
        "diagnostic",
        "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat",
        "selected_purchased_air_branch",
        "declared_ideal_loads_branch",
        "inactive_branches",
        "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md",
        "node_output_timestamp_alignment: timestamp",
        "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir"
    )

    foreach ($marker in $requiredMarkers) {
        Assert-TextMatches -Text $scriptText -Pattern ([regex]::Escape($marker)) -Description "$CaseId gate metadata guard: $marker"
    }
}

$readmeText = Read-RepoText -Path "README.md"
$currentStatusText = Read-RepoText -Path "docs\src\current\current-status.md"
$variableCoverageText = Read-RepoText -Path "specs\variable_coverage.toml"
$devText = Read-RepoText -Path "scripts\dev.ps1"

$caseFiles = @(
    Get-ChildItem -LiteralPath "data\conformance_cases" -Directory |
        Where-Object { $_.Name -like "ideal_loads*" } |
        ForEach-Object { Join-Path $_.FullName "case.toml" } |
        Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } |
        Sort-Object
)

if ($caseFiles.Count -eq 0) {
    throw "No IdealLoads case manifests found."
}

$promotedCases = @()
$diagnosticOrBaselineCount = 0
$reportMetadataGuardCount = 0
foreach ($caseFile in $caseFiles) {
    $text = Read-RepoText -Path $caseFile
    $caseId = Get-TomlString -Text $text -Key "id" -Description $caseFile
    $comparisonClass = Get-TomlString -Text $text -Key "comparison_class" -Description $caseId
    $conformanceClaim = Get-TomlBool -Text $text -Key "conformance_claim" -Description $caseId
    $hasConformanceLevel = [regex]::IsMatch($text, '(?m)^\s*level\s*=\s*"conformance"\s*$')

    if ($conformanceClaim) {
        if ($comparisonClass -ne "conformance") {
            throw "$caseId sets conformance_claim=true without comparison_class=conformance"
        }

        $conformanceBlockCount = Assert-ConformanceBlocksHaveTolerances -Text $text -CaseId $caseId
        Assert-TextMatches -Text $text -Pattern '(?m)^\s*path\s*=\s*".*compare-report\.md"\s*$' -Description "$caseId compare report path"
        Assert-TextMatches -Text $text -Pattern '(?m)^\s*blocking\s*=\s*true\s*$' -Description "$caseId blocking gate"

        $gateMatch = [regex]::Match($text, '(?m)^\s*script\s*=\s*"scripts[\\/]dev\.cmd\s+(?<command>[^"]+)"\s*$')
        if (-not $gateMatch.Success) {
            throw "$caseId gate script must use scripts/dev.cmd <command>"
        }
        $gateCommand = $gateMatch.Groups["command"].Value
        $gateCommandPattern = '(?m)^\s*"' + [regex]::Escape($gateCommand) + '"\s*=\s*@\{'
        Assert-TextMatches -Text $devText -Pattern $gateCommandPattern -Description "$caseId dev gate command $gateCommand"
        $gateScriptPath = Get-DevCommandScriptPath -DevText $devText -Command $gateCommand
        Assert-ConformanceGateReportMetadataGuards -Path $gateScriptPath -CaseId $caseId
        $reportMetadataGuardCount += 1

        $casePattern = [regex]::Escape($caseId)
        Assert-TextMatches -Text $readmeText -Pattern $casePattern -Description "$caseId README claim inventory"
        Assert-TextMatches -Text $currentStatusText -Pattern $casePattern -Description "$caseId current-status claim inventory"

        $promotedCases += [pscustomobject]@{
            Id = $caseId
            Blocks = $conformanceBlockCount
            Gate = $gateCommand
        }
    }
    else {
        if ($hasConformanceLevel) {
            throw "$caseId has conformance-level output or meter blocks without conformance_claim=true"
        }
        $diagnosticOrBaselineCount += 1
    }
}

$promotedCaseIds = @{}
foreach ($case in $promotedCases) {
    $promotedCaseIds[$case.Id] = $true
}

$coverageBlocks = [regex]::Matches($variableCoverageText, '(?ms)^\[\[variable\]\]\s*(?<body>.*?)(?=^\[|\z)')
$idealLoadsCoverageRefs = 0
foreach ($block in $coverageBlocks) {
    $body = $block.Groups["body"].Value
    $status = Get-TomlString -Text $body -Key "status" -Description "variable coverage block"
    if ($status -ne "conformance") {
        continue
    }

    $firstCase = Get-TomlString -Text $body -Key "first_case" -Description "conformance variable coverage block"
    $firstEvidence = Get-TomlString -Text $body -Key "first_evidence" -Description "conformance variable coverage block"
    foreach ($caseRef in @($firstCase, $firstEvidence)) {
        if ($caseRef -notlike "ideal_loads*") {
            continue
        }
        $idealLoadsCoverageRefs += 1
        if (-not $promotedCaseIds.ContainsKey($caseRef)) {
            throw "Variable coverage references non-promoted IdealLoads conformance case: $caseRef"
        }
    }
}

$broadExclusionPatterns = @(
    @("full IdealLoads", "full IdealLoads exclusion"),
    @("broad HVAC", "broad HVAC exclusion"),
    @("broad meter conformance", "broad meter exclusion"),
    @("annual meter aggregation", "annual meter aggregation exclusion"),
    @("broader DCV combinations", "broader DCV exclusion"),
    @("CO2 contaminant-balance/concentration conformance", "CO2 contaminant boundary")
)

foreach ($entry in $broadExclusionPatterns) {
    $pattern = [regex]::Escape($entry[0])
    Assert-TextMatches -Text $readmeText -Pattern $pattern -Description "README $($entry[1])"
    Assert-TextMatches -Text $currentStatusText -Pattern $pattern -Description "current-status $($entry[1])"
}

Write-Host "IdealLoads claim inventory audit complete."
Write-Host "  promoted_ideal_loads_cases: $($promotedCases.Count)"
Write-Host "  diagnostic_or_baseline_ideal_loads_cases: $diagnosticOrBaselineCount"
Write-Host "  conformance_blocks_checked: $(($promotedCases | Measure-Object -Property Blocks -Sum).Sum)"
Write-Host "  report_metadata_guards_checked: $reportMetadataGuardCount"
Write-Host "  variable_coverage_ideal_loads_refs: $idealLoadsCoverageRefs"
