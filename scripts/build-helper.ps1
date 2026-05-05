$ErrorActionPreference = "Stop"

$root = [System.IO.Path]::GetFullPath([System.IO.Path]::Combine($PSScriptRoot, ".."))
$helperDir = [System.IO.Path]::Combine($root, "services", "hardware-helper")
$helperExe = [System.IO.Path]::Combine($helperDir, "bin", "Release", "net8.0", "hardware-helper.exe")

Write-Host "=== build-helper.ps1 ==="
Write-Host "Project root: $root"
Write-Host "Helper dir: $helperDir"
Write-Host ""

if (-not (Test-Path $helperDir)) {
    Write-Error "Helper directory not found: $helperDir"
    exit 1
}

Push-Location $helperDir

Write-Host "Running dotnet restore..."
dotnet restore
if ($LASTEXITCODE -ne 0) {
    Pop-Location
    exit $LASTEXITCODE
}

Write-Host "Running dotnet build -c Release..."
dotnet build -c Release
if ($LASTEXITCODE -ne 0) {
    Pop-Location
    exit $LASTEXITCODE
}

Pop-Location

if (-not (Test-Path $helperExe)) {
    Write-Error "Build finished but exe not found: $helperExe"
    exit 1
}

Write-Host ""
Write-Host "[OK] hardware-helper built:"
Write-Host $helperExe
exit 0
