# Flex

![App Icon](assets/icons/icon.png)

Flex is a simple Rust project designed to display ASCII art from `.ascii` files.

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

1. Build the project:

    ```bash
    cargo build --release
    ```

2. Run the program with an `.ascii` file:

    ```bash
    ./target/release/flex [option] 
    # or 
    ./target/release/flex <path to the .ascii file>
    ```

    wait until the program and enter the path of the .ascii file

## Code of Respect

[Code of conduct](code-of-conduct.md)
