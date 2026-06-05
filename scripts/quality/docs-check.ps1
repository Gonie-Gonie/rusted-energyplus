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
    "src\development-plan-v2.md",
    "src\architecture\rust-only-policy.md",
    "src\architecture\data-architecture.md",
    "src\operations\oracle-setup.md",
    "src\operations\external-checkpoints.md"
)

foreach ($relative in $required) {
    $path = Join-Path $DocsRoot $relative
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing documentation file: $path"
    }
}

$mdbook = Get-Command mdbook -ErrorAction SilentlyContinue
if ($null -ne $mdbook) {
    & $mdbook.Source build $DocsRoot
    if ($LASTEXITCODE -ne 0) { throw "mdbook build failed" }
}
else {
    Write-Warning "mdbook is not installed; structural docs check passed without building the book."
}

Write-Host "Docs check complete."
