// #![warn(missing_docs)]

/*!
# läspa
A simple programming language written in Rust. It's a Lisp-like language, but with a more
traditional syntax. Each element of the language is designed to be as easy to implement as
possible (hence the [RPN](https://en.wikipedia.org/wiki/Reverse_Polish_notation)), so that it
can be used as a learning tool for people who want to learn how to write their own programming
language. The language itself is not very useful, but it's a good starting
point for learning how to write a programming language.

## Example
```ignore
let x 10;
while < x 100
    := x + x 1;
end

if < x 100
    return 1;
else
    return 2;
end
```

## Syntax
The syntax is very simple. Each statement is separated by a newline or a semicolon. Comments are
denoted by `//`. The language is **whitespace sensitive**, but indentation is **not** important. The language
is also case sensitive.

**Warning**: There is no error handling yet, so if you make a mistake, the program will panic, or the result will be incorrect.

## Usage
The easiest way to use the language is to use the [`Interpreter`] struct. This will interpret the
language and return the result. This is the default compiler for the language. In the future, there
will be more compilers, such as an LLVM compiler or machine code compiler.

## Source Files
Source files are just text files with the `.laspa` extension. You can use any extension you want,
but `.laspa` is the default. You can use the [`Interpreter::from_file`] method to read from a file.

```rust
use laspa::{Interpreter, Compile};

let result = Interpreter::from_source("return + 1 2;");
assert_eq!(result, 3.0);
```
 */

use lazy_static::lazy_static;
use regex::{Regex, Split};
use std::{collections::HashMap, str::SplitWhitespace};

/// The default number type. Every number is a [`f64`] number for simplicity.
#[derive(Debug, PartialEq, Clone)]
pub struct Number(pub f64);

impl Number {
    /// Create a new number from a string. This will return an error if the string is not a valid
    pub fn new(s: &str) -> Result<Self, String> {
        match s.parse::<f64>() {
            Ok(n) => Ok(Self(n)),
            Err(_) => Err(format!("Invalid number: {s}")),
        }
    }
}

/// The default operator type. This is used for arithmetic and comparison operations.
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    /// Greater than
    Gt,
    /// Less than
    Lt,
}

impl Op {
    /// Create a new operator from a string. This will panic if the string is not a valid operator.
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

/// The default binary expression type. This is used for arithmetic and comparison operations (e.g. `+ 1 2` would equal `3`).
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub op: Op,
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
}

/// The default bind expression type. This is used to bind a value to a variable (e.g. `let x 10` binding the number `10` to `x`).
#[derive(Debug, PartialEq, Clone)]
pub struct BindExpr {
    pub name: String,
    pub value: Vec<Node>,
}

/// The default return expression type. This is used to return a value from a function. If this is not used, the last value in the function will be returned.
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnExpr {
    pub value: Vec<Node>,
}

/// The default mutate expression type. This is used to mutate a variable (e.g. `:= x 10` setting the value of `x` to `10`).
/// Variables can only be mutable.
#[derive(Debug, PartialEq, Clone)]
pub struct MutateExpr {
    pub name: String,
    pub value: Vec<Node>,
}

/// The default while expression type. This is used to create a while loop (e.g. `while < x 10` will loop while `x` is less than `10`).
#[derive(Debug, PartialEq, Clone)]
pub struct WhileExpr {
    pub condition: Vec<Node>,
    pub body: Vec<Node>,
}

/// The default if expression type. This is used to create an if statement (e.g. `if < x 10` will run the code in the if statement if `x` is less than `10`).
/// The else statement is optional.
#[derive(Debug, PartialEq, Clone)]
pub struct IfExpr {
    pub condition: Vec<Node>,
    pub body: Vec<Node>,
    pub else_body: Vec<Node>,
}

/// The default node type. This is used to represent every element of the language. This is used to create an abstract syntax tree (AST).
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

/// Lex a string into tokens. This will split the string into tokens, which can then be parsed into an AST.
pub fn lex(s: &str) -> regex::Split<'static, '_> {
    RE.split(s)
}

/// Parse tokens into an AST. This will parse a string of tokens into an AST, which can then be evaluated.
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

/// Parse a sentence into an AST. This will parse a sentence into an AST, which can then be evaluated.
/// Sentences are separated by newlines or `;` as provided by the regex in the lexer.
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
                nodes.push(Node::IfExpr(IfExpr {
                    condition,
                    body,
                    else_body,
                }));
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

/// Evaluate an AST. This will evaluate an AST and return the result. All variables are in the global scope.
/// This is essentially the interpreter for the language.
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

/// The default trait for compiling a language. This is used to compile a language from a specific source.
/// This trait can be implemented for any output: llvm, interpreter, etc.
pub trait Compile {
    /// The output type of the compiler. Varies depending on the compiler.
    type Output;

    /// Compile an AST into the output type.
    fn from_ast(nodes: Vec<Node>) -> Self::Output;

    /// Compile a string into the output type.
    fn from_source(source: &str) -> Self::Output {
        let mut tokens = lex(source);
        // println!("tokens: {:?}", lex(source).collect::<Vec<_>>());
        let nodes = parse(&mut tokens);
        println!("ast: {:?}", nodes);
        Self::from_ast(nodes)
    }

    /// Compile a file into the output type. Supply the crate-relative path to the file.
    fn from_file(path: &str) -> Self::Output {
        let source = std::fs::read_to_string(path).unwrap();
        Self::from_source(&source)
    }
}

/// The default interpreter.
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
