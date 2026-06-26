[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$DocsRoot = Join-Path $RepoRoot "docs"

$required = @(
    "book.toml",
    "src\SUMMARY.md",
    "src\current\project-contract.md",
    "src\current\current-status.md",
    "src\current\roadmap.md",
    "src\current\verification.md",
    "src\current\architecture-overview.md",
    "src\current\launcher-and-run-framework.md",
    "src\guides\setup.md",
    "src\guides\developer-workflow.md",
    "src\guides\release-process.md",
    "src\adr\0001-docs-specs-and-evidence-retention.md",
    "src\generated\milestone-map.md",
    "src\generated\algorithm-ledger.md",
    "src\generated\conformance-case-index.md",
    "src\generated\object-coverage.md",
    "src\generated\variable-coverage.md",
    "src\generated\script-index.md",
    "src\generated\docs-inventory.md"
)

foreach ($relative in $required) {
    $path = Join-Path $DocsRoot $relative
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing documentation file: $path"
    }
}

$archiveRoot = Join-Path $DocsRoot "src\archive"
if (Test-Path -LiteralPath $archiveRoot) {
    throw "docs/src/archive is not retained. Move current material into current docs, specs, or ADRs."
}

$projectScopeRoot = Join-Path $DocsRoot "src\project-scope"
if (Test-Path -LiteralPath $projectScopeRoot) {
    throw "docs/src/project-scope is not retained. Move current material into current docs, specs, generated docs, or ADRs."
}

foreach ($relative in @(
    "src\operations\full-compatibility-reset.md",
    "src\porting-map\algorithm-porting-readiness.md"
)) {
    $path = Join-Path $DocsRoot $relative
    if (Test-Path -LiteralPath $path) {
        throw "Old planning/readiness documentation is not retained: $path"
    }
}

$summaryPath = Join-Path $DocsRoot "src\SUMMARY.md"
$summary = Get-Content -Raw -LiteralPath $summaryPath
$summaryLines = Get-Content -LiteralPath $summaryPath
$expectedCurrentLinks = @(
    "current/project-contract.md",
    "current/current-status.md",
    "current/roadmap.md",
    "current/verification.md",
    "current/architecture-overview.md",
    "current/launcher-and-run-framework.md"
)
$actualCurrentLinks = @()
$inCurrentSection = $false
foreach ($line in $summaryLines) {
    if ($line -eq "# Current") {
        $inCurrentSection = $true
        continue
    }
    if ($line.StartsWith("# ") -and $line -ne "# Current") {
        $inCurrentSection = $false
    }
    if ($inCurrentSection -and $line -match '\]\(([^)]+)\)') {
        $actualCurrentLinks += $Matches[1]
    }
}
if (@($actualCurrentLinks).Count -ne $expectedCurrentLinks.Count) {
    throw "SUMMARY.md Current navigation must contain exactly $($expectedCurrentLinks.Count) docs."
}
for ($index = 0; $index -lt $expectedCurrentLinks.Count; $index += 1) {
    if ($actualCurrentLinks[$index] -ne $expectedCurrentLinks[$index]) {
        throw "SUMMARY.md Current navigation mismatch at index $index`: expected $($expectedCurrentLinks[$index]), found $($actualCurrentLinks[$index])"
    }
}
$readmePath = Join-Path $RepoRoot "README.md"
$readme = Get-Content -Raw -LiteralPath $readmePath
$readmeH2Count = ([regex]::Matches($readme, '(?m)^## ')).Count
if ($readmeH2Count -gt 7) {
    throw "README.md must stay at 7 or fewer h2 sections; found $readmeH2Count."
}
$expectedReadmeCurrentLinks = @($expectedCurrentLinks | ForEach-Object { "docs/src/$_" })
$readmeCurrentLinks = @(
    [regex]::Matches($readme, '`(docs/src/current/[^`]+\.md)`') |
        ForEach-Object { $_.Groups[1].Value }
)
if (@($readmeCurrentLinks).Count -ne $expectedReadmeCurrentLinks.Count) {
    throw "README.md current docs list must contain exactly $($expectedReadmeCurrentLinks.Count) docs."
}
for ($index = 0; $index -lt $expectedReadmeCurrentLinks.Count; $index += 1) {
    if ($readmeCurrentLinks[$index] -ne $expectedReadmeCurrentLinks[$index]) {
        throw "README.md current docs mismatch at index $index`: expected $($expectedReadmeCurrentLinks[$index]), found $($readmeCurrentLinks[$index])"
    }
}
foreach ($forbidden in @("# Archive", "archive/")) {
    if ($summary.Contains($forbidden)) {
        throw "SUMMARY.md must not reference archive documentation: $forbidden"
    }
}

$docsSourceFiles = Get-ChildItem -LiteralPath (Join-Path $DocsRoot "src") -Recurse -File -Filter "*.md" |
    Where-Object {
        $_.FullName -notlike "*\src\adr\0001-docs-specs-and-evidence-retention.md" -and
        $_.FullName -notlike "*\src\generated\*"
    }
$forbiddenArchiveReferences = @(
    "docs/src/archive",
    "docs\src\archive",
    "archive/pre-alpha",
    "archive/old-readiness-notes"
)
foreach ($file in $docsSourceFiles) {
    $text = Get-Content -Raw -LiteralPath $file.FullName
    foreach ($forbidden in $forbiddenArchiveReferences) {
        if ($text.Contains($forbidden)) {
            throw "Documentation must not reference retained archive docs: $($file.FullName) contains $forbidden"
        }
    }
}

Invoke-DevCommand -Command "docs-generate" -Arguments @("-Check")

$mdbook = Get-Command mdbook -ErrorAction SilentlyContinue
if ($null -ne $mdbook) {
    & $mdbook.Source clean $DocsRoot
    if ($LASTEXITCODE -ne 0) { throw "mdbook clean failed" }
    & $mdbook.Source build $DocsRoot
    if ($LASTEXITCODE -ne 0) { throw "mdbook build failed" }
}
else {
    Write-Warning "mdbook is not installed; structural docs check passed without building the book."
}

Write-Host "Docs check complete."
