#!/bin/sh
set -e

# --- CONFIGURATION ---
REPO="koushikhr/the-init-project"
# FIX: Updated filename to 'the-init-project-linux.tar.gz'
DOWNLOAD_URL="https://github.com/$REPO/releases/download/v0.1.0/the-init-project-linux.tar.gz"
# ---------------------

echo "🚀 Initializing Setup..."

# 1. Create a temporary directory safely
TEMP_DIR=$(mktemp -d)

# 2. Download and Extract
echo "⬇️  Downloading from GitHub..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" | tar -xz -C "$TEMP_DIR"
elif command -v wget >/dev/null 2>&1; then
    wget -qO- "$DOWNLOAD_URL" | tar -xz -C "$TEMP_DIR"
else
    echo "Error: Need curl or wget installed."
    exit 1
fi

# 3. Run the App
echo "✅ Starting Init..."
(cd "$TEMP_DIR" && ./init_app)

# 4. Cleanup (Optional)
# rm -rf "$TEMP_DIR"
