[CmdletBinding()]
param(
    [string]$BodyPath,
    [string]$Body,
    [switch]$SelfTest
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

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

function Test-CheckedItem {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Label
    )
    $pattern = "(?im)^\s*-\s*\[[xX]\]\s*$([regex]::Escape($Label))\s*$"
    return $Text -match $pattern
}

function Get-TicketField {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name
    )
    $pattern = "(?im)^\s*-\s*$([regex]::Escape($Name)):\s*(?<value>.*?)\s*$"
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        return $null
    }
    return $match.Groups["value"].Value.Trim()
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
    if ($placeholderValues -contains $value) {
        throw "Algorithm Port Ticket field still contains a placeholder: $Name"
    }
    return $value
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

function Test-AlgorithmPortTicketBody {
    param([AllowEmptyString()][Parameter(Mandatory = $true)][string]$Text)

    if ([string]::IsNullOrWhiteSpace($Text)) {
        throw "PR body is empty; source-order algorithm PRs require an Algorithm Port Ticket or an explicit non-algorithm checkbox."
    }

    if (Test-CheckedItem -Text $Text -Label "Not an algorithm/source-order change") {
        return [pscustomobject]@{
            status = "pass"
            classification = "not_algorithm_or_source_order_change"
        }
    }

    $commonFields = @(
        "Ticket path or PR section",
        "Algorithm ID",
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
        "First target case",
        "Blocking gate",
        "Conformance claim",
        "Not-claimed branches"
    )
    $values = @{}
    foreach ($field in $commonFields) {
        $values[$field] = Assert-FieldValue -Text $Text -Name $field
    }

    $portType = $values["Port type"].Trim().ToLowerInvariant()
    switch ($portType) {
        "compatibility" {
            if (-not (Test-CheckedItem -Text $Text -Label "Compatibility port ticket completed")) {
                throw "Compatibility source-order ports must check 'Compatibility port ticket completed'."
            }
            Assert-FieldEquals -Actual $values["Compatibility path"] -Expected "true" -Name "Compatibility path"
            Assert-FieldEquals -Actual $values["Diagnostic probe used"] -Expected "false" -Name "Diagnostic probe used"
        }
        "diagnostic_probe" {
            if (-not (Test-CheckedItem -Text $Text -Label "Diagnostic probe only; no conformance claim")) {
                throw "Diagnostic probe PRs must check 'Diagnostic probe only; no conformance claim'."
            }
            Assert-FieldEquals -Actual $values["Compatibility path"] -Expected "false" -Name "Compatibility path"
            Assert-FieldEquals -Actual $values["Diagnostic probe used"] -Expected "true" -Name "Diagnostic probe used"
            Assert-FieldEquals -Actual $values["Conformance claim"] -Expected "no" -Name "Conformance claim"
            [void](Assert-FieldValue -Text $Text -Name "Diagnostic-only variables")
        }
        "refactor_only" {
            Assert-FieldEquals -Actual $values["Diagnostic probe used"] -Expected "false" -Name "Diagnostic probe used"
            Assert-FieldEquals -Actual $values["Conformance claim"] -Expected "no" -Name "Conformance claim"
        }
        default {
            throw "Port type must be compatibility, diagnostic_probe, or refactor_only, got '$($values["Port type"])'."
        }
    }

    return [pscustomobject]@{
        status = "pass"
        classification = $portType
        algorithm_id = $values["Algorithm ID"]
    }
}

function New-TestBody {
    param(
        [string]$PortType = "compatibility",
        [string]$CompatibilityPath = "true",
        [string]$DiagnosticProbeUsed = "false",
        [string]$ConformanceClaim = "yes",
        [string]$CheckedLine = "- [x] Compatibility port ticket completed"
    )
    return @"
## Algorithm Port Ticket

- [ ] Not an algorithm/source-order change
$CheckedLine
- Ticket path or PR section: PR body
- Algorithm ID: heat_balance/example
- Port type: $PortType
- EnergyPlus version: 26.1.0
- EnergyPlus source file: HeatBalanceManager.cc
- EnergyPlus routine: ManageHeatBalance
- EnergyPlus source-order stage: manage_heat_balance
- Rust target module: crates/ep_runtime/src/heat_balance/manager.rs
- Rust target function: manage_heat_balance
- ExecutionStageKind: ManageHeatBalance
- Compatibility path: $CompatibilityPath
- Diagnostic probe used: $DiagnosticProbeUsed
- Read state: zone state
- Write state: result store
- History/state ownership: heat_balance::state
- Unsupported state: fenestration
- Affected variables: Zone Mean Air Temperature
- Affected meters: none
- Diagnostic-only variables: storage probe
- First target case: official_1zone_uncontrolled_dynamic_001
- Proof variables: Zone Mean Air Temperature
- Tolerance candidate: 0.01 C
- Report path: .runtime/report.md
- Blocking gate: compare-heat-balance-conformance
- Conformance claim: $ConformanceClaim
- Not-claimed branches: HVAC, plant
- Partial run allowed: no
"@
}

function Invoke-SelfTest {
    $cases = @(
        [pscustomobject]@{
            name = "non_algorithm"
            body = "- [x] Not an algorithm/source-order change"
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "compatibility"
            body = New-TestBody
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "diagnostic"
            body = New-TestBody `
                -PortType "diagnostic_probe" `
                -CompatibilityPath "false" `
                -DiagnosticProbeUsed "true" `
                -ConformanceClaim "no" `
                -CheckedLine "- [x] Diagnostic probe only; no conformance claim"
            shouldPass = $true
        },
        [pscustomobject]@{
            name = "missing_ticket"
            body = "## Algorithm Port Ticket`n`n- [ ] Not an algorithm/source-order change"
            shouldPass = $false
        },
        [pscustomobject]@{
            name = "diagnostic_claim"
            body = New-TestBody `
                -PortType "diagnostic_probe" `
                -CompatibilityPath "false" `
                -DiagnosticProbeUsed "true" `
                -ConformanceClaim "yes" `
                -CheckedLine "- [x] Diagnostic probe only; no conformance claim"
            shouldPass = $false
        }
    )

    foreach ($case in $cases) {
        $passed = $true
        try {
            [void](Test-AlgorithmPortTicketBody -Text $case.body)
        }
        catch {
            $passed = $false
        }
        if ($passed -ne $case.shouldPass) {
            throw "Unexpected PR port-ticket self-test result for $($case.name): pass=$passed"
        }
        Write-Host "OK PR port-ticket self-test: $($case.name)"
    }
}

if ($SelfTest) {
    Invoke-SelfTest
    return
}

$bodyText = Get-PrBodyText -Path $BodyPath -InlineBody $Body
$result = Test-AlgorithmPortTicketBody -Text $bodyText
Write-Host "PR port-ticket check passed: $($result.classification)"
