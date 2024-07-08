pub(crate) mod ast;
pub(crate) mod lexer;

use ast::Token;
use lexer::Lexer;

fn main() {
    // let mut inpt = "시험".to_string();
    let mut inpt = "1+1".to_string();

    let mut lex = Lexer::new(&mut inpt);

    while let Some(tkn) = lex.next() {
        if tkn == Token::Eos {
            break;
        }

        println!("{:?}", tkn);
    }

    println!("{:?}", lex);

    println!("Hello, world!");
}
