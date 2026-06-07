[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$caseFiles = @(Get-ChildItem -LiteralPath (Join-Path $RepoRoot "data\conformance_cases") -Recurse -Filter "case.toml" | Sort-Object FullName)
if ($caseFiles.Count -eq 0) {
    throw "No conformance case manifests found."
}

$requiredSchema = "schema_v2: rusted-energyplus.case-manifest.v2"
$requiredStatus = "status: valid"
$caseCount = 0
$tierCounts = @{}
$conformanceCount = 0
$diagnosticOrBaselineCount = 0

foreach ($caseFile in $caseFiles) {
    $relative = $caseFile.FullName.Substring($RepoRoot.Length).TrimStart("\", "/")
    Write-Host "Validating v2 manifest: $relative"
    $output = & $cargo.Source run -p ep_cli --quiet -- conformance validate-case-v2 $relative 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Manifest v2 validation failed: $relative"
    }

    $text = ($output | Out-String)
    if (-not $text.Contains($requiredSchema)) {
        throw "Missing v2 schema marker for $relative"
    }
    if (-not $text.Contains($requiredStatus)) {
        throw "Missing valid status marker for $relative"
    }
    if ($text.Contains("conformance_claim: true")) {
        if (-not $text.Contains("level=conformance")) {
            throw "Conformance case lacks a conformance-level output: $relative"
        }
        $conformanceCount += 1
    }
    else {
        if ($text.Contains("level=conformance")) {
            throw "Non-conformance case has conformance-level output: $relative"
        }
        $diagnosticOrBaselineCount += 1
    }

    $tierMatch = [regex]::Match($text, "(?m)^\s*tier:\s*(?<tier>[ABC])\s*$")
    if (-not $tierMatch.Success) {
        throw "Missing tier marker for $relative"
    }
    $tier = $tierMatch.Groups["tier"].Value
    if (-not $tierCounts.ContainsKey($tier)) {
        $tierCounts[$tier] = 0
    }
    $tierCounts[$tier] += 1
    $caseCount += 1
}

Write-Host "Manifest v2 validation"
Write-Host "  cases: $caseCount"
foreach ($tier in @("A", "B", "C")) {
    $count = 0
    if ($tierCounts.ContainsKey($tier)) {
        $count = $tierCounts[$tier]
    }
    Write-Host "  tier_${tier}: $count"
}
Write-Host "  conformance_cases: $conformanceCount"
Write-Host "  baseline_or_diagnostic_cases: $diagnosticOrBaselineCount"
Write-Host "  schema: rusted-energyplus.case-manifest.v2"
Write-Host "  status: valid"
