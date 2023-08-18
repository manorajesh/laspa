use std::{collections::HashMap, path::Path};

use inkwell::{self, context::Context, module::Module, builder::Builder, passes::PassManager, values::{FunctionValue, FloatValue, IntValue, BasicMetadataValueEnum, PointerValue}, targets::{Target, InitializationConfig}, types::BasicMetadataTypeEnum};

use crate::{Compile, Node, Op, CompileConfig, FnExpr};

pub enum LLVMValue<'ctx> {
    Float(FloatValue<'ctx>),
    Int(IntValue<'ctx>),
}

impl<'ctx> From<IntValue<'ctx>> for LLVMValue<'ctx> {
    fn from(val: IntValue<'ctx>) -> Self {
        LLVMValue::Int(val)
    }
}

impl<'ctx> From<FloatValue<'ctx>> for LLVMValue<'ctx> {
    fn from(val: FloatValue<'ctx>) -> Self {
        LLVMValue::Float(val)
    }
}

// impl<'ctx> From<LLVMValue<'ctx>> for FloatValue<'ctx> {
//     fn from(val: LLVMValue<'ctx>) -> Self {
//         match val {
//             LLVMValue::Float(val) => val,
//             _ => panic!("Expected float value")
//         }
//     }
// }

// impl<'ctx> From<LLVMValue<'ctx>> for IntValue<'ctx> {
//     fn from(val: LLVMValue<'ctx>) -> Self {
//         match val {
//             LLVMValue::Int(val) => val,
//             _ => panic!("Expected int value")
//         }
//     }
// }

impl<'ctx> LLVMValue<'ctx> {
    pub fn as_int(&self) -> Option<IntValue<'ctx>> {
        match self {
            LLVMValue::Int(int_val) => Some(*int_val),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<FloatValue<'ctx>> {
        match self {
            LLVMValue::Float(float_val) => Some(*float_val),
            _ => None,
        }
    }
}

pub struct LLVMCompiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub variables: Vec<HashMap<String, inkwell::values::PointerValue<'ctx>>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> LLVMCompiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
    ) -> Self {
        let variables= vec![HashMap::new()];
        Self {
            context,
            builder,
            module,
            fpm,
            variables,
            fn_value_opt: None,
        }
    }

    pub fn codegen(&mut self, nodes: Vec<Node>) -> Result<FunctionValue<'ctx>, &'static str> {
        self.gen_main(nodes)
    }

    pub fn gen_main(&mut self, nodes: Vec<Node>) -> Result<FunctionValue<'ctx>, &'static str> {
        let main_type = self.context.f64_type().fn_type(&[], false);
        let main_func = self.module.add_function("main", main_type, None);

        let basic_block = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(basic_block);

        self.fn_value_opt = Some(main_func);

        let ret = self.gen_body(&nodes)?.as_float().expect("Expected float value. Comparisons cannot be returned");

        self.builder.build_return(Some(&ret));

        Ok(main_func)
    }

    pub fn gen_body(&mut self, nodes: &[Node]) -> Result<LLVMValue<'ctx>, &'static str> {
        let mut result: Option<LLVMValue<'ctx>> = None;
        for node in nodes {
            result = Some(self.gen_expr(node)?);

            if let Node::ReturnExpr(_) = node {
                return Ok(result.unwrap());
            }
        };
        Ok(result.unwrap_or(LLVMValue::Float(self.context.f64_type().const_float(0.0))))
    }

    pub fn gen_expr(&mut self, node: &Node) -> Result<LLVMValue<'ctx>, &'static str> {
            match node {
                Node::Number(n) => {
                    return Ok(self.context.f64_type().const_float(n.0).into());
                }
                Node::BinaryExpr(e) => {
                    let lhs = self.gen_body(&e.lhs)?.as_float().expect("Expected float value. Comparisons cannot be used for operations");
                    let rhs = self.gen_body(&e.rhs)?.as_float().expect("Expected float value. Comparisons cannot be used for operations");

                    match e.op {
                        Op::Add => {
                            return Ok(LLVMValue::Float(self.builder.build_float_add(lhs, rhs, "addtmp")));
                        }
                        Op::Sub => {
                            return Ok(LLVMValue::Float(self.builder.build_float_sub(lhs, rhs, "subtmp")));
                        }
                        Op::Mul => {
                            return Ok(LLVMValue::Float(self.builder.build_float_mul(lhs, rhs, "multmp")));
                        }
                        Op::Div => {
                            return Ok(LLVMValue::Float(self.builder.build_float_div(lhs, rhs, "divtmp")));
                        }
                        Op::Mod => {
                            return Ok(LLVMValue::Float(self.builder.build_float_rem(lhs, rhs, "modtmp")));
                        }
                        Op::Gt => {
                            return Ok(LLVMValue::Int(self.builder.build_float_compare(inkwell::FloatPredicate::OGT, lhs, rhs, "gttmp")));
                        }
                        Op::Lt => {
                            return Ok(LLVMValue::Int(self.builder.build_float_compare(inkwell::FloatPredicate::OLT, lhs, rhs, "lttmp")));
                        }
                        Op::Eqt => {
                            return Ok(LLVMValue::Int(self.builder.build_float_compare(inkwell::FloatPredicate::OEQ, lhs, rhs, "eqttmp")));
                        }
                    }
                }
                Node::BindExpr(e) => {
                    let value = self.gen_body(&e.value)?.as_float().expect("Expected float value");
                
                    let f64_type = self.context.f64_type();
                    let alloca = self.builder.build_alloca(f64_type, &e.name.as_str());
                    self.builder.build_store(alloca, value);
                
                    self.variables
                        .last_mut()
                        .expect("No variable scopes found")
                        .insert(e.name.to_string(), alloca);
                }
                Node::Variable(name) => {
                    let f64_type = self.context.f64_type();
                    let alloca = self.variables
                        .last()
                        .expect("No variable scopes found")
                        .get(name)
                        .expect(format!("Variable '{}' not found!", name).as_str());
                
                    let loaded_value = self.builder.build_load(f64_type, *alloca, &name);
                
                    return Ok(LLVMValue::Float(loaded_value.into_float_value()));
                }
                 
                Node::ReturnExpr(e) => {
                    let value = self.gen_body(&e.value)?.as_float().expect("Expected float value. Comparisons cannot be used for operations");

                    self.builder.build_return(Some(&value));
                    return Ok(LLVMValue::Float(value));
                }
                Node::MutateExpr(e) => {
                    let value = self.gen_body(&e.value)?.as_float().expect("Expected float value. Comparisons cannot be used for operations");
                    let alloca = self.variables
                                    .last()             
                                    .expect("No variable scopes found")
                                    .get(&e.name)
                    .expect(format!("Variable '{}' not found to mutate!", e.name).as_str());

                    self.builder.build_store(*alloca, value);
                }
                Node::WhileExpr(e) => {
                    let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                
                    let loop_cond_bb = self.context.append_basic_block(function, "loop_cond");
                    let loop_body_bb = self.context.append_basic_block(function, "loop_body");
                    let loop_end_bb = self.context.append_basic_block(function, "loop_end");
                
                    // Start from the current position (should be the end of the entry block or the previous block)
                    self.builder.build_unconditional_branch(loop_cond_bb);
                
                    // Now, handle the loop condition
                    self.builder.position_at_end(loop_cond_bb);
                    let cond = self.gen_body(&e.condition)?.as_int().expect("Expected int value. Other operations cannot be used for comparisons");
                    self.builder.build_conditional_branch(cond, loop_body_bb, loop_end_bb);
                
                    // Generate the loop body
                    self.builder.position_at_end(loop_body_bb);
                    for node in e.body.iter() {
                        self.gen_expr(node)?;
                    }
                    self.builder.build_unconditional_branch(loop_cond_bb);
                
                    // Position builder at the end block after the loop
                    self.builder.position_at_end(loop_end_bb);
                }                
                Node::IfExpr(e) => {
                    let function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                
                    let if_cond_bb = self.context.append_basic_block(function, "if_cond");
                    let then_bb = self.context.append_basic_block(function, "then_block");
                    let else_bb = if !e.else_body.is_empty() {
                        Some(self.context.append_basic_block(function, "else_block"))
                    } else {
                        None
                    };

                    let end_if_bb = self.context.append_basic_block(function, "end_if");

                    // Start from the current position (should be the end of the entry block or the previous block)
                    self.builder.build_unconditional_branch(if_cond_bb);
                
                    // Evaluate the condition
                    self.builder.position_at_end(if_cond_bb);
                    let cond = self.gen_body(&e.condition)?.as_int().expect("Expected int value. Other operations cannot be used for comparisons");
                    
                    match else_bb {
                        Some(else_block) => {
                            self.builder.build_conditional_branch(cond, then_bb, else_block);
                        },
                        None => {
                            self.builder.build_conditional_branch(cond, then_bb, end_if_bb);
                        }
                    }
                
                    // Generate then block
                    self.builder.position_at_end(then_bb);
                    for node in e.body.iter() {
                        self.gen_expr(node)?;
                    }
                    self.builder.build_unconditional_branch(end_if_bb);
                
                    // Generate else block if it exists
                    if else_bb.is_some() {
                        self.builder.position_at_end(else_bb.unwrap());
                        for node in e.else_body.iter() {
                            self.gen_expr(node)?;
                        }
                        self.builder.build_unconditional_branch(end_if_bb); 
                    }
                
                    // Position builder at the end block after the if statement
                    self.builder.position_at_end(end_if_bb);
                }
                Node::FnExpr(e) => {
                    // Save the current block so we can restore it later.
                    let current_block = self.builder.get_insert_block().unwrap();

                    let function = self.compile_prototype(e)?;

                    // got external function, returning only compiled prototype
                    // if self.function.body.is_none() {
                    //     return Ok(function);
                    // }

                    let entry = self.context.append_basic_block(function, "entry");

                    self.builder.position_at_end(entry);

                    self.fn_value_opt = Some(function);

                    // build variables map
                    self.variables.push(HashMap::new());
                    self.variables.reserve(e.args.len());

                    // all paramters will be mutable by default
                    // so we need to create alloca for each of them
                    for (i, arg) in function.get_param_iter().enumerate() {
                        let arg_name = if let Node::Variable(name) = &e.args[i] {
                            name
                        } else {
                            panic!("Expected variable name")
                        };
                        let alloca = self.create_entry_block_alloca(arg_name);

                        self.builder.build_store(alloca, arg);

                        self.variables
                            .last_mut()
                            .expect("No variable scopes found")
                            .insert(arg_name.to_string(), alloca);
                    }

                    // compile body
                    let _body = self.gen_body(&e.body)?;

                    self.builder.position_at_end(current_block);
                    self.variables.pop();

                    // return the whole thing after verification and optimization
                    // if function.verify(true) {
                    //     self.fpm.run_on(&function);

                    //     // return Ok(function)
                    // } else {
                    //     unsafe {
                    //         function.delete();
                    //     }

                    //     return Err("Invalid generated function.")
                    // }

                }
                Node::FnCallExpr(e) => {
                    let mut compiled_args = Vec::with_capacity(e.args.len());

                    for arg in &e.args {
                        compiled_args.push(self.gen_expr(&arg)?.as_float().unwrap());
                    }

                    let argsv: Vec<BasicMetadataValueEnum> =
                        compiled_args.iter().by_ref().map(|&val| val.into()).collect();

                    let function = self.module.get_function(&e.name).expect("Function not found");

                    match self
                        .builder
                        .build_call(function, argsv.as_slice(), "tmp")
                        .try_as_basic_value()
                        .left()
                    {
                        Some(value) => return Ok(LLVMValue::Float(value.into_float_value())),
                        None => return Err("Invalid call produced."),
                    };
                }  
                Node::PrintStdoutExpr(_e) => {
                    todo!("Return expression")
                }
            }
            Ok(LLVMValue::Float(self.context.f64_type().const_float(0.0)))
        }

        #[inline]
        fn fn_value(&self) -> FunctionValue<'ctx> {
            self.fn_value_opt.unwrap()
        }

        fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
            let builder = self.context.create_builder();
    
            let entry = self.fn_value().get_first_basic_block().unwrap();
    
            match entry.get_first_instruction() {
                Some(first_instr) => builder.position_before(&first_instr),
                None => builder.position_at_end(entry),
            }
    
            builder.build_alloca(self.context.f64_type(), name)
        }

        fn compile_prototype(&mut self, proto: &FnExpr) -> Result<FunctionValue<'ctx>, &'static str> {
            let ret_type = self.context.f64_type();
            let args_types = std::iter::repeat(ret_type)
                .take(proto.args.len())
                .map(|f| f.into())
                .collect::<Vec<BasicMetadataTypeEnum>>();
            let args_types = args_types.as_slice();
    
            let fn_type = self.context.f64_type().fn_type(args_types, false);
            let fn_val = self.module.add_function(proto.name.as_str(), fn_type, None);
    
            // set arguments names
            for (i, arg) in fn_val.get_param_iter().enumerate() {
                let name = if let Node::Variable(name) = &proto.args[i] {
                    name
                } else {
                    panic!("Expected variable name")
                };
                arg.set_name(name);
            }            
    
            // finally return built prototype
            Ok(fn_val)
    }
}

impl Compile for LLVMCompiler<'_, '_> {
    type Output = Result<f64, &'static str>;

    fn from_ast(nodes: Vec<Node>, config: &CompileConfig) -> Self::Output {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("main");
        let fpm = PassManager::create(&module);

        let mut compiler = LLVMCompiler::new(&context, &builder, &module, &fpm);

        compiler.codegen(nodes).expect("Failed to generate IR");

        if config.show_ir {
            let ir = module.print_to_string();

            println!("\n{}\n", ir);
        }

        if config.use_jit {
            Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");

            let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::Aggressive).expect("Failed to create JIT execution engine");

            let main_func = unsafe { execution_engine.get_function::<unsafe extern "C" fn() -> f64>("main").expect("Failed to get main function") };
            let result = unsafe { main_func.call() };
            return Ok(result);
        }

        let path = Path::new("output.ll");
        module.print_to_file(&path).expect("Error writing file");

        Ok(0.0)
    }
}