[CmdletBinding()]
param(
    [switch]$SkipRustComparison
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-material-roof-vegetation\26.1.0"
$MaterialsOnlyOutputRoot = Join-Path $OutputRoot "materials-only"
$ConstructionsOnlyOutputRoot = Join-Path $OutputRoot "constructions-only"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\material_roof_vegetation_001"
$ReportPath = Join-Path $ReportRoot "compare-report.md"

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

function New-Directory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
}

function Invoke-External {
    param(
        [Parameter(Mandatory = $true)][string]$FilePath,
        [Parameter(Mandatory = $true)][string[]]$Arguments
    )

    & $FilePath @Arguments | Out-Host
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed ($LASTEXITCODE): $FilePath $($Arguments -join ' ')"
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Text.IndexOf($Pattern, [System.StringComparison]::Ordinal) -lt 0) {
        Write-Host $Text
        throw "Missing $($Description): $Pattern"
    }
    Write-Host "OK $($Description): $Pattern"
}

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Text.IndexOf($Pattern, [System.StringComparison]::Ordinal) -ge 0) {
        Write-Host $Text
        throw "Unexpected $($Description): $Pattern"
    }
    Write-Host "OK absent $($Description): $Pattern"
}

function Assert-UniqueExactEioRow {
    param(
        [Parameter(Mandatory = $true)][string[]]$Lines,
        [Parameter(Mandatory = $true)][string]$Prefix,
        [Parameter(Mandatory = $true)][string]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $rows = @(
        $Lines |
            ForEach-Object { $_.Trim() } |
            Where-Object { $_.StartsWith($Prefix, [System.StringComparison]::Ordinal) }
    )
    if ($rows.Count -ne 1) {
        $rows | ForEach-Object { Write-Host $_ }
        throw "Expected exactly one $Description row with prefix '$Prefix'; found $($rows.Count)."
    }
    if (-not $rows[0].Equals($Expected, [System.StringComparison]::Ordinal)) {
        Write-Host "Expected: $Expected"
        Write-Host "Actual:   $($rows[0])"
        throw "Exact $Description EIO row mismatch."
    }
    Write-Host "OK exact unique $($Description): $Expected"
}

function Assert-ExactOrderedEioRows {
    param(
        [Parameter(Mandatory = $true)][string[]]$Lines,
        [Parameter(Mandatory = $true)][string[]]$Prefixes,
        [Parameter(Mandatory = $true)][string[]]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $rows = @(
        $Lines |
            ForEach-Object { $_.Trim() } |
            Where-Object {
                $line = $_
                @($Prefixes | Where-Object {
                    $line.StartsWith($_, [System.StringComparison]::Ordinal)
                }).Count -gt 0
            }
    )
    if ($rows.Count -ne $Expected.Count) {
        $rows | ForEach-Object { Write-Host $_ }
        throw "Expected $($Expected.Count) ordered $Description rows; found $($rows.Count)."
    }
    for ($index = 0; $index -lt $Expected.Count; $index++) {
        if (-not $rows[$index].Equals($Expected[$index], [System.StringComparison]::Ordinal)) {
            Write-Host "Expected[$index]: $($Expected[$index])"
            Write-Host "Actual[$index]:   $($rows[$index])"
            throw "Ordered $Description EIO row mismatch at index $index."
        }
    }
    Write-Host "OK exact ordered $Description rows: $($rows.Count)"
}

function Assert-CommaSeparatedTokenCount {
    param(
        [Parameter(Mandatory = $true)][string]$Row,
        [Parameter(Mandatory = $true)][int]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $actual = $Row.Split([char]',').Count
    if ($actual -ne $Expected) {
        throw "Expected $Expected comma-separated tokens for $Description; found $actual in: $Row"
    }
    Write-Host "OK $Description token count: $actual"
}

function Assert-CleanOracleLog {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "EnergyPlus did not produce required $Description log: $Path"
    }
    $text = Get-Content -LiteralPath $Path -Raw
    Assert-Contains -Text $text -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "$Description clean oracle completion"
    if ($text -match "(?m)^\s*\*\* (?:Warning|Severe) \*\*") {
        Write-Host $text
        throw "EnergyPlus emitted a warning or severe diagnostic in the $Description run despite the required clean summary."
    }
}

function Invoke-OracleCase {
    param(
        [Parameter(Mandatory = $true)][string]$Description,
        [Parameter(Mandatory = $true)][string]$OutputDirectory,
        [Parameter(Mandatory = $true)][string]$IdfName,
        [Parameter(Mandatory = $true)][string]$Selector
    )

    New-Directory -Path $OutputDirectory
    $idfPath = Join-Path $OutputDirectory $IdfName
    if ($Selector -eq "Constructions,Materials") {
        Copy-Item -LiteralPath $fixtureIdf -Destination $idfPath -Force
    }
    else {
        $fixtureText = [System.IO.File]::ReadAllText($fixtureIdf)
        $derivedText = $fixtureText.Replace(
            "Output:Constructions,Constructions,Materials;",
            "Output:Constructions,$Selector;"
        )
        if ($derivedText.Equals($fixtureText, [System.StringComparison]::Ordinal)) {
            throw "Could not derive $Description fixture because the exact Output:Constructions selector was not found."
        }
        $utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText($idfPath, $derivedText, $utf8WithoutBom)
    }

    Write-Host "Running EnergyPlus Material:RoofVegetation $Description oracle case."
    Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputDirectory, $idfPath)

    $eioPath = Join-Path $OutputDirectory "eplusout.eio"
    $errPath = Join-Path $OutputDirectory "eplusout.err"
    if (-not (Test-Path -LiteralPath $eioPath -PathType Leaf)) {
        throw "EnergyPlus did not produce required $Description EIO: $eioPath"
    }
    Assert-CleanOracleLog -Path $errPath -Description $Description

    Push-Location $OutputDirectory
    try {
        Invoke-External -FilePath $converter -Arguments @($IdfName)
    }
    finally {
        Pop-Location
    }

    $epJsonName = [System.IO.Path]::GetFileNameWithoutExtension($IdfName) + ".epJSON"
    $epJsonPath = Join-Path $OutputDirectory $epJsonName
    if (-not (Test-Path -LiteralPath $epJsonPath -PathType Leaf)) {
        throw "ConvertInputFormat did not produce $epJsonName"
    }

    return [pscustomobject]@{
        Eio = $eioPath
        EpJson = $epJsonPath
    }
}

function Invoke-RoofVegetationComparison {
    param(
        [Parameter(Mandatory = $true)][string]$CargoPath,
        [Parameter(Mandatory = $true)][string]$EpJsonPath,
        [Parameter(Mandatory = $true)][string]$EioPath,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Write-Host "Comparing bounded Rust Material:RoofVegetation inputs with $Description EnergyPlus EIO evidence."
    $output = & $CargoPath run -p ep_cli --quiet -- compare material-roof-vegetation $EpJsonPath $EioPath --tolerance exact 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Material:RoofVegetation $Description comparison smoke failed."
    }
    $text = $output -join [Environment]::NewLine
    Assert-Contains -Text $text -Pattern "Material Roof Vegetation Comparison" -Description "$Description comparison header"
    Assert-Contains -Text $text -Pattern "case_id: material_roof_vegetation_001" -Description "$Description case identity"
    Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "$Description comparison class"
    Assert-Contains -Text $text -Pattern "evidence: diagnostic-only" -Description "$Description diagnostic evidence"
    Assert-Contains -Text $text -Pattern "blocking: false" -Description "$Description nonblocking boundary"
    Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "$Description conformance boundary"
    Assert-Contains -Text $text -Pattern "runtime_claim: false" -Description "$Description runtime boundary"
    Assert-Contains -Text $text -Pattern "plant_physics_claim: false" -Description "$Description plant-physics boundary"
    Assert-Contains -Text $text -Pattern "moisture_balance_claim: false" -Description "$Description moisture boundary"
    Assert-Contains -Text $text -Pattern "ecoroof_manager_claim: false" -Description "$Description EcoRoof-manager boundary"
    Assert-Contains -Text $text -Pattern "construction_behavior_claim: false" -Description "$Description construction boundary"
    Assert-Contains -Text $text -Pattern "construction_occurrence_claim: false" -Description "$Description construction-occurrence boundary"
    Assert-Contains -Text $text -Pattern "surface_behavior_claim: false" -Description "$Description surface boundary"
    Assert-Contains -Text $text -Pattern "rust_eio_serialization_claim: false" -Description "$Description Rust-EIO-serialization boundary"
    Assert-Contains -Text $text -Pattern "broad_idf_declaration_order_claim: false" -Description "$Description broad-declaration-order boundary"
    Assert-Contains -Text $text -Pattern "tolerance_mode: exact" -Description "$Description exact tolerance"
    Assert-Contains -Text $text -Pattern "material_objects: 3" -Description "$Description typed material count"
    Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "$Description first divergence"
    Assert-Contains -Text $text -Pattern "status: pass" -Description "$Description comparison status"
    return $text
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
$weather = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
foreach ($path in @($energyPlus, $converter, $weather)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\material_roof_vegetation_001\material_roof_vegetation.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing Material:RoofVegetation fixture: $fixtureIdf"
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$primary = Invoke-OracleCase -Description "primary Constructions-and-Materials" -OutputDirectory $OutputRoot -IdfName "material-roof-vegetation.idf" -Selector "Constructions,Materials"
$materialsOnly = Invoke-OracleCase -Description "Materials-only" -OutputDirectory $MaterialsOnlyOutputRoot -IdfName "material-roof-vegetation-materials-only.idf" -Selector "Materials"
$constructionsOnly = Invoke-OracleCase -Description "Constructions-only" -OutputDirectory $ConstructionsOnlyOutputRoot -IdfName "material-roof-vegetation-constructions-only.idf" -Selector "Constructions"

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$genericRows = @(
    "Material Details,Z USED EXPLICIT ROOF VEGETATION,0.4500,MediumSmooth,0.1800,0.400,641.000,1100.000,0.9500,0.8000,0.7000",
    "Material Details,M DEFAULTED UNUSED ROOF VEGETATION,0.2857,MediumRough,0.1000,0.350,1100.000,1200.000,0.9000,0.7000,0.7500",
    "Material Details,A UNUSED EXPLICIT ROOF VEGETATION,0.9000,MediumSmooth,0.3600,0.400,641.000,1100.000,0.9500,0.8000,0.7000"
)
$genericPrefixes = @(
    "Material Details,Z USED EXPLICIT ROOF VEGETATION,",
    "Material Details,M DEFAULTED UNUSED ROOF VEGETATION,",
    "Material Details,A UNUSED EXPLICIT ROOF VEGETATION,"
)
$ctfHeader = "! <Material CTF Summary>,Material Name,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},ThermalResistance {m2-K/w}"
$usedCtfRow = "Material CTF Summary,Z USED EXPLICIT ROOF VEGETATION,  0.1800,         0.400,    641.000,     1100.000,        0.45"

Assert-CommaSeparatedTokenCount -Row $materialDetailsHeader -Expected 11 -Description "generic material-details header"
foreach ($row in $genericRows) {
    Assert-CommaSeparatedTokenCount -Row $row -Expected 11 -Description "RoofVegetation generic definition row"
}
Assert-CommaSeparatedTokenCount -Row $ctfHeader -Expected 7 -Description "shared material CTF header"
Assert-CommaSeparatedTokenCount -Row $usedCtfRow -Expected 7 -Description "used RoofVegetation CTF row"

$primaryLines = @(Get-Content -LiteralPath $primary.Eio)
$primaryText = $primaryLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "primary generic material-details header"
Assert-ExactOrderedEioRows -Lines $primaryLines -Prefixes $genericPrefixes -Expected $genericRows -Description "primary RoofVegetation generic definition Z,M,A"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "! <Material CTF Summary>," -Expected $ctfHeader -Description "primary shared material CTF header"
Assert-UniqueExactEioRow -Lines $primaryLines -Prefix "Material CTF Summary,Z USED EXPLICIT ROOF VEGETATION," -Expected $usedCtfRow -Description "primary used Z RoofVegetation CTF oracle lock"
Assert-NotContains -Text $primaryText -Pattern "Material CTF Summary,M DEFAULTED UNUSED ROOF VEGETATION," -Description "primary unused M CTF occurrence"
Assert-NotContains -Text $primaryText -Pattern "Material CTF Summary,A UNUSED EXPLICIT ROOF VEGETATION," -Description "primary unused A CTF occurrence"
Assert-NotContains -Text $primaryText -Pattern "! <Material:RoofVegetation>" -Description "primary dedicated RoofVegetation EIO table"

$materialsOnlyLines = @(Get-Content -LiteralPath $materialsOnly.Eio)
$materialsOnlyText = $materialsOnlyLines -join [Environment]::NewLine
Assert-UniqueExactEioRow -Lines $materialsOnlyLines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "Materials-only generic material-details header"
Assert-ExactOrderedEioRows -Lines $materialsOnlyLines -Prefixes $genericPrefixes -Expected $genericRows -Description "Materials-only RoofVegetation generic definition Z,M,A"
Assert-NotContains -Text $materialsOnlyText -Pattern "! <Material CTF Summary>," -Description "Materials-only material CTF header"
Assert-NotContains -Text $materialsOnlyText -Pattern "Material CTF Summary,Z USED EXPLICIT ROOF VEGETATION," -Description "Materials-only used Z CTF occurrence"
Assert-NotContains -Text $materialsOnlyText -Pattern "! <Material:RoofVegetation>" -Description "Materials-only dedicated RoofVegetation EIO table"

$constructionsOnlyLines = @(Get-Content -LiteralPath $constructionsOnly.Eio)
$constructionsOnlyText = $constructionsOnlyLines -join [Environment]::NewLine
Assert-NotContains -Text $constructionsOnlyText -Pattern "! <Material Details>," -Description "Constructions-only generic material-details header"
foreach ($prefix in $genericPrefixes) {
    Assert-NotContains -Text $constructionsOnlyText -Pattern $prefix -Description "Constructions-only RoofVegetation generic definition"
}
Assert-UniqueExactEioRow -Lines $constructionsOnlyLines -Prefix "! <Material CTF Summary>," -Expected $ctfHeader -Description "Constructions-only shared material CTF header"
Assert-UniqueExactEioRow -Lines $constructionsOnlyLines -Prefix "Material CTF Summary,Z USED EXPLICIT ROOF VEGETATION," -Expected $usedCtfRow -Description "Constructions-only used Z RoofVegetation CTF oracle lock"
Assert-NotContains -Text $constructionsOnlyText -Pattern "Material CTF Summary,M DEFAULTED UNUSED ROOF VEGETATION," -Description "Constructions-only unused M CTF occurrence"
Assert-NotContains -Text $constructionsOnlyText -Pattern "Material CTF Summary,A UNUSED EXPLICIT ROOF VEGETATION," -Description "Constructions-only unused A CTF occurrence"
Assert-NotContains -Text $constructionsOnlyText -Pattern "! <Material:RoofVegetation>" -Description "Constructions-only dedicated RoofVegetation EIO table"

$primaryComparison = "Skipped by -SkipRustComparison after exact oracle validation."
$materialsOnlyComparison = $primaryComparison
$constructionsOnlyComparison = $primaryComparison
if (-not $SkipRustComparison) {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }
    $primaryComparison = Invoke-RoofVegetationComparison -CargoPath $cargo.Source -EpJsonPath $primary.EpJson -EioPath $primary.Eio -Description "primary Constructions-and-Materials"
    $materialsOnlyComparison = Invoke-RoofVegetationComparison -CargoPath $cargo.Source -EpJsonPath $materialsOnly.EpJson -EioPath $materialsOnly.Eio -Description "Materials-only"
    $constructionsOnlyComparison = Invoke-RoofVegetationComparison -CargoPath $cargo.Source -EpJsonPath $constructionsOnly.EpJson -EioPath $constructionsOnly.Eio -Description "Constructions-only"

    Assert-Contains -Text $primaryComparison -Pattern "materials_report_requested: true" -Description "primary Rust Materials selector"
    Assert-Contains -Text $primaryComparison -Pattern "constructions_report_requested: true" -Description "primary Rust Constructions selector"
    Assert-Contains -Text $primaryComparison -Pattern "oracle_roof_vegetation_rows: 3" -Description "primary Rust RoofVegetation rows"
    Assert-Contains -Text $primaryComparison -Pattern "oracle_material_detail_rows: 4" -Description "primary Rust complete generic rows"
    Assert-Contains -Text $primaryComparison -Pattern "material_details_header_rows: 1" -Description "primary Rust generic header"

    Assert-Contains -Text $materialsOnlyComparison -Pattern "materials_report_requested: true" -Description "Materials-only Rust Materials selector"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "constructions_report_requested: false" -Description "Materials-only Rust Constructions selector"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_roof_vegetation_rows: 3" -Description "Materials-only Rust RoofVegetation rows"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "oracle_material_detail_rows: 4" -Description "Materials-only Rust complete generic rows"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "material_details_header_rows: 1" -Description "Materials-only Rust generic header"

    Assert-Contains -Text $constructionsOnlyComparison -Pattern "materials_report_requested: false" -Description "Constructions-only Rust Materials selector"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "constructions_report_requested: true" -Description "Constructions-only Rust Constructions selector"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "oracle_roof_vegetation_rows: 0" -Description "Constructions-only Rust RoofVegetation rows"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "oracle_material_detail_rows: 0" -Description "Constructions-only Rust complete generic rows"
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "material_details_header_rows: 0" -Description "Constructions-only Rust generic-header boundary"
}

$reportLines = @(
    "# Material roof vegetation smoke report",
    "",
    "- Case: material_roof_vegetation_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Runtime claim: false",
    "- Plant-physics claim: false",
    "- Moisture/water-balance claim: false",
    "- Construction and surface-behavior claims: false",
    "- Tolerance mode: exact",
    "",
    "## Exact primary oracle fixture evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    $ctfHeader,
    $usedCtfRow,
    "~~~",
    "",
    "The generic Material Details rows are exact one-per-definition echoes in fixture IDF source order Z, M, A. M and A are unused by every construction but remain present exactly once.",
    "Each generic header and row has eleven comma-separated tokens. The row shape exposes only identity and the shared dry-soil base material projections; it is not a dedicated Material:RoofVegetation object-type discriminator.",
    "The shared Material CTF Summary header and used Z row are oracle-only fixture-integrity locks. They add no Rust construction, CTF, surface, or runtime claim and are not a case output proof variable.",
    "",
    "## Materials-only reporting boundary",
    "",
    "The clean Materials-only run retains the exact generic Z, M, A rows and emits no Material CTF Summary header or data rows.",
    "",
    "## Constructions-only reporting boundary",
    "",
    "The clean Constructions-only run emits no generic Material Details header or data rows and retains exactly one shared Material CTF Summary occurrence for used Z; unused M and A remain absent.",
    "",
    "## Bounded typed-input comparisons",
    "",
    "### Primary",
    "~~~text",
    $primaryComparison,
    "~~~",
    "",
    "### Materials-only",
    "~~~text",
    $materialsOnlyComparison,
    "~~~",
    "",
    "### Constructions-only",
    "~~~text",
    $constructionsOnlyComparison,
    "~~~",
    "",
    "Plant height, leaf properties, stomatal resistance, soil-layer label, saturation/residual/initial moisture, initial-moisture recovery, diffusion method, EcoRoof calculations, water balance, precipitation/irrigation, surface heat balance, and runtime are absent from the compared generic rows and remain explicit nonclaims.",
    "",
    "This report is non-blocking diagnostic-only static material evidence and makes no Material:RoofVegetation conformance, construction behavior, surface behavior, runtime, Rust EIO serialization, broad declaration-order, or diagnostic-text claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "Material:RoofVegetation report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Material roof vegetation smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Conformance claim: false" -Description "report conformance boundary"
Assert-Contains -Text $reportText -Pattern "Tolerance mode: exact" -Description "report exact tolerance mode"
Assert-Contains -Text $reportText -Pattern ($genericRows -join [Environment]::NewLine) -Description "report exact generic RoofVegetation rows"
Assert-Contains -Text $reportText -Pattern $usedCtfRow -Description "report used Z CTF oracle lock"
Assert-Contains -Text $reportText -Pattern "fixture IDF source order Z, M, A" -Description "report generic definition order"
Assert-Contains -Text $reportText -Pattern "emits no Material CTF Summary header or data rows" -Description "report Materials-only activation boundary"
Assert-Contains -Text $reportText -Pattern "emits no generic Material Details header or data rows" -Description "report Constructions-only activation boundary"
Assert-Contains -Text $reportText -Pattern "add no Rust construction, CTF, surface, or runtime claim" -Description "report CTF oracle-only boundary"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
if (-not $SkipRustComparison) {
    Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
    Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"
}

Write-Host "Material:RoofVegetation oracle and comparison smoke passed."
if ($SkipRustComparison) {
    Write-Host "Rust comparison was skipped explicitly; all three EnergyPlus selector and exact-row gates passed."
}
Write-Host "Diagnostic-only, nonblocking evidence; no RoofVegetation construction, surface, runtime, plant-physics, moisture-balance, or conformance claim."
Write-Host "Report: $ReportPath"
