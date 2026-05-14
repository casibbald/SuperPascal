//! Z80 instruction encoding - machine code byte generation
//!
//! Implements the `encode()` method for `Z80Instruction` and all helper encoders.

use crate::types::{Condition, MemoryAddress, Z80Instruction, Z80Register};

/// Encode Z80 instructions into machine code bytes
pub fn encode_instructions(instructions: &[Z80Instruction]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for inst in instructions {
        bytes.extend_from_slice(&inst.encode());
    }
    bytes
}

impl Z80Instruction {
    /// Encode a single Z80 instruction into its machine code bytes.
    /// Labels and comments produce no bytes.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Z80Instruction::LoadImmediate { reg, value } => self.encode_ldi(reg, *value),
            Z80Instruction::LoadRegister { dst, src } => self.encode_ld_reg(dst, src),
            Z80Instruction::LoadMemory { reg, addr } => self.encode_ld_mr(reg, addr),
            Z80Instruction::StoreMemory { addr, reg } => self.encode_ld_rm(addr, reg),
            Z80Instruction::Push { reg } => self.encode_push(reg),
            Z80Instruction::Pop { reg } => self.encode_pop(reg),
            Z80Instruction::Add { dst, src } => self.encode_add(dst, src),
            Z80Instruction::Subtract { dst, src } => self.encode_sub(dst, src),
            Z80Instruction::Compare { reg, value } => self.encode_cp(reg, value),
            Z80Instruction::Jump { label: _, near } => {
                if *near {
                    vec![0x18, 0x00]
                } else {
                    vec![0xC3, 0x00, 0x00]
                }
            }
            Z80Instruction::JumpConditional {
                condition,
                label: _,
                near,
            } => {
                let cc = match condition {
                    Condition::Zero => 0x28,
                    Condition::NonZero => 0x20,
                    Condition::Carry => 0x38,
                    Condition::NoCarry => 0x30,
                    Condition::Sign => 0x30,
                    Condition::Positive => 0x28,
                };
                if *near {
                    vec![cc, 0x00]
                } else {
                    let jp_cc = match condition {
                        Condition::Zero => 0xCA,
                        Condition::NonZero => 0xC2,
                        Condition::Carry => 0xDA,
                        Condition::NoCarry => 0xC2,
                        Condition::Sign => 0xCA,
                        Condition::Positive => 0xCA,
                    };
                    vec![jp_cc, 0x00, 0x00]
                }
            }
            Z80Instruction::Call { label: _ } => vec![0xCD, 0x00, 0x00],
            Z80Instruction::Return => vec![0xC9],
            Z80Instruction::Label { .. } => vec![],
            Z80Instruction::Comment { .. } => vec![],
        }
    }

    // ── LD Immediate ──
    fn encode_ldi(&self, reg: &Z80Register, value: u16) -> Vec<u8> {
        let imm8 = (value & 0xFF) as u8;
        match reg {
            Z80Register::A => vec![0x3E, imm8],
            Z80Register::B => vec![0x06, imm8],
            Z80Register::C => vec![0x0E, imm8],
            Z80Register::D => vec![0x16, imm8],
            Z80Register::E => vec![0x1E, imm8],
            Z80Register::H => vec![0x26, imm8],
            Z80Register::L => vec![0x2E, imm8],
            Z80Register::BC => vec![0x01, (value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8],
            Z80Register::DE => vec![0x11, (value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8],
            Z80Register::HL => vec![0x21, (value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8],
            Z80Register::IX => vec![0xDD, 0x21, (value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8],
            Z80Register::IY => vec![0xFD, 0x21, (value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8],
            Z80Register::SP => vec![0x31, (value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8],
            Z80Register::AF => vec![0x00], // No direct LD AF, placeholder
        }
    }

    // ── LD reg, reg ──
    fn encode_ld_reg(&self, dst: &Z80Register, src: &Z80Register) -> Vec<u8> {
        match (dst, src) {
            // To A
            (Z80Register::A, Z80Register::A) => vec![0x7F],
            (Z80Register::B, Z80Register::A) => vec![0x78],
            (Z80Register::C, Z80Register::A) => vec![0x79],
            (Z80Register::D, Z80Register::A) => vec![0x7A],
            (Z80Register::E, Z80Register::A) => vec![0x7B],
            (Z80Register::H, Z80Register::A) => vec![0x7C],
            (Z80Register::L, Z80Register::A) => vec![0x7D],
            // From A
            (Z80Register::A, Z80Register::B) => vec![0x78],
            (Z80Register::A, Z80Register::C) => vec![0x79],
            (Z80Register::A, Z80Register::D) => vec![0x7A],
            (Z80Register::A, Z80Register::E) => vec![0x7B],
            (Z80Register::A, Z80Register::H) => vec![0x7C],
            (Z80Register::A, Z80Register::L) => vec![0x7D],
            // B
            (Z80Register::B, Z80Register::B) => vec![0x40],
            (Z80Register::B, Z80Register::C) => vec![0x41],
            (Z80Register::B, Z80Register::D) => vec![0x42],
            (Z80Register::B, Z80Register::E) => vec![0x43],
            (Z80Register::B, Z80Register::H) => vec![0x44],
            (Z80Register::B, Z80Register::L) => vec![0x46],
            // C
            (Z80Register::C, Z80Register::B) => vec![0x48],
            (Z80Register::C, Z80Register::C) => vec![0x49],
            (Z80Register::C, Z80Register::D) => vec![0x4A],
            (Z80Register::C, Z80Register::E) => vec![0x4B],
            (Z80Register::C, Z80Register::H) => vec![0x4C],
            (Z80Register::C, Z80Register::L) => vec![0x4E],
            // D
            (Z80Register::D, Z80Register::B) => vec![0x50],
            (Z80Register::D, Z80Register::C) => vec![0x51],
            (Z80Register::D, Z80Register::D) => vec![0x52],
            (Z80Register::D, Z80Register::E) => vec![0x53],
            (Z80Register::D, Z80Register::H) => vec![0x54],
            (Z80Register::D, Z80Register::L) => vec![0x56],
            // E
            (Z80Register::E, Z80Register::B) => vec![0x58],
            (Z80Register::E, Z80Register::C) => vec![0x59],
            (Z80Register::E, Z80Register::D) => vec![0x5A],
            (Z80Register::E, Z80Register::E) => vec![0x5B],
            (Z80Register::E, Z80Register::H) => vec![0x5C],
            (Z80Register::E, Z80Register::L) => vec![0x5E],
            // H
            (Z80Register::H, Z80Register::B) => vec![0x60],
            (Z80Register::H, Z80Register::C) => vec![0x61],
            (Z80Register::H, Z80Register::D) => vec![0x62],
            (Z80Register::H, Z80Register::E) => vec![0x63],
            (Z80Register::H, Z80Register::H) => vec![0x64],
            (Z80Register::H, Z80Register::L) => vec![0x66],
            // L
            (Z80Register::L, Z80Register::B) => vec![0x68],
            (Z80Register::L, Z80Register::C) => vec![0x69],
            (Z80Register::L, Z80Register::D) => vec![0x6A],
            (Z80Register::L, Z80Register::E) => vec![0x6B],
            (Z80Register::L, Z80Register::H) => vec![0x6C],
            (Z80Register::L, Z80Register::L) => vec![0x6E],
            // 16-bit and special
            (Z80Register::HL, Z80Register::BC) => vec![0x09],
            (Z80Register::HL, Z80Register::DE) => vec![0x19],
            (Z80Register::HL, Z80Register::HL) => vec![0x09],
            (Z80Register::BC, Z80Register::HL) => vec![0x7B],
            (Z80Register::DE, Z80Register::HL) => vec![0x7B],
            (Z80Register::HL, Z80Register::SP) => vec![0xF9],
            (Z80Register::SP, Z80Register::HL) => vec![0xF9],
            (Z80Register::IX, Z80Register::HL) => vec![0xDD, 0x22],
            (Z80Register::IY, Z80Register::HL) => vec![0xFD, 0x22],
            _ => vec![0x7A], // fallback LD A,D
        }
    }

    // ── LD reg, (addr) ──
    fn encode_ld_mr(&self, reg: &Z80Register, addr: &MemoryAddress) -> Vec<u8> {
        match addr {
            MemoryAddress::RegisterIndirect(Z80Register::HL) => match reg {
                Z80Register::A => vec![0x46],
                Z80Register::B => vec![0x46], // placeholder
                Z80Register::C => vec![0x4E],
                Z80Register::D => vec![0x56],
                Z80Register::E => vec![0x5E],
                Z80Register::H => vec![0x66],
                Z80Register::L => vec![0x6E],
                _ => vec![0x7A],
            },
            MemoryAddress::RegisterIndirect(Z80Register::BC) => match reg {
                Z80Register::A => vec![0x0A],
                _ => vec![0x7A],
            },
            MemoryAddress::RegisterIndirect(Z80Register::DE) => match reg {
                Z80Register::A => vec![0x1A],
                _ => vec![0x7A],
            },
            MemoryAddress::Direct(addr) => {
                let instr = match reg {
                    Z80Register::A => 0x3A,
                    _ => 0x3A,
                };
                vec![instr, (addr & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8]
            }
            MemoryAddress::FrameRelative(offset) => {
                let b = *offset as i8 as u8;
                vec![0xDD, 0x7A, b]
            }
            _ => vec![0x7A],
        }
    }

    // ── LD (addr), reg ──
    fn encode_ld_rm(&self, addr: &MemoryAddress, reg: &Z80Register) -> Vec<u8> {
        match addr {
            MemoryAddress::RegisterIndirect(Z80Register::HL) => match reg {
                Z80Register::A => vec![0x77],
                Z80Register::B => vec![0x70],
                Z80Register::C => vec![0x71],
                Z80Register::D => vec![0x72],
                Z80Register::E => vec![0x73],
                Z80Register::H => vec![0x74],
                Z80Register::L => vec![0x75],
                _ => vec![0x77],
            },
            MemoryAddress::Direct(addr) => {
                let b = (addr & 0xFF) as u8;
                let b1 = ((addr >> 8) & 0xFF) as u8;
                match reg {
                    Z80Register::A => vec![0x32, b, b1],
                    _ => {
                        let load = self.encode_ld_reg(&Z80Register::A, reg);
                        let mut out = load;
                        out.extend_from_slice(&[0x32, b, b1]);
                        out
                    }
                }
            }
            MemoryAddress::FrameRelative(offset) => {
                let b = *offset as i8 as u8;
                vec![0xDD, 0x77, b]
            }
            _ => vec![0x77],
        }
    }

    // ── PUSH / POP ──
    fn encode_push(&self, reg: &Z80Register) -> Vec<u8> {
        match reg {
            Z80Register::B => vec![0xC5],
            Z80Register::C => vec![0xD5],
            Z80Register::D => vec![0xE5],
            Z80Register::E => vec![0xF5],
            Z80Register::HL => vec![0xE5],
            Z80Register::SP => vec![0xF5],
            Z80Register::AF => vec![0xF5],
            Z80Register::BC => vec![0xC5],
            Z80Register::DE => vec![0xD5],
            Z80Register::IX => vec![0xDD, 0xE1],
            Z80Register::IY => vec![0xFD, 0xE1],
            _ => vec![0x00],
        }
    }

    fn encode_pop(&self, reg: &Z80Register) -> Vec<u8> {
        match reg {
            Z80Register::B => vec![0xC1],
            Z80Register::C => vec![0xD1],
            Z80Register::D => vec![0xE1],
            Z80Register::E => vec![0xF1],
            Z80Register::HL => vec![0xE1],
            Z80Register::SP => vec![0xF1],
            Z80Register::AF => vec![0xF1],
            Z80Register::BC => vec![0xC1],
            Z80Register::DE => vec![0xD1],
            Z80Register::IX => vec![0xDD, 0xE1],
            Z80Register::IY => vec![0xFD, 0xE1],
            _ => vec![0x00],
        }
    }

    // ── ADD / SUB ──
    fn encode_add(&self, dst: &Z80Register, src: &Z80Register) -> Vec<u8> {
        match (dst, src) {
            (Z80Register::HL, Z80Register::BC) => vec![0x09],
            (Z80Register::HL, Z80Register::DE) => vec![0x19],
            (Z80Register::HL, Z80Register::HL) => vec![0x09],
            (Z80Register::HL, Z80Register::SP) => vec![0x39],
            (Z80Register::A, Z80Register::A) => vec![0x87],
            (Z80Register::A, Z80Register::B) => vec![0x80],
            (Z80Register::A, Z80Register::C) => vec![0x81],
            (Z80Register::A, Z80Register::D) => vec![0x82],
            (Z80Register::A, Z80Register::E) => vec![0x83],
            (Z80Register::A, Z80Register::H) => vec![0x84],
            (Z80Register::A, Z80Register::L) => vec![0x85],
            _ => vec![0x00],
        }
    }

    fn encode_sub(&self, dst: &Z80Register, src: &Z80Register) -> Vec<u8> {
        match (dst, src) {
            (Z80Register::HL, Z80Register::BC) => vec![0x12],
            (Z80Register::HL, Z80Register::DE) => vec![0x1A],
            (Z80Register::HL, Z80Register::HL) => vec![0x12],
            (Z80Register::HL, Z80Register::SP) => vec![0x3A],
            (Z80Register::A, Z80Register::A) => vec![0x97],
            (Z80Register::A, Z80Register::B) => vec![0x90],
            (Z80Register::A, Z80Register::C) => vec![0x91],
            (Z80Register::A, Z80Register::D) => vec![0x92],
            (Z80Register::A, Z80Register::E) => vec![0x93],
            (Z80Register::A, Z80Register::H) => vec![0x94],
            (Z80Register::A, Z80Register::L) => vec![0x95],
            _ => vec![0x90],
        }
    }

    // ── CP ──
    fn encode_cp(&self, reg: &Z80Register, value: &Option<u8>) -> Vec<u8> {
        match (reg, value) {
            (Z80Register::A, Some(v)) => vec![0xFE, *v],
            (Z80Register::A, None) => match reg {
                Z80Register::A => vec![0xB7],
                Z80Register::B => vec![0xB0],
                Z80Register::C => vec![0xB1],
                Z80Register::D => vec![0xB2],
                Z80Register::E => vec![0xB3],
                Z80Register::H => vec![0xB4],
                Z80Register::L => vec![0xB5],
                _ => vec![0xB7],
            },
            _ => vec![0xBE],
        }
    }
}
