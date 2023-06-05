# enigma-rs

A toy project to implement the Enigma Machine in Rust.

This is a Rust project. Please install Rust before trying to run this project. Visit https://rustup.rs/ to get started.

## Installation

This project is not ready to be installed into your environment. Although you can, it is advised against. This project currently uses the repository root as its running environment, and has not yet been updated to utilise a config directory in your `~` dir or something.

To run enigma:

1. Clone from GitHub:

```sh
$ git clone https://github.com/MattLimb/enigma-rs.git 
```

2. Change Directory into `enigma-rs`:

```sh
$ cd enigma-rs
```

3. Run through Cargo:

```sh
$ cargo run -- encrypt "Hello World!"
```

## Planned Features

- Variable Number Rotors (more or less than 3). Its currently locked at 3.
- Rotor Offsets

## Example Usage

```
$ cargo run -- help
A toy project to implement the Enigma Cipher in Rust.

Usage: enigma.exe [OPTIONS] <COMMAND>

Commands:
  encrypt
  decrypt
  rotors
  help     Print this message or the help of the given subcommand(s)

Options:
  -r, --rotors <FILENAME>  [default: config/Rotors.toml]
  -c, --config <FILENAME>  [default: config/Enigma.toml]
  -h, --help               Print help
  -V, --version            Print version
```

```
$ cargo run -- --rotors config/Rotors.toml --config config/Enigma.toml encrypt "Hello World"
gwxnw ybehj
```

```
$ cargo run -- --rotors config/Rotors.toml --config config/Enigma.toml decrypt "gwxnw ybehj"
hello world
```

```
$ cargo run -- rotors
-------------------------------------------------------------------------------------------------------------------------------------------
Rotor  : rotor_iii (in use)
-------------------------------------------------------------------------------------------------------------------------------------------
Rotor  : rotor_ii (in use)
-------------------------------------------------------------------------------------------------------------------------------------------
Rotor  : rotor_i (in use)
-------------------------------------------------------------------------------------------------------------------------------------------
```