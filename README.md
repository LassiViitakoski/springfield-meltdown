# Springfield Meltdown

Real-time action roguelite where you navigate mutated Springfield, rescue trapped citizens, and unlock new playable characters. Built in Rust with ggez.

## Core Gameplay

- **Rescue Missions:** Navigate contaminated zones, rescue citizens, unlock them as playable characters
- **Resource Management:** Dual currency system (cash/donuts), weapon loadouts, ammunition scarcity
- **Permadeath:** Strategic pre-mission planning with armor, buffs, and weapon selection
- **Progression:** Permanent weapon purchases, character unlocks, donut-based meta upgrades

## Build Instructions

### Prerequisites

- Rust 1.70 or newer (install from [rust lang.org](https://www.rustlang.org/))
- OpenGL 3.2+ compatible graphics drivers

### Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd springfield-meltdown

# Build and run (debug mode)
cargo run

# Build optimized release version
cargo build --release

# Run release version
./target/release/springfield-meltdown
```

### Development

```bash
# Check for errors without building
cargo check

# Run with optimizations for faster debug builds
cargo run --release
```

## Current Status

**Phase 1 - Combat Prototype Foundation (In Progress)**

Currently implementing Story SPRING-001-1.1: Project setup complete with window rendering at 800x600 and 60 FPS game loop.
