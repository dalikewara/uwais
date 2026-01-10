use std::env::args as env_args;
use std::io::{stdin, stdout, Write};

use crate::print::print_input;

#[inline]
pub fn read_args() -> Vec<String> {
    env_args().collect()
}

pub fn read_input(prompt: &str) -> String {
    if prompt.contains('\0') {
        eprintln!("Warning: Prompt contains null bytes, using empty prompt");
        print_input("");
    } else {
        print_input(prompt);
    }

    if let Err(e) = stdout().flush() {
        eprintln!("Warning: Failed to flush stdout: {}", e);
        return String::new();
    }

    let mut input = String::with_capacity(128);

    match stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            eprintln!("Error: Failed to read from stdin: {}", e);
            String::new()
        }
    }
}
