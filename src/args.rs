use clap::Parser;
use clap::ValueHint;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "A simple Lisp-like language built with Rust",
    long_about = "A simple Lisp-like language built with Rust. It is a toy language and is not meant to be used in production, but it features JIT and AOT compilation with LLVM"
)]
pub struct Args {
    /// The file to build
    #[clap(value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: String,

    /// Optimization level for the compiler
    #[clap(short = 'O', long, default_value = "1")]
    pub optimization_level: u8,

    /// Produce an executable file
    #[clap(short, long, default_value = "true")]
    pub executable: bool,

    /// Interpret the file
    #[clap(short, long)]
    pub interpret: bool,

    /// Verbose output
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Executable name
    #[clap(short = 'o', long, default_value = "main")]
    pub executable_name: String,

    /// Execute IR with JIT
    #[clap(long)]
    pub jit: bool,
}
