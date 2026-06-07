[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$BaselineRoot = Join-Path $RepoRoot ".runtime\official-baseline\26.1.0"
$ReportRoot = Join-Path $BaselineRoot "report-skeleton"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\official_1zone_uncontrolled_baseline_001\case.toml"

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
        throw "Missing required official baseline input: $path"
    }
}

Remove-RepoDirectory -Path $BaselineRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating official baseline with manifest output injection."
$baselineOutput = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $BaselineRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $baselineOutput | ForEach-Object { Write-Host $_ }
    throw "Official baseline generation failed."
}

$baselineText = ($baselineOutput -join "`n")
Assert-Contains -Text $baselineText -Pattern "Conformance Baseline" -Description "baseline header"
Assert-Contains -Text $baselineText -Pattern "id: official_1zone_uncontrolled_baseline_001" -Description "case id"
Assert-Contains -Text $baselineText -Pattern "source_kind: energy-plus-examplefile" -Description "official source kind"
Assert-Contains -Text $baselineText -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $baselineText -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $baselineText -Pattern "injected_outputs: 2" -Description "injected output count"
Assert-Contains -Text $baselineText -Pattern "injected_meters: 0" -Description "injected meter count"
Assert-Contains -Text $baselineText -Pattern "status: generated" -Description "baseline status"

$CaseOutput = Join-Path $BaselineRoot "official_1zone_uncontrolled_baseline_001"
$stagedIdf = Join-Path $CaseOutput "input.idf"
$expandedManifest = Join-Path $CaseOutput "case-expanded.toml"
Assert-FileExists -Path $stagedIdf -Description "staged official IDF"
Assert-FileExists -Path (Join-Path $CaseOutput "input.epJSON") -Description "converted official epJSON"
Assert-FileExists -Path (Join-Path $CaseOutput "eplusout.eso") -Description "official EnergyPlus ESO"
Assert-FileExists -Path (Join-Path $CaseOutput "eplusout.err") -Description "official EnergyPlus ERR"
Assert-FileExists -Path $expandedManifest -Description "expanded official manifest"

$stagedText = Get-Content -Raw -LiteralPath $stagedIdf
Assert-Contains -Text $stagedText -Pattern "eplus-rs output request injection begin" -Description "output injection marker"
Assert-Contains -Text $stagedText -Pattern "Site Wind Speed" -Description "weather output injection"
Assert-Contains -Text $stagedText -Pattern "Zone Total Internal Convective Heating Rate" -Description "internal-gain output injection"

$expanded = Get-Content -Raw -LiteralPath $expandedManifest
Assert-Contains -Text $expanded -Pattern 'schema = "rusted-energyplus.baseline-expanded.v1"' -Description "expanded manifest schema"
Assert-Contains -Text $expanded -Pattern 'schema = "rusted-energyplus.output-injection.v1"' -Description "injection schema"
Assert-Contains -Text $expanded -Pattern "staged_idf_contains_manifest_requests = true" -Description "injection staging policy"
Assert-Contains -Text $expanded -Pattern "outputs = 2" -Description "expanded output count"
Assert-Contains -Text $expanded -Pattern "meters = 0" -Description "expanded meter count"

Write-Host "Generating official baseline report skeleton."
$reportOutput = & $cargo.Source run -p ep_cli --quiet -- conformance report-skeleton $CasePath $CaseOutput $ReportRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $reportOutput | ForEach-Object { Write-Host $_ }
    throw "Official report skeleton generation failed."
}

$reportText = ($reportOutput -join "`n")
Assert-Contains -Text $reportText -Pattern "Conformance Report Skeleton" -Description "report header"
Assert-Contains -Text $reportText -Pattern "series: 2" -Description "baseline series count"
Assert-Contains -Text $reportText -Pattern "status: baseline-only" -Description "report status"

$reportPath = Join-Path $ReportRoot "official_1zone_uncontrolled_baseline_001\compare-report.md"
$summaryPath = Join-Path $ReportRoot "official_1zone_uncontrolled_baseline_001\compare-summary.json"
Assert-FileExists -Path $reportPath -Description "official baseline report"
Assert-FileExists -Path $summaryPath -Description "official baseline summary"
$report = Get-Content -Raw -LiteralPath $reportPath
Assert-Contains -Text $report -Pattern "Site Wind Speed" -Description "report weather row"
Assert-Contains -Text $report -Pattern "Zone Total Internal Convective Heating Rate" -Description "report internal-gain row"
Assert-Contains -Text $report -Pattern "baseline-only" -Description "report baseline boundary"

Write-Host "Official baseline smoke passed."
