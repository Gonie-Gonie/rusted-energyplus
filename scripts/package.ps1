[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "common.ps1")
Add-CargoBinToPath

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

cargo build -p ep_cli --release
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

Write-Host "Package assembly is not implemented yet; release binary build passed."
