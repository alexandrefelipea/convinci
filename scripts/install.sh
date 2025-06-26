#!/bin/bash

set -euo pipefail

# Configuration
REPO_OWNER="alexandrefelipea"
REPO_NAME="convinci"
BINARY_NAME="convinci"
INSTALL_DIR="/usr/local/bin"
TMP_DIR=$(mktemp -d)

# Error messages
error() {
    echo -e "\033[1;31mError: $1\033[0m" >&2
    exit 1
}

warn() {
    echo -e "\033[1;33mWarning: $1\033[0m" >&2
}

# Check dependencies
check_dependencies() {
    local missing=()

    if ! command -v curl &> /dev/null; then
        missing+=("curl")
    fi

    if ! command -v unzip &> /dev/null && ! command -v tar &> /dev/null; then
        missing+=("unzip or tar")
    fi

    if [ ${#missing[@]} -gt 0 ]; then
        error "Missing dependencies: ${missing[*]}"
    fi
}

# Detect system and architecture
detect_system() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$ARCH" in
        x86_64|amd64) ARCH="x86_64" ;;
        arm64|aarch64) ARCH="aarch64" ;;
        *) error "Unsupported architecture: $ARCH" ;;
    esac

    case "$OS" in
        linux) TARGET="${ARCH}-unknown-linux-musl" ;;
        darwin) TARGET="${ARCH}-apple-darwin" ;;
        *) error "Unsupported operating system: $OS" ;;
    esac
}

# Get latest version
get_latest_version() {
    API_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
    VERSION=$(curl -sSL "$API_URL" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$VERSION" ]; then
        error "Could not retrieve the latest version"
    fi

    echo "$VERSION"
}

# Download and install
install_convinci() {
    local version="$1"

    # CORREÇÃO: Removido o ${version} do nome do arquivo
    DOWNLOAD_URL="https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$version/convinci-${TARGET}.tar.gz"

    echo "📦 Downloading convinci $version for $TARGET..."
    curl -sSL -o "$TMP_DIR/convinci.tar.gz" "$DOWNLOAD_URL" || error "Failed to download binary"

    echo "🔍 Verifying binary..."
    tar xzf "$TMP_DIR/convinci.tar.gz" -C "$TMP_DIR" || error "Failed to extract file"

    local bin_path="$TMP_DIR/$BINARY_NAME"
    if [ ! -f "$bin_path" ]; then
        error "Binary not found in the package"
    fi

    chmod +x "$bin_path" || warn "Failed to add execute permission"

    echo "🚀 Installing to $INSTALL_DIR..."
    if [ ! -w "$INSTALL_DIR" ]; then
        echo "🔒 Requires sudo permission to install to $INSTALL_DIR"
        sudo mv "$bin_path" "$INSTALL_DIR/$BINARY_NAME" || error "Failed to install with sudo"
    else
        mv "$bin_path" "$INSTALL_DIR/$BINARY_NAME" || error "Failed to install"
    fi
}

# Verify installation
verify_installation() {
    if command -v "$BINARY_NAME" &> /dev/null; then
        echo -e "\n✅ \033[1;32mInstallation completed successfully!\033[0m"
        echo "Run convinci with: $BINARY_NAME, git convinci, git cv"
    else
        warn "Installation seems to have completed, but the binary is not in your PATH"
        echo "Please add $INSTALL_DIR to your PATH:"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
}

# Cleanup
cleanup() {
    rm -rf "$TMP_DIR"
}

# Add git aliases
add_git_aliases() {
     echo "➕ Adding git aliases..."

        if git config --global alias.convinci "!$INSTALL_DIR/$BINARY_NAME" && \
           git config --global alias.cv "!$INSTALL_DIR/$BINARY_NAME"; then
            echo "✅ Added global git aliases: 'git convinci' and 'git cv'"
        else
            echo "⚠️ Failed to add global git aliases. You may need to set them manually." >&2
            echo "   Run these commands to set them manually:"
            echo "   git config --global alias.convinci \"!$INSTALL_DIR/$BINARY_NAME\""
            echo "   git config --global alias.cv \"!$INSTALL_DIR/$BINARY_NAME\""
        fi
}

# Main flow
main() {
    echo -e "\n\033[1;34mconvinci Installer\033[0m"
    echo "====================="

    check_dependencies
    detect_system
    version=$(get_latest_version)

    install_convinci "$version"
    add_git_aliases
    verify_installation

    echo -e "\n💡 Tip: Run 'convinci --help' to see options"
    echo "📄 Documentation: https://github.com/$REPO_OWNER/$REPO_NAME"
}

trap cleanup EXIT
main
