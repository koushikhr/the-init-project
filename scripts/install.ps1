# --- CONFIGURATION ---
$Repo = "koushikhr/the-init-project"

# Fix: TLS 1.2 is required for GitHub on older Windows versions
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

# Get Latest Tag
Write-Host "🔍 Checking for latest version..." -ForegroundColor Cyan
try {
    $LatestRel = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    $Tag = $LatestRel.tag_name
} catch {
    Write-Error "Failed to fetch latest version info. Check your internet connection."
    exit 1
}

if (-not $Tag) {
    Write-Error "Could not find latest release tag."
    exit 1
}

Write-Host "⬇️  Found version: $Tag" -ForegroundColor Cyan
$Url = "https://github.com/$Repo/releases/download/$Tag/the-init-project-windows.zip"
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
    Invoke-RestMethod -Uri $Url -OutFile $ZipPath
} catch {
    Write-Error "Failed to download. Check your internet connection or if the Release exists."
    exit 1
}

# 3. Extract
Write-Host "📦 Extracting..." -ForegroundColor Cyan
Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force

# 4. Update Config & Icons
Write-Host "⬇️  Updating Configuration & Assets..." -ForegroundColor Cyan
$ConfigUrl = "https://raw.githubusercontent.com/$Repo/master/apps.toml"
$IconsBase = "https://raw.githubusercontent.com/$Repo/master/icons"

try {
    Invoke-RestMethod -Uri $ConfigUrl -OutFile (Join-Path $InstallDir "apps.toml")

    $IconsDir = Join-Path $InstallDir "icons"
    if (-not (Test-Path $IconsDir)) { New-Item -ItemType Directory -Path $IconsDir | Out-Null }

    $Icons = @("default.svg", "firefox.svg", "vscode.svg", "vlc.svg", "discord.svg")
    foreach ($Icon in $Icons) {
        Invoke-RestMethod -Uri "$IconsBase/$Icon" -OutFile (Join-Path $IconsDir $Icon)
    }
} catch {
    Write-Warning "Failed to update config/icons from master. Using bundled versions."
}

# 5. Run
Write-Host "✅ Starting Init..." -ForegroundColor Green
Set-Location $InstallDir
.\init_app.exe
