[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$required = @(
    "crates\ep_conformance\Cargo.toml",
    "crates\ep_conformance\src\lib.rs",
    "data\conformance_cases\schedule_constant_001\case.toml",
    "data\conformance_cases\schedule_constant_001\schedule_constant.idf",
    "data\conformance_suites\foundation.toml"
)

foreach ($relative in $required) {
    $path = Join-Path $RepoRoot $relative
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing conformance schema fixture: $path"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

& $cargo.Source test -p ep_conformance
if ($LASTEXITCODE -ne 0) {
    throw "ep_conformance tests failed"
}

Write-Host "Conformance schema smoke passed."
