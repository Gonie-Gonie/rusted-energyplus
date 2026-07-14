function Get-GitFileText {
    param(
        [Parameter(Mandatory = $true)][string]$Revision,
        [Parameter(Mandatory = $true)][string]$Path
    )

    if ([string]::IsNullOrWhiteSpace($Revision)) {
        return $null
    }
    # A path may legitimately be new at head. Suppress native stderr so
    # ErrorActionPreference=Stop does not turn the expected miss into a
    # terminating RemoteException; the exit code remains authoritative.
    $previousErrorActionPreference = $ErrorActionPreference
    try {
        $ErrorActionPreference = "Continue"
        $output = @(& git -C $RepoRoot show "${Revision}:$Path" 2>$null)
        $gitExitCode = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $previousErrorActionPreference
    }
    if ($gitExitCode -ne 0) {
        return $null
    }
    return $output -join [Environment]::NewLine
}

function Get-TomlArrayTableBlocksById {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Table
    )

    # Windows PowerShell materializes native-command output with CRLF while
    # Get-Content -Raw preserves the checked-out file's LF endings. Compare a
    # canonical representation so unchanged base/head blocks stay unchanged.
    $normalizedText = $Text -replace "\r\n?", "`n"
    $result = @{}
    $pattern = "(?ms)^\[\[$([regex]::Escape($Table))\]\]\s*(?<body>.*?)(?=^\[|\z)"
    foreach ($match in [regex]::Matches($normalizedText, $pattern)) {
        $body = $match.Groups["body"].Value
        $id = Get-TomlStringValue -Text $body -Name "id"
        if (-not [string]::IsNullOrWhiteSpace($id)) {
            $result[$id] = $body.Trim()
        }
    }
    return $result
}

function Get-ChangedTomlBlockIds {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$BaseText,
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$HeadText,
        [Parameter(Mandatory = $true)][string]$Table
    )

    $baseBlocks = Get-TomlArrayTableBlocksById -Text $BaseText -Table $Table
    $headBlocks = Get-TomlArrayTableBlocksById -Text $HeadText -Table $Table
    $ids = @($baseBlocks.Keys + $headBlocks.Keys | Sort-Object -Unique)
    return @(
        $ids | Where-Object {
            -not $baseBlocks.ContainsKey($_) -or
            -not $headBlocks.ContainsKey($_) -or
            $baseBlocks[$_] -ne $headBlocks[$_]
        }
    )
}

function Get-TomlDocumentSectionMap {
    param([AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text)

    $normalized = $Text -replace "\r\n?", "`n"
    $headers = @([regex]::Matches(
        $normalized,
        '(?m)^(?<header>\[\[[^\]\r\n]+\]\]|\[(?!\[)[^\]\r\n]+\])[ \t]*(?:#.*)?$'
    ))
    $sections = @{}
    $rootEnd = if ($headers.Count -gt 0) { $headers[0].Index } else { $normalized.Length }
    $root = $normalized.Substring(0, $rootEnd).Trim()
    if (-not [string]::IsNullOrWhiteSpace($root)) {
        $sections["root"] = $root
    }

    for ($index = 0; $index -lt $headers.Count; $index += 1) {
        $match = $headers[$index]
        $end = if (($index + 1) -lt $headers.Count) { $headers[$index + 1].Index } else { $normalized.Length }
        $segment = $normalized.Substring($match.Index, $end - $match.Index).Trim()
        $header = $match.Groups["header"].Value
        if ($header.StartsWith("[[", [System.StringComparison]::Ordinal)) {
            $table = $header.Substring(2, $header.Length - 4).Trim()
            $bodyStart = $match.Index + $match.Length
            $body = $normalized.Substring($bodyStart, $end - $bodyStart)
            $id = Get-TomlStringValue -Text $body -Name "id"
            if ([string]::IsNullOrWhiteSpace($id)) {
                throw "TOML array table [[${table}]] must define an id for contract diff coverage."
            }
            $key = "array:${table}:${id}"
        }
        else {
            $table = $header.Substring(1, $header.Length - 2).Trim()
            $key = "table:${table}"
        }
        if ($sections.ContainsKey($key)) {
            throw "TOML contract diff found a duplicate section key: $key"
        }
        $sections[$key] = $segment
    }
    return $sections
}

function Get-ChangedTomlSectionKeys {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$BaseText,
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$HeadText
    )

    $baseSections = Get-TomlDocumentSectionMap -Text $BaseText
    $headSections = Get-TomlDocumentSectionMap -Text $HeadText
    $keys = @($baseSections.Keys + $headSections.Keys | Sort-Object -Unique)
    return @(
        $keys | Where-Object {
            -not $baseSections.ContainsKey($_) -or
            -not $headSections.ContainsKey($_) -or
            $baseSections[$_] -ne $headSections[$_]
        }
    )
}

function Get-RoutineCompletionIds {
    param([AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text)

    return @(
        [regex]::Matches(
            $Text,
            '(?m)^[ \t]*routine\.(?<id>[a-z0-9][a-z0-9_]*)\.[a-z][a-z0-9_]*[ \t]*='
        ) |
            ForEach-Object { $_.Groups["id"].Value } |
            Sort-Object -Unique
    )
}

function Remove-NewRoutineCompletionAssignments {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text,
        [AllowEmptyCollection()][string[]]$RoutineIds = @()
    )

    $routineIdSet = @{}
    foreach ($routineId in $RoutineIds) {
        $routineIdSet[$routineId] = $true
    }
    $keptLines = [System.Collections.Generic.List[string]]::new()
    foreach ($line in [regex]::Split(($Text -replace "\r\n?", "`n"), "`n")) {
        $match = [regex]::Match(
            $line,
            '^[ \t]*routine\.(?<id>[a-z0-9][a-z0-9_]*)\.[a-z][a-z0-9_]*[ \t]*='
        )
        if ($match.Success -and $routineIdSet.ContainsKey($match.Groups["id"].Value)) {
            continue
        }
        $keptLines.Add($line)
    }
    return ($keptLines -join "`n").Trim()
}

function Test-NewRoutineCompletionAssignments {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$AlgorithmText,
        [Parameter(Mandatory = $true)][string]$RoutineId
    )

    if ($RoutineId -notmatch '^[a-z0-9][a-z0-9_]*$') {
        return $false
    }
    $assignmentPattern = (
        '^[ \t]*routine\.{0}\.(?<field>[a-z][a-z0-9_]*)[ \t]*=[ \t]*(?<value>.+?)[ \t]*$' -f
            [regex]::Escape($RoutineId)
    )
    $prefixPattern = '^[ \t]*routine\.{0}\.' -f [regex]::Escape($RoutineId)
    $assignments = @{}
    $routineLines = @(
        [regex]::Split(($AlgorithmText -replace "\r\n?", "`n"), "`n") |
            Where-Object { $_ -match $prefixPattern }
    )
    foreach ($line in $routineLines) {
        $match = [regex]::Match($line, $assignmentPattern)
        if (-not $match.Success) {
            return $false
        }
        $field = $match.Groups["field"].Value
        if ($assignments.ContainsKey($field)) {
            return $false
        }
        $assignments[$field] = $match.Groups["value"].Value
    }

    $requiredFields = @(
        "source_file",
        "source_routine",
        "source_map",
        "completion_status",
        "required_for_full_domain"
    )
    if ($assignments.Count -ne $requiredFields.Count) {
        return $false
    }
    foreach ($field in $requiredFields) {
        if (-not $assignments.ContainsKey($field)) {
            return $false
        }
    }
    foreach ($field in @("source_file", "source_routine", "source_map")) {
        if ($assignments[$field] -notmatch '^"[^"\r\n]+"$') {
            return $false
        }
    }
    $sourceFile = $assignments["source_file"].Trim('"')
    $sourceMap = $assignments["source_map"].Trim('"')
    if (
        $sourceFile -notmatch '^src/EnergyPlus/.+\.(cc|hh)$' -or
        $sourceMap -notmatch '^docs/src/porting-map/.+\.md$' -or
        $sourceFile -match '\\' -or
        $sourceMap -match '\\' -or
        $sourceFile -match '(^|/)\.\.(/|$)' -or
        $sourceMap -match '(^|/)\.\.(/|$)' -or
        $assignments["source_routine"] -notmatch '^"[A-Za-z_][A-Za-z0-9_]*"$'
    ) {
        return $false
    }
    if ($assignments["completion_status"] -cne '"source_mapped"') {
        return $false
    }
    if ($assignments["required_for_full_domain"] -cne "true") {
        return $false
    }
    return $true
}

function Test-RoutineCompletionMetadataBootstrap {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$BaseText,
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$HeadText
    )

    try {
        $baseSections = Get-TomlDocumentSectionMap -Text $BaseText
        $headSections = Get-TomlDocumentSectionMap -Text $HeadText
    }
    catch {
        return $false
    }
    $sectionKeys = @($baseSections.Keys + $headSections.Keys | Sort-Object -Unique)
    if ($sectionKeys.Count -ne $baseSections.Count -or $sectionKeys.Count -ne $headSections.Count) {
        return $false
    }
    $changedSections = @(
        $sectionKeys | Where-Object { $baseSections[$_] -ne $headSections[$_] }
    )
    if (
        $changedSections.Count -eq 0 -or
        @($changedSections | Where-Object { $_ -notlike "array:algorithm:*" }).Count -gt 0
    ) {
        return $false
    }

    $baseRoutineIdsById = @{}
    foreach ($sectionKey in $sectionKeys) {
        if ($sectionKey -notlike "array:algorithm:*") {
            continue
        }
        foreach ($routineId in @(Get-RoutineCompletionIds -Text ([string]$baseSections[$sectionKey]))) {
            if ($baseRoutineIdsById.ContainsKey($routineId)) {
                return $false
            }
            $baseRoutineIdsById[$routineId] = $true
        }
    }
    $seenNewRoutineIds = @{}
    foreach ($sectionKey in $changedSections) {
        $baseBlock = [string]$baseSections[$sectionKey]
        $headBlock = [string]$headSections[$sectionKey]
        $baseRoutineIds = @(Get-RoutineCompletionIds -Text $baseBlock)
        $headRoutineIds = @(Get-RoutineCompletionIds -Text $headBlock)
        $newRoutineIds = @($headRoutineIds | Where-Object { $baseRoutineIds -notcontains $_ })
        if ($newRoutineIds.Count -eq 0) {
            return $false
        }
        foreach ($routineId in $newRoutineIds) {
            if (
                $baseRoutineIdsById.ContainsKey($routineId) -or
                $seenNewRoutineIds.ContainsKey($routineId) -or
                -not (Test-NewRoutineCompletionAssignments -AlgorithmText $headBlock -RoutineId $routineId)
            ) {
                return $false
            }
            $seenNewRoutineIds[$routineId] = $true
        }
        $strippedHead = Remove-NewRoutineCompletionAssignments `
            -Text $headBlock `
            -RoutineIds $newRoutineIds
        $normalizedBase = ($baseBlock -replace "\r\n?", "`n").Trim()
        if ($strippedHead -cne $normalizedBase) {
            return $false
        }
    }
    return $seenNewRoutineIds.Count -gt 0
}

function Test-RoutineCompletionSchemaMarkerTransition {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$BaseText,
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$HeadText
    )

    if (
        [string]::IsNullOrWhiteSpace($BaseText) -or
        [string]::IsNullOrWhiteSpace($HeadText)
    ) {
        return $false
    }
    try {
        $baseSections = Get-TomlDocumentSectionMap -Text $BaseText
        $headSections = Get-TomlDocumentSectionMap -Text $HeadText
    }
    catch {
        return $false
    }
    $baseRoot = if ($baseSections.ContainsKey("root")) { [string]$baseSections["root"] } else { "" }
    $headRoot = if ($headSections.ContainsKey("root")) { [string]$headSections["root"] } else { "" }
    $markerKeyPattern = '(?m)^[ \t]*routine_completion_schema[ \t]*='
    $exactMarkerPattern = (
        '(?m)^[ \t]*routine_completion_schema[ \t]*=[ \t]*' +
        '"routine_completion\.v1"[ \t]*(?:#.*)?$'
    )
    return (
        [regex]::Matches($baseRoot, $markerKeyPattern).Count -eq 0 -and
        [regex]::Matches($headRoot, $markerKeyPattern).Count -eq 1 -and
        [regex]::Matches($headRoot, $exactMarkerPattern).Count -eq 1
    )
}

function Test-RoutineCompletionMetadataBootstrapTransition {
    param(
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$BaseLedgerText,
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$HeadLedgerText,
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$BaseContractText,
        [AllowEmptyString()][Parameter(Mandatory = $true)][string]$HeadContractText,
        [AllowEmptyCollection()][string[]]$ChangedFiles = @()
    )

    if (-not (Test-RoutineCompletionMetadataBootstrap `
        -BaseText $BaseLedgerText `
        -HeadText $HeadLedgerText)) {
        return $false
    }
    if (-not (Test-RoutineCompletionSchemaMarkerTransition `
        -BaseText $BaseContractText `
        -HeadText $HeadContractText)) {
        return $false
    }

    $normalizedChangedFiles = @(
        $ChangedFiles |
            ForEach-Object { ConvertTo-NormalizedRepoPath -Path ([string]$_) } |
            Sort-Object -Unique
    )
    if (
        $normalizedChangedFiles.Count -eq 0 -or
        $normalizedChangedFiles -notcontains "specs/algorithm_ledger.toml" -or
        $normalizedChangedFiles -notcontains "specs/project_contract.toml"
    ) {
        return $false
    }
    $disallowedPaths = @(
        $normalizedChangedFiles |
            Where-Object {
                $script:RoutineCompletionMetadataBootstrapAllowedPaths -notcontains $_
            }
    )
    return $disallowedPaths.Count -eq 0
}

function Get-ChangedCommandNames {
    param([Parameter(Mandatory = $true)][string]$BaseRevision)

    $baseText = Get-GitFileText -Revision $BaseRevision -Path "scripts/dev/commands.json"
    if ([string]::IsNullOrWhiteSpace($baseText)) {
        throw "Cannot cross-check command catalog changes without the base command catalog."
    }
    $baseCatalog = $baseText | ConvertFrom-Json
    $headCatalog = Get-Content -Encoding UTF8 -Raw -LiteralPath (
        Join-Path $RepoRoot "scripts\dev\commands.json"
    ) | ConvertFrom-Json
    $baseEntries = @{}
    $headEntries = @{}
    foreach ($entry in $baseCatalog.commands) {
        $baseEntries[[string]$entry.name] = "$(($entry | ConvertTo-Json -Compress))"
    }
    foreach ($entry in $headCatalog.commands) {
        $headEntries[[string]$entry.name] = "$(($entry | ConvertTo-Json -Compress))"
    }
    $names = @($baseEntries.Keys + $headEntries.Keys | Sort-Object -Unique)
    return @(
        $names | Where-Object {
            -not $baseEntries.ContainsKey($_) -or
            -not $headEntries.ContainsKey($_) -or
            $baseEntries[$_] -ne $headEntries[$_]
        }
    )
}

function Get-EvidenceGateCommandNames {
    param([string]$BaseRevision)

    $names = [System.Collections.Generic.HashSet[string]]::new(
        [System.StringComparer]::OrdinalIgnoreCase
    )
    $caseRoot = Join-Path $RepoRoot "data\conformance_cases"
    foreach ($caseFile in Get-ChildItem -LiteralPath $caseRoot -Recurse -File -Filter "case.toml") {
        $text = Get-Content -Encoding UTF8 -Raw -LiteralPath $caseFile.FullName
        foreach ($match in [regex]::Matches($text, '(?m)^script\s*=\s*"scripts/dev\.cmd\s+(?<command>[^"\s]+)')) {
            [void]$names.Add($match.Groups["command"].Value)
        }
    }
    if (-not [string]::IsNullOrWhiteSpace($BaseRevision)) {
        $baseLines = @(& git -C $RepoRoot grep -h -E '^script[[:space:]]*=' $BaseRevision -- 'data/conformance_cases/*/case.toml' 2>$null)
        if ($LASTEXITCODE -notin @(0, 1)) {
            throw "Unable to inspect base case-manifest gate commands."
        }
        foreach ($line in $baseLines) {
            $match = [regex]::Match([string]$line, '^script\s*=\s*"scripts/dev\.cmd\s+(?<command>[^"\s]+)')
            if ($match.Success) {
                [void]$names.Add($match.Groups["command"].Value)
            }
        }
    }
    return @($names)
}

function Get-GateCommandNameForCase {
    param(
        [Parameter(Mandatory = $true)][string]$CaseId,
        [string]$Revision
    )

    $caseRelativePath = "data/conformance_cases/$CaseId/case.toml"
    $caseText = if ([string]::IsNullOrWhiteSpace($Revision)) {
        $path = Join-Path $RepoRoot ($caseRelativePath -replace '/', '\')
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { return $null }
        Get-Content -Encoding UTF8 -Raw -LiteralPath $path
    }
    else {
        Get-GitFileText -Revision $Revision -Path $caseRelativePath
    }
    if ([string]::IsNullOrWhiteSpace($caseText)) { return $null }
    $gateBlock = Get-TomlSectionBlock -Text $caseText -Name "gate"
    return Get-DevCommandName -Value (Get-TomlStringValue -Text $gateBlock -Name "script")
}

function Get-AllowedGateCommandNames {
    param(
        [AllowEmptyCollection()][string[]]$CaseIds = @(),
        [string]$Revision
    )

    return @(
        $CaseIds |
            ForEach-Object { Get-GateCommandNameForCase -CaseId $_ -Revision $Revision } |
            Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
            Sort-Object -Unique
    )
}

function Get-AllowedGateCommandBoundaryNames {
    param(
        [AllowEmptyCollection()][string[]]$HeadCaseIds = @(),
        [AllowEmptyCollection()][string[]]$BaseCaseIds = @(),
        [string]$BaseRevision
    )

    return @(
        Get-AllowedGateCommandNames -CaseIds $HeadCaseIds
        Get-AllowedGateCommandNames -CaseIds $BaseCaseIds -Revision $BaseRevision
    ) | Sort-Object -Unique
}

function Get-UnrelatedEvidenceCommandNames {
    param(
        [AllowEmptyCollection()][string[]]$ChangedCommands = @(),
        [AllowEmptyCollection()][string[]]$EvidenceCommands = @(),
        [AllowEmptyCollection()][string[]]$AllowedCommands = @()
    )

    return @(
        $ChangedCommands |
            Where-Object { $EvidenceCommands -contains $_ -and $AllowedCommands -notcontains $_ } |
            Sort-Object -Unique
    )
}

function Test-EvidenceCommandCatalogChange {
    param([Parameter(Mandatory = $true)][string]$BaseRevision)

    $changed = @(Get-ChangedCommandNames -BaseRevision $BaseRevision)
    $evidence = @(Get-EvidenceGateCommandNames -BaseRevision $BaseRevision)
    return [bool]($changed | Where-Object { $evidence -contains $_ })
}

function Get-LedgerAllowedCaseIds {
    param([Parameter(Mandatory = $true)][string]$LedgerBlock)

    $ids = [System.Collections.Generic.HashSet[string]]::new(
        [System.StringComparer]::OrdinalIgnoreCase
    )
    foreach ($field in @("first_case", "first_evidence")) {
        $value = Get-TomlStringValue -Text $LedgerBlock -Name $field
        if (-not [string]::IsNullOrWhiteSpace($value)) {
            [void]$ids.Add($value)
        }
    }
    foreach ($value in @(Get-TomlStringArrayValues -Text $LedgerBlock -Name "family_cases")) {
        if (-not [string]::IsNullOrWhiteSpace($value)) {
            [void]$ids.Add($value)
        }
    }
    $boundary = Get-TomlStringValue -Text $LedgerBlock -Name "support_boundary"
    foreach ($match in [regex]::Matches([string]$boundary, '(?<![A-Za-z0-9_])(?<id>[a-z0-9][a-z0-9_-]*_[0-9]{3})(?![A-Za-z0-9_])')) {
        [void]$ids.Add($match.Groups["id"].Value)
    }
    return @($ids)
}

function Get-GateScriptPathForCase {
    param(
        [Parameter(Mandatory = $true)][string]$CaseId,
        [string]$Revision
    )

    $caseRelativePath = "data/conformance_cases/$CaseId/case.toml"
    $caseText = if ([string]::IsNullOrWhiteSpace($Revision)) {
        $path = Join-Path $RepoRoot ($caseRelativePath -replace '/', '\')
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { return $null }
        Get-Content -Encoding UTF8 -Raw -LiteralPath $path
    }
    else {
        Get-GitFileText -Revision $Revision -Path $caseRelativePath
    }
    if ([string]::IsNullOrWhiteSpace($caseText)) { return $null }
    $gateBlock = Get-TomlSectionBlock -Text $caseText -Name "gate"
    $commandName = Get-DevCommandName -Value (Get-TomlStringValue -Text $gateBlock -Name "script")

    $catalogText = if ([string]::IsNullOrWhiteSpace($Revision)) {
        Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $RepoRoot "scripts\dev\commands.json")
    }
    else {
        Get-GitFileText -Revision $Revision -Path "scripts/dev/commands.json"
    }
    if ([string]::IsNullOrWhiteSpace($catalogText)) { return $null }
    $catalog = $catalogText | ConvertFrom-Json
    $entry = @($catalog.commands | Where-Object { $_.name -eq $commandName }) | Select-Object -First 1
    if ($null -eq $entry) { return $null }
    return ConvertTo-NormalizedRepoPath -Path ("scripts/" + [string]$entry.path)
}

function Get-AllowedGateScriptPaths {
    param(
        [AllowEmptyCollection()][string[]]$CaseIds = @(),
        [string]$Revision
    )

    return @(
        $CaseIds |
            ForEach-Object { Get-GateScriptPathForCase -CaseId $_ -Revision $Revision } |
            Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
            Sort-Object -Unique
    )
}

function Assert-ChangedContractCoverage {
    param(
        [Parameter(Mandatory = $true)][string]$AlgorithmId,
        [Parameter(Mandatory = $true)][string]$RustModule,
        [Parameter(Mandatory = $true)][string]$CaseId,
        [Parameter(Mandatory = $true)][string]$ExpectedGate,
        [Parameter(Mandatory = $true)][string]$ExpectedGatePath,
        [Parameter(Mandatory = $true)][string]$LedgerBlock,
        [Parameter(Mandatory = $true)][string[]]$MappedTargetPaths,
        [Parameter(Mandatory = $true)][string[]]$AllowedCaseIds,
        [Parameter(Mandatory = $true)][string[]]$SourceOrderFiles,
        [object[]]$ChangedFileRecords = @(),
        [string]$BaseRevision
    )

    $records = @($ChangedFileRecords)
    if ($records.Count -eq 0) {
        $records = @($SourceOrderFiles | ForEach-Object {
            [pscustomobject]@{ status = "M"; side = "head"; path = $_ }
        })
    }
    $records = @($records | Where-Object { $SourceOrderFiles -contains $_.path })

    $headRust = @($records | Where-Object { $_.side -eq "head" -and $_.path -match '^crates/.+\.rs$' } | ForEach-Object path | Sort-Object -Unique)
    $baseRust = @($records | Where-Object { $_.side -eq "base" -and $_.path -match '^crates/.+\.rs$' } | ForEach-Object path | Sort-Object -Unique)
    $unmappedHeadRust = @($headRust | Where-Object { $MappedTargetPaths -notcontains $_ })
    if ($unmappedHeadRust.Count -gt 0) {
        throw "Algorithm $AlgorithmId does not map every head-side sensitive Rust path: $($unmappedHeadRust -join ', ')"
    }
    if ($headRust.Count -gt 0 -and $headRust -notcontains $RustModule) {
        throw "Rust target module named by the ticket is not in the head-side sensitive changed-file set: $RustModule"
    }

    $baseLedgerBlock = $null
    $baseTargetPaths = @()
    $baseAllowedCaseIds = @()
    if (-not [string]::IsNullOrWhiteSpace($BaseRevision)) {
        $baseLedgerText = Get-GitFileText -Revision $BaseRevision -Path "specs/algorithm_ledger.toml"
        try {
            $baseLedgerBlock = Get-AlgorithmLedgerBlock -AlgorithmId $AlgorithmId -LedgerText $baseLedgerText
        }
        catch {
            if ($_.Exception.Message -notlike "*unknown algorithm ledger id*") {
                throw
            }
        }
        if ($null -ne $baseLedgerBlock) {
            $baseTargets = @(Get-TomlStringArrayValues -Text $baseLedgerBlock -Name "rust_target")
            $baseTargetPaths = @($baseTargets | ForEach-Object { ConvertTo-NormalizedRepoPath -Path (([string]$_ -split '::', 2)[0]) } | Sort-Object -Unique)
            $baseAllowedCaseIds = @(Get-LedgerAllowedCaseIds -LedgerBlock $baseLedgerBlock)
        }
    }
    elseif ($baseRust.Count -gt 0 -or ($records | Where-Object { $_.side -eq "base" })) {
        throw "Base-side rename/delete coverage requires a merge-base revision."
    }
    $unmappedBaseRust = @($baseRust | Where-Object { $baseTargetPaths -notcontains $_ })
    if ($unmappedBaseRust.Count -gt 0) {
        throw "Algorithm $AlgorithmId did not map every deleted/pre-rename Rust path at merge base: $($unmappedBaseRust -join ', ')"
    }

    $headAllowedScripts = @($MappedTargetPaths + $ExpectedGatePath + (Get-AllowedGateScriptPaths -CaseIds $AllowedCaseIds) | Sort-Object -Unique)
    $baseAllowedScripts = @($baseTargetPaths + (Get-AllowedGateScriptPaths -CaseIds $baseAllowedCaseIds -Revision $BaseRevision) | Sort-Object -Unique)
    $headScripts = @($records | Where-Object { $_.side -eq "head" -and $_.path -match '^scripts/.+\.ps1$' } | ForEach-Object path | Sort-Object -Unique)
    $baseScripts = @($records | Where-Object { $_.side -eq "base" -and $_.path -match '^scripts/.+\.ps1$' } | ForEach-Object path | Sort-Object -Unique)
    $unmappedHeadScripts = @($headScripts | Where-Object { $headAllowedScripts -notcontains $_ })
    $unmappedBaseScripts = @($baseScripts | Where-Object { $baseAllowedScripts -notcontains $_ })
    if ($unmappedHeadScripts.Count -gt 0) {
        throw "Algorithm $AlgorithmId does not map every head-side sensitive gate script: $($unmappedHeadScripts -join ', ')"
    }
    if ($unmappedBaseScripts.Count -gt 0) {
        throw "Algorithm $AlgorithmId did not map every deleted/pre-rename gate script at merge base: $($unmappedBaseScripts -join ', ')"
    }

    if ($SourceOrderFiles -contains "specs/algorithm_ledger.toml") {
        if ([string]::IsNullOrWhiteSpace($BaseRevision)) { throw "Ledger block coverage requires a merge-base revision." }
        $baseText = Get-GitFileText -Revision $BaseRevision -Path "specs/algorithm_ledger.toml"
        $headText = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $RepoRoot "specs\algorithm_ledger.toml")
        $changedSections = @(Get-ChangedTomlSectionKeys -BaseText $baseText -HeadText $headText)
        $expectedSection = "array:algorithm:${AlgorithmId}"
        if ($changedSections.Count -ne 1 -or $changedSections[0] -ne $expectedSection) {
            throw "Changed algorithm ledger sections must be exactly ${expectedSection}: $($changedSections -join ', ')"
        }
    }

    if ($SourceOrderFiles -contains "specs/capabilities.toml") {
        if ([string]::IsNullOrWhiteSpace($BaseRevision)) { throw "Capability block coverage requires a merge-base revision." }
        $baseText = Get-GitFileText -Revision $BaseRevision -Path "specs/capabilities.toml"
        $headText = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $RepoRoot "specs\capabilities.toml")
        $baseBlocks = Get-TomlArrayTableBlocksById -Text $baseText -Table "capability"
        $headBlocks = Get-TomlArrayTableBlocksById -Text $headText -Table "capability"
        $changedSections = @(Get-ChangedTomlSectionKeys -BaseText $baseText -HeadText $headText)
        $nonCapabilitySections = @($changedSections | Where-Object { $_ -notlike "array:capability:*" })
        if ($nonCapabilitySections.Count -gt 0) {
            throw "Capability registry changes outside [[capability]] blocks are not covered by an Algorithm Port Ticket: $($nonCapabilitySections -join ', ')"
        }
        $changedIds = @(
            $changedSections |
                ForEach-Object { $_.Substring("array:capability:".Length) }
        )
        if ($changedIds.Count -eq 0) { throw "Capability spec changed without a changed capability block." }
        foreach ($id in $changedIds) {
            $algorithms = @()
            if ($baseBlocks.ContainsKey($id)) { $algorithms += @(Get-TomlStringArrayValues -Text $baseBlocks[$id] -Name "algorithms") }
            if ($headBlocks.ContainsKey($id)) { $algorithms += @(Get-TomlStringArrayValues -Text $headBlocks[$id] -Name "algorithms") }
            if ($algorithms -notcontains $AlgorithmId) {
                throw "Changed capability $id is not linked to ticket Algorithm ID $AlgorithmId."
            }
        }
    }

    if ($SourceOrderFiles -contains "scripts/dev/commands.json") {
        if ([string]::IsNullOrWhiteSpace($BaseRevision)) { throw "Command catalog coverage requires a merge-base revision." }
        $changedCommands = @(Get-ChangedCommandNames -BaseRevision $BaseRevision)
        $evidenceCommands = @(Get-EvidenceGateCommandNames -BaseRevision $BaseRevision)
        $allowedCommands = @(
            Get-AllowedGateCommandBoundaryNames `
                -HeadCaseIds $AllowedCaseIds `
                -BaseCaseIds $baseAllowedCaseIds `
                -BaseRevision $BaseRevision
        )
        $unrelatedEvidenceCommands = @(
            Get-UnrelatedEvidenceCommandNames `
                -ChangedCommands $changedCommands `
                -EvidenceCommands $evidenceCommands `
                -AllowedCommands $allowedCommands
        )
        if ($unrelatedEvidenceCommands.Count -gt 0) {
            throw "Changed evidence commands are outside ticket Algorithm ID ${AlgorithmId}: $($unrelatedEvidenceCommands -join ', ')"
        }
        $changedEvidenceCommands = @($changedCommands | Where-Object { $evidenceCommands -contains $_ })
        if ($changedEvidenceCommands.Count -eq 0) {
            throw "Command catalog changed without an evidence command in ticket Algorithm ID ${AlgorithmId}."
        }
    }

    $caseRecords = @($records | Where-Object { $_.path -match '^data/conformance_cases/(?<id>[^/]+)/case\.toml$' })
    if ($caseRecords.Count -gt 0) {
        $allowedAll = @($AllowedCaseIds + $baseAllowedCaseIds | Sort-Object -Unique)
        $changedCaseIds = @($caseRecords | ForEach-Object { [regex]::Match($_.path, '^data/conformance_cases/(?<id>[^/]+)/').Groups["id"].Value } | Sort-Object -Unique)
        $unrelatedCases = @($changedCaseIds | Where-Object { $allowedAll -notcontains $_ })
        if ($unrelatedCases.Count -gt 0) {
            throw "Changed case manifests are not linked to ticket Algorithm ID ${AlgorithmId}: $($unrelatedCases -join ', ')"
        }
        $headCaseIds = @($caseRecords | Where-Object side -eq "head" | ForEach-Object { [regex]::Match($_.path, '^data/conformance_cases/(?<id>[^/]+)/').Groups["id"].Value })
        if ($headCaseIds.Count -gt 0 -and $headCaseIds -notcontains $CaseId) {
            throw "Ticket First target case is not among changed head-side case manifests: $CaseId"
        }
    }
}
