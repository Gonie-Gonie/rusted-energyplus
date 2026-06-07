[CmdletBinding()]
param(
    [switch]$Check
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
. (Join-Path $ScriptsRoot "lib\python.ps1")

$RepoRoot = Get-RepoRoot
$generator = Join-Path $RepoRoot "tools\docs\generate_docs.py"
if (-not (Test-Path -LiteralPath $generator -PathType Leaf)) {
    throw "Missing docs generator: $generator"
}

$pythonCandidates = @(
    (Get-ReportPythonExe),
    (Get-PortablePythonExe)
)

$python = $null
foreach ($candidate in $pythonCandidates) {
    if (Test-Path -LiteralPath $candidate -PathType Leaf) {
        $python = $candidate
        break
    }
}

if ($null -eq $python) {
    $command = Get-Command python -ErrorAction SilentlyContinue
    if ($null -ne $command) {
        $python = $command.Source
    }
}

if ($null -eq $python) {
    throw "Python 3.11+ was not found. Run .\scripts\dev.cmd setup first."
}

$args = @($generator, "--repo-root", $RepoRoot)
if ($Check) {
    $args += "--check"
}

& $python @args
if ($LASTEXITCODE -ne 0) {
    if ($Check) {
        throw "Generated docs are stale. Run .\scripts\dev.cmd docs-generate."
    }
    throw "Docs generation failed."
}

if ($Check) {
    Write-Host "Generated docs are current."
}
else {
    Write-Host "Generated docs updated."
}
