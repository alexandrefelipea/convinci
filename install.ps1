<#
.SYNOPSIS
Installer for Convinci on Windows

.DESCRIPTION
Downloads and installs Convinci - Conventional Commits Helper
#>

$REPO_OWNER = "alexandrefelipea"
$REPO_NAME = "convinci"
$TARGET = "x86_64-pc-windows-gnu"
$BINARY_NAME = "convinci.exe"
$INSTALL_DIR = "$env:USERPROFILE\bin"  # Directory in user's PATH
$TEMP_PATH = [System.IO.Path]::GetTempFileName()

# Error handling
function Exit-WithError {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red
    exit 1
}

function Show-Warning {
    param([string]$Message)
    Write-Host "⚠️ $Message" -ForegroundColor Yellow
}

function Get-LatestVersion {
    $API_URL = "https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
    try {
        $response = Invoke-RestMethod -Uri $API_URL -ErrorAction Stop
        return $response.tag_name
    }
    catch {
        Exit-WithError "Failed to retrieve latest version: $_"
    }
}

try {
    Write-Host "`n🎯 convinci Windows Installer" -ForegroundColor Cyan
    Write-Host "============================"

    # Get latest version
    $version = Get-LatestVersion
    Write-Host "Latest version: $version" -ForegroundColor Magenta

    # Create install directory if needed
    if (-not (Test-Path -Path $INSTALL_DIR)) {
        New-Item -ItemType Directory -Path $INSTALL_DIR | Out-Null
        Write-Host "Created install directory: $INSTALL_DIR"
    }

    # Build download URL (CORRECTED)
    $DOWNLOAD_URL = "https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$version/convinci-${TARGET}.zip"

    # Download
    Write-Host "📦 Downloading convinci $version..." -ForegroundColor Cyan
    try {
        Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TEMP_PATH -ErrorAction Stop
    }
    catch {
        Exit-WithError "Download failed: $_"
    }

    # Extract
    Write-Host "🔍 Extracting files..." -ForegroundColor Cyan
    $EXTRACT_PATH = Join-Path -Path $env:TEMP -ChildPath "convinci-$version"

    if (Test-Path -Path $EXTRACT_PATH) {
        Remove-Item -Path $EXTRACT_PATH -Recurse -Force
    }

    Expand-Archive -Path $TEMP_PATH -DestinationPath $EXTRACT_PATH -Force

    # Find binary
    $BINARY_PATH = Join-Path -Path $EXTRACT_PATH -ChildPath $BINARY_NAME
    if (-not (Test-Path -Path $BINARY_PATH)) {
        Exit-WithError "Binary not found in package"
    }

    # Install
    $INSTALL_PATH = Join-Path -Path $INSTALL_DIR -ChildPath $BINARY_NAME
    Copy-Item -Path $BINARY_PATH -Destination $INSTALL_PATH -Force
    Write-Host "✅ Installed to: $INSTALL_PATH" -ForegroundColor Green

    # Add to PATH if needed
    $CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($CurrentPath -notlike "*$INSTALL_DIR*") {
        [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$INSTALL_DIR", "User")
        Write-Host "➡️ Added $INSTALL_DIR to user PATH" -ForegroundColor Green
        Write-Host "   Please restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
    }

    # Cleanup
    Remove-Item -Path $TEMP_PATH -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $EXTRACT_PATH -Recurse -Force -ErrorAction SilentlyContinue

    Write-Host "`n🎉 Installation completed successfully!" -ForegroundColor Green
    Write-Host "Run Convinci with: convinci" -ForegroundColor Cyan
    Write-Host "Documentation: https://github.com/$REPO_OWNER/$REPO_NAME" -ForegroundColor Cyan
}
catch {
    Exit-WithError "Installation failed: $_"
}