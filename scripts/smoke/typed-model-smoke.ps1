[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Invoke-Compile {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [int]$ExpectedExitCode = 0
    )

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }

    $output = & $cargo.Source run -p ep_cli --quiet -- model compile $Path 2>&1
    if ($LASTEXITCODE -ne $ExpectedExitCode) {
        $output | ForEach-Object { Write-Host $_ }
        throw "TypedModel compile exit code $LASTEXITCODE did not match expected $ExpectedExitCode for $Path"
    }

    return ($output -join "`n")
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        Write-Host $Text
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

$validFixture = "data\testcases\minimal\typed-model.epJSON"
if (-not (Test-Path -LiteralPath $validFixture -PathType Leaf)) {
    throw "Missing TypedModel fixture: $validFixture"
}

Write-Host "Compiling typed fixture: $validFixture"
$validOutput = Invoke-Compile -Path $validFixture
Assert-Contains -Text $validOutput -Pattern "TypedModel" -Description "fixture typed model header"
Assert-Contains -Text $validOutput -Pattern "version: 26.1.0" -Description "fixture typed model version"
Assert-Contains -Text $validOutput -Pattern "raw_objects: 10" -Description "fixture raw count"
Assert-Contains -Text $validOutput -Pattern "typed_objects: 9" -Description "fixture typed object count"
Assert-Contains -Text $validOutput -Pattern "diagnostics: 0" -Description "fixture diagnostics"
Assert-Contains -Text $validOutput -Pattern "defaults_applied: 21" -Description "fixture defaults"
Assert-Contains -Text $validOutput -Pattern "coverage:" -Description "fixture coverage section"
Assert-Contains -Text $validOutput -Pattern "BuildingSurface:Detailed: 1 [typed]" -Description "fixture typed coverage"

$missingReferenceFixture = "data\testcases\minimal\missing-reference.epJSON"
if (-not (Test-Path -LiteralPath $missingReferenceFixture -PathType Leaf)) {
    throw "Missing TypedModel negative fixture: $missingReferenceFixture"
}

Write-Host "Compiling missing-reference fixture: $missingReferenceFixture"
$missingReferenceOutput = Invoke-Compile -Path $missingReferenceFixture -ExpectedExitCode 1
Assert-Contains -Text $missingReferenceOutput -Pattern "Compile diagnostics" -Description "missing-reference diagnostics header"
Assert-Contains -Text $missingReferenceOutput -Pattern "diagnostics: 1" -Description "missing-reference diagnostic count"
Assert-Contains -Text $missingReferenceOutput -Pattern "MissingReference BuildingSurface:Detailed/Broken Surface field zone_name" -Description "missing-reference diagnostic"
Assert-Contains -Text $missingReferenceOutput -Pattern "BuildingSurface:Detailed: 1 [typed]" -Description "missing-reference typed coverage"

$invalidEnumFixture = "data\testcases\minimal\invalid-enum.epJSON"
if (-not (Test-Path -LiteralPath $invalidEnumFixture -PathType Leaf)) {
    throw "Missing TypedModel invalid enum fixture: $invalidEnumFixture"
}

Write-Host "Compiling invalid-enum fixture: $invalidEnumFixture"
$invalidEnumOutput = Invoke-Compile -Path $invalidEnumFixture -ExpectedExitCode 1
Assert-Contains -Text $invalidEnumOutput -Pattern "Compile diagnostics" -Description "invalid-enum diagnostics header"
Assert-Contains -Text $invalidEnumOutput -Pattern "diagnostics: 1" -Description "invalid-enum diagnostic count"
Assert-Contains -Text $invalidEnumOutput -Pattern "InvalidEnumValue Building/Bad Building field terrain" -Description "invalid-enum diagnostic"
Assert-Contains -Text $invalidEnumOutput -Pattern "Building: 1 [typed]" -Description "invalid-enum typed coverage"

$oracleEpjson = ".runtime\oracle-smoke\26.1.0\convert\smoke.epJSON"
if (-not (Test-Path -LiteralPath $oracleEpjson -PathType Leaf)) {
    Write-Host "Oracle smoke epJSON is missing; running oracle smoke first."
    Invoke-DevCommand -Command "oracle-smoke"
}

Write-Host "Compiling oracle-generated epJSON: $oracleEpjson"
$oracleOutput = Invoke-Compile -Path $oracleEpjson
Assert-Contains -Text $oracleOutput -Pattern "TypedModel" -Description "typed model header"
Assert-Contains -Text $oracleOutput -Pattern "version: 26.1.0" -Description "typed model version"
Assert-Contains -Text $oracleOutput -Pattern "raw_objects: 88" -Description "typed model raw count"
Assert-Contains -Text $oracleOutput -Pattern "typed_objects: 22" -Description "typed model object count"
Assert-Contains -Text $oracleOutput -Pattern "building: 1" -Description "typed model building"
Assert-Contains -Text $oracleOutput -Pattern "other_equipment: 2" -Description "typed model other equipment count"
Assert-Contains -Text $oracleOutput -Pattern "zones: 1" -Description "typed model zone count"
Assert-Contains -Text $oracleOutput -Pattern "surfaces: 6" -Description "typed model surface count"
Assert-Contains -Text $oracleOutput -Pattern "diagnostics: 0" -Description "typed model diagnostics"
Assert-Contains -Text $oracleOutput -Pattern "Output:Variable: 48 [raw-only]" -Description "typed model raw-only coverage"

Write-Host "TypedModel smoke passed."
