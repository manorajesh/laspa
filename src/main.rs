use clap::Parser;
use env_logger::Builder;
use laspa::{CompileConfig, Interpreter, Compile, Compiler};
use log::LevelFilter;

mod args;

fn main() {
    let args = args::Args::parse();

    // Map verbosity count to log level
    let log_level = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4 | _ => LevelFilter::Trace,  // 4 and above are trace
    };

    // Set up logging
    Builder::new()
        .filter(None, log_level)
        .default_format()
        .init();
    
    if args.optimization_level > 3 {
        log::error!("Error: optimization_level should be between 0 (none) and 3 (aggressive).");
        return
    }

    let config = CompileConfig {
        use_jit: args.jit,
        optimization_level: args.optimization_level,
        show_ir: true,
        name: args.executable_name,
    };

    if args.interpret {
        log::info!("Interpreting file {}", args.file);
        let result = Interpreter::from_file(&args.file, &config);
        log::trace!("Result: {:?}", result);
    } else {
        log::info!("Compiling file {}", args.file);
        let result = Compiler::from_file(&args.file, &config);
        if let Err(e) = result {
            log::error!("Error: {}", e);
        }
    }
}