use std::collections::HashMap;

use inkwell::{self, context::Context, module::Module};

use crate::{Compile, Node};

pub struct LLVMCompiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {

    pub fn new (
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        nodes: Vec<Node>
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let mut compiler = Self {
            context,
            builder,
            module,
            fpm,
        };

        compiler.gen_ir(nodes)
    }

    pub fn gen_ir(&mut self, nodes: Vec<Node>) -> Result<(), ()> {
        todo!("Generate LLVM IR code from AST nodes");
        // Ok(())
    }

    // Template function for compiling expressions (e.g. binary ops, literals, variables)
    pub fn compile_expr(&mut self, _expr: &Node) {
        // TODO: Based on the type of expr, generate the appropriate IR
    }
}

impl Compile for LLVMCompiler<'_, '_> {
    type Output = Result<(), ()>;

    fn from_ast(nodes: Vec<Node>) -> Self::Output {
        let mut compiler = LLVMCompiler::new();

        compiler.new(nodes)
    }
}
