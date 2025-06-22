<#
.SYNOPSIS
Convinci Installer for Windows

.DESCRIPTION
Downloads and installs Convinci - Conventional Commits Assistant
#>

# Configurations
$REPO_OWNER = "alexandrefelipea"
$REPO_NAME = "convinci"
$VERSION = "v0.1.0" # Update with each new version
$TARGET = "x86_64-pc-windows-gnu"
$DOWNLOAD_URL = "https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$VERSION/convinci-${VERSION}-${TARGET}.zip"
$INSTALL_DIR = "$env:USERPROFILE\bin" # Can be another directory in PATH
$BINARY_NAME = "convinci.exe"
$TEMP_PATH = [System.IO.Path]::GetTempFileName()

# Check and create installation directory
if (-not (Test-Path -Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR | Out-Null
}

# Download the binary
Write-Host "📦 Downloading Convinci $VERSION..." -ForegroundColor Cyan
try {
    (New-Object System.Net.WebClient).DownloadFile($DOWNLOAD_URL, $TEMP_PATH)
}
catch {
    Write-Host "❌ Download failed: $_" -ForegroundColor Red
    exit 1
}

# Extract
Write-Host "🔍 Extracting files..." -ForegroundColor Cyan
$ZIP_PATH = $TEMP_PATH
$EXTRACT_PATH = Join-Path -Path $env:TEMP -ChildPath "convinci-$VERSION"

if (Test-Path -Path $EXTRACT_PATH) {
    Remove-Item -Path $EXTRACT_PATH -Recurse -Force
}

Expand-Archive -Path $ZIP_PATH -DestinationPath $EXTRACT_PATH -Force

# Install
$BINARY_PATH = Join-Path -Path $EXTRACT_PATH -ChildPath $BINARY_NAME
if (-not (Test-Path -Path $BINARY_PATH)) {
    Write-Host "❌ Binary not found!" -ForegroundColor Red
    exit 1
}

Copy-Item -Path $BINARY_PATH -Destination "$INSTALL_DIR\$BINARY_NAME" -Force

# Add to PATH if necessary
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$INSTALL_DIR*") {
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$INSTALL_DIR", "User")
    Write-Host "✅ Added $INSTALL_DIR to user PATH" -ForegroundColor Green
    Write-Host "⚠️ Please restart your terminal to apply PATH changes" -ForegroundColor Yellow
}

# Cleanup
Remove-Item -Path $ZIP_PATH -Force
Remove-Item -Path $EXTRACT_PATH -Recurse -Force -ErrorAction SilentlyContinue

# Final message
Write-Host "`n🎉 Installation completed successfully!" -ForegroundColor Green
Write-Host "Run Convinci with: convinci" -ForegroundColor Cyan
Write-Host "Documentation: https://github.com/$REPO_OWNER/$REPO_NAME`n" -ForegroundColor Cyan