mod ast;
mod cli;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::LL1;
use std::env;

fn main() {
    let mut expression: String = cli::parse_input(env::args());

    let lexer = Lexer::new(&mut expression);

    let parser = LL1::new(lexer);

    println!("Hello, world!");
}
