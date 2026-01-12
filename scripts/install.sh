#!/bin/sh
set -e

# --- CONFIGURATION ---
REPO="https://github.com/koushikhr/the-init-project"
# This URL points to where to upload the file in Step 4
DOWNLOAD_URL="https://github.com/koushikhr/the-init-project/releases/download/v0.1.0/init-linux.tar.gz"
# ---------------------

echo "🚀 Initializing Setup..."

# 1. Create a temporary directory safely
TEMP_DIR=$(mktemp -d)

# 2. Download and Extract
# We pipe curl output directly to tar to avoid saving the .tar.gz file to disk
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
# We execute inside the directory so it finds apps.toml and icons/
(cd "$TEMP_DIR" && ./init_app)

# 4. Cleanup (Optional: uncomment to delete after running)
# rm -rf "$TEMP_DIR"
