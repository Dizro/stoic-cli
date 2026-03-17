#!/bin/sh
# stoic-cli installer
# Usage: curl -fsSL https://raw.githubusercontent.com/Dizro/stoic-cli/main/install.sh | sh

set -e

REPO="Dizro/stoic-cli"
BINARY="stoic"

detect_target() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$OS" in
        linux)
            case "$ARCH" in
                x86_64) echo "x86_64-unknown-linux-gnu" ;;
                aarch64|arm64) echo "aarch64-unknown-linux-gnu" ;;
                *) echo "unsupported" ;;
            esac
            ;;
        darwin)
            case "$ARCH" in
                x86_64) echo "x86_64-apple-darwin" ;;
                arm64|aarch64) echo "aarch64-apple-darwin" ;;
                *) echo "unsupported" ;;
            esac
            ;;
        *) echo "unsupported" ;;
    esac
}

install() {
    TARGET=$(detect_target)
    if [ "$TARGET" = "unsupported" ]; then
        echo "  Unsupported platform: $(uname -s) $(uname -m)"
        exit 1
    fi

    echo "  Installing stoic-cli..."

    # Get latest version
    VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/')

    if [ -z "$VERSION" ]; then
        echo "  Failed to get latest version"
        exit 1
    fi

    echo "  Version: v$VERSION"
    echo "  Target: $TARGET"

    URL="https://github.com/$REPO/releases/download/v${VERSION}/${BINARY}-${TARGET}.tar.gz"

    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT

    curl -fsSL "$URL" -o "${TMPDIR}/stoic.tar.gz"

    # Extract
    tar xzf "${TMPDIR}/stoic.tar.gz" -C "$TMPDIR"

    # Install to ~/.local/bin (XDG standard, no Rust required)
    INSTALL_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
    mkdir -p "$INSTALL_DIR"
    mv "${TMPDIR}/${BINARY}" "$INSTALL_DIR/${BINARY}"
    chmod +x "$INSTALL_DIR/${BINARY}"

    echo ""
    echo "  stoic-cli v${VERSION} installed successfully!"
    echo ""
    echo "  Run 'stoic' to start reading Stoic philosophy."
    echo ""
    # Warn if not in PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *) echo "  ⚠  Add '$INSTALL_DIR' to your PATH if 'stoic' is not found:" ;;
    esac
    echo ""
}

install
