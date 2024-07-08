// This calculator is be limited in the following aspects
// - only simple expressions will be supported
// - all numbers must be positive
// - only grouping with () and * / + - will be supported
// - 5(2+1) will not be interpreted as 5*(2+1) this must be specified

#[derive(Debug, PartialEq)]
pub(crate) enum Paren {
    OParen,
    CParen,
}

#[derive(Debug, PartialEq)]
pub(crate) enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Parenthesis(Paren),
    Operator(BinOp),
    Number(f64),
    Eos,
}

impl Token {
    pub(crate) fn build_token(chr: char) -> Self {
        match chr {
            '(' => Token::Parenthesis(Paren::OParen),
            ')' => Token::Parenthesis(Paren::CParen),
            '*' => Token::Operator(BinOp::Mul),
            '/' => Token::Operator(BinOp::Div),
            '+' => Token::Operator(BinOp::Add),
            '-' => Token::Operator(BinOp::Sub),
            _ => panic!("Error: Unkown character '{}'", chr),
        }
    }

    pub(crate) fn build_num_token(num: String) -> Self {
        Token::Number(num.parse::<f64>().unwrap())
    }
}

/// Parser grammar
/// ```text
/// E  -> TE'
/// E' -> +TE' | -TE' | ''
/// T  -> FT'
/// T' -> *FT' | /FT' | ''
/// F  -> id | (E)
/// ```
#[derive(Debug)]
pub(crate) enum Node {
    /// E(T, E')
    Expr { t: Box<Self>, ep: Box<Self> },
    /// E'(T, E')
    ExprPrime {
        op: BinOp,
        t: Box<Self>,
        ep: Box<Self>,
    },
    /// T(F, T')
    Term { f: Box<Self>, tp: Box<Self> },
    /// T'(F, T')
    TermPrime {
        op: BinOp,
        f: Box<Self>,
        tp: Box<Self>,
    },
    /// F(id)
    Factor { id: f64 },
    /// (E)
    Paren { e: Box<Self> },
}

#[derive(Debug)]
pub(crate) struct Ast {
    root: Node,
}
