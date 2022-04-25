#[derive(Debug, PartialEq, Clone)]
enum Pulse {
    Short,
    Long,
}
use Pulse::{Long, Short};

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut morse_code: Message = Vec::new();
        for byte in self.bytes() {
            match byte as char {
                'a' | 'A' => morse_code.push(vec![Short, Long]),
                'b' | 'B' => morse_code.push(vec![Long, Short, Short, Short]),
                'c' | 'C' => morse_code.push(vec![Long, Short, Long, Short]),
                'd' | 'D' => morse_code.push(vec![Long, Short, Short]),
                'e' | 'E' => morse_code.push(vec![Short]),
                'f' | 'F' => morse_code.push(vec![Short, Short, Long, Short]),
                'g' | 'G' => morse_code.push(vec![Long, Long, Short]),
                'h' | 'H' => morse_code.push(vec![Short; 4]),
                'i' | 'I' => morse_code.push(vec![Short; 2]),
                'j' | 'J' => morse_code.push(vec![Short, Long, Long, Long]),
                'k' | 'K' => morse_code.push(vec![Long, Short, Long]),
                'l' | 'L' => morse_code.push(vec![Short, Long, Short, Short]),
                'm' | 'M' => morse_code.push(vec![Long; 2]),
                'n' | 'N' => morse_code.push(vec![Long, Short]),
                'o' | 'O' => morse_code.push(vec![Long; 3]),
                'p' | 'P' => morse_code.push(vec![Short, Long, Long, Short]),
                'q' | 'Q' => morse_code.push(vec![Long, Long, Short, Long]),
                'r' | 'R' => morse_code.push(vec![Short, Long, Short]),
                's' | 'S' => morse_code.push(vec![Short; 3]),
                't' | 'T' => morse_code.push(vec![Long]),
                'u' | 'U' => morse_code.push(vec![Short, Short, Long]),
                'v' | 'V' => morse_code.push(vec![Short, Short, Short, Long]),
                'w' | 'W' => morse_code.push(vec![Short, Long, Long]),
                'x' | 'X' => morse_code.push(vec![Long, Short, Short, Long]),
                'y' | 'Y' => morse_code.push(vec![Long, Short, Long, Long]),
                'z' | 'Z' => morse_code.push(vec![Long, Long, Short, Short]),
                '0' => morse_code.push(vec![Long; 5]),
                '1' => morse_code.push(vec![Short, Long, Long, Long, Long]),
                '2' => morse_code.push(vec![Short, Short, Long, Long, Long]),
                '3' => morse_code.push(vec![Short, Short, Short, Long, Long]),
                '4' => morse_code.push(vec![Short, Short, Short, Short, Long]),
                '5' => morse_code.push(vec![Short; 5]),
                '6' => morse_code.push(vec![Long, Short, Short, Short, Short]),
                '7' => morse_code.push(vec![Long, Long, Short, Short, Short]),
                '8' => morse_code.push(vec![Long, Long, Long, Short, Short]),
                '9' => morse_code.push(vec![Long, Long, Long, Long, Short]),
                _ => continue,
            }
        }
        morse_code
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{pulse}");
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
