# Flex

![App Icon](assets/icons/icon.png)

Flex-rs is a simple Rust project designed to display ASCII art from `.ascii` files.

## Features

- Reads ASCII art from files with the `.ascii` extension
- Prints the ASCII art to the terminal

## Compatible Operating System

- Windows
- GNU/Linux
- BSD (not tested yet)
- MacOS (could, but unsupported as I do not own a Mac)
- Any OS in UNIX Spectrum (not tested yet)

## Usage

This is how you use it in portable mode, installation version guide coming later

1. Build the project:

```bash
cargo build --release
```

2. Run the program with an `.ascii` file:

```bash
./target/release/flex-rs [option]

./target/release/flex-rs <path to the .ascii file>

./target/release/flex-rs --version # ./target/release/flex-rs --v

./target/release/flex-rs --license # ./target/release/flex-rs --l

./target/release/flex-rs --copyright # ./target/release/flex-rs --c

./target/release/flex-rs --about # ./target/release/flex-rs --a

./target/release/flex-rs --color-code # ./target/release/flex-rs --cc

./target/release/flex-rs --help # ./target/release/flex-rs --h for more information about other commands
```

wait until the program and enter the path of the .ascii file

## Code of Respect

[Code of Conduct](code-of-conduct.md)