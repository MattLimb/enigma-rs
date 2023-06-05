mod cli;
mod enigma;
mod rotors;

use std::path::PathBuf;

use clap::Parser;
use cli::{EnigmaCli, EnigmaCommands};
use enigma::EnigmaTrait;

fn encrypt(input_string: String, rotors: PathBuf, config: PathBuf) {
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

fn list_rotors(rotors: PathBuf, config: PathBuf, detail: bool) {
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
    }
}
