[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
. (Join-Path $ScriptsRoot "lib\python.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$python = Get-PortablePythonExe
if (-not (Test-Path -LiteralPath $python -PathType Leaf)) {
    throw "Portable Python is missing. Run .\scripts\dev.cmd setup first."
}

$script = Join-Path $RepoRoot "tools\docs\validate_algorithm_ledger.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing algorithm ledger validator: $script"
}

& $python $script --repo-root $RepoRoot
if ($LASTEXITCODE -ne 0) {
    throw "Algorithm ledger check failed with exit code $LASTEXITCODE"
}
