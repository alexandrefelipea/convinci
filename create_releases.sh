#!/bin/bash

# Stop on the first error
set -e

VERSION=$(grep 'version' Cargo.toml | head -1 | awk -F '"' '{print $2}')
TARGETS=("x86_64-unknown-linux-musl" "x86_64-pc-windows-gnu")

mkdir -p releases

for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    cargo build --release --target "$target"

    BIN_PATH="target/$target/release/"

    if [[ "$target" == *"windows"* ]]; then
        BIN_NAME="convinci.exe"
        ARCHIVE="convinci-v${VERSION}-${target}.zip"
    else
        BIN_NAME="convinci"
        ARCHIVE="convinci-v${VERSION}-${target}.tar.gz"
    fi

    # Check if the binary exists
    if [ ! -f "${BIN_PATH}${BIN_NAME}" ]; then
        echo "Error: Binary not found at ${BIN_PATH}${BIN_NAME}"
        exit 1
    fi

    cp "${BIN_PATH}${BIN_NAME}" .

    if [[ "$target" == *"windows"* ]]; then
        zip -9 "$ARCHIVE" "$BIN_NAME"
    else
        tar czvf "$ARCHIVE" "$BIN_NAME"
    fi

    rm "$BIN_NAME"
    mv "$ARCHIVE" releases/
done

echo "Releases created in the releases/ folder"