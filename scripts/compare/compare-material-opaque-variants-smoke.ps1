[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-material-opaque-variants\26.1.0"
$ReportRoot = Join-Path $RepoRoot ".runtime\conformance\material_opaque_variants_001"
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
    & $FilePath @Arguments
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
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-ConstructionBlock {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][int]$LayerCount,
        [Parameter(Mandatory = $true)][string]$OutsideMaterial,
        [Parameter(Mandatory = $true)][object[]]$ExpectedLayers
    )

    $marker = "  construction: $Name declared_layers: $LayerCount oracle_layers: $LayerCount rust_layers: $LayerCount"
    $start = $Text.IndexOf($marker, [System.StringComparison]::Ordinal)
    if ($start -lt 0) {
        Write-Host $Text
        throw "Missing construction block`: $marker"
    }

    $nextConstruction = $Text.IndexOf("`n  construction:", $start + $marker.Length, [System.StringComparison]::Ordinal)
    $divergence = $Text.IndexOf("`n  first_divergence:", $start + $marker.Length, [System.StringComparison]::Ordinal)
    $end = $Text.Length
    foreach ($candidate in @($nextConstruction, $divergence)) {
        if ($candidate -ge 0 -and $candidate -lt $end) {
            $end = $candidate
        }
    }
    $block = $Text.Substring($start, $end - $start)

    Assert-Contains -Text $block -Pattern "material: $OutsideMaterial/$OutsideMaterial" -Description "$Name outside material"
    Assert-Contains -Text $block -Pattern "status: pass" -Description "$Name comparison status"

    $searchOffset = 0
    for ($index = 0; $index -lt $ExpectedLayers.Count; $index += 1) {
        $layer = $ExpectedLayers[$index]
        $ordinal = $index + 1
        $pattern = "    layer: $ordinal material: $($layer.Name)/$($layer.Name) eio_format: $($layer.Format) thermal_resistance_m2_k_per_w: $($layer.Resistance)/$($layer.Resistance)"
        $position = $block.IndexOf($pattern, $searchOffset, [System.StringComparison]::Ordinal)
        if ($position -lt 0) {
            Write-Host $block
            throw "Missing or out-of-order $Name layer $ordinal`: $pattern"
        }
        Write-Host "OK $Name ordered layer $ordinal`: $pattern"
        $searchOffset = $position + $pattern.Length
    }
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
$weather = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
foreach ($path in @($energyPlus, $converter, $weather)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot
Remove-RepoDirectory -Path $ReportRoot
New-Directory -Path $OutputRoot
New-Directory -Path $ReportRoot

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\material_opaque_variants_001\material_opaque_variants.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing material opaque variants fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "material-opaque-variants.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus material opaque variants oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eio = Join-Path $OutputRoot "eplusout.eio"
$err = Join-Path $OutputRoot "eplusout.err"
foreach ($path in @($eio, $err)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "EnergyPlus did not produce required output: $path"
    }
}
$oracleLog = Get-Content -LiteralPath $err -Raw
Assert-Contains -Text $oracleLog -Pattern "EnergyPlus Completed Successfully--" -Description "oracle completion"
Assert-Contains -Text $oracleLog -Pattern "0 Severe Errors" -Description "oracle severe-error count"

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("material-opaque-variants.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "material-opaque-variants.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce material-opaque-variants.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing ordered Rust construction/material layers with EnergyPlus EIO."
$output = & $cargo.Source run -p ep_cli --quiet -- compare construction-materials $epjson $eio 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Material opaque variants comparison smoke failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Construction Material Comparison" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: smoke" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: absolute-0.001" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "constructions: 4" -Description "construction count"
Assert-Contains -Text $text -Pattern "oracle_constructions: 4" -Description "oracle construction count"
Assert-Contains -Text $text -Pattern "materials: 10" -Description "Rust layer occurrence count"
Assert-Contains -Text $text -Pattern "oracle_materials: 10" -Description "oracle layer occurrence count"

Assert-ConstructionBlock -Text $text -Name "EXTERIOR SOLID" -LayerCount 1 -OutsideMaterial "EXTERIOR REGULAR" -ExpectedLayers @(
    [pscustomobject]@{ Name = "EXTERIOR REGULAR"; Format = "material"; Resistance = "0.100000" }
)
Assert-ConstructionBlock -Text $text -Name "MIXED OPAQUE" -LayerCount 4 -OutsideMaterial "OPAQUE OUTSIDE REGULAR" -ExpectedLayers @(
    [pscustomobject]@{ Name = "OPAQUE OUTSIDE REGULAR"; Format = "material"; Resistance = "0.050000" },
    [pscustomobject]@{ Name = "OPAQUE GAP ONE"; Format = "air"; Resistance = "0.180000" },
    [pscustomobject]@{ Name = "OPAQUE GAP TWO"; Format = "air"; Resistance = "0.120000" },
    [pscustomobject]@{ Name = "OPAQUE INSIDE REGULAR"; Format = "material"; Resistance = "0.200000" }
)
Assert-ConstructionBlock -Text $text -Name "MIXED OPAQUE REVERSE" -LayerCount 4 -OutsideMaterial "OPAQUE INSIDE REGULAR" -ExpectedLayers @(
    [pscustomobject]@{ Name = "OPAQUE INSIDE REGULAR"; Format = "material"; Resistance = "0.200000" },
    [pscustomobject]@{ Name = "OPAQUE GAP TWO"; Format = "air"; Resistance = "0.120000" },
    [pscustomobject]@{ Name = "OPAQUE GAP ONE"; Format = "air"; Resistance = "0.180000" },
    [pscustomobject]@{ Name = "OPAQUE OUTSIDE REGULAR"; Format = "material"; Resistance = "0.050000" }
)
Assert-ConstructionBlock -Text $text -Name "IRT ONLY" -LayerCount 1 -OutsideMaterial "IRT MATERIAL" -ExpectedLayers @(
    [pscustomobject]@{ Name = "IRT MATERIAL"; Format = "material"; Resistance = "0.010000" }
)

$finalPassMarker = "  first_divergence: none`n  status: pass"
Assert-Contains -Text $text -Pattern $finalPassMarker -Description "final no-divergence pass status"

$report = @(
    "# Material opaque variants smoke report",
    "",
    '- Case: `material_opaque_variants_001`',
    "- Oracle: EnergyPlus 26.1.0",
    '- Comparison class: `smoke`',
    '- Conformance claim: `false`',
    "",
    "## Construction/material comparison",
    "",
    "~~~text",
    $text,
    "~~~",
    ""
) -join "`n"
$utf8WithoutBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($ReportPath, $report, $utf8WithoutBom)

if (-not (Test-Path -LiteralPath $ReportPath -PathType Leaf)) {
    throw "Material opaque variants report was not written: $ReportPath"
}
$reportText = Get-Content -LiteralPath $ReportPath -Raw
Assert-Contains -Text $reportText -Pattern "# Material opaque variants smoke report" -Description "report heading"
Assert-Contains -Text $reportText -Pattern $finalPassMarker -Description "report final pass marker"

Write-Host "Material opaque variants comparison smoke passed."
Write-Host "Report: $ReportPath"
