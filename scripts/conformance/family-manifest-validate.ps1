[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Assert-ContainsLiteral {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text.IndexOf($Needle, [System.StringComparison]::Ordinal) -lt 0) {
        throw "$Description missing in $Path"
    }
}

function Get-TomlString {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $pattern = '(?m)^\s*' + [regex]::Escape($Key) + '\s*=\s*"(?<value>[^"]*)"'
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        throw "$Description missing key: $Key"
    }
    return $match.Groups["value"].Value
}

function Get-MemberBlocks {
    param([Parameter(Mandatory = $true)][string]$Text)
    return @([regex]::Matches($Text, '(?ms)^\[\[members\]\]\s*(?<body>.*?)(?=^\[\[members\]\]|\z)'))
}

function Get-MemberString {
    param(
        [Parameter(Mandatory = $true)][string]$MemberText,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Description
    )

    return Get-TomlString -Text $MemberText -Key $Key -Description $Description
}

function Resolve-FamilyReference {
    param(
        [Parameter(Mandatory = $true)][string]$FamilyDirectory,
        [Parameter(Mandatory = $true)][string]$Reference
    )

    if ([System.IO.Path]::IsPathRooted($Reference)) {
        $candidate = [System.IO.Path]::GetFullPath($Reference)
    }
    else {
        $candidate = [System.IO.Path]::GetFullPath((Join-Path $FamilyDirectory $Reference))
    }

    if (-not $candidate.StartsWith($RepoRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Family manifest reference escapes the repository: $Reference"
    }

    return $candidate
}

function Assert-FileReference {
    param(
        [Parameter(Mandatory = $true)][string]$FamilyDirectory,
        [Parameter(Mandatory = $true)][string]$Reference,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $path = Resolve-FamilyReference -FamilyDirectory $FamilyDirectory -Reference $Reference
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "$Description missing: $Reference"
    }
}

$familyFiles = @(Get-ChildItem -LiteralPath (Join-Path $RepoRoot "data\conformance_families") -Recurse -Filter "family.toml" | Sort-Object FullName)
if ($familyFiles.Count -eq 0) {
    throw "No conformance family manifests found."
}

$requiredTopLevelLiterals = @(
    'schema = "rusted-energyplus.case-family.v1"',
    'varied_parameters = [',
    'invariant_capabilities = [',
    'family_required_variables = [',
    'family_tolerances = [',
    'family_not_claimed = [',
    'aggregation_report_path = "',
    'regression_rule = "',
    'regression_policy = "',
    'report_path = "'
)

$familiesValidated = 0
$membersValidated = 0

foreach ($familyFile in $familyFiles) {
    $relative = $familyFile.FullName.Substring($RepoRoot.Length).TrimStart("\", "/")
    Write-Host "Validating family manifest: $relative"

    $text = Read-RepoText -Path $familyFile.FullName
    foreach ($literal in $requiredTopLevelLiterals) {
        Assert-ContainsLiteral -Path $familyFile.FullName -Needle $literal -Description "family manifest field $literal"
    }

    $id = Get-TomlString -Text $text -Key "id" -Description "family manifest"
    $familyId = Get-TomlString -Text $text -Key "family_id" -Description "family manifest"
    if ($id -ne $familyId) {
        throw "family_id must match id in $relative"
    }

    $primaryCase = Get-TomlString -Text $text -Key "primary_case" -Description "family manifest"
    $baseCase = Get-TomlString -Text $text -Key "base_case_id" -Description "family manifest"
    if ($primaryCase -ne $baseCase) {
        throw "base_case_id must match primary_case in $relative"
    }

    $sharedOutputs = Get-TomlString -Text $text -Key "shared_output_requests" -Description "family manifest"
    Assert-FileReference -FamilyDirectory $familyFile.DirectoryName -Reference $sharedOutputs -Description "family shared output request file"

    $memberBlocks = Get-MemberBlocks -Text $text
    if ($memberBlocks.Count -eq 0) {
        throw "Family manifest has no members: $relative"
    }

    $sawBaseCase = $false
    foreach ($memberBlock in $memberBlocks) {
        $body = $memberBlock.Groups["body"].Value
        $caseId = Get-MemberString -MemberText $body -Key "case_id" -Description "$relative member"
        $caseToml = Get-MemberString -MemberText $body -Key "case_toml" -Description "$relative member $caseId"
        $outputRequests = Get-MemberString -MemberText $body -Key "output_requests" -Description "$relative member $caseId"
        $parameterDelta = Get-MemberString -MemberText $body -Key "parameter_delta" -Description "$relative member $caseId"
        $expectedStatus = Get-MemberString -MemberText $body -Key "expected_status" -Description "$relative member $caseId"

        if ([string]::IsNullOrWhiteSpace($parameterDelta)) {
            throw "Empty parameter_delta for $caseId in $relative"
        }
        if ([string]::IsNullOrWhiteSpace($expectedStatus)) {
            throw "Empty expected_status for $caseId in $relative"
        }
        if ($caseId -eq $baseCase) {
            $sawBaseCase = $true
        }

        Assert-FileReference -FamilyDirectory $familyFile.DirectoryName -Reference $caseToml -Description "family case manifest for $caseId"
        Assert-FileReference -FamilyDirectory $familyFile.DirectoryName -Reference $outputRequests -Description "family output requests for $caseId"
        $membersValidated += 1
    }

    if (-not $sawBaseCase) {
        throw "base_case_id is not present as a family member in $relative"
    }

    $reportCommand = Get-TomlString -Text $text -Key "report_command" -Description "family manifest"
    $commandName = [System.IO.Path]::GetFileNameWithoutExtension($reportCommand)
    if ($reportCommand -match 'scripts/dev\.cmd\s+(?<name>\S+)') {
        $commandName = $Matches["name"]
    }
    $reportScript = Join-Path $RepoRoot "scripts\conformance\$commandName.ps1"
    if (-not (Test-Path -LiteralPath $reportScript -PathType Leaf)) {
        throw "Family report script missing for ${id}: $reportScript"
    }

    Assert-ContainsLiteral -Path $reportScript -Needle "parameter_variations" -Description "$id family report JSON parameter variation metadata"
    Assert-ContainsLiteral -Path $reportScript -Needle "parameter-error-scatter.svg" -Description "$id family report parameter scatter plot"
    $familiesValidated += 1
}

Write-Host "Family manifest validation"
Write-Host "  families: $familiesValidated"
Write-Host "  members: $membersValidated"
Write-Host "  schema: rusted-energyplus.case-family.v1"
Write-Host "  status: valid"
