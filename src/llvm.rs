use std::collections::HashMap;

use inkwell::{self, context::Context, module::Module};

pub struct LLVMCompiler<'a> {
    pub context: Context,
    pub module: Module<'a>,
    pub variables: HashMap<String, f64>,
}