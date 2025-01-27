#!/bin/bash

if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust and Cargo first." >&2
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Cargo first." >&2
    exit 1
fi

echo "Building the project with Cargo..."
if cargo build --release; then
    echo "Build completed successfully!"
else
    echo "Error: Build failed. Please check the Cargo output for details." >&2
    exit 1
fi

BIN_DEST="/usr/local/bin/waifu-colorscripts"
if sudo cp target/release/waifu-colorscripts "$BIN_DEST"; then
    echo "Installed waifu-colorscripts to $BIN_DEST"
else
    echo "Error: Failed to install waifu-colorscripts to $BIN_DEST" >&2
    exit 1
fi

SHELL_CONFIG=""
if [[ "$SHELL" == *"zsh"* ]]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [[ "$SHELL" == *"bash"* ]]; then
    SHELL_CONFIG="$HOME/.bashrc"
else
    echo "Unsupported shell: $SHELL. Please manually add 'waifu-colorscripts' to your shell's configuration file." >&2
    exit 1
fi

if [[ -f "$SHELL_CONFIG" ]]; then
    if ! grep -q "waifu-colorscripts" "$SHELL_CONFIG"; then
        echo "Adding waifu-colorscripts to $SHELL_CONFIG..."
        echo -e "\n# Run waifu-colorscripts on terminal startup\nwaifu-colorscripts" >> "$SHELL_CONFIG"
        echo "waifu-colorscripts added to $SHELL_CONFIG successfully!"
    else
        echo "waifu-colorscripts is already configured in $SHELL_CONFIG."
    fi
else
    echo "Error: Shell configuration file $SHELL_CONFIG not found." >&2
    exit 1
fi

if [[ "$TERM_PROGRAM" == "iTerm.app" ]]; then
    echo "Detected iTerm. Adding waifu-colorscripts to iTerm profile..."
    if ! grep -q "waifu-colorscripts" "$SHELL_CONFIG"; then
        echo -e "\n# Run waifu-colorscripts on iTerm startup\nwaifu-colorscripts" >> "$SHELL_CONFIG"
        echo "waifu-colorscripts added to iTerm profile successfully!"
    else
        echo "waifu-colorscripts is already configured for iTerm."
    fi
fi

CONF_SOURCE="conf.toml"
IMAGES_SOURCE="images/"
CONF_DEST="$HOME/.local/share/waifu-colorscripts/"
KITTY_CONF="$HOME/.config/kitty/kitty.conf"

mkdir -p "$CONF_DEST"

if cp "$CONF_SOURCE" "$CONF_DEST"; then
    echo "Copied $CONF_SOURCE to $CONF_DEST"
else
    echo "Error: Failed to copy $CONF_SOURCE to $CONF_DEST" >&2
    exit 1
fi

if cp -r "$IMAGES_SOURCE" "$CONF_DEST"; then
    echo "Copied $IMAGES_SOURCE to $CONF_DEST"
else
    echo "Error: Failed to copy $IMAGES_SOURCE to $CONF_DEST" >&2
    exit 1
fi

if [[ ! -f "$KITTY_CONF" ]]; then
    echo "Error: $KITTY_CONF does not exist. Please ensure Kitty is installed and configured." >&2
    exit 1
fi

if grep -q "^allow_remote_control" "$KITTY_CONF"; then
    echo "allow_remote_control is already configured in $KITTY_CONF"
else
    echo "allow_remote_control yes" >> "$KITTY_CONF"
    echo "Added allow_remote_control yes to $KITTY_CONF"
fi

echo "Installation completed successfully!"
