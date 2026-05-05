$ErrorActionPreference = "Stop"

$root = [System.IO.Path]::GetFullPath([System.IO.Path]::Combine($PSScriptRoot, ".."))
$buildHelper = [System.IO.Path]::Combine($root, "scripts", "build-helper.ps1")
$desktopDir = [System.IO.Path]::Combine($root, "apps", "desktop")

Write-Host "=== dev.ps1 ==="
Write-Host "Project root: $root"
Write-Host ""

Write-Host "Step 1: build hardware-helper"
& $buildHelper
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "Step 2: start Tauri dev"
Push-Location $desktopDir
npm run tauri dev
$code = $LASTEXITCODE
Pop-Location

exit $code
