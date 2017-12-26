use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::default::Default;

#[derive(Default)]
struct State {
    result: String,
    mode : Mode,
}

enum Mode {
    Uppercase,
    Lowercase,
    Punctuation,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Uppercase
    }
}

impl Mode {
    fn next(&mut self) {
        *self = match *self {
            Mode::Uppercase => {
                Mode::Lowercase
            },
            Mode::Lowercase => {
                Mode::Punctuation
            },
            Mode::Punctuation => {
                Mode::Uppercase
            },
        }
    }
}

static UPPERCASE : [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',];
static LOWERCASE : [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',];
static PUNCTUATION : [char; 8] = ['!', '?', ',', '.', ' ', ';', '"', '\''];

fn decode(state: &mut State, byte: u8) {
    let possible_char = match state.mode {
        Mode::Uppercase => {
            let modulus = byte % 27;
            if modulus == 0 {
                None
            } else {
                Some(UPPERCASE[(modulus - 1) as usize])
            }
        },
        Mode::Lowercase => {
            let modulus = byte % 27;
            if modulus == 0 {
                None
            } else {
                Some(LOWERCASE[(modulus - 1) as usize])
            }
        },
        Mode::Punctuation => {
            let modulus = byte % 9;
            if modulus == 0 {
                None
            } else {
                Some(PUNCTUATION[(modulus - 1) as usize])
            }
        },
    };

    if let Some(c) = possible_char {
        state.result.push(c);
    } else {
        state.mode.next();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut invaild_args = true;

    if let Some(filename) = args.get(1).map(Path::new) {
        if let Ok(file) = File::open(filename) {
            invaild_args = false;

            let mut state = State::default();

            for byte_result in file.bytes() {
                match byte_result {
                    Ok(byte) => {
                        decode(&mut state, byte);
                    },
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                }
            }

            println!("{}", state.result);
        }
    }

    if invaild_args {
        println!("usage: tlap_decode filename");
    }
}
