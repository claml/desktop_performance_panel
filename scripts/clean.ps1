param(
    [switch]$All
)

$ErrorActionPreference = "Continue"

$root = [System.IO.Path]::GetFullPath([System.IO.Path]::Combine($PSScriptRoot, ".."))

Write-Host "=== clean.ps1 ==="
Write-Host "Project root: $root"
Write-Host "All: $All"
Write-Host ""

function Remove-Dir {
    param([string]$Path)

    if (Test-Path $Path) {
        Write-Host "Removing: $Path"
        Remove-Item -Recurse -Force $Path
    } else {
        Write-Host "Skip missing: $Path"
    }
}

Remove-Dir ([System.IO.Path]::Combine($root, "apps", "desktop", "src-tauri", "target"))
Remove-Dir ([System.IO.Path]::Combine($root, "apps", "desktop", "dist"))
Remove-Dir ([System.IO.Path]::Combine($root, "apps", "desktop", "node_modules", ".vite"))

if ($All) {
    Remove-Dir ([System.IO.Path]::Combine($root, "services", "hardware-helper", "bin"))
    Remove-Dir ([System.IO.Path]::Combine($root, "services", "hardware-helper", "obj"))
}

Write-Host ""
Write-Host "Clean completed."
