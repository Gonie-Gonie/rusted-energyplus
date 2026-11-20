[CmdletBinding()]
param(
    [string]$Version = "0.2.0",
    [string]$Tag = "",
    [string]$Repo = "Gonie-Gonie/rusted-energyplus",
    [string]$Artifact = "",
    [string]$NotesFile = ""
)
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "..\lib\common.ps1")
Set-Location -LiteralPath (Get-RepoRoot)
if ($Version -notmatch '^\d+\.\d+\.\d+$') { throw 'Expected MAJOR.MINOR.PATCH.' }
if ([string]::IsNullOrWhiteSpace($Tag)) { $Tag = "v$Version" }
if ($Tag -ne "v$Version") { throw 'Tag and version differ.' }
if ([string]::IsNullOrWhiteSpace($Artifact)) { $Artifact = "dist/eplus-rs-$Tag-windows-x64.zip" }
if ([string]::IsNullOrWhiteSpace($NotesFile)) { $NotesFile = "docs/src/releases/$Tag.md" }
$assets = @($Artifact, "$Artifact.sha256", "$Artifact.verification.json")
foreach ($path in @($assets) + @($NotesFile)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { throw "Missing release file: $path" }
}
$verification = Get-Content -LiteralPath "$Artifact.verification.json" -Raw | ConvertFrom-Json
$hash = (Get-ReleaseFileHash -LiteralPath $Artifact -Algorithm SHA256).Hash.ToLowerInvariant()
if ($verification.passed -ne $true -or $verification.version -ne $Version -or $verification.zip_sha256 -ne $hash) {
    throw 'Portable verification does not bind this release ZIP.'
}
if ($verification.source_tree_clean -ne $true -or $verification.source_commit -notmatch '^[0-9a-f]{40}$') {
    throw 'Publication requires a package produced from a clean, identified commit.'
}
$expectedChecksum = "$hash  $([IO.Path]::GetFileName($Artifact))"
if ((Get-Content -LiteralPath "$Artifact.sha256" -Raw).Trim() -ne $expectedChecksum) {
    throw 'Release ZIP checksum differs.'
}
# GitHub CLI uses its configured credential provider; no token is printed.
# --verify-tag requires the caller to have pushed the reviewed tag first.
$tagJson = & gh api "repos/$Repo/git/ref/tags/$Tag"
if ($LASTEXITCODE -ne 0) { throw 'Cannot resolve the published tag.' }
$tagObject = ($tagJson | ConvertFrom-Json).object
$depth = 0
while ($tagObject.type -eq 'tag') {
    if ($depth -ge 8) { throw 'Unexpected annotated tag nesting.' }
    $tagJson = & gh api "repos/$Repo/git/tags/$($tagObject.sha)"
    if ($LASTEXITCODE -ne 0) { throw 'Cannot peel the published tag.' }
    $tagObject = ($tagJson | ConvertFrom-Json).object
    $depth += 1
}
if ($tagObject.type -ne 'commit' -or $tagObject.sha -ne $verification.source_commit) {
    throw 'Package producing commit differs from the remote release tag.'
}
& gh release create $Tag @assets --repo $Repo --verify-tag --title "eplus-rs $Version" --notes-file $NotesFile --prerelease
if ($LASTEXITCODE -ne 0) { throw 'GitHub release creation or upload failed.' }
