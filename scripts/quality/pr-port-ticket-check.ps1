[CmdletBinding()]
param(
    [string]$BodyPath,
    [string]$Body,
    [string]$ChangedFilesPath,
    [string]$BaseSha,
    [string]$HeadSha,
    [switch]$SelfTest
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..\..")).Path
$script:RoutineCompletionMetadataBootstrapAllowedPaths = @(
    ".github/workflows/pull-request.yml",
    "docs/src/current/current-status.md",
    "docs/src/current/project-contract.md",
    "docs/src/generated/algorithm-ledger.md",
    "docs/src/generated/docs-inventory.md",
    "docs/src/generated/script-index.md",
    "docs/src/porting-map/algorithm-ledger.md",
    "docs/src/porting-map/zone-air-update-map.md",
    "scripts/dev/commands.json",
    "scripts/quality/algorithm-ledger-check.ps1",
    "scripts/quality/check.ps1",
    "scripts/quality/pr-port-ticket-check.ps1",
    "scripts/quality/pr-port-ticket-check/contract-diff.ps1",
    "scripts/quality/pr-port-ticket-check/self-tests.ps1",
    "scripts/quality/project-contract-check.ps1",
    "scripts/quality/strict-no-false-conformance.ps1",
    "specs/algorithm_ledger.toml",
    "specs/project_contract.toml",
    "specs/script_inventory.toml",
    "tools/docs/algorithm_ledger_self_tests.py",
    "tools/docs/fetch_energyplus_reference_subset.py",
    "tools/docs/generate_docs.py",
    "tools/docs/generated-docs.manifest.json",
    "tools/docs/routine_completion_contract.py",
    "tools/docs/testdata/routine-state-map-v1.md",
    "tools/docs/validate_algorithm_ledger.py",
    "tools/docs/validate_project_contract.py"
)

function Get-PrBodyText {
    param(
        [string]$Path,
        [string]$InlineBody
    )
    if (-not [string]::IsNullOrWhiteSpace($Path)) {
        if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
            throw "PR body file does not exist: $Path"
        }
        return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
    }
    if (-not [string]::IsNullOrWhiteSpace($InlineBody)) {
        return $InlineBody
    }
    if (-not [string]::IsNullOrWhiteSpace($env:PR_BODY)) {
        return $env:PR_BODY
    }
    return ""
}

function ConvertTo-NormalizedRepoPath {
    param([Parameter(Mandatory = $true)][string]$Path)

    return (($Path.Trim() -replace "\\", "/") -replace "^\./", "")
}

$changedFilesLibrary = Join-Path $PSScriptRoot "pr-port-ticket-check\changed-files.ps1"
if (-not (Test-Path -LiteralPath $changedFilesLibrary -PathType Leaf)) {
    throw "PR port-ticket changed-file library does not exist: $changedFilesLibrary"
}
. $changedFilesLibrary
$contractDiffLibrary = Join-Path $PSScriptRoot "pr-port-ticket-check\contract-diff.ps1"
if (-not (Test-Path -LiteralPath $contractDiffLibrary -PathType Leaf)) {
    throw "PR port-ticket contract-diff library does not exist: $contractDiffLibrary"
}
. $contractDiffLibrary

function Test-CheckedItem {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Label
    )
    $pattern = "(?im)^[ ]{0,3}-[ \t]*\[[xX]\][ \t]*$([regex]::Escape($Label))[ \t]*\r?$"
    return $Text -match $pattern
}

function Remove-MarkdownCodeFences {
    param([Parameter(Mandatory = $true)][string]$Text)

    $lines = [regex]::Split($Text, "\r?\n")
    $output = [System.Collections.Generic.List[string]]::new()
    $insideFence = $false
    $fenceCharacter = ""
    $fenceLength = 0
    foreach ($line in $lines) {
        if (-not $insideFence) {
            $opening = [regex]::Match($line, '^[ ]{0,3}(?<marker>`{3,}|~{3,})')
            if ($opening.Success) {
                $marker = $opening.Groups["marker"].Value
                $insideFence = $true
                $fenceCharacter = $marker.Substring(0, 1)
                $fenceLength = $marker.Length
                continue
            }
            $output.Add($line)
            continue
        }

        $closingPattern = '^[ ]{{0,3}}{0}{{{1},}}[ \t]*$' -f `
            [regex]::Escape($fenceCharacter), $fenceLength
        if ($line -match $closingPattern) {
            $insideFence = $false
            $fenceCharacter = ""
            $fenceLength = 0
        }
    }
    return $output -join [Environment]::NewLine
}

function Remove-MarkdownHtmlComments {
    param([Parameter(Mandatory = $true)][string]$Text)

    return [regex]::Replace($Text, '(?s)<!--(?:.*?-->|.*\z)', '')
}

function Get-AlgorithmPortTicketSection {
    param([Parameter(Mandatory = $true)][string]$Text)

    $matches = [regex]::Matches(
        $Text,
        '(?ms)^[ ]{0,3}##[ \t]+Algorithm Port Ticket[ \t]*\r?\n(?<body>.*?)(?=^[ ]{0,3}##[ \t]+|\z)'
    )
    if ($matches.Count -ne 1) {
        throw "PR body must contain exactly one level-two Algorithm Port Ticket section."
    }
    return $matches[0].Groups["body"].Value
}

function Get-TicketField {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name
    )
    $pattern = "(?im)^[ ]{0,3}-[ \t]*$([regex]::Escape($Name)):[ \t]*(?<value>.*?)[ \t]*\r?$"
    $matches = [regex]::Matches($Text, $pattern)
    if ($matches.Count -gt 1) {
        throw "Algorithm Port Ticket field must appear exactly once: $Name"
    }
    if ($matches.Count -eq 0) {
        return $null
    }
    return $matches[0].Groups["value"].Value.Trim()
}

function Assert-FieldValue {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name
    )
    $value = Get-TicketField -Text $Text -Name $Name
    if ([string]::IsNullOrWhiteSpace($value)) {
        throw "Algorithm Port Ticket field is required: $Name"
    }
    $placeholderValues = @(
        "compatibility / diagnostic_probe / refactor_only",
        "true / false",
        "yes / no"
    )
    if ($placeholderValues -contains $value.Trim().ToLowerInvariant()) {
        throw "Algorithm Port Ticket field still contains a placeholder: $Name"
    }
    $normalized = $value.Trim().Trim('`').Trim().ToLowerInvariant()
    if (
        $normalized -match "^(todo|tbd|n/?a|not applicable|unknown|placeholder|maybe)(\b|:)" -or
        $normalized -match "^(\?|[-]+)$" -or
        $normalized -match "^<.*>$"
    ) {
        throw "Algorithm Port Ticket field still contains a placeholder: $Name"
    }
    return $value
}

function Assert-ConcreteFieldValue {
    param(
        [Parameter(Mandatory = $true)][string]$Value,
        [Parameter(Mandatory = $true)][string]$Name
    )

    $normalized = $Value.Trim().Trim('`').Trim().ToLowerInvariant()
    if (
        $normalized -match "^(todo|tbd|n/?a|not applicable|none|unknown|placeholder|maybe)(\b|:)" -or
        $normalized -match "^(\?|[-]+)$" -or
        $normalized -match "^<.*>$"
    ) {
        throw "Algorithm Port Ticket field must contain a concrete value: $Name"
    }
}

function Assert-AllowedFieldValue {
    param(
        [Parameter(Mandatory = $true)][string]$Value,
        [Parameter(Mandatory = $true)][string[]]$Allowed,
        [Parameter(Mandatory = $true)][string]$Name
    )

    $normalized = $Value.Trim().ToLowerInvariant()
    if ($Allowed -notcontains $normalized) {
        throw "Algorithm Port Ticket field '$Name' must be one of $($Allowed -join ', '), got '$Value'."
    }
}

function Assert-FieldEquals {
    param(
        [Parameter(Mandatory = $true)][string]$Actual,
        [Parameter(Mandatory = $true)][string]$Expected,
        [Parameter(Mandatory = $true)][string]$Name
    )
    if ($Actual.Trim().ToLowerInvariant() -ne $Expected.ToLowerInvariant()) {
        throw "Algorithm Port Ticket field '$Name' must be '$Expected', got '$Actual'."
    }
}

function Resolve-RepoRelativeFile {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $trimmed = $Path.Trim().Trim('`').Trim()
    if ([System.IO.Path]::IsPathRooted($trimmed)) {
        throw "$Description must be a repo-relative path: $Path"
    }
    $fullPath = [System.IO.Path]::GetFullPath((Join-Path $RepoRoot $trimmed))
    $repoPrefix = $RepoRoot.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar
    if (-not $fullPath.StartsWith($repoPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "$Description escapes the repository: $Path"
    }
    if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
        throw "$Description does not exist: $Path"
    }
    return $fullPath
}

function Assert-TicketLocation {
    param([Parameter(Mandatory = $true)][string]$Value)

    $location = $Value.Trim().Trim('`').Trim()
    if ($location -match "(?i)^PR body(?:#[-a-z0-9_]+)?$") {
        return
    }
    if ($location -match "(?i)^https://github\.com/[^/]+/[^/]+/(issues|pull|blob)/.+$") {
        return
    }
    $pathOnly = ($location -split "#", 2)[0]
    [void](Resolve-RepoRelativeFile -Path $pathOnly -Description "Algorithm Port Ticket location")
}

function Get-AlgorithmLedgerBlock {
    param(
        [Parameter(Mandatory = $true)][string]$AlgorithmId,
        [AllowEmptyString()][string]$LedgerText
    )

    if ([string]::IsNullOrWhiteSpace($LedgerText)) {
        $ledgerPath = Join-Path $RepoRoot "specs\algorithm_ledger.toml"
        $LedgerText = Get-Content -Encoding UTF8 -Raw -LiteralPath $ledgerPath
    }
    $blocks = [regex]::Matches(
        $LedgerText,
        "(?ms)^\[\[algorithm\]\]\s*(?<body>.*?)(?=^\[\[algorithm\]\]|\z)"
    )
    foreach ($blockMatch in $blocks) {
        $block = $blockMatch.Groups["body"].Value
        $idMatch = [regex]::Match($block, '(?m)^id\s*=\s*"(?<value>[^"]+)"\s*$')
        if ($idMatch.Success -and $idMatch.Groups["value"].Value -eq $AlgorithmId) {
            return $block
        }
    }
    throw "Algorithm Port Ticket references an unknown algorithm ledger id: $AlgorithmId"
}

function Get-TomlStringValue {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name
    )

    $match = [regex]::Match(
        $Text,
        "(?m)^$([regex]::Escape($Name))\s*=\s*`"(?<value>[^`"]+)`"\s*$"
    )
    if (-not $match.Success) {
        return $null
    }
    return $match.Groups["value"].Value
}

function Get-TomlSectionBlock {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name
    )

    $match = [regex]::Match(
        $Text,
        "(?ms)^\[$([regex]::Escape($Name))\]\s*(?<body>.*?)(?=^\[|\z)"
    )
    if (-not $match.Success) {
        throw "First target case is missing TOML section: [$Name]"
    }
    return $match.Groups["body"].Value
}

function Get-TomlStringArrayValues {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name
    )

    $arrayMatch = [regex]::Match(
        $Text,
        "(?ms)^$([regex]::Escape($Name))\s*=\s*\[(?<body>.*?)^\]\s*$"
    )
    if (-not $arrayMatch.Success) {
        return @()
    }
    return @(
        [regex]::Matches($arrayMatch.Groups["body"].Value, '"(?<value>[^"]+)"') |
            ForEach-Object { $_.Groups["value"].Value }
    )
}

function Split-TicketList {
    param([Parameter(Mandatory = $true)][string]$Value)

    return @(
        $Value.Split([char[]]@(',', ';'), [System.StringSplitOptions]::RemoveEmptyEntries) |
            ForEach-Object { $_.Trim().Trim('`').Trim() } |
            Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    )
}

function Test-CaseOutputLevel {
    param(
        [Parameter(Mandatory = $true)][string]$CaseText,
        [Parameter(Mandatory = $true)][string]$Variable,
        [Parameter(Mandatory = $true)][string]$Level
    )

    $blocks = [regex]::Matches(
        $CaseText,
        "(?ms)^\[\[outputs\]\]\s*(?<body>.*?)(?=^\[\[outputs\]\]|^\[\[meters\]\]|^\[[^\[]|\z)"
    )
    foreach ($blockMatch in $blocks) {
        $block = $blockMatch.Groups["body"].Value
        if (
            (Get-TomlStringValue -Text $block -Name "variable") -eq $Variable -and
            (Get-TomlStringValue -Text $block -Name "level") -eq $Level
        ) {
            return $true
        }
    }
    return $false
}

function Test-CaseOutputName {
    param(
        [Parameter(Mandatory = $true)][string]$CaseText,
        [Parameter(Mandatory = $true)][string]$Variable
    )

    $blocks = [regex]::Matches(
        $CaseText,
        "(?ms)^\[\[outputs\]\]\s*(?<body>.*?)(?=^\[\[outputs\]\]|^\[\[meters\]\]|^\[[^\[]|\z)"
    )
    return [bool]($blocks | Where-Object {
        (Get-TomlStringValue -Text $_.Groups["body"].Value -Name "variable") -eq $Variable
    })
}

function Test-CaseMeterName {
    param(
        [Parameter(Mandatory = $true)][string]$CaseText,
        [Parameter(Mandatory = $true)][string]$Meter
    )

    $blocks = [regex]::Matches(
        $CaseText,
        "(?ms)^\[\[meters\]\]\s*(?<body>.*?)(?=^\[\[meters\]\]|^\[\[outputs\]\]|^\[[^\[]|\z)"
    )
    return [bool]($blocks | Where-Object {
        (Get-TomlStringValue -Text $_.Groups["body"].Value -Name "name") -eq $Meter
    })
}

function Get-DevCommandName {
    param([Parameter(Mandatory = $true)][string]$Value)

    $normalized = $Value.Trim() -replace '(?i)^scripts[/\\]dev\.(cmd|ps1)\s+', ''
    $match = [regex]::Match($normalized, '^(?<command>[a-z0-9][a-z0-9-]*)(?:\s|$)')
    if (-not $match.Success) {
        throw "Blocking gate must name a registered scripts/dev command: $Value"
    }
    return $match.Groups["command"].Value
}

function Get-NormalizedDevCommandInvocation {
    param([Parameter(Mandatory = $true)][string]$Value)

    [void](Get-DevCommandName -Value $Value)
    $normalized = $Value.Trim() -replace '(?i)^scripts[/\\]dev\.(cmd|ps1)\s+', ''
    return (($normalized -split '\s+') -join ' ')
}

function Assert-SourceMapCodeToken {
    param(
        [Parameter(Mandatory = $true)][string]$SourceMapText,
        [Parameter(Mandatory = $true)][string]$Token,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Token -notmatch '^[A-Za-z_][A-Za-z0-9_]*$' -or $Token -cnotmatch '[A-Z]') {
        throw "$Description must be a case-sensitive EnergyPlus routine or stage identifier: $Token"
    }
    $pattern = '(?m)`[^`\r\n]*(?<![A-Za-z0-9_]){0}(?![A-Za-z0-9_])[^`\r\n]*`' -f `
        [regex]::Escape($Token)
    if (-not [regex]::IsMatch($SourceMapText, $pattern)) {
        throw "$Description is not documented as code by the selected algorithm source map: $Token"
    }
}

function Test-RustFunctionTarget {
    param(
        [Parameter(Mandatory = $true)][string]$RustText,
        [Parameter(Mandatory = $true)][string]$Target
    )

    $parts = @($Target -split "::")
    $functionToken = $parts[-1]
    if ($functionToken -notmatch "^[A-Za-z_][A-Za-z0-9_]*$") {
        return $false
    }
    $functionPattern = "(?m)\bfn\s+$([regex]::Escape($functionToken))\b"
    if ($parts.Count -eq 1) {
        return [regex]::IsMatch($RustText, $functionPattern)
    }
    if ($parts.Count -ne 2) {
        return $false
    }

    $ownerToken = $parts[0]
    if ($ownerToken -notmatch "^[A-Za-z_][A-Za-z0-9_]*$") {
        return $false
    }
    $owner = [regex]::Escape($ownerToken)
    $implPattern = "(?m)^\s*impl(?:\s*<[^>{}\r\n]*>)?\s+(?:(?:[A-Za-z_][A-Za-z0-9_:<>]*\s+for\s+))?${owner}(?:\s*<[^>{}\r\n]*>)?(?:\s+where[^\{]*)?\s*\{"
    foreach ($match in [regex]::Matches($RustText, $implPattern)) {
        $openBraceIndex = $match.Index + $match.Length - 1
        $depth = 0
        $closeBraceIndex = -1
        for ($index = $openBraceIndex; $index -lt $RustText.Length; $index += 1) {
            if ($RustText[$index] -eq '{') {
                $depth += 1
            }
            elseif ($RustText[$index] -eq '}') {
                $depth -= 1
                if ($depth -eq 0) {
                    $closeBraceIndex = $index
                    break
                }
            }
        }
        if ($closeBraceIndex -lt 0) {
            continue
        }
        $bodyStart = $openBraceIndex + 1
        $bodyLength = $closeBraceIndex - $bodyStart
        if ([regex]::IsMatch($RustText.Substring($bodyStart, $bodyLength), $functionPattern)) {
            return $true
        }
    }
    return $false
}

function Assert-TicketReferences {
    param(
        [Parameter(Mandatory = $true)][hashtable]$Values,
        [Parameter(Mandatory = $true)][string]$PortType,
        [string[]]$SourceOrderFiles = @(),
        [object[]]$ChangedFileRecords = @(),
        [string]$BaseRevision
    )

    $algorithmId = $Values["Algorithm ID"].Trim()
    if ($algorithmId -notmatch "^[a-z0-9][a-z0-9_-]*$") {
        throw "Algorithm ID must be a stable lowercase ledger slug: $algorithmId"
    }
    $ledgerBlock = Get-AlgorithmLedgerBlock -AlgorithmId $algorithmId
    $ledgerDomain = Get-TomlStringValue -Text $ledgerBlock -Name "domain"
    Assert-FieldEquals -Actual $Values["Domain"] -Expected $ledgerDomain -Name "Domain"

    $sourceFile = ConvertTo-NormalizedRepoPath -Path $Values["EnergyPlus source file"].Trim('`')
    $ledgerSources = @(Get-TomlStringArrayValues -Text $ledgerBlock -Name "energyplus_source")
    if ($ledgerSources -notcontains $sourceFile) {
        throw "EnergyPlus source file is not mapped by algorithm $algorithmId`: $sourceFile"
    }
    $routine = $Values["EnergyPlus routine"].Trim().Trim('`').Trim()
    $routineToken = ($routine -split "::")[-1]
    $sourceMap = Get-TomlStringValue -Text $ledgerBlock -Name "source_map"
    $sourceMapPath = Resolve-RepoRelativeFile -Path $sourceMap -Description "Algorithm source map"
    $sourceMapText = Get-Content -Encoding UTF8 -Raw -LiteralPath $sourceMapPath
    Assert-SourceMapCodeToken `
        -SourceMapText $sourceMapText `
        -Token $routineToken `
        -Description "EnergyPlus routine"
    $sourceOrderStage = $Values["EnergyPlus source-order stage"].Trim().Trim('`').Trim()
    Assert-SourceMapCodeToken `
        -SourceMapText $sourceMapText `
        -Token $sourceOrderStage `
        -Description "EnergyPlus source-order stage"

    $rustModule = ConvertTo-NormalizedRepoPath -Path $Values["Rust target module"].Trim('`')
    $rustModulePath = Resolve-RepoRelativeFile -Path $rustModule -Description "Rust target module"
    $ledgerTargets = @(Get-TomlStringArrayValues -Text $ledgerBlock -Name "rust_target")
    $rustFunction = $Values["Rust target function"].Trim().Trim('`').Trim()
    $exactRustTarget = "${rustModule}::${rustFunction}"
    if ($ledgerTargets -notcontains $exactRustTarget) {
        throw "Rust target function is not exactly mapped by algorithm $algorithmId`: $exactRustTarget"
    }
    $mappedTargetPaths = @(
        $ledgerTargets |
            ForEach-Object {
                ConvertTo-NormalizedRepoPath -Path (([string]$_ -split "::", 2)[0])
            } |
            Sort-Object -Unique
    )
    $rustText = Get-Content -Encoding UTF8 -Raw -LiteralPath $rustModulePath
    if (-not (Test-RustFunctionTarget -RustText $rustText -Target $rustFunction)) {
        throw "Rust target function does not exist in ${rustModule}: $rustFunction"
    }

    $executionStage = $Values["ExecutionStageKind"].Trim().Trim('`').Trim()
    $portTicketMappings = @(Get-TomlStringArrayValues -Text $ledgerBlock -Name "port_ticket_mappings")
    $exactPortTicketMapping = "${sourceFile}|${routineToken}|${sourceOrderStage}|${executionStage}"
    if ($portTicketMappings -notcontains $exactPortTicketMapping) {
        throw "Source file, routine, source-order stage, and ExecutionStageKind are not linked by algorithm ${algorithmId}: $exactPortTicketMapping"
    }
    $executionPlan = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $RepoRoot "crates\ep_runtime\src\execution_plan.rs")
    if (-not [regex]::IsMatch($executionPlan, "(?m)^\s*$([regex]::Escape($executionStage)),\s*$")) {
        throw "ExecutionStageKind variant does not exist: $executionStage"
    }

    $caseId = $Values["First target case"].Trim().Trim('`').Trim()
    if ($caseId -notmatch "^[a-z0-9][a-z0-9_-]*$") {
        throw "First target case must be a case-manifest id: $caseId"
    }
    $allowedCaseIds = @(Get-LedgerAllowedCaseIds -LedgerBlock $ledgerBlock)
    if ($allowedCaseIds -notcontains $caseId) {
        throw "First target case is not linked by algorithm $algorithmId first evidence, family cases, or support boundary: $caseId"
    }
    $casePath = Join-Path $RepoRoot "data\conformance_cases\$caseId\case.toml"
    if (-not (Test-Path -LiteralPath $casePath -PathType Leaf)) {
        throw "First target case manifest does not exist: $caseId"
    }
    $caseText = Get-Content -Encoding UTF8 -Raw -LiteralPath $casePath
    $proofVariables = @(Split-TicketList -Value $Values["Proof variables"])
    $affectedVariables = @(Split-TicketList -Value $Values["Affected variables"])
    $affectedMeters = @(Split-TicketList -Value $Values["Affected meters"])
    $diagnosticVariables = @(Split-TicketList -Value $Values["Diagnostic-only variables"])
    if ($affectedMeters.Count -eq 1 -and $affectedMeters[0].ToLowerInvariant() -eq "none") {
        $affectedMeters = @()
    }
    if ($diagnosticVariables.Count -eq 1 -and $diagnosticVariables[0].ToLowerInvariant() -eq "none") {
        $diagnosticVariables = @()
    }
    if ($proofVariables.Count -eq 0) {
        throw "Proof variables must contain at least one variable."
    }
    $ledgerProofVariables = @(Get-TomlStringArrayValues -Text $ledgerBlock -Name "proof_variables")
    $coverageText = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $RepoRoot "specs\variable_coverage.toml")
    foreach ($variable in $affectedVariables) {
        if (-not [regex]::IsMatch($coverageText, "(?m)^name\s*=\s*`"$([regex]::Escape($variable))`"\s*$")) {
            throw "Affected variable is missing from variable coverage: $variable"
        }
        if (-not (Test-CaseOutputName -CaseText $caseText -Variable $variable)) {
            throw "Affected variable is not requested by first target case ${caseId}: $variable"
        }
    }
    foreach ($variable in $diagnosticVariables) {
        if (-not [regex]::IsMatch($coverageText, "(?m)^name\s*=\s*`"$([regex]::Escape($variable))`"\s*$")) {
            throw "Diagnostic-only variable is missing from variable coverage: $variable"
        }
        if (-not (Test-CaseOutputLevel -CaseText $caseText -Variable $variable -Level "diagnostic")) {
            throw "Diagnostic-only variable is not a diagnostic output of ${caseId}: $variable"
        }
    }
    foreach ($meter in $affectedMeters) {
        if (-not (Test-CaseMeterName -CaseText $caseText -Meter $meter)) {
            throw "Affected meter is not requested by first target case ${caseId}: $meter"
        }
    }
    foreach ($variable in $proofVariables) {
        if ($affectedVariables -notcontains $variable) {
            throw "Proof variable must also appear in Affected variables: $variable"
        }
        if ($ledgerProofVariables -notcontains $variable) {
            throw "Proof variable is not mapped by algorithm $algorithmId`: $variable"
        }
        if (-not [regex]::IsMatch($coverageText, "(?m)^name\s*=\s*`"$([regex]::Escape($variable))`"\s*$")) {
            throw "Proof variable is missing from variable coverage: $variable"
        }
        if (-not (Test-CaseOutputLevel -CaseText $caseText -Variable $variable -Level "conformance") -and
            -not (Test-CaseOutputLevel -CaseText $caseText -Variable $variable -Level "diagnostic") -and
            -not (Test-CaseOutputLevel -CaseText $caseText -Variable $variable -Level "baseline")) {
            throw "Proof variable is not requested by first target case ${caseId}: $variable"
        }
    }

    $reportBlock = Get-TomlSectionBlock -Text $caseText -Name "report"
    $gateBlock = Get-TomlSectionBlock -Text $caseText -Name "gate"
    $reportPath = Get-TomlStringValue -Text $reportBlock -Name "path"
    Assert-FieldEquals -Actual $Values["Report path"] -Expected $reportPath -Name "Report path"
    $gateScript = Get-TomlStringValue -Text $gateBlock -Name "script"
    $expectedGateInvocation = Get-NormalizedDevCommandInvocation -Value $gateScript
    $actualGateInvocation = Get-NormalizedDevCommandInvocation -Value $Values["Blocking gate"]
    Assert-FieldEquals -Actual $actualGateInvocation -Expected $expectedGateInvocation -Name "Blocking gate"
    $expectedGate = Get-DevCommandName -Value $gateScript
    $commandCatalog = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $RepoRoot "scripts\dev\commands.json") |
        ConvertFrom-Json
    $registeredCommands = @($commandCatalog.commands | ForEach-Object { [string]$_.name })
    if ($registeredCommands -notcontains $expectedGate) {
        throw "First target case gate is not a registered dev command: $expectedGate"
    }
    $gateCommand = @($commandCatalog.commands | Where-Object { $_.name -eq $expectedGate })[0]
    $expectedGatePath = ConvertTo-NormalizedRepoPath -Path ("scripts/" + [string]$gateCommand.path)
    Assert-ChangedContractCoverage `
        -AlgorithmId $algorithmId `
        -RustModule $rustModule `
        -CaseId $caseId `
        -ExpectedGate $expectedGate `
        -ExpectedGatePath $expectedGatePath `
        -LedgerBlock $ledgerBlock `
        -MappedTargetPaths $mappedTargetPaths `
        -AllowedCaseIds @($allowedCaseIds) `
        -SourceOrderFiles $SourceOrderFiles `
        -ChangedFileRecords $ChangedFileRecords `
        -BaseRevision $BaseRevision
    if ($PortType -eq "compatibility" -and $gateBlock -notmatch "(?m)^blocking\s*=\s*true\s*$") {
        throw "First target case must define a blocking gate: $caseId"
    }

    $tolerance = $Values["Tolerance candidate"].Trim()
    if ($tolerance -notmatch '(?i)(\d|exact([ -]match)?|bit[ -]for[ -]bit|no tolerance because|not proposed because)') {
        throw "Tolerance candidate must include a numeric value, exact-match policy, or an explicit no-tolerance reason."
    }

    if ($Values["Conformance claim"].Trim().ToLowerInvariant() -eq "yes") {
        if ($PortType -ne "compatibility") {
            throw "Only compatibility tickets may set Conformance claim to yes."
        }
        if ((Get-TomlStringValue -Text $ledgerBlock -Name "status") -ne "conformance") {
            throw "Conformance claim requires a conformance algorithm ledger entry: $algorithmId"
        }
        if ($caseText -notmatch "(?m)^conformance_claim\s*=\s*true\s*$") {
            throw "Conformance claim requires a conformance first target case: $caseId"
        }
        foreach ($variable in $proofVariables) {
            if (-not (Test-CaseOutputLevel -CaseText $caseText -Variable $variable -Level "conformance")) {
                throw "Conformance proof variable must be a conformance output of ${caseId}: $variable"
            }
        }
    }
}

function Test-AlgorithmPortTicketBody {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text,
        [string[]]$ChangedFiles = @(),
        [bool]$ChangedFilesProvided = $false,
        [object[]]$ChangedFileRecords = @(),
        [string]$BaseRevision
    )

    $sourceOrderFiles = @(Get-AlgorithmSourceOrderFiles -ChangedFiles $ChangedFiles)
    if (
        $sourceOrderFiles -contains "scripts/dev/commands.json" -and
        -not [string]::IsNullOrWhiteSpace($BaseRevision) -and
        -not (Test-EvidenceCommandCatalogChange -BaseRevision $BaseRevision)
    ) {
        $sourceOrderFiles = @($sourceOrderFiles | Where-Object { $_ -ne "scripts/dev/commands.json" })
    }
    # A one-time routine inventory bootstrap may span algorithm blocks. Only
    # exact source_mapped metadata additions qualify; promotions and all other
    # source-order changes continue through the single-algorithm ticket path.
    if (
        $ChangedFilesProvided -and
        $sourceOrderFiles.Count -eq 1 -and
        $sourceOrderFiles[0] -eq "specs/algorithm_ledger.toml" -and
        -not [string]::IsNullOrWhiteSpace($BaseRevision)
    ) {
        $baseLedgerText = Get-GitFileText `
            -Revision $BaseRevision `
            -Path "specs/algorithm_ledger.toml"
        $baseContractText = Get-GitFileText `
            -Revision $BaseRevision `
            -Path "specs/project_contract.toml"
        $headLedgerPath = Join-Path $RepoRoot "specs\algorithm_ledger.toml"
        $headContractPath = Join-Path $RepoRoot "specs\project_contract.toml"
        $headLedgerText = Get-Content -Encoding UTF8 -Raw -LiteralPath $headLedgerPath
        $headContractText = Get-Content -Encoding UTF8 -Raw -LiteralPath $headContractPath
        if (
            -not [string]::IsNullOrWhiteSpace($baseLedgerText) -and
            -not [string]::IsNullOrWhiteSpace($baseContractText) -and
            (Test-RoutineCompletionMetadataBootstrapTransition `
                -BaseLedgerText $baseLedgerText `
                -HeadLedgerText $headLedgerText `
                -BaseContractText $baseContractText `
                -HeadContractText $headContractText `
                -ChangedFiles $ChangedFiles)
        ) {
            return [pscustomobject]@{
                status = "pass"
                classification = "routine_completion_metadata_bootstrap"
                sensitive_file_count = 1
            }
        }
    }
    if ($ChangedFilesProvided -and $sourceOrderFiles.Count -eq 0) {
        return [pscustomobject]@{
            status = "pass"
            classification = "not_algorithm_or_source_order_change"
            sensitive_file_count = 0
        }
    }

    if ([string]::IsNullOrWhiteSpace($Text)) {
        throw "PR body is empty; source-order algorithm PRs require an Algorithm Port Ticket or an explicit non-algorithm checkbox."
    }
    $ticketText = Remove-MarkdownHtmlComments -Text $Text
    $ticketText = Remove-MarkdownCodeFences -Text $ticketText

    if (Test-CheckedItem -Text $ticketText -Label "Not an algorithm/source-order change") {
        if (-not $ChangedFilesProvided) {
            throw "Cannot accept 'Not an algorithm/source-order change' without an explicit changed-files context."
        }
        if ($sourceOrderFiles.Count -gt 0) {
            $summary = ($sourceOrderFiles | Select-Object -First 10) -join ", "
            throw "Algorithm/source-order-sensitive files changed, so a completed Algorithm Port Ticket is required: $summary"
        }
        return [pscustomobject]@{
            status = "pass"
            classification = "not_algorithm_or_source_order_change"
            sensitive_file_count = 0
        }
    }

    $ticketSection = Get-AlgorithmPortTicketSection -Text $ticketText

    $commonFields = @(
        "Ticket path or PR section",
        "Algorithm ID",
        "Domain",
        "Port type",
        "EnergyPlus version",
        "EnergyPlus source file",
        "EnergyPlus routine",
        "EnergyPlus source-order stage",
        "Rust target module",
        "Rust target function",
        "ExecutionStageKind",
        "Compatibility path",
        "Diagnostic probe used",
        "Read state",
        "Write state",
        "History/state ownership",
        "Unsupported state",
        "Inactive branches",
        "Unsupported active branches",
        "Affected variables",
        "Affected meters",
        "Diagnostic-only variables",
        "First target case",
        "Proof variables",
        "Tolerance candidate",
        "Report path",
        "Blocking gate",
        "Conformance claim",
        "Not-claimed branches",
        "Partial run allowed"
    )
    $values = @{}
    foreach ($field in $commonFields) {
        $values[$field] = Assert-FieldValue -Text $ticketSection -Name $field
    }
    $concreteFields = @(
        "Ticket path or PR section",
        "Algorithm ID",
        "Domain",
        "EnergyPlus source file",
        "EnergyPlus routine",
        "EnergyPlus source-order stage",
        "Rust target module",
        "Rust target function",
        "ExecutionStageKind",
        "Read state",
        "Write state",
        "History/state ownership",
        "Affected variables",
        "First target case",
        "Proof variables",
        "Tolerance candidate",
        "Report path",
        "Blocking gate",
        "Not-claimed branches"
    )
    foreach ($field in $concreteFields) {
        Assert-ConcreteFieldValue -Value $values[$field] -Name $field
    }
    Assert-TicketLocation -Value $values["Ticket path or PR section"]
    Assert-FieldEquals -Actual $values["EnergyPlus version"] -Expected "26.1.0" -Name "EnergyPlus version"
    Assert-AllowedFieldValue -Value $values["Compatibility path"] -Allowed @("true", "false") -Name "Compatibility path"
    Assert-AllowedFieldValue -Value $values["Diagnostic probe used"] -Allowed @("true", "false") -Name "Diagnostic probe used"
    Assert-AllowedFieldValue -Value $values["Conformance claim"] -Allowed @("yes", "no") -Name "Conformance claim"
    Assert-AllowedFieldValue -Value $values["Partial run allowed"] -Allowed @("yes", "no") -Name "Partial run allowed"

    $compatibilityChecked = Test-CheckedItem -Text $ticketSection -Label "Compatibility port ticket completed"
    $diagnosticChecked = Test-CheckedItem -Text $ticketSection -Label "Diagnostic probe only; no conformance claim"
    if ($compatibilityChecked -and $diagnosticChecked) {
        throw "Compatibility and diagnostic Algorithm Port Ticket classifications are mutually exclusive."
    }

    $portType = $values["Port type"].Trim().ToLowerInvariant()
    switch ($portType) {
        "compatibility" {
            if (-not $compatibilityChecked) {
                throw "Compatibility source-order ports must check 'Compatibility port ticket completed'."
            }
            Assert-FieldEquals -Actual $values["Compatibility path"] -Expected "true" -Name "Compatibility path"
            Assert-FieldEquals -Actual $values["Diagnostic probe used"] -Expected "false" -Name "Diagnostic probe used"
        }
        "diagnostic_probe" {
            if (-not $diagnosticChecked) {
                throw "Diagnostic probe PRs must check 'Diagnostic probe only; no conformance claim'."
            }
            Assert-FieldEquals -Actual $values["Compatibility path"] -Expected "false" -Name "Compatibility path"
            Assert-FieldEquals -Actual $values["Diagnostic probe used"] -Expected "true" -Name "Diagnostic probe used"
            Assert-FieldEquals -Actual $values["Conformance claim"] -Expected "no" -Name "Conformance claim"
        }
        "refactor_only" {
            if ($compatibilityChecked -or $diagnosticChecked) {
                throw "Refactor-only tickets must not check compatibility or diagnostic ticket classifications."
            }
            Assert-FieldEquals -Actual $values["Diagnostic probe used"] -Expected "false" -Name "Diagnostic probe used"
            Assert-FieldEquals -Actual $values["Conformance claim"] -Expected "no" -Name "Conformance claim"
        }
        default {
            throw "Port type must be compatibility, diagnostic_probe, or refactor_only, got '$($values["Port type"])'."
        }
    }
    Assert-TicketReferences `
        -Values $values `
        -PortType $portType `
        -SourceOrderFiles $sourceOrderFiles `
        -ChangedFileRecords $ChangedFileRecords `
        -BaseRevision $BaseRevision

    return [pscustomobject]@{
        status = "pass"
        classification = $portType
        algorithm_id = $values["Algorithm ID"]
        sensitive_file_count = $sourceOrderFiles.Count
    }
}

if ($SelfTest) {
    $selfTestPath = Join-Path $PSScriptRoot "pr-port-ticket-check\self-tests.ps1"
    if (-not (Test-Path -LiteralPath $selfTestPath -PathType Leaf)) {
        throw "PR port-ticket self-test library does not exist: $selfTestPath"
    }
    . $selfTestPath
    Invoke-PrPortTicketSelfTest
    return
}

$bodyText = Get-PrBodyText -Path $BodyPath -InlineBody $Body
$changedFileContext = Get-ChangedFileContext -Path $ChangedFilesPath -Base $BaseSha -Head $HeadSha
if (-not $changedFileContext.provided) {
    throw "PR port-ticket check requires BaseSha/HeadSha or an explicit ChangedFilesPath."
}
$result = Test-AlgorithmPortTicketBody `
    -Text $bodyText `
    -ChangedFiles $changedFileContext.files `
    -ChangedFilesProvided $changedFileContext.provided `
    -ChangedFileRecords $changedFileContext.records `
    -BaseRevision $changedFileContext.merge_base
Write-Host "PR port-ticket check passed: $($result.classification); sensitive_files=$($result.sensitive_file_count)"
