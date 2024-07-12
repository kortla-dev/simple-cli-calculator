use crate::ast::{Ast, Node, Token};
use std::{collections::HashMap, mem};

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
    stack: Vec<&'prsr str>,
    prsr_tbl: ParserTable<'prsr>,
    tkn_strm: I,
    cur_tkn: Token,
}

impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tkn_strm: I) -> Self {
        let stack = vec!["$", "E"];
        let prsr_tbl: ParserTable = get_table();
        let cur_tkn = Token::Eos;

        Self {
            stack,
            prsr_tbl,
            tkn_strm,
            cur_tkn,
        }
    }

    pub fn get_ast(&mut self) -> Ast {
        self.dbg("START");
        let root = self.expr();
        println!("{:#?}", root);
        Ast::new(root)
    }

    /// Advances to the next token in the token stream and returns the current token.
    ///
    /// This method replaces the current token (`self.cur_tkn`) with the next token
    /// in the token stream (`self.tkn_strm`). It consumes and returns the replaced token.
    ///
    fn advance_token(&mut self) -> Token {
        mem::replace(&mut self.cur_tkn, self.tkn_strm.next().unwrap())
    }

    fn stack_extend(&mut self, node: &str) {
        let ext = self
            .prsr_tbl
            .get(&(node, self.cur_tkn.as_terminal()))
            .expect("Error: Invalid syntax");

        self.stack.extend(ext);
    }

    fn pop_extend(&mut self, node: &str) {
        self.stack.pop();
        self.stack_extend(node);
    }

    #[cfg(debug_assertions)]
    fn dbg(&self, caller: &str) {
        println!(
            "{}\nstack: {:?}\ncur_tkn: {:?}\n",
            caller, self.stack, self.cur_tkn
        );
    }

    #[cfg(not(debug_assertions))]
    fn dbg(&self, _caller: &str) {}
}

// Handles Expr and ExprPrime
impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn expr(&mut self) -> Node {
        self.advance_token();
        self.pop_extend("E");
        self.dbg("expr");

        let term = self.term();
        let expr_prime = self.expr_prime();

        Node::Expr {
            t: Box::new(term),
            ep: Box::new(expr_prime),
        }
    }

    fn expr_prime(&mut self) -> Node {
        self.pop_extend("Ep");
        self.dbg("expr_prime");

        let op = match self.cur_tkn.as_terminal() {
            "+" | "-" => {
                self.stack.pop();
                self.advance_token().get_op()
            }
            _ => return Node::Eps,
        };

        let term = self.term();
        let expr_prime = self.expr_prime();

        Node::ExprPrime {
            op,
            t: Box::new(term),
            ep: Box::new(expr_prime),
        }
    }
}

// Handles Term and TermPrime
impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn term(&mut self) -> Node {
        self.pop_extend("T");
        self.dbg("term");

        let factor = self.factor();
        let term_prime = self.term_prime();

        Node::Term {
            f: Box::new(factor),
            tp: Box::new(term_prime),
        }
    }

    fn term_prime(&mut self) -> Node {
        self.pop_extend("Tp");
        self.dbg("term_prime");

        let op = match self.cur_tkn.as_terminal() {
            "*" | "/" => {
                self.stack.pop();
                self.advance_token().get_op()
            }
            _ => return Node::Eps,
        };

        let factor = self.factor();
        let term_prime = self.term_prime();

        Node::TermPrime {
            op,
            f: Box::new(factor),
            tp: Box::new(term_prime),
        }
    }
}

// Handles Factor and Paren
impl<I> LL1<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn factor(&mut self) -> Node {
        self.pop_extend("F");
        self.dbg("factor");

        self.stack.pop();

        if "id" == self.cur_tkn.as_terminal() {
            Node::Factor {
                id: self.advance_token().get_num(),
            }
        } else {
            self.paren()
        }
    }

    fn paren(&mut self) -> Node {
        let expr = self.expr();

        if ")" == self.cur_tkn.as_terminal() {
            self.stack.pop();
            self.advance_token();
        } else {
            panic!("Error: Missing closing parenthesis ')'");
        }

        Node::Paren { e: Box::new(expr) }
    }
}
