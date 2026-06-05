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

$fixture = "data\testcases\minimal\schedule-compact.epJSON"
if (-not (Test-Path -LiteralPath $fixture -PathType Leaf)) {
    throw "Missing Schedule:Compact fixture: $fixture"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Compiling Schedule:Compact fixture: $fixture"
$output = & $cargo.Source run -p ep_cli --quiet -- model compile $fixture 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Schedule:Compact compile failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "TypedModel" -Description "typed model header"
Assert-Contains -Text $text -Pattern "raw_objects: 3" -Description "raw object count"
Assert-Contains -Text $text -Pattern "typed_objects: 3" -Description "typed object count"
Assert-Contains -Text $text -Pattern "schedules: 1" -Description "total schedule count"
Assert-Contains -Text $text -Pattern "compact_schedules: 1" -Description "compact schedule count"
Assert-Contains -Text $text -Pattern "diagnostics: 0" -Description "diagnostics"
Assert-Contains -Text $text -Pattern "Schedule:Compact: 1 [typed]" -Description "typed coverage"

Write-Host "Schedule:Compact smoke passed."
