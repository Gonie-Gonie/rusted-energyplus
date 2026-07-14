$script:EvidenceGateScriptPaths = $null
$script:LedgerMappedScriptPaths = $null

function Get-ChangedFileContext {
    param(
        [string]$Path,
        [string]$Base,
        [string]$Head
    )

    if (-not [string]::IsNullOrWhiteSpace($Path)) {
        if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
            throw "Changed-files list does not exist: $Path"
        }
        $files = @(Get-Content -Encoding UTF8 -LiteralPath $Path)
        $records = @(
            $files |
                ForEach-Object {
                    [pscustomobject]@{
                        status = "M"
                        side = "head"
                        path = ConvertTo-NormalizedRepoPath -Path ([string]$_)
                    }
                } |
                Where-Object { -not [string]::IsNullOrWhiteSpace($_.path) }
        )
        return [pscustomobject]@{
            provided = $true
            merge_base = ""
            head_revision = ""
            records = $records
            files = @(
                $records.path |
                    Sort-Object -Unique
            )
        }
    }

    $hasBase = -not [string]::IsNullOrWhiteSpace($Base)
    $hasHead = -not [string]::IsNullOrWhiteSpace($Head)
    if ($hasBase -xor $hasHead) {
        throw "BaseSha and HeadSha must be provided together."
    }
    if (-not $hasBase) {
        return [pscustomobject]@{
            provided = $false
            merge_base = ""
            head_revision = ""
            records = @()
            files = @()
        }
    }
    foreach ($entry in @($Base, $Head)) {
        if ($entry -notmatch "^[0-9a-fA-F]{40}$") {
            throw "Git revision must be a full 40-character SHA: $entry"
        }
    }

    $mergeBaseOutput = @(& git -C $RepoRoot merge-base $Base $Head 2>&1)
    if ($LASTEXITCODE -ne 0) {
        throw "Unable to find the PR merge base for $Base and $Head`: $($mergeBaseOutput -join [Environment]::NewLine)"
    }
    $mergeBase = ([string]($mergeBaseOutput | Select-Object -First 1)).Trim()
    if ($mergeBase -notmatch "^[0-9a-fA-F]{40}$") {
        throw "Git merge-base returned an invalid revision: $mergeBase"
    }

    $gitOutput = & git -C $RepoRoot diff --name-status -z --find-renames=50% $mergeBase $Head -- 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "Unable to enumerate PR changed files for $mergeBase..$Head`: $gitOutput"
    }
    $records = @(ConvertFrom-GitNameStatusRecordsZ -Text ([string]$gitOutput))
    return [pscustomobject]@{
        provided = $true
        merge_base = $mergeBase
        head_revision = $Head
        records = $records
        files = @(
            $records.path |
                ForEach-Object { ConvertTo-NormalizedRepoPath -Path ([string]$_) } |
                Sort-Object -Unique
        )
    }
}

function ConvertFrom-GitNameStatusZ {
    param([AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text)

    return @((ConvertFrom-GitNameStatusRecordsZ -Text $Text).path)
}

function ConvertFrom-GitNameStatusRecordsZ {
    param([AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text)

    if ([string]::IsNullOrEmpty($Text)) {
        return @()
    }
    $tokens = @(
        $Text.Split(
            [char[]]@([char]0),
            [System.StringSplitOptions]::RemoveEmptyEntries
        )
    )
    $records = [System.Collections.Generic.List[object]]::new()
    for ($index = 0; $index -lt $tokens.Count;) {
        $status = $tokens[$index]
        $index += 1
        if ($status -match "^[ACDMRTUXB]$") {
            if ($index -ge $tokens.Count) {
                throw "Malformed git name-status record for status $status."
            }
            $side = if ($status -eq "D") { "base" } else { "head" }
            $records.Add([pscustomobject]@{
                status = $status
                side = $side
                path = ConvertTo-NormalizedRepoPath -Path $tokens[$index]
            })
            $index += 1
            continue
        }
        if ($status -match "^[RC][0-9]{1,3}$") {
            if (($index + 1) -ge $tokens.Count) {
                throw "Malformed git rename/copy record for status $status."
            }
            $records.Add([pscustomobject]@{
                status = $status
                side = "base"
                path = ConvertTo-NormalizedRepoPath -Path $tokens[$index]
            })
            $records.Add([pscustomobject]@{
                status = $status
                side = "head"
                path = ConvertTo-NormalizedRepoPath -Path $tokens[$index + 1]
            })
            $index += 2
            continue
        }
        throw "Unsupported git name-status record: $status"
    }
    return @($records)
}

function Get-EvidenceGateScriptPaths {
    if ($null -ne $script:EvidenceGateScriptPaths) {
        return @($script:EvidenceGateScriptPaths)
    }

    $commandCatalogPath = Join-Path $RepoRoot "scripts\dev\commands.json"
    $commandCatalog = Get-Content -Encoding UTF8 -Raw -LiteralPath $commandCatalogPath |
        ConvertFrom-Json
    $commandPaths = @{}
    foreach ($command in $commandCatalog.commands) {
        $commandPaths[[string]$command.name] = ConvertTo-NormalizedRepoPath `
            -Path ("scripts/" + [string]$command.path)
    }

    $paths = [System.Collections.Generic.HashSet[string]]::new(
        [System.StringComparer]::OrdinalIgnoreCase
    )
    $caseRoot = Join-Path $RepoRoot "data\conformance_cases"
    foreach ($caseFile in Get-ChildItem -LiteralPath $caseRoot -Recurse -File -Filter "case.toml") {
        $caseText = Get-Content -Encoding UTF8 -Raw -LiteralPath $caseFile.FullName
        foreach ($match in [regex]::Matches(
            $caseText,
            '(?m)^script\s*=\s*"scripts/dev\.cmd\s+(?<command>[^"\s]+)'
        )) {
            $commandName = $match.Groups["command"].Value
            if (-not $commandPaths.ContainsKey($commandName)) {
                throw "Case manifest references an unregistered blocking gate: $commandName"
            }
            [void]$paths.Add($commandPaths[$commandName])
        }
    }
    $script:EvidenceGateScriptPaths = @($paths | Sort-Object)
    return @($script:EvidenceGateScriptPaths)
}

function Get-LedgerMappedScriptPaths {
    if ($null -ne $script:LedgerMappedScriptPaths) {
        return @($script:LedgerMappedScriptPaths)
    }

    $ledgerPath = Join-Path $RepoRoot "specs\algorithm_ledger.toml"
    $ledgerText = Get-Content -Encoding UTF8 -Raw -LiteralPath $ledgerPath
    $script:LedgerMappedScriptPaths = @(
        [regex]::Matches($ledgerText, '"(?<path>scripts/[^"\r\n]+\.ps1)(?:::[^"\r\n]+)?"') |
            ForEach-Object {
                ConvertTo-NormalizedRepoPath -Path $_.Groups["path"].Value
            } |
            Sort-Object -Unique
    )
    return @($script:LedgerMappedScriptPaths)
}

function Test-AlgorithmSourceOrderPath {
    param([Parameter(Mandatory = $true)][string]$Path)

    $normalized = ConvertTo-NormalizedRepoPath -Path $Path
    $lower = $normalized.ToLowerInvariant()
    $durableClaimPaths = @(
        "specs/algorithm_ledger.toml",
        "specs/capabilities.toml",
        "scripts/dev/commands.json"
    )
    if ($durableClaimPaths -contains $lower) {
        return $true
    }
    if ($lower -match "^data/conformance_cases/[^/]+/case\.toml$") {
        return $true
    }
    if (
        @(Get-EvidenceGateScriptPaths) -contains $normalized -or
        @(Get-LedgerMappedScriptPaths) -contains $normalized
    ) {
        return $true
    }
    if (
        $lower -like "scripts/compare/*probe*.ps1" -or
        $lower -like "scripts/compare/*compat*.ps1" -or
        $lower -like "scripts/compare/*conformance*.ps1" -or
        $lower -like "scripts/compare/*diagnostic*.ps1" -or
        $lower -like "scripts/compare/official-dynamic-heat-balance*.ps1"
    ) {
        return $true
    }

    $isRustTest =
        $lower -match "/tests?/" -or
        $lower -match "/test\.rs$" -or
        $lower -match "/tests\.rs$" -or
        $lower -match "/test_[^/]*\.rs$" -or
        $lower -match "_tests?\.rs$"
    if ($isRustTest) {
        return $false
    }
    if (
        $lower -like "crates/ep_runtime/src/*.rs" -or
        $lower -like "crates/ep_compiler/src/*.rs" -or
        $lower -like "crates/ep_run/src/*.rs"
    ) {
        return $true
    }
    if (
        $lower -eq "crates/ep_cli/src/main.rs" -or
        $lower -like "crates/ep_cli/src/ideal_loads*.rs" -or
        $lower -like "crates/ep_cli/src/ideal_loads/*.rs"
    ) {
        return $true
    }
    return $false
}

function Get-AlgorithmSourceOrderFiles {
    param([string[]]$ChangedFiles)

    return @(
        $ChangedFiles |
            Where-Object { Test-AlgorithmSourceOrderPath -Path $_ } |
            Sort-Object -Unique
    )
}
