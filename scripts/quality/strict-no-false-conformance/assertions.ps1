# Assertion helpers for strict-no-false-conformance.ps1.

function Assert-DoesNotContain {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for false-conformance guard: $Path"
    }

    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -ne $match) {
        $match | ForEach-Object { Write-Host "$($_.Path):$($_.LineNumber): $($_.Line)" }
        throw "Forbidden false-conformance wording found for $Description`: $Pattern"
    }
    Write-Host "OK no false-conformance wording for $Description"
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for false-conformance guard: $Path"
    }

    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -eq $match) {
        throw "Missing required compatibility boundary for $Description`: $Pattern"
    }
    Write-Host "OK compatibility boundary for $Description`: $Pattern"
}

function Assert-PathMissing {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (Test-Path -LiteralPath $Path) {
        throw "Forbidden retained documentation path for $Description`: $Path"
    }
    Write-Host "OK retained path absent for $Description`: $Path"
}

function Assert-CaseOutputLevel {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Variable,
        [Parameter(Mandatory = $true)][string]$Level,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for case output guard: $Path"
    }

    $text = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    $outputBlockPattern = '(?ms)^\[\[outputs\]\]\s*(.*?)(?=^\[\[|\n\[|\z)'
    $blocks = [regex]::Matches($text, $outputBlockPattern)
    $keyPattern = '(?m)^key\s*=\s*"' + [regex]::Escape($Key) + '"\s*$'
    $variablePattern = '(?m)^variable\s*=\s*"' + [regex]::Escape($Variable) + '"\s*$'
    $levelPattern = '(?m)^level\s*=\s*"' + [regex]::Escape($Level) + '"\s*$'

    foreach ($block in $blocks) {
        $body = $block.Groups[1].Value
        if ($body -match $keyPattern -and $body -match $variablePattern -and $body -match $levelPattern) {
            Write-Host "OK output level for $Description`: $Key / $Variable = $Level"
            return
        }
    }

    throw "Missing required output level for $Description`: key=$Key variable=$Variable level=$Level"
}

function Assert-CaseMeterLevel {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$Level,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for case meter guard: $Path"
    }

    $text = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    $meterBlockPattern = '(?ms)^\[\[meters\]\]\s*(.*?)(?=^\[\[|\n\[|\z)'
    $blocks = [regex]::Matches($text, $meterBlockPattern)
    $namePattern = '(?m)^name\s*=\s*"' + [regex]::Escape($Name) + '"\s*$'
    $levelPattern = '(?m)^level\s*=\s*"' + [regex]::Escape($Level) + '"\s*$'

    foreach ($block in $blocks) {
        $body = $block.Groups[1].Value
        if ($body -match $namePattern -and $body -match $levelPattern) {
            Write-Host "OK meter level for $Description`: $Name = $Level"
            return
        }
    }

    throw "Missing required meter level for $Description`: name=$Name level=$Level"
}

function Assert-CaseMeterLevelFrequency {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$Frequency,
        [Parameter(Mandatory = $true)][string]$Level,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for case meter guard: $Path"
    }

    $text = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    $meterBlockPattern = '(?ms)^\[\[meters\]\]\s*(.*?)(?=^\[\[|\n\[|\z)'
    $blocks = [regex]::Matches($text, $meterBlockPattern)
    $namePattern = '(?m)^name\s*=\s*"' + [regex]::Escape($Name) + '"\s*$'
    $frequencyPattern = '(?m)^frequency\s*=\s*"' + [regex]::Escape($Frequency) + '"\s*$'
    $levelPattern = '(?m)^level\s*=\s*"' + [regex]::Escape($Level) + '"\s*$'

    foreach ($block in $blocks) {
        $body = $block.Groups[1].Value
        if ($body -match $namePattern -and $body -match $frequencyPattern -and $body -match $levelPattern) {
            Write-Host "OK meter level for $Description`: $Name / $Frequency = $Level"
            return
        }
    }

    throw "Missing required meter level for $Description`: name=$Name frequency=$Frequency level=$Level"
}

function Assert-CaseMeterLevelFrequencyAbsent {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$Frequency,
        [Parameter(Mandatory = $true)][string]$Level,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing file for case meter guard: $Path"
    }

    $text = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    $meterBlockPattern = '(?ms)^\[\[meters\]\]\s*(.*?)(?=^\[\[|\n\[|\z)'
    $blocks = [regex]::Matches($text, $meterBlockPattern)
    $namePattern = '(?m)^name\s*=\s*"' + [regex]::Escape($Name) + '"\s*$'
    $frequencyPattern = '(?m)^frequency\s*=\s*"' + [regex]::Escape($Frequency) + '"\s*$'
    $levelPattern = '(?m)^level\s*=\s*"' + [regex]::Escape($Level) + '"\s*$'

    foreach ($block in $blocks) {
        $body = $block.Groups[1].Value
        if ($body -match $namePattern -and $body -match $frequencyPattern -and $body -match $levelPattern) {
            throw "Forbidden meter level for $Description`: name=$Name frequency=$Frequency level=$Level"
        }
    }

    Write-Host "OK forbidden meter level absent for $Description`: $Name / $Frequency != $Level"
}
