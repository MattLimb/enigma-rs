use crate::enigma::EnigmaTrait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotorsConfig(pub HashMap<String, Rotor>);

impl RotorsConfig {
    pub fn get_rotor(self, name: &String) -> Rotor {
        match self.0.get(name) {
            Some(rtr) => rtr.clone(),
            None => {
                println!("Rotor Config for {} does not exist.", name);
                exit(1);
            }
        }
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rotor {
    pub input: [char; 26],
    pub output: VecDeque<char>,
}

impl Rotor {
    fn rotor_error() {
        println!("ERROR: Rotor is empty.");
        exit(1);
    }

    pub fn rotate(&mut self) {
        if let Some(tmp_storage) = self.output.pop_front() {
            self.output.push_back(tmp_storage);
        } else {
            Self::rotor_error();
        }
    }

    pub fn get_reflector() -> Self {
        Self {
            input: [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ],
            output: VecDeque::from([
                'e', 'j', 'm', 'z', 'a', 'l', 'y', 'x', 'v', 'b', 'w', 'f', 'c', 'r', 'q', 'u',
                'o', 'n', 't', 's', 'p', 'i', 'k', 'h', 'g', 'd',
            ]),
        }
    }
}

impl EnigmaTrait for Rotor {
    fn mutate(&mut self, input_char: char, backwards: bool) -> char {
        if backwards {
            match self.output.iter().position(|&r| r == input_char) {
                Some(idx) => self.input[idx],
                None => input_char,
            }
        } else {
            match self.input.iter().position(|&r| r == input_char) {
                Some(idx) => self.output[idx],
                None => input_char,
            }
        }
    }
}
