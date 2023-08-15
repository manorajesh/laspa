use inkwell::{self, context::Context, module::Module, builder::Builder, passes::PassManager, values::{FunctionValue, FloatValue}, targets::{Target, InitializationConfig}};

use crate::{Compile, Node, Op};

pub struct LLVMCompiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>
}

impl<'a, 'ctx> LLVMCompiler<'a, 'ctx> {
    pub fn codegen (
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

        compiler.gen_main(nodes)
    }

    pub fn gen_main(&mut self, nodes: Vec<Node>) -> Result<FunctionValue<'ctx>, &'static str> {
        let main_type = self.context.f64_type().fn_type(&[], false);
        let main_func = self.module.add_function("main", main_type, None);

        let basic_block = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(basic_block);

        let ret = self.gen_expr(nodes)?;

        self.builder.build_return(Some(&ret));

        Ok(main_func)
    }

    pub fn gen_expr(&mut self, nodes: Vec<Node>) -> Result<FloatValue<'ctx>, &'static str> {
        for node in nodes {
            match node {
                Node::Number(n) => {
                    return Ok(self.context.f64_type().const_float(n.0));
                }
                Node::BinaryExpr(e) => {
                    let lhs = self.gen_expr(e.lhs)?;
                    let rhs = self.gen_expr(e.rhs)?;

                    match e.op {
                        Op::Add => {
                            return Ok(self.builder.build_float_add(lhs, rhs, "addtmp"));
                        }
                        Op::Sub => {
                            return Ok(self.builder.build_float_sub(lhs, rhs, "subtmp"));
                        }
                        Op::Mul => {
                            return Ok(self.builder.build_float_mul(lhs, rhs, "multmp"));
                        }
                        Op::Div => {
                            return Ok(self.builder.build_float_div(lhs, rhs, "divtmp"));
                        }
                        Op::Mod => {
                            return Ok(self.builder.build_float_rem(lhs, rhs, "modtmp"));
                        }
                        // Op::Gt => {
                        //     return Ok(self.builder.build_float_compare(
                        //         inkwell::FloatPredicate::OGT,
                        //         lhs,
                        //         rhs,
                        //         "gttmp",
                        //     ));
                        // }
                        // Op::Lt => {
                        //     return Ok(self.builder.build_float_compare(
                        //         inkwell::FloatPredicate::OLT,
                        //         lhs,
                        //         rhs,
                        //         "lttmp",
                        //     ));
                        // }
                        // Op::Eqt => {
                        //     return Ok(self.builder.build_float_compare(
                        //         inkwell::FloatPredicate::OEQ,
                        //         lhs,
                        //         rhs,
                        //         "eqttmp",
                        //     ));
                        // }

                        _ => panic!("Unknown binary operator")
                    }
                }

                _ => panic!("Unknown node type")
            }
        }

        Ok(self.context.f64_type().const_float(0.0))
    }

    // Template function for compiling expressions (e.g. binary ops, literals, variables)
    // pub fn compile_expr(&mut self, _expr: &Node) {
    //     // TODO: Based on the type of expr, generate the appropriate IR
    // }
}

impl Compile for LLVMCompiler<'_, '_> {
    type Output = Result<f64, &'static str>;

    fn from_ast(nodes: Vec<Node>, jit: bool) -> Self::Output {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("main");
        let fpm = PassManager::create(&module);

        LLVMCompiler::codegen(&context, &builder, &module, &fpm, nodes).expect("Failed to generate IR");

        if jit {
            Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");

            let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::Default).expect("Failed to create JIT execution engine");

            let main_func = unsafe { execution_engine.get_function::<unsafe extern "C" fn() -> f64>("main").expect("Failed to get main function") };
            let result = unsafe { main_func.call() };
            return Ok(result);
        }

        Ok(0.0)
    }
}
