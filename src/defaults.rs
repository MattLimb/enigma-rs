use crate::{
    enigma::{EnigmaMachine, EnigmaPlugboard},
    rotors::{Rotor, RotorsConfig},
};
use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::PathBuf,
    process::exit,
};

pub fn write_default_rotors(mut path: PathBuf) {
    if !path.exists() {
        match fs::create_dir(&path) {
            Ok(_) => (),
            Err(error) => {
                println!("ERROR: Couldn't create folder: {}", error);
                exit(1)
            }
        }
    }

    path.push("Rotors.toml");

    let default_input: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut rotors_hashmap: HashMap<String, Rotor> = HashMap::new();

    rotors_hashmap.insert(
        "rotor_i".to_string(),
        Rotor {
            input: default_input,
            output: VecDeque::from([
                'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h',
                'x', 'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
            ]),
        },
    );

    rotors_hashmap.insert(
        "rotor_ii".to_string(),
        Rotor {
            input: default_input,
            output: VecDeque::from([
                'a', 'j', 'd', 'k', 's', 'i', 'r', 'u', 'x', 'b', 'l', 'h', 'w', 't', 'm', 'c',
                'q', 'g', 'z', 'n', 'p', 'y', 'f', 'v', 'o', 'e',
            ]),
        },
    );

    rotors_hashmap.insert(
        "rotor_iii".to_string(),
        Rotor {
            input: default_input,
            output: VecDeque::from([
                'b', 'd', 'f', 'h', 'j', 'l', 'c', 'p', 'r', 't', 'x', 'v', 'z', 'n', 'y', 'e',
                'i', 'w', 'g', 'a', 'k', 'm', 'u', 's', 'q', 'o',
            ]),
        },
    );

    RotorsConfig(rotors_hashmap).dump(path);
}

pub fn write_default_enigma(mut path: PathBuf) {
    if !path.exists() {
        match fs::create_dir(&path) {
            Ok(_) => (),
            Err(error) => {
                println!("ERROR: Couldn't create folder: {}", error);
                exit(1)
            }
        }
    }

    path.push("Enigma.toml");

    let enigma = EnigmaMachine {
        rotors: vec![
            "rotor_i".to_string(),
            "rotor_ii".to_string(),
            "rotor_iii".to_string(),
        ],
        plugboard: EnigmaPlugboard(HashMap::new()),
        offset: HashMap::new(),
        rotor_slots: Vec::new(),
        reflector: Rotor::get_reflector(),
        rotated: Vec::new(),
    };

    enigma.dump(path)
}
