use std::str::SplitWhitespace;

#[derive(Debug, PartialEq)]
pub struct Number(pub f64);

impl Number {
    pub fn new(s: &str) -> Self {
        match s.parse::<f64>() {
            Ok(n) => Self(n),
            Err(_) => panic!("Invalid number: {s}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Gt, // Greater than
    Lt, // Less than
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            ">" => Self::Gt,
            "<" => Self::Lt,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub op: Op,
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(Number),
    Expr(Expr),
}

pub fn lex(s: &str) -> SplitWhitespace {
    s.split_whitespace()
}

pub fn parse(tokens: &mut SplitWhitespace) -> Vec<Node> {
    let mut nodes = Vec::new();
    match tokens.next() {
        Some(t) => {
            match t {
                "+" | "-" | "*" | "/" | ">" | "<" => {
                    nodes.push(Node::Expr(Expr {
                        op: Op::new(t),
                        lhs: parse(tokens),
                        rhs: parse(tokens),
                    }));
                }
                _ => {
                    nodes.push(Node::Number(Number::new(t)));
                }
            }
        }

        None => panic!("No tokens found"),
    }
    nodes
}

pub fn eval(ast: &Vec<Node>) -> f64 {
    let mut return_val: f64 = 0.0;
    for node in ast {
        match node {
            Node::Number(n) => return_val += n.0,
            Node::Expr(e) => {
                let lhs = eval(&e.lhs);
                let rhs = eval(&e.rhs);

                return_val = match e.op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                    Op::Gt => {
                        if lhs > rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    Op::Lt => {
                        if lhs < rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                }
            }
        }
    }
    return_val
}

pub trait Compile {
    type Output;

    fn from_ast(nodes: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        let mut tokens = lex(source);
        let nodes = parse(&mut tokens);
        Self::from_ast(nodes)
    }
}

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = f64;

    fn from_ast(nodes: Vec<Node>) -> Self::Output {
        eval(&nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("1.0"), Number(1.0));
        assert_eq!(Number::new("4"), Number(4.0));
    }

    #[test]
    fn parse_add() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_sub() {
        assert_eq!(Op::new("-"), Op::Sub);
    }

    #[test]
    fn parse_mul() {
        assert_eq!(Op::new("*"), Op::Mul);
    }

    #[test]
    fn parse_div() {
        assert_eq!(Op::new("/"), Op::Div);
    }

    #[test]
    fn parse_gt() {
        assert_eq!(Op::new(">"), Op::Gt);
    }

    #[test]
    fn parse_lt() {
        assert_eq!(Op::new("<"), Op::Lt);
    }

    #[test]
    fn parse_expr() {
        let mut tokens = lex("+ * -2 3 - 2 3.5");
        let nodes = parse(&mut tokens);
        assert_eq!(
            nodes,
            vec![
                Node::Expr(Expr {
                    op: Op::Add,
                    lhs: vec![Node::Expr(Expr {
                        op: Op::Mul,
                        lhs: vec![Node::Number(Number(-2.0))],
                        rhs: vec![Node::Number(Number(3.0))],
                    })],
                    rhs: vec![Node::Expr(Expr {
                        op: Op::Sub,
                        lhs: vec![Node::Number(Number(2.0))],
                        rhs: vec![Node::Number(Number(3.5))],
                    })],
                }),
            ]
        )
    }

    #[test]
    fn eval_expr() {
        let mut tokens = lex("+ * -2 3 - 2 3.5");
        let nodes = parse(&mut tokens);
        assert_eq!(eval(&nodes), -7.5);
    }

    #[test]
    fn interpret() {
        assert_eq!(Interpreter::from_source("+ * -2 3 - 2 3.5"), -7.5);
    }
}
