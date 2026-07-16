[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-surface-geometry\26.1.0"

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

function Assert-SurfaceWorldVertices {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$SurfaceName,
        [Parameter(Mandatory = $true)][string]$SurfaceClass,
        [Parameter(Mandatory = $true)][string]$Azimuth,
        [Parameter(Mandatory = $true)][string]$Tilt
    )
    $pattern = "surface: $SurfaceName class: $SurfaceClass/$SurfaceClass area_net_m2: 1.000000/1.000000 area_gross_m2: 1.000000/1.000000 azimuth_deg: $Azimuth/$Azimuth tilt_deg: $Tilt/$Tilt sides: 4/4 world_vertices: pass"
    Assert-Contains -Text $Text -Pattern $pattern -Description "$SurfaceName transformed world vertices"
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

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\surface_geometry_001\surface_geometry.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing surface geometry fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "surface-geometry.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus surface geometry comparison oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eio = Join-Path $OutputRoot "eplusout.eio"
if (-not (Test-Path -LiteralPath $eio -PathType Leaf)) {
    throw "EnergyPlus did not produce eplusout.eio"
}

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("surface-geometry.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "surface-geometry.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce surface-geometry.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing Rust surface geometry with EnergyPlus EIO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare surface-geometry $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Surface geometry comparison smoke failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Surface Geometry Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.01-relative-0.000001" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "surfaces: 6" -Description "surface count"
Assert-Contains -Text $text -Pattern "oracle_surfaces: 6" -Description "oracle surface count"
Assert-Contains -Text $text -Pattern "surface: FLOOR" -Description "floor surface"
Assert-Contains -Text $text -Pattern "surface: ROOF" -Description "roof surface"
Assert-Contains -Text $text -Pattern "surface: WALL X0" -Description "wall X0 surface"
Assert-Contains -Text $text -Pattern "surface: WALL X1" -Description "wall X1 surface"
Assert-Contains -Text $text -Pattern "surface: WALL Y0" -Description "wall Y0 surface"
Assert-Contains -Text $text -Pattern "surface: WALL Y1" -Description "wall Y1 surface"
Assert-Contains -Text $text -Pattern "azimuth_deg: 270.000000/270.000000" -Description "floor azimuth"
Assert-Contains -Text $text -Pattern "tilt_deg: 180.000000/180.000000" -Description "floor tilt"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

Write-Host "Base surface geometry comparison smoke passed."

$transformFixtureIdf = Join-Path $RepoRoot "data\conformance_cases\surface_geometry_transform_001\surface_geometry_transform.idf"
if (-not (Test-Path -LiteralPath $transformFixtureIdf -PathType Leaf)) {
    throw "Missing transformed surface geometry fixture: $transformFixtureIdf"
}
$transformOutputRoot = Join-Path $OutputRoot "transform"
New-Directory -Path $transformOutputRoot
$transformIdf = Join-Path $transformOutputRoot "surface-geometry-transform.idf"
Copy-Item -LiteralPath $transformFixtureIdf -Destination $transformIdf -Force

Write-Host "Running EnergyPlus relative-coordinate surface geometry oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $transformOutputRoot, $transformIdf)

$transformEio = Join-Path $transformOutputRoot "eplusout.eio"
if (-not (Test-Path -LiteralPath $transformEio -PathType Leaf)) {
    throw "EnergyPlus did not produce transformed eplusout.eio"
}

Push-Location $transformOutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("surface-geometry-transform.idf")
}
finally {
    Pop-Location
}

$transformEpjson = Join-Path $transformOutputRoot "surface-geometry-transform.epJSON"
if (-not (Test-Path -LiteralPath $transformEpjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce surface-geometry-transform.epJSON"
}

Write-Host "Comparing Rust relative-coordinate world vertices with EnergyPlus EIO."
$transformOutput = & $cargo.Source run -p ep_cli --quiet -- compare surface-geometry $transformEpjson $transformEio 2>&1
if ($LASTEXITCODE -ne 0) {
    $transformOutput | ForEach-Object { Write-Host $_ }
    throw "Transformed surface geometry comparison smoke failed."
}

$transformText = ($transformOutput -join "`n")
Assert-Contains -Text $transformText -Pattern "Surface Geometry Comparison" -Description "transformed comparison header"
Assert-Contains -Text $transformText -Pattern "comparison_class: smoke" -Description "transformed comparison class"
Assert-Contains -Text $transformText -Pattern "conformance_claim: false" -Description "transformed conformance boundary"
Assert-Contains -Text $transformText -Pattern "tolerance_policy: absolute-0.01-relative-0.000001" -Description "transformed tolerance policy"
Assert-Contains -Text $transformText -Pattern "surfaces: 6" -Description "transformed surface count"
Assert-Contains -Text $transformText -Pattern "oracle_surfaces: 6" -Description "transformed oracle surface count"
Assert-Contains -Text $transformText -Pattern "rules: starting_corner: UpperLeftCorner/UpperLeftCorner vertex_direction: Counterclockwise/Counterclockwise coordinate_system: RelativeCoordinateSystem/RelativeCoordinateSystem daylight_coordinate_system: RelativeCoordinateSystem/RelativeCoordinateSystem rectangular_coordinate_system: RelativeToZoneOrigin/RelativeToZoneOrigin status: pass" -Description "official global geometry rules"
Assert-SurfaceWorldVertices -Text $transformText -SurfaceName "FLOOR" -SurfaceClass "Floor" -Azimuth "345.000000" -Tilt "180.000000"
Assert-SurfaceWorldVertices -Text $transformText -SurfaceName "ROOF" -SurfaceClass "Roof" -Azimuth "75.000000" -Tilt "0.000000"
Assert-SurfaceWorldVertices -Text $transformText -SurfaceName "WALL X0" -SurfaceClass "Wall" -Azimuth "165.000000" -Tilt "90.000000"
Assert-SurfaceWorldVertices -Text $transformText -SurfaceName "WALL X1" -SurfaceClass "Wall" -Azimuth "345.000000" -Tilt "90.000000"
Assert-SurfaceWorldVertices -Text $transformText -SurfaceName "WALL Y0" -SurfaceClass "Wall" -Azimuth "75.000000" -Tilt "90.000000"
Assert-SurfaceWorldVertices -Text $transformText -SurfaceName "WALL Y1" -SurfaceClass "Wall" -Azimuth "255.000000" -Tilt "90.000000"
Assert-Contains -Text $transformText -Pattern "first_divergence: none" -Description "transformed first divergence"
Assert-Contains -Text $transformText -Pattern "status: pass" -Description "transformed comparison status"

Write-Host "Surface geometry comparison smoke passed."
