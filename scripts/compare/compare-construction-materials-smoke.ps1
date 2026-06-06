[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-construction-materials\26.1.0"

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

function New-Directory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
}

function Invoke-External {
    param(
        [Parameter(Mandatory = $true)][string]$FilePath,
        [Parameter(Mandatory = $true)][string[]]$Arguments
    )
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed ($LASTEXITCODE): $FilePath $($Arguments -join ' ')"
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

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
$weather = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
foreach ($path in @($energyPlus, $converter, $weather)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot
New-Directory -Path $OutputRoot

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\construction_materials_001\construction_materials.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing construction/material fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "construction-materials.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus construction/material comparison oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eio = Join-Path $OutputRoot "eplusout.eio"
if (-not (Test-Path -LiteralPath $eio -PathType Leaf)) {
    throw "EnergyPlus did not produce eplusout.eio"
}

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("construction-materials.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "construction-materials.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce construction-materials.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing Rust construction/material thermal inputs with EnergyPlus EIO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare construction-materials $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Construction/material comparison smoke failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Construction Material Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.001" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "constructions: 3" -Description "construction count"
Assert-Contains -Text $text -Pattern "oracle_constructions: 3" -Description "oracle construction count"
Assert-Contains -Text $text -Pattern "materials: 3" -Description "material count"
Assert-Contains -Text $text -Pattern "oracle_materials: 3" -Description "oracle material count"
Assert-Contains -Text $text -Pattern "construction: R13WALL" -Description "wall construction"
Assert-Contains -Text $text -Pattern "material: R13LAYER/R13LAYER" -Description "wall material"
Assert-Contains -Text $text -Pattern "construction: FLOOR" -Description "floor construction"
Assert-Contains -Text $text -Pattern "construction: ROOF31" -Description "roof construction"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

Write-Host "Construction/material comparison smoke passed."
