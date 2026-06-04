[CmdletBinding()]
param(
    [string]$Version = "0.1.0",
    [string]$Target = "windows-x64"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
. (Join-Path $PSScriptRoot "common.ps1")
Add-CargoBinToPath

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside repository: $full"
    }
}

function Remove-RepoDirectory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (Test-Path -LiteralPath $Path) {
        Assert-RepoSubPath -Path $Path
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

function Copy-RepoItem {
    param(
        [Parameter(Mandatory = $true)][string]$Source,
        [Parameter(Mandatory = $true)][string]$Destination
    )
    $parent = Split-Path -Parent $Destination
    if (-not (Test-Path -LiteralPath $parent)) {
        New-Item -ItemType Directory -Force -Path $parent | Out-Null
    }
    Copy-Item -LiteralPath $Source -Destination $Destination -Recurse -Force
}

cargo build -p ep_cli --release
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

$distRoot = Join-Path $RepoRoot "dist"
$stageRoot = Join-Path $distRoot "stage\eplus-rs-v$Version-$Target"
$zipPath = Join-Path $distRoot "eplus-rs-v$Version-$Target.zip"

Remove-RepoDirectory -Path $stageRoot
if (Test-Path -LiteralPath $zipPath) {
    Assert-RepoSubPath -Path $zipPath
    Remove-Item -LiteralPath $zipPath -Force
}

New-Item -ItemType Directory -Force -Path (Join-Path $stageRoot "bin") | Out-Null
Copy-RepoItem -Source (Join-Path $RepoRoot "target\release\eplus-rs.exe") -Destination (Join-Path $stageRoot "bin\eplus-rs.exe")

$packagedExe = Join-Path $stageRoot "bin\eplus-rs.exe"
$binaryVersion = & $packagedExe --version
if ($LASTEXITCODE -ne 0) {
    throw "Packaged binary version check failed"
}
$expectedVersion = "eplus-rs $Version"
if ($binaryVersion -ne $expectedVersion) {
    throw "Packaged binary reported '$binaryVersion', expected '$expectedVersion'"
}

Copy-RepoItem -Source (Join-Path $RepoRoot "README.md") -Destination (Join-Path $stageRoot "README.md")
Copy-RepoItem -Source (Join-Path $RepoRoot "CONTRIBUTING.md") -Destination (Join-Path $stageRoot "CONTRIBUTING.md")
Copy-RepoItem -Source (Join-Path $RepoRoot "CHANGELOG.md") -Destination (Join-Path $stageRoot "CHANGELOG.md")
Copy-RepoItem -Source (Join-Path $RepoRoot "Cargo.toml") -Destination (Join-Path $stageRoot "Cargo.toml")
Copy-RepoItem -Source (Join-Path $RepoRoot "Cargo.lock") -Destination (Join-Path $stageRoot "Cargo.lock")
Copy-RepoItem -Source (Join-Path $RepoRoot "rust-toolchain.toml") -Destination (Join-Path $stageRoot "rust-toolchain.toml")
Copy-RepoItem -Source (Join-Path $RepoRoot "scripts") -Destination (Join-Path $stageRoot "scripts")
Copy-RepoItem -Source (Join-Path $RepoRoot "config\default.toml") -Destination (Join-Path $stageRoot "config\default.toml")
Copy-RepoItem -Source (Join-Path $RepoRoot "config\local.toml.example") -Destination (Join-Path $stageRoot "config\local.toml.example")
Copy-RepoItem -Source (Join-Path $RepoRoot "tools") -Destination (Join-Path $stageRoot "tools")
Copy-RepoItem -Source (Join-Path $RepoRoot "data\testcases") -Destination (Join-Path $stageRoot "data\testcases")
Copy-RepoItem -Source (Join-Path $RepoRoot "docs\src") -Destination (Join-Path $stageRoot "docs\src")
Copy-RepoItem -Source (Join-Path $RepoRoot "docs\book.toml") -Destination (Join-Path $stageRoot "docs\book.toml")

$packageItems = Get-ChildItem -LiteralPath $stageRoot
Compress-Archive -LiteralPath $packageItems.FullName -DestinationPath $zipPath -Force
Write-Host "Package created: $zipPath"
