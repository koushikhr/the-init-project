# --- CONFIGURATION ---
$Repo = "https://github.com/koushikhr/the-init-project"
$Url = "https://github.com/$Repo/releases/latest/download/init-windows.zip"
# ---------------------

Write-Host "🚀 Initializing Setup..." -ForegroundColor Cyan

# 1. Setup Temp Directory
$InstallDir = Join-Path $env:TEMP "init_bootstrap"
# Clean up previous runs if they exist
if (Test-Path $InstallDir) { Remove-Item -Path $InstallDir -Recurse -Force }
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# 2. Download
$ZipPath = Join-Path $InstallDir "init.zip"
Write-Host "⬇️  Downloading from GitHub..." -ForegroundColor Cyan
try {
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
