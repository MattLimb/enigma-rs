mod cli;
mod defaults;
mod enigma;
mod rotors;

use std::path::PathBuf;

use clap::Parser;
use cli::{EnigmaCli, EnigmaCommands};
use enigma::EnigmaTrait;

const DEFAULT_PATH: &'static str = "__default__";

fn default_folder() -> PathBuf {
    match home::home_dir() {
        Some(mut dir) => {
            dir.push("enigma");
            dir
        },
        None => PathBuf::from("./config")
    }
}

fn encrypt(input_string: String, mut rotors: PathBuf, mut config: PathBuf) {
    let default = PathBuf::from(DEFAULT_PATH);

    if rotors == default {
        rotors = default_folder();
        rotors.push("Rotors.toml");
    }

    if config == default {
        config = default_folder();
        config.push("Enigma.toml");
    }

    let rotor_config = rotors::RotorsConfig::load(rotors);
    let mut enigma_config = enigma::EnigmaMachine::load(config);
    enigma_config.init(rotor_config);

    let mut output_string = String::new();

    let banned_chars: [char; 33] = [
        ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';',
        '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ];

    for c in input_string.to_lowercase().chars() {
        if banned_chars.contains(&c) {
            output_string.push(c);
        } else {
            output_string.push(enigma_config.mutate(c, false));
        }
    }

    println!("{}", output_string)
}

fn list_rotors(mut rotors: PathBuf, mut config: PathBuf, detail: bool) {
    let default = PathBuf::from(DEFAULT_PATH);

    if rotors == default {
        rotors = default_folder();
        rotors.push("Rotors.toml");
    }

    if config == default {
        config = default_folder();
        config.push("Enigma.toml");
    }

    let rotor_config = rotors::RotorsConfig::load(rotors);
    let mut enigma_config = enigma::EnigmaMachine::load(config);
    enigma_config.init(rotor_config.clone());

    for (idx, (name, rotor)) in rotor_config.0.iter().enumerate() {
        let mut nameplate: String = format!("Rotor  : {}", name);

        if enigma_config.rotors.contains(name) {
            nameplate = format!("{} (in use)", nameplate);
        }

        println!("{:->139}\n{}", "-", nameplate);

        if detail {
            println!("Input  : {:?}", rotor.input);
            println!("Output : {:?}", rotor.output);
        }

        if (idx + 1) == rotor_config.0.len() {
            println!("{:->139}", "-");
        }
    }
}

fn write_defaults(mut folder: PathBuf) {
    if folder == PathBuf::from(DEFAULT_PATH) {
        folder = default_folder();
    }

    defaults::write_default_rotors(folder.clone());
    println!("Written Rotors.toml");

    defaults::write_default_enigma(folder);
    println!("Written Enigma.toml");
}

fn main() {
    let cli_args: EnigmaCli = EnigmaCli::parse();

    match cli_args.commands {
        EnigmaCommands::Encrypt { input_string } => {
            encrypt(input_string.into_inner(), cli_args.rotors, cli_args.config)
        }
        EnigmaCommands::Decrypt { input_string } => {
            encrypt(input_string.into_inner(), cli_args.rotors, cli_args.config)
        }
        EnigmaCommands::Rotors { detail } => list_rotors(cli_args.rotors, cli_args.config, detail),
        EnigmaCommands::Default { folder } => write_defaults(folder),
    }
}
