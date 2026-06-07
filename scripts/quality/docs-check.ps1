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
    "src\guides\setup.md",
    "src\guides\developer-workflow.md",
    "src\guides\release-process.md",
    "src\adr\0001-docs-specs-and-evidence-retention.md",
    "src\generated\milestone-map.md",
    "src\generated\algorithm-ledger.md",
    "src\generated\conformance-case-index.md",
    "src\generated\object-coverage.md",
    "src\generated\variable-coverage.md"
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

$summaryPath = Join-Path $DocsRoot "src\SUMMARY.md"
$summary = Get-Content -Raw -LiteralPath $summaryPath
foreach ($forbidden in @("# Archive", "archive/")) {
    if ($summary.Contains($forbidden)) {
        throw "SUMMARY.md must not reference archive documentation: $forbidden"
    }
}

$docsSourceFiles = Get-ChildItem -LiteralPath (Join-Path $DocsRoot "src") -Recurse -File -Filter "*.md" |
    Where-Object { $_.FullName -notlike "*\src\adr\0001-docs-specs-and-evidence-retention.md" }
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
