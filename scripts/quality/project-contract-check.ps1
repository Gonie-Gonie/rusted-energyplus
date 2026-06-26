[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
. (Join-Path $ScriptsRoot "lib\python.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$python = $null
$portablePython = Get-PortablePythonExe
if (Test-Path -LiteralPath $portablePython -PathType Leaf) {
    $python = $portablePython
}
else {
    $command = Get-Command python -ErrorAction SilentlyContinue
    if ($null -ne $command) {
        $python = $command.Source
    }
}
if ($null -eq $python) {
    throw "Python 3.11+ was not found. Run .\scripts\dev.cmd setup first."
}

$script = Join-Path $RepoRoot "tools\docs\validate_project_contract.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing project contract validator: $script"
}

& $python $script --repo-root $RepoRoot
if ($LASTEXITCODE -ne 0) {
    throw "Project contract check failed with exit code $LASTEXITCODE"
}
