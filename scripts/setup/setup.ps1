[CmdletBinding()]
param(
    [switch]$InstallRust,
    [switch]$InstallDocsTools,
    [switch]$SkipOracleDownload,
    [switch]$SkipSourceDownload,
    [switch]$SkipOracleSmoke
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleVersion = "26.1.0"
$OracleCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$RustToolchain = "1.96.0-x86_64-pc-windows-gnu"
$MdBookVersion = "0.5.3"

$OracleArchiveName = "EnergyPlus-26.1.0-6f2e40d102-Windows-x86_64.zip"
$OracleArchiveUrl = "https://github.com/NatLabRockies/EnergyPlus/releases/download/v26.1.0/$OracleArchiveName"
$OracleArchiveSha256 = "0bb6932d277eed62f996b625f37c533b8c35f9af0c53710d961d8442fc4e70b3"

$SourceArchiveName = "EnergyPlus-v26.1.0-source.zip"
$SourceArchiveUrl = "https://github.com/NREL/EnergyPlus/archive/refs/tags/v26.1.0.zip"

function Join-RepoPath {
    param([Parameter(Mandatory = $true)][string]$RelativePath)
    return (Join-Path $RepoRoot $RelativePath)
}

function New-Directory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
}

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

function Get-Sha256 {
    param([Parameter(Mandatory = $true)][string]$Path)
    return (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash.ToLowerInvariant()
}

function Download-File {
    param(
        [Parameter(Mandatory = $true)][string]$Url,
        [Parameter(Mandatory = $true)][string]$OutFile
    )
    if (Test-Path -LiteralPath $OutFile) {
        Write-Host "Using cached download: $OutFile"
        return
    }

    New-Directory -Path (Split-Path -Parent $OutFile)
    Write-Host "Downloading $Url"
    Invoke-WebRequest -UseBasicParsing -Uri $Url -OutFile $OutFile
}

function Invoke-External {
    param(
        [Parameter(Mandatory = $true)][string]$FilePath,
        [Parameter(Mandatory = $true)][string[]]$Arguments
    )
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed ($LASTEXITCODE): $FilePath $($Arguments -join ' ')"
    }
}

function Install-RustIfRequested {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -ne $cargo) {
        Write-Host "Rust is available: $($cargo.Source)"
        return
    }

    if (-not $InstallRust) {
        Write-Warning "Rust was not found. Re-run setup with -InstallRust to install rustup and Rust $RustToolchain."
        return
    }

    $rustupPath = Join-RepoPath ".runtime\downloads\rustup-init.exe"
    Download-File -Url "https://win.rustup.rs/x86_64" -OutFile $rustupPath
    Write-Host "Installing Rust $RustToolchain with rustup"
    Invoke-External -FilePath $rustupPath -Arguments @("-y", "--default-toolchain", $RustToolchain, "--profile", "minimal")

    $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if (Test-Path -LiteralPath $cargoBin) {
        Add-CargoBinToPath
    }
}

function Ensure-RustComponents {
    $rustup = Get-Command rustup -ErrorAction SilentlyContinue
    if ($null -eq $rustup) {
        Write-Warning "rustup is not available; skipping component installation."
        return
    }

    Invoke-External -FilePath $rustup.Source -Arguments @("toolchain", "install", $RustToolchain, "--profile", "minimal")
    Invoke-External -FilePath $rustup.Source -Arguments @("component", "add", "rustfmt", "clippy", "--toolchain", $RustToolchain)
}

function Ensure-DocsTools {
    Add-CargoBinToPath
    $mdbook = Get-Command mdbook -ErrorAction SilentlyContinue
    if ($null -ne $mdbook) {
        Write-Host "mdBook is available: $($mdbook.Source)"
        return
    }

    if (-not $InstallDocsTools) {
        Write-Warning "mdBook was not found. Re-run setup with -InstallDocsTools to install mdbook $MdBookVersion."
        return
    }

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo is required to install mdbook. Run setup with -InstallRust first."
    }

    Write-Host "Installing mdbook $MdBookVersion"
    Invoke-External -FilePath $cargo.Source -Arguments @("install", "mdbook", "--version", $MdBookVersion, "--locked")
}

function Expand-ArchiveToSingleRoot {
    param(
        [Parameter(Mandatory = $true)][string]$ArchivePath,
        [Parameter(Mandatory = $true)][string]$TargetPath,
        [Parameter(Mandatory = $true)][string]$MarkerFile
    )

    if (Test-Path -LiteralPath (Join-Path $TargetPath $MarkerFile)) {
        Write-Host "Already extracted: $TargetPath"
        return
    }

    New-Directory -Path (Split-Path -Parent $TargetPath)
    $tempPath = Join-RepoPath ".runtime\extract\$([System.IO.Path]::GetFileNameWithoutExtension($ArchivePath))-$PID"
    Remove-RepoDirectory -Path $tempPath
    New-Directory -Path $tempPath

    Write-Host "Extracting $ArchivePath"
    Expand-Archive -LiteralPath $ArchivePath -DestinationPath $tempPath -Force

    $marker = Get-ChildItem -LiteralPath $tempPath -Recurse -File -Filter $MarkerFile | Select-Object -First 1
    if ($null -eq $marker) {
        throw "Could not find marker file '$MarkerFile' after extracting $ArchivePath"
    }

    $root = $marker.Directory.FullName
    if (Test-Path -LiteralPath $TargetPath) {
        Remove-RepoDirectory -Path $TargetPath
    }

    if ($root.Equals($tempPath, [System.StringComparison]::OrdinalIgnoreCase)) {
        New-Directory -Path $TargetPath
        Get-ChildItem -LiteralPath $tempPath | Move-Item -Destination $TargetPath
    }
    else {
        Move-Item -LiteralPath $root -Destination $TargetPath
    }

    Remove-RepoDirectory -Path $tempPath
}

function Ensure-OracleBinary {
    if ($SkipOracleDownload) {
        Write-Host "Skipping EnergyPlus binary download."
        return
    }

    $archivePath = Join-RepoPath ".runtime\downloads\$OracleArchiveName"
    Download-File -Url $OracleArchiveUrl -OutFile $archivePath

    $actualSha = Get-Sha256 -Path $archivePath
    if ($actualSha -ne $OracleArchiveSha256) {
        throw "EnergyPlus archive SHA256 mismatch. Expected $OracleArchiveSha256, got $actualSha"
    }
    Write-Host "Verified EnergyPlus binary SHA256: $actualSha"

    $targetPath = Join-RepoPath ".runtime\energyplus\$OracleVersion"
    Expand-ArchiveToSingleRoot -ArchivePath $archivePath -TargetPath $targetPath -MarkerFile "energyplus.exe"
}

function Ensure-ReferenceSource {
    if ($SkipSourceDownload) {
        Write-Host "Skipping EnergyPlus source download."
        return
    }

    $archivePath = Join-RepoPath ".reference\downloads\$SourceArchiveName"
    Download-File -Url $SourceArchiveUrl -OutFile $archivePath
    $actualSha = Get-Sha256 -Path $archivePath
    Write-Host "Observed EnergyPlus source archive SHA256: $actualSha"

    $targetPath = Join-RepoPath ".reference\energyplus-src\$OracleVersion"
    $digestFile = Join-Path $targetPath "source.sha256"
    if (Test-Path -LiteralPath $digestFile) {
        $lockedSha = (Get-Content -Raw -LiteralPath $digestFile).Trim()
        if ($lockedSha -ne $actualSha) {
            throw "EnergyPlus source archive bootstrap SHA256 mismatch. Expected $lockedSha, got $actualSha"
        }
    }

    Expand-ArchiveToSingleRoot -ArchivePath $archivePath -TargetPath $targetPath -MarkerFile "CMakeLists.txt"
    Set-Content -LiteralPath $digestFile -Value $actualSha -Encoding UTF8

    $gitHead = Join-Path $targetPath ".git_archival.txt"
    if (Test-Path -LiteralPath $gitHead) {
        $archival = Get-Content -Raw -LiteralPath $gitHead
        if ($archival -notmatch $OracleCommit) {
            Write-Warning "Source archive metadata did not contain locked commit $OracleCommit."
        }
    }
}

function Write-LocalConfig {
    $path = Join-RepoPath "config\local.toml"
    if (Test-Path -LiteralPath $path) {
        Write-Host "Keeping existing config/local.toml"
        return
    }

    $content = @"
[local]
created_by_setup = true

[oracle]
energyplus_exe = ".runtime/energyplus/26.1.0/energyplus.exe"
convert_input_format_exe = ".runtime/energyplus/26.1.0/ConvertInputFormat.exe"
weather_dir = ".runtime/energyplus/26.1.0/WeatherData"
example_dir = ".runtime/energyplus/26.1.0/ExampleFiles"
source_dir = ".reference/energyplus-src/26.1.0"
"@
    Set-Content -LiteralPath $path -Value $content -Encoding UTF8
    Write-Host "Wrote config/local.toml"
}

New-Directory -Path (Join-RepoPath ".runtime")
New-Directory -Path (Join-RepoPath ".reference")
New-Directory -Path (Join-RepoPath "config")

Install-RustIfRequested
Ensure-RustComponents
Ensure-DocsTools
Ensure-OracleBinary
Ensure-ReferenceSource
Write-LocalConfig

if (-not $SkipOracleSmoke) {
    Invoke-DevCommand -Command "oracle-smoke"
}

Write-Host "Setup complete."
