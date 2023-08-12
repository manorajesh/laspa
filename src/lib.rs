use std::{str::{SplitWhitespace}, collections::HashMap};
use regex::{Regex, Split};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
pub struct Number(pub f64);

impl Number {
    pub fn new(s: &str) -> Result<Self, String> {
        match s.parse::<f64>() {
            Ok(n) => Ok(Self(n)),
            Err(_) => Err(format!("Invalid number: {s}")),
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
    Assign,
    Return,
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
            "let" => Self::Assign,
            "return" => Self::Return,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub op: Op,
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct BindExpr {
    pub name: String,
    pub value: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnExpr {
    pub value: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(Number),
    BinaryExpr(BinaryExpr),
    BindExpr(BindExpr),
    Variable(String),
    ReturnExpr(ReturnExpr),
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"[;\n]").unwrap();
}

pub fn lex<'a>(s: &'a str) -> regex::Split<'static, 'a> {
    RE.split(s)
}


pub fn parse<'a>(tokens: &mut Split<'static, 'a>) -> Vec<Node> {
    let mut nodes = Vec::new();
    for token in tokens {
        if let Ok(mut new_nodes) = parse_sentence(&mut token.split_whitespace()) {
            nodes.append(&mut new_nodes);
        }
    }
    nodes
}

fn parse_sentence(tokens: &mut SplitWhitespace) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();
    match tokens.next() {
        Some(t) => {
            match t {
                "+" | "-" | "*" | "/" | ">" | "<" => {
                    nodes.push(Node::BinaryExpr(BinaryExpr {
                        op: Op::new(t),
                        lhs: parse_sentence(tokens).unwrap(),
                        rhs: parse_sentence(tokens).unwrap(),
                    }));
                }

                "let" => {
                    let name = tokens.next().unwrap();
                    let value = parse_sentence(tokens).unwrap();
                    nodes.push(Node::BindExpr(BindExpr {
                        name: name.to_string(),
                        value,
                    }));
                }

                ";" => {}

                "//" => {
                    while let Some(_) = tokens.next() {}
                }

                "return" => {
                    nodes.push(Node::ReturnExpr(ReturnExpr {
                        value: parse_sentence(tokens).unwrap(),
                    }));
                }

                _ => {
                    match Number::new(t) {
                        Ok(n) => nodes.push(Node::Number(n)),
                        Err(_) => nodes.push(Node::Variable(t.to_string())),
                    }
                }
            }
        }

        None => return Err("No tokens found".to_string()),
    }

    Ok(nodes)
}

pub fn eval(ast: &Vec<Node>, globals: &mut HashMap<String, f64>) -> f64 {
    let mut return_val: Option<f64> = None;
    let mut last_val: f64 = 0.0;

    for node in ast {
        last_val = match node {
            Node::Number(n) => n.0,
            Node::BinaryExpr(e) => {
                let lhs = eval(&e.lhs, globals);
                let rhs = eval(&e.rhs, globals);

                match e.op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                    _ => 0.0,
                }
            }
            Node::BindExpr(e) => {
                let value = eval(&e.value, globals);
                globals.insert(e.name.clone(), value);
                value
            }
            Node::Variable(v) => match globals.get(v) {
                Some(n) => *n,
                None => panic!("Variable not found: {v}"),
            },
            Node::ReturnExpr(e) => {
                return_val = Some(eval(&e.value, globals));
                0.0  // This doesn't matter, because we'll check return_val at the end
            }
        };
    }

    return_val.unwrap_or(last_val)
}


pub trait Compile {
    type Output;

    fn from_ast(nodes: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        let mut tokens = lex(source);
        // println!("tokens: {:?}", tokens);
        let nodes = parse(&mut tokens);
        println!("ast: {:?}", nodes);
        Self::from_ast(nodes)
    }

    fn from_file(path: &str) -> Self::Output {
        let source = std::fs::read_to_string(path).unwrap();
        Self::from_source(&source)
    }
}

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = f64;

    fn from_ast(nodes: Vec<Node>) -> Self::Output {
        eval(&nodes, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("1.0").unwrap(), Number(1.0));
        assert_eq!(Number::new("4").unwrap(), Number(4.0));
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
                Node::BinaryExpr(BinaryExpr {
                    op: Op::Add,
                    lhs: vec![Node::BinaryExpr(BinaryExpr {
                        op: Op::Mul,
                        lhs: vec![Node::Number(Number(-2.0))],
                        rhs: vec![Node::Number(Number(3.0))],
                    })],
                    rhs: vec![Node::BinaryExpr(BinaryExpr {
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
        let mut tokens = lex("return + * -2 3 - 2 3.5");
        let nodes = parse(&mut tokens);
        assert_eq!(eval(&nodes, &mut HashMap::new()), -7.5);
    }

    #[test]
    fn interpret() {
        assert_eq!(Interpreter::from_source("+ * -2 3 - 2 3.5"), -7.5);
    }

    #[test]
    fn define_variable() {
        assert_eq!(Interpreter::from_source(r#"
            let x 1
        "#), 1.0);
    }

    #[test]
    fn variable_arithmetic() {
        assert_eq!(Interpreter::from_source("let x 2;
        let y 1;
        + x y;"), 3.0);
    }

    #[test]
    fn variable_arithmetic_complex() {
        assert_eq!(Interpreter::from_source("let x 2;
        let y 1;
        let z + x * y 2;
        z;"), 4.0);
    }

    #[test]
    fn read_from_file() {
        assert_eq!(Interpreter::from_file("examples/test.laspa"), 22.0);
    }
}
