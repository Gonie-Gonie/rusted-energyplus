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
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-window-material-gap\26.1.0"
$MaterialsOnlyOutputRoot = Join-Path $OutputRoot "materials-only"
$ConstructionsOnlyOutputRoot = Join-Path $OutputRoot "constructions-only"
$DefaultOutputRoot = Join-Path $OutputRoot "default"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\window_material_gap_001"
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
        throw "EnergyPlus emitted a warning or severe diagnostic in the $Description run."
    }
}

function Assert-NoSpecializedWindowEvidence {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )
    foreach ($pattern in @(
        "! <WindowMaterial:Gap>,",
        "WindowMaterial:Gap,",
        "! <WindowMaterial:Glazing>",
        "WindowMaterial:Glazing,",
        "! <WindowConstruction>",
        "WindowConstruction,"
    )) {
        Assert-NotContains -Text $Text -Pattern $pattern -Description "$Description specialized/window evidence"
    }
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
            throw "Could not derive $Description fixture because the primary selector was not found."
        }
        $utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText($idfPath, $derivedText, $utf8WithoutBom)
    }

    Write-Host "Running EnergyPlus WindowMaterial:Gap $Description oracle case."
    Invoke-External -FilePath $energyPlus -Arguments @("-D", "-d", $OutputDirectory, $idfPath)
    $eioPath = Join-Path $OutputDirectory "eplusout.eio"
    if (-not (Test-Path -LiteralPath $eioPath -PathType Leaf)) {
        throw "EnergyPlus did not produce required $Description EIO: $eioPath"
    }
    Assert-CleanOracleLog -Path (Join-Path $OutputDirectory "eplusout.err") -Description $Description

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
    return [pscustomobject]@{ Eio = $eioPath; EpJson = $epJsonPath }
}

function Invoke-GapComparison {
    param(
        [Parameter(Mandatory = $true)][string]$CargoPath,
        [Parameter(Mandatory = $true)][string]$EpJsonPath,
        [Parameter(Mandatory = $true)][string]$EioPath,
        [Parameter(Mandatory = $true)][string]$Description
    )
    Write-Host "Comparing bounded Rust WindowMaterial:Gap inputs with $Description EIO evidence."
    $output = & $CargoPath run -p ep_cli --quiet -- compare window-material-gap $EpJsonPath $EioPath --tolerance exact 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "WindowMaterial:Gap $Description comparison smoke failed."
    }
    $text = $output -join [Environment]::NewLine
    foreach ($pattern in @(
        "Window Material Gap Comparison",
        "case_id: window_material_gap_001",
        "comparison_class: smoke",
        "evidence: diagnostic-only",
        "blocking: false",
        "conformance_claim: false",
        "runtime_claim: false",
        "gas_state_reporting_claim: false",
        "pressure_reporting_claim: false",
        "deflection_reporting_claim: false",
        "support_pillar_reporting_claim: false",
        "construction_use_claim: false",
        "specialized_gap_claim: false",
        "specialized_glazing_claim: false",
        "window_construction_claim: false",
        "window_thermal_claim: false",
        "rust_eio_serialization_claim: false",
        "broad_idf_declaration_order_claim: false",
        "tolerance_mode: exact",
        "tolerance_policy: energyplus-26.1-window-material-gap-material-details-4R-normalized-exact",
        "material_objects: 3",
        "specialized_gap_header_rows: 0",
        "specialized_gap_rows: 0",
        "specialized_glazing_header_rows: 0",
        "specialized_glazing_rows: 0",
        "window_construction_header_rows: 0",
        "window_construction_rows: 0",
        "first_divergence: none",
        "status: pass"
    )) {
        Assert-Contains -Text $text -Pattern $pattern -Description "$Description comparison contract"
    }
    return $text
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
foreach ($path in @($energyPlus, $converter)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}
$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\window_material_gap_001\window_material_gap.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing WindowMaterial:Gap fixture: $fixtureIdf"
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$primary = Invoke-OracleCase -Description "primary Constructions-and-Materials" -OutputDirectory $OutputRoot -IdfName "window-material-gap.idf" -SelectorObject "Output:Constructions,Constructions,Materials;"
$materialsOnly = Invoke-OracleCase -Description "Materials-only" -OutputDirectory $MaterialsOnlyOutputRoot -IdfName "window-material-gap-materials-only.idf" -SelectorObject "Output:Constructions,Materials;"
$constructionsOnly = Invoke-OracleCase -Description "Constructions-only" -OutputDirectory $ConstructionsOnlyOutputRoot -IdfName "window-material-gap-constructions-only.idf" -SelectorObject "Output:Constructions,Constructions;"
$default = Invoke-OracleCase -Description "blank/default-selector" -OutputDirectory $DefaultOutputRoot -IdfName "window-material-gap-default.idf" -SelectorObject "Output:Constructions,;"

$materialDetailsHeader = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible"
$materialAirHeader = "! <Material:Air>,Material Name,ThermalResistance {m2-K/w}"
$gapRows = @(
    "Material Details,Z DEFAULT PRESSURE,0.0000,Rough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,M SAME THICKNESS DIFFERENT STATE,0.0000,Rough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000",
    "Material Details,A DIFFERENT THICKNESS,0.0000,Rough,6.0000E-003,0.000,0.000,0.000,0.0000,0.0000,0.0000"
)
$gapPrefixes = @(
    "Material Details,Z DEFAULT PRESSURE,",
    "Material Details,M SAME THICKNESS DIFFERENT STATE,",
    "Material Details,A DIFFERENT THICKNESS,"
)
Assert-CommaSeparatedTokenCount -Row $materialDetailsHeader -Expected 11 -Description "generic material-details header"
foreach ($row in $gapRows) {
    Assert-CommaSeparatedTokenCount -Row $row -Expected 11 -Description "WindowMaterial:Gap generic definition row"
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
        Assert-UniqueExactEioRow -Lines $lines -Prefix "! <Material:Air>," -Expected $materialAirHeader -Description "$($lane.Name) shared material-air header"
        Assert-ExactOrderedEioRows -Lines $lines -Prefixes $gapPrefixes -Expected $gapRows -Description "$($lane.Name) WindowMaterial:Gap Z,M,A"
    }
    else {
        Assert-NotContains -Text $text -Pattern "! <Material Details>," -Description "$($lane.Name) generic material-details header"
        Assert-NotContains -Text $text -Pattern "Material Details," -Description "$($lane.Name) generic material-details data"
        Assert-NotContains -Text $text -Pattern "! <Material:Air>," -Description "$($lane.Name) shared material-air header"
    }
    Assert-NoSpecializedWindowEvidence -Text $text -Description $lane.Name
}

$skippedComparison = "Skipped by -SkipRustComparison after exact oracle validation."
$comparisons = @($skippedComparison, $skippedComparison, $skippedComparison, $skippedComparison)
if (-not $SkipRustComparison) {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -eq $cargo) {
        throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
    }
    $comparisons = @(
        (Invoke-GapComparison -CargoPath $cargo.Source -EpJsonPath $primary.EpJson -EioPath $primary.Eio -Description "primary Constructions-and-Materials"),
        (Invoke-GapComparison -CargoPath $cargo.Source -EpJsonPath $materialsOnly.EpJson -EioPath $materialsOnly.Eio -Description "Materials-only"),
        (Invoke-GapComparison -CargoPath $cargo.Source -EpJsonPath $constructionsOnly.EpJson -EioPath $constructionsOnly.Eio -Description "Constructions-only"),
        (Invoke-GapComparison -CargoPath $cargo.Source -EpJsonPath $default.EpJson -EioPath $default.Eio -Description "blank/default-selector")
    )
    foreach ($comparison in $comparisons[0..1]) {
        foreach ($pattern in @("materials_report_requested: true", "oracle_window_material_gap_rows: 3", "oracle_material_detail_rows: 5", "material_details_header_rows: 1")) {
            Assert-Contains -Text $comparison -Pattern $pattern -Description "Materials-enabled comparison"
        }
    }
    Assert-Contains -Text $comparisons[0] -Pattern "constructions_report_requested: true" -Description "primary Constructions selector"
    Assert-Contains -Text $comparisons[1] -Pattern "constructions_report_requested: false" -Description "Materials-only Constructions selector"
    foreach ($comparison in $comparisons[2..3]) {
        foreach ($pattern in @("materials_report_requested: false", "oracle_window_material_gap_rows: 0", "oracle_material_detail_rows: 0", "material_details_header_rows: 0")) {
            Assert-Contains -Text $comparison -Pattern $pattern -Description "Materials-disabled comparison"
        }
    }
    Assert-Contains -Text $comparisons[2] -Pattern "constructions_report_requested: true" -Description "Constructions-only selector"
    Assert-Contains -Text $comparisons[3] -Pattern "constructions_report_requested: false" -Description "default Constructions selector"
}

$reportLines = @(
    "# Window material gap smoke report",
    "",
    "- Case: window_material_gap_001",
    "- Oracle: EnergyPlus 26.1.0",
    "- Evidence level: diagnostic-only",
    "- Blocking: false",
    "- Conformance claim: false",
    "- Runtime claim: false",
    "- Complex-fenestration construction claim: false",
    "- Tolerance mode: exact",
    "",
    "## Exact Materials-enabled target evidence",
    "",
    "~~~text",
    "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;",
    $materialDetailsHeader,
    $materialAirHeader
) + $gapRows + @(
    "~~~",
    "",
    "Both Materials-enabled lanes emit the exact generic header and exactly three target rows in fixture IDF source order Z, M, A.",
    "The source gas and gas-mixture definitions contribute separate generic rows; the target-row assertion intentionally filters only WindowMaterial:Gap identities.",
    "Z and M have the same own thickness but different identities, gas, pressure, deflection, and pillar state; their identical numeric payloads prove only the bounded generic projection.",
    "Constructions-only and blank/default-selector lanes emit no Material Details header or data rows.",
    "All four lanes omit dedicated WindowMaterial:Gap, WindowMaterial:Glazing, and WindowConstruction headers and data rows.",
    "Every oracle lane completes with zero warnings and zero severe errors and produces a converted epJSON artifact.",
    "",
    "## Bounded typed-input comparisons",
    "",
    "### Primary Constructions-and-Materials",
    "~~~text", $comparisons[0], "~~~", "",
    "### Materials-only",
    "~~~text", $comparisons[1], "~~~", "",
    "### Constructions-only",
    "~~~text", $comparisons[2], "~~~", "",
    "### Blank/default selector",
    "~~~text", $comparisons[3], "~~~", "",
    "Gas composition, pressure, deflection, support pillars, complex-fenestration construction use, heat transfer, surfaces, runtime, diagnostics, Rust EIO serialization, and broad IDF/epJSON declaration order remain explicit nonclaims.",
    "",
    "This report is non-blocking diagnostic-only static material evidence and makes no WindowMaterial:Gap conformance claim.",
    ""
)
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, ($reportLines -join [Environment]::NewLine), $utf8WithoutBom)
$reportText = Get-Content -LiteralPath $ReportPath -Raw
foreach ($pattern in @(
    "# Window material gap smoke report",
    "Evidence level: diagnostic-only",
    "Blocking: false",
    "Conformance claim: false",
    "Tolerance mode: exact",
    ($gapRows -join [Environment]::NewLine),
    "fixture IDF source order Z, M, A",
    "emit no Material Details header or data rows",
    "omit dedicated WindowMaterial:Gap, WindowMaterial:Glazing, and WindowConstruction"
)) {
    Assert-Contains -Text $reportText -Pattern $pattern -Description "report contract"
}
Assert-NotContains -Text $reportText -Pattern "System.Object[]" -Description "unflattened report row array"
if (-not $SkipRustComparison) {
    Assert-Contains -Text $reportText -Pattern "first_divergence: none" -Description "report no-divergence marker"
    Assert-Contains -Text $reportText -Pattern "status: pass" -Description "report pass marker"
}

Write-Host "WindowMaterial:Gap oracle and comparison smoke passed."
if ($SkipRustComparison) {
    Write-Host "Rust comparison was skipped explicitly; all four exact oracle selector and absence gates passed."
}
Write-Host "Diagnostic-only, nonblocking evidence; no gas-state, complex-construction, runtime, or conformance claim."
Write-Host "Report: $ReportPath"
