//! Z80 backend types - enums, structs, and display formatting

use std::fmt;

// ─── Core Types ───────────────────────────────────────────────────────────────

/// Z80 register names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Z80Register {
    /// 8-bit registers
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    /// 16-bit register pairs
    AF,
    BC,
    DE,
    HL,
    /// Index registers
    IX,
    IY,
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
    LoadMemory {
        reg: Z80Register,
        addr: MemoryAddress,
    },
    /// Store register to memory: `ld (addr), reg` or `ld (ix+offset), reg`
    StoreMemory {
        addr: MemoryAddress,
        reg: Z80Register,
    },
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
    JumpConditional {
        condition: Condition,
        label: String,
        near: bool,
    },
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

// ─── Display Formatting ───────────────────────────────────────────────────────

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
            Z80Instruction::LoadMemory { reg, addr } => match addr {
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
            },
            Z80Instruction::StoreMemory { addr, reg } => match addr {
                MemoryAddress::Direct(addr) => write!(f, "    ld ({}, {}", addr, reg),
                MemoryAddress::FrameRelative(offset) => {
                    if *offset >= 0 {
                        write!(f, "    ld (ix+{}), {}", offset, reg)
                    } else {
                        write!(f, "    ld (ix{}), {}", offset, reg)
                    }
                }
                MemoryAddress::RegisterIndirect(reg_ind) => {
                    write!(f, "    ld ({}, {}", reg_ind, reg)
                }
            },
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
            Z80Instruction::JumpConditional {
                condition,
                label,
                near,
            } => {
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
