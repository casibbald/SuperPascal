//! SuperPascal Compiler Driver
//!
//! This is the main entry point for the SuperPascal compiler.
//! It orchestrates the compilation pipeline:
//! 1. Lexical Analysis (lexer)
//! 2. Parsing (parser)
//! 3. Semantic Analysis (semantics)
//! 4. IR Generation (ir)
//! 5. Code Generation (backend)
//! 6. Object File Generation (object-zealz80)

use std::env;
use std::process;

mod compiler;

use compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let mut compiler = Compiler::new();

    match command.as_str() {
        "build" | "compile" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage();
                process::exit(1);
            }
            let input_file = &args[2];
            let output_file = args.get(3).map(|s| s.as_str());
            
            match compiler.compile_file(input_file, output_file) {
                Ok(_) => {
                    println!("Compilation successful");
                }
                Err(e) => {
                    eprintln!("Compilation failed: {}", e);
                    process::exit(1);
                }
            }
        }
        "check" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage();
                process::exit(1);
            }
            let input_file = &args[2];
            
            match compiler.check_file(input_file) {
                Ok(_) => {
                    println!("Type checking successful");
                }
                Err(e) => {
                    eprintln!("Type checking failed: {}", e);
                    process::exit(1);
                }
            }
        }
        "emit-ast" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage();
                process::exit(1);
            }
            let input_file = &args[2];
            
            match compiler.emit_ast(input_file) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to emit AST: {}", e);
                    process::exit(1);
                }
            }
        }
        "emit-ir" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage();
                process::exit(1);
            }
            let input_file = &args[2];
            
            match compiler.emit_ir(input_file) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to emit IR: {}", e);
                    process::exit(1);
                }
            }
        }
        "asm" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage();
                process::exit(1);
            }
            let input_file = &args[2];
            
            match compiler.emit_assembly(input_file) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to emit assembly: {}", e);
                    process::exit(1);
                }
            }
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("SuperPascal Compiler (spc)");
    println!();
    println!("Usage: spc <command> [options] <file>");
    println!();
    println!("Commands:");
    println!("  build, compile <file> [output]  Compile Pascal source to object file");
    println!("  check <file>                    Type check only (no code generation)");
    println!("  emit-ast <file>                 Emit AST (for debugging)");
    println!("  emit-ir <file>                  Emit IR (for debugging)");
    println!("  asm <file>                      Emit assembly code");
    println!("  help                            Show this help message");
    println!();
    println!("Examples:");
    println!("  spc build program.pas");
    println!("  spc check program.pas");
    println!("  spc emit-ast program.pas");
    println!("  spc asm program.pas");
}
