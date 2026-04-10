# LOTM Build Guide

## Quick Start

### Build on Linux/macOS
```bash
chmod +x build.sh
./build.sh
```

### Build on Windows
```powershell
cargo build --release
```

## Platform-Specific Instructions

### Linux

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install -y build-essential pkg-config libssl-dev

# Build
cargo build --release

# Run client
./target/release/lotm-client

# Run server
./target/release/lotm-server
```

### Windows

```powershell
# Install Visual Studio Build Tools
# https://visualstudio.microsoft.com/visual-cpp-build-tools/

# Build
cargo build --release

# Run
.\target\release\lotm-client.exe
.\target\release\lotm-server.exe
```

### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Build
cargo build --release

# Run
./target/release/lotm-client
```

### Android

```bash
# Install Android NDK
# https://developer.android.com/ndk/downloads

# Add Android target
rustup target add aarch64-linux-android

# Set up linker
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[target.aarch64-linux-android]
linker = "$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
ar = "$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
EOF

# Build
cargo build --target aarch64-linux-android --release
```

### Raspberry Pi (ARM64)

```bash
# Add ARM64 target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compiler
sudo apt-get install -y gcc-aarch64-linux-gnu

# Set up linker
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

# Build
cargo build --target aarch64-unknown-linux-gnu --release
```

## Build Profiles

### Debug (fast compilation, slow runtime)
```bash
cargo build
```

### Release (slow compilation, fast runtime)
```bash
cargo build --release
```

### With optimizations
```bash
cargo build --profile release
```

## Troubleshooting

### Out of memory during build
```bash
# Reduce parallel compilation
cargo build -j 2
```

### Missing dependencies
```bash
# Ubuntu/Debian
sudo apt-get install -y build-essential pkg-config libssl-dev

# Fedora
sudo dnf install -y gcc pkg-config openssl-devel

# Arch Linux
sudo pacman -S base-devel pkg-config openssl
```

### Android build fails
```bash
# Check NDK installation
echo $ANDROID_NDK

# Verify target is installed
rustup target list --installed | grep android
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **CI Workflow**: Builds on every push and PR
  - Linux, Windows, macOS
  - Android (aarch64)
  - ARM64 (Linux)
  - Code quality checks (fmt, clippy)
  - Tests and coverage

- **Release Workflow**: Creates releases on tag push
  - Automatic binaries for all platforms
  - Release notes generation
  - Multi-platform packaging

To trigger a release:
```bash
git tag v0.1.0
git push origin v0.1.0
```
