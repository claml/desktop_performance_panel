$ErrorActionPreference = "Stop"

$root = [System.IO.Path]::GetFullPath([System.IO.Path]::Combine($PSScriptRoot, ".."))

$desktopDir = [System.IO.Path]::Combine($root, "apps", "desktop")
$tauriDir = [System.IO.Path]::Combine($desktopDir, "src-tauri")
$helperDir = [System.IO.Path]::Combine($root, "services", "hardware-helper")
$distAlpha = [System.IO.Path]::Combine($root, "dist-alpha")
$distResources = [System.IO.Path]::Combine($distAlpha, "resources")
$distDocs = [System.IO.Path]::Combine($distAlpha, "docs")

$tauriReleaseExe = [System.IO.Path]::Combine($tauriDir, "target", "release", "desktop-performance-panel.exe")
$distMainExe = [System.IO.Path]::Combine($distAlpha, "desktop-performance-panel.exe")

Write-Host "=== package-alpha.ps1 ==="
Write-Host "Project root: $root"
Write-Host ""

Write-Host "=== Step 1: Clean dist-alpha ==="
if (Test-Path $distAlpha) {
    Remove-Item -Recurse -Force $distAlpha
}

New-Item -ItemType Directory -Force -Path $distAlpha | Out-Null
New-Item -ItemType Directory -Force -Path $distResources | Out-Null
New-Item -ItemType Directory -Force -Path $distDocs | Out-Null

Write-Host "Created: $distAlpha"
Write-Host ""

Write-Host "=== Step 2: Publish hardware-helper ==="
Push-Location $helperDir

dotnet restore
if ($LASTEXITCODE -ne 0) {
    Pop-Location
    exit $LASTEXITCODE
}

dotnet publish -c Release -r win-x64 --self-contained false -o $distResources
if ($LASTEXITCODE -ne 0) {
    Pop-Location
    exit $LASTEXITCODE
}

Pop-Location

$helperExe = [System.IO.Path]::Combine($distResources, "hardware-helper.exe")
$helperDll = [System.IO.Path]::Combine($distResources, "hardware-helper.dll")
$helperRuntimeConfig = [System.IO.Path]::Combine($distResources, "hardware-helper.runtimeconfig.json")
$helperDeps = [System.IO.Path]::Combine($distResources, "hardware-helper.deps.json")

if (-not (Test-Path $helperExe)) {
    Write-Error "Missing: $helperExe"
    exit 1
}
if (-not (Test-Path $helperDll)) {
    Write-Error "Missing: $helperDll"
    exit 1
}
if (-not (Test-Path $helperRuntimeConfig)) {
    Write-Error "Missing: $helperRuntimeConfig"
    exit 1
}
if (-not (Test-Path $helperDeps)) {
    Write-Error "Missing: $helperDeps"
    exit 1
}

Write-Host "[OK] hardware-helper published to:"
Write-Host $distResources
Write-Host ""

Write-Host "=== Step 3: Build Tauri release ==="
Push-Location $desktopDir

npm run build
if ($LASTEXITCODE -ne 0) {
    Pop-Location
    exit $LASTEXITCODE
}

npm run tauri build
if ($LASTEXITCODE -ne 0) {
    Pop-Location
    exit $LASTEXITCODE
}

Pop-Location

if (-not (Test-Path $tauriReleaseExe)) {
    Write-Error "Missing Tauri release exe: $tauriReleaseExe"
    exit 1
}

Copy-Item -Force $tauriReleaseExe $distMainExe

Write-Host "[OK] copied main exe:"
Write-Host $distMainExe
Write-Host ""

Write-Host "=== Step 4: Copy docs ==="

$docsToCopy = @(
    "README_DEV.md",
    "RELEASE_ALPHA.md",
    "KNOWN_ISSUES.md"
)

foreach ($doc in $docsToCopy) {
    $src = [System.IO.Path]::Combine($root, "docs", $doc)
    $dst = [System.IO.Path]::Combine($distDocs, $doc)

    if (Test-Path $src) {
        Copy-Item -Force $src $dst
        Write-Host "[OK] copied $doc"
    } else {
        Write-Host "[WARN] missing doc: $src"
    }
}

Write-Host ""
Write-Host "=== dist-alpha ready ==="
Get-ChildItem -Recurse $distAlpha | ForEach-Object {
    Write-Host $_.FullName.Replace($root + "\", "")
}

Write-Host ""
Write-Host "Run:"
Write-Host ".\dist-alpha\desktop-performance-panel.exe"
