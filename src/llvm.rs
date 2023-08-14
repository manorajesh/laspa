use inkwell::{self, context::Context, module::Module};

pub struct LLVMCompiler {
    pub context: Context,
    pub module: Module,
    pub variables: HashMap<String, f64>,
}