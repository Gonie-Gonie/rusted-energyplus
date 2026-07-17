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
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-simple-glazing-system\26.1.0"
$MaterialsOnlyOutputRoot = Join-Path $OutputRoot "materials-only"
$ConstructionsOnlyOutputRoot = Join-Path $OutputRoot "constructions-only"
$DefaultOutputRoot = Join-Path $OutputRoot "default"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_simple_glazing_system_001"
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

function Assert-NoSpecializedWindowEvidence {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-NotContains -Text $Text -Pattern "! <WindowMaterial:Glazing>" -Description "$Description specialized glazing header"
    Assert-NotContains -Text $Text -Pattern "WindowMaterial:Glazing," -Description "$Description specialized glazing data"
    Assert-NotContains -Text $Text -Pattern "! <WindowConstruction>" -Description "$Description window-construction header"
    Assert-NotContains -Text $Text -Pattern "WindowConstruction," -Description "$Description window-construction data"
}

function Invoke-OracleCase {
    param(
        [Parameter(Mandatory = $true)][string]$Description,
        [Parameter(Mandatory = $true)][string]$OutputDirectory,
        [Parameter(Mandatory = $true)][string]$IdfName,
        [Parameter(Mandatory = $true)][string]$SelectorObject
    )

    New-Directory -Path $OutputDirectory
    $idfPath = Join-Path $OutputDirectory $IdfName
    if ($SelectorObject -eq "Output:Constructions,Constructions,Materials;") {
        Copy-Item -LiteralPath $fixtureIdf -Destination $idfPath -Force
    }
    else {
        $fixtureText = [System.IO.File]::ReadAllText($fixtureIdf)
        $derivedText = $fixtureText.Replace(
            "Output:Constructions,Constructions,Materials;",
            $SelectorObject
        )
        if ($derivedText.Equals($fixtureText, [System.StringComparison]::Ordinal)) {
            throw "Could not derive $Description fixture because the exact primary Output:Constructions selector was not found."
        }
        $utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText($idfPath, $derivedText, $utf8WithoutBom)
    }

    Write-Host "Running EnergyPlus WindowMaterial:SimpleGlazingSystem $Description oracle case."
    Invoke-External -FilePath $energyPlus -Arguments @("-D", "-d", $OutputDirectory, $idfPath)

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

function Invoke-SimpleGlazingComparison {
    param(
        [Parameter(Mandatory = $true)][string]$CargoPath,
        [Parameter(Mandatory = $true)][string]$EpJsonPath,
        [Parameter(Mandatory = $true)][string]$EioPath,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Write-Host "Comparing bounded Rust WindowMaterial:SimpleGlazingSystem inputs with $Description EnergyPlus EIO evidence."
    $output = & $CargoPath run -p ep_cli --quiet -- compare window-material-simple-glazing-system $EpJsonPath $EioPath --tolerance exact 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "WindowMaterial:SimpleGlazingSystem $Description comparison smoke failed."
    }
    $text = $output -join [Environment]::NewLine
    Assert-Contains -Text $text -Pattern "Window Material Simple Glazing System Comparison" -Description "$Description comparison header"
    Assert-Contains -Text $text -Pattern "case_id: window_material_simple_glazing_system_001" -Description "$Description case identity"
    Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "$Description comparison class"
    Assert-Contains -Text $text -Pattern "evidence: diagnostic-only" -Description "$Description diagnostic evidence"
    Assert-Contains -Text $text -Pattern "blocking: false" -Description "$Description nonblocking boundary"
    foreach ($claim in @(
        "conformance_claim",
        "runtime_claim",
        "input_u_factor_reporting_claim",
        "input_shgc_reporting_claim",
        "input_visible_transmittance_reporting_claim",
        "specialized_glazing_claim",
        "window_construction_claim",
        "construction_use_claim",
        "window_optics_claim",
        "incident_angle_optics_claim",
        "hemispherical_optics_claim",
        "window_thermal_claim",
        "ratings_claim",
        "surface_behavior_claim",
        "daylighting_claim",
        "rust_eio_serialization_claim",
        "broad_idf_declaration_order_claim"
    )) {
        Assert-Contains -Text $text -Pattern "$($claim): false" -Description "$Description $claim boundary"
    }
    Assert-Contains -Text $text -Pattern "tolerance_mode: exact" -Description "$Description exact tolerance"
    Assert-Contains -Text $text -Pattern "tolerance_policy: energyplus-26.1-simple-glazing-material-details-4R-3R-normalized-exact" -Description "$Description source-format policy"
    Assert-Contains -Text $text -Pattern "material_objects: 3" -Description "$Description typed material count"
    Assert-Contains -Text $text -Pattern "specialized_glazing_header_rows: 0" -Description "$Description specialized-header boundary"
    Assert-Contains -Text $text -Pattern "specialized_glazing_rows: 0" -Description "$Description specialized-row boundary"
    Assert-Contains -Text $text -Pattern "window_construction_header_rows: 0" -Description "$Description window-construction-header boundary"
    Assert-Contains -Text $text -Pattern "window_construction_rows: 0" -Description "$Description window-construction-row boundary"
    Assert-Contains -Text $text -Pattern "first_divergence: none" -Description "$Description first divergence"
    Assert-Contains -Text $text -Pattern "status: pass" -Description "$Description comparison status"
    return $text
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
foreach ($path in @($energyPlus, $converter)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_simple_glazing_system_001\window_material_simple_glazing_system.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:SimpleGlazingSystem fixture: $fixtureIdf"
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$primary = Invoke-OracleCase -Description "primary Constructions-and-Materials" -OutputDirectory $OutputRoot -IdfName "window-material-simple-glazing-system.idf" -SelectorObject "Output:Constructions,Constructions,Materials;"
$materialsOnly = Invoke-OracleCase -Description "Materials-only" -OutputDirectory $MaterialsOnlyOutputRoot -IdfName "window-material-simple-glazing-system-materials-only.idf" -SelectorObject "Output:Constructions,Materials;"
$constructionsOnly = Invoke-OracleCase -Description "Constructions-only" -OutputDirectory $ConstructionsOnlyOutputRoot -IdfName "window-material-simple-glazing-system-constructions-only.idf" -SelectorObject "Output:Constructions,Constructions;"
$default = Invoke-OracleCase -Description "blank/default-selector" -OutputDirectory $DefaultOutputRoot -IdfName "window-material-simple-glazing-system-default.idf" -SelectorObject "Output:Constructions,;"

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$genericRows = @(
    "Material Details,Z MISSING VISIBLE,0.1993,VerySmooth,2.3314E-002,0.117,0.000,0.000,0.8400,0.0000,0.0000",
    "Material Details,M CHANGED OPTICS,0.1993,VerySmooth,2.3314E-002,0.117,0.000,0.000,0.8400,0.0000,0.0000",
    "Material Details,A CHANGED THERMAL,3.3019E-002,VerySmooth,2.0000E-003,6.057E-002,0.000,0.000,0.8400,0.0000,0.0000"
)
$genericPrefixes = @(
    "Material Details,Z MISSING VISIBLE,",
    "Material Details,M CHANGED OPTICS,",
    "Material Details,A CHANGED THERMAL,"
)

Assert-CommaSeparatedTokenCount -Row $materialDetailsHeader -Expected 11 -Description "generic material-details header"
foreach ($row in $genericRows) {
    Assert-CommaSeparatedTokenCount -Row $row -Expected 11 -Description "SimpleGlazing generic definition row"
}

foreach ($lane in @(
    [pscustomobject]@{ Name = "primary"; Result = $primary; Materials = $true },
    [pscustomobject]@{ Name = "Materials-only"; Result = $materialsOnly; Materials = $true },
    [pscustomobject]@{ Name = "Constructions-only"; Result = $constructionsOnly; Materials = $false },
    [pscustomobject]@{ Name = "blank/default-selector"; Result = $default; Materials = $false }
)) {
    $lines = @(Get-Content -LiteralPath $lane.Result.Eio)
    $text = $lines -join [Environment]::NewLine
    if ($lane.Materials) {
        Assert-UniqueExactEioRow -Lines $lines -Prefix "! <Material Details>," -Expected $materialDetailsHeader -Description "$($lane.Name) generic material-details header"
        Assert-ExactOrderedEioRows -Lines $lines -Prefixes $genericPrefixes -Expected $genericRows -Description "$($lane.Name) SimpleGlazing generic definition Z,M,A"
    }
    else {
        Assert-NotContains -Text $text -Pattern "! <Material Details>," -Description "$($lane.Name) generic material-details header"
        Assert-NotContains -Text $text -Pattern "Material Details," -Description "$($lane.Name) generic material-details data"
    }
    Assert-NoSpecializedWindowEvidence -Text $text -Description $lane.Name
}

$skippedComparison = "Skipped by -SkipRustComparison after exact oracle validation."
$primaryComparison = $skippedComparison
$materialsOnlyComparison = $skippedComparison
$constructionsOnlyComparison = $skippedComparison
$defaultComparison = $skippedComparison
if (-not $SkipRustComparison) {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }

    $primaryComparison = Invoke-SimpleGlazingComparison -CargoPath $cargo.Source -EpJsonPath $primary.EpJson -EioPath $primary.Eio -Description "primary Constructions-and-Materials"
    $materialsOnlyComparison = Invoke-SimpleGlazingComparison -CargoPath $cargo.Source -EpJsonPath $materialsOnly.EpJson -EioPath $materialsOnly.Eio -Description "Materials-only"
    $constructionsOnlyComparison = Invoke-SimpleGlazingComparison -CargoPath $cargo.Source -EpJsonPath $constructionsOnly.EpJson -EioPath $constructionsOnly.Eio -Description "Constructions-only"
    $defaultComparison = Invoke-SimpleGlazingComparison -CargoPath $cargo.Source -EpJsonPath $default.EpJson -EioPath $default.Eio -Description "blank/default-selector"

    foreach ($comparison in @($primaryComparison, $materialsOnlyComparison)) {
        Assert-Contains -Text $comparison -Pattern "materials_report_requested: true" -Description "Materials-enabled Rust selector"
        Assert-Contains -Text $comparison -Pattern "oracle_simple_glazing_rows: 3" -Description "Materials-enabled Rust SimpleGlazing rows"
        Assert-Contains -Text $comparison -Pattern "oracle_material_detail_rows: 3" -Description "Materials-enabled Rust complete generic rows"
        Assert-Contains -Text $comparison -Pattern "material_details_header_rows: 1" -Description "Materials-enabled Rust generic header"
    }
    Assert-Contains -Text $primaryComparison -Pattern "constructions_report_requested: true" -Description "primary Rust Constructions selector"
    Assert-Contains -Text $materialsOnlyComparison -Pattern "constructions_report_requested: false" -Description "Materials-only Rust Constructions selector"

    foreach ($comparison in @($constructionsOnlyComparison, $defaultComparison)) {
        Assert-Contains -Text $comparison -Pattern "materials_report_requested: false" -Description "Materials-disabled Rust selector"
        Assert-Contains -Text $comparison -Pattern "oracle_simple_glazing_rows: 0" -Description "Materials-disabled Rust SimpleGlazing rows"
        Assert-Contains -Text $comparison -Pattern "oracle_material_detail_rows: 0" -Description "Materials-disabled Rust generic rows"
        Assert-Contains -Text $comparison -Pattern "material_details_header_rows: 0" -Description "Materials-disabled Rust generic-header boundary"
    }
    Assert-Contains -Text $constructionsOnlyComparison -Pattern "constructions_report_requested: true" -Description "Constructions-only Rust Constructions selector"
    Assert-Contains -Text $defaultComparison -Pattern "constructions_report_requested: false" -Description "default Rust Constructions selector"
}

$reportLines = @(
    "# Window material simple glazing system smoke report",
    "",
    "- Case: window_material_simple_glazing_system_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Runtime claim: false",
    "- Construction/window-optics claim: false",
    "- Tolerance mode: exact",
    "",
    "## Exact Materials-enabled oracle evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader
) + $genericRows + @(
    "~~~",
    "",
    "Both the primary Constructions-and-Materials lane and the Materials-only lane emit this exact eleven-token header and exactly three rows in fixture IDF source order Z, M, A.",
    "Z and M have the same U-factor but different SHGC and visible inputs; their identical generic rows prove only the shared thermal block-layer projection, not input or derived optical reporting.",
    "",
    "## Reporting-selector and specialized-output boundaries",
    "",
    "The Constructions-only and genuinely blank/default-selector lanes emit no Material Details header or data rows.",
    "All four lanes omit WindowMaterial:Glazing and WindowConstruction headers and data because the fixture has no Construction and every SimpleGlazing definition is unused.",
    "Every oracle lane completes with exactly zero warnings and zero severe errors and produces a converted epJSON artifact.",
    "",
    "## Bounded typed-input comparisons",
    "",
    "### Primary Constructions-and-Materials",
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
    "### Blank/default selector",
    "~~~text",
    $defaultComparison,
    "~~~",
    "",
    "SHGC and visible optical reporting, solar/visible reflectance or transmittance, angular/diffuse optics, emissivity faces, dirt factor, solar diffusing, high-U warnings, subnormal-U diagnostics, specialized glazing, construction ratings, window thermal behavior, surfaces, daylighting, runtime, and Rust EIO serialization remain explicit nonclaims.",
    "",
    "This report is non-blocking diagnostic-only static material evidence and makes no WindowMaterial:SimpleGlazingSystem conformance, construction-use, surface, fenestration, runtime, broad declaration-order, or diagnostic-text claim.",
    ""
)
$report = $reportLines -join [Environment]::NewLine
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "WindowMaterial:SimpleGlazingSystem report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Window material simple glazing system smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern "Evidence level: diagnostic-only" -Description "report diagnostic boundary"
Assert-Contains -Text $reportText -Pattern "Blocking: false" -Description "report nonblocking boundary"
Assert-Contains -Text $reportText -Pattern "Conformance claim: false" -Description "report conformance boundary"
Assert-Contains -Text $reportText -Pattern "Tolerance mode: exact" -Description "report exact tolerance mode"
Assert-Contains -Text $reportText -Pattern ($genericRows -join [Environment]::NewLine) -Description "report exact generic SimpleGlazing rows"
Assert-Contains -Text $reportText -Pattern "fixture IDF source order Z, M, A" -Description "report generic definition order"
Assert-Contains -Text $reportText -Pattern "emit no Material Details header or data rows" -Description "report Materials-disabled boundary"
Assert-Contains -Text $reportText -Pattern "omit WindowMaterial:Glazing and WindowConstruction headers and data" -Description "report specialized-output boundary"
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
if (-not $SkipRustComparison) {
    Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
    Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"
}

Write-Host "WindowMaterial:SimpleGlazingSystem oracle and comparison smoke passed."
if ($SkipRustComparison) {
    Write-Host "Rust comparison was skipped explicitly; all four EnergyPlus selector, exact-row, and specialized-absence gates passed."
}
Write-Host "Diagnostic-only, nonblocking evidence; no SimpleGlazing optics, construction, surface, runtime, or conformance claim."
Write-Host "Report: $ReportPath"
