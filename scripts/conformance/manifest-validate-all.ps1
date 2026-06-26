[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
. (Join-Path $ScriptsRoot "lib\python.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$python = $null
$portablePython = Get-PortablePythonExe
if (Test-Path -LiteralPath $portablePython -PathType Leaf) {
    $python = $portablePython
}
else {
    $pythonCommand = Get-Command python -ErrorAction SilentlyContinue
    if ($null -ne $pythonCommand) {
        $python = $pythonCommand.Source
    }
}
if ($null -eq $python) {
    throw "Python 3.11+ was not found. Run .\scripts\dev.cmd setup first."
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
    $hasConformanceEvidence = $text.Contains("level=conformance") -or [regex]::IsMatch($text, "(?m)^\s+.+ /\s*conformance\s*$")
    if ($text.Contains("conformance_claim: true")) {
        if (-not $hasConformanceEvidence) {
            throw "Conformance case lacks a conformance-level output or meter: $relative"
        }
        $conformanceCount += 1
    }
    else {
        if ($hasConformanceEvidence) {
            throw "Non-conformance case has conformance-level output or meter: $relative"
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

$claimSourceScript = Join-Path $RepoRoot "tools\docs\validate_claim_sources.py"
if (-not (Test-Path -LiteralPath $claimSourceScript -PathType Leaf)) {
    throw "Missing claim source validator: $claimSourceScript"
}

& $python $claimSourceScript --repo-root $RepoRoot
if ($LASTEXITCODE -ne 0) {
    throw "Claim source validation failed with exit code $LASTEXITCODE"
}
