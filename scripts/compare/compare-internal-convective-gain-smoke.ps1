[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-internal-convective-gain\26.1.0"

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

$idf = Join-Path $OutputRoot "internal-convective-gain.idf"
@"
Version,26.1;

SimulationControl,No,No,No,No,Yes,No;

Building,Internal Convective Gain Compare,0.0,Suburbs,0.04,0.4,FullExterior,25,6;

Timestep,1;

Site:Location,Golden,39.74,-105.18,-7.0,1829.0;

RunPeriod,Run Period 1,1,1,2013,1,1,2013,Tuesday,Yes,Yes,No,Yes,Yes;

GlobalGeometryRules,UpperLeftCorner,CounterClockWise,World;

ScheduleTypeLimits,Fraction,0,1,Continuous;

Schedule:Constant,AlwaysOn,Fraction,1.0;

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

OtherEquipment,
  Plug Load,
  None,
  ZONE ONE,
  AlwaysOn,
  EquipmentLevel,
  12.0,
  ,
  ,
  0.0,
  0.25,
  0.0;

Output:Variable,ZONE ONE,Zone Total Internal Convective Heating Rate,Hourly;
"@ | Set-Content -LiteralPath $idf -Encoding ASCII

Write-Host "Running EnergyPlus internal convective gain oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eso = Join-Path $OutputRoot "eplusout.eso"
if (-not (Test-Path -LiteralPath $eso -PathType Leaf)) {
    throw "EnergyPlus did not produce eplusout.eso"
}

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("internal-convective-gain.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "internal-convective-gain.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce internal-convective-gain.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing Rust internal convective gain trace with EnergyPlus ESO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare internal-convective-gain $epjson $eso 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Internal convective gain comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Internal Convective Gain Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: default" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "zones: 1" -Description "zone count"
Assert-Contains -Text $text -Pattern "zone: ZONE ONE" -Description "zone name"
Assert-Contains -Text $text -Pattern "samples: 24" -Description "sample count"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

Write-Host "Internal convective gain comparison smoke passed."
