# Game Boy Emulator #237

*Because the world needs another one.*

Built in Rust. No specific goal beside learning how to do it.

## Project Structure
This repo is a Cargo workspace with two packages:

### gbcore

Includes the CPU emulation core, memory, and registers. Provides an API for some frontend, including a framebuffer to diplay.

### app

The frontend to display the graphics, handle user inputs, and load ROMS.

## Getting Started

1. Install Rust (using [Rustup](https://www.rust-lang.org/learn/get-started) is preferred)
2. Clone this repo
3. Build with `cargo build`
4. Build docs with `cargo doc`
