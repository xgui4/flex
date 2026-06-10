# Flex

![App Icon](assets/icons/icon.png)

Flex-rs is a simple Rust project designed to display ASCII art from `.ascii` files.

## Features

- Reads ASCII art from files with the `.ascii` extension
- Prints the ASCII art to the terminal

## Compatible Operating System

### Technical Requirement

- A terminal shell
- Color Escape for your terminal support higthly recommended (support for older terminals coming later)

### Official Packages/Installer

- Windows (Inno Setup)
- Arch Based distro (PKGBUILD file)

### Tier 2 (Tested, no official package yet)

- FreeBSD
- GNU/Linux (AppImage)

### Tier 3 (Not tested, but should work, no official package planned)

- MacOS (could, but unsupported as I do not own a Mac)
- Any OS in UNIX Spectrum that the Rust toolchain support

## Crate Used

- [Rust-i18n](https://github.com/longbridge/rust-i18n)
- windows_exe_info

## How to build

see [`COMPILING.md`](COMPILING.MD)

## License

This project is licensed under the GNU General Public License v3.0 License. See the [LICENSE](LICENSE) file for details.

## Code of Respect

[Code of Conduct](code-of-conduct.md)
