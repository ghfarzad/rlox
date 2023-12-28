use std::env;

mod lox;

use lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lox = Lox::new();
    match args.len() {
        1 => {
            let _ = lox.run_prompt();
        },
        2 => {
            let _ = lox.run_file(&args[0]);
        },
        _ => {
            println!("Input Error!!!");
        }
    }
}
