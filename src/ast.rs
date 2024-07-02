// This calculator is be limited in the following aspects
// - only simple expressions will be supported
// - all numbers must be positive
// - only grouping with () and * / + - will be supported
// - 5(2+1) will not be interpreted as 5*(2+1) this must be specified

pub(crate) enum Paren {
    OParen,
    CParen,
}

pub(crate) enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

pub(crate) enum Token {
    Parenthesis(Paren),
    Operator(BinOp),
    Number(f64),
}

/// Parser grammar
/// ```text
/// E  -> TE'
/// E' -> +TE' | -TE' | ''
/// T  -> FT'
/// T' -> *FT' | /FT' | ''
/// F  -> id | (E)
/// ```
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

pub(crate) struct Ast {
    root: Node,
}
