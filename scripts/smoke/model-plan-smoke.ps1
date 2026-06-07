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

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$fixture = "data\testcases\minimal\typed-model.epJSON"
if (-not (Test-Path -LiteralPath $fixture -PathType Leaf)) {
    throw "Missing plan fixture: $fixture"
}

Write-Host "Planning typed fixture: $fixture"
$output = & $cargo.Source run -p ep_cli --quiet -- model plan $fixture 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Execution plan smoke failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "ExecutionPlan" -Description "plan header"
Assert-Contains -Text $text -Pattern "zone_surface_edges: 1" -Description "zone-surface graph"
Assert-Contains -Text $text -Pattern "construction_material_edges: 1" -Description "construction-material graph"
Assert-Contains -Text $text -Pattern "stages: 3" -Description "stage count"
Assert-Contains -Text $text -Pattern "steps: 8" -Description "registry-backed step count"
Assert-Contains -Text $text -Pattern "environment: 2" -Description "environment stage"
Assert-Contains -Text $text -Pattern "zone: 1" -Description "zone stage"
Assert-Contains -Text $text -Pattern "output: 5" -Description "registry-backed output stage"
Assert-Contains -Text $text -Pattern "WriteOutput(4)" -Description "resolved output handle"

Write-Host "Model plan smoke passed."
