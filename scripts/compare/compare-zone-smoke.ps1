[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-zone\26.1.0"

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

$idf = Join-Path $OutputRoot "zone-temperature.idf"
@"
Version,26.1;

SimulationControl,No,No,No,No,Yes,No;

Building,Zone Compare,0.0,Suburbs,0.04,0.4,FullExterior,25,6;

Timestep,4;

Site:Location,Golden,39.74,-105.18,-7.0,1829.0;

RunPeriod,Run Period 1,1,1,2013,1,1,2013,Tuesday,Yes,Yes,No,Yes,Yes;

GlobalGeometryRules,UpperLeftCorner,CounterClockWise,World;

Material:NoMass,R1,Rough,1.0,0.9,0.7,0.7;

Construction,Wall Construction,R1;

Zone,ZONE ONE,0,0,0,0,1,1,1,1;

BuildingSurface:Detailed,
  Floor,
  Floor,
  Wall Construction,
  ZONE ONE,
  ,
  Outdoors,
  ,
  NoSun,
  NoWind,
  1.0,
  4,
  0,0,0,
  1,0,0,
  1,1,0,
  0,1,0;

BuildingSurface:Detailed,
  Roof,
  Roof,
  Wall Construction,
  ZONE ONE,
  ,
  Outdoors,
  ,
  NoSun,
  NoWind,
  0.0,
  4,
  0,0,1,
  0,1,1,
  1,1,1,
  1,0,1;

BuildingSurface:Detailed,
  Wall X0,
  Wall,
  Wall Construction,
  ZONE ONE,
  ,
  Outdoors,
  ,
  NoSun,
  NoWind,
  0.5,
  4,
  0,0,0,
  0,1,0,
  0,1,1,
  0,0,1;

BuildingSurface:Detailed,
  Wall X1,
  Wall,
  Wall Construction,
  ZONE ONE,
  ,
  Outdoors,
  ,
  NoSun,
  NoWind,
  0.5,
  4,
  1,0,0,
  1,0,1,
  1,1,1,
  1,1,0;

BuildingSurface:Detailed,
  Wall Y0,
  Wall,
  Wall Construction,
  ZONE ONE,
  ,
  Outdoors,
  ,
  NoSun,
  NoWind,
  0.5,
  4,
  0,0,0,
  0,0,1,
  1,0,1,
  1,0,0;

BuildingSurface:Detailed,
  Wall Y1,
  Wall,
  Wall Construction,
  ZONE ONE,
  ,
  Outdoors,
  ,
  NoSun,
  NoWind,
  0.5,
  4,
  0,1,0,
  1,1,0,
  1,1,1,
  0,1,1;

Output:Variable,ZONE ONE,Zone Mean Air Temperature,Hourly;
"@ | Set-Content -LiteralPath $idf -Encoding ASCII

Write-Host "Running EnergyPlus zone temperature comparison oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eso = Join-Path $OutputRoot "eplusout.eso"
if (-not (Test-Path -LiteralPath $eso -PathType Leaf)) {
    throw "EnergyPlus did not produce eplusout.eso"
}

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("zone-temperature.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "zone-temperature.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce zone-temperature.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing Rust first-zone result store with EnergyPlus ESO zone temperatures."
$output = & $cargo.Source run -p ep_cli --quiet -- compare zone-temperature $epjson $weather $eso 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Zone temperature comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Zone Temperature Diagnostic" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: none" -Description "tolerance boundary"
Assert-Contains -Text $text -Pattern "zone: ZONE ONE" -Description "zone name"
Assert-Contains -Text $text -Pattern "samples: 24" -Description "sample count"
Assert-Contains -Text $text -Pattern "max_abs_delta:" -Description "delta summary"
Assert-Contains -Text $text -Pattern "exact_match: not_available" -Description "exact-match boundary"
Assert-Contains -Text $text -Pattern "exit_code_semantics: extraction-only" -Description "exit-code boundary"
Assert-Contains -Text $text -Pattern "status: extracted" -Description "comparison status"

Write-Host "Zone temperature comparison smoke passed."
