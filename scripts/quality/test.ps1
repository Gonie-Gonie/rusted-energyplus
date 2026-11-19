[CmdletBinding()]
param(
    [string]$Package = "",
    [string]$Filter = "",
    [switch]$Lib,
    [switch]$List,
    [switch]$Plan
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Set-Location (Get-RepoRoot)

$packages = @()
if ($PSBoundParameters.ContainsKey("Package")) {
    $packages = @($Package.Split(",") | ForEach-Object { $_.Trim() })
    foreach ($name in $packages) {
        if ($name -notmatch '^[A-Za-z0-9][A-Za-z0-9_-]*$') {
            throw "Package must contain comma-separated Cargo package names."
        }
    }
    if (@($packages | Select-Object -Unique).Count -ne $packages.Count) {
        throw "Package contains duplicate Cargo package names."
    }
}

$runWorkspaceDocs = $packages.Count -eq 0 -and -not $Filter -and -not $Lib -and -not $List
$docArguments = @("test", "--locked", "--workspace", "--doc")
$arguments = @("test", "--locked")
if ($packages.Count -eq 0) {
    $arguments += "--workspace"
    if ($runWorkspaceDocs) {
        $arguments += @("--lib", "--bins", "--tests")
        Write-Host "Test scope: workspace libraries, binaries and integration tests, then doctests."
    }
    else {
        Write-Host "Test scope: entire workspace."
    }
}
else {
    foreach ($name in $packages) { $arguments += @("-p", $name) }
    Write-Host "Test scope: $($packages -join ', ')."
}
if ($Lib) { $arguments += "--lib" }
if ($Filter) { $arguments += $Filter }
if ($List) { $arguments += @("--", "--list") }
$displayArguments = @($arguments | ForEach-Object {
    if ($_ -match '[\s"'']') { "'" + $_.Replace("'", "''") + "'" } else { $_ }
})
Write-Host ("cargo " + ($displayArguments -join " "))
if ($Plan) {
    if ($runWorkspaceDocs) { Write-Host ("cargo " + ($docArguments -join " ")) }
    Write-Host "Plan only; no Cargo command was executed."
    return
}

Add-CargoBinToPath
if ($null -eq (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}
& cargo @arguments | Tee-Object -Variable testOutput
if ($LASTEXITCODE -ne 0) { throw "cargo test failed" }
if ($Filter -and -not $List) {
    $summaries = [regex]::Matches(($testOutput -join "`n"), 'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;')
    $passed = 0
    $ignored = 0
    foreach ($summary in $summaries) {
        $passed += [int]$summary.Groups[1].Value
        $ignored += [int]$summary.Groups[3].Value
    }
    if ($passed + $ignored -eq 0) {
        throw "Filter '$Filter' selected zero tests. Check the package and filter, or use -List to inspect test names."
    }
    if ($passed -eq 0) {
        throw "Filter '$Filter' selected only ignored tests; no test executed. Use -List to inspect test names."
    }
}

if ($runWorkspaceDocs) {
    Write-Host ("cargo " + ($docArguments -join " "))
    & cargo @docArguments
    if ($LASTEXITCODE -ne 0) { throw "cargo doctests failed" }
}
