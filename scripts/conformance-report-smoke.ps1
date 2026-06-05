[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "common.ps1")
Add-CargoBinToPath

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$BaselineRoot = Join-Path $RepoRoot ".runtime\conformance-baseline\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance-report\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\schedule_constant_001\case.toml"

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
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required report input: $path"
    }
}

Remove-RepoDirectory -Path $BaselineRoot
Remove-RepoDirectory -Path $ReportRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\setup.cmd -InstallRust first."
}

Write-Host "Generating baseline prerequisite for report skeleton."
$baselineOutput = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $BaselineRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $baselineOutput | ForEach-Object { Write-Host $_ }
    throw "Conformance baseline generation failed."
}

$BaselineCaseDir = Join-Path $BaselineRoot "schedule_constant_001"
Write-Host "Generating conformance report skeleton."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance report-skeleton $CasePath $BaselineCaseDir $ReportRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Conformance report skeleton generation failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Conformance Report Skeleton" -Description "report header"
Assert-Contains -Text $text -Pattern "id: schedule_constant_001" -Description "case id"
Assert-Contains -Text $text -Pattern "series: 1" -Description "series count"
Assert-Contains -Text $text -Pattern "tolerance_policy: none" -Description "tolerance boundary"
Assert-Contains -Text $text -Pattern "status: baseline-only" -Description "report status"

$ReportPath = Join-Path $ReportRoot "schedule_constant_001\compare-report.md"
Assert-FileExists -Path $ReportPath -Description "report skeleton"
$report = Get-Content -Raw -LiteralPath $ReportPath
Assert-Contains -Text $report -Pattern "comparison_class: smoke" -Description "report class"
Assert-Contains -Text $report -Pattern "conformance_claim: false" -Description "report claim"
Assert-Contains -Text $report -Pattern "Schedule Value" -Description "report variable"
Assert-Contains -Text $report -Pattern "baseline-only" -Description "report baseline status"

Write-Host "Conformance report smoke passed."
