[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
. (Join-Path $ScriptsRoot "lib\python.ps1")

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "Found $Description`: $Path"
}

function Invoke-PythonVersion {
    param(
        [Parameter(Mandatory = $true)][string]$PythonExe,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $version = & $PythonExe --version
    if ($LASTEXITCODE -ne 0) {
        throw "$Description failed to report a version."
    }
    Write-Host "$Description`: $version"
    if ($version -notmatch [regex]::Escape($ProjectPythonVersion)) {
        throw "$Description version mismatch. Expected $ProjectPythonVersion, got $version"
    }
}

$portablePython = Get-PortablePythonExe
$reportPython = Get-ReportPythonExe

Assert-FileExists -Path $portablePython -Description "portable Python"
Assert-FileExists -Path $reportPython -Description "report Python venv"
Assert-FileExists -Path (Get-ReportRequirementsPath) -Description "report Python requirements"

Invoke-PythonVersion -PythonExe $portablePython -Description "portable Python"
Invoke-PythonVersion -PythonExe $reportPython -Description "report Python"

$oodocsVersion = & $reportPython -c "import oodocs; print(oodocs.__version__)"
if ($LASTEXITCODE -ne 0) {
    throw "Failed to import oodocs from report Python venv."
}
Write-Host "oodocs: $oodocsVersion"
if ($oodocsVersion.Trim() -ne $ProjectOodocsVersion) {
    throw "oodocs version mismatch. Expected $ProjectOodocsVersion, got $oodocsVersion"
}

$matplotlibVersion = & $reportPython -c "import matplotlib; print(matplotlib.__version__)"
if ($LASTEXITCODE -ne 0) {
    throw "Failed to import matplotlib from report Python venv."
}
Write-Host "matplotlib: $matplotlibVersion"
if ($matplotlibVersion.Trim() -ne $ProjectMatplotlibVersion) {
    throw "matplotlib version mismatch. Expected $ProjectMatplotlibVersion, got $matplotlibVersion"
}

Write-Host "Python smoke passed."
