pub(crate) mod ast;
pub(crate) mod cli;
pub(crate) mod lexer;

use lexer::Lexer;
use std::env;

fn main() {
    let mut expression: String = cli::parse_input(env::args());

    let lexer = Lexer::new(&mut expression);

    println!("Hello, world!");
}
