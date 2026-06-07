[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\static-model-conformance\26.1.0"
$CaseId = "official_1zone_static_model_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"

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

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "ExampleFiles\1ZoneUncontrolled.idf"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required static model input"
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating official ExampleFile static model conformance report."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance static-model-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Static model conformance report failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Static Model Conformance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "outputs: 19" -Description "output count"
Assert-Contains -Text $text -Pattern "conformance_outputs: 19" -Description "conformance output count"
Assert-Contains -Text $text -Pattern "status: pass" -Description "status"

$caseRoot = Join-Path $OutputRoot $CaseId
$oracleRoot = Join-Path $caseRoot "oracle"
$compareRoot = Join-Path $caseRoot "compare"
$stagedIdf = Join-Path $oracleRoot "input.idf"
$reportPath = Join-Path $compareRoot "compare-report.md"
$summaryPath = Join-Path $compareRoot "compare-summary.json"

Assert-FileExists -Path $stagedIdf -Description "staged official IDF"
Assert-FileExists -Path (Join-Path $oracleRoot "eplusout.eio") -Description "oracle EIO"
Assert-FileExists -Path (Join-Path $oracleRoot "input.epJSON") -Description "staged epJSON"
Assert-FileExists -Path $reportPath -Description "static model markdown report"
Assert-FileExists -Path $summaryPath -Description "static model JSON summary"

$stagedText = Get-Content -Raw -LiteralPath $stagedIdf
Assert-Contains -Text $stagedText -Pattern "eplus-rs output request injection begin" -Description "output injection marker"
Assert-Contains -Text $stagedText -Pattern "Output:Surfaces:List,Details;" -Description "surface detail injection"

$reportText = Get-Content -Raw -LiteralPath $reportPath
Assert-Contains -Text $reportText -Pattern "Static Model Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "claim_boundary: static EIO model evidence only" -Description "claim boundary"
Assert-Contains -Text $reportText -Pattern "surface_details_injected: true" -Description "surface detail report marker"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "blocking gate marker"
Assert-Contains -Text $reportText -Pattern "| heat-transfer surfaces | 6 | 6 |" -Description "surface object count"
Assert-Contains -Text $reportText -Pattern "| constructions | 3 | 3 |" -Description "construction object count"
Assert-Contains -Text $reportText -Pattern "| other equipment | 2 | 2 |" -Description "internal gain object count"

$summary = Get-Content -Raw -LiteralPath $summaryPath | ConvertFrom-Json
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Static model summary must retain conformance_claim=true"
}
if ($summary.runtime_class -ne "static-model") {
    throw "Unexpected runtime_class: $($summary.runtime_class)"
}
if ($summary.status -ne "pass") {
    throw "Static model summary did not pass: $($summary.status)"
}
if ($summary.surface_details_injected -ne $true) {
    throw "Static model summary must record surface detail injection"
}
$rows = @($summary.rows)
if ($rows.Count -ne 19) {
    throw "Expected 19 static output rows, got $($rows.Count)"
}
if (@($rows | Where-Object { $_.level -ne "conformance" }).Count -ne 0) {
    throw "All static rows must be conformance-level"
}
if (@($rows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All static rows must pass"
}
if ($summary.object_counts.surfaces -ne 6 -or $summary.object_counts.oracle_surfaces -ne 6) {
    throw "Unexpected surface counts in summary"
}
if ($summary.object_counts.constructions -ne 3 -or $summary.object_counts.oracle_constructions -ne 3) {
    throw "Unexpected construction counts in summary"
}
if ($summary.object_counts.materials -ne 3 -or $summary.object_counts.oracle_materials -ne 3) {
    throw "Unexpected material counts in summary"
}
if ($summary.object_counts.other_equipment -ne 2 -or $summary.object_counts.oracle_other_equipment -ne 2) {
    throw "Unexpected OtherEquipment counts in summary"
}
if (($rows | Measure-Object -Property max_abs_delta -Maximum).Maximum -gt 0.02) {
    throw "Static max_abs_delta exceeded declared tolerance envelope"
}
if (($rows | Measure-Object -Property max_rel_delta -Maximum).Maximum -gt 0.001) {
    throw "Static max_rel_delta exceeded declared tolerance envelope"
}

Write-Host "Static model conformance gate passed."
