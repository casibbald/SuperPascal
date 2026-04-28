//! SuperPascal Z80 Object File Format (.ZOF)
//!
//! This crate implements the Zeal Object File (.ZOF) format for ZealZ80.
//! Object files enable incremental compilation, separate compilation units,
//! and efficient linking.
//!
//! # Format Overview
//!
//! .ZOF files contain:
//! - **Header**: Magic number, version, flags
//! - **Sections**: CODE, DATA, BSS
//! - **Symbol Table**: Exported and imported symbols
//! - **Relocation Entries**: Address fixups for linking
//! - **Debug Info**: Source locations (optional)
//!
//! # Incremental Compilation
//!
//! Object files enable fast rebuilds:
//! - Only recompile changed units
//! - Link pre-compiled object files
//! - Checksum-based dependency tracking

use std::io::{Read, Write};

/// ZOF file magic number: "ZOF\0" (Zeal Object File)
pub const ZOF_MAGIC: &[u8] = b"ZOF\0";
pub const ZOF_VERSION: u16 = 1;

/// Object file sections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    /// Code section (machine code)
    Code = 0,
    /// Initialized data section
    Data = 1,
    /// Uninitialized data section (BSS)
    Bss = 2,
}

impl Section {
    pub fn name(&self) -> &'static str {
        match self {
            Section::Code => "CODE",
            Section::Data => "DATA",
            Section::Bss => "BSS",
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Section::Code),
            1 => Some(Section::Data),
            2 => Some(Section::Bss),
            _ => None,
        }
    }
}

/// Symbol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// Function/procedure
    Function,
    /// Variable
    Variable,
    /// Constant
    Constant,
    /// Type definition
    Type,
    /// External symbol (imported)
    External,
}

/// Symbol visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolVisibility {
    /// Public (exported)
    Public,
    /// Private (internal)
    Private,
}

/// Symbol entry in symbol table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub visibility: SymbolVisibility,
    pub section: Section,
    pub offset: u16, // Offset within section
    pub size: u16,   // Size in bytes
}

/// Relocation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelocationType {
    /// Absolute 16-bit address
    Absolute16,
    /// Relative 8-bit offset (for JR instructions)
    Relative8,
    /// Relative 16-bit offset (for CALL, JP)
    Relative16,
    /// High byte of address
    HighByte,
    /// Low byte of address
    LowByte,
}

/// Relocation entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relocation {
    pub section: Section,
    pub offset: u16, // Offset within section where relocation applies
    pub relocation_type: RelocationType,
    pub symbol_name: String, // Symbol to resolve
    pub addend: i16,         // Addend value (for PC-relative, etc.)
}

/// ZOF object file structure
#[derive(Debug, Clone)]
pub struct ObjectFile {
    /// Source unit name
    pub unit_name: String,
    /// Code section data
    pub code: Vec<u8>,
    /// Data section data
    pub data: Vec<u8>,
    /// BSS section size (uninitialized data)
    pub bss_size: u16,
    /// Symbol table
    pub symbols: Vec<Symbol>,
    /// Relocation entries
    pub relocations: Vec<Relocation>,
    /// Unit initialization address (if any)
    pub init_address: Option<u16>,
    /// Unit finalization address (if any)
    pub fini_address: Option<u16>,
}

impl ObjectFile {
    /// Create a new empty object file
    pub fn new(unit_name: String) -> Self {
        Self {
            unit_name,
            code: Vec::new(),
            data: Vec::new(),
            bss_size: 0,
            symbols: vec![],
            relocations: vec![],
            init_address: None,
            fini_address: None,
        }
    }

    /// Add code bytes
    pub fn add_code(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    /// Add data bytes
    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    /// Set BSS size
    pub fn set_bss_size(&mut self, size: u16) {
        self.bss_size = size;
    }

    /// Add a symbol
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    /// Add a relocation entry
    pub fn add_relocation(&mut self, relocation: Relocation) {
        self.relocations.push(relocation);
    }

    /// Write object file to binary format
    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // Write header
        writer.write_all(ZOF_MAGIC)?;
        writer.write_all(&ZOF_VERSION.to_le_bytes())?;

        // Write unit name (length-prefixed string)
        let unit_name_bytes = self.unit_name.as_bytes();
        writer.write_all(&(unit_name_bytes.len() as u16).to_le_bytes())?;
        writer.write_all(unit_name_bytes)?;

        // Write sections
        // CODE section
        writer.write_all(&(self.code.len() as u32).to_le_bytes())?;
        writer.write_all(&self.code)?;

        // DATA section
        writer.write_all(&(self.data.len() as u32).to_le_bytes())?;
        writer.write_all(&self.data)?;

        // BSS section (size only, no data)
        writer.write_all(&self.bss_size.to_le_bytes())?;

        // Write symbol table
        writer.write_all(&(self.symbols.len() as u16).to_le_bytes())?;
        for symbol in &self.symbols {
            self.write_symbol(writer, symbol)?;
        }

        // Write relocation entries
        writer.write_all(&(self.relocations.len() as u16).to_le_bytes())?;
        for relocation in &self.relocations {
            self.write_relocation(writer, relocation)?;
        }

        // Write init/fini addresses
        writer.write_all(&[self.init_address.is_some() as u8])?;
        if let Some(addr) = self.init_address {
            writer.write_all(&addr.to_le_bytes())?;
        }
        writer.write_all(&[self.fini_address.is_some() as u8])?;
        if let Some(addr) = self.fini_address {
            writer.write_all(&addr.to_le_bytes())?;
        }

        Ok(())
    }

    /// Read object file from binary format
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        // Read and verify magic
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if &magic != ZOF_MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid ZOF magic number",
            ));
        }

        // Read version
        let mut version_bytes = [0u8; 2];
        reader.read_exact(&mut version_bytes)?;
        let version = u16::from_le_bytes(version_bytes);
        if version != ZOF_VERSION {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("Unsupported ZOF version: {}", version),
            ));
        }

        // Read unit name
        let mut name_len_bytes = [0u8; 2];
        reader.read_exact(&mut name_len_bytes)?;
        let name_len = u16::from_le_bytes(name_len_bytes) as usize;
        let mut name_bytes = vec![0u8; name_len];
        reader.read_exact(&mut name_bytes)?;
        let unit_name = String::from_utf8(name_bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Read CODE section
        let mut code_len_bytes = [0u8; 4];
        reader.read_exact(&mut code_len_bytes)?;
        let code_len = u32::from_le_bytes(code_len_bytes) as usize;
        let mut code = vec![0u8; code_len];
        reader.read_exact(&mut code)?;

        // Read DATA section
        let mut data_len_bytes = [0u8; 4];
        reader.read_exact(&mut data_len_bytes)?;
        let data_len = u32::from_le_bytes(data_len_bytes) as usize;
        let mut data = vec![0u8; data_len];
        reader.read_exact(&mut data)?;

        // Read BSS size
        let mut bss_size_bytes = [0u8; 2];
        reader.read_exact(&mut bss_size_bytes)?;
        let bss_size = u16::from_le_bytes(bss_size_bytes);

        // Read symbol table
        let mut symbol_count_bytes = [0u8; 2];
        reader.read_exact(&mut symbol_count_bytes)?;
        let symbol_count = u16::from_le_bytes(symbol_count_bytes);
        let mut symbols = Vec::with_capacity(symbol_count as usize);
        for _ in 0..symbol_count {
            symbols.push(Self::read_symbol(reader)?);
        }

        // Read relocation entries
        let mut reloc_count_bytes = [0u8; 2];
        reader.read_exact(&mut reloc_count_bytes)?;
        let reloc_count = u16::from_le_bytes(reloc_count_bytes);
        let mut relocations = Vec::with_capacity(reloc_count as usize);
        for _ in 0..reloc_count {
            relocations.push(Self::read_relocation(reader)?);
        }

        // Read init/fini addresses
        let mut has_init = [0u8; 1];
        reader.read_exact(&mut has_init)?;
        let init_address = if has_init[0] != 0 {
            let mut addr_bytes = [0u8; 2];
            reader.read_exact(&mut addr_bytes)?;
            Some(u16::from_le_bytes(addr_bytes))
        } else {
            None
        };

        let mut has_fini = [0u8; 1];
        reader.read_exact(&mut has_fini)?;
        let fini_address = if has_fini[0] != 0 {
            let mut addr_bytes = [0u8; 2];
            reader.read_exact(&mut addr_bytes)?;
            Some(u16::from_le_bytes(addr_bytes))
        } else {
            None
        };

        Ok(Self {
            unit_name,
            code,
            data,
            bss_size,
            symbols,
            relocations,
            init_address,
            fini_address,
        })
    }

    fn write_symbol<W: Write>(&self, _writer: &mut W, symbol: &Symbol) -> std::io::Result<()> {
        // Name (length-prefixed)
        let name_bytes = symbol.name.as_bytes();
        _writer.write_all(&[name_bytes.len() as u8])?;
        _writer.write_all(name_bytes)?;

        // Type, visibility, section (packed into 1 byte)
        let section_val: u8 = symbol.section as u8;
        let flags =
            (symbol.symbol_type as u8) | ((symbol.visibility as u8) << 4) | (section_val << 5);
        _writer.write_all(&[flags])?;

        // Offset, size
        _writer.write_all(&symbol.offset.to_le_bytes())?;
        _writer.write_all(&symbol.size.to_le_bytes())?;

        Ok(())
    }

    fn read_symbol<R: Read>(reader: &mut R) -> std::io::Result<Symbol> {
        // Name
        let mut name_len = [0u8; 1];
        reader.read_exact(&mut name_len)?;
        let mut name_bytes = vec![0u8; name_len[0] as usize];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8(name_bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Flags
        let mut flags = [0u8; 1];
        reader.read_exact(&mut flags)?;
        let symbol_type = match flags[0] & 0x0F {
            0 => SymbolType::Function,
            1 => SymbolType::Variable,
            2 => SymbolType::Constant,
            3 => SymbolType::Type,
            4 => SymbolType::External,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid symbol type",
                ));
            }
        };
        let visibility = if (flags[0] & 0x10) != 0 {
            SymbolVisibility::Public
        } else {
            SymbolVisibility::Private
        };
        let section = Section::from_u8((flags[0] >> 5) & 0x03).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid section")
        })?;

        // Offset, size
        let mut offset_bytes = [0u8; 2];
        reader.read_exact(&mut offset_bytes)?;
        let offset = u16::from_le_bytes(offset_bytes);

        let mut size_bytes = [0u8; 2];
        reader.read_exact(&mut size_bytes)?;
        let size = u16::from_le_bytes(size_bytes);

        Ok(Symbol {
            name,
            symbol_type,
            visibility,
            section,
            offset,
            size,
        })
    }

    fn write_relocation<W: Write>(
        &self,
        _writer: &mut W,
        reloc: &Relocation,
    ) -> std::io::Result<()> {
        // Section, type (packed)
        let section_val: u8 = reloc.section as u8;
        let flags = section_val | ((reloc.relocation_type as u8) << 2);
        _writer.write_all(&[flags])?;

        // Offset
        _writer.write_all(&reloc.offset.to_le_bytes())?;

        // Symbol name (length-prefixed)
        let name_bytes = reloc.symbol_name.as_bytes();
        _writer.write_all(&[name_bytes.len() as u8])?;
        _writer.write_all(name_bytes)?;

        // Addend
        _writer.write_all(&reloc.addend.to_le_bytes())?;

        Ok(())
    }

    fn read_relocation<R: Read>(reader: &mut R) -> std::io::Result<Relocation> {
        // Flags
        let mut flags = [0u8; 1];
        reader.read_exact(&mut flags)?;
        let section = Section::from_u8(flags[0] & 0x03).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid section")
        })?;
        let relocation_type = match (flags[0] >> 2) & 0x07 {
            0 => RelocationType::Absolute16,
            1 => RelocationType::Relative8,
            2 => RelocationType::Relative16,
            3 => RelocationType::HighByte,
            4 => RelocationType::LowByte,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid relocation type",
                ));
            }
        };

        // Offset
        let mut offset_bytes = [0u8; 2];
        reader.read_exact(&mut offset_bytes)?;
        let offset = u16::from_le_bytes(offset_bytes);

        // Symbol name
        let mut name_len = [0u8; 1];
        reader.read_exact(&mut name_len)?;
        let mut name_bytes = vec![0u8; name_len[0] as usize];
        reader.read_exact(&mut name_bytes)?;
        let symbol_name = String::from_utf8(name_bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Addend
        let mut addend_bytes = [0u8; 2];
        reader.read_exact(&mut addend_bytes)?;
        let addend = i16::from_le_bytes(addend_bytes);

        Ok(Relocation {
            section,
            offset,
            relocation_type,
            symbol_name,
            addend,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_file_creation() {
        let mut obj = ObjectFile::new("TestUnit".to_string());
        obj.add_code(&[0x3E, 0x42]); // ld a, 0x42
        obj.add_data(&[0x01, 0x02, 0x03]);
        obj.set_bss_size(10);

        assert_eq!(obj.code.len(), 2);
        assert_eq!(obj.data.len(), 3);
        assert_eq!(obj.bss_size, 10);
    }

    #[test]
    fn test_object_file_write_read() {
        let mut obj = ObjectFile::new("TestUnit".to_string());
        obj.add_code(&[0x3E, 0x42, 0xC9]); // ld a, 0x42; ret
        obj.add_data(&[0x01, 0x02]);
        obj.set_bss_size(5);

        obj.add_symbol(Symbol {
            name: "TestFunc".to_string(),
            symbol_type: SymbolType::Function,
            visibility: SymbolVisibility::Public,
            section: Section::Code,
            offset: 0,
            size: 3,
        });

        obj.add_relocation(Relocation {
            section: Section::Code,
            offset: 1,
            relocation_type: RelocationType::Absolute16,
            symbol_name: "ExternalSymbol".to_string(),
            addend: 0,
        });

        // Write to buffer
        let mut buffer = Vec::new();
        obj.write(&mut buffer).unwrap();

        // Read back
        let mut reader = std::io::Cursor::new(buffer);
        let obj2 = ObjectFile::read(&mut reader).unwrap();

        assert_eq!(obj.unit_name, obj2.unit_name);
        assert_eq!(obj.code, obj2.code);
        assert_eq!(obj.data, obj2.data);
        assert_eq!(obj.bss_size, obj2.bss_size);
        assert_eq!(obj.symbols.len(), obj2.symbols.len());
        assert_eq!(obj.relocations.len(), obj2.relocations.len());
    }

    #[test]
    fn test_symbol_table() {
        let mut obj = ObjectFile::new("TestUnit".to_string());
        obj.add_symbol(Symbol {
            name: "PublicFunc".to_string(),
            symbol_type: SymbolType::Function,
            visibility: SymbolVisibility::Public,
            section: Section::Code,
            offset: 0,
            size: 10,
        });
        obj.add_symbol(Symbol {
            name: "PrivateVar".to_string(),
            symbol_type: SymbolType::Variable,
            visibility: SymbolVisibility::Private,
            section: Section::Data,
            offset: 0,
            size: 2,
        });

        assert_eq!(obj.symbols.len(), 2);
        assert_eq!(obj.symbols[0].name, "PublicFunc");
        assert_eq!(obj.symbols[1].name, "PrivateVar");
    }
}
