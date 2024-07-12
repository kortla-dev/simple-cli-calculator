use crate::ast::{Ast, BinOp, Node};

pub fn exec(ast: Ast) -> f64 {
    expr(ast.root)
}

fn expr(e: Node) -> f64 {
    let t_: Node;
    let ep_: Node;

    match e {
        Node::Expr { t, ep } => {
            t_ = *t;
            ep_ = *ep;
        }
        _ => unreachable!("{:?}", e),
    }

    let left = term(t_);

    if ep_ == Node::Eps {
        return left;
    }

    let (op, right) = expr_prime(ep_);

    binop_exec(op, left, right)
}

fn expr_prime(ep: Node) -> (BinOp, f64) {
    let op_: BinOp;
    let t_: Node;
    let ep_: Node;

    match ep {
        Node::ExprPrime { op, t, ep } => {
            op_ = op;
            t_ = *t;
            ep_ = *ep;
        }
        _ => unreachable!(),
    }

    let left = term(t_);

    if ep_ == Node::Eps {
        return (op_, left);
    }

    let (inner_op, right) = expr_prime(ep_);

    (op_, binop_exec(inner_op, left, right))
}

fn term(t: Node) -> f64 {
    let f_: Node;
    let tp_: Node;

    match t {
        Node::Term { f, tp } => {
            f_ = *f;
            tp_ = *tp;
        }
        _ => unreachable!(),
    }

    let left = factor(f_);

    if tp_ == Node::Eps {
        return left;
    }

    let (op, right) = term_prime(tp_);

    binop_exec(op, left, right)
}

fn term_prime(tp: Node) -> (BinOp, f64) {
    let op_: BinOp;
    let f_: Node;
    let tp_: Node;

    match tp {
        Node::TermPrime { op, f, tp } => {
            op_ = op;
            f_ = *f;
            tp_ = *tp;
        }
        _ => unreachable!(),
    }

    let left = factor(f_);

    if tp_ == Node::Eps {
        return (op_, left);
    }

    let (inner_op, right) = term_prime(tp_);

    (op_, binop_exec(inner_op, left, right))
}

fn factor(f: Node) -> f64 {
    match f {
        Node::Factor { id } => id,
        Node::Paren { e } => expr(*e),
        _ => unreachable!(),
    }
}

fn binop_exec(op: BinOp, left: f64, right: f64) -> f64 {
    match op {
        BinOp::Mul => left * right,
        BinOp::Div => left / right,
        BinOp::Add => left + right,
        BinOp::Sub => left - right,
    }
}
