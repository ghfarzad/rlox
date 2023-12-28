use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod scanner;

use scanner::Scanner;

fn report(line: i32, location: &String, message: &String) {
    println!("[line {}]Error {}: {}", line, location, message);
    //TODO(FG): set error flag
}

fn error(line: i32, message: &String) {
    report(line, &String::new(), message);
}

fn run(src: &String) {
    println!("{}", src);

    let scanner = Scanner::new(src);

    let _tokens = scanner.scan_tokens();
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(_n) => {
                run(&input);
            }
            Err(error) => println!("error: {error}")
        }
    }
}

fn run_file(file_path: &String) -> io::Result<()> {
    let mut f = File::open(file_path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    run(&buffer);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let _ = run_prompt();
        },
        2 => {
            let _ = run_file(&args[0]);
        },
        _ => {
            println!("Input Error!!!");
        }
    }
}
