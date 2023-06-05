use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use crate::rotors::{Rotor, RotorsConfig};

pub trait EnigmaTrait {
    fn mutate(&mut self, input_char: char, backwards: bool) -> char;
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnigmaMachine {
    pub rotors: [String; 3],
    plugboard: EnigmaPlugboard,

    // Right (0) - Middle (1) - Left (2) - Reflector (3)
    #[serde(skip, default)]
    rotor_slots: Vec<Rotor>,

    #[serde(default)]
    left_rotated: i8,
    #[serde(default)]
    middle_rotated: i8,
    #[serde(default)]
    right_rotated: i8,
}

impl EnigmaTrait for EnigmaMachine {
    fn mutate(&mut self, input_char: char, _backwards: bool) -> char {
        self.rotate();

        // Send through plugboard
        let mut output: char = self.plugboard.mutate(input_char, false);

        // Send through the Rotors
        // Right (2) -> Middle (1) -> Left (0)
        output = self.rotor_slots[0].mutate(output, false);
        output = self.rotor_slots[1].mutate(output, false);
        output = self.rotor_slots[2].mutate(output, false);

        // Send through the Reflector
        output = self.rotor_slots[3].mutate(output, false);

        // Send through the Rotors again
        // Left (0) -> Middle (1) -> Right (2)
        output = self.rotor_slots[2].mutate(output, true);
        output = self.rotor_slots[1].mutate(output, true);
        output = self.rotor_slots[0].mutate(output, true);

        // Send through Plugboard
        self.plugboard.mutate(output, false)
    }
}

impl EnigmaMachine {
    pub fn init(&mut self, all_rotors: RotorsConfig) {
        self.rotor_slots = vec![
            all_rotors.clone().get_rotor(&self.rotors[0]),
            all_rotors.clone().get_rotor(&self.rotors[1]),
            all_rotors.get_rotor(&self.rotors[2]),
            Rotor::get_reflector(),
        ];

        self.plugboard.init();
    }

    pub fn load(path: PathBuf) -> Self {
        let file_content: String = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(error) => {
                println!("ERROR: {:?}", error.to_string());
                exit(1)
            }
        };

        match toml::from_str(&file_content) {
            Ok(content) => content,
            Err(error) => {
                println!("ERROR: {:?}", error.to_string());
                exit(1)
            }
        }
    }

    fn rotate_right(&mut self) {
        if self.right_rotated == 26 {
            self.rotate_middle();
            self.right_rotated = -1;
        }

        self.rotor_slots[0].rotate();
    }

    fn rotate_middle(&mut self) {
        if self.middle_rotated == 26 {
            self.rotate_left();
            self.middle_rotated = -1;
        }

        self.rotor_slots[1].rotate();
    }

    fn rotate_left(&mut self) {
        self.rotor_slots[1].rotate();
        self.left_rotated += 1;
    }

    pub fn rotate(&mut self) {
        self.rotate_right();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnigmaPlugboard(HashMap<char, char>);

impl EnigmaPlugboard {
    pub fn init(&mut self) {
        for (key, value) in self.0.clone().iter() {
            self.0.insert(*value, *key);
        }
    }
}

impl EnigmaTrait for EnigmaPlugboard {
    fn mutate(&mut self, input_char: char, _backwards: bool) -> char {
        match self.0.get(&input_char) {
            Some(ch) => *ch,
            None => input_char,
        }
    }
}
