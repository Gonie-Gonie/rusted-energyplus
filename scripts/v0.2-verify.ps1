[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "common.ps1")
Add-CargoBinToPath

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

& (Join-Path $RepoRoot "scripts\v0.1-verify.ps1")
& (Join-Path $RepoRoot "scripts\raw-model-smoke.ps1")

Write-Host "v0.2.0 verification passed."

