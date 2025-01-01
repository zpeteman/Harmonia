#!/bin/bash

# Step 1: Check if Rust is installed
if ! command -v rustc &>/dev/null; then
  echo "Rust is not installed. Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  echo "Rust has been installed. Please restart your terminal or run 'source $HOME/.cargo/env' to use Rust."
  source "$HOME/.cargo/env"
else
  echo "Rust is already installed."
fi

# Step 2: Build the app
echo "Building the application..."
cargo build --release

# Step 3: Copy the binary to /usr/local/bin (or another PATH directory)
echo "Installing the binary..."
sudo cp target/release/hr /usr/local/bin/

# Step 4: Create the Harmonia directory
MUSIC_DIR="$HOME/Music/Harmonia"
if [ ! -d "$MUSIC_DIR" ]; then
  echo "Creating the directory '$MUSIC_DIR'..."
  mkdir -p "$MUSIC_DIR"
  echo "Directory '$MUSIC_DIR' has been created. Please place your music files here."
else
  echo "Directory '$MUSIC_DIR' already exists."
fi

# Step 5: Success message
echo "Installation completed successfully! You can now use the 'hr' command to play your music."
