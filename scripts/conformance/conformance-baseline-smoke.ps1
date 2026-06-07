[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\conformance-baseline\26.1.0"
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
        throw "Missing required baseline input: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating conformance baseline from case manifest."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance baseline $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Conformance baseline generation failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Conformance Baseline" -Description "baseline header"
Assert-Contains -Text $text -Pattern "id: schedule_constant_001" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "expanded_manifest:" -Description "expanded manifest path"
Assert-Contains -Text $text -Pattern "injected_outputs: 0" -Description "idempotent injected output count"
Assert-Contains -Text $text -Pattern "injected_meters: 0" -Description "injected meter count"
Assert-Contains -Text $text -Pattern "status: generated" -Description "baseline status"

$CaseOutput = Join-Path $OutputRoot "schedule_constant_001"
Assert-FileExists -Path (Join-Path $CaseOutput "input.idf") -Description "staged IDF"
Assert-FileExists -Path (Join-Path $CaseOutput "input.epJSON") -Description "converted epJSON"
Assert-FileExists -Path (Join-Path $CaseOutput "eplusout.eso") -Description "EnergyPlus ESO"
Assert-FileExists -Path (Join-Path $CaseOutput "eplusout.eio") -Description "EnergyPlus EIO"
Assert-FileExists -Path (Join-Path $CaseOutput "eplusout.err") -Description "EnergyPlus ERR"
Assert-FileExists -Path (Join-Path $CaseOutput "case-expanded.toml") -Description "expanded case manifest"

$expanded = Get-Content -Raw -LiteralPath (Join-Path $CaseOutput "case-expanded.toml")
Assert-Contains -Text $expanded -Pattern 'schema = "rusted-energyplus.baseline-expanded.v1"' -Description "expanded manifest schema"
Assert-Contains -Text $expanded -Pattern 'schema = "rusted-energyplus.output-injection.v1"' -Description "output injection schema"
Assert-Contains -Text $expanded -Pattern "staged_idf_contains_manifest_requests = true" -Description "staged output request policy"
Assert-Contains -Text $expanded -Pattern "outputs = 0" -Description "expanded output injection count"
Assert-Contains -Text $expanded -Pattern 'source = "eso"' -Description "expanded output source"
Assert-Contains -Text $expanded -Pattern 'eso = "eplusout.eso"' -Description "expanded ESO artifact"

$staged = Get-Content -Raw -LiteralPath (Join-Path $CaseOutput "input.idf")
Assert-Contains -Text $staged -Pattern "eplus-rs output request injection begin" -Description "staged output injection marker"
Assert-Contains -Text $staged -Pattern "Schedule Value" -Description "staged schedule output request"

Write-Host "Conformance baseline smoke passed."
