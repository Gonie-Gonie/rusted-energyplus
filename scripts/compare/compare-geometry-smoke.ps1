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
$eio = ".runtime\oracle-smoke\26.1.0\eplusout.eio"
if (
    -not (Test-Path -LiteralPath $fixture -PathType Leaf) -or
    -not (Test-Path -LiteralPath $eio -PathType Leaf)
) {
    Write-Host "Oracle geometry artifacts are missing; running oracle smoke first."
    Invoke-DevCommand -Command "oracle-smoke"
}
foreach ($path in @($fixture, $eio)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing oracle geometry artifact: $path"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing Rust geometry summary with EnergyPlus EIO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare geometry $fixture $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Geometry comparison smoke failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Geometry Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.02" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "zones: 1" -Description "zone count"
Assert-Contains -Text $text -Pattern "oracle_zones: 1" -Description "oracle zone count"
Assert-Contains -Text $text -Pattern "zone: ZONE ONE" -Description "zone name"
Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "first divergence"
Assert-Contains -Text $text -Pattern "status: pass" -Description "comparison status"

Write-Host "Geometry comparison smoke passed."
