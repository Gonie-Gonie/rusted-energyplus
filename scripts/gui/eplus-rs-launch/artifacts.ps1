# Artifact preview helpers for eplus-rs-launch.ps1.

function Read-ArtifactPreview {
    param(
        [string]$Path,
        [string]$MissingText,
        [int]$MaxCharacters = 12000
    )
    if (-not (Test-LeafPath -Path $Path)) {
        return $MissingText
    }
    try {
        $text = Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
        if ($text.Length -le $MaxCharacters) {
            return $text
        }
        return $text.Substring(0, $MaxCharacters) + "`r`n... truncated in launcher preview; open the artifact for the full file."
    }
    catch {
        return "Failed to read artifact: $Path"
    }
}

function Read-PlotArtifactPreview {
    param([string]$OutputDir)
    $plotRoots = @(
        (Join-Path $OutputDir "reports\plots"),
        (Join-Path $OutputDir "plots"),
        (Join-Path $OutputDir "compare\plots")
    )
    $plotExtensions = @(".png", ".jpg", ".jpeg", ".svg", ".pdf", ".csv")
    $files = @()
    foreach ($root in $plotRoots) {
        if (-not (Test-Path -LiteralPath $root -PathType Container)) {
            continue
        }
        $files += @(Get-ChildItem -LiteralPath $root -File -Recurse | Where-Object {
                $plotExtensions -contains $_.Extension.ToLowerInvariant()
            } | Sort-Object FullName)
    }
    if (@($files).Count -eq 0) {
        return "Plot artifacts are not available for this run."
    }
    $lines = @("Plot artifacts:")
    foreach ($file in $files) {
        $lines += $file.FullName
    }
    return ($lines -join "`r`n")
}
function Get-EvidenceArtifactPaths {
    param([string]$OutputDir)
    $reportsDir = Join-Path $OutputDir "reports"
    $candidatePaths = @(
        (Join-Path $reportsDir "evidence-summary.md"),
        (Join-Path $reportsDir "evidence-summary.pdf"),
        (Join-Path $reportsDir "numeric-conformance-evidence.pdf"),
        (Join-Path $reportsDir "release-evidence-manifest.pdf"),
        (Join-Path $reportsDir "user-coverage-handbook.pdf")
    )
    if (Test-Path -LiteralPath $reportsDir -PathType Container) {
        $candidatePaths += @(Get-ChildItem -LiteralPath $reportsDir -File -Recurse | Where-Object {
                @(".pdf", ".html", ".json", ".md") -contains $_.Extension.ToLowerInvariant() -and
                ($_.Name -match "evidence|manifest|handbook|summary")
            } | Sort-Object FullName | ForEach-Object { $_.FullName })
    }

    $paths = @()
    foreach ($candidate in $candidatePaths) {
        if (-not (Test-LeafPath -Path $candidate)) {
            continue
        }
        $resolved = (Resolve-Path -LiteralPath $candidate).Path
        if ($paths -notcontains $resolved) {
            $paths += $resolved
        }
    }
    return $paths
}

function Find-EvidenceArtifactPath {
    param([string]$OutputDir)
    $paths = @(Get-EvidenceArtifactPaths -OutputDir $OutputDir)
    if ($paths.Count -eq 0) {
        return $null
    }
    return $paths[0]
}

function Read-EvidenceArtifactPreview {
    param([string]$OutputDir)
    $paths = @(Get-EvidenceArtifactPaths -OutputDir $OutputDir)
    if ($paths.Count -eq 0) {
        return "Evidence summary/PDF artifacts are not available for this run."
    }

    $lines = @("Evidence artifacts:")
    foreach ($path in $paths) {
        $lines += $path
    }

    $previewPath = @($paths | Where-Object {
            @(".md", ".txt", ".json", ".html") -contains ([System.IO.Path]::GetExtension($_).ToLowerInvariant())
        } | Select-Object -First 1)
    if ($previewPath.Count -eq 0) {
        return ($lines -join "`r`n")
    }

    $lines += ""
    $lines += "Preview: $($previewPath[0])"
    $lines += ""
    $lines += Read-ArtifactPreview `
        -Path $previewPath[0] `
        -MissingText "Evidence summary preview is not available."
    return ($lines -join "`r`n")
}
