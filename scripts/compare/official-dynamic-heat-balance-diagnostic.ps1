[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\official-dynamic-diagnostic\26.1.0"
$CaseId = "official_1zone_uncontrolled_dynamic_diagnostic_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"

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
    (Join-Path $OracleRoot "ExampleFiles\1ZoneUncontrolled.idf"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required official dynamic diagnostic file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running official dynamic heat-balance diagnostic gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance heat-balance-diagnostic-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Official dynamic heat-balance diagnostic failed to generate."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Diagnostic Heat Balance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "status: fail" -Description "current diagnostic status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "official dynamic diagnostic summary"
Assert-FileExists -Path $reportPath -Description "official dynamic diagnostic report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "Official dynamic diagnostic must not claim conformance"
}
if ($summary.gate.blocking -ne $false) {
    throw "Official dynamic diagnostic gate must be non-blocking"
}
if ($summary.status -ne "fail") {
    throw "Official dynamic diagnostic should remain fail until the case is promoted intentionally: $($summary.status)"
}
if ($summary.samples -ne 8760) {
    throw "Expected RUN PERIOD filtered sample count 8760, got $($summary.samples)"
}
if ($summary.series_count -ne 6) {
    throw "Unexpected series_count: $($summary.series_count)"
}
if ($summary.max_abs_delta_c -le 1.0) {
    throw "Expected current official dynamic diagnostic delta to remain visible, got $($summary.max_abs_delta_c)"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Mean Air Temperature" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Mean Air Temperature series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Inside Face Conduction Heat Transfer Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Inside Face Conduction Heat Transfer Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Opaque Surface Inside Faces Conduction Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Opaque Surface Inside Faces Conduction Rate series"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Heat Balance Diagnostic Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "comparison_class: diagnostic-only" -Description "markdown comparison class"
Assert-Contains -Text $reportText -Pattern "conformance_claim: false" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "failure_reasons:" -Description "markdown failure diagnostics"
Assert-Contains -Text $reportText -Pattern "mean_abs_delta_c" -Description "markdown mean absolute delta column"
Assert-Contains -Text $reportText -Pattern "## Hourly Samples" -Description "markdown hourly sample section"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Inside Faces Conduction Rate" -Description "markdown zone conduction variable"
Assert-Contains -Text $reportText -Pattern "status: fail" -Description "markdown diagnostic status"

Write-Host "Official dynamic heat-balance diagnostic passed."
