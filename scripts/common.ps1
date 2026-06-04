function Add-CargoBinToPath {
    $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if (-not (Test-Path -LiteralPath $cargoBin)) {
        return
    }

    $parts = $env:Path -split ";"
    foreach ($part in $parts) {
        if ($part.Equals($cargoBin, [System.StringComparison]::OrdinalIgnoreCase)) {
            return
        }
    }

    $env:Path = "$cargoBin;$env:Path"
}

