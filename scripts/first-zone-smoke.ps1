[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "common.ps1")
Add-CargoBinToPath

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
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

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\setup.cmd -InstallRust first."
}

$epjson = ".runtime\oracle-smoke\26.1.0\convert\smoke.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    Write-Host "Oracle smoke epJSON is missing; running oracle smoke first."
    & (Join-Path $RepoRoot "scripts\oracle-smoke.ps1")
}

$weather = ".runtime\energyplus\26.1.0\WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
if (-not (Test-Path -LiteralPath $weather -PathType Leaf)) {
    throw "Missing runtime smoke EPW: $weather"
}

Write-Host "Running first-zone simulation smoke: $epjson"
$output = & $cargo.Source run -p ep_cli --quiet -- run first-zone $epjson $weather --hours 24 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "First-zone simulation smoke failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "First Zone Simulation" -Description "run header"
Assert-Contains -Text $text -Pattern "zone: ZONE ONE" -Description "zone name"
Assert-Contains -Text $text -Pattern "samples: 24" -Description "sample count"
Assert-Contains -Text $text -Pattern "result_series: 2" -Description "result series count"
Assert-Contains -Text $text -Pattern "volume_m3:" -Description "derived zone volume"
Assert-Contains -Text $text -Pattern "conductance_w_per_k:" -Description "derived conductance"
Assert-Contains -Text $text -Pattern "first_zone_temp_c:" -Description "first zone temperature"
Assert-Contains -Text $text -Pattern "last_zone_temp_c:" -Description "last zone temperature"
Assert-Contains -Text $text -Pattern "status: pass" -Description "run status"

Write-Host "First-zone simulation smoke passed."
