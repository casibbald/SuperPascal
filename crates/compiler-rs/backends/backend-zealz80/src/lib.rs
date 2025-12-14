//! SuperPascal Z80 Backend (ZealZ80)
//!
//! This crate implements code generation from SuperPascal IR to Z80 assembly
//! for the ZealZ80 platform (Zilog Z80 @ 10 MHz).
//!
//! # Architecture
//!
//! - **Frame Pointer**: IX (callee-saved)
//! - **Return Value**: HL (16-bit), L (8-bit)
//! - **Scratch Registers**: AF, BC, DE, HL
//! - **Stack**: Grows downward, Pascal convention (callee cleans)
//!
//! # ABI Reference
//!
//! See `platforms/ZealZ80/ABI.md` for complete ABI specification.

use ir::{BasicBlock, Function, Instruction, Opcode, Program, Value};
use std::fmt;

/// Z80 register names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Z80Register {
    /// 8-bit registers
    A, B, C, D, E, H, L,
    /// 16-bit register pairs
    AF, BC, DE, HL,
    /// Index registers
    IX, IY,
    /// Stack pointer
    SP,
}

impl fmt::Display for Z80Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Z80Register::A => write!(f, "a"),
            Z80Register::B => write!(f, "b"),
            Z80Register::C => write!(f, "c"),
            Z80Register::D => write!(f, "d"),
            Z80Register::E => write!(f, "e"),
            Z80Register::H => write!(f, "h"),
            Z80Register::L => write!(f, "l"),
            Z80Register::AF => write!(f, "af"),
            Z80Register::BC => write!(f, "bc"),
            Z80Register::DE => write!(f, "de"),
            Z80Register::HL => write!(f, "hl"),
            Z80Register::IX => write!(f, "ix"),
            Z80Register::IY => write!(f, "iy"),
            Z80Register::SP => write!(f, "sp"),
        }
    }
}

/// Z80 assembly instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Z80Instruction {
    /// Load register with immediate: `ld reg, value`
    LoadImmediate { reg: Z80Register, value: u16 },
    /// Load register from register: `ld dst, src`
    LoadRegister { dst: Z80Register, src: Z80Register },
    /// Load register from memory: `ld reg, (addr)` or `ld reg, (ix+offset)`
    LoadMemory { reg: Z80Register, addr: MemoryAddress },
    /// Store register to memory: `ld (addr), reg` or `ld (ix+offset), reg`
    StoreMemory { addr: MemoryAddress, reg: Z80Register },
    /// Push register onto stack: `push reg`
    Push { reg: Z80Register },
    /// Pop register from stack: `pop reg`
    Pop { reg: Z80Register },
    /// Add: `add hl, reg` or `add a, reg`
    Add { dst: Z80Register, src: Z80Register },
    /// Subtract: `sub reg` or `sbc hl, reg`
    Subtract { dst: Z80Register, src: Z80Register },
    /// Compare: `cp value` or `cp reg`
    Compare { reg: Z80Register, value: Option<u8> },
    /// Unconditional jump: `jp label` or `jr label`
    Jump { label: String, near: bool },
    /// Conditional jump: `jp cc, label` or `jr cc, label`
    JumpConditional { condition: Condition, label: String, near: bool },
    /// Call function: `call label`
    Call { label: String },
    /// Return: `ret`
    Return,
    /// Label definition: `label:`
    Label { name: String },
    /// Comment: `; comment`
    Comment { text: String },
}

/// Memory address for load/store operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryAddress {
    /// Direct address: `(nnnn)`
    Direct(u16),
    /// Frame-relative: `(ix+offset)` or `(ix-offset)`
    FrameRelative(i16),
    /// Register indirect: `(hl)`, `(bc)`, `(de)`
    RegisterIndirect(Z80Register),
}

/// Condition codes for conditional jumps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    /// Zero flag set (Z)
    Zero,
    /// Zero flag clear (NZ)
    NonZero,
    /// Carry flag set (C)
    Carry,
    /// Carry flag clear (NC)
    NoCarry,
    /// Sign flag set (M)
    Sign,
    /// Sign flag clear (P)
    Positive,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Zero => write!(f, "z"),
            Condition::NonZero => write!(f, "nz"),
            Condition::Carry => write!(f, "c"),
            Condition::NoCarry => write!(f, "nc"),
            Condition::Sign => write!(f, "m"),
            Condition::Positive => write!(f, "p"),
        }
    }
}

/// Z80 code generator
pub struct CodeGenerator {
    /// Current function being generated
    current_function: Option<String>,
    /// Local variable offset from frame pointer
    local_offset: i16,
    /// Temporary counter for SSA temporaries
    #[allow(dead_code)] // Reserved for future SSA temporary generation
    temp_counter: usize,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new() -> Self {
        Self {
            current_function: None,
            local_offset: 0,
            temp_counter: 0,
        }
    }

    /// Generate Z80 assembly from IR program
    pub fn generate(&mut self, program: &Program) -> Vec<Z80Instruction> {
        let mut instructions = Vec::new();

        // Generate code for each function
        for function in &program.functions {
            instructions.extend(self.generate_function(function));
        }

        // Apply jump optimization (iterative, Turbo Pascal style)
        self.optimize_jumps(&mut instructions);

        instructions
    }

    /// Generate code for a function
    fn generate_function(&mut self, function: &Function) -> Vec<Z80Instruction> {
        let mut instructions = Vec::new();
        
        self.current_function = Some(function.name.clone());
        self.local_offset = 0;

        // Function label
        instructions.push(Z80Instruction::Label {
            name: self.mangle_name(&function.name),
        });

        // Function prologue
        instructions.extend(self.generate_prologue(function));

        // Generate code for each basic block
        for block in &function.blocks {
            instructions.extend(self.generate_block(block));
        }

        // Function epilogue
        instructions.extend(self.generate_epilogue(function));

        self.current_function = None;
        instructions
    }

    /// Generate function prologue
    fn generate_prologue(&mut self, function: &Function) -> Vec<Z80Instruction> {
        let mut instructions = Vec::new();

        // Calculate local variable size
        let local_size = self.calculate_local_size(function);
        
        if local_size > 0 {
            // Save frame pointer
            instructions.push(Z80Instruction::Push { reg: Z80Register::IX });
            
            // Set IX to current SP
            instructions.push(Z80Instruction::LoadRegister {
                dst: Z80Register::IX,
                src: Z80Register::SP,
            });
            
            // Allocate local variables: SP = SP - local_size
            instructions.push(Z80Instruction::LoadImmediate {
                reg: Z80Register::HL,
                value: local_size as u16,
            });
            instructions.push(Z80Instruction::Subtract {
                dst: Z80Register::SP,
                src: Z80Register::HL,
            });
        } else {
            // No locals, just save frame pointer
            instructions.push(Z80Instruction::Push { reg: Z80Register::IX });
            instructions.push(Z80Instruction::LoadRegister {
                dst: Z80Register::IX,
                src: Z80Register::SP,
            });
        }

        instructions
    }

    /// Generate function epilogue
    fn generate_epilogue(&mut self, _function: &Function) -> Vec<Z80Instruction> {
        let mut instructions = Vec::new();

        // Restore SP from IX
        instructions.push(Z80Instruction::LoadRegister {
            dst: Z80Register::SP,
            src: Z80Register::IX,
        });

        // Restore frame pointer
        instructions.push(Z80Instruction::Pop { reg: Z80Register::IX });

        // Return
        instructions.push(Z80Instruction::Return);

        instructions
    }

    /// Generate code for a basic block
    fn generate_block(&mut self, block: &BasicBlock) -> Vec<Z80Instruction> {
        let mut instructions = Vec::new();

        // Block label
        instructions.push(Z80Instruction::Label {
            name: block.label.clone(),
        });

        // Generate code for each instruction
        for ir_inst in &block.instructions {
            instructions.extend(self.generate_instruction(ir_inst));
        }

        instructions
    }

    /// Generate code for an IR instruction
    fn generate_instruction(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        match &inst.opcode {
            Opcode::Mov => self.generate_mov(inst),
            Opcode::Add => self.generate_add(inst),
            Opcode::Sub => self.generate_sub(inst),
            Opcode::Cmp => self.generate_cmp(inst),
            Opcode::Jump => self.generate_jump(inst),
            Opcode::CJump => self.generate_cjump(inst),
            Opcode::Call => self.generate_call(inst),
            Opcode::Ret => self.generate_ret(inst),
            Opcode::Load => self.generate_load(inst),
            Opcode::Store => self.generate_store(inst),
            Opcode::Push => self.generate_push(inst),
            Opcode::Pop => self.generate_pop(inst),
            _ => {
                // Unimplemented opcodes
                vec![Z80Instruction::Comment {
                    text: format!("TODO: {:?}", inst.opcode),
                }]
            }
        }
    }

    /// Generate MOV instruction
    fn generate_mov(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.len() < 2 {
            return vec![];
        }

        let dst = &inst.operands[0];
        let src = &inst.operands[1];

        match (dst, src) {
            (Value::Register(dst_reg), Value::Immediate(imm)) => {
                vec![Z80Instruction::LoadImmediate {
                    reg: self.parse_register(dst_reg),
                    value: *imm as u16,
                }]
            }
            (Value::Register(dst_reg), Value::Register(src_reg)) => {
                vec![Z80Instruction::LoadRegister {
                    dst: self.parse_register(dst_reg),
                    src: self.parse_register(src_reg),
                }]
            }
            (Value::Memory { base: _, offset }, Value::Immediate(imm)) => {
                vec![
                    Z80Instruction::LoadImmediate {
                        reg: Z80Register::HL,
                        value: *imm as u16,
                    },
                    Z80Instruction::StoreMemory {
                        addr: MemoryAddress::FrameRelative(*offset as i16),
                        reg: Z80Register::HL,
                    },
                ]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: MOV {:?} <- {:?}", dst, src),
            }],
        }
    }

    /// Generate ADD instruction
    fn generate_add(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.len() < 3 {
            return vec![];
        }

        let dst = &inst.operands[0];
        let src1 = &inst.operands[1];
        let src2 = &inst.operands[2];

        // Load src1 into HL
        let mut instructions = self.load_value_into_hl(src1);
        
        // Add src2 to HL
        match src2 {
            Value::Immediate(imm) => {
                instructions.push(Z80Instruction::LoadImmediate {
                    reg: Z80Register::DE,
                    value: *imm as u16,
                });
                instructions.push(Z80Instruction::Add {
                    dst: Z80Register::HL,
                    src: Z80Register::DE,
                });
            }
            Value::Register(reg) => {
                instructions.push(Z80Instruction::Add {
                    dst: Z80Register::HL,
                    src: self.parse_register(reg),
                });
            }
            _ => {
                instructions.push(Z80Instruction::Comment {
                    text: format!("TODO: ADD src2 {:?}", src2),
                });
            }
        }

        // Store HL to dst
        instructions.extend(self.store_hl_to_value(dst));

        instructions
    }

    /// Generate SUB instruction
    fn generate_sub(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.len() < 3 {
            return vec![];
        }

        let dst = &inst.operands[0];
        let src1 = &inst.operands[1];
        let src2 = &inst.operands[2];

        // Load src1 into HL
        let mut instructions = self.load_value_into_hl(src1);
        
        // Subtract src2 from HL
        match src2 {
            Value::Immediate(imm) => {
                instructions.push(Z80Instruction::LoadImmediate {
                    reg: Z80Register::DE,
                    value: *imm as u16,
                });
                instructions.push(Z80Instruction::Subtract {
                    dst: Z80Register::HL,
                    src: Z80Register::DE,
                });
            }
            Value::Register(reg) => {
                instructions.push(Z80Instruction::Subtract {
                    dst: Z80Register::HL,
                    src: self.parse_register(reg),
                });
            }
            _ => {
                instructions.push(Z80Instruction::Comment {
                    text: format!("TODO: SUB src2 {:?}", src2),
                });
            }
        }

        // Store HL to dst
        instructions.extend(self.store_hl_to_value(dst));

        instructions
    }

    /// Generate CMP instruction
    fn generate_cmp(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.len() < 2 {
            return vec![];
        }

        let src1 = &inst.operands[0];
        let src2 = &inst.operands[1];

        // Load src1 into A
        let mut instructions = self.load_value_into_a(src1);
        
        // Compare with src2
        match src2 {
            Value::Immediate(imm) => {
                instructions.push(Z80Instruction::Compare {
                    reg: Z80Register::A,
                    value: Some(*imm as u8),
                });
            }
            Value::Register(_reg) => {
                instructions.push(Z80Instruction::Compare {
                    reg: Z80Register::A,
                    value: None,
                });
            }
            _ => {
                instructions.push(Z80Instruction::Comment {
                    text: format!("TODO: CMP src2 {:?}", src2),
                });
            }
        }

        instructions
    }

    /// Generate JUMP instruction
    fn generate_jump(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.is_empty() {
            return vec![];
        }

        match &inst.operands[0] {
            Value::Label(label) => {
                vec![Z80Instruction::Jump {
                    label: label.clone(),
                    near: false, // TODO: Determine if near jump is possible
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: JUMP {:?}", inst.operands),
            }],
        }
    }

    /// Generate CJUMP instruction
    fn generate_cjump(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.len() < 3 {
            return vec![];
        }

        // CJUMP condition, label_true, label_false
        // The condition is set by a previous CMP instruction
        // For now, assume condition is in operands[0]
        
        let label_true = match &inst.operands[1] {
            Value::Label(l) => l.clone(),
            _ => return vec![],
        };
        
        let label_false = match &inst.operands[2] {
            Value::Label(l) => l.clone(),
            _ => return vec![],
        };

        // TODO: Map IR condition to Z80 condition code
        // For now, use zero/non-zero
        vec![
            Z80Instruction::JumpConditional {
                condition: Condition::Zero,
                label: label_true,
                near: false,
            },
            Z80Instruction::Jump {
                label: label_false,
                near: false,
            },
        ]
    }

    /// Generate CALL instruction
    fn generate_call(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.is_empty() {
            return vec![];
        }

        match &inst.operands[0] {
            Value::Label(label) => {
                vec![Z80Instruction::Call {
                    label: self.mangle_name(label),
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: CALL {:?}", inst.operands),
            }],
        }
    }

    /// Generate RET instruction
    fn generate_ret(&mut self, _inst: &Instruction) -> Vec<Z80Instruction> {
        vec![Z80Instruction::Return]
    }

    /// Generate LOAD instruction
    fn generate_load(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.len() < 2 {
            return vec![];
        }

        let dst = &inst.operands[0];
        let src = &inst.operands[1];

        match (dst, src) {
            (Value::Register(_), Value::Memory { base: _, offset }) => {
                vec![Z80Instruction::LoadMemory {
                    reg: Z80Register::HL,
                    addr: MemoryAddress::FrameRelative(*offset as i16),
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: LOAD {:?} <- {:?}", dst, src),
            }],
        }
    }

    /// Generate STORE instruction
    fn generate_store(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.len() < 2 {
            return vec![];
        }

        let dst = &inst.operands[0];
        let src = &inst.operands[1];

        match (dst, src) {
            (Value::Memory { base: _, offset }, Value::Register(_)) => {
                vec![Z80Instruction::StoreMemory {
                    addr: MemoryAddress::FrameRelative(*offset as i16),
                    reg: Z80Register::HL,
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: STORE {:?} <- {:?}", dst, src),
            }],
        }
    }

    /// Generate PUSH instruction
    fn generate_push(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.is_empty() {
            return vec![];
        }

        match &inst.operands[0] {
            Value::Register(reg) => {
                vec![Z80Instruction::Push {
                    reg: self.parse_register(reg),
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: PUSH {:?}", inst.operands),
            }],
        }
    }

    /// Generate POP instruction
    fn generate_pop(&mut self, inst: &Instruction) -> Vec<Z80Instruction> {
        if inst.operands.is_empty() {
            return vec![];
        }

        match &inst.operands[0] {
            Value::Register(reg) => {
                vec![Z80Instruction::Pop {
                    reg: self.parse_register(reg),
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: POP {:?}", inst.operands),
            }],
        }
    }

    // Helper methods

    /// Load a value into HL register
    fn load_value_into_hl(&self, value: &Value) -> Vec<Z80Instruction> {
        match value {
            Value::Immediate(imm) => {
                vec![Z80Instruction::LoadImmediate {
                    reg: Z80Register::HL,
                    value: *imm as u16,
                }]
            }
            Value::Register(reg) => {
                vec![Z80Instruction::LoadRegister {
                    dst: Z80Register::HL,
                    src: self.parse_register(reg),
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: load {:?} into HL", value),
            }],
        }
    }

    /// Load a value into A register
    fn load_value_into_a(&self, value: &Value) -> Vec<Z80Instruction> {
        match value {
            Value::Immediate(imm) => {
                vec![Z80Instruction::LoadImmediate {
                    reg: Z80Register::A,
                    value: *imm as u16,
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: load {:?} into A", value),
            }],
        }
    }

    /// Store HL register to a value
    fn store_hl_to_value(&self, value: &Value) -> Vec<Z80Instruction> {
        match value {
            Value::Register(reg) => {
                vec![Z80Instruction::LoadRegister {
                    dst: self.parse_register(reg),
                    src: Z80Register::HL,
                }]
            }
            Value::Memory { base: _, offset } => {
                vec![Z80Instruction::StoreMemory {
                    addr: MemoryAddress::FrameRelative(*offset as i16),
                    reg: Z80Register::HL,
                }]
            }
            _ => vec![Z80Instruction::Comment {
                text: format!("TODO: store HL to {:?}", value),
            }],
        }
    }

    /// Parse register name string to Z80Register
    fn parse_register(&self, name: &str) -> Z80Register {
        match name.to_lowercase().as_str() {
            "a" => Z80Register::A,
            "b" => Z80Register::B,
            "c" => Z80Register::C,
            "d" => Z80Register::D,
            "e" => Z80Register::E,
            "h" => Z80Register::H,
            "l" => Z80Register::L,
            "af" => Z80Register::AF,
            "bc" => Z80Register::BC,
            "de" => Z80Register::DE,
            "hl" => Z80Register::HL,
            "ix" => Z80Register::IX,
            "iy" => Z80Register::IY,
            "sp" => Z80Register::SP,
            _ => Z80Register::HL, // Default to HL
        }
    }

    /// Mangle function name for Z80 (add underscore prefix for now)
    fn mangle_name(&self, name: &str) -> String {
        format!("_{}", name)
    }

    /// Calculate total size of local variables
    fn calculate_local_size(&self, _function: &Function) -> usize {
        // TODO: Calculate from function parameters and local variables
        0
    }

    /// Optimize jumps: Convert JP (absolute, 3 bytes) to JR (relative, 2 bytes) when possible.
    /// 
    /// This implements Turbo Pascal's iterative jump optimization algorithm:
    /// 1. Calculate instruction offsets
    /// 2. For each jump, check if displacement fits in 8-bit signed range (-126 to +129)
    /// 3. Convert JP to JR if possible
    /// 4. Recalculate offsets (since JR is 1 byte shorter)
    /// 5. Repeat until no more changes
    /// 
    /// This is iterative because each optimization reduces code size, potentially
    /// enabling more jumps to be optimized.
    fn optimize_jumps(&self, instructions: &mut Vec<Z80Instruction>) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100; // Safety limit

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            // Calculate current instruction offsets
            let offsets = self.calculate_instruction_offsets(instructions);
            
            // Build label-to-offset map
            let mut label_offsets = std::collections::HashMap::new();
            for (idx, inst) in instructions.iter().enumerate() {
                if let Z80Instruction::Label { name } = inst {
                    if let Some(&offset) = offsets.get(&idx) {
                        label_offsets.insert(name.clone(), offset);
                    }
                }
            }

            // Try to convert each jump
            for (idx, inst) in instructions.iter_mut().enumerate() {
                if let Some(&jump_offset) = offsets.get(&idx) {
                    match inst {
                        Z80Instruction::Jump { label, near } if !*near => {
                            if let Some(&target_offset) = label_offsets.get(label) {
                                // Displacement is relative to the instruction AFTER the jump
                                // JP is 3 bytes, so next instruction is at jump_offset + 3
                                // JR would be 2 bytes, so next instruction would be at jump_offset + 2
                                // We calculate as if we're using JR (2 bytes)
                                let next_instruction_offset = jump_offset + 2;
                                let displacement = target_offset as i32 - next_instruction_offset as i32;
                                
                                // Z80 JR range: -128 to +127 (8-bit signed)
                                // Turbo Pascal uses -126 to +129 to account for instruction boundaries
                                if displacement >= -126 && displacement <= 129 {
                                    *near = true; // Convert to JR
                                    changed = true;
                                }
                            }
                        }
                        Z80Instruction::JumpConditional { label, near, .. } if !*near => {
                            if let Some(&target_offset) = label_offsets.get(label) {
                                // Same calculation for conditional jumps
                                let next_instruction_offset = jump_offset + 2;
                                let displacement = target_offset as i32 - next_instruction_offset as i32;
                                
                                if displacement >= -126 && displacement <= 129 {
                                    *near = true; // Convert to JR
                                    changed = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if iterations >= MAX_ITERATIONS {
            eprintln!("Warning: Jump optimization reached max iterations");
        }
    }

    /// Calculate byte offsets for each instruction in the instruction list.
    /// Returns a map from instruction index to byte offset.
    fn calculate_instruction_offsets(&self, instructions: &[Z80Instruction]) -> std::collections::HashMap<usize, usize> {
        let mut offsets = std::collections::HashMap::new();
        let mut current_offset = 0;

        for (idx, inst) in instructions.iter().enumerate() {
            offsets.insert(idx, current_offset);
            current_offset += self.instruction_size(inst);
        }

        offsets
    }

    /// Calculate the size in bytes of a Z80 instruction.
    /// This is used for offset calculation during jump optimization.
    fn instruction_size(&self, inst: &Z80Instruction) -> usize {
        match inst {
            // 1-byte instructions
            Z80Instruction::Return => 1,
            Z80Instruction::Label { .. } => 0, // Labels don't generate code
            
            // 2-byte instructions
            Z80Instruction::LoadImmediate { value, .. } => {
                if *value <= 0xFF {
                    2 // ld reg, 8-bit immediate
                } else {
                    3 // ld reg, 16-bit immediate
                }
            }
            Z80Instruction::LoadRegister { .. } => 1,
            Z80Instruction::Push { .. } => 1,
            Z80Instruction::Pop { .. } => 1,
            Z80Instruction::Add { .. } => 1,
            Z80Instruction::Subtract { .. } => 1,
            Z80Instruction::Compare { value, .. } => {
                if value.is_some() {
                    2 // cp, 8-bit immediate
                } else {
                    1 // cp, register
                }
            }
            
            // Jump instructions (size depends on near flag)
            Z80Instruction::Jump { near, .. } => {
                if *near {
                    2 // jr (relative, 2 bytes)
                } else {
                    3 // jp (absolute, 3 bytes)
                }
            }
            Z80Instruction::JumpConditional { near, .. } => {
                if *near {
                    2 // jr cc (relative, 2 bytes)
                } else {
                    3 // jp cc (absolute, 3 bytes)
                }
            }
            
            // 3-byte instructions
            Z80Instruction::Call { .. } => 3,
            
            // Memory operations (variable size)
            Z80Instruction::LoadMemory { addr, .. } => match addr {
                MemoryAddress::Direct(_) => 3, // ld reg, (nn)
                MemoryAddress::FrameRelative(_) => 3, // ld reg, (ix+d)
                MemoryAddress::RegisterIndirect(_) => 1, // ld reg, (hl)
            },
            Z80Instruction::StoreMemory { addr, .. } => match addr {
                MemoryAddress::Direct(_) => 3, // ld (nn), reg
                MemoryAddress::FrameRelative(_) => 3, // ld (ix+d), reg
                MemoryAddress::RegisterIndirect(_) => 1, // ld (hl), reg
            },
            
            // Comments don't generate code
            Z80Instruction::Comment { .. } => 0,
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Format Z80 instructions as assembly text
impl fmt::Display for Z80Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Z80Instruction::LoadImmediate { reg, value } => {
                write!(f, "    ld {}, {}", reg, value)
            }
            Z80Instruction::LoadRegister { dst, src } => {
                write!(f, "    ld {}, {}", dst, src)
            }
            Z80Instruction::LoadMemory { reg, addr } => {
                match addr {
                    MemoryAddress::Direct(addr) => write!(f, "    ld {}, ({})", reg, addr),
                    MemoryAddress::FrameRelative(offset) => {
                        if *offset >= 0 {
                            write!(f, "    ld {}, (ix+{})", reg, offset)
                        } else {
                            write!(f, "    ld {}, (ix{})", reg, offset)
                        }
                    }
                    MemoryAddress::RegisterIndirect(reg_ind) => {
                        write!(f, "    ld {}, ({})", reg, reg_ind)
                    }
                }
            }
            Z80Instruction::StoreMemory { addr, reg } => {
                match addr {
                    MemoryAddress::Direct(addr) => write!(f, "    ld ({}), {}", addr, reg),
                    MemoryAddress::FrameRelative(offset) => {
                        if *offset >= 0 {
                            write!(f, "    ld (ix+{}), {}", offset, reg)
                        } else {
                            write!(f, "    ld (ix{}), {}", offset, reg)
                        }
                    }
                    MemoryAddress::RegisterIndirect(reg_ind) => {
                        write!(f, "    ld ({}), {}", reg_ind, reg)
                    }
                }
            }
            Z80Instruction::Push { reg } => {
                write!(f, "    push {}", reg)
            }
            Z80Instruction::Pop { reg } => {
                write!(f, "    pop {}", reg)
            }
            Z80Instruction::Add { dst, src } => {
                write!(f, "    add {}, {}", dst, src)
            }
            Z80Instruction::Subtract { dst: _, src } => {
                write!(f, "    sub {}", src) // Z80 uses 'sub' for A, 'sbc hl' for HL
            }
            Z80Instruction::Compare { reg, value } => {
                if let Some(val) = value {
                    write!(f, "    cp {}", val)
                } else {
                    write!(f, "    cp {}", reg)
                }
            }
            Z80Instruction::Jump { label, near } => {
                if *near {
                    write!(f, "    jr {}", label)
                } else {
                    write!(f, "    jp {}", label)
                }
            }
            Z80Instruction::JumpConditional { condition, label, near } => {
                if *near {
                    write!(f, "    jr {}, {}", condition, label)
                } else {
                    write!(f, "    jp {}, {}", condition, label)
                }
            }
            Z80Instruction::Call { label } => {
                write!(f, "    call {}", label)
            }
            Z80Instruction::Return => {
                write!(f, "    ret")
            }
            Z80Instruction::Label { name } => {
                write!(f, "{}:", name)
            }
            Z80Instruction::Comment { text } => {
                write!(f, "    ; {}", text)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{Function, Program};

    #[test]
    fn test_codegen_empty_program() {
        let mut codegen = CodeGenerator::new();
        let program = Program {
            functions: vec![],
            globals: vec![],
        };
        let instructions = codegen.generate(&program);
        assert_eq!(instructions.len(), 0);
    }

    #[test]
    fn test_codegen_simple_function() {
        let mut codegen = CodeGenerator::new();
        let entry_label = "test_entry".to_string();
        let function = Function {
            name: "test".to_string(),
            params: vec![],
            return_type: None,
            blocks: vec![BasicBlock::new(entry_label.clone())],
            entry_block: entry_label,
        };
        let program = Program {
            functions: vec![function],
            globals: vec![],
        };
        let instructions = codegen.generate(&program);
        
        // Should have prologue, epilogue, and label
        assert!(instructions.len() > 0);
    }

    // ===== Jump Optimization Tests =====

    #[test]
    fn test_jump_optimization_convert_jp_to_jr() {
        let codegen = CodeGenerator::new();
        let mut instructions = vec![
            Z80Instruction::Label { name: "start".to_string() },
            Z80Instruction::LoadImmediate { reg: Z80Register::A, value: 42 },
            Z80Instruction::Jump { label: "end".to_string(), near: false }, // JP (3 bytes)
            Z80Instruction::LoadImmediate { reg: Z80Register::B, value: 0 },
            Z80Instruction::Label { name: "end".to_string() },
            Z80Instruction::Return,
        ];

        // Before optimization: jump should be JP (near: false)
        assert!(!matches!(
            instructions[2],
            Z80Instruction::Jump { near: true, .. }
        ));

        codegen.optimize_jumps(&mut instructions);

        // After optimization: jump should be JR (near: true) since displacement is small
        assert!(matches!(
            instructions[2],
            Z80Instruction::Jump { near: true, .. }
        ));
    }

    #[test]
    fn test_jump_optimization_keep_jp_for_large_displacement() {
        let codegen = CodeGenerator::new();
        let mut instructions = vec![
            Z80Instruction::Label { name: "start".to_string() },
        ];

        // Add many instructions to create large displacement (> 129 bytes)
        // Each LoadImmediate is 2 bytes, so we need > 65 instructions
        for i in 0..100 {
            instructions.push(Z80Instruction::LoadImmediate {
                reg: Z80Register::A,
                value: i as u16,
            });
        }

        instructions.push(Z80Instruction::Jump {
            label: "end".to_string(),
            near: false, // JP (3 bytes)
        });
        
        // Add more instructions to push the target label far away
        for i in 0..100 {
            instructions.push(Z80Instruction::LoadImmediate {
                reg: Z80Register::B,
                value: i as u16,
            });
        }
        
        instructions.push(Z80Instruction::Label { name: "end".to_string() });
        instructions.push(Z80Instruction::Return);

        let jump_idx = 101; // Jump is after 101 instructions (start label + 100 LoadImmediate)
        codegen.optimize_jumps(&mut instructions);

        // Jump should remain JP (near: false) since displacement is too large
        // Displacement: (100 * 2) bytes after jump = 200 bytes, which is > 129
        assert!(matches!(
            instructions[jump_idx],
            Z80Instruction::Jump { near: false, .. }
        ));
    }

    #[test]
    fn test_jump_optimization_conditional_jump() {
        let codegen = CodeGenerator::new();
        let mut instructions = vec![
            Z80Instruction::Label { name: "start".to_string() },
            Z80Instruction::Compare {
                reg: Z80Register::A,
                value: Some(0),
            },
            Z80Instruction::JumpConditional {
                condition: Condition::Zero,
                label: "end".to_string(),
                near: false, // JP cc (3 bytes)
            },
            Z80Instruction::LoadImmediate { reg: Z80Register::A, value: 1 },
            Z80Instruction::Label { name: "end".to_string() },
            Z80Instruction::Return,
        ];

        codegen.optimize_jumps(&mut instructions);

        // Conditional jump should be converted to JR cc
        assert!(matches!(
            instructions[2],
            Z80Instruction::JumpConditional { near: true, .. }
        ));
    }

    #[test]
    fn test_jump_optimization_iterative() {
        let codegen = CodeGenerator::new();
        // Create a scenario where multiple optimizations are needed
        let mut instructions = vec![
            Z80Instruction::Label { name: "start".to_string() },
            Z80Instruction::Jump {
                label: "mid".to_string(),
                near: false, // JP
            },
            // Add some instructions
            Z80Instruction::LoadImmediate { reg: Z80Register::A, value: 1 },
            Z80Instruction::LoadImmediate { reg: Z80Register::B, value: 2 },
            Z80Instruction::Label { name: "mid".to_string() },
            Z80Instruction::Jump {
                label: "end".to_string(),
                near: false, // JP
            },
            Z80Instruction::LoadImmediate { reg: Z80Register::C, value: 3 },
            Z80Instruction::Label { name: "end".to_string() },
            Z80Instruction::Return,
        ];

        codegen.optimize_jumps(&mut instructions);

        // Both jumps should be optimized to JR
        assert!(matches!(
            instructions[1],
            Z80Instruction::Jump { near: true, .. }
        ));
        assert!(matches!(
            instructions[5],
            Z80Instruction::Jump { near: true, .. }
        ));
    }

    #[test]
    fn test_instruction_size_calculation() {
        let codegen = CodeGenerator::new();

        assert_eq!(codegen.instruction_size(&Z80Instruction::Return), 1);
        assert_eq!(
            codegen.instruction_size(&Z80Instruction::LoadImmediate {
                reg: Z80Register::A,
                value: 42
            }),
            2
        );
        assert_eq!(
            codegen.instruction_size(&Z80Instruction::LoadImmediate {
                reg: Z80Register::HL,
                value: 0x1234
            }),
            3
        );
        assert_eq!(
            codegen.instruction_size(&Z80Instruction::Jump {
                label: "test".to_string(),
                near: false
            }),
            3
        );
        assert_eq!(
            codegen.instruction_size(&Z80Instruction::Jump {
                label: "test".to_string(),
                near: true
            }),
            2
        );
        assert_eq!(
            codegen.instruction_size(&Z80Instruction::Label {
                name: "test".to_string()
            }),
            0
        );
    }
}
