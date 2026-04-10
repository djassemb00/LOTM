# LOTM - Lord of the Mysteries

<div align="center">

![LOTM Logo](https://img.shields.io/badge/LOTM-Lord%20of%20the%20Mysteries-purple)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange)
![License](https://img.shields.io/badge/License-GPL--3.0-blue)
![Status](https://img.shields.io/badge/Status-Development-red)

**An open-world RPG based on the novel "Lord of the Mysteries"**

[Features](#features) • [Architecture](#architecture) • [Building](#building) • [Contributing](#contributing)

</div>

---

## 📖 About

LOTM is a 3D open-world RPG set in the Victorian-era universe of "Lord of the Mysteries" by Cuttlefish That Loves Diving (爱潜水的乌贼).

Experience the world of:
- 🏭 **Industrial Revolution** - Victorian-era cities with factories and fog
- 🔮 **Beyonder Pathways** - 9 divine pathways with 9 sequences each
- 🌫️ **Corruption & Fog** - Dangerous corrupted zones
- ⚔️ **Sealed Artifacts** - Powerful items with curses
- 🌍 **Multiple Worlds** - Main world + Forsaken Land of the Gods

## ✨ Features

### World Generation
- ✅ Procedural terrain generation (adapted from Veloren)
- ✅ Victorian cities (Tingen, Backlund, Bayam)
- ✅ Corruption/fog zones
- ✅ Underground caves and crypts
- ✅ Churches and sealed artifact vaults

### Gameplay
- 🎮 Multiple camera modes (First Person, Third Person, Top-Down, Free)
- 🌐 Multiplayer support (QUIC protocol)
- 📱 Android support (via winit + wgpu)
- 🎯 Beyonder pathway system

### Technical
- 🦀 Written in Rust for performance
- 🎨 wgpu rendering (Vulkan/Metal)
- 🏗️ ECS architecture (specs)
- 📦 Modular crate structure

## 🏗️ Architecture

```
LOTM/
├── lotm-common/      # Shared types and utilities
├── lotm-world/       # Procedural world generation
│   ├── sim/          # Terrain simulation
│   ├── layer/        # World layers (corruption, structures, etc.)
│   ├── civ/          # Civilization/city generation
│   └── util/         # Utility functions
├── lotm-client/      # Game client
│   ├── game.rs       # Main game loop
│   ├── camera.rs     # Camera system
│   ├── input.rs      # Input handling
│   └── renderer.rs   # wgpu rendering
├── lotm-server/      # Multiplayer server
│   ├── server.rs     # Server state
│   ├── network.rs    # QUIC networking
│   └── world_manager.rs  # World management
├── lotm-engine/      # Core game engine
│   ├── ecs.rs        # ECS integration
│   ├── events.rs     # Event system
│   └── time.rs       # Time management
└── lotm-assets/      # Game assets
```

## 🚀 Building

### Prerequisites
- Rust 1.75+
- Vulkan SDK (for rendering)

### Build Commands

```bash
# Build all
cargo build

# Build client
cargo build --bin lotm-client

# Build server
cargo build --bin lotm-server

# Release build
cargo build --release
```

### Running

```bash
# Run client
cargo run --bin lotm-client

# Run server
cargo run --bin lotm-server

# Run with logging
RUST_LOG=info cargo run --bin lotm-client
```

## 📱 Android Support

```bash
# Install Android NDK
rustup target add aarch64-linux-android

# Build for Android
cargo build --target aarch64-linux-android --bin lotm-client
```

## 🎮 Controls

| Action | Keyboard | Touch |
|--------|----------|-------|
| Move | WASD | Virtual Joystick |
| Camera | Mouse | Swipe |
| Switch Camera | C | Two-finger tap |
| Interact | E | Tap |
| Inventory | I | Swipe down |

## 🌍 World Generation

The world is generated procedurally using noise functions:

```rust
use lotm_world::World;

let opts = lotm_world::sim::WorldOpts::default();
let (world, index) = World::generate(
    12345, // seed
    opts,
    &threadpool,
    &|stage| println!("Stage: {:?}", stage),
);
```

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

### Development Setup

```bash
# Clone repository
git clone https://github.com/djassemb00/LOTM.git
cd LOTM

# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## 📜 License

This project is licensed under the **GNU General Public License v3.0**.

See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- **Cuttlefish That Loves Diving** - Original novel author
- **Veloren** - World generation inspiration and code adaptation
- **Rust Community** - Amazing tools and libraries

## 📞 Contact

- **GitHub**: [djassemb00](https://github.com/djassemb00)
- **Discord**: [Join our server](#)
- **Email**: [Contact us](#)

---

<div align="center">

**"In the name of the Fool, I summon thee!"**

Made with ❤️ by the LOTM Team

</div>
