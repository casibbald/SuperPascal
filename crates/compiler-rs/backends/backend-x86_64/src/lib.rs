//! x86_64 Backend for SuperPascal
//!
//! This crate implements code generation from SuperPascal IR to x86_64 assembly
//! and raw binary machine code.
//!
//! # Architecture
//!
//! - **Registers**: 16 general-purpose 64-bit registers (RAX, RCX, RDX, R8-R15)
//! - **Calling Convention**: System V AMD64 ABI
//! - **Stack**: Grows downward, 16-byte aligned at call sites
//! - **Return Values**: RAX (integer), XMM0 (float)
//! - **Frame Pointer**: RBP (callee-saved)

use ir::{BasicBlock, Function, Instruction, Program};
use ir::Value;
use std::fmt;

/// x86_64 register names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum X86Register {
    RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15,
}

impl fmt::Display for X86Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X86Register::RAX => write!(f, "rax"),
            X86Register::RCX => write!(f, "rcx"),
            X86Register::RDX => write!(f, "rdx"),
            X86Register::RBX => write!(f, "rbx"),
            X86Register::RSP => write!(f, "rsp"),
            X86Register::RBP => write!(f, "rbp"),
            X86Register::RSI => write!(f, "rsi"),
            X86Register::RDI => write!(f, "rdi"),
            X86Register::R8 => write!(f, "r8"),
            X86Register::R9 => write!(f, "r9"),
            X86Register::R10 => write!(f, "r10"),
            X86Register::R11 => write!(f, "r11"),
            X86Register::R12 => write!(f, "r12"),
            X86Register::R13 => write!(f, "r13"),
            X86Register::R14 => write!(f, "r14"),
            X86Register::R15 => write!(f, "r15"),
        }
    }
}

/// Memory addressing mode for memory operands
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryAddress {
    BaseIndex {
        base: Option<X86Register>,
        index: Option<X86Register>,
        scale: u8,
        disp: i64,
    },
}

/// x86_64 assembly instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X86Instruction {
    Label { name: String },
    Comment { text: String },
    Section { name: String },
    Export { name: String },
    External { name: String },
    GlobalVar { name: String, bytes: Vec<u8> },
    BSSVar { name: String, size: u64 },
    MovImm { dst: X86Register, imm: i64 },
    MovReg { dst: X86Register, src: X86Register },
    Lea { dst: X86Register, addr: MemoryAddress },
    Load { dst: X86Register, addr: MemoryAddress, size: u8 },
    Store { addr: MemoryAddress, src: X86Register, size: u8 },
    Add { dst: X86Register, src: X86Register },
    Sub { dst: X86Register, src: X86Register },
    IMul { dst: X86Register, src: X86Register },
    IDiv { src: X86Register },
    Inc { reg: X86Register },
    Dec { reg: X86Register },
    Neg { reg: X86Register },
    And { dst: X86Register, src: X86Register },
    Or { dst: X86Register, src: X86Register },
    Xor { dst: X86Register, src: X86Register },
    Cmp { reg1: X86Register, reg2: X86Register },
    CmpImm { reg: X86Register, imm: i32 },
    SetCC { dst: X86Register, condition: Condition },
    Jump { label: String },
    JumpCond { condition: Condition, label: String },
    Call { target: String },
    Ret,
    Push { reg: X86Register },
    Pop { reg: X86Register },
    Xchg { reg1: X86Register, reg2: X86Register },
}

/// Condition codes for conditional jumps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Above,
    AboveEqual,
    Below,
    BelowEqual,
    Carry,
    NoCarry,
    Sign,
    NoSign,
    Overflow,
    NoOverflow,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Equal => write!(f, "e"),
            Condition::NotEqual => write!(f, "ne"),
            Condition::Greater => write!(f, "g"),
            Condition::GreaterEqual => write!(f, "ge"),
            Condition::Less => write!(f, "l"),
            Condition::LessEqual => write!(f, "le"),
            Condition::Above => write!(f, "a"),
            Condition::AboveEqual => write!(f, "ae"),
            Condition::Below => write!(f, "b"),
            Condition::BelowEqual => write!(f, "be"),
            Condition::Carry => write!(f, "c"),
            Condition::NoCarry => write!(f, "nc"),
            Condition::Sign => write!(f, "s"),
            Condition::NoSign => write!(f, "ns"),
            Condition::Overflow => write!(f, "o"),
            Condition::NoOverflow => write!(f, "no"),
        }
    }
}

/// x86_64 code generator
pub struct CodeGenerator {
    current_function: Option<String>,
    temp_counter: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            current_function: None,
        }
    }

    /// Generate x86_64 instructions from IR program
    pub fn generate(&mut self, program: &Program) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();
        instructions.push(X86Instruction::Section {
            name: ".text".to_string(),
        });
        for function in &program.functions {
            instructions.extend(self.generate_function(function));
        }
        instructions
    }

    fn generate_function(&mut self, function: &Function) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();
        self.current_function = Some(function.name.clone());

        instructions.push(X86Instruction::Label {
            name: format!("_{}", function.name),
        });

        // Prologue
        instructions.push(X86Instruction::Push { reg: X86Register::RBP });
        instructions.push(X86Instruction::MovReg {
            dst: X86Register::RBP,
            src: X86Register::RSP,
        });

        for block in &function.blocks {
            instructions.extend(self.generate_block(block));
        }

        // Epilogue
        instructions.push(X86Instruction::MovReg {
            dst: X86Register::RSP,
            src: X86Register::RBP,
        });
        instructions.push(X86Instruction::Pop { reg: X86Register::RBP });
        instructions.push(X86Instruction::Ret);

        self.current_function = None;
        instructions
    }

    fn generate_block(&mut self, block: &BasicBlock) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();
        instructions.push(X86Instruction::Label {
            name: block.label.clone(),
        });
        for inst in &block.instructions {
            instructions.extend(self.generate_instruction(inst));
        }
        instructions
    }

    fn generate_instruction(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        match &inst.opcode {
            ir::Opcode::Mov => self.generate_mov(inst),
            ir::Opcode::Add => self.generate_binop(inst, BinOp::Add),
            ir::Opcode::Sub => self.generate_binop(inst, BinOp::Sub),
            ir::Opcode::Mul => self.generate_binop(inst, BinOp::Mul),
            ir::Opcode::Div => self.generate_binop(inst, BinOp::Div),
            ir::Opcode::Cmp => self.generate_cmp(inst),
            ir::Opcode::Jump => self.generate_jump(inst),
            ir::Opcode::CJump => self.generate_cjump(inst),
            ir::Opcode::Call => self.generate_call(inst),
            ir::Opcode::Ret => self.generate_ret(inst),
            ir::Opcode::Load => self.generate_load(inst),
            ir::Opcode::Store => self.generate_store(inst),
            ir::Opcode::Push => self.generate_push(inst),
            ir::Opcode::Pop => self.generate_pop(inst),
            _ => vec![X86Instruction::Comment {
                text: format!("TODO: {:?}", inst.opcode),
            }],
        }
    }

    fn generate_mov(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.len() < 2 { return vec![]; }
        let dst = &inst.operands[0];
        let src = &inst.operands[1];
        match (dst, src) {
            (Value::Register(_), Value::Immediate(imm)) => {
                vec![X86Instruction::MovImm {
                    dst: X86Register::RAX,
                    imm: *imm as i64,
                }]
            }
            (Value::Register(_), Value::Register(_)) => {
                vec![X86Instruction::MovReg {
                    dst: X86Register::RAX,
                    src: X86Register::RCX,
                }]
            }
            (Value::Register(_), Value::Memory { base, offset }) => {
                let base_reg = self.parse_register(&Value::Register(base.clone()));
                vec![
                    X86Instruction::Load {
                        dst: X86Register::RAX,
                        addr: MemoryAddress::BaseIndex {
                            base: Some(base_reg),
                            index: None,
                            scale: 1,
                            disp: *offset as i64,
                        },
                        size: 8,
                    },
                ]
            }
            _ => vec![X86Instruction::Comment {
                text: format!("TODO: MOV {:?} <- {:?}", dst, src),
            }],
        }
    }

    fn generate_cmp(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.len() < 2 { return vec![]; }
        let src1 = &inst.operands[0];
        let src2 = &inst.operands[1];
        let mut instructions = Vec::new();
        match src1 {
            Value::Immediate(imm) => {
                instructions.push(X86Instruction::MovImm {
                    dst: X86Register::RAX, imm: *imm as i64,
                });
                if let Value::Immediate(imm2) = src2 {
                    instructions.push(X86Instruction::CmpImm {
                        reg: X86Register::RAX, imm: *imm2,
                    });
                } else {
                    instructions.push(X86Instruction::Comment {
                        text: "TODO: CMP imm,mem".to_string(),
                    });
                }
            }
            Value::Register(reg) => {
                instructions.push(X86Instruction::MovReg {
                    dst: X86Register::RAX,
                    src: self.parse_register(src1),
                });
                match src2 {
                    Value::Immediate(imm) => {
                        instructions.push(X86Instruction::MovImm {
                            dst: X86Register::RCX, imm: *imm as i64,
                        });
                        instructions.push(X86Instruction::Cmp {
                            reg1: X86Register::RAX, reg2: X86Register::RCX,
                        });
                    }
                    Value::Register(_) => {
                        instructions.push(X86Instruction::Cmp {
                            reg1: X86Register::RAX,
                            reg2: self.parse_register(src2),
                        });
                    }
                    _ => {
                        instructions.push(X86Instruction::Comment {
                            text: format!("TODO: CMP reg, {:?}", src2),
                        });
                    }
                }
            }
            _ => {
                instructions.push(X86Instruction::Comment {
                    text: format!("TODO: CMP {:?}", src1),
                });
            }
        }
        instructions
    }

    fn generate_not(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.len() < 2 { return vec![]; }
        let dst = &inst.operands[0];
        let src = &inst.operands[1];
        let mut instructions = Vec::new();
        match src {
            Value::Immediate(imm) => {
                instructions.push(X86Instruction::MovImm {
                    dst: X86Register::RAX, imm: *imm as i64,
                });
                instructions.push(X86Instruction::Neg { reg: X86Register::RAX });
            }
            Value::Register(_) => {
                instructions.push(X86Instruction::MovReg {
                    dst: X86Register::RAX,
                    src: self.parse_register(src),
                });
                instructions.push(X86Instruction::Neg { reg: X86Register::RAX });
            }
            _ => {
                instructions.push(X86Instruction::Comment {
                    text: format!("TODO: NOT {:?}", src),
                });
                return instructions;
            }
        }
        instructions.extend(self.store_rax_to_value(dst));
        instructions
    }

    fn generate_jump(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.is_empty() { return vec![]; }
        if let Value::Label(label) = &inst.operands[0] {
            vec![X86Instruction::Jump { label: label.clone() }]
        } else {
            vec![X86Instruction::Comment {
                text: format!("TODO: JUMP {:?}", inst.operands),
            }]
        }
    }

    fn generate_cjump(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.len() < 3 { return vec![]; }
        let label_true = match &inst.operands[1] {
            Value::Label(l) => l.clone(),
            _ => return vec![],
        };
        let label_false = match &inst.operands[2] {
            Value::Label(l) => l.clone(),
            _ => return vec![],
        };
        vec![
            X86Instruction::JumpCond {
                condition: Condition::Equal,
                label: label_true,
            },
            X86Instruction::Jump { label: label_false },
        ]
    }

    fn generate_call(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.is_empty() { return vec![]; }
        if let Value::Label(label) = &inst.operands[0] {
            vec![X86Instruction::Call { target: label.clone() }]
        } else {
            vec![X86Instruction::Comment {
                text: format!("TODO: CALL {:?}", inst.operands),
            }]
        }
    }

    fn generate_ret(&mut self, _inst: &Instruction) -> Vec<X86Instruction> {
        vec![X86Instruction::Ret]
    }

    fn generate_load(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.len() < 2 { return vec![]; }
        let dst = &inst.operands[0];
        let src = &inst.operands[1];
        let mut instructions = Vec::new();
        match dst {
            Value::Register(dst_reg) => {
                let dst_reg_name = dst_reg.clone();
                match src {
                    Value::Memory { base, offset } => {
                        let base_reg = self.parse_register(&Value::Register(base.clone()));
                        instructions.push(X86Instruction::Load {
                            dst: self.parse_register(&Value::Register(dst_reg_name)),
                            addr: MemoryAddress::BaseIndex {
                                base: Some(base_reg),
                                index: None,
                                scale: 1,
                                disp: *offset as i64,
                            },
                            size: 8,
                        });
                    }
                    Value::Immediate(imm) => {
                        instructions.push(X86Instruction::MovImm {
                            dst: self.parse_register(&Value::Register(dst_reg_name)),
                            imm: *imm as i64,
                        });
                    }
                    Value::Register(_) => {
                        instructions.push(X86Instruction::MovReg {
                            dst: self.parse_register(&Value::Register(dst_reg_name)),
                            src: self.parse_register(src),
                        });
                    }
                    _ => {
                        instructions.push(X86Instruction::Comment {
                            text: format!("TODO: LOAD {:?}", src),
                        });
                    }
                }
            }
            _ => {
                instructions.push(X86Instruction::Comment {
                    text: format!("TODO: LOAD dst {:?}", dst),
                });
            }
        }
        instructions
    }

    fn generate_store(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.len() < 2 { return vec![]; }
        let dst = &inst.operands[0];
        let src = &inst.operands[1];
        let mut instructions = Vec::new();
        match dst {
            Value::Memory { base, offset } => {
                let base_reg = self.parse_register(&Value::Register(base.clone()));
                match src {
                    Value::Immediate(imm) => {
                        instructions.push(X86Instruction::MovImm {
                            dst: X86Register::RAX, imm: *imm as i64,
                        });
                        instructions.push(X86Instruction::Store {
                            addr: MemoryAddress::BaseIndex {
                                base: Some(base_reg),
                                index: None,
                                scale: 1,
                                disp: *offset as i64,
                            },
                            src: X86Register::RAX,
                            size: 8,
                        });
                    }
                    Value::Register(_) => {
                        instructions.push(X86Instruction::Store {
                            addr: MemoryAddress::BaseIndex {
                                base: Some(base_reg),
                                index: None,
                                scale: 1,
                                disp: *offset as i64,
                            },
                            src: self.parse_register(src),
                            size: 8,
                        });
                    }
                    _ => {
                        instructions.push(X86Instruction::Comment {
                            text: format!("TODO: STORE {:?}", src),
                        });
                    }
                }
            }
            _ => {
                instructions.push(X86Instruction::Comment {
                    text: format!("TODO: STORE dst {:?}", dst),
                });
            }
        }
        instructions
    }

    fn generate_push(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.is_empty() { return vec![]; }
        match &inst.operands[0] {
            Value::Register(_) => {
                vec![X86Instruction::Push { reg: self.parse_register(&inst.operands[0]) }]
            }
            Value::Immediate(imm) => {
                vec![
                    X86Instruction::MovImm { dst: X86Register::RAX, imm: *imm as i64 },
                    X86Instruction::Push { reg: X86Register::RAX },
                ]
            }
            _ => vec![X86Instruction::Comment {
                text: format!("TODO: PUSH {:?}", inst.operands),
            }],
        }
    }

    fn generate_pop(&mut self, inst: &Instruction) -> Vec<X86Instruction> {
        if inst.operands.is_empty() { return vec![]; }
        match &inst.operands[0] {
            Value::Register(_) => {
                vec![X86Instruction::Pop { reg: self.parse_register(&inst.operands[0]) }]
            }
            _ => vec![X86Instruction::Comment {
                text: format!("TODO: POP {:?}", inst.operands),
            }],
        }
    }

    /// Generate a binary operation (add, sub, mul, div, and, or, xor)
    fn generate_binop(&mut self, inst: &Instruction, op: BinOp) -> Vec<X86Instruction> {
        if inst.operands.len() < 3 { return vec![]; }
        let dst = &inst.operands[0];
        let src1 = &inst.operands[1];
        let src2 = &inst.operands[2];
        let mut instructions = Vec::new();

        // Load src1 into RAX
        match src1 {
            Value::Immediate(imm) => {
                instructions.push(X86Instruction::MovImm {
                    dst: X86Register::RAX, imm: *imm as i64,
                });
            }
            Value::Register(_) => {
                instructions.push(X86Instruction::MovReg {
                    dst: X86Register::RAX,
                    src: self.parse_register(src1),
                });
            }
            _ => {
                instructions.push(X86Instruction::Comment {
                    text: format!("TODO: binop src1 {:?}", src1),
                });
                return instructions;
            }
        }

        // Apply op with src2
        match src2 {
            Value::Immediate(imm) => {
                instructions.push(X86Instruction::MovImm {
                    dst: X86Register::RCX, imm: *imm as i64,
                });
                instructions.extend(op.encode_r1_r2(X86Register::RAX, X86Register::RCX));
            }
            Value::Register(_) => {
                instructions.extend(op.encode_r1_r2(X86Register::RAX, self.parse_register(src2)));
            }
            _ => {
                instructions.push(X86Instruction::Comment {
                    text: format!("TODO: binop src2 {:?}", src2),
                });
            }
        }

        instructions.extend(self.store_rax_to_value(dst));
        instructions
    }

    fn store_rax_to_value(&mut self, value: &Value) -> Vec<X86Instruction> {
        match value {
            Value::Register(reg) => {
                vec![X86Instruction::MovReg {
                    dst: self.parse_register(value),
                    src: X86Register::RAX,
                }]
            }
            Value::Memory { base, offset } => {
                let base_reg = self.parse_register(&Value::Register(base.clone()));
                vec![X86Instruction::Store {
                    addr: MemoryAddress::BaseIndex {
                        base: Some(base_reg),
                        index: None,
                        scale: 1,
                        disp: *offset as i64,
                    },
                    src: X86Register::RAX,
                    size: 8,
                }]
            }
            _ => vec![X86Instruction::Comment {
                text: format!("TODO: store_rax_to_value {:?}", value),
            }],
        }
    }

    fn next_temp_register(&mut self) -> X86Register {
        let idx = self.temp_counter % 8;
        self.temp_counter += 1;
        match idx {
            0 => X86Register::RAX,
            1 => X86Register::RCX,
            2 => X86Register::RDX,
            3 => X86Register::R8,
            4 => X86Register::R9,
            5 => X86Register::R10,
            6 => X86Register::R11,
            7 => X86Register::R12,
            _ => X86Register::RAX,
        }
    }

    fn parse_register(&self, reg: &Value) -> X86Register {
        match reg {
            Value::Register(r) => {
                let r_lower = r.to_lowercase();
                match r_lower.as_str() {
                    "rax" | "eax" => X86Register::RAX,
                    "rcx" | "ecx" => X86Register::RCX,
                    "rdx" | "edx" => X86Register::RDX,
                    "rbx" | "ebx" => X86Register::RBX,
                    "rsp" => X86Register::RSP,
                    "rbp" => X86Register::RBP,
                    "rsi" => X86Register::RSI,
                    "rdi" => X86Register::RDI,
                    "r8" => X86Register::R8,
                    "r9" => X86Register::R9,
                    "r10" => X86Register::R10,
                    "r11" => X86Register::R11,
                    "r12" => X86Register::R12,
                    "r13" => X86Register::R13,
                    "r14" => X86Register::R14,
                    "r15" => X86Register::R15,
                    _ => X86Register::RAX,
                }
            }
            _ => X86Register::RAX,
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Binary operation encoding helper
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
}

impl BinOp {
    fn encode_r1_r2(self, r1: X86Register, r2: X86Register) -> Vec<X86Instruction> {
        match self {
            BinOp::Add => vec![X86Instruction::Add { dst: r1, src: r2 }],
            BinOp::Sub => vec![X86Instruction::Sub { dst: r1, src: r2 }],
            BinOp::Mul => vec![X86Instruction::IMul { dst: r1, src: r2 }],
            BinOp::Div => vec![X86Instruction::IDiv { src: r2 }],
            BinOp::And => vec![X86Instruction::And { dst: r1, src: r2 }],
            BinOp::Or => vec![X86Instruction::Or { dst: r1, src: r2 }],
            BinOp::Xor => vec![X86Instruction::Xor { dst: r1, src: r2 }],
        }
    }
}

/// Encode x86_64 instructions to raw binary machine code
pub fn encode_instructions(instructions: &[X86Instruction]) -> Vec<u8> {
    let mut bytes = Vec::new();

    for inst in instructions {
        match inst {
            X86Instruction::Label { .. }
            | X86Instruction::Comment { .. }
            | X86Instruction::Section { .. }
            | X86Instruction::Export { .. }
            | X86Instruction::External { .. }
            | X86Instruction::GlobalVar { .. }
            | X86Instruction::BSSVar { .. } => {}
            X86Instruction::MovImm { dst, imm } => {
                let reg = register_code(*dst);
                bytes.push(0xB8 + reg);
                bytes.extend_from_slice(&(*imm as i32).to_le_bytes());
            }
            X86Instruction::MovReg { dst, src } => {
                let d = register_code(*dst);
                let s = register_code(*src);
                bytes.push(0x89);
                bytes.push(0xC0 + d | (s << 3));
            }
            X86Instruction::Load { dst, addr, size: _ } => {
                let d = register_code(*dst);
                if let MemoryAddress::BaseIndex { base, index, scale: _, disp } = addr {
                    let b = match base {
                        Some(r) => register_code(*r),
                        None => 5,
                    };
                    if index.is_none() {
                        let modrm = 0x40 | (d << 3) | b;
                        bytes.push(0x8B);
                        bytes.push(modrm);
                        bytes.push(*disp as u8);
                    }
                }
            }
            X86Instruction::Store { addr, src, size: _ } => {
                let s = register_code(*src);
                if let MemoryAddress::BaseIndex { base, index, scale: _, disp } = addr {
                    let b = match base {
                        Some(r) => register_code(*r),
                        None => 5,
                    };
                    if index.is_none() {
                        let modrm = 0x40 | (s << 3) | b;
                        bytes.push(0x89);
                        bytes.push(modrm);
                        bytes.push(*disp as u8);
                    }
                }
            }
            X86Instruction::Add { dst, src } => {
                let d = register_code(*dst);
                let s = register_code(*src);
                bytes.push(0x01);
                bytes.push(0xC0 + d | (s << 3));
            }
            X86Instruction::Sub { dst, src } => {
                let d = register_code(*dst);
                let s = register_code(*src);
                bytes.push(0x29);
                bytes.push(0xC0 + d | (s << 3));
            }
            X86Instruction::IMul { dst, src } => {
                let d = register_code(*dst);
                let s = register_code(*src);
                bytes.push(0x0F);
                bytes.push(0xAF);
                bytes.push(0xC0 + d | (s << 3));
            }
            X86Instruction::IDiv { src } => {
                let s = register_code(*src);
                bytes.push(0xF7);
                bytes.push(0xD0 | s);
            }
            X86Instruction::Inc { reg } => {
                bytes.push(0x40 + register_code(*reg));
            }
            X86Instruction::Dec { reg } => {
                bytes.push(0x48 + register_code(*reg));
            }
            X86Instruction::Neg { reg } => {
                let r = register_code(*reg);
                bytes.push(0xF7);
                bytes.push(0xD0 | r);
            }
            X86Instruction::And { dst, src } => {
                let d = register_code(*dst);
                let s = register_code(*src);
                bytes.push(0x21);
                bytes.push(0xC0 + d | (s << 3));
            }
            X86Instruction::Or { dst, src } => {
                let d = register_code(*dst);
                let s = register_code(*src);
                bytes.push(0x09);
                bytes.push(0xC0 + d | (s << 3));
            }
            X86Instruction::Xor { dst, src } => {
                let d = register_code(*dst);
                let s = register_code(*src);
                bytes.push(0x31);
                bytes.push(0xC0 + d | (s << 3));
            }
            X86Instruction::Cmp { reg1, reg2 } => {
                let r1 = register_code(*reg1);
                let r2 = register_code(*reg2);
                bytes.push(0x39);
                bytes.push(0xC0 + r1 | (r2 << 3));
            }
            X86Instruction::CmpImm { reg, imm } => {
                let r = register_code(*reg);
                bytes.push(0x83);
                bytes.push(0xF8 | r);
                bytes.extend_from_slice(&imm.to_le_bytes());
            }
            X86Instruction::Jump { label: _ } => {
                bytes.push(0xE9);
                bytes.extend_from_slice(&0u32.to_le_bytes());
            }
            X86Instruction::JumpCond { condition: _, label: _ } => {
                bytes.push(0x74);
                bytes.push(0);
            }
            X86Instruction::Call { target: _ } => {
                bytes.push(0xE8);
                bytes.extend_from_slice(&0u32.to_le_bytes());
            }
            X86Instruction::Ret => {
                bytes.push(0xC3);
            }
            X86Instruction::Push { reg } => {
                bytes.push(0x50 + register_code(*reg));
            }
            X86Instruction::Pop { reg } => {
                bytes.push(0x58 + register_code(*reg));
            }
            X86Instruction::Xchg { reg1, reg2 } => {
                let r1 = register_code(*reg1);
                let r2 = register_code(*reg2);
                bytes.push(0x87);
                bytes.push(0xC0 + r1 | (r2 << 3));
            }
            _ => {}
        }
    }

    bytes
}

fn register_code(reg: X86Register) -> u8 {
    match reg {
        X86Register::RAX => 0,
        X86Register::RCX => 1,
        X86Register::RDX => 2,
        X86Register::RBX => 3,
        X86Register::RSP => 4,
        X86Register::RBP => 5,
        X86Register::RSI => 6,
        X86Register::RDI => 7,
        X86Register::R8 => 8,
        X86Register::R9 => 9,
        X86Register::R10 => 10,
        X86Register::R11 => 11,
        X86Register::R12 => 12,
        X86Register::R13 => 13,
        X86Register::R14 => 14,
        X86Register::R15 => 15,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let mut codegen = CodeGenerator::new();
        assert_eq!(codegen.parse_register(&Value::Register("rax".to_string())), X86Register::RAX);
        assert_eq!(codegen.parse_register(&Value::Register("rcx".to_string())), X86Register::RCX);
        assert_eq!(codegen.parse_register(&Value::Register("RDX".to_string())), X86Register::RDX);
        assert_eq!(codegen.parse_register(&Value::Register("R12".to_string())), X86Register::R12);
        assert_eq!(codegen.parse_register(&Value::Register("unknown".to_string())), X86Register::RAX);
    }

    #[test]
    fn test_generate_simple_function() {
        let mut codegen = CodeGenerator::new();
        let program = Program {
            functions: vec![
                Function {
                    name: "main".to_string(),
                    blocks: vec![
                        BasicBlock {
                            label: "entry".to_string(),
                            instructions: vec![
                                Instruction {
                                    opcode: ir::Opcode::Mov,
                                    operands: vec![
                                        Value::Register("rax".to_string()),
                                        Value::Immediate(42),
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        };
        let instructions = codegen.generate(&program);
        assert!(!instructions.is_empty());
    }

    #[test]
    fn test_generate_with_jump() {
        let mut codegen = CodeGenerator::new();
        let program = Program {
            functions: vec![
                Function {
                    name: "main".to_string(),
                    blocks: vec![
                        BasicBlock {
                            label: "entry".to_string(),
                            instructions: vec![],
                        },
                        BasicBlock {
                            label: "exit".to_string(),
                            instructions: vec![
                                Instruction {
                                    opcode: ir::Opcode::Ret,
                                    operands: vec![],
                                },
                            ],
                        },
                    ],
                },
            ],
        };
        let instructions = codegen.generate(&program);
        assert!(instructions.iter().any(|i| matches!(i, X86Instruction::Ret)));
    }

    #[test]
    fn test_encode_mov_rax_imm() {
        let instructions = vec![
            X86Instruction::MovImm {
                dst: X86Register::RAX,
                imm: 42,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0xB8, 0x2A, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_encode_mov_rax_rcx() {
        let instructions = vec![
            X86Instruction::MovReg {
                dst: X86Register::RAX,
                src: X86Register::RCX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x89, 0xC1]);
    }

    #[test]
    fn test_encode_ret() {
        let instructions = vec![X86Instruction::Ret];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0xC3]);
    }

    #[test]
    fn test_encode_prologue_epilogue() {
        let instructions = vec![
            X86Instruction::Push { reg: X86Register::RBP },
            X86Instruction::MovReg {
                dst: X86Register::RBP,
                src: X86Register::RSP,
            },
            X86Instruction::Pop { reg: X86Register::RBP },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x55, 0x89, 0xE5, 0x5D]);
    }

    #[test]
    fn test_encode_add() {
        let instructions = vec![
            X86Instruction::Add {
                dst: X86Register::RAX,
                src: X86Register::RCX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x01, 0xC1]);
    }

    #[test]
    fn test_encode_sub() {
        let instructions = vec![
            X86Instruction::Sub {
                dst: X86Register::RAX,
                src: X86Register::RCX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x29, 0xC1]);
    }

    #[test]
    fn test_encode_xor_zero() {
        let instructions = vec![
            X86Instruction::Xor {
                dst: X86Register::RAX,
                src: X86Register::RAX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x31, 0xC0]);
    }

    #[test]
    fn test_encode_and() {
        let instructions = vec![
            X86Instruction::And {
                dst: X86Register::RAX,
                src: X86Register::RCX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x21, 0xC1]);
    }

    #[test]
    fn test_encode_or() {
        let instructions = vec![
            X86Instruction::Or {
                dst: X86Register::RAX,
                src: X86Register::RCX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x09, 0xC1]);
    }

    #[test]
    fn test_encode_imul() {
        let instructions = vec![
            X86Instruction::IMul {
                dst: X86Register::RAX,
                src: X86Register::RCX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x0F, 0xAF, 0xC1]);
    }

    #[test]
    fn test_encode_cmp_reg_imm() {
        let instructions = vec![
            X86Instruction::CmpImm {
                reg: X86Register::RAX,
                imm: 42,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x83, 0xF8, 0x2A, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_encode_cmp_two_regs() {
        let instructions = vec![
            X86Instruction::Cmp {
                reg1: X86Register::RAX,
                reg2: X86Register::RCX,
            },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x39, 0xC1]);
    }

    #[test]
    fn test_encode_push_pop() {
        let instructions = vec![
            X86Instruction::Push { reg: X86Register::RAX },
            X86Instruction::Pop { reg: X86Register::RAX },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x50, 0x58]);
    }

    #[test]
    fn test_encode_inc_dec() {
        let instructions = vec![
            X86Instruction::Inc { reg: X86Register::RAX },
            X86Instruction::Dec { reg: X86Register::RAX },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x48, 0x48]);
    }

    #[test]
    fn test_encode_neg() {
        let instructions = vec![
            X86Instruction::Neg { reg: X86Register::RAX },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0xF7, 0xD0]);
    }

    #[test]
    fn test_encode_idiv() {
        let instructions = vec![
            X86Instruction::IDiv { src: X86Register::RCX },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0xF7, 0xD8]);
    }

    #[test]
    fn test_encode_xchg() {
        let instructions = vec![
            X86Instruction::Xchg { reg1: X86Register::RAX, reg2: X86Register::RCX },
        ];
        let bytes = encode_instructions(&instructions);
        assert_eq!(bytes, vec![0x87, 0xD0]);
    }
}
