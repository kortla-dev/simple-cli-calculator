use crate::ast::{Ast, Node, Token};
use std::collections::HashMap;

type ParserTable<'tbl> = HashMap<(&'tbl str, &'tbl str), Vec<&'tbl str>>;

// not the best way to do this but is fine for a small grammar like this
fn get_table<'def>() -> ParserTable<'def> {
    let mut table = ParserTable::new();

    // Empty vectors are epsilon
    table.insert(("E", "id"), vec!["Ep", "T"]);
    table.insert(("E", "("), vec!["Ep", "T"]);

    table.insert(("Ep", ")"), vec![]); // epsilon
    table.insert(("Ep", "$"), vec![]); // epsilon
    table.insert(("Ep", "+"), vec!["Ep", "T", "+"]);
    table.insert(("Ep", "-"), vec!["Ep", "T", "-"]);

    table.insert(("T", "id"), vec!["Tp", "F"]);
    table.insert(("T", "("), vec!["Tp", "F"]);

    table.insert(("Tp", ")"), vec![]); // epsilon
    table.insert(("Tp", "+"), vec![]); // epsilon
    table.insert(("Tp", "-"), vec![]); // epsilon
    table.insert(("Tp", "$"), vec![]); // epsilon
    table.insert(("Tp", "*"), vec!["Tp", "F", "*"]);
    table.insert(("Tp", "/"), vec!["Tp", "F", "/"]);

    table.insert(("F", "id"), vec!["id"]);
    table.insert(("F", "("), vec![")", "E", "("]);

    table
}

pub(crate) struct LL1<'prsr, I>
where
    I: Iterator<Item = Token>,
{
    parser_table: ParserTable<'prsr>,
    token_stream: I,
}

impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(token_stream: I) -> Self {
        let parser_table: ParserTable = get_table();

        Self {
            parser_table,
            token_stream,
        }
    }

    pub fn get_ast(&mut self) -> Ast {
        let root = self.expr();

        Ast::new(root)
    }
}

// Handles Expr and ExprPrime
impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn expr(&mut self) -> Node {
        Node::Eps
    }

    fn expr_prime(&mut self) -> Node {
        Node::Eps
    }
}

// Handles Term and TermPrime
impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn term(&mut self) -> Node {
        Node::Eps
    }

    fn term_prime(&mut self) -> Node {
        Node::Eps
    }
}

// Handles Factor and Paren
impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn factor(&mut self) -> Node {
        Node::Eps
    }

    fn paren(&mut self) -> Node {
        Node::Eps
    }
}
