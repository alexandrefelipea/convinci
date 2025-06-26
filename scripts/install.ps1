<#
.SYNOPSIS
Robust installer for Convinci on Windows

.DESCRIPTION
Downloads and installs Convinci with progress tracking and retries
#>

# Configuration
$REPO_OWNER = "alexandrefelipea"
$REPO_NAME = "convinci"
$TARGET = "x86_64-pc-windows-gnu"
$BINARY_NAME = "convinci.exe"
$INSTALL_DIR = "$env:USERPROFILE\bin"
$MAX_RETRIES = 3

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

# Get latest version
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

# Download with progress and retries
function Download-File {
    param(
        [string]$Url,
        [string]$OutputPath,
        [int]$Retries = 3
    )

    $attempt = 0
    while ($attempt -lt $Retries) {
        $attempt++
        try {
            Write-Host "📥 Download attempt $attempt/$Retries..." -ForegroundColor Cyan
            Invoke-WebRequest -Uri $Url -OutFile $OutputPath -ErrorAction Stop

            if ((Get-Item $OutputPath).Length -gt 0) {
                Write-Host "✅ Download completed successfully" -ForegroundColor Green
                return
            } else {
                throw "Downloaded file is empty."
            }
        }
        catch {
            $lastError = $_.Exception.Message
            Write-Host "⚠️ Download failed on attempt ${attempt}: $lastError" -ForegroundColor Yellow

            if ($attempt -lt $Retries) {
                Write-Host "⏳ Retrying in 5 seconds..." -ForegroundColor Yellow
                Start-Sleep -Seconds 5
            }
        }
    }

    Exit-WithError "Failed to download after $Retries attempts. Last error: $lastError"
}

# Add git aliases
function Add-GitAliases {
    Write-Host "➕ Adding git aliases..." -ForegroundColor Cyan

    try {
        git config --global alias.convinci "!$INSTALL_PATH"
        git config --global alias.cv "!$INSTALL_PATH"
        Write-Host "✅ Added global git aliases: 'git convinci' and 'git cv'" -ForegroundColor Green
    }
    catch {
        Write-Host "⚠️ Could not add global git aliases. Adding local instead." -ForegroundColor Yellow
        git config alias.convinci "!$INSTALL_PATH"
        git config alias.cv "!$INSTALL_PATH"
        Write-Host "✅ Added local git aliases: 'git convinci' and 'git cv'" -ForegroundColor Green
    }
}


# Extract ZIP file using .NET libraries
function Extract-Zip {
    param(
        [string]$ZipFile,
        [string]$Destination
    )

    try {
        Add-Type -AssemblyName System.IO.Compression.FileSystem
        if (-not (Test-Path -Path $Destination)) {
            New-Item -ItemType Directory -Path $Destination -Force | Out-Null
        }
        [System.IO.Compression.ZipFile]::ExtractToDirectory($ZipFile, $Destination)
    }
    catch {
        Exit-WithError "Failed to extract ZIP: $_"
    }
}

# Main installation flow
try {
    Write-Host "`n🎯 convinci Windows Installer" -ForegroundColor Cyan
    Write-Host "============================"

    $version = Get-LatestVersion
    Write-Host "Latest version: $version" -ForegroundColor Magenta

    if (-not (Test-Path -Path $INSTALL_DIR)) {
        New-Item -ItemType Directory -Path $INSTALL_DIR | Out-Null
        Write-Host "Created install directory: $INSTALL_DIR"
    }

    $DOWNLOAD_URL = "https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$version/convinci-${TARGET}.zip"

    $TEMP_DIR = Join-Path -Path $env:TEMP -ChildPath "convinci-install-$version"
    if (Test-Path -Path $TEMP_DIR) {
        Remove-Item -Path $TEMP_DIR -Recurse -Force
    }
    New-Item -ItemType Directory -Path $TEMP_DIR | Out-Null

    $ZIP_FILE = Join-Path -Path $TEMP_DIR -ChildPath "convinci.zip"
    Write-Host "📦 Downloading convinci $version..." -ForegroundColor Cyan
    Download-File -Url $DOWNLOAD_URL -OutputPath $ZIP_FILE -Retries $MAX_RETRIES

    Write-Host "🔍 Extracting files..." -ForegroundColor Cyan
    Extract-Zip -ZipFile $ZIP_FILE -Destination $TEMP_DIR

    $BINARY_PATH = Join-Path -Path $TEMP_DIR -ChildPath $BINARY_NAME
    if (-not (Test-Path -Path $BINARY_PATH)) {
        Exit-WithError "Binary not found in package"
    }

    $INSTALL_PATH = Join-Path -Path $INSTALL_DIR -ChildPath $BINARY_NAME
    Copy-Item -Path $BINARY_PATH -Destination $INSTALL_PATH -Force
    Write-Host "✅ Installed to: $INSTALL_PATH" -ForegroundColor Green

    Add-GitAliases

    $CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($CurrentPath -notlike "*$INSTALL_DIR*") {
        $newPath = ($CurrentPath.Split(';') | Where-Object { $_ -ne $INSTALL_DIR }) + $INSTALL_DIR
        [Environment]::SetEnvironmentVariable("Path", ($newPath -join ';'), "User")
        Write-Host "➡️ Added $INSTALL_DIR to user PATH" -ForegroundColor Green
        Write-Host "   Please restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
    }

    Remove-Item -Path $TEMP_DIR -Recurse -Force -ErrorAction SilentlyContinue

    Write-Host "`n🎉 Installation completed successfully!" -ForegroundColor Green
    Write-Host "Run Convinci with: convinci" -ForegroundColor Cyan
    Write-Host "Documentation: https://github.com/$REPO_OWNER/$REPO_NAME" -ForegroundColor Cyan
    Write-Host "`nNote: Just added Convinci to PATH, please restart your terminal.`n" -ForegroundColor Yellow
}
catch {
    Exit-WithError "Installation failed: $_"
}
