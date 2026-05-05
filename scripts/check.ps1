$ErrorActionPreference = "Continue"

$root = [System.IO.Path]::GetFullPath([System.IO.Path]::Combine($PSScriptRoot, ".."))

Write-Host "=== check.ps1 ==="
Write-Host "Project root: $root"
Write-Host ""

function Check-Cmd {
    param(
        [string]$Name,
        [string[]]$Args
    )

    Write-Host "Checking $Name ..."
    try {
        $output = & $Name @Args 2>&1 | Select-Object -First 1
        Write-Host "[OK] $Name - $output"
    } catch {
        Write-Host "[FAIL] $Name - $($_.Exception.Message)"
    }
}

Write-Host "=== Toolchain ==="
Check-Cmd "dotnet" @("--version")
Check-Cmd "rustc" @("--version")
Check-Cmd "cargo" @("--version")
Check-Cmd "node" @("-v")
Check-Cmd "npm" @("-v")

Write-Host ""
Write-Host "=== Project Files ==="

$nodeModules = [System.IO.Path]::Combine($root, "apps", "desktop", "node_modules")
$helperExe = [System.IO.Path]::Combine($root, "services", "hardware-helper", "bin", "Release", "net8.0", "hardware-helper.exe")
$settingsJson = [System.IO.Path]::Combine($env:APPDATA, "desktop-performance-panel", "settings.json")

if (Test-Path $nodeModules) {
    Write-Host "[OK] node_modules: $nodeModules"
} else {
    Write-Host "[WARN] node_modules missing: $nodeModules"
}

if (Test-Path $helperExe) {
    Write-Host "[OK] helper exe: $helperExe"
} else {
    Write-Host "[WARN] helper exe missing: $helperExe"
}

if (Test-Path $settingsJson) {
    Write-Host "[OK] settings.json: $settingsJson"
} else {
    Write-Host "[WARN] settings.json missing: $settingsJson"
}

Write-Host ""
Write-Host "Check completed."
