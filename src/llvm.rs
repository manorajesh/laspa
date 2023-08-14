use std::collections::HashMap;

use inkwell::{self, context::Context, module::Module};

use crate::Compile;

pub struct LLVMCompiler<'a> {
    pub context: Context,
    pub module: Module<'a>,
    pub variables: HashMap<String, f64>,
}

impl<'a> LLVMCompiler<'a> {
    // Constructor to easily create a new compiler instance
    pub fn new() -> Self {
        let context = Context::create();
        let module = context.create_module("my_module");
        let variables = HashMap::new();

        LLVMCompiler {
            context,
            module,
            variables,
        }
    }

    pub fn generate_ir(&mut self, nodes: Vec<Node>) -> Result<(), ()> {
        todo!("Generate LLVM IR code from AST nodes");
        Ok(())
    }

    // Template function for compiling expressions (e.g. binary ops, literals, variables)
    pub fn compile_expr(&mut self, expr: &Expr) {
        // TODO: Based on the type of expr, generate the appropriate IR
    }
}

impl Compile for LLVMCompiler<'_> {
    type Output = Result<(), ()>;

    fn from_ast(nodes: Vec<Node>) -> Self::Output {
        let mut compiler = LLVMCompiler::new();

        compiler.generate_ir(nodes)
    }
}