use crate::{interpreter, BoxError};
use std::io::{self, Write};

pub struct REPL {
    commands: Vec<String>,
}

impl REPL {
    pub fn new() -> REPL {
        REPL { commands: vec![] }
    }

    pub fn run(&mut self) -> Result<(), BoxError> {
        println!("Welcome to the Iodine REPL!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">> ");
            io::stdout().flush().expect("Unable to flush stdout!");
            stdin.read_line(&mut buffer).expect("Unable to read line!");
            self.commands.push(buffer.to_owned());

            match buffer.as_str() {
                ".quit" => {
                    println!("See you!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.commands {
                        println!("{}", command);
                    }
                }
                _ => {
                    interpreter::eval(buffer)?;
                }
            }
        }
    }
}
