pub(crate) mod ast;
pub(crate) mod cli;
pub(crate) mod lexer;

use std::env;

fn main() {
    let expression = cli::parse_input(env::args());
    println!("Hello, world!");
}
