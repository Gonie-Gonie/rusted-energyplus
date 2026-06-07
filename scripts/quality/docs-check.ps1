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
    "src\generated\milestone-map.md",
    "src\generated\algorithm-ledger.md",
    "src\generated\conformance-case-index.md",
    "src\generated\object-coverage.md",
    "src\generated\variable-coverage.md",
    "src\archive\index.md"
)

foreach ($relative in $required) {
    $path = Join-Path $DocsRoot $relative
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing documentation file: $path"
    }
}

Invoke-DevCommand -Command "docs-generate" -Arguments @("-Check")

$mdbook = Get-Command mdbook -ErrorAction SilentlyContinue
if ($null -ne $mdbook) {
    & $mdbook.Source build $DocsRoot
    if ($LASTEXITCODE -ne 0) { throw "mdbook build failed" }
}
else {
    Write-Warning "mdbook is not installed; structural docs check passed without building the book."
}

Write-Host "Docs check complete."
