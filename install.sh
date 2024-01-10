#!/bin/bash
OS_TYPE="$(uname)"
echo "Operating System: $OS_TYPE"

if [[ "$OS_TYPE" == "Linux" ]]; then
    FILENAME="airnc-linux.tar.gz"
elif [[ "$OS_TYPE" == "Darwin" ]]; then
    ARCH="$(uname -m)"
    if [[ "$ARCH" == "x86_64" ]]; then
        FILENAME="airnc-macos.tar.gz"
    elif [[ "$ARCH" == "arm64" ]]; then
        FILENAME="airnc-macos-aarch64.tar.gz"
    else
        FILENAME="airnc-macos.tar.gz"
        echo "Unknown macOS Architecture: $ARCH"
    fi
elif [[ "$OS_TYPE" == "Windows_NT" ]]; then
    FILENAME="airnc-windows.7z"
else
    echo "Unknown Operating System: $OS_TYPE"
    exit 1
fi

ARCHIVE_URL="https://github.com/zhazhazhu/airnc/releases/latest/download/$FILENAME"

DOWNLOAD_DIR=$(mktemp -d)

CURRENT_SHELL="$(basename "$SHELL")"

TEMP_FILE="$DOWNLOAD_DIR/airnc.tar.gz"

if [ -d "$HOME/.airnc" ]; then
  INSTALL_DIR="$HOME/.airnc"
elif [ -n "$XDG_DATA_HOME" ]; then
  INSTALL_DIR="$XDG_DATA_HOME/.airnc"
elif [ "$OS" = "Darwin" ]; then
  INSTALL_DIR="$HOME/Library/Application Support/.airnc"
else
  INSTALL_DIR="$HOME/.local/share/.airnc"
fi

echo "CURRENT_SHELL is $CURRENT_SHELL"

echo "INSTALL_DIR is $INSTALL_DIR"

echo "DOWNLOAD_DIR is $DOWNLOAD_DIR"

curl -L "$ARCHIVE_URL" -o "$TEMP_FILE"

tar -xzvf "$TEMP_FILE" -C "$DOWNLOAD_DIR"

if [ ! -d "$INSTALL_DIR" ]; then
  mkdir "$INSTALL_DIR"
fi

mv "$DOWNLOAD_DIR/airnc" "$INSTALL_DIR/airnc"

rm -rf "$DOWNLOAD_DIR"

ensure_containing_dir_exists() {
  local CONTAINING_DIR
  CONTAINING_DIR="$(dirname "$1")"
  if [ ! -d "$CONTAINING_DIR" ]; then
    echo " >> Creating directory $CONTAINING_DIR"
    mkdir -p "$CONTAINING_DIR"
  fi
}

setup_shell() {
  CURRENT_SHELL="$(basename "$SHELL")"

  if [ "$CURRENT_SHELL" = "zsh" ]; then
    CONF_FILE=${ZDOTDIR:-$HOME}/.zshrc
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Zsh. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # airnc'
    echo '  export PATH="'"$INSTALL_DIR"':$PATH"'

    echo '' >>$CONF_FILE
    echo '# airnc' >>$CONF_FILE
    echo 'export PATH="'$INSTALL_DIR':$PATH"' >>$CONF_FILE

  elif [ "$CURRENT_SHELL" = "fish" ]; then
    CONF_FILE=$HOME/.config/fish/conf.d/airnc.fish
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Fish. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # airnc'
    echo '  set PATH "'"$INSTALL_DIR"'" $PATH'
    echo '  airnc env | source'

    echo '# airnc' >>$CONF_FILE
    echo 'set PATH "'"$INSTALL_DIR"'" $PATH' >>$CONF_FILE
    echo 'airnc env | source' >>$CONF_FILE

  elif [ "$CURRENT_SHELL" = "bash" ]; then
    if [ "$OS" = "Darwin" ]; then
      CONF_FILE=$HOME/.profile
    else
      CONF_FILE=$HOME/.bashrc
    fi
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Bash. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # airnc'
    echo '  export PATH="'"$INSTALL_DIR"':$PATH"'

    echo '' >>$CONF_FILE
    echo '# airnc' >>$CONF_FILE
    echo 'export PATH="'"$INSTALL_DIR"':$PATH"' >>$CONF_FILE

  else
    echo "Could not infer shell type. Please set up manually."
    exit 1
  fi

  echo '# airnc end' >>$CONF_FILE
  echo ""
  echo "In order to apply the changes, open a new terminal or run the following command:"
  echo ""
  echo "  source $CONF_FILE"
}

setup_shell

echo "Installation completed successfully."