# --- CONFIGURATION ---
$Repo = "koushikhr/the-init-project"
# MAKE SURE THIS FILENAME MATCHES WHAT YOU UPLOADED
$Url = "https://github.com/$Repo/releases/download/v0.1.0/the-init-project-windows.zip"
# ---------------------

Write-Host "🚀 Initializing Setup..." -ForegroundColor Cyan

# 1. Setup Temp Directory
$InstallDir = Join-Path $env:TEMP "init_bootstrap"
if (Test-Path $InstallDir) { Remove-Item -Path $InstallDir -Recurse -Force }
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# 2. Download
$ZipPath = Join-Path $InstallDir "init.zip"
Write-Host "⬇️  Downloading from GitHub..." -ForegroundColor Cyan
try {
    # Fix: TLS 1.2 is required for GitHub on older Windows versions
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-RestMethod -Uri $Url -OutFile $ZipPath
} catch {
    Write-Error "Failed to download. Check your internet connection or if the Release exists."
    exit 1
}

# 3. Extract
Write-Host "📦 Extracting..." -ForegroundColor Cyan
Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force

# 4. Run
Write-Host "✅ Starting Init..." -ForegroundColor Green
Set-Location $InstallDir
.\init_app.exe
