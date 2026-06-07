[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

$warningLimit = 800
$failLimit = 1200

$waived = @(
    "crates\ep_cli\src\main.rs",
    "crates\ep_compiler\src\compiler.rs",
    "crates\ep_runtime\src\runtime.rs"
)

$waiverSet = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::OrdinalIgnoreCase)
foreach ($path in $waived) {
    [void]$waiverSet.Add($path)
}

$extensions = @(".rs", ".ps1", ".py")
$roots = @("crates", "scripts", "tools")
$failed = $false

foreach ($root in $roots) {
    if (-not (Test-Path -LiteralPath $root -PathType Container)) {
        continue
    }

    $files = Get-ChildItem -LiteralPath $root -Recurse -File |
        Where-Object { $extensions -contains $_.Extension.ToLowerInvariant() } |
        Where-Object { $_.FullName -notmatch '\\(__pycache__|target|docs\\book)\\' }

    foreach ($file in $files) {
        $fullPath = (Resolve-Path -LiteralPath $file.FullName).Path
        $relative = $fullPath.Substring($RepoRoot.Length).TrimStart("\", "/")
        $lineCount = (Get-Content -LiteralPath $file.FullName | Measure-Object -Line).Lines
        if ($lineCount -gt $failLimit) {
            if ($waiverSet.Contains($relative)) {
                Write-Host ("WAIVE {0}: {1} LOC (legacy refactor target)" -f $relative, $lineCount)
            }
            else {
                Write-Host ("FAIL  {0}: {1} LOC" -f $relative, $lineCount)
                $failed = $true
            }
        }
        elseif ($lineCount -gt $warningLimit) {
            Write-Host ("WARN  {0}: {1} LOC" -f $relative, $lineCount)
        }
    }
}

if ($failed) {
    throw "File-size check failed. Split files over $failLimit LOC or add an explicit temporary waiver."
}

Write-Host "File-size check complete."
