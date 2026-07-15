[CmdletBinding()]
param(
    [switch]$SelfTest
)

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

$script = Join-Path $RepoRoot "tools\docs\validate_algorithm_ledger.py"
if (-not (Test-Path -LiteralPath $script -PathType Leaf)) {
    throw "Missing algorithm ledger validator: $script"
}

$arguments = @($script, "--repo-root", $RepoRoot)
if ($SelfTest) {
    $arguments += "--self-test"
}
& $python @arguments
if ($LASTEXITCODE -ne 0) {
    throw "Algorithm ledger check failed with exit code $LASTEXITCODE"
}

$psychrometricInventoryScript = Join-Path $RepoRoot "tools\docs\validate_psychrometric_inventory.py"
if (-not (Test-Path -LiteralPath $psychrometricInventoryScript -PathType Leaf)) {
    throw "Missing psychrometric routine inventory validator: $psychrometricInventoryScript"
}

$psychrometricInventoryArguments = @($psychrometricInventoryScript, "--repo-root", $RepoRoot)
if ($SelfTest) {
    $psychrometricInventoryArguments += "--self-test"
}
& $python @psychrometricInventoryArguments
if ($LASTEXITCODE -ne 0) {
    throw "Psychrometric routine inventory check failed with exit code $LASTEXITCODE"
}
