#!/bin/bash
# LOTM Build Script
# This script builds LOTM for all supported platforms

set -e

echo "========================================="
echo "  LOTM - Lord of the Mysteries"
echo "  Build Script"
echo "========================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Install Rust from: https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}Rust version:${NC} $(rustc --version)"
echo -e "${GREEN}Cargo version:${NC} $(cargo --version)"
echo ""

# Build for current platform
echo -e "${YELLOW}Building for current platform...${NC}"
cargo build --release

echo ""
echo -e "${GREEN}Build successful!${NC}"
echo ""

# Build for Android if target is installed
if rustup target list --installed | grep -q "aarch64-linux-android"; then
    echo -e "${YELLOW}Building for Android (aarch64)...${NC}"
    cargo build --target aarch64-linux-android --release
    echo -e "${GREEN}Android build successful!${NC}"
else
    echo -e "${YELLOW}Android target not installed. Skipping Android build.${NC}"
    echo "To add Android target: rustup target add aarch64-linux-android"
fi

echo ""
echo "========================================="
echo "  Build Complete!"
echo "========================================="
echo ""
echo "Binaries location:"
echo "  Client: target/release/lotm-client"
echo "  Server: target/release/lotm-server"
echo ""
