[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$Artifact,
    [string]$EvidenceRoot = "",
    [string[]]$EvidenceAssetNames = @(
        "numeric-conformance-evidence.pdf",
        "conformance-index-report.pdf",
        "support-coverage-report.pdf",
        "user-coverage-handbook.pdf"
    ),
    [switch]$RequireEvidenceAssets
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Add-ResolvedAsset {
    param(
        [System.Collections.Generic.List[string]]$Assets,
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }

    $Assets.Add((Resolve-Path -LiteralPath $Path).Path) | Out-Null
}

$assets = New-Object System.Collections.Generic.List[string]
Add-ResolvedAsset -Assets $assets -Path $Artifact -Description "release package"

if (-not [string]::IsNullOrWhiteSpace($EvidenceRoot)) {
    if (Test-Path -LiteralPath $EvidenceRoot -PathType Container) {
        $publicNames = @{}
        foreach ($name in $EvidenceAssetNames) {
            $publicNames[$name.ToLowerInvariant()] = $true
            $path = Join-Path $EvidenceRoot $name
            if (Test-Path -LiteralPath $path -PathType Leaf) {
                Add-ResolvedAsset -Assets $assets -Path $path -Description "public evidence asset"
            }
            elseif ($RequireEvidenceAssets) {
                throw "Missing required public evidence asset: $path"
            }
            else {
                Write-Warning "Skipping missing public evidence asset: $path"
            }
        }

        $localOnly = @(
            Get-ChildItem -LiteralPath $EvidenceRoot -File |
                Where-Object { -not $publicNames.ContainsKey($_.Name.ToLowerInvariant()) } |
                Sort-Object Name
        )
        if ($localOnly.Count -gt 0) {
            Write-Verbose "Keeping local-only evidence out of GitHub Release assets:"
            foreach ($file in $localOnly) {
                Write-Verbose " - $($file.Name)"
            }
        }
    }
    elseif ($RequireEvidenceAssets) {
        throw "Missing release evidence directory: $EvidenceRoot"
    }
    else {
        Write-Warning "Skipping missing release evidence directory: $EvidenceRoot"
    }
}

foreach ($asset in $assets) {
    Write-Output $asset
}
