[CmdletBinding()]
param(
    [string]$Tag = "v0.11.0",
    [string]$Version = "0.11.0",
    [string]$Repo = "Gonie-Gonie/rusted-energyplus",
    [string]$Artifact = "dist/eplus-rs-v0.11.0-windows-x64.zip",
    [string]$NotesFile = "docs/src/releases/v0.11.0.md",
    [string]$EvidenceRoot = ""
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
if ([string]::IsNullOrWhiteSpace($EvidenceRoot)) {
    $EvidenceRoot = ".runtime\release-evidence\v$Version"
}

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

function Get-AssetContentType {
    param([Parameter(Mandatory = $true)][string]$Path)

    switch ([System.IO.Path]::GetExtension($Path).ToLowerInvariant()) {
        ".zip" { "application/zip"; break }
        ".pdf" { "application/pdf"; break }
        ".html" { "text/html"; break }
        ".json" { "application/json"; break }
        default { "application/octet-stream"; break }
    }
}

function Upload-ReleaseAsset {
    param(
        [Parameter(Mandatory = $true)]$Release,
        [Parameter(Mandatory = $true)][string]$Path
    )

    $assetPath = (Resolve-Path -LiteralPath $Path).Path
    $assetName = [System.IO.Path]::GetFileName($assetPath)
    $uploadUri = ($Release.upload_url -replace "\{\?name,label\}", "") + "?name=$([System.Uri]::EscapeDataString($assetName))"
    $assetBytes = [System.IO.File]::ReadAllBytes($assetPath)
    $contentType = Get-AssetContentType -Path $assetPath

    Invoke-RestMethod -Method Post -Headers $headers -Uri $uploadUri -Body $assetBytes -ContentType $contentType | Out-Null
    Write-Host "Uploaded release asset: $assetName"
}

Upload-ReleaseAsset -Release $release -Path $artifactPath

if (Test-Path -LiteralPath $EvidenceRoot -PathType Container) {
    Get-ChildItem -LiteralPath $EvidenceRoot -File |
        Sort-Object Name |
        ForEach-Object { Upload-ReleaseAsset -Release $release -Path $_.FullName }
}

Write-Host "GitHub Release created: $($release.html_url)"
