#!/bin/sh

set -e

REPO_OWNER="dalikewara"
REPO_NAME="uwais"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="uwais"
TMP_DIR="/tmp/uwais_install"

print_info() {
    printf "[INFO] %s\n" "$1"
}

print_error() {
    printf "[ERROR] %s\n" "$1"
}

print_warning() {
    printf "[WARNING] %s\n" "$1"
}

detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux*)
            PLATFORM="linux"
            ;;
        Darwin*)
            PLATFORM="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            PLATFORM="windows"
            ;;
        *)
            print_error "Unsupported operating system: $OS"
            exit 1
            ;;
    esac

    print_info "Detected platform: $PLATFORM ($ARCH)"
}

get_latest_release_url() {
    print_info "Fetching latest release information from GitHub..."

    RELEASE_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"

    if command -v curl > /dev/null 2>&1; then
        RELEASE_DATA=$(curl -s "$RELEASE_URL")
    elif command -v wget > /dev/null 2>&1; then
        RELEASE_DATA=$(wget -qO- "$RELEASE_URL")
    else
        print_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi

    if [ -z "$RELEASE_DATA" ]; then
        print_error "Failed to fetch release information"
        exit 1
    fi

    VERSION=$(echo "$RELEASE_DATA" | grep '"tag_name":' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')

    if [ -z "$VERSION" ]; then
        print_error "Failed to parse release version"
        exit 1
    fi

    print_info "Latest version: $VERSION"

    case "$PLATFORM" in
        linux)
            ASSET_PATTERN="linux.tar.gz"
            ;;
        macos)
            ASSET_PATTERN="macos.zip"
            ;;
        windows)
            ASSET_PATTERN="windows-installer.exe"
            ;;
    esac

    DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep -E "$ASSET_PATTERN" | head -n 1 | sed -E 's/.*"browser_download_url": *"([^"]+)".*/\1/')

    if [ -z "$DOWNLOAD_URL" ]; then
        print_error "Failed to find download URL for $PLATFORM"
        print_warning "Asset pattern: $ASSET_PATTERN"
        exit 1
    fi

    FILENAME=$(basename "$DOWNLOAD_URL")
    print_info "Download URL: $DOWNLOAD_URL"
}

download_file() {
    print_info "Downloading $FILENAME..."

    mkdir -p "$TMP_DIR"

    DOWNLOAD_PATH="$TMP_DIR/$FILENAME"

    if command -v curl > /dev/null 2>&1; then
        curl -L -o "$DOWNLOAD_PATH" "$DOWNLOAD_URL"
    elif command -v wget > /dev/null 2>&1; then
        wget -O "$DOWNLOAD_PATH" "$DOWNLOAD_URL"
    fi

    if [ ! -f "$DOWNLOAD_PATH" ]; then
        print_error "Download failed"
        exit 1
    fi

    print_info "Download completed"
}

install_unix() {
    print_info "Extracting archive..."

    cd "$TMP_DIR"
    tar -xzf "$FILENAME"

    BINARY_PATH=$(find "$TMP_DIR" -name "$BINARY_NAME" -type f | head -n 1)

    if [ -z "$BINARY_PATH" ]; then
        print_error "Binary not found in archive"
        exit 1
    fi

    print_info "Installing $BINARY_NAME to $INSTALL_DIR..."

    if [ ! -d "$INSTALL_DIR" ]; then
        print_warning "$INSTALL_DIR does not exist, using /usr/bin instead"
        INSTALL_DIR="/usr/bin"
    fi

    if [ -w "$INSTALL_DIR" ]; then
        mv "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    else
        print_info "Requires elevated permissions..."
        sudo mv "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi

    if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
        print_info "$BINARY_NAME installed successfully!"
        print_info "Version: $("$INSTALL_DIR/$BINARY_NAME" version 2>/dev/null || echo 'unknown')"
        print_info "Location: $INSTALL_DIR/$BINARY_NAME"
    else
        print_error "Installation verification failed"
        exit 1
    fi
}

install_windows() {
    print_info "Running Windows installer..."

    INSTALLER_PATH="$TMP_DIR/$FILENAME"

    if [ ! -f "$INSTALLER_PATH" ]; then
        print_error "Installer not found: $INSTALLER_PATH"
        exit 1
    fi

    print_info "Installer location: $INSTALLER_PATH"

    if command -v cygpath > /dev/null 2>&1; then
        WIN_PATH=$(cygpath -w "$INSTALLER_PATH")
    else
        WIN_PATH=$(echo "$INSTALLER_PATH" | sed 's|^/\([a-zA-Z]\)/|\1:/|' | sed 's|/|\\|g')
    fi

    print_info "Launching installer..."

    if "$INSTALLER_PATH" 2>/dev/null; then
        print_info "Installer launched successfully."
        print_info "Please follow the installation wizard."
        return 0
    fi

    if cmd.exe /c "\"$WIN_PATH\"" 2>/dev/null; then
        print_info "Installer launched via cmd."
        print_info "Please follow the installation wizard."
        return 0
    fi

    if powershell.exe -Command "& {Start-Process -FilePath '${WIN_PATH}'}" 2>/dev/null; then
        print_info "Installer launched via PowerShell."
        print_info "Please follow the installation wizard."
        return 0
    fi

    if cmd.exe /c "start \"\" \"${WIN_PATH}\"" 2>/dev/null; then
        print_info "Installer launched via start command."
        print_info "Please follow the installation wizard."
        return 0
    fi

    print_error "Failed to launch installer automatically."
    print_info ""
    print_info "Please manually run the installer:"
    print_info "  $WIN_PATH"
    print_info ""
    print_info "Or from Git Bash:"
    print_info "  $INSTALLER_PATH"
}

cleanup() {
    print_info "Cleaning up temporary files..."
    rm -rf "$TMP_DIR"
}

main() {
    print_info "=== Uwais Installation Script ==="

    detect_platform
    get_latest_release_url
    download_file

    case "$PLATFORM" in
        linux|macos)
            install_unix
            cleanup
            ;;
        windows)
            install_windows
            print_warning "Temporary files kept at: $TMP_DIR"
            ;;
    esac

    echo ""
    print_info "=== Installation Complete ==="

    if [ "$PLATFORM" != "windows" ]; then
        echo ""
        print_info "You can now use 'uwais' command from your terminal."
        print_info "Try running: uwais"
    fi
}

main
