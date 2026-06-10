# How to compile

## Portable Mode (Build from Source without installation)

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

Others coming soon!
