use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;

#[derive(Parser, Debug)]
#[command(name = "enigma-rs")]
#[command(author, version, propagate_version = true)]
#[command(about = "A toy project to implement the Enigma Cipher in Rust.")]
pub struct EnigmaCli {
    #[arg(
        short,
        long,
        value_name = "FILENAME",
        default_value = "config/Rotors.toml"
    )]
    pub rotors: PathBuf,

    #[arg(
        short,
        long,
        value_name = "FILENAME",
        default_value = "config/Enigma.toml"
    )]
    pub config: PathBuf,

    #[command(subcommand)]
    pub commands: EnigmaCommands,
}

#[derive(Debug, Subcommand)]
pub enum EnigmaCommands {
    Encrypt {
        input_string: MaybeStdin<String>,
    },
    Decrypt {
        input_string: MaybeStdin<String>,
    },
    Rotors {
        #[arg(short, long)]
        detail: bool,
    },
    Default {
        #[arg(value_name = "FOLDER")]
        folder: PathBuf,
    },
}
