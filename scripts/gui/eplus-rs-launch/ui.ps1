# WinForms helpers for eplus-rs-launch.ps1.

function New-ReadOnlyMultilineBox {
    $box = New-Object System.Windows.Forms.TextBox
    $box.Multiline = $true
    $box.ReadOnly = $true
    $box.ScrollBars = "Both"
    $box.WordWrap = $false
    $box.Dock = "Fill"
    return $box
}