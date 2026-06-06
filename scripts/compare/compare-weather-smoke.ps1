[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-weather\26.1.0"

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
$weather = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
foreach ($path in @($energyPlus, $weather)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot
New-Directory -Path $OutputRoot

$idf = Join-Path $OutputRoot "weather-fields.idf"
@"
Version,26.1;

Building,Weather Compare,0.0,Suburbs,0.04,0.4,FullExterior,25,6;

Timestep,1;

GlobalGeometryRules,UpperLeftCorner,CounterClockWise,World;

RunPeriod,Run Period 1,1,1,2013,1,3,2013,Tuesday,Yes,Yes,No,Yes,Yes;

Output:Variable,*,Site Outdoor Air Drybulb Temperature,Hourly;
Output:Variable,*,Site Outdoor Air Dewpoint Temperature,Hourly;
Output:Variable,*,Site Outdoor Air Relative Humidity,Hourly;
Output:Variable,*,Site Outdoor Air Barometric Pressure,Hourly;
Output:Variable,*,Site Wind Speed,Hourly;
Output:Variable,*,Site Wind Direction,Hourly;
"@ | Set-Content -LiteralPath $idf -Encoding ASCII

Write-Host "Running EnergyPlus weather comparison oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eso = Join-Path $OutputRoot "eplusout.eso"
if (-not (Test-Path -LiteralPath $eso -PathType Leaf)) {
    throw "EnergyPlus did not produce eplusout.eso"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing Rust EPW weather fields with EnergyPlus ESO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare weather-fields $weather $eso 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Weather comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Weather Field Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.00001-relative-0.000001" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "fields: 6" -Description "field count"
Assert-Contains -Text $text -Pattern "samples: 72" -Description "sample count"
Assert-Contains -Text $text -Pattern "field: dry_bulb_c" -Description "dry-bulb field"
Assert-Contains -Text $text -Pattern "field: dew_point_c" -Description "dew-point field"
Assert-Contains -Text $text -Pattern "field: relative_humidity_percent" -Description "relative humidity field"
Assert-Contains -Text $text -Pattern "field: barometric_pressure_pa" -Description "barometric pressure field"
Assert-Contains -Text $text -Pattern "field: wind_speed_m_per_s" -Description "wind speed field"
Assert-Contains -Text $text -Pattern "field: wind_direction_deg" -Description "wind direction field"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

Write-Host "Weather comparison smoke passed."
