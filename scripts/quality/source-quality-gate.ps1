[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Test-IsTestPath {
    param([Parameter(Mandatory = $true)][string]$RelativePath)

    $normalized = $RelativePath -replace '/', '\'
    return (
        $normalized -match '(^|\\)tests(\\|\.rs$)' -or
        $normalized -match '(^|\\)[^\\]+_tests(\\|(?:_[^\\]+)?\.rs$)' -or
        $normalized -match '(^|\\)test_[^\\]+\.rs$'
    )
}

$deniedPatterns = @(
    [pscustomobject]@{
        Id = "unsafe-code"
        Pattern = '\bunsafe\s*(\{|fn\b|impl\b|trait\b|extern\b)'
        Description = "unsafe code construct"
    },
    [pscustomobject]@{
        Id = "unwrap"
        Pattern = '\.unwrap\s*\('
        Description = ".unwrap()"
    },
    [pscustomobject]@{
        Id = "expect"
        Pattern = '\.expect\s*\('
        Description = ".expect()"
    },
    [pscustomobject]@{
        Id = "panic"
        Pattern = '\bpanic!\s*\('
        Description = "panic!()"
    },
    [pscustomobject]@{
        Id = "todo"
        Pattern = '\btodo!\s*\('
        Description = "todo!()"
    },
    [pscustomobject]@{
        Id = "unimplemented"
        Pattern = '\bunimplemented!\s*\('
        Description = "unimplemented!()"
    }
)

$violations = New-Object System.Collections.Generic.List[object]
$sourceFiles = @(
    Get-ChildItem -LiteralPath "crates" -Recurse -File -Filter "*.rs" |
        Sort-Object FullName
)
$checkedFileCount = 0
$skippedTestFileCount = 0

foreach ($file in $sourceFiles) {
    $relativePath = (Resolve-Path -LiteralPath $file.FullName -Relative) -replace '^\.[\\/]', ''
    if (Test-IsTestPath -RelativePath $relativePath) {
        $skippedTestFileCount += 1
        continue
    }

    $checkedFileCount += 1
    $lines = Get-Content -Encoding UTF8 -LiteralPath $file.FullName
    $firstCfgTestLine = $null
    for ($lineIndex = 0; $lineIndex -lt $lines.Count; $lineIndex++) {
        if ($lines[$lineIndex] -match '^\s*#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]') {
            $firstCfgTestLine = $lineIndex + 1
            break
        }
    }

    for ($lineIndex = 0; $lineIndex -lt $lines.Count; $lineIndex++) {
        $lineNumber = $lineIndex + 1
        if ($null -ne $firstCfgTestLine -and $lineNumber -ge $firstCfgTestLine) {
            continue
        }

        $line = $lines[$lineIndex]
        foreach ($deniedPattern in $deniedPatterns) {
            if ($line -match $deniedPattern.Pattern) {
                $violations.Add([pscustomobject]@{
                        Path = $relativePath
                        Line = $lineNumber
                        Rule = $deniedPattern.Id
                        Description = $deniedPattern.Description
                        Text = $line.Trim()
                    }) | Out-Null
            }
        }
    }
}

if ($violations.Count -gt 0) {
    Write-Host "Source quality gate failed."
    foreach ($violation in $violations) {
        Write-Host "$($violation.Path):$($violation.Line): $($violation.Rule): $($violation.Text)"
    }
    throw "Found $($violations.Count) denied source quality pattern(s)."
}

Write-Host "Source quality gate passed."
Write-Host "  checked_rust_files: $checkedFileCount"
Write-Host "  skipped_test_files: $skippedTestFileCount"
