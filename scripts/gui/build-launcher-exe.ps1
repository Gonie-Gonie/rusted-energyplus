[CmdletBinding()]
param(
    [string]$OutputPath = "",
    [switch]$SelfTest
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $ScriptsRoot "..")).Path

if ([string]::IsNullOrWhiteSpace($OutputPath)) {
    $OutputPath = Join-Path $RepoRoot "target\launcher\eplus-rs-launch.exe"
}

if (-not [System.IO.Path]::IsPathRooted($OutputPath)) {
    $OutputPath = Join-Path $RepoRoot $OutputPath
}

$OutputPath = [System.IO.Path]::GetFullPath($OutputPath)
$parent = Split-Path -Parent $OutputPath
if (-not (Test-Path -LiteralPath $parent)) {
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
}
if (Test-Path -LiteralPath $OutputPath -PathType Leaf) {
    Remove-Item -LiteralPath $OutputPath -Force
}

$sourcePath = Join-Path $PSScriptRoot "eplus-rs-launcher.cs"
if (-not (Test-Path -LiteralPath $sourcePath -PathType Leaf)) {
    throw "Missing launcher source: $sourcePath"
}

$sourceText = Get-Content -Raw -Encoding UTF8 -LiteralPath $sourcePath
if ($sourceText -match 'powershell(\.exe)?') {
    throw "Direct launcher source must not start PowerShell at runtime: $sourcePath"
}

Add-Type `
    -Path $sourcePath `
    -ReferencedAssemblies @(
        "System.dll",
        "System.Core.dll",
        "System.Drawing.dll",
        "System.Web.Extensions.dll",
        "System.Windows.Forms.dll"
    ) `
    -OutputAssembly $OutputPath `
    -OutputType WindowsApplication

if (-not (Test-Path -LiteralPath $OutputPath -PathType Leaf)) {
    throw "Launcher executable was not created: $OutputPath"
}

$item = Get-Item -LiteralPath $OutputPath
if ($SelfTest) {
    [pscustomobject]@{
        output_path = $item.FullName
        bytes = $item.Length
        output_type = "WindowsApplication"
        runtime = "direct-winforms"
        starts_powershell = $false
        source_path = "scripts\gui\eplus-rs-launcher.cs"
        script_path = "not-used-at-runtime"
    } | ConvertTo-Json -Depth 3
}
else {
    Write-Host "Launcher executable created: $($item.FullName)"
}
