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

    /// Build statements and return the generated instructions
    /// This is a helper method to avoid borrow checker issues when building control flow
    fn build_statements_to_instructions(&mut self, statements: &[Node]) -> Vec<Instruction> {
        // Save current function state
        let current_func = self.current_function.take();
        let saved_variable_types = self.variable_types.clone();
        
        // Create a temporary function to build into
        let temp_func_name = format!("temp_{}", self.label_counter);
        self.start_function(temp_func_name.clone(), None);
        
        // Build statements
        for stmt in statements {
            self.build_node(stmt);
        }
        
        // Extract instructions from the temporary function's entry block
        let mut instructions = Vec::new();
        if let Some(temp_func) = self.current_function.take() {
            if let Some(entry) = temp_func.blocks.first() {
                instructions = entry.instructions.clone();
            }
        }
        
        // Restore state
        self.current_function = current_func;
        self.variable_types = saved_variable_types;
        
        instructions
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

        // Check if target is Procedural type
        if let Some(Type::Procedural { .. }) = target_type {
            // Procedural type assignment: store function address
            // If value is a function name (IdentExpr), get its address
            if let Node::IdentExpr(ident) = assign.value.as_ref() {
                // Store function address in procedural type variable
                let dest_ptr = self.get_variable_address(&target_name.unwrap());
                let func_addr = Value::Label(ident.name.clone());
                instructions.push(Instruction::new(
                    Opcode::Mov,
                    vec![dest_ptr, func_addr],
                ));
            } else {
                // Value is already a procedural type variable - copy it
                let dest_ptr = self.get_variable_address(&target_name.unwrap());
                instructions.push(Instruction::new(
                    Opcode::Mov,
                    vec![dest_ptr, value_result],
                ));
            }
        }
        // Check if target is Variant type
        else if let Some(Type::Variant) = target_type {
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
                        // String literals would need special handling (string pool)
                        // For now, return a placeholder
                        Value::Immediate(0)
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
                        match bin.op {
                            // Arithmetic operations
                            ast::BinaryOp::Add => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Add,
                                    vec![result.clone(), left, right],
                                ));
                            }
                            ast::BinaryOp::Subtract => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Sub,
                                    vec![result.clone(), left, right],
                                ));
                            }
                            ast::BinaryOp::Multiply => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Mul,
                                    vec![result.clone(), left, right],
                                ));
                            }
                            ast::BinaryOp::Divide | ast::BinaryOp::Div => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Div,
                                    vec![result.clone(), left, right],
                                ));
                            }
                            ast::BinaryOp::Mod => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Mod,
                                    vec![result.clone(), left, right],
                                ));
                            }
                            // Comparison operations - generate CMP and return boolean result
                            ast::BinaryOp::Equal => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![left, right],
                                ));
                                // Result is 1 if equal, 0 otherwise (handled by backend)
                                // For now, return the temp - backend will set it based on condition flags
                            }
                            ast::BinaryOp::NotEqual => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![left, right],
                                ));
                            }
                            ast::BinaryOp::Less => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![left, right],
                                ));
                            }
                            ast::BinaryOp::LessEqual => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![left, right],
                                ));
                            }
                            ast::BinaryOp::Greater => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![left, right],
                                ));
                            }
                            ast::BinaryOp::GreaterEqual => {
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![left, right],
                                ));
                            }
                            // Logical operations - need to be handled specially
                            ast::BinaryOp::And | ast::BinaryOp::Or => {
                                // Short-circuit evaluation would need control flow
                                // For now, treat as arithmetic (non-short-circuit)
                                // TODO: Implement proper short-circuit evaluation
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![left, right],
                                ));
                            }
                            // Set membership, type operations - need special handling
                            ast::BinaryOp::In | ast::BinaryOp::Is | ast::BinaryOp::As => {
                                // These need runtime support or special code generation
                                // For now, return placeholder
                            }
                        }
                    }
                }
                result
            }
            Node::UnaryExpr(unary) => {
                let expr_val = self.build_expression(unary.expr.as_ref());
                let result = self.new_temp();
                
                if let Some(func) = self.current_function_mut() {
                    let entry_label = func.entry_block.clone();
                    if let Some(entry) = func.get_block_mut(&entry_label) {
                        match unary.op {
                            ast::UnaryOp::Plus => {
                                // Unary plus - no operation needed, just return the value
                                return expr_val;
                            }
                            ast::UnaryOp::Minus => {
                                // Unary minus - subtract from 0
                                let zero = Value::Immediate(0);
                                entry.add_instruction(Instruction::new(
                                    Opcode::Sub,
                                    vec![result.clone(), zero, expr_val],
                                ));
                            }
                            ast::UnaryOp::Not => {
                                // Logical NOT - XOR with 1
                                let one = Value::Immediate(1);
                                entry.add_instruction(Instruction::new(
                                    Opcode::Cmp,
                                    vec![expr_val, one],
                                ));
                                // Result is 1 if expr_val == 0, 0 otherwise
                                // Backend will handle this based on condition flags
                            }
                            ast::UnaryOp::AddressOf => {
                                // Address-of operator - return the address directly
                                // For now, return the expression value (would need proper address calculation)
                                return expr_val;
                            }
                        }
                    }
                }
                result
            }
            Node::CallExpr(call) => {
                // Build argument expressions
                let mut args = Vec::new();
                for arg in &call.args {
                    args.push(self.build_expression(arg));
                }
                
                let result = self.new_temp();
                
                if let Some(func) = self.current_function_mut() {
                    let entry_label = func.entry_block.clone();
                    if let Some(entry) = func.get_block_mut(&entry_label) {
                        // Build CALL instruction: CALL function_name, result, arg1, arg2, ...
                        let mut operands = vec![Value::Label(call.name.clone()), result.clone()];
                        operands.extend(args);
                        entry.add_instruction(Instruction::new(
                            Opcode::Call,
                            operands,
                        ));
                    }
                }
                result
            }
            Node::IndexExpr(index) => {
                // Array indexing: array[index]
                let array_val = self.build_expression(index.array.as_ref());
                let index_val = self.build_expression(index.index.as_ref());
                let result = self.new_temp();
                let temp_offset = self.new_temp();
                
                if let Some(func) = self.current_function_mut() {
                    let entry_label = func.entry_block.clone();
                    if let Some(entry) = func.get_block_mut(&entry_label) {
                        // Calculate address: array_base + (index * element_size)
                        // For now, simplified - backend will handle element size
                        entry.add_instruction(Instruction::new(
                            Opcode::Mul,
                            vec![temp_offset.clone(), index_val, Value::Immediate(1)], // element_size = 1 for now
                        ));
                        entry.add_instruction(Instruction::new(
                            Opcode::Add,
                            vec![result.clone(), array_val, temp_offset],
                        ));
                        // Load from memory
                        entry.add_instruction(Instruction::new(
                            Opcode::Load,
                            vec![result.clone(), result.clone()],
                        ));
                    }
                }
                result
            }
            Node::SliceExpr(slice) => {
                // Array slicing: array[start:end:step]
                // This generates a call to array_slice runtime function
                let array_val = self.build_expression(slice.array.as_ref());
                
                // Build start index (or -1 for None)
                let start_val = if let Some(start) = &slice.start {
                    self.build_expression(start)
                } else {
                    Value::Immediate(-1) // -1 indicates None
                };
                
                // Build end index (or -1 for None)
                let end_val = if let Some(end) = &slice.end {
                    self.build_expression(end)
                } else {
                    Value::Immediate(-1) // -1 indicates None
                };
                
                // Build step (or 0 for None, which means step of 1)
                let step_val = if let Some(step) = &slice.step {
                    self.build_expression(step)
                } else {
                    Value::Immediate(0) // 0 indicates None (default step of 1)
                };
                
                // For now, we'll use placeholders for array_length and element_size
                // In a full implementation, these would come from type information
                let array_length = Value::Immediate(0); // Placeholder - would need type info
                let element_size = Value::Immediate(4); // Placeholder - would need type info
                
                // Generate IR using the helper function
                let (inst, result) = self.generate_array_slice(
                    array_val,
                    array_length,
                    element_size,
                    start_val,
                    end_val,
                    step_val,
                );
                
                if let Some(func) = self.current_function_mut() {
                    let entry_label = func.entry_block.clone();
                    if let Some(entry) = func.get_block_mut(&entry_label) {
                        entry.add_instruction(inst);
                    }
                }
                result
            }
            Node::FieldExpr(field) => {
                // Field access: record.field
                let record_val = self.build_expression(field.record.as_ref());
                let result = self.new_temp();
                let temp_addr = self.new_temp();
                
                if let Some(func) = self.current_function_mut() {
                    let entry_label = func.entry_block.clone();
                    if let Some(entry) = func.get_block_mut(&entry_label) {
                        // Calculate field offset (would need type information)
                        // For now, simplified - backend will handle field offsets
                        let field_offset = Value::Immediate(0); // Placeholder
                        entry.add_instruction(Instruction::new(
                            Opcode::Add,
                            vec![temp_addr.clone(), record_val, field_offset],
                        ));
                        entry.add_instruction(Instruction::new(
                            Opcode::Load,
                            vec![result.clone(), temp_addr],
                        ));
                    }
                }
                result
            }
            Node::DerefExpr(deref) => {
                // Pointer dereference: ^pointer
                let pointer_val = self.build_expression(deref.pointer.as_ref());
                let result = self.new_temp();
                
                if let Some(func) = self.current_function_mut() {
                    let entry_label = func.entry_block.clone();
                    if let Some(entry) = func.get_block_mut(&entry_label) {
                        // Load from address stored in pointer
                        entry.add_instruction(Instruction::new(
                            Opcode::Load,
                            vec![result.clone(), pointer_val],
                        ));
                    }
                }
                result
            }
            Node::AddressOfExpr(addr) => {
                // Address-of: @variable
                // Return the address of the target
                self.build_expression(addr.target.as_ref())
            }
            _ => {
                // For other expression types (EnumLiteralExpr, AnonymousFunction, etc.)
                // return a placeholder for now
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

    /// Build a call statement (procedure call)
    fn build_call_stmt(&mut self, call: &ast::CallStmt) {
        // Build argument expressions
        let mut args = Vec::new();
        for arg in &call.args {
            args.push(self.build_expression(arg));
        }
        
        if let Some(func) = self.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Build CALL instruction: CALL procedure_name, arg1, arg2, ...
                let mut operands = vec![Value::Label(call.name.clone())];
                operands.extend(args);
                entry.add_instruction(Instruction::new(
                    Opcode::Call,
                    operands,
                ));
            }
        }
    }

    /// Build an if statement
    fn build_if_stmt(&mut self, if_stmt: &ast::IfStmt) {
        // Generate labels first (before borrowing func)
        let then_label = self.new_label("if_then");
        let else_label = if if_stmt.else_block.is_some() {
            Some(self.new_label("if_else"))
        } else {
            None
        };
        let end_label = self.new_label("if_end");
        
        // Evaluate condition (before borrowing func)
        let cond_val = self.build_expression(if_stmt.condition.as_ref());
        
        // Build then block statements (before borrowing func)
        let then_instructions = if let Node::Block(block) = if_stmt.then_block.as_ref() {
            self.build_statements_to_instructions(&block.statements)
        } else {
            // Single statement
            self.build_statements_to_instructions(&[if_stmt.then_block.as_ref().clone()])
        };
        
        // Build else block statements if present (before borrowing func)
        let else_instructions = if let Some(else_block_node) = &if_stmt.else_block {
            if let Some(else_lbl) = &else_label {
                let instructions = if let Node::Block(block) = else_block_node.as_ref() {
                    self.build_statements_to_instructions(&block.statements)
                } else {
                    // Single statement
                    self.build_statements_to_instructions(&[else_block_node.as_ref().clone()])
                };
                Some((else_lbl.clone(), instructions))
            } else {
                None
            }
        } else {
            None
        };
        
        if let Some(func) = self.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Compare condition with 0 (false)
                let zero = Value::Immediate(0);
                entry.add_instruction(Instruction::new(
                    Opcode::Cmp,
                    vec![cond_val, zero],
                ));
                
                // Conditional jump: if condition is false (0), jump to else or end
                if let Some(else_lbl) = &else_label {
                    entry.add_instruction(Instruction::new(
                        Opcode::CJump,
                        vec![
                            Value::Label(else_lbl.clone()),
                            Value::Label(then_label.clone()),
                        ],
                    ));
                } else {
                    entry.add_instruction(Instruction::new(
                        Opcode::CJump,
                        vec![
                            Value::Label(end_label.clone()),
                            Value::Label(then_label.clone()),
                        ],
                    ));
                }
                
                // Update entry block to point to then/else (before dropping entry borrow)
                entry.successors.push(then_label.clone());
                if let Some(else_lbl) = &else_label {
                    entry.successors.push(else_lbl.clone());
                }
            } // Drop entry borrow here
            
            // Now we can add blocks without conflicting borrows
            // Add then block
            let mut then_block = BasicBlock::new(then_label.clone());
            then_block.instructions = then_instructions;
            // Jump to end after then block
            then_block.add_instruction(Instruction::new(
                Opcode::Jump,
                vec![Value::Label(end_label.clone())],
            ));
            then_block.successors.push(end_label.clone());
            func.add_block(then_block);
            
            // Add else block if present
            if let Some((else_lbl, instructions)) = else_instructions {
                let mut else_block = BasicBlock::new(else_lbl.clone());
                else_block.instructions = instructions;
                // Jump to end after else block
                else_block.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(end_label.clone())],
                ));
                else_block.successors.push(end_label.clone());
                func.add_block(else_block);
            }
            
            // Add end block (merge point)
            let end_block = BasicBlock::new(end_label.clone());
            func.add_block(end_block);
        }
    }

    /// Build a while statement
    fn build_while_stmt(&mut self, while_stmt: &ast::WhileStmt) {
        // Generate labels first (before borrowing func)
        let loop_label = self.new_label("while_loop");
        let body_label = self.new_label("while_body");
        let end_label = self.new_label("while_end");
        
        // Evaluate condition (before borrowing func)
        let cond_val = self.build_expression(while_stmt.condition.as_ref());
        
        // Build body statements (before borrowing func)
        let body_instructions = if let Node::Block(block) = while_stmt.body.as_ref() {
            self.build_statements_to_instructions(&block.statements)
        } else {
            // Single statement
            self.build_statements_to_instructions(&[while_stmt.body.as_ref().clone()])
        };
        
        if let Some(func) = self.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Jump to loop condition check
                entry.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(loop_label.clone())],
                ));
                entry.successors.push(loop_label.clone());
                
                // Loop condition block
                let mut loop_block = BasicBlock::new(loop_label.clone());
                let zero = Value::Immediate(0);
                loop_block.add_instruction(Instruction::new(
                    Opcode::Cmp,
                    vec![cond_val, zero],
                ));
                // If condition is false, jump to end; otherwise jump to body
                loop_block.add_instruction(Instruction::new(
                    Opcode::CJump,
                    vec![
                        Value::Label(end_label.clone()),
                        Value::Label(body_label.clone()),
                    ],
                ));
                loop_block.successors.push(end_label.clone());
                loop_block.successors.push(body_label.clone());
                func.add_block(loop_block);
                
                // Body block
                let mut body_block = BasicBlock::new(body_label.clone());
                body_block.instructions = body_instructions;
                // Jump back to loop condition
                body_block.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(loop_label.clone())],
                ));
                body_block.successors.push(loop_label.clone());
                func.add_block(body_block);
                
                // End block (after loop)
                let end_block = BasicBlock::new(end_label.clone());
                func.add_block(end_block);
            }
        }
    }

    /// Build a for statement
    fn build_for_stmt(&mut self, for_stmt: &ast::ForStmt) {
        // Get loop variable name
        let var_name = for_stmt.var_name.clone();
        
        // Generate labels first (before borrowing func)
        let loop_label = self.new_label("for_loop");
        let body_label = self.new_label("for_body");
        let inc_label = self.new_label("for_inc");
        let end_label = self.new_label("for_end");
        
        // Build expressions (before borrowing func)
        let start_val = self.build_expression(for_stmt.start_expr.as_ref());
        let end_val = self.build_expression(for_stmt.end_expr.as_ref());
        let var_addr = self.get_variable_address(&var_name);
        
        // Build body statements (before borrowing func)
        let body_instructions = if let Node::Block(block) = for_stmt.body.as_ref() {
            self.build_statements_to_instructions(&block.statements)
        } else {
            // Single statement
            self.build_statements_to_instructions(&[for_stmt.body.as_ref().clone()])
        };
        
        if let Some(func) = self.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Initialize loop variable
                entry.add_instruction(Instruction::new(
                    Opcode::Mov,
                    vec![var_addr.clone(), start_val],
                ));
                entry.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(loop_label.clone())],
                ));
                entry.successors.push(loop_label.clone());
                
                // Loop condition check
                let mut loop_block = BasicBlock::new(loop_label.clone());
                // Compare loop variable with end value
                loop_block.add_instruction(Instruction::new(
                    Opcode::Cmp,
                    vec![var_addr.clone(), end_val],
                ));
                // If loop variable > end (for TO) or < end (for DOWNTO), jump to end
                // For now, use simple comparison - backend will handle direction
                loop_block.add_instruction(Instruction::new(
                    Opcode::CJump,
                    vec![
                        Value::Label(end_label.clone()),
                        Value::Label(body_label.clone()),
                    ],
                ));
                loop_block.successors.push(end_label.clone());
                loop_block.successors.push(body_label.clone());
                func.add_block(loop_block);
                
                // Body block
                let mut body_block = BasicBlock::new(body_label.clone());
                body_block.instructions = body_instructions;
                body_block.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(inc_label.clone())],
                ));
                body_block.successors.push(inc_label.clone());
                func.add_block(body_block);
                
                // Increment/decrement block
                let mut inc_block = BasicBlock::new(inc_label.clone());
                let one = Value::Immediate(1);
                if for_stmt.direction == ast::ForDirection::To {
                    // Increment: var := var + 1
                    inc_block.add_instruction(Instruction::new(
                        Opcode::Add,
                        vec![var_addr.clone(), var_addr.clone(), one],
                    ));
                } else {
                    // Decrement: var := var - 1
                    inc_block.add_instruction(Instruction::new(
                        Opcode::Sub,
                        vec![var_addr.clone(), var_addr.clone(), one],
                    ));
                }
                // Jump back to loop condition
                inc_block.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(loop_label.clone())],
                ));
                inc_block.successors.push(loop_label.clone());
                func.add_block(inc_block);
                
                // End block
                let end_block = BasicBlock::new(end_label.clone());
                func.add_block(end_block);
            }
        }
    }

    /// Build a repeat statement
    fn build_repeat_stmt(&mut self, repeat: &ast::RepeatStmt) {
        // Generate labels first (before borrowing func)
        let body_label = self.new_label("repeat_body");
        let cond_label = self.new_label("repeat_cond");
        let end_label = self.new_label("repeat_end");
        
        // Build body statements (before borrowing func)
        let body_instructions = self.build_statements_to_instructions(&repeat.statements);
        
        // Evaluate condition (before borrowing func)
        let cond_val = self.build_expression(repeat.condition.as_ref());
        
        if let Some(func) = self.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Jump to body
                entry.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(body_label.clone())],
                ));
                entry.successors.push(body_label.clone());
                
                // Body block
                let mut body_block = BasicBlock::new(body_label.clone());
                body_block.instructions = body_instructions;
                body_block.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(cond_label.clone())],
                ));
                body_block.successors.push(cond_label.clone());
                func.add_block(body_block);
                
                // Condition check block
                let mut cond_block = BasicBlock::new(cond_label.clone());
                let zero = Value::Immediate(0);
                cond_block.add_instruction(Instruction::new(
                    Opcode::Cmp,
                    vec![cond_val, zero],
                ));
                // If condition is false, jump back to body; otherwise jump to end
                cond_block.add_instruction(Instruction::new(
                    Opcode::CJump,
                    vec![
                        Value::Label(body_label.clone()),
                        Value::Label(end_label.clone()),
                    ],
                ));
                cond_block.successors.push(body_label.clone());
                cond_block.successors.push(end_label.clone());
                func.add_block(cond_block);
                
                // End block
                let end_block = BasicBlock::new(end_label.clone());
                func.add_block(end_block);
            }
        }
    }

    /// Build a case statement
    fn build_case_stmt(&mut self, case_stmt: &ast::CaseStmt) {
        // Evaluate selector expression (before borrowing func)
        let selector_val = self.build_expression(case_stmt.expr.as_ref());
        
        // Create labels and build branch instructions (before borrowing func)
        let mut case_labels = Vec::new();
        let end_label = self.new_label("case_end");
        let mut branch_instructions = Vec::new();
        
        for (i, branch) in case_stmt.cases.iter().enumerate() {
            let branch_label = self.new_label(&format!("case_branch_{}", i));
            case_labels.push(branch_label.clone());
            
            // Build branch statement instructions
            let instructions = if let Node::Block(block) = branch.statement.as_ref() {
                self.build_statements_to_instructions(&block.statements)
            } else {
                // Single statement
                self.build_statements_to_instructions(&[branch.statement.as_ref().clone()])
            };
            branch_instructions.push((branch_label, instructions));
        }
        
        // Build else branch instructions if present (before borrowing func)
        let else_instructions = if let Some(else_block) = &case_stmt.else_branch {
            let else_label = self.new_label("case_else");
            let instructions = if let Node::Block(block) = else_block.as_ref() {
                self.build_statements_to_instructions(&block.statements)
            } else {
                // Single statement
                self.build_statements_to_instructions(&[else_block.as_ref().clone()])
            };
            Some((else_label, instructions))
        } else {
            None
        };
        
        // Build case value expressions (before borrowing func)
        let mut case_value_pairs = Vec::new();
        for (i, branch) in case_stmt.cases.iter().enumerate() {
            for case_val_expr in &branch.values {
                let case_val = self.build_expression(case_val_expr);
                case_value_pairs.push((i, case_val));
            }
        }
        
        if let Some(func) = self.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                // Build dispatch logic in entry block
                // Compare selector with each case value and jump to appropriate branch
                for (i, case_val) in case_value_pairs {
                    entry.add_instruction(Instruction::new(
                        Opcode::Cmp,
                        vec![selector_val.clone(), case_val],
                    ));
                    entry.add_instruction(Instruction::new(
                        Opcode::CJump,
                        vec![
                            Value::Label(case_labels[i].clone()),
                            Value::Label(end_label.clone()), // Default: continue to next case
                        ],
                    ));
                }
                
                // Update entry block successors (before dropping entry borrow)
                entry.successors.extend(case_labels.clone());
                entry.successors.push(end_label.clone());
            } // Drop entry borrow here
            
            // Now we can add blocks without conflicting borrows
            // Add case branch blocks
            for (branch_label, instructions) in branch_instructions {
                let mut branch_block = BasicBlock::new(branch_label.clone());
                branch_block.instructions = instructions;
                // Jump to end after branch
                branch_block.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(end_label.clone())],
                ));
                branch_block.successors.push(end_label.clone());
                func.add_block(branch_block);
            }
            
            // Handle else branch if present
            if let Some((else_label, instructions)) = else_instructions {
                let mut else_block_ir = BasicBlock::new(else_label.clone());
                else_block_ir.instructions = instructions;
                else_block_ir.add_instruction(Instruction::new(
                    Opcode::Jump,
                    vec![Value::Label(end_label.clone())],
                ));
                else_block_ir.successors.push(end_label.clone());
                func.add_block(else_block_ir);
            }
            
            // End block
            let end_block = BasicBlock::new(end_label.clone());
            func.add_block(end_block);
        }
    }

    /// Get the built program
    pub fn into_program(self) -> Program {
        self.program
    }

    // ===== Array Runtime Support =====
    // These helper functions generate IR calls to Array runtime functions

    /// Generate IR for array slicing: array_slice(array_ptr, array_length, element_size, start, end, step)
    /// Returns a temporary value with the ArraySlice pointer result
    pub fn generate_array_slice(
        &mut self,
        array_ptr: Value,
        array_length: Value,
        element_size: Value,
        start: Value,
        end: Value,
        step: Value,
    ) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        // CALL array_slice, array_ptr, array_length, element_size, start, end, step -> result_temp
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("array_slice".to_string()),
                array_ptr,
                array_length,
                element_size,
                start,
                end,
                step,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
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

    // ===== Interface Runtime Support =====
    // These helper functions generate IR calls to interface runtime functions
    // They will be used when AST to IR conversion is implemented for interfaces

    /// Generate IR for adding a reference to an interface: interface_add_ref(interface_ptr)
    /// Returns a temporary value with the new reference count
    pub fn generate_interface_add_ref(&mut self, interface_ptr: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("interface_add_ref".to_string()),
                interface_ptr,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for releasing a reference to an interface: interface_release(interface_ptr)
    /// Returns a temporary value with the new reference count (0 if object was freed)
    pub fn generate_interface_release(&mut self, interface_ptr: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("interface_release".to_string()),
                interface_ptr,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for assigning an interface: interface_assign(dest_ptr, source_ptr)
    /// This handles reference counting: releases old interface, adds reference to new
    /// Returns a temporary value with the assigned interface pointer
    pub fn generate_interface_assign(&mut self, dest_ptr: Value, source_ptr: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("interface_assign".to_string()),
                dest_ptr,
                source_ptr,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
    }

    /// Generate IR for creating an interface from an object: interface_from_object(object_ptr, guid)
    /// This is called when casting an object to an interface
    /// Returns a temporary value with the interface pointer
    pub fn generate_interface_from_object(&mut self, object_ptr: Value, guid: Option<Value>) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let mut operands = vec![
            Value::Label("interface_from_object".to_string()),
            object_ptr,
        ];
        if let Some(guid_val) = guid {
            operands.push(guid_val);
        } else {
            operands.push(Value::Immediate(0)); // Null GUID
        }
        operands.push(result_temp.clone());
        
        let inst = Instruction::new(Opcode::Call, operands);
        (inst, result_temp)
    }

    /// Generate IR for querying an interface: interface_query_interface(object_ptr, guid)
    /// Returns a temporary value with the interface pointer (0 if not supported)
    pub fn generate_interface_query_interface(&mut self, object_ptr: Value, guid: Value) -> (Instruction, Value) {
        let result_temp = self.new_temp();
        let inst = Instruction::new(
            Opcode::Call,
            vec![
                Value::Label("interface_query_interface".to_string()),
                object_ptr,
                guid,
                result_temp.clone(),
            ],
        );
        (inst, result_temp)
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

    // ===== Interface Runtime Support Tests =====

    #[test]
    fn test_generate_interface_add_ref() {
        let mut builder = IRBuilder::new();
        let interface_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let (inst, result) = builder.generate_interface_add_ref(interface_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.operands[0], Value::Label("interface_add_ref".to_string()));
        assert_eq!(inst.operands[1], interface_ptr);
        assert_eq!(inst.operands[2], result);
    }

    #[test]
    fn test_generate_interface_release() {
        let mut builder = IRBuilder::new();
        let interface_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let (inst, result) = builder.generate_interface_release(interface_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.operands[0], Value::Label("interface_release".to_string()));
        assert_eq!(inst.operands[1], interface_ptr);
        assert_eq!(inst.operands[2], result);
    }

    #[test]
    fn test_generate_interface_assign() {
        let mut builder = IRBuilder::new();
        let dest_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let source_ptr = Value::Memory { base: "sp".to_string(), offset: -8 };
        let (inst, result) = builder.generate_interface_assign(dest_ptr.clone(), source_ptr.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("interface_assign".to_string()));
        assert_eq!(inst.operands[1], dest_ptr);
        assert_eq!(inst.operands[2], source_ptr);
        assert_eq!(inst.operands[3], result);
    }

    #[test]
    fn test_generate_interface_from_object() {
        let mut builder = IRBuilder::new();
        let object_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let guid = Value::Label("{12345678-1234-1234-1234-123456789ABC}".to_string());
        let (inst, result) = builder.generate_interface_from_object(object_ptr.clone(), Some(guid.clone()));

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("interface_from_object".to_string()));
        assert_eq!(inst.operands[1], object_ptr);
        assert_eq!(inst.operands[2], guid);
        assert_eq!(inst.operands[3], result);
    }

    #[test]
    fn test_generate_interface_from_object_no_guid() {
        let mut builder = IRBuilder::new();
        let object_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let (inst, result) = builder.generate_interface_from_object(object_ptr.clone(), None);

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("interface_from_object".to_string()));
        assert_eq!(inst.operands[1], object_ptr);
        assert_eq!(inst.operands[2], Value::Immediate(0)); // Null GUID
        assert_eq!(inst.operands[3], result);
    }

    #[test]
    fn test_generate_interface_query_interface() {
        let mut builder = IRBuilder::new();
        let object_ptr = Value::Memory { base: "sp".to_string(), offset: -4 };
        let guid = Value::Label("{87654321-4321-4321-4321-CBA987654321}".to_string());
        let (inst, result) = builder.generate_interface_query_interface(object_ptr.clone(), guid.clone());

        assert_eq!(inst.opcode, Opcode::Call);
        assert_eq!(inst.operands.len(), 4);
        assert_eq!(inst.operands[0], Value::Label("interface_query_interface".to_string()));
        assert_eq!(inst.operands[1], object_ptr);
        assert_eq!(inst.operands[2], guid);
        assert_eq!(inst.operands[3], result);
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

    // ===== AST→IR Conversion Tests =====

    #[test]
    fn test_build_if_statement() {
        use ast::{IfStmt, LiteralExpr, LiteralValue};
        use tokens::Span;
        
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Create: if true then x := 1 else x := 2
        let condition = Node::LiteralExpr(LiteralExpr {
            value: LiteralValue::Boolean(true),
            span: Span::at(0, 1, 1),
        });
        
        let then_stmt = Node::AssignStmt(ast::AssignStmt {
            target: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "x".to_string(),
                span: Span::at(10, 1, 11),
            })),
            value: Box::new(Node::LiteralExpr(LiteralExpr {
                value: LiteralValue::Integer(1),
                span: Span::at(14, 1, 15),
            })),
            span: Span::at(10, 1, 15),
        });
        
        let else_stmt = Node::AssignStmt(ast::AssignStmt {
            target: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "x".to_string(),
                span: Span::at(20, 1, 21),
            })),
            value: Box::new(Node::LiteralExpr(LiteralExpr {
                value: LiteralValue::Integer(2),
                span: Span::at(24, 1, 25),
            })),
            span: Span::at(20, 1, 25),
        });
        
        let if_stmt = IfStmt {
            condition: Box::new(condition),
            then_block: Box::new(then_stmt),
            else_block: Some(Box::new(else_stmt)),
            span: Span::at(0, 1, 25),
        };
        
        builder.build_if_stmt(&if_stmt);
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        // Should have entry, then, else, and end blocks
        assert!(func.blocks.len() >= 4);
    }

    #[test]
    fn test_build_while_statement() {
        use ast::{WhileStmt, LiteralExpr, LiteralValue};
        use tokens::Span;
        
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Create: while x < 10 do x := x + 1
        let condition = Node::BinaryExpr(ast::BinaryExpr {
            op: ast::BinaryOp::Less,
            left: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "x".to_string(),
                span: Span::at(6, 1, 7),
            })),
            right: Box::new(Node::LiteralExpr(LiteralExpr {
                value: LiteralValue::Integer(10),
                span: Span::at(10, 1, 12),
            })),
            span: Span::at(6, 1, 12),
        });
        
        let body = Node::AssignStmt(ast::AssignStmt {
            target: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "x".to_string(),
                span: Span::at(16, 1, 17),
            })),
            value: Box::new(Node::BinaryExpr(ast::BinaryExpr {
                op: ast::BinaryOp::Add,
                left: Box::new(Node::IdentExpr(ast::IdentExpr {
                    name: "x".to_string(),
                    span: Span::at(20, 1, 21),
                })),
                right: Box::new(Node::LiteralExpr(LiteralExpr {
                    value: LiteralValue::Integer(1),
                    span: Span::at(24, 1, 25),
                })),
                span: Span::at(20, 1, 25),
            })),
            span: Span::at(16, 1, 25),
        });
        
        let while_stmt = WhileStmt {
            condition: Box::new(condition),
            body: Box::new(body),
            span: Span::at(0, 1, 25),
        };
        
        builder.build_while_stmt(&while_stmt);
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        // Should have entry, loop, body, and end blocks
        assert!(func.blocks.len() >= 4);
    }

    #[test]
    fn test_build_for_statement() {
        use ast::{ForStmt, ForDirection, LiteralExpr, LiteralValue};
        use tokens::Span;
        
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Create: for i := 1 to 10 do x := x + i
        let for_stmt = ForStmt {
            var_name: "i".to_string(),
            start_expr: Box::new(Node::LiteralExpr(LiteralExpr {
                value: LiteralValue::Integer(1),
                span: Span::at(8, 1, 9),
            })),
            direction: ForDirection::To,
            end_expr: Box::new(Node::LiteralExpr(LiteralExpr {
                value: LiteralValue::Integer(10),
                span: Span::at(13, 1, 15),
            })),
            body: Box::new(Node::AssignStmt(ast::AssignStmt {
                target: Box::new(Node::IdentExpr(ast::IdentExpr {
                    name: "x".to_string(),
                    span: Span::at(19, 1, 20),
                })),
                value: Box::new(Node::BinaryExpr(ast::BinaryExpr {
                    op: ast::BinaryOp::Add,
                    left: Box::new(Node::IdentExpr(ast::IdentExpr {
                        name: "x".to_string(),
                        span: Span::at(23, 1, 24),
                    })),
                    right: Box::new(Node::IdentExpr(ast::IdentExpr {
                        name: "i".to_string(),
                        span: Span::at(27, 1, 28),
                    })),
                    span: Span::at(23, 1, 28),
                })),
                span: Span::at(19, 1, 28),
            })),
            span: Span::at(0, 1, 28),
        };
        
        builder.build_for_stmt(&for_stmt);
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        // Should have entry, loop, body, inc, and end blocks
        assert!(func.blocks.len() >= 5);
    }

    #[test]
    fn test_build_repeat_statement() {
        use ast::{RepeatStmt, LiteralExpr, LiteralValue};
        use tokens::Span;
        
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Create: repeat x := x + 1 until x > 10
        let repeat_stmt = RepeatStmt {
            statements: vec![
                Node::AssignStmt(ast::AssignStmt {
                    target: Box::new(Node::IdentExpr(ast::IdentExpr {
                        name: "x".to_string(),
                        span: Span::at(7, 1, 8),
                    })),
                    value: Box::new(Node::BinaryExpr(ast::BinaryExpr {
                        op: ast::BinaryOp::Add,
                        left: Box::new(Node::IdentExpr(ast::IdentExpr {
                            name: "x".to_string(),
                            span: Span::at(11, 1, 12),
                        })),
                        right: Box::new(Node::LiteralExpr(LiteralExpr {
                            value: LiteralValue::Integer(1),
                            span: Span::at(15, 1, 16),
                        })),
                        span: Span::at(11, 1, 16),
                    })),
                    span: Span::at(7, 1, 16),
                }),
            ],
            condition: Box::new(Node::BinaryExpr(ast::BinaryExpr {
                op: ast::BinaryOp::Greater,
                left: Box::new(Node::IdentExpr(ast::IdentExpr {
                    name: "x".to_string(),
                    span: Span::at(23, 1, 24),
                })),
                right: Box::new(Node::LiteralExpr(LiteralExpr {
                    value: LiteralValue::Integer(10),
                    span: Span::at(27, 1, 29),
                })),
                span: Span::at(23, 1, 29),
            })),
            span: Span::at(0, 1, 29),
        };
        
        builder.build_repeat_stmt(&repeat_stmt);
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        // Should have entry, body, cond, and end blocks
        assert!(func.blocks.len() >= 4);
    }

    #[test]
    fn test_build_case_statement() {
        use ast::{CaseStmt, CaseBranch, LiteralExpr, LiteralValue};
        use tokens::Span;
        
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Create: case x of 1: y := 10; 2: y := 20; else y := 0; end
        let case_stmt = CaseStmt {
            expr: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "x".to_string(),
                span: Span::at(5, 1, 6),
            })),
            cases: vec![
                CaseBranch {
                    values: vec![
                        Node::LiteralExpr(LiteralExpr {
                            value: LiteralValue::Integer(1),
                            span: Span::at(10, 1, 11),
                        }),
                    ],
                    statement: Box::new(Node::AssignStmt(ast::AssignStmt {
                        target: Box::new(Node::IdentExpr(ast::IdentExpr {
                            name: "y".to_string(),
                            span: Span::at(13, 1, 14),
                        })),
                        value: Box::new(Node::LiteralExpr(LiteralExpr {
                            value: LiteralValue::Integer(10),
                            span: Span::at(17, 1, 19),
                        })),
                        span: Span::at(13, 1, 19),
                    })),
                    span: Span::at(10, 1, 19),
                },
                CaseBranch {
                    values: vec![
                        Node::LiteralExpr(LiteralExpr {
                            value: LiteralValue::Integer(2),
                            span: Span::at(21, 1, 22),
                        }),
                    ],
                    statement: Box::new(Node::AssignStmt(ast::AssignStmt {
                        target: Box::new(Node::IdentExpr(ast::IdentExpr {
                            name: "y".to_string(),
                            span: Span::at(24, 1, 25),
                        })),
                        value: Box::new(Node::LiteralExpr(LiteralExpr {
                            value: LiteralValue::Integer(20),
                            span: Span::at(28, 1, 30),
                        })),
                        span: Span::at(24, 1, 30),
                    })),
                    span: Span::at(21, 1, 30),
                },
            ],
            else_branch: Some(Box::new(Node::AssignStmt(ast::AssignStmt {
                target: Box::new(Node::IdentExpr(ast::IdentExpr {
                    name: "y".to_string(),
                    span: Span::at(37, 1, 38),
                })),
                value: Box::new(Node::LiteralExpr(LiteralExpr {
                    value: LiteralValue::Integer(0),
                    span: Span::at(41, 1, 42),
                })),
                span: Span::at(37, 1, 42),
            }))),
            span: Span::at(0, 1, 42),
        };
        
        builder.build_case_stmt(&case_stmt);
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        // Should have entry, 2 case branches, else, and end blocks
        assert!(func.blocks.len() >= 5);
    }

    #[test]
    fn test_build_call_statement() {
        use ast::CallStmt;
        use tokens::Span;
        
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Create: WriteLn(42, "hello")
        let call_stmt = CallStmt {
            name: "WriteLn".to_string(),
            args: vec![
                Node::LiteralExpr(ast::LiteralExpr {
                    value: ast::LiteralValue::Integer(42),
                    span: Span::at(8, 1, 10),
                }),
                Node::LiteralExpr(ast::LiteralExpr {
                    value: ast::LiteralValue::String("hello".to_string()),
                    span: Span::at(12, 1, 19),
                }),
            ],
            span: Span::at(0, 1, 19),
        };
        
        builder.build_call_stmt(&call_stmt);
        builder.finish_function();
        
        let program = builder.into_program();
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        // Should have a CALL instruction in the entry block
        let entry = func.blocks.iter().find(|b| b.label == func.entry_block).unwrap();
        assert!(entry.instructions.iter().any(|inst| inst.opcode == Opcode::Call));
    }

    #[test]
    fn test_build_expression_binary_ops() {
        use ast::{BinaryExpr, BinaryOp, LiteralExpr, LiteralValue};
        use tokens::Span;
        
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Test: 5 + 3
        let expr = Node::BinaryExpr(BinaryExpr {
            op: BinaryOp::Add,
            left: Box::new(Node::LiteralExpr(LiteralExpr {
                value: LiteralValue::Integer(5),
                span: Span::at(0, 1, 1),
            })),
            right: Box::new(Node::LiteralExpr(LiteralExpr {
                value: LiteralValue::Integer(3),
                span: Span::at(4, 1, 5),
            })),
            span: Span::at(0, 1, 5),
        });
        
        let result = builder.build_expression(&expr);
        builder.finish_function();
        
        // Should return a temp value
        assert!(matches!(result, Value::Temp(_)));
        
        let program = builder.into_program();
        let func = &program.functions[0];
        let entry = func.blocks.iter().find(|b| b.label == func.entry_block).unwrap();
        // Should have an ADD instruction
        assert!(entry.instructions.iter().any(|inst| inst.opcode == Opcode::Add));
    }

    // ===== Procedural Type Tests =====

    #[test]
    fn test_procedural_type_creation() {
        use types::Type;
        
        // Create a procedural type: function(x, y: integer): integer
        let proc_type = Type::procedural(
            true, // is_function
            vec![Type::integer(), Type::integer()], // params
            Some(Type::integer()), // return_type
            false, // is_method_pointer
        );
        
        // Verify it's a procedural type
        match proc_type {
            Type::Procedural { is_function, params, return_type, is_method_pointer } => {
                assert!(is_function);
                assert_eq!(params.len(), 2);
                assert_eq!(params[0], Type::integer());
                assert_eq!(params[1], Type::integer());
                assert_eq!(return_type, Some(Box::new(Type::integer())));
                assert!(!is_method_pointer);
            }
            _ => panic!("Expected Procedural type"),
        }
    }

    #[test]
    fn test_procedural_type_equality() {
        use types::Type;
        
        let proc1 = Type::procedural(
            true,
            vec![Type::integer()],
            Some(Type::integer()),
            false,
        );
        
        let proc2 = Type::procedural(
            true,
            vec![Type::integer()],
            Some(Type::integer()),
            false,
        );
        
        let proc3 = Type::procedural(
            false, // procedure, not function
            vec![Type::integer()],
            None,
            false,
        );
        
        assert!(proc1.equals(&proc2));
        assert!(!proc1.equals(&proc3));
    }

    #[test]
    fn test_procedural_type_assignment_compatibility() {
        use types::Type;
        
        let proc1 = Type::procedural(
            true,
            vec![Type::integer()],
            Some(Type::integer()),
            false,
        );
        
        let proc2 = Type::procedural(
            true,
            vec![Type::integer()],
            Some(Type::integer()),
            false,
        );
        
        // Same procedural types should be assignable
        assert!(proc1.is_assignable_to(&proc2));
        
        // Different parameter types should not be assignable
        let proc3 = Type::procedural(
            true,
            vec![Type::boolean()], // Different param type
            Some(Type::integer()),
            false,
        );
        assert!(!proc1.is_assignable_to(&proc3));
    }

    #[test]
    fn test_generate_array_slice() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        let array_ptr = Value::Temp(1);
        let array_length = Value::Immediate(10);
        let element_size = Value::Immediate(4);
        let start = Value::Immediate(1);
        let end = Value::Immediate(5);
        let step = Value::Immediate(1);
        
        let (inst, result) = builder.generate_array_slice(
            array_ptr,
            array_length,
            element_size,
            start,
            end,
            step,
        );
        
        // Verify instruction
        assert_eq!(inst.opcode, Opcode::Call);
        if let Value::Label(label) = &inst.operands[0] {
            assert_eq!(label, "array_slice");
        } else {
            panic!("Expected Label for function name");
        }
        
        // Verify result is a temp
        assert!(matches!(result, Value::Temp(_)));
    }

    #[test]
    fn test_build_slice_expression() {
        let mut builder = IRBuilder::new();
        builder.start_function("test".to_string(), None);
        
        // Create a slice expression: arr[1:5]
        let slice = Node::SliceExpr(ast::SliceExpr {
            array: Box::new(Node::IdentExpr(ast::IdentExpr {
                name: "arr".to_string(),
                span: Span::new(0, 3, 1, 1),
            })),
            start: Some(Box::new(Node::LiteralExpr(ast::LiteralExpr {
                value: ast::LiteralValue::Integer(1),
                span: Span::new(5, 6, 1, 1),
            }))),
            end: Some(Box::new(Node::LiteralExpr(ast::LiteralExpr {
                value: ast::LiteralValue::Integer(5),
                span: Span::new(7, 8, 1, 1),
            }))),
            step: None,
            span: Span::new(0, 8, 1, 1),
        });
        
        let result = builder.build_expression(&slice);
        
        // Verify result is a temp
        assert!(matches!(result, Value::Temp(_)));
        
        // Verify array_slice instruction was generated
        if let Some(func) = builder.current_function_mut() {
            let entry_label = func.entry_block.clone();
            if let Some(entry) = func.get_block_mut(&entry_label) {
                let slice_inst = entry.instructions.iter()
                    .find(|inst| inst.opcode == Opcode::Call && 
                          matches!(&inst.operands[0], Value::Label(label) if label == "array_slice"));
                assert!(slice_inst.is_some(), "Should have array_slice instruction");
            }
        }
    }
}
