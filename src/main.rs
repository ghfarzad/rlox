use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn run(src: &String) {
    println!("{}", src);
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
