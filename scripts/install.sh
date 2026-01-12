#!/bin/sh
set -e

# 1. Configuration
REPO="koushikhr/the-init-project"
VERSION="v0.1.0"
INSTALL_DIR="$HOME/.local/bin/init_project"
BIN_URL="https://github.com/$REPO/releases/download/$VERSION/init_app"
CONFIG_URL="https://raw.githubusercontent.com/$REPO/master/apps.toml"

echo "🚀 Installing Init Project..."

# 2. Setup Directory
mkdir -p "$INSTALL_DIR/icons"
cd "$INSTALL_DIR" || exit

# 3. Download Binary
echo "⬇️ Downloading Binary..."
curl -fsSL -o init_app "$BIN_URL"
chmod +x init_app

# 4. Download Config & Icons (Temporary workaround until Remote Manifests)
echo "⬇️ Downloading Config..."
curl -fsSL -o apps.toml "$CONFIG_URL"
# (Optional: You would loop here to download icons if needed,
# or zip them in the release. For now, we assume default icon)

# 5. Add to Path (Optional, or just tell user where it is)
echo ""
echo "✅ Installation Complete!"
echo "Run it with: $INSTALL_DIR/init_app"
