use std::io::{self, prelude::*};

pub fn prompt(mode: &str) -> String {
    let mut input = String::new();

    print!("{})> ", mode);

    std::io::stdout().flush().unwrap(); 

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
