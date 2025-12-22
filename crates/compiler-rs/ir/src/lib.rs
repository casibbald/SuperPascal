//! SuperPascal Intermediate Representation (IR)
//!
//! This crate defines a platform-agnostic intermediate representation
//! for SuperPascal programs. The IR is designed to be:
//! - Simple and linear (IR1)
//! - Platform-independent
//! - Easy to optimize
//! - Easy to translate to target assembly

use ast::Node;
use tokens::Span;
use types::Type;
use runtime::variant::VariantType as RuntimeVariantType;

/// Represents an IR value (immediate, register, memory, temporary)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// Immediate constant value
    Immediate(i32),
    /// Register (platform-specific, but abstracted here)
    Register(String),
    /// Memory location (address)
    Memory { base: String, offset: i32 },
    /// Temporary value (SSA-style)
    Temp(usize),
    /// Label reference
    Label(String),
}

/// IR instruction opcodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    // Data movement
    Mov,  // MOV dst, src
    // Arithmetic
    Add,  // ADD dst, src1, src2
    Sub,  // SUB dst, src1, src2
    Mul,  // MUL dst, src1, src2
    Div,  // DIV dst, src1, src2
    Mod,  // MOD dst, src1, src2
    // Comparison
    Cmp,  // CMP src1, src2 (sets condition flags)
    // Control flow
    Jump,   // JUMP label
    CJump,  // CJUMP condition, label_true, label_false
    Call,   // CALL function, result
    Ret,    // RET value (optional)
    // Memory operations
    Load,   // LOAD dst, src (load from memory)
    Store,  // STORE dst, src (store to memory)
    // Stack operations
    Push,   // PUSH src
    Pop,    // POP dst
}

/// Condition codes for conditional jumps
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
}

/// Represents a single IR instruction
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Value>,
    pub span: Option<Span>, // Source location for debugging
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<Value>) -> Self {
        Self {
            opcode,
            operands,
            span: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
}

/// Represents a basic block in the IR
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub successors: Vec<String>, // Labels of successor blocks
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label,
            instructions: vec![],
            successors: vec![],
        }
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    pub fn add_successor(&mut self, label: String) {
        if !self.successors.contains(&label) {
            self.successors.push(label);
        }
    }
}

/// Represents a complete IR function/procedure
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>, // (name, type)
    pub return_type: Option<Type>,
    pub blocks: Vec<BasicBlock>,
    pub entry_block: String, // Label of entry block
}

impl Function {
    pub fn new(name: String, return_type: Option<Type>) -> Self {
        let entry_label = format!("{}_entry", name);
        let entry_block = BasicBlock::new(entry_label.clone());
        
        Self {
            name,
            params: vec![],
            return_type,
            blocks: vec![entry_block],
            entry_block: entry_label,
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    pub fn get_block_mut(&mut self, label: &str) -> Option<&mut BasicBlock> {
        self.blocks.iter_mut().find(|b| b.label == label)
    }
}

/// Represents a complete IR program
#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    pub globals: Vec<(String, Type)>, // (name, type)
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            globals: vec![],
        }
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

/// IR builder for constructing IR from AST
pub struct IRBuilder {
    program: Program,
    current_function: Option<Function>,
    temp_counter: usize,
    label_counter: usize,
    /// Variable type information (name -> type)
    /// Used to determine when to use Variant runtime functions
    variable_types: std::collections::HashMap<String, Type>,
}

impl IRBuilder {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            current_function: None,
            temp_counter: 0,
            label_counter: 0,
            variable_types: std::collections::HashMap::new(),
        }
    }

    /// Generate a new temporary value
    pub fn new_temp(&mut self) -> Value {
        let temp = self.temp_counter;
        self.temp_counter += 1;
        Value::Temp(temp)
    }

    /// Generate a new label
    pub fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Start building a new function
    pub fn start_function(&mut self, name: String, return_type: Option<Type>) {
        self.current_function = Some(Function::new(name, return_type));
    }

    /// Finish the current function and add it to the program
    pub fn finish_function(&mut self) {
        if let Some(func) = self.current_function.take() {
            self.program.add_function(func);
        }
    }

    /// Get the current function (mutable)
    pub fn current_function_mut(&mut self) -> Option<&mut Function> {
        self.current_function.as_mut()
    }

    /// Build IR from AST
    pub fn build(&mut self, ast: &Node) -> Program {
        match ast {
            Node::Program(prog) => {
                if let Node::Block(block) = prog.block.as_ref() {
                    self.build_block(block);
                }
            }
            _ => {
                // For other top-level nodes, build them directly
                self.build_node(ast);
            }
        }
        self.program.clone()
    }

    /// Build a single AST node
    fn build_node(&mut self, node: &Node) {
        match node {
            Node::Block(block) => {
                self.build_block(block);
            }
            Node::VarDecl(var_decl) => {
                self.build_var_decl(var_decl);
            }
            Node::AssignStmt(assign) => {
                self.build_assign_stmt(assign);
            }
            Node::CallStmt(call) => {
                self.build_call_stmt(call);
            }
            Node::IfStmt(if_stmt) => {
                self.build_if_stmt(if_stmt);
            }
            Node::WhileStmt(while_stmt) => {
                self.build_while_stmt(while_stmt);
            }
            Node::ForStmt(for_stmt) => {
                self.build_for_stmt(for_stmt);
            }
            Node::RepeatStmt(repeat) => {
                self.build_repeat_stmt(repeat);
            }
            Node::CaseStmt(case_stmt) => {
                self.build_case_stmt(case_stmt);
            }
            // Add other statement types as needed
            _ => {
                // For now, ignore unsupported nodes
            }
        }
    }

    /// Build a block (declarations and statements)
    fn build_block(&mut self, block: &ast::Block) {
        // Build declarations first (to register variable types)
        for decl in &block.var_decls {
            self.build_node(decl);
        }
        // Then build statements
        for stmt in &block.statements {
            self.build_node(stmt);
        }
    }

    /// Build a variable declaration
    fn build_var_decl(&mut self, var_decl: &ast::VarDecl) {
        // Determine the type of the variable
        let var_type = self.analyze_type_expr(&var_decl.type_expr);
        
        // Register variable types for later use
        for name in &var_decl.names {
            self.variable_types.insert(name.clone(), var_type.clone());
        }

        // Generate IR for variable allocation
        // For Variant types, we need to allocate memory and initialize
        if var_type == Type::variant() {
            // Generate instructions first (before borrowing func)
            let mut instructions = Vec::new();
            for _name in &var_decl.names {
                // Initialize Variant to empty
                let (inst, _result) = self.generate_variant_new_empty();
                instructions.push(inst);
            }
            
            // Then add instructions to the function
            if let Some(func) = self.current_function_mut() {
                let entry_label = func.entry_block.clone();
                if let Some(entry) = func.get_block_mut(&entry_label) {
                    for inst in instructions {
                        entry.add_instruction(inst);
                    }
                }
            }
        }
        // For other types, allocation would be handled by the backend
    }

    /// Build an assignment statement
    fn build_assign_stmt(&mut self, assign: &ast::AssignStmt) {
        // Get target variable name and type (before any borrowing)
        let target_name = if let Node::IdentExpr(ident) = assign.target.as_ref() {
            Some(ident.name.clone())
        } else {
            None
        };

        let target_type = target_name
            .as_ref()
            .and_then(|name| self.variable_types.get(name))
            .cloned();

        // Build the value expression first (before borrowing func)
        let value_result = self.build_expression(assign.value.as_ref());
        let value_type = self.analyze_expression_type(assign.value.as_ref());

        // Generate instructions based on types (before borrowing func)
        let mut instructions = Vec::new();

        // Check if target is Variant type
        if let Some(Type::Variant) = target_type {
            // Check if source is also Variant (Variant to Variant assignment)
            if let Some(value_ty) = value_type {
                if value_ty == Type::variant() {
                    // Variant to Variant: use variant_copy
                    let dest_ptr = self.get_variable_address(&target_name.unwrap());
                    let src_ptr = value_result;
                    let inst = self.generate_variant_copy(dest_ptr, src_ptr);
                    instructions.push(inst);
                } else {
                    // Variant assignment: use variant_assign runtime function
                    let variant_ptr = self.get_variable_address(&target_name.unwrap());
                    let type_id = self.get_variant_type_id(&value_ty);
                    
                    // Generate variant_assign call
                    let inst = self.generate_variant_assign(variant_ptr, type_id, value_result);
                    instructions.push(inst);
                }
            } else {
                // Fallback: use variant_assign with Error type
                let variant_ptr = self.get_variable_address(&target_name.unwrap());
                let type_id = Value::Immediate(0); // VariantType::Empty
                let inst = self.generate_variant_assign(variant_ptr, type_id, value_result);
                instructions.push(inst);
            }
        } else if let Some(value_ty) = value_type {
            // Check if source is Variant type (assigning from Variant)
            if value_ty == Type::variant() {
                // Variant to other type assignment: use conversion function
                let target_ptr = self.get_variable_address(&target_name.unwrap());
                
                // Determine target type and use appropriate conversion
                if let Some(target_ty) = target_type {
                    match target_ty {
                        Type::Primitive(types::PrimitiveType::Integer) => {
                            let (inst, converted_value) = self.generate_variant_to_integer(value_result);
                            instructions.push(inst);
                            instructions.push(Instruction::new(
                                Opcode::Store,
                                vec![target_ptr, converted_value],
                            ));
                        }
                        Type::Primitive(types::PrimitiveType::Boolean) => {
                            // Use variant_to_boolean (would need to add this helper)
                            let (inst, converted_value) = self.generate_variant_to_integer(value_result); // Placeholder
                            instructions.push(inst);
                            instructions.push(Instruction::new(
                                Opcode::Store,
                                vec![target_ptr, converted_value],
                            ));
                        }
                        _ => {
                            // For other types, use generic conversion or error
                            // In a full implementation, we'd have more conversion functions
                        }
                    }
                }
            } else {
                // Regular assignment (non-Variant)
                let target_ptr = self.get_variable_address(&target_name.unwrap());
                instructions.push(Instruction::new(
                    Opcode::Store,
                    vec![target_ptr, value_result],
                ));
            }
        } else {
            // Regular assignment (fallback)
            if let Some(name) = &target_name {
                let target_ptr = self.get_variable_address(name);
                instructions.push(Instruction::new(
                    Opcode::Store,
                    vec![target_ptr, value_result],
                ));
            }
        }

        // Add all instructions to the function (after generating them)
        if let Some(func) = self.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                for inst in instructions {
                    entry.add_instruction(inst);
                }
            }
        }
    }

    /// Build an expression and return the IR value
    fn build_expression(&mut self, expr: &Node) -> Value {
        match expr {
            Node::LiteralExpr(lit) => {
                match &lit.value {
                    ast::LiteralValue::Integer(i) => Value::Immediate(*i as i32),
                    ast::LiteralValue::Boolean(b) => Value::Immediate(if *b { 1 } else { 0 }),
                    ast::LiteralValue::Char(c) => Value::Immediate(*c as i32),
                    ast::LiteralValue::String(_) => {
                        // String literals would need special handling
                        Value::Immediate(0) // Placeholder
                    }
                }
            }
            Node::IdentExpr(ident) => {
                // Return the address/value of the variable
                self.get_variable_address(&ident.name)
            }
            Node::BinaryExpr(bin) => {
                let left = self.build_expression(bin.left.as_ref());
                let right = self.build_expression(bin.right.as_ref());
                let result = self.new_temp();
                
                if let Some(func) = self.current_function_mut() {
                    let entry_label = func.entry_block.clone();
                    if let Some(entry) = func.get_block_mut(&entry_label) {
                        let opcode = match bin.op {
                            ast::BinaryOp::Add => Opcode::Add,
                            ast::BinaryOp::Subtract => Opcode::Sub,
                            ast::BinaryOp::Multiply => Opcode::Mul,
                            ast::BinaryOp::Divide => Opcode::Div,
                            ast::BinaryOp::Mod => Opcode::Mod,
                            _ => {
                                // For comparison operators, we'd need different handling
                                return result; // Placeholder
                            }
                        };
                        entry.add_instruction(Instruction::new(
                            opcode,
                            vec![result.clone(), left, right],
                        ));
                    }
                }
                result
            }
            _ => {
                // For other expression types, return a placeholder
                self.new_temp()
            }
        }
    }

    /// Analyze a type expression to get the Type
    fn analyze_type_expr(&self, type_expr: &Node) -> Type {
        match type_expr {
            Node::NamedType(named) => {
                match named.name.to_lowercase().as_str() {
                    "integer" => Type::integer(),
                    "boolean" => Type::boolean(),
                    "char" => Type::char(),
                    "byte" => Type::byte(),
                    "word" => Type::word(),
                    "variant" => Type::variant(),
                    _ => Type::Error,
                }
            }
            _ => Type::Error,
        }
    }

    /// Analyze an expression to determine its type
    fn analyze_expression_type(&self, expr: &Node) -> Option<Type> {
        match expr {
            Node::LiteralExpr(lit) => {
                match &lit.value {
                    ast::LiteralValue::Integer(_) => Some(Type::integer()),
                    ast::LiteralValue::Boolean(_) => Some(Type::boolean()),
                    ast::LiteralValue::Char(_) => Some(Type::char()),
                    ast::LiteralValue::String(_) => Some(Type::array(Type::integer(), Type::char())),
                }
            }
            Node::IdentExpr(ident) => {
                self.variable_types.get(&ident.name).cloned()
            }
            _ => None,
        }
    }

    /// Get the VariantType ID for a given Type
    fn get_variant_type_id(&self, ty: &Type) -> Value {
        let variant_type = RuntimeVariantType::from_type(ty);
        Value::Immediate(variant_type as i32)
    }

    /// Get the address/value of a variable
    fn get_variable_address(&self, _name: &str) -> Value {
        // In a full implementation, this would look up the variable's memory location
        // For now, use a simplified memory address
        Value::Memory {
            base: "sp".to_string(),
            offset: 0, // Would be calculated based on variable's position
        }
    }

    // Placeholder methods for other statement types
    fn build_call_stmt(&mut self, _call: &ast::CallStmt) {
        // TODO: Implement
    }

    fn build_if_stmt(&mut self, _if_stmt: &ast::IfStmt) {
        // TODO: Implement
    }

    fn build_while_stmt(&mut self, _while_stmt: &ast::WhileStmt) {
        // TODO: Implement
    }

    fn build_for_stmt(&mut self, _for_stmt: &ast::ForStmt) {
        // TODO: Implement
    }

    fn build_repeat_stmt(&mut self, _repeat: &ast::RepeatStmt) {
        // TODO: Implement
    }

    fn build_case_stmt(&mut self, _case_stmt: &ast::CaseStmt) {
        // TODO: Implement
    }

    /// Get the built program
    pub fn into_program(self) -> Program {
        self.program
    }

    // ===== Variant Runtime Support =====
    // These helper functions generate IR calls to Variant runtime functions
    // They will be used when AST to IR conversion is implemented

    /// Generate IR for Variant assignment: variant := value
    /// This generates a call to variant_assign runtime function
    /// Parameters: variant_ptr (pointer to Variant), type_id (VariantType enum value), value (the value to assign)
    pub fn generate_variant_assign(&mut self, variant_ptr: Value, type_id: Value, value: Value) -> Instruction {
        // CALL variant_assign(variant_ptr, type_id, value)
        // Note: In actual implementation, parameters would be pushed on stack or passed in registers
        Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_assign".to_string()),
                variant_ptr,
                type_id,
                value,
            ],
        )
    }

    /// Generate IR for Variant copy: dest := src
    /// This generates a call to variant_copy runtime function
    pub fn generate_variant_copy(&mut self, dest_ptr: Value, src_ptr: Value) -> Instruction {
        // CALL variant_copy, dest_ptr, src_ptr
        Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_copy".to_string()),
                dest_ptr,
                src_ptr,
            ],
        )
    }

    /// Generate IR for Variant type check: variant_is_type(variant_ptr, type_id)
    /// Returns a temporary value with the boolean result
    pub fn generate_variant_is_type(&mut self, variant_ptr: Value, type_id: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        // CALL variant_is_type, variant_ptr, type_id -> result_temp
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_is_type".to_string()),
                variant_ptr,
                type_id,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for Variant to integer conversion: variant_to_integer(variant_ptr)
    /// Returns a temporary value with the integer result
    pub fn generate_variant_to_integer(&mut self, variant_ptr: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        // CALL variant_to_integer, variant_ptr -> result_temp
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_to_integer".to_string()),
                variant_ptr,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for Variant to string conversion: variant_to_string(variant_ptr)
    /// Returns a temporary value with the string pointer result
    pub fn generate_variant_to_string(&mut self, variant_ptr: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        // CALL variant_to_string, variant_ptr -> result_temp
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_to_string".to_string()),
                variant_ptr,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for creating a new Variant: variant_new(type_id, value)
    /// Returns a temporary value with the Variant pointer result
    pub fn generate_variant_new(&mut self, type_id: Value, value: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        // CALL variant_new, type_id, value -> result_temp
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_new".to_string()),
                type_id,
                value,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for creating an empty Variant: variant_new_empty()
    /// Returns a temporary value with the Variant pointer result
    pub fn generate_variant_new_empty(&mut self) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        // CALL variant_new_empty -> result_temp
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_new_empty".to_string()),
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for getting Variant type: variant_get_type(variant_ptr)
    /// Returns a temporary value with the type ID result
    pub fn generate_variant_get_type(&mut self, variant_ptr: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        // CALL variant_get_type, variant_ptr -> result_temp
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("variant_get_type".to_string()),
                variant_ptr,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    // ===== Closure Runtime Support =====
    // These helper functions generate IR calls to closure runtime functions
    // They will be used when AST to IR conversion is implemented

    /// Generate IR for creating a closure: closure_new(function_id, captured_names, captured_values, param_count, returns_value)
    /// Returns a temporary value with the closure pointer result
    pub fn generate_closure_new(
        &mut self,
        function_id: Value,
        captured_names: Vec<Value>,
        captured_values: Vec<Value>,
        param_count: Value,
        returns_value: Value,
    ) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let mut operands = vec![
            Value::Label("closure_new".to_string()),
            function_id,
        ];
        // Add captured names count and array
        operands.push(Value::Immediate(captured_names.len() as i32));
        for name in captured_names {
            operands.push(name);
        }
        // Add captured values array
        for value in captured_values {
            operands.push(value);
        }
        operands.push(param_count);
        operands.push(returns_value);
        operands.push(result_temp.clone());
        
        let inst = Instruction::new(Opcode::Call, operands);
        (inst, result_temp)
    }

    /// Generate IR for creating a simple closure without captured variables
    /// Returns a temporary value with the closure pointer result
    pub fn generate_closure_new_simple(
        &mut self,
        function_id: Value,
        param_count: Value,
        returns_value: Value,
    ) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("closure_new_simple".to_string()),
                function_id,
                param_count,
                returns_value,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for calling a closure: closure_call(closure_ptr, params...)
    /// Returns a temporary value with the result (if the closure returns a value)
    pub fn generate_closure_call(
        &mut self,
        closure_ptr: Value,
        params: Vec<Value>,
    ) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let mut operands = vec![
            Value::Label("closure_call".to_string()),
            closure_ptr,
        ];
        // Add parameter count
        operands.push(Value::Immediate(params.len() as i32));
        // Add parameters
        for param in params {
            operands.push(param);
        }
        operands.push(result_temp.clone());
        
        let inst = Instruction::new(Opcode::Call, operands);
        (inst, result_temp)
    }

    /// Generate IR for getting a captured variable: closure_get_captured(closure_ptr, name)
    /// Returns a temporary value with the captured variable value
    pub fn generate_closure_get_captured(
        &mut self,
        closure_ptr: Value,
        name: Value,
    ) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("closure_get_captured".to_string()),
                closure_ptr,
                name,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for setting a captured variable: closure_set_captured(closure_ptr, name, value)
    pub fn generate_closure_set_captured(
        &mut self,
        closure_ptr: Value,
        name: Value,
        value: Value,
    ) -> Instruction {
        Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("closure_set_captured".to_string()),
                closure_ptr,
                name,
                value,
            ],
        )
    }

    /// Generate IR for freeing a closure: closure_free(closure_ptr)
    pub fn generate_closure_free(&mut self, closure_ptr: Value) -> Instruction {
        Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("closure_free".to_string()),
                closure_ptr,
            ],
        )
    }
}

impl Default for IRBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokens::Span;

    // Value tests
    #[test]
    fn test_value_immediate() {
        let imm = Value::Immediate(42);
        assert_eq!(imm, Value::Immediate(42));
        assert_ne!(imm, Value::Immediate(43));
    }

    #[test]
    fn test_value_register() {
        let reg = Value::Register("r0".to_string());
        assert_eq!(reg, Value::Register("r0".to_string()));
        assert_ne!(reg, Value::Register("r1".to_string()));
    }

    #[test]
    fn test_value_memory() {
        let mem = Value::Memory { base: "sp".to_string(), offset: 4 };
        assert_eq!(mem, Value::Memory { base: "sp".to_string(), offset: 4 });
        assert_ne!(mem, Value::Memory { base: "sp".to_string(), offset: 8 });
    }

    #[test]
    fn test_value_temp() {
        let temp = Value::Temp(0);
        assert_eq!(temp, Value::Temp(0));
        assert_ne!(temp, Value::Temp(1));
    }

    #[test]
    fn test_value_label() {
        let label = Value::Label("loop".to_string());
        assert_eq!(label, Value::Label("loop".to_string()));
        assert_ne!(label, Value::Label("end".to_string()));
    }

    // Opcode tests
    #[test]
    fn test_opcode_equality() {
        assert_eq!(Opcode::Add, Opcode::Add);
        assert_ne!(Opcode::Add, Opcode::Sub);
        assert_eq!(Opcode::Mov, Opcode::Mov);
        assert_eq!(Opcode::Jump, Opcode::Jump);
        assert_eq!(Opcode::Call, Opcode::Call);
        assert_eq!(Opcode::Ret, Opcode::Ret);
    }

    // Condition tests
    #[test]
    fn test_condition_equality() {
        assert_eq!(Condition::Equal, Condition::Equal);
        assert_ne!(Condition::Equal, Condition::NotEqual);
        assert_eq!(Condition::Less, Condition::Less);
        assert_eq!(Condition::GreaterEqual, Condition::GreaterEqual);
    }

    // Instruction tests
    #[test]
    fn test_instruction_creation() {
        let inst = Instruction::new(
            Opcode::Add,
            vec![
                Value::Temp(0),
                Value::Immediate(1),
                Value::Immediate(2),
            ],
        );
        
        assert_eq!(inst.opcode, Opcode::Add);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.span, None);
    }

    #[test]
    fn test_instruction_with_span() {
        let span = Span::new(0, 10, 1, 1);
        let inst = Instruction::new(Opcode::Mov, vec![
            Value::Temp(0),
            Value::Immediate(42),
        ]).with_span(span);
        
        assert_eq!(inst.opcode, Opcode::Mov);
        assert_eq!(inst.span, Some(span));
    }

    #[test]
    fn test_instruction_all_opcodes() {
        let ops = vec![
            Opcode::Mov, Opcode::Add, Opcode::Sub, Opcode::Mul,
            Opcode::Div, Opcode::Mod, Opcode::Cmp, Opcode::Jump,
            Opcode::CJump, Opcode::Call, Opcode::Ret, Opcode::Load,
            Opcode::Store, Opcode::Push, Opcode::Pop,
        ];
        
        for op in &ops {
            let inst = Instruction::new(op.clone(), vec![]);
            assert_eq!(inst.opcode, *op);
        }
    }

    // BasicBlock tests
    #[test]
    fn test_basic_block_creation() {
        let block = BasicBlock::new("label1".to_string());
        assert_eq!(block.label, "label1");
        assert_eq!(block.instructions.len(), 0);
        assert_eq!(block.successors.len(), 0);
    }

    #[test]
    fn test_basic_block_add_instruction() {
        let mut block = BasicBlock::new("label1".to_string());
        block.add_instruction(Instruction::new(Opcode::Mov, vec![
            Value::Temp(0),
            Value::Immediate(42),
        ]));
        block.add_instruction(Instruction::new(Opcode::Add, vec![
            Value::Temp(1),
            Value::Temp(0),
            Value::Immediate(1),
        ]));
        
        assert_eq!(block.instructions.len(), 2);
        assert_eq!(block.instructions[0].opcode, Opcode::Mov);
        assert_eq!(block.instructions[1].opcode, Opcode::Add);
    }

    #[test]
    fn test_basic_block_add_successor() {
        let mut block = BasicBlock::new("label1".to_string());
        block.add_successor("label2".to_string());
        block.add_successor("label3".to_string());
        
        assert_eq!(block.successors.len(), 2);
        assert!(block.successors.contains(&"label2".to_string()));
        assert!(block.successors.contains(&"label3".to_string()));
    }

    #[test]
    fn test_basic_block_duplicate_successor() {
        let mut block = BasicBlock::new("label1".to_string());
        block.add_successor("label2".to_string());
        block.add_successor("label2".to_string()); // Duplicate
        
        assert_eq!(block.successors.len(), 1); // Should not add duplicate
    }

    // Function tests
    #[test]
    fn test_function_creation() {
        let func = Function::new("test".to_string(), Some(Type::integer()));
        assert_eq!(func.name, "test");
        assert_eq!(func.blocks.len(), 1); // Entry block
        assert_eq!(func.entry_block, "test_entry");
        assert_eq!(func.params.len(), 0);
        assert_eq!(func.return_type, Some(Type::integer()));
    }

    #[test]
    fn test_function_procedure() {
        let func = Function::new("proc".to_string(), None);
        assert_eq!(func.name, "proc");
        assert_eq!(func.return_type, None); // Procedure has no return type
    }

    #[test]
    fn test_function_add_block() {
        let mut func = Function::new("test".to_string(), Some(Type::integer()));
        let block1 = BasicBlock::new("block1".to_string());
        let block2 = BasicBlock::new("block2".to_string());
        func.add_block(block1);
        func.add_block(block2);
        
        assert_eq!(func.blocks.len(), 3); // Entry + 2 blocks
    }

    #[test]
    fn test_function_get_block_mut() {
        let mut func = Function::new("test".to_string(), None);
        let mut block = BasicBlock::new("block1".to_string());
        block.add_instruction(Instruction::new(Opcode::Mov, vec![]));
        func.add_block(block);
        
        if let Some(entry_block) = func.get_block_mut("test_entry") {
            entry_block.add_instruction(Instruction::new(Opcode::Ret, vec![]));
            assert_eq!(entry_block.instructions.len(), 1);
        } else {
            panic!("Entry block not found");
        }
        
        if let Some(block) = func.get_block_mut("block1") {
            assert_eq!(block.instructions.len(), 1);
        } else {
            panic!("Block1 not found");
        }
        
        assert!(func.get_block_mut("nonexistent").is_none());
    }

    // Program tests
    #[test]
    fn test_program_creation() {
        let program = Program::new();
        assert_eq!(program.functions.len(), 0);
        assert_eq!(program.globals.len(), 0);
    }

    #[test]
    fn test_program_default() {
        let program = Program::default();
        assert_eq!(program.functions.len(), 0);
        assert_eq!(program.globals.len(), 0);
    }

    #[test]
    fn test_program_add_function() {
        let mut program = Program::new();
        let func1 = Function::new("func1".to_string(), Some(Type::integer()));
        let func2 = Function::new("func2".to_string(), None);
        program.add_function(func1);
        program.add_function(func2);
        
        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "func1");
        assert_eq!(program.functions[1].name, "func2");
    }

    // IRBuilder tests
    #[test]
    fn test_ir_builder_new() {
        let builder = IRBuilder::new();
        assert_eq!(builder.temp_counter, 0);
        assert_eq!(builder.label_counter, 0);
        assert!(builder.current_function.is_none());
    }

    #[test]
    fn test_ir_builder_default() {
        let builder = IRBuilder::default();
        assert_eq!(builder.temp_counter, 0);
    }

    #[test]
    fn test_ir_builder_new_temp() {
        let mut builder = IRBuilder::new();
        let temp1 = builder.new_temp();
        let temp2 = builder.new_temp();
        let temp3 = builder.new_temp();
        
        assert_eq!(temp1, Value::Temp(0));
        assert_eq!(temp2, Value::Temp(1));
        assert_eq!(temp3, Value::Temp(2));
    }

    #[test]
    fn test_ir_builder_new_label() {
        let mut builder = IRBuilder::new();
        let label1 = builder.new_label("loop");
        let label2 = builder.new_label("loop");
        let label3 = builder.new_label("end");
        
        assert_eq!(label1, "loop_0");
        assert_eq!(label2, "loop_1");
        assert_eq!(label3, "end_2");
    }

    #[test]
    fn test_ir_builder_start_function() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), Some(Type::integer()));
        
        assert!(builder.current_function.is_some());
        let func = builder.current_function.as_ref().unwrap();
        assert_eq!(func.name, "test");
        assert_eq!(func.return_type, Some(Type::integer()));
    }

    #[test]
    fn test_ir_builder_finish_function() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        builder.finish_function();
        
        assert!(builder.current_function.is_none());
        assert_eq!(builder.program.functions.len(), 1);
        assert_eq!(builder.program.functions[0].name, "test");
    }

    #[test]
    fn test_ir_builder_current_function_mut() {
        let mut builder = IRBuilder::new();
        assert!(builder.current_function_mut().is_none());
        
        builder.start_function("test".to_string(), None);
        if let Some(func) = builder.current_function_mut() {
            func.add_block(BasicBlock::new("block1".to_string()));
            assert_eq!(func.blocks.len(), 2); // Entry + block1
        } else {
            panic!("Function should exist");
        }
    }

    #[test]
    fn test_ir_builder_into_program() {
        let mut builder = IRBuilder::new();
        builder.start_function("func1".to_string(), None);
        builder.finish_function();
        builder.start_function("func2".to_string(), Some(Type::integer()));
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 2);
    }

    // Integration tests
    #[test]
    fn test_complete_ir_program() {
        let mut program = Program::new();
        
        // Create main function
        let mut main_func = Function::new("main".to_string(), None);
        let mut entry_block = BasicBlock::new("main_entry".to_string());
        entry_block.add_instruction(Instruction::new(Opcode::Mov, vec![
            Value::Temp(0),
            Value::Immediate(42),
        ]));
        entry_block.add_instruction(Instruction::new(Opcode::Ret, vec![]));
        main_func.add_block(entry_block);
        program.add_function(main_func);
        
        // Create helper function
        let mut helper_func = Function::new("helper".to_string(), Some(Type::integer()));
        let mut helper_entry = BasicBlock::new("helper_entry".to_string());
        helper_entry.add_instruction(Instruction::new(Opcode::Add, vec![
            Value::Temp(0),
            Value::Temp(1),
            Value::Immediate(1),
        ]));
        helper_entry.add_instruction(Instruction::new(Opcode::Ret, vec![
            Value::Temp(0),
        ]));
        helper_func.add_block(helper_entry);
        program.add_function(helper_func);
        
        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "main");
        assert_eq!(program.functions[1].name, "helper");
    }

    #[test]
    fn test_ir_builder_complete_workflow() {
        let mut builder = IRBuilder::new();
        
        // Build main function
        builder.start_function("main".to_string(), None);
        let temp = builder.new_temp(); // Generate temp before borrowing func
        if let Some(func) = builder.current_function_mut() {
            if let Some(entry) = func.get_block_mut("main_entry") {
                entry.add_instruction(Instruction::new(Opcode::Mov, vec![
                    temp.clone(),
                    Value::Immediate(10),
                ]));
                entry.add_instruction(Instruction::new(Opcode::Ret, vec![]));
            }
        }
        builder.finish_function();
        
        // Build helper function
        builder.start_function("helper".to_string(), Some(Type::integer()));
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].blocks[0].instructions.len(), 2);
    }

    // ===== Variant IR Generation Tests =====

    #[test]
    fn test_generate_variant_assign() {
        let mut builder = IRBuilder::new();
        let variant_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let type_id = Value::Immediate(1); // VariantType::Integer
        let value = Value::Immediate(42);

        let inst = builder.generate_variant_assign(variant_ptr.clone(), type_id.clone(), value.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("variant_assign".to_string()));
        assert_eq!(inst.operands[1], variant_ptr);
        assert_eq!(inst.operands[2], type_id);
        assert_eq!(inst.operands[3], value);
    }

    #[test]
    fn test_generate_variant_copy() {
        let mut builder = IRBuilder::new();
        let dest_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let src_ptr = Value::Memory { base: "sp".to_string(), offset: -8 };

        let inst = builder.generate_variant_copy(dest_ptr.clone(), src_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.operands[0], Value::Label("variant_copy".to_string()));
        assert_eq!(inst.operands[1], dest_ptr);
        assert_eq!(inst.operands[2], src_ptr);
    }

    #[test]
    fn test_generate_variant_new_empty() {
        let mut builder = IRBuilder::new();
        let (inst, result) = builder.generate_variant_new_empty();

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 2);
        assert_eq!(inst.operands[0], Value::Label("variant_new_empty".to_string()));
        assert_eq!(inst.operands[1], result);
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_variant_new() {
        let mut builder = IRBuilder::new();
        let type_id = Value::Immediate(1); // VariantType::Integer
        let value = Value::Immediate(42);
        let (inst, result) = builder.generate_variant_new(type_id.clone(), value.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("variant_new".to_string()));
        assert_eq!(inst.operands[1], type_id);
        assert_eq!(inst.operands[2], value);
        assert_eq!(inst.operands[3], result);
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_variant_is_type() {
        let mut builder = IRBuilder::new();
        let variant_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let type_id = Value::Immediate(1); // VariantType::Integer
        let (inst, result) = builder.generate_variant_is_type(variant_ptr.clone(), type_id.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("variant_is_type".to_string()));
        assert_eq!(inst.operands[1], variant_ptr);
        assert_eq!(inst.operands[2], type_id);
        assert_eq!(inst.operands[3], result);
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_variant_to_integer() {
        let mut builder = IRBuilder::new();
        let variant_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let (inst, result) = builder.generate_variant_to_integer(variant_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.operands[0], Value::Label("variant_to_integer".to_string()));
        assert_eq!(inst.operands[1], variant_ptr);
        assert_eq!(inst.operands[2], result);
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_variant_to_string() {
        let mut builder = IRBuilder::new();
        let variant_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let (inst, result) = builder.generate_variant_to_string(variant_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.operands[0], Value::Label("variant_to_string".to_string()));
        assert_eq!(inst.operands[1], variant_ptr);
        assert_eq!(inst.operands[2], result);
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_variant_get_type() {
        let mut builder = IRBuilder::new();
        let variant_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let (inst, result) = builder.generate_variant_get_type(variant_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.operands[0], Value::Label("variant_get_type".to_string()));
        assert_eq!(inst.operands[1], variant_ptr);
        assert_eq!(inst.operands[2], result);
        assert_eq!(result, Value::Temp(0));
    }

    // ===== Closure IR Generation Tests =====

    #[test]
    fn test_generate_closure_new_simple() {
        let mut builder = IRBuilder::new();
        let function_id = Value::Label("func_123".to_string());
        let param_count = Value::Immediate(2);
        let returns_value = Value::Immediate(1); // true
        let (inst, result) = builder.generate_closure_new_simple(function_id.clone(), param_count.clone(), returns_value.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 5);
        assert_eq!(inst.operands[0], Value::Label("closure_new_simple".to_string()));
        assert_eq!(inst.operands[1], function_id);
        assert_eq!(inst.operands[2], param_count);
        assert_eq!(inst.operands[3], returns_value);
        assert_eq!(inst.operands[4], result);
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_closure_new_with_captures() {
        let mut builder = IRBuilder::new();
        let function_id = Value::Label("func_456".to_string());
        let captured_names = vec![
            Value::Label("x".to_string()),
            Value::Label("y".to_string()),
        ];
        let captured_values = vec![
            Value::Immediate(42),
            Value::Immediate(100),
        ];
        let param_count = Value::Immediate(1);
        let returns_value = Value::Immediate(0); // false
        let (inst, result) = builder.generate_closure_new(
            function_id.clone(),
            captured_names.clone(),
            captured_values.clone(),
            param_count.clone(),
            returns_value.clone(),
        );

        assert_eq!(inst.opcode, Opcode::Call);
        // Should have: function_id, count, names..., values..., param_count, returns_value, result
        assert!(inst.operands.len() >= 7);
        assert_eq!(inst.operands[0], Value::Label("closure_new".to_string()));
        assert_eq!(inst.operands[1], function_id);
        assert_eq!(inst.operands[2], Value::Immediate(2)); // captured count
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_closure_call() {
        let mut builder = IRBuilder::new();
        let closure_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let params = vec![
            Value::Immediate(42),
            Value::Immediate(100),
        ];
        let (inst, result) = builder.generate_closure_call(closure_ptr.clone(), params);

        assert_eq!(inst.opcode, Opcode::Call);
        assert!(inst.operands.len() >= 5); // closure_call, closure_ptr, param_count, params..., result
        assert_eq!(inst.operands[0], Value::Label("closure_call".to_string()));
        assert_eq!(inst.operands[1], closure_ptr);
        assert_eq!(inst.operands[2], Value::Immediate(2)); // param count
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_closure_get_captured() {
        let mut builder = IRBuilder::new();
        let closure_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let name = Value::Label("x".to_string());
        let (inst, result) = builder.generate_closure_get_captured(closure_ptr.clone(), name.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("closure_get_captured".to_string()));
        assert_eq!(inst.operands[1], closure_ptr);
        assert_eq!(inst.operands[2], name);
        assert_eq!(inst.operands[3], result);
        assert_eq!(result, Value::Temp(0));
    }

    #[test]
    fn test_generate_closure_set_captured() {
        let mut builder = IRBuilder::new();
        let closure_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let name = Value::Label("counter".to_string());
        let value = Value::Immediate(5);
        let inst = builder.generate_closure_set_captured(closure_ptr.clone(), name.clone(), value.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("closure_set_captured".to_string()));
        assert_eq!(inst.operands[1], closure_ptr);
        assert_eq!(inst.operands[2], name);
        assert_eq!(inst.operands[3], value);
    }

    #[test]
    fn test_generate_closure_free() {
        let mut builder = IRBuilder::new();
        let closure_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let inst = builder.generate_closure_free(closure_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 2);
        assert_eq!(inst.operands[0], Value::Label("closure_free".to_string()));
        assert_eq!(inst.operands[1], closure_ptr);
    }

    // ===== Variant AST→IR Integration Tests =====

    #[test]
    fn test_build_variant_variable_declaration() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);

        // Create: var v: Variant;
        let var_decl = Node::VarDecl(ast::VarDecl {
            names: vec!["v".to_string()],
            type_expr: Box::new(Node::NamedType(ast::NamedType {
                name: "Variant".to_string(),
                generic_args: vec![],
                span: Span::new(0, 10, 1, 1),
            })),
            is_class_var: false,
            absolute_address: None,
            span: Span::new(0, 10, 1, 1),
        });

        if let Node::VarDecl(var_decl_stmt) = &var_decl {
            builder.build_var_decl(var_decl_stmt);
        }

        // Verify variable type was registered
        assert_eq!(builder.variable_types.get("v"), Some(&Type::variant()));

        // Verify initialization instruction was added
        if let Some(func) = builder.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                assert!(entry.instructions.len() >= 1);
                assert_eq!(entry.instructions[0].opcode, Opcode::Call);
                assert_eq!(entry.instructions[0].operands[0], Value::Label("variant_new_empty".to_string()));
            }
        }
    }

    #[test]
    fn test_build_variant_assignment_integer() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);

        // Register Variant variable
        builder.variable_types.insert("v".to_string(), Type::variant());

        // Create: v := 42;
        let assign = Node::AssignStmt(ast::AssignStmt {
            target: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "v".to_string(),
                span: Span::new(0, 10, 1, 1),
            })),
            value: Box::new(Node::LiteralExpr(ast::LiteralExpr {
                value: ast::LiteralValue::Integer(42),
                span: Span::new(0, 10, 1, 1),
            })),
            span: Span::new(0, 10, 1, 1),
        });

        if let Node::AssignStmt(assign_stmt) = &assign {
            builder.build_assign_stmt(assign_stmt);
        }

        // Verify variant_assign instruction was generated
        if let Some(func) = builder.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Should have variant_assign call
                let variant_assign_inst = entry.instructions.iter()
                    .find(|inst| inst.opcode == Opcode::Call && 
                          matches!(&inst.operands[0], Value::Label(label) if label == "variant_assign"));
                assert!(variant_assign_inst.is_some(), "Should have variant_assign instruction");
            }
        }
    }

    #[test]
    fn test_build_variant_to_integer_assignment() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);

        // Register variables
        builder.variable_types.insert("v".to_string(), Type::variant());
        builder.variable_types.insert("i".to_string(), Type::integer());

        // Create: i := v;
        let assign = Node::AssignStmt(ast::AssignStmt {
            target: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "i".to_string(),
                span: Span::new(0, 10, 1, 1),
            })),
            value: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "v".to_string(),
                span: Span::new(0, 10, 1, 1),
            })),
            span: Span::new(0, 10, 1, 1),
        });

        if let Node::AssignStmt(assign_stmt) = &assign {
            builder.build_assign_stmt(assign_stmt);
        }

        // Verify variant_to_integer conversion was generated
        if let Some(func) = builder.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Should have variant_to_integer call followed by Store
                let variant_to_int = entry.instructions.iter()
                    .find(|inst| inst.opcode == Opcode::Call && 
                          matches!(&inst.operands[0], Value::Label(label) if label == "variant_to_integer"));
                assert!(variant_to_int.is_some(), "Should have variant_to_integer instruction");
            }
        }
    }

    #[test]
    fn test_build_variant_assignment_string() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);

        // Register Variant variable
        builder.variable_types.insert("v".to_string(), Type::variant());

        // Create: v := 'Hello';
        let assign = Node::AssignStmt(ast::AssignStmt {
            target: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "v".to_string(),
                span: Span::new(0, 10, 1, 1),
            })),
            value: Box::new(Node::LiteralExpr(ast::LiteralExpr {
                value: ast::LiteralValue::String("Hello".to_string()),
                span: Span::new(0, 10, 1, 1),
            })),
            span: Span::new(0, 10, 1, 1),
        });

        if let Node::AssignStmt(assign_stmt) = &assign {
            builder.build_assign_stmt(assign_stmt);
        }

        // Verify variant_assign instruction was generated
        if let Some(func) = builder.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                let variant_assign_inst = entry.instructions.iter()
                    .find(|inst| inst.opcode == Opcode::Call && 
                          matches!(&inst.operands[0], Value::Label(label) if label == "variant_assign"));
                assert!(variant_assign_inst.is_some(), "Should have variant_assign instruction");
            }
        }
    }

    #[test]
    fn test_build_variant_copy_assignment() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);

        // Register Variant variables
        builder.variable_types.insert("v1".to_string(), Type::variant());
        builder.variable_types.insert("v2".to_string(), Type::variant());

        // Create: v2 := v1;
        let assign = Node::AssignStmt(ast::AssignStmt {
            target: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "v2".to_string(),
                span: Span::new(0, 10, 1, 1),
            })),
            value: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "v1".to_string(),
                span: Span::new(0, 10, 1, 1),
            })),
            span: Span::new(0, 10, 1, 1),
        });

        if let Node::AssignStmt(assign_stmt) = &assign {
            builder.build_assign_stmt(assign_stmt);
        }

        // Verify variant_copy instruction was generated
        if let Some(func) = builder.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                let variant_copy_inst = entry.instructions.iter()
                    .find(|inst| inst.opcode == Opcode::Call && 
                          matches!(&inst.operands[0], Value::Label(label) if label == "variant_copy"));
                assert!(variant_copy_inst.is_some(), "Should have variant_copy instruction");
            }
        }
    }

    #[test]
    fn test_build_program_with_variant() {
        let mut builder = IRBuilder::new();
        
        // Create a simple program: var v: Variant; v := 42;
        let program = Node::Program(ast::Program {
            name: "test".to_string(),
            directives: vec![],
            block: Box::new(Node::Block(ast::Block {
                directives: vec![],
                label_decls: vec![],
                const_decls: vec![],
                type_decls: vec![],
                var_decls: vec![Node::VarDecl(ast::VarDecl {
                    names: vec!["v".to_string()],
                    type_expr: Box::new(Node::NamedType(ast::NamedType {
                        name: "Variant".to_string(),
                        generic_args: vec![],
                        span: Span::new(0, 10, 1, 1),
                    })),
                    is_class_var: false,
                    absolute_address: None,
                    span: Span::new(0, 10, 1, 1),
                })],
                threadvar_decls: vec![],
                proc_decls: vec![],
                func_decls: vec![],
                operator_decls: vec![],
                statements: vec![Node::AssignStmt(ast::AssignStmt {
                    target: Box::new(Node::IdentExpr(ast::IdentExpr {
                        name: "v".to_string(),
                        span: Span::new(0, 10, 1, 1),
                    })),
                    value: Box::new(Node::LiteralExpr(ast::LiteralExpr {
                        value: ast::LiteralValue::Integer(42),
                        span: Span::new(0, 10, 1, 1),
                    })),
                    span: Span::new(0, 10, 1, 1),
                })],
                span: Span::new(0, 50, 1, 1),
            })),
            span: Span::new(0, 50, 1, 1),
        });

        // Start a function before building (build() doesn't create functions automatically)
        builder.start_function("main".to_string(), None);
        let _ir_program = builder.build(&program);
        builder.finish_function();

        // Verify the program was built
        assert_eq!(builder.program.functions.len(), 1);
        
        // Verify Variant variable was registered
        assert_eq!(builder.variable_types.get("v"), Some(&Type::variant()));
    }
}
