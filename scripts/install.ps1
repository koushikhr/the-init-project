# # --- CONFIGURATION ---
# $Repo = "koushikhr/the-init-project"
# # MAKE SURE THIS FILENAME MATCHES WHAT YOU UPLOADED
# $Url = "https://github.com/$Repo/releases/download/v0.1.0/the-init-project-windows.zip"
# # ---------------------

# Write-Host "🚀 Initializing Setup..." -ForegroundColor Cyan

# # 1. Setup Temp Directory
# $InstallDir = Join-Path $env:TEMP "init_bootstrap"
# if (Test-Path $InstallDir) { Remove-Item -Path $InstallDir -Recurse -Force }
# New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# # 2. Download
# $ZipPath = Join-Path $InstallDir "init.zip"
# Write-Host "⬇️  Downloading from GitHub..." -ForegroundColor Cyan
# try {
#     # Fix: TLS 1.2 is required for GitHub on older Windows versions
#     [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
#     Invoke-RestMethod -Uri $Url -OutFile $ZipPath
# } catch {
#     Write-Error "Failed to download. Check your internet connection or if the Release exists."
#     exit 1
# }

# # 3. Extract
# Write-Host "📦 Extracting..." -ForegroundColor Cyan
# Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force

# # 4. Run
# Write-Host "✅ Starting Init..." -ForegroundColor Green
# Set-Location $InstallDir
# .\init_app.exe


# --- CONFIGURATION ---
$Repo = "koushikhr/the-init-project"
# ---------------------

Write-Host "🚀 Initializing Setup..." -ForegroundColor Cyan

# 1. Fetch Latest Release Tag
Write-Host "🔍 Checking for latest version..." -ForegroundColor Cyan
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $LatestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    $Tag = $LatestRelease.tag_name
    
    if ([string]::IsNullOrWhiteSpace($Tag)) {
        throw "Could not find a valid tag name."
    }
    
    Write-Host "⭐ Found version: $Tag" -ForegroundColor Green
} catch {
    Write-Error "Failed to fetch latest release info from GitHub. Check repo name and internet."
    exit 1
}

# 2. Construct Download URL
# NOTE: Ensure the filename here matches exactly what your GitHub Action uploads!
$Filename = "the-init-project-windows.zip" 
$Url = "https://github.com/$Repo/releases/download/$Tag/$Filename"

# 3. Setup Temp Directory
$InstallDir = Join-Path $env:TEMP "init_bootstrap"
if (Test-Path $InstallDir) { Remove-Item -Path $InstallDir -Recurse -Force }
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# 4. Download
$ZipPath = Join-Path $InstallDir "init.zip"
Write-Host "⬇️  Downloading..." -ForegroundColor Cyan
try {
    Invoke-RestMethod -Uri $Url -OutFile $ZipPath
} catch {
    Write-Error "Failed to download binary. URL attempted: $Url"
    exit 1
}

# 5. Extract
Write-Host "📦 Extracting..." -ForegroundColor Cyan
try {
    Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force
} catch {
    Write-Error "Failed to extract archive. The download might be corrupt."
    exit 1
}

# 6. Run
Write-Host "✅ Starting Init..." -ForegroundColor Green
Set-Location $InstallDir

# Check if the exe exists before running (common error if archive structure changes)
if (Test-Path ".\init_app.exe") {
    .\init_app.exe
} else {
    Write-Error "Executable not found in extracted folder. Contents:"
    Get-ChildItem . | Format-Table Name
    exit 1
}
