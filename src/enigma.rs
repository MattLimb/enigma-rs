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
    pub rotors: Vec<String>,
    offset: HashMap<String, u8>,
    plugboard: EnigmaPlugboard,

    // Right (0) - Middle (1) - Left (2) - Reflector (3)
    #[serde(skip, default)]
    rotor_slots: Vec<Rotor>,

    #[serde(skip, default = "Rotor::get_reflector")]
    reflector: Rotor,

    #[serde(default)]
    rotated: Vec<u8>,
}

impl EnigmaTrait for EnigmaMachine {
    fn mutate(&mut self, input_char: char, _backwards: bool) -> char {
        self.rotate();

        // Send through plugboard
        let mut output: char = self.plugboard.mutate(input_char, false);

        // Send through the Rotors in the Correct Order
        for rtr in self.rotor_slots.iter_mut() {
            output = rtr.mutate(output, false);
        }

        // Send through the Reflector
        output = self.reflector.mutate(output, false);

        // Send through the Rotors again in reverse
        let mut reversed_rotors = self.rotor_slots.clone();
        reversed_rotors.reverse();

        for rtr in reversed_rotors.iter_mut() {
            output = rtr.mutate(output, true);
        }

        // Send through Plugboard and Return
        self.plugboard.mutate(output, false)
    }
}

impl EnigmaMachine {
    pub fn init(&mut self, all_rotors: RotorsConfig) {
        let mut rotor_slots: Vec<Rotor> = vec![];

        for rtr in self.rotors.clone() {
            let mut rotor = all_rotors.clone().get_rotor(&rtr);

            if let Some(offset) = self.offset.get(&rtr) {
                for _int in 0..*offset {
                    rotor.rotate();
                }
            }

            rotor_slots.push(rotor);
            self.rotated.push(0);
        }

        self.rotor_slots = rotor_slots;
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

    pub fn rotate(&mut self) {
        for (idx, rotor) in self.rotor_slots.iter_mut().enumerate() {
            rotor.rotate();
            self.rotated[idx] += 1;

            match self.rotated[idx].cmp(&26) {
                std::cmp::Ordering::Equal => self.rotated[idx] = 0,
                _ => break,
            }
        }
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
