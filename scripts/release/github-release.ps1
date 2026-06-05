[CmdletBinding()]
param(
    [string]$Tag = "v0.1.0",
    [string]$Version = "0.1.0",
    [string]$Repo = "Gonie-Gonie/rusted-energyplus",
    [string]$Artifact = "dist/eplus-rs-v0.1.0-windows-x64.zip",
    [string]$NotesFile = "docs/src/releases/v0.1.0.md"
)

# Local/manual fallback publisher.
# Normal releases are created by .github/workflows/release.yml when a vX.Y.Z tag is pushed.

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$token = $env:GH_TOKEN
if ([string]::IsNullOrWhiteSpace($token)) {
    $token = $env:GITHUB_TOKEN
}

if ([string]::IsNullOrWhiteSpace($token)) {
    throw "GH_TOKEN or GITHUB_TOKEN is required to create a GitHub Release."
}

$artifactPath = (Resolve-Path -LiteralPath $Artifact).Path
$notesPath = (Resolve-Path -LiteralPath $NotesFile).Path
$notes = Get-Content -Raw -LiteralPath $notesPath

$headers = @{
    "Accept" = "application/vnd.github+json"
    "Authorization" = "Bearer $token"
    "User-Agent" = "rusted-energyplus-release"
    "X-GitHub-Api-Version" = "2022-11-28"
}

$releaseBody = @{
    tag_name = $Tag
    target_commitish = "main"
    name = "eplus-rs $Version"
    body = $notes
    draft = $false
    prerelease = $true
    generate_release_notes = $false
} | ConvertTo-Json -Depth 5

$releaseUri = "https://api.github.com/repos/$Repo/releases"
$release = Invoke-RestMethod -Method Post -Headers $headers -Uri $releaseUri -Body $releaseBody -ContentType "application/json"

$assetName = [System.IO.Path]::GetFileName($artifactPath)
$uploadUri = ($release.upload_url -replace "\{\?name,label\}", "") + "?name=$([System.Uri]::EscapeDataString($assetName))"
$assetBytes = [System.IO.File]::ReadAllBytes($artifactPath)

Invoke-RestMethod -Method Post -Headers $headers -Uri $uploadUri -Body $assetBytes -ContentType "application/zip" | Out-Null

Write-Host "GitHub Release created: $($release.html_url)"
