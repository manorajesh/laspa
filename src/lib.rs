#![warn(missing_docs)]

use lazy_static::lazy_static;
use regex::{Regex, Split};
use std::{collections::HashMap, str::SplitWhitespace};

#[derive(Debug, PartialEq, Clone)]
pub struct Number(pub f64);

impl Number {
    pub fn new(s: &str) -> Result<Self, String> {
        match s.parse::<f64>() {
            Ok(n) => Ok(Self(n)),
            Err(_) => Err(format!("Invalid number: {s}")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub op: Op,
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BindExpr {
    pub name: String,
    pub value: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnExpr {
    pub value: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MutateExpr {
    pub name: String,
    pub value: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileExpr {
    pub condition: Vec<Node>,
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpr {
    pub condition: Vec<Node>,
    pub body: Vec<Node>,
    pub else_body: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Number(Number),
    BinaryExpr(BinaryExpr),
    BindExpr(BindExpr),
    Variable(String),
    ReturnExpr(ReturnExpr),
    MutateExpr(MutateExpr),
    WhileExpr(WhileExpr),
    IfExpr(IfExpr),
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"[;\n]").unwrap();
}

pub fn lex(s: &str) -> regex::Split<'static, '_> {
    RE.split(s)
}

pub fn parse(tokens: &mut Split<'static, '_>) -> Vec<Node> {
    let mut nodes = Vec::new();
    while let Some(token) = tokens.next() {
        // println!("token: {}", token);
        if token.trim() == "end" {
            break;
        }

        if let Ok(mut new_nodes) = parse_sentence(&mut token.split_whitespace()) {
            nodes.append(&mut new_nodes);
        }

        if let Some(Node::WhileExpr(e)) = nodes.last_mut() {
            if e.body.is_empty() {
                e.body = parse(tokens);
            }
        }

        if let Some(Node::IfExpr(e)) = nodes.last_mut() {
            if e.body.is_empty() {
                let body = parse(tokens);
                let mut body = body.split(|n| n == &Node::Variable("else".to_string()));
                e.body = body.next().unwrap().to_vec();
                e.else_body = body.next().unwrap_or(&Vec::new()).to_vec();
            }
        }
        // println!("nodes: {:?}", nodes)
    }
    nodes
}

fn parse_sentence(tokens: &mut SplitWhitespace) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();
    match tokens.next() {
        Some(t) => match t {
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

            "//" => {
                return Ok(nodes);
            }

            "return" => {
                nodes.push(Node::ReturnExpr(ReturnExpr {
                    value: parse_sentence(tokens).unwrap(),
                }));
            }

            ":=" => {
                let name = tokens.next().unwrap();
                let value = parse_sentence(tokens).unwrap();
                nodes.push(Node::MutateExpr(MutateExpr {
                    name: name.to_string(),
                    value,
                }));
            }

            "while" => {
                let condition = parse_sentence(tokens).unwrap();
                let body = Vec::new();
                nodes.push(Node::WhileExpr(WhileExpr { condition, body }));
            }

            "if" => {
                let condition = parse_sentence(tokens).unwrap();
                let body = Vec::new();
                let else_body = Vec::new();
                nodes.push(Node::IfExpr(IfExpr { condition, body, else_body }));
            }

            _ => match Number::new(t) {
                Ok(n) => nodes.push(Node::Number(n)),
                Err(_) => nodes.push(Node::Variable(t.to_string())),
            },
        },

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
                    Op::Gt => (lhs > rhs) as i32 as f64,
                    Op::Lt => (lhs < rhs) as i32 as f64,
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
                0.0 // This doesn't matter, because we'll check return_val at the end
            }
            Node::MutateExpr(e) => {
                let value = eval(&e.value, globals);
                if let Some(n) = globals.get_mut(&e.name) {
                    *n = value;
                } else {
                    panic!("Variable not found: {}", e.name);
                }
                value
            }
            Node::WhileExpr(e) => {
                while eval(&e.condition, globals) != 0.0 {
                    eval(&e.body, globals);
                }
                0.0
            }
            Node::IfExpr(e) => {
                if eval(&e.condition, globals) != 0.0 {
                    eval(&e.body, globals)
                } else {
                    eval(&e.else_body, globals)
                }
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
        // println!("tokens: {:?}", lex(source).collect::<Vec<_>>());
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
            vec![Node::BinaryExpr(BinaryExpr {
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
            }),]
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
        assert_eq!(
            Interpreter::from_source(
                r#"
            let x 1
        "#
            ),
            1.0
        );
    }

    #[test]
    fn variable_arithmetic() {
        assert_eq!(
            Interpreter::from_source(
                "let x 2;
        let y 1;
        + x y;"
            ),
            3.0
        );
    }

    #[test]
    fn variable_arithmetic_complex() {
        assert_eq!(
            Interpreter::from_source(
                "let x 2;
        let y 1;
        let z + x * y 2;
        z;"
            ),
            4.0
        );
    }

    #[test]
    fn return_only() {
        assert_eq!(Interpreter::from_source("+ 2 3;return 1;"), 1.0);
    }

    #[test]
    fn while_loop() {
        assert_eq!(
            Interpreter::from_source(
                r#"
        let x 0;
        // let y 0;
        
        while < x 1000
            let i 0;
            while < i 100
                := x + x 1;
                := i + i 1;
            end
        end
        
        return + x i;
        "#
            ),
            1100.0
        );
    }

    #[test]
    fn if_else() {
        assert_eq!(
            Interpreter::from_source(
                r#"
        let x 0;
        if < x 1
            return 1;
        else
            return 2;
        end
        "#
            ),
            1.0
        );
    }

    #[test]
    fn only_if() {
        assert_eq!(
            Interpreter::from_source(
                r#"
                let x 10;
                let y 2
                
                if < x y
                then
                    return y
                end
                
                return x
        "#
            ),
            10.0
        );
    }

    #[test]
    fn read_from_file() {
        assert_eq!(Interpreter::from_file("examples/test.laspa"), 10.0);
    }
}
