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