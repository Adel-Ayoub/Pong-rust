# Pong-Rust

## A Classic Pong Game Implementation in Rust

A modern implementation of the classic Pong arcade game built with Rust and the Tetra framework.

> **Work in Progress** - This project is currently under active development.

## How to Install pong-rust

### From Source

You can clone this repository using the git CLI.
After that, assuming a **Rust with Cargo** setup, you can use commands like:

| Command | Arguments | Description |
| ------- | --------- | ----------- |
| **`cargo run`** | none | Compiles and runs the game |
| **`cargo build --release`** | none | Builds optimized version |
| **`cargo test`** | none | Runs unit tests |
| **`cargo clean`** | none | Removes build artifacts |

```bash
git clone https://github.com/Adel-Ayoub/Pong-rust.git
cd Pong-rust
cargo run
```

## Controls

| Player | Action | Key |
| ------ | ------ | --- |
| **Player 1** | Move Up/Down | W/S |
| **Player 2** | Move Up/Down | Up/Down Arrow |
| **General** | Exit Game | ESC |

## Project Structure

```
src/
├── main.rs              # Application entry point
├── gamestate.rs         # Game state and main loop
└── entities/
    ├── mod.rs           # Entity module
    ├── ball.rs          # Ball physics and collision
    └── player.rs        # Paddle movement logic
```

## Features

| Feature | Implementation |
| ------- | -------------- |
| **Ball Physics** | Vector-based movement with collision detection |
| **Player Input** | Real-time keyboard handling |
| **Rendering** | 60 FPS sprite rendering via Tetra framework |
| **Cross-platform** | Windows, macOS, Linux support |

## Technical Stack

- **Rust** - Systems programming language
- **Tetra** - 2D game framework
- **SDL2** - Cross-platform multimedia (via Tetra)
- **OpenGL** - Graphics rendering (via Tetra)

## Known Issues

> None currently known.
