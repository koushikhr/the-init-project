#!/bin/bash
# Check if pwsh is available
if command -v pwsh &> /dev/null; then
    echo "PowerShell (pwsh) is installed. Testing command syntax..."
    # We are NOT running the actual script because it requires the URL to be live and valid with the new changes.
    # Instead, we are just printing what the user would run to confirm we have the right tool.
    echo "To test the Windows install on Linux (if you have PowerShell Core):"
    echo "pwsh -Command \"iwr -useb https://raw.githubusercontent.com/koushikhr/the-init-project/master/scripts/install.ps1 | iex\""
else
    echo "PowerShell (pwsh) is not found. You can try installing it or use Wine."
    echo "To run with Wine, you typically need to download the script first or use a Windows command prompt within Wine."
fi
