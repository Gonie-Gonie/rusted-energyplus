[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$BaselineRoot = Join-Path $RepoRoot ".runtime\conformance-baseline\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance-report\26.1.0"
$ScheduleCasePath = Join-Path $RepoRoot "data\conformance_cases\schedule_constant_001\case.toml"
$WeatherCasePath = Join-Path $RepoRoot "data\conformance_cases\weather_fields_001\case.toml"

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

function Invoke-ReportSkeletonCase {
    param(
        [Parameter(Mandatory = $true)][string]$CasePath,
        [Parameter(Mandatory = $true)][string]$CaseId,
        [Parameter(Mandatory = $true)][int]$ExpectedSeries,
        [Parameter(Mandatory = $true)][string]$ExpectedVariable
    )

    Write-Host "Generating baseline prerequisite for report skeleton: $CaseId"
    $baselineOutput = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $BaselineRoot 2>&1
    if ($LASTEXITCODE -ne 0) {
        $baselineOutput | ForEach-Object { Write-Host $_ }
        throw "Conformance baseline generation failed for $CaseId."
    }

    $baselineCaseDir = Join-Path $BaselineRoot $CaseId
    Write-Host "Generating conformance report skeleton: $CaseId"
    $output = & $cargo.Source run -p ep_cli --quiet -- conformance report-skeleton $CasePath $baselineCaseDir $ReportRoot 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Conformance report skeleton generation failed for $CaseId."
    }

    $text = ($output -join "`n")
    Assert-Contains -Text $text -Pattern "Conformance Report Skeleton" -Description "$CaseId report header"
    Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "$CaseId case id"
    Assert-Contains -Text $text -Pattern "series: $ExpectedSeries" -Description "$CaseId series count"
    Assert-Contains -Text $text -Pattern "tolerance_policy: none" -Description "$CaseId tolerance boundary"
    Assert-Contains -Text $text -Pattern "status: baseline-only" -Description "$CaseId report status"

    $reportPath = Join-Path $ReportRoot "$CaseId\compare-report.md"
    Assert-FileExists -Path $reportPath -Description "$CaseId report skeleton"
    $report = Get-Content -Raw -LiteralPath $reportPath
    Assert-Contains -Text $report -Pattern "comparison_class: smoke" -Description "$CaseId report class"
    Assert-Contains -Text $report -Pattern "conformance_claim: false" -Description "$CaseId report claim"
    Assert-Contains -Text $report -Pattern $ExpectedVariable -Description "$CaseId report variable"
    Assert-Contains -Text $report -Pattern "baseline-only" -Description "$CaseId report baseline status"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $ScheduleCasePath,
    $WeatherCasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required report input: $path"
    }
}

Remove-RepoDirectory -Path $BaselineRoot
Remove-RepoDirectory -Path $ReportRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Invoke-ReportSkeletonCase -CasePath $ScheduleCasePath -CaseId "schedule_constant_001" -ExpectedSeries 1 -ExpectedVariable "Schedule Value"
Invoke-ReportSkeletonCase -CasePath $WeatherCasePath -CaseId "weather_fields_001" -ExpectedSeries 6 -ExpectedVariable "Site Wind Direction"

Write-Host "Conformance report smoke passed."
