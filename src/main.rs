mod ast;
mod cli;
mod evaluator;
mod lexer;
mod parser;

use crate::{lexer::Lexer, parser::LL1};
use std::env;

fn main() {
    let mut expression: String = cli::parse_input(env::args());

    let lexer = Lexer::new(&mut expression);

    let mut parser = LL1::new(lexer);

    let ast = parser.get_ast();

    println!("{}", evaluator::exec(ast));
}
