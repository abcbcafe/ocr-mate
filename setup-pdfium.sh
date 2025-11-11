#!/bin/bash
# Setup script to download PDFium library for OCR-Mate

set -e

echo "Setting up PDFium library for OCR-Mate..."

# Detect platform
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux-x64"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if [[ $(uname -m) == "arm64" ]]; then
        PLATFORM="mac-arm64"
    else
        PLATFORM="mac-x64"
    fi
else
    echo "Unsupported platform: $OSTYPE"
    exit 1
fi

# PDFium version (chromium 6666)
VERSION="chromium%2F6666"
URL="https://github.com/bblanchon/pdfium-binaries/releases/download/${VERSION}/pdfium-${PLATFORM}.tgz"

echo "Downloading PDFium for $PLATFORM..."
curl -L -o pdfium.tgz "$URL"

echo "Extracting PDFium library..."
tar -xzf pdfium.tgz

# Copy library to project root
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    cp lib/libpdfium.so ./
    echo "PDFium library installed: libpdfium.so"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    cp lib/libpdfium.dylib ./
    echo "PDFium library installed: libpdfium.dylib"
fi

# Clean up
rm -rf pdfium.tgz lib include LICENSE PDFium VERSION VERSION.txt 2>/dev/null || true

echo "✓ PDFium setup complete!"
echo ""
echo "You can now build and run OCR-Mate:"
echo "  cargo build --release"
echo "  cargo run --release"
