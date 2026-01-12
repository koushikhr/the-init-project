#!/bin/bash

# 1. Configuration
REPO="koushikhr/the-init-project"
# Use a temporary directory for a "Run Once" experience
INSTALL_DIR=$(mktemp -d -t init_project.XXXXXX)
CONFIG_URL="https://raw.githubusercontent.com/$REPO/master/apps.toml"

# Ensure cleanup on exit
trap 'rm -rf "$INSTALL_DIR"' EXIT

echo "🚀 Initializing The Init Project..."

# 2. Get the Latest Release Tag from GitHub API
echo "🔍 Checking for latest version..."
LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "❌ Error: Could not find latest release. Have you pushed a tag yet?"
    exit 1
fi

echo "⬇️  Downloading version: $LATEST_TAG"
BIN_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/init_app"

# 3. Setup Directory
mkdir -p "$INSTALL_DIR/icons"
cd "$INSTALL_DIR" || exit

# 4. Download Binary
curl -fsSL -o init_app "$BIN_URL"
chmod +x init_app

# 5. Download Config & Icons
echo "⬇️  Downloading Configuration & Assets..."
curl -fsSL -o apps.toml "$CONFIG_URL"

# Download Icons (Hardcoded for now)
ICONS_BASE="https://raw.githubusercontent.com/$REPO/master/icons"
curl -fsSL -o "icons/default.svg" "$ICONS_BASE/default.svg"
curl -fsSL -o "icons/firefox.svg" "$ICONS_BASE/firefox.svg"
curl -fsSL -o "icons/vscode.svg" "$ICONS_BASE/vscode.svg"
curl -fsSL -o "icons/vlc.svg" "$ICONS_BASE/vlc.svg"
curl -fsSL -o "icons/discord.svg" "$ICONS_BASE/discord.svg"

echo ""
echo "✅ Ready! Launching..."
"$INSTALL_DIR/init_app"
