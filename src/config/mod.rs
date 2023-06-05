pub mod enigma;
pub mod rotors;


pub trait EnigmaTrait {
    fn mutate(&mut self, input_char: char, backwards: bool) -> char;
}