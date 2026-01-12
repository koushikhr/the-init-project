#!/bin/bash

# 1. Configuration
REPO="koushikhr/the-init-project"
INSTALL_DIR="$HOME/.local/bin/init_project"
CONFIG_URL="https://raw.githubusercontent.com/$REPO/master/apps.toml"

echo "🚀 Installing The Init Project..."

# 2. Get the Latest Release Tag from GitHub API
echo "🔍 Checking for latest version..."
LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "❌ Error: Could not find latest release. Have you pushed a tag yet?"
    exit 1
fi

echo "⬇️  Downloading version: $LATEST_TAG"
BIN_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/init_app"

# 3. Setup Directory (Clear old version to be safe)
mkdir -p "$INSTALL_DIR/icons"
rm -f "$INSTALL_DIR/init_app"
cd "$INSTALL_DIR" || exit

# 4. Download Binary
curl -fsSL -o init_app "$BIN_URL"
chmod +x init_app

# 5. Download Config
echo "⬇️  Downloading Configuration..."
curl -fsSL -o apps.toml "$CONFIG_URL"

echo ""
echo "✅ Installation Complete ($LATEST_TAG)!"
echo "Run it with: $INSTALL_DIR/init_app"
