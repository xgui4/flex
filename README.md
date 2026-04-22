# Flex

![App Icon](assets/icons/icon.png)

Flex-rs is a simple Rust project designed to display ASCII art from `.ascii` files.

## Features

- Reads ASCII art from files with the `.ascii` extension
- Prints the ASCII art to the terminal

## Compatible Operating System

### Technical Requirement

- A terminal shell
- Color Escape support higly recommended
- A .ascii file

### Official Packages/Installer
- Windows (Inno Setup)
- Arch Based distro (PKGBUILD file)

### Tier 2 (Tested, no official package yet)
- FreeBSD
- GNU/Linux (App Image)

### Tier 3 (Not tested, but should work, no official package planned)
- MacOS (could, but unsupported as I do not own a Mac)
- Any OS in UNIX Spectrum that the Rust toolchain support

## Crate Used : 

- [Rust-i18n](https://github.com/longbridge/rust-i18n)
- windows_exe_info

## Usage

# Portable Mode (Build from Source without installation)

1. Build the project:

```bash
cargo build --release
```

2. Run the program with an `.ascii` file:

```bash
./target/release/flex-rs [option]

./target/release/flex-rs "assets/examples/infinity.ascii" # this is a example, replace the file name with the desired .ascii file to show

./target/release/flex-rs --version # ./target/release/flex-rs --v

./target/release/flex-rs --license # ./target/release/flex-rs --l

./target/release/flex-rs --about # ./target/release/flex-rs --a

./target/release/flex-rs --color-code # ./target/release/flex-rs -c

./target/release/flex-rs --help # ./target/release/flex-rs --h for more information about other commands
```

wait until the program and enter the path of the .ascii file

## Full Installaion

### Arch (Git)

```bash

./install aur

```

### Arch (Stable)

```bash

./install aur-stable

```

Others comming soon!

## License

This project is licensed under the GNU General Public License v3.0 License. See the [LICENSE](LICENSE) file for details.

## Code of Respect

[Code of Conduct](code-of-conduct.md)