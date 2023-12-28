use std::fs::File;
use std::io;
use std::io::prelude::*;


mod scanner;

use scanner::Scanner;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Lox {
        return Lox {
            had_error: false
        }
    }

    pub fn run_prompt(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            input.clear();
            match stdin.read_line(&mut input) {
                Ok(_n) => {
                    self.run(&input);
                }
                Err(error) => println!("error: {error}")
            }
        }
    }

    pub fn run_file(&self, file_path: &String) -> io::Result<()> {
        let mut f = File::open(file_path)?;
        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;

        self.run(&buffer);
        Ok(())
    }

    fn report(&self, line: i32, location: &String, message: &String) {
        println!("[line {}]Error {}: {}", line, location, message);
        //TODO(FG): set error flag
    }

    fn error(&self, line: i32, message: &String) {
        self.report(line, &String::new(), message);
    }

    fn run(&self, src: &String) {
        println!("{}", src);

        let scanner = Scanner::new(src);

        let _tokens = scanner.scan_tokens();
    }
}
