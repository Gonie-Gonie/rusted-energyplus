[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

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

$fixture = ".runtime\oracle-smoke\26.1.0\convert\smoke.epJSON"
if (-not (Test-Path -LiteralPath $fixture -PathType Leaf)) {
    Write-Host "Oracle converted fixture is missing; running oracle smoke first."
    Invoke-DevCommand -Command "oracle-smoke"
}
if (-not (Test-Path -LiteralPath $fixture -PathType Leaf)) {
    throw "Missing converted oracle fixture: $fixture"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Summarizing geometry fixture: $fixture"
$output = & $cargo.Source run -p ep_cli --quiet -- model geometry $fixture 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Geometry summary smoke failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Geometry Summary" -Description "geometry header"
Assert-Contains -Text $text -Pattern "zones: 1" -Description "zone count"
Assert-Contains -Text $text -Pattern "zone: ZONE ONE" -Description "zone name"
Assert-Contains -Text $text -Pattern "surfaces: 6" -Description "surface count"
Assert-Contains -Text $text -Pattern "floor_area_m2:" -Description "floor area"
Assert-Contains -Text $text -Pattern "volume_m3:" -Description "volume"
Assert-Contains -Text $text -Pattern "exterior_wall_area_m2:" -Description "exterior wall area"
Assert-Contains -Text $text -Pattern "status: summarized" -Description "status"

Write-Host "Geometry smoke passed."
