//! Compiler pipeline orchestration

use std::fs;
use std::path::PathBuf;

use backend_zealz80::{encode_instructions, CodeGenerator, Z80Instruction};
use errors::{Diagnostic, ErrorSeverity};
use ir::{IRBuilder, Program};
use object_zealz80::{ObjectFile, Section, Symbol, SymbolType, SymbolVisibility};
use parser::Parser;
use runtime_spec::{capabilities, TargetPlatform};
use semantics::feature_checker;
use semantics::SemanticAnalyzer;

/// Output format for compiled files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Raw binary machine code
    Binary,
    /// Z80 object file with symbols and metadata
    Object,
}

/// Compiler instance that orchestrates the compilation pipeline
pub struct Compiler {
    target: TargetPlatform,
    check_features: bool, // Whether to check feature compatibility
}

impl Compiler {
    /// Create a new compiler instance with default target (ZealZ80)
    pub fn new() -> Self {
        Self {
            target: TargetPlatform::ZealZ80,
            check_features: true,
        }
    }

    /// Create a new compiler instance for a specific target platform
    #[allow(dead_code)] // Public API method
    pub fn new_with_target(target: TargetPlatform) -> Self {
        Self {
            target,
            check_features: true,
        }
    }

    /// Create a compiler instance with feature checking disabled
    #[allow(dead_code)] // Public API method
    pub fn new_without_feature_check(target: TargetPlatform) -> Self {
        Self {
            target,
            check_features: false,
        }
    }

    /// Get the current target platform
    #[allow(dead_code)] // Public API method
    pub fn target(&self) -> TargetPlatform {
        self.target
    }

    /// Set the target platform
    #[allow(dead_code)] // Public API method
    pub fn set_target(&mut self, target: TargetPlatform) {
        self.target = target;
    }

    /// Enable or disable feature checking
    #[allow(dead_code)] // Public API method
    pub fn set_feature_checking(&mut self, enabled: bool) {
        self.check_features = enabled;
    }

    /// Compile a Pascal source file to an object or binary file
    pub fn compile_file(
        &mut self,
        input_file: &str,
        output_file: Option<&str>,
    ) -> Result<(), String> {
        // Read source file
        let source = fs::read_to_string(input_file)
            .map_err(|e| format!("Failed to read file '{}': {}", input_file, e))?;

        // Parse source to AST
        let (program, diagnostics) = self.compile_source(&source, Some(input_file.to_string()))?;

        // Check for errors
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == ErrorSeverity::Error)
            .collect();

        if !errors.is_empty() {
            return Err(format!(
                "Type checking failed with {} error(s)",
                errors.len()
            ));
        }

        // Generate code and track function byte offsets
        let mut codegen = CodeGenerator::new();
        let instructions = codegen.generate(&program);

        // Convert Z80 instructions to machine code bytes
        let code_bytes = self.instructions_to_bytes(&instructions)?;

        // Determine output format from extension
        let output_path = output_file
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.default_output_file(input_file));
        let format = self.output_format(&output_path);

        match format {
            OutputFormat::Binary => {
                // Write raw binary machine code
                let mut file = fs::File::create(&output_path).map_err(|e| {
                    format!("Failed to create output file '{}': {}", output_path, e)
                })?;
                std::io::Write::write_all(&mut file, &code_bytes)
                    .map_err(|e| format!("Failed to write binary file: {}", e))?;
                println!("Generated: {} ({} bytes)", output_path, code_bytes.len());
                println!("Compilation successful");
            }
            OutputFormat::Object => {
                // Create object file with symbols and metadata
                let unit_name = self.extract_unit_name(input_file);
                let mut obj_file = ObjectFile::new(unit_name);
                obj_file.add_code(&code_bytes);

                // Calculate symbol offsets by scanning for function labels
                let mut current_offset: u16 = 0;
                let mut label_positions: Vec<(String, u16)> = Vec::new();

                for inst in &instructions {
                    if let Z80Instruction::Label { name } = inst {
                        label_positions.push((name.clone(), current_offset));
                    }
                    current_offset += codegen.instruction_size(inst) as u16;
                }

                // Add symbols for each function
                for (i, function) in program.functions.iter().enumerate() {
                    let mangled_name = format!("_{}", function.name);
                    let offset = label_positions
                        .iter()
                        .find(|(name, _)| *name == mangled_name)
                        .map(|(_, off)| *off)
                        .unwrap_or(0);

                    let size = if i + 1 < label_positions.len() {
                        let next_offset = label_positions[i + 1].1;
                        next_offset.saturating_sub(offset)
                    } else {
                        (code_bytes.len() as u16).saturating_sub(offset)
                    };

                    obj_file.add_symbol(Symbol {
                        name: function.name.clone(),
                        symbol_type: SymbolType::Function,
                        visibility: SymbolVisibility::Public,
                        section: Section::Code,
                        offset,
                        size,
                    });
                }

                // Add data section (empty for now, reserved for globals/constants)
                obj_file.add_data(&[]);

                // Write object file
                let mut file = fs::File::create(&output_path).map_err(|e| {
                    format!("Failed to create output file '{}': {}", output_path, e)
                })?;
                obj_file
                    .write(&mut file)
                    .map_err(|e| format!("Failed to write object file: {}", e))?;
                println!("Generated: {}", output_path);
                println!("Compilation successful");
            }
        }

        Ok(())
    }

    /// Extract unit name from source filename
    fn extract_unit_name(&self, filename: &str) -> String {
        PathBuf::from(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("main")
            .to_string()
    }

    /// Generate default output filename
    fn default_output_file(&self, input_file: &str) -> String {
        PathBuf::from(input_file)
            .with_extension("zof")
            .to_string_lossy()
            .to_string()
    }

    /// Determine output format from file extension
    fn output_format(&self, path: &str) -> OutputFormat {
        match path.to_lowercase().split('.').last() {
            Some("bin") => OutputFormat::Binary,
            Some("zof") => OutputFormat::Object,
            _ => OutputFormat::Object,
        }
    }

    /// Convert Z80 instructions to bytes using the Z80 opcode encoder
    fn instructions_to_bytes(&self, instructions: &[Z80Instruction]) -> Result<Vec<u8>, String> {
        Ok(encode_instructions(instructions))
    }

    /// Type check an AST, returning errors
    #[allow(dead_code)] // Public API method
    pub fn type_check(&self, source: &str) -> Result<(), String> {
        let mut parser =
            Parser::new_with_file(source, None).map_err(|e| format!("Parse error: {}", e))?;
        let ast = parser.parse().map_err(|e| {
            let diag = parser.error_to_diagnostic(&e);
            format!("Parse error: {}", diag)
        })?;

        // Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new(None);
        let diagnostics = analyzer.analyze(&ast);

        // Type checking
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == ErrorSeverity::Error)
            .collect();

        if !errors.is_empty() {
            let msg: Vec<_> = errors.iter().map(|d| d.message.clone()).collect();
            return Err(format!(
                "Type checking failed with {} error(s): {}",
                errors.len(),
                msg.join("; ")
            ));
        }

        Ok(())
    }

    /// Emit assembly listing for a source file (debugging aid)
    #[allow(dead_code)] // Public API method
    pub fn emit_asm(&self, source: &str) -> Result<String, String> {
        let mut parser =
            Parser::new_with_file(source, None).map_err(|e| format!("Parse error: {}", e))?;
        let ast = parser.parse().map_err(|e| {
            let diag = parser.error_to_diagnostic(&e);
            format!("Parse error: {}", diag)
        })?;

        // Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new(None);
        let _diagnostics = analyzer.analyze(&ast);

        // IR Generation
        let mut ir_builder = IRBuilder::new();
        ir_builder.start_function("main".to_string(), None);
        ir_builder.build(&ast);
        ir_builder.finish_function();
        let program = ir_builder.into_program();

        // Code Generation
        let mut codegen = CodeGenerator::new();
        let instructions = codegen.generate(&program);

        // Format as assembly listing
        let mut asm = String::new();
        for inst in &instructions {
            asm.push_str(&format!("{}\n", inst));
        }

        Ok(asm)
    }

    /// Type check a Pascal source file
    pub fn check_file(&self, filename: &str) -> Result<(), String> {
        let source = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        self.type_check(&source)
    }

    /// Emit AST from a Pascal source file
    pub fn emit_ast(&self, filename: &str) -> Result<(), String> {
        let source = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        let mut parser = Parser::new_with_file(&source, Some(filename.to_string()))
            .map_err(|e| format!("Parse error: {}", e))?;
        let ast = parser.parse().map_err(|e| {
            let diag = parser.error_to_diagnostic(&e);
            format!("Parse error: {}", diag)
        })?;
        println!("{:#?}", ast);
        Ok(())
    }

    /// Emit IR from a Pascal source file
    pub fn emit_ir(&self, filename: &str) -> Result<(), String> {
        let source = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        let mut parser = Parser::new_with_file(&source, Some(filename.to_string()))
            .map_err(|e| format!("Parse error: {}", e))?;
        let ast = parser.parse().map_err(|e| {
            let diag = parser.error_to_diagnostic(&e);
            format!("Parse error: {}", diag)
        })?;
        let mut analyzer = SemanticAnalyzer::new(Some(filename.to_string()));
        let _diagnostics = analyzer.analyze(&ast);
        let mut ir_builder = IRBuilder::new();
        ir_builder.start_function("main".to_string(), None);
        ir_builder.build(&ast);
        ir_builder.finish_function();
        let program = ir_builder.into_program();
        println!("{:#?}", program);
        Ok(())
    }

    /// Emit assembly from a Pascal source file
    pub fn emit_assembly(&self, filename: &str) -> Result<(), String> {
        let source = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
        let asm = self.emit_asm(&source)?;
        print!("{}", asm);
        Ok(())
    }

    /// Core compilation pipeline
    fn compile_source(
        &mut self,
        source: &str,
        filename: Option<String>,
    ) -> Result<(Program, Vec<Diagnostic>), String> {
        // 1. Parsing (parser has its own lexer)
        let mut parser = Parser::new_with_file(source, filename.clone())
            .map_err(|e| format!("Parse error: {}", e))?;
        let ast = parser.parse().map_err(|e| {
            let diag = parser.error_to_diagnostic(&e);
            format!("Parse error: {}", diag)
        })?;

        // 3. Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new(filename.clone());
        let mut diagnostics = analyzer.analyze(&ast);

        // 4. Feature Compatibility Checking
        if self.check_features {
            let capabilities = capabilities::get_capabilities(self.target);
            let mut feature_checker = feature_checker::FeatureChecker::new(capabilities, filename);
            feature_checker.check(&ast);
            diagnostics.extend_from_slice(feature_checker.diagnostics());
        }

        // 5. IR Generation
        let mut ir_builder = IRBuilder::new();
        // Start the main function/program
        ir_builder.start_function("main".to_string(), None);
        // Build IR from AST
        ir_builder.build(&ast);
        ir_builder.finish_function();
        let program = ir_builder.into_program();

        Ok((program, diagnostics))
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backend_zealz80::Z80Register;

    /// Helper to run the full pipeline: source -> parse -> sem -> IR -> code gen -> bytes
    fn compile_to_bytes(source: &str) -> Result<(Vec<u8>, Vec<Z80Instruction>), String> {
        // 1. Parse
        let mut parser =
            Parser::new_with_file(source, None).map_err(|e| format!("Parse error: {}", e))?;
        let ast = parser.parse().map_err(|e| {
            let diag = parser.error_to_diagnostic(&e);
            format!("Parse error: {}", diag)
        })?;

        // 2. Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new(None);
        let diagnostics = analyzer.analyze(&ast);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == ErrorSeverity::Error)
            .collect();
        if !errors.is_empty() {
            return Err(format!("Type checking failed: {:?}", errors));
        }

        // 3. IR Generation
        let mut ir_builder = IRBuilder::new();
        ir_builder.start_function("main".to_string(), None);
        ir_builder.build(&ast);
        ir_builder.finish_function();
        let program = ir_builder.into_program();

        // 4. Code Generation
        let mut codegen = CodeGenerator::new();
        let instructions = codegen.generate(&program);

        // 5. Encode to bytes
        let bytes = encode_instructions(&instructions);

        Ok((bytes, instructions))
    }

    #[test]
    fn test_full_pipeline_simple_assignment() {
        let source = "
PROGRAM Main;
VAR X: INTEGER;
BEGIN
  X := 10
END.
";
        let (bytes, _instructions) = compile_to_bytes(source).unwrap();
        assert!(!bytes.is_empty(), "Should produce non-empty machine code");
    }

    #[test]
    fn test_full_pipeline_multiple_variables() {
        let source = "
PROGRAM Main;
VAR A, B, C: INTEGER;
BEGIN
  A := 10;
  B := 20;
  C := A + B
END.
";
        let (bytes, _instructions) = compile_to_bytes(source).unwrap();
        assert!(!bytes.is_empty(), "Should produce machine code");
    }

    #[test]
    fn test_full_pipeline_with_function() {
        let source = "
PROGRAM Main;
VAR X: INTEGER;
FUNCTION Double(N: INTEGER): INTEGER;
BEGIN
  N := N + N
END;
BEGIN
  X := 1;
END.
";
        let (bytes, _instructions) = compile_to_bytes(source).unwrap();
        assert!(!bytes.is_empty(), "Should produce machine code");
    }

    #[test]
    fn test_parse_error_no_panic() {
        let source = "PROGRAM Bad; VAR X; BEGIN END.";
        let result = std::panic::catch_unwind(|| {
            let mut parser = Parser::new_with_file(source, None);
            parser.map(|mut p| p.parse())
        });
        assert!(result.is_ok(), "Parser should not panic on bad input");
    }

    #[test]
    fn test_empty_program() {
        let source = "PROGRAM Empty;\nBEGIN\nEND.\n";
        let (bytes, _instructions) = compile_to_bytes(source).unwrap();
        assert!(!bytes.is_empty(), "Should produce at least minimal code");
    }

    #[test]
    fn test_type_checking_detects_bad_assignment() {
        let source = "
PROGRAM Bad;
VAR X: INTEGER;
    S: STRING;
BEGIN
  X := S
END.
";
        let result = compile_to_bytes(source);
        assert!(result.is_err(), "Should detect type mismatch");
    }

    #[test]
    fn test_instruction_encoding_real_opcodes() {
        let instructions = vec![
            Z80Instruction::Label {
                name: "_main".to_string(),
            },
            Z80Instruction::Push {
                reg: Z80Register::IX,
            },
            Z80Instruction::LoadRegister {
                dst: Z80Register::IX,
                src: Z80Register::SP,
            },
            Z80Instruction::LoadImmediate {
                reg: Z80Register::HL,
                value: 10,
            },
            Z80Instruction::Return,
        ];

        let bytes = encode_instructions(&instructions);
        assert!(!bytes.is_empty());

        // RET = 0xC9
        assert_eq!(bytes[bytes.len() - 1], 0xC9, "Should end with RET opcode");

        // PUSH IX = 0xDD 0xE1
        assert_eq!(&bytes[0..2], &[0xDD, 0xE1], "Should start with PUSH IX");
    }

    #[test]
    fn test_code_generator_produces_label() {
        let source = "
PROGRAM Main;
VAR X: INTEGER;
BEGIN
  X := 42
END.
";
        let (_, instructions) = compile_to_bytes(source).unwrap();
        assert!(
            instructions.iter().any(
                |inst| matches!(inst, Z80Instruction::Label { name } if name.starts_with('_'))
            ),
            "Code should contain function labels"
        );
    }

    #[test]
    fn test_compile_file_produces_bin_output() {
        use std::io::{Read, Write};

        let source = "\
PROGRAM TestBin;
VAR X: INTEGER;
BEGIN
  X := 10;
END.
";
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("spc_test_bin.pas");
        let mut src_file = fs::File::create(&src_path).unwrap();
        src_file.write_all(source.as_bytes()).unwrap();
        drop(src_file);

        let bin_path = temp_dir.join("spc_test_out.bin");
        let bin_path_str = bin_path.to_str().unwrap().to_string();
        let mut compiler = Compiler::new_without_feature_check(TargetPlatform::ZealZ80);
        compiler
            .compile_file(src_path.to_str().unwrap(), Some(&bin_path_str))
            .unwrap();

        let mut file = fs::File::open(&bin_path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        assert!(!bytes.is_empty(), "Binary file should contain machine code");

        fs::remove_file(&bin_path).ok();
        fs::remove_file(&src_path).ok();
    }

    #[test]
    fn test_compile_file_produces_zof_output() {
        use object_zealz80::ZOF_MAGIC;
        use std::io::{Read, Write};

        let source = "\
PROGRAM TestZOF;
VAR X: INTEGER;
BEGIN
  X := 42;
END.
";
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("spc_test_zof_src.pas");
        let mut src_file = fs::File::create(&src_path).unwrap();
        src_file.write_all(source.as_bytes()).unwrap();
        drop(src_file);

        let zof_path = temp_dir.join("spc_test_zof_out.zof");
        let zof_path_str = zof_path.to_str().unwrap().to_string();
        let mut compiler = Compiler::new_without_feature_check(TargetPlatform::ZealZ80);
        compiler
            .compile_file(src_path.to_str().unwrap(), Some(&zof_path_str))
            .unwrap();

        let mut file = fs::File::open(&zof_path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        assert!(!bytes.is_empty(), "Object file should not be empty");
        assert!(
            bytes.starts_with(ZOF_MAGIC),
            "File should start with ZOF magic number {:?}, got {:?}",
            ZOF_MAGIC,
            &bytes[..4]
        );

        let mut cursor = std::io::Cursor::new(bytes);
        let obj = object_zealz80::ObjectFile::read(&mut cursor).unwrap();
        assert!(!obj.code.is_empty(), "Object file should contain code");

        fs::remove_file(&zof_path).ok();
        fs::remove_file(&src_path).ok();
    }

    #[test]
    fn test_compile_file_default_output_is_zof() {
        let source = "\
PROGRAM DefaultExt;
VAR A: INTEGER;
BEGIN
  A := 1
END.
";
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("spc_test_default.pas");
        fs::write(&src_path, source).unwrap();

        let mut compiler = Compiler::new_without_feature_check(TargetPlatform::ZealZ80);
        compiler
            .compile_file(src_path.to_str().unwrap(), None)
            .unwrap();

        let default_path = temp_dir.join("spc_test_default.zof");
        assert!(
            fs::metadata(&default_path).is_ok(),
            "Default output file should be .zof"
        );
        fs::remove_file(&default_path).ok();
        fs::remove_file(&src_path).ok();
    }

    #[test]
    fn test_compile_file_detects_errors() {
        let source = "\
PROGRAM BadType;
VAR X: INTEGER;
BEGIN
  X := \"hello\"
END.
";
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("spc_test_error.pas");
        fs::write(&src_path, source).unwrap();

        let mut compiler = Compiler::new_without_feature_check(TargetPlatform::ZealZ80);
        let result = compiler.compile_file(src_path.to_str().unwrap(), None);
        assert!(result.is_err(), "Should fail on type error");

        fs::remove_file(&src_path).ok();
    }

    #[test]
    fn test_bin_and_zof_produce_same_code() {
        use object_zealz80::ZOF_MAGIC;
        use std::io::{Read, Write};

        let source = "\
PROGRAM SameCode;
VAR X: INTEGER;
BEGIN
  X := 100
END.
";
        let temp_dir = std::env::temp_dir();
        let src_path = temp_dir.join("spc_test_compare.pas");
        let mut src_file = fs::File::create(&src_path).unwrap();
        src_file.write_all(source.as_bytes()).unwrap();
        drop(src_file);

        // Compile to .bin
        let bin_path = temp_dir.join("spc_test_compare.bin");
        let bin_path_str = bin_path.to_str().unwrap().to_string();
        let mut compiler = Compiler::new_without_feature_check(TargetPlatform::ZealZ80);
        compiler
            .compile_file(src_path.to_str().unwrap(), Some(&bin_path_str))
            .unwrap();

        let mut bin_file = fs::File::open(&bin_path).unwrap();
        let mut bin_bytes = Vec::new();
        bin_file.read_to_end(&mut bin_bytes).unwrap();

        // Compile to .zof
        let zof_path = temp_dir.join("spc_test_compare.zof");
        let zof_path_str = zof_path.to_str().unwrap().to_string();
        let mut compiler2 = Compiler::new_without_feature_check(TargetPlatform::ZealZ80);
        compiler2
            .compile_file(src_path.to_str().unwrap(), Some(&zof_path_str))
            .unwrap();

        let mut zof_file = fs::File::open(&zof_path).unwrap();
        let mut zof_bytes = Vec::new();
        zof_file.read_to_end(&mut zof_bytes).unwrap();

        let mut cursor = std::io::Cursor::new(zof_bytes);
        let obj = object_zealz80::ObjectFile::read(&mut cursor).unwrap();

        assert_eq!(
            bin_bytes, obj.code,
            "Binary and ZOF code sections should match"
        );

        fs::remove_file(&bin_path).ok();
        fs::remove_file(&zof_path).ok();
        fs::remove_file(&src_path).ok();
    }
}
