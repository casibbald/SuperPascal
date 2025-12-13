# Crypto Library

**Location:** `SuperPascal/lib/crypto/`

---

## Overview

The Crypto library provides cryptographic checksum algorithms for SuperPascal. Currently implements CRC (Cyclic Redundancy Check) algorithms for data integrity verification.

**Status:** ✅ Complete (3 modules)

---

## Module Structure

```
lib/crypto/
├── mod.pas          # Main entry point
├── types.pas        # Core types and constants
└── crc.pas          # CRC checksum algorithms
```

---

## Algorithms

### CRC (Cyclic Redundancy Check)

**Module:** `Crypto_CRC`

CRC is a checksum algorithm used for detecting errors in data transmission or storage. The library provides table-driven implementations for fast CRC calculation.

**Supported Standards:**
- **CRC-16 (Standard):** CRC-16-IBM polynomial, used in many protocols
- **CRC-16 (Reversed):** Non-reflected variant
- **CRC-16-CCITT:** Used in X.25, HDLC, and other protocols
- **CRC-32:** Used in ZIP files, PNG images, Ethernet, and many other formats

**Best For:**
- Data integrity verification
- Error detection in file transfers
- Checksum validation
- File format compatibility (ZIP, PNG, etc.)

**Performance:**
- Calculation: O(n) time, O(1) space (after table initialization)
- Table initialization: O(256) time, O(256) space
- Very fast: ~1 cycle per byte (table-driven)

**API:**

```pascal
uses Crypto;

var
  data: PByte;
  dataSize: LongInt;
  crc16: Word;
  crc32: LongWord;
begin
  // Calculate CRC-16 (standard)
  crc16 := CalculateCRC16(data, dataSize, nil);
  
  // Calculate CRC-16 with custom parameters
  var params: TCRCParams;
  params := CRC16_CCITT;
  crc16 := CalculateCRC16(data, dataSize, @params);
  
  // Calculate CRC-32
  crc32 := CalculateCRC32(data, dataSize);
end.
```

**Incremental API:**

```pascal
uses Crypto_CRC;

var
  context: TCRCContext;
  crc: LongWord;
begin
  // Initialize CRC context
  CRCInit(context, CRC32_STANDARD);
  
  // Update with data in chunks
  CRCUpdateBlock(context, data1, size1);
  CRCUpdateBlock(context, data2, size2);
  CRCUpdateByte(context, singleByte);
  
  // Finalize and get result
  crc := CRCFinalize(context);
end.
```

---

## Quick Start

### Example 1: Simple CRC-16 Calculation

```pascal
program ExampleCRC16;

uses Crypto;

var
  data: array[0..99] of Byte;
  crc: Word;
  i: Integer;
begin
  // Create test data
  for i := 0 to 99 do
    data[i] := Byte(i);
  
  // Calculate CRC-16
  crc := CalculateCRC16(@data[0], 100, nil);
  WriteLn('CRC-16: $', HexStr(crc, 4));
end.
```

### Example 2: CRC-32 for File Integrity

```pascal
program ExampleCRC32;

uses Crypto;

var
  fileData: PByte;
  fileSize: LongInt;
  crc32: LongWord;
begin
  // Read file into memory
  // ... file I/O code ...
  
  // Calculate CRC-32
  crc32 := CalculateCRC32(fileData, fileSize);
  WriteLn('File CRC-32: $', HexStr(crc32, 8));
  
  // Verify against stored checksum
  if crc32 = expectedCRC32 then
    WriteLn('File integrity verified!')
  else
    WriteLn('File corrupted!');
end.
```

### Example 3: Incremental CRC Calculation

```pascal
program ExampleIncrementalCRC;

uses Crypto_CRC;

var
  context: TCRCContext;
  crc: LongWord;
  chunk1, chunk2: array[0..49] of Byte;
  i: Integer;
begin
  // Initialize CRC-32
  CRCInit(context, CRC32_STANDARD);
  
  // Process first chunk
  for i := 0 to 49 do
    chunk1[i] := Byte(i);
  CRCUpdateBlock(context, @chunk1[0], 50);
  
  // Process second chunk
  for i := 0 to 49 do
    chunk2[i] := Byte(i + 50);
  CRCUpdateBlock(context, @chunk2[0], 50);
  
  // Finalize
  crc := CRCFinalize(context);
  WriteLn('CRC-32: $', HexStr(crc, 8));
end.
```

---

## CRC Parameters

### Standard CRC-16 (CRC-16-IBM)

```pascal
CRC16_STANDARD: TCRCParams = (
  Width: 16;
  Poly: $8005;      // Reflected polynomial
  Init: $0000;
  RefIn: True;
  RefOut: True;
  XorOut: $0000
);
```

### CRC-16-CCITT

```pascal
CRC16_CCITT: TCRCParams = (
  Width: 16;
  Poly: $1021;
  Init: $FFFF;
  RefIn: False;
  RefOut: False;
  XorOut: $0000
);
```

### Standard CRC-32 (ZIP/PNG)

```pascal
CRC32_STANDARD: TCRCParams = (
  Width: 32;
  Poly: $EDB88320;  // Reflected polynomial
  Init: $FFFFFFFF;
  RefIn: True;
  RefOut: True;
  XorOut: $FFFFFFFF
);
```

---

## Dependencies

- None (standalone library)

---

## Platform Considerations

### All Platforms

- **Pure Pascal implementation** - No assembly required
- **Table-driven** - Fast O(1) per-byte calculation
- **Portable** - Works on all target platforms
- **Memory efficient** - 256-entry lookup table (1KB for CRC-32, 512 bytes for CRC-16)

### Performance Notes

- **Table initialization:** One-time cost, typically done at startup
- **CRC calculation:** Very fast, ~1 cycle per byte (table lookup + XOR)
- **Memory usage:** 256 entries × 4 bytes = 1KB for CRC-32, 512 bytes for CRC-16

### Optimization Opportunities

If profiling shows CRC is a bottleneck:

1. **Platform-specific tables:** Pre-computed tables for each platform
2. **Inline assembly:** Table lookup + XOR in assembly (if needed)
3. **SIMD:** Use SIMD for parallel CRC calculation (ARM64, if needed)

---

## Algorithm Details

### CRC Calculation Process

1. **Initialize:** Set CRC to initial value
2. **Update:** For each byte:
   - Look up value in pre-computed table
   - XOR with current CRC value
   - Shift/reflect as needed
3. **Finalize:** Reflect (if needed) and XOR with final value

### Table Generation

The lookup table is generated using the CRC polynomial. Each entry represents the CRC value for a single byte input, pre-computed to avoid repeated polynomial division.

---

## Use Cases

1. **File Integrity:** Verify files haven't been corrupted
2. **Network Protocols:** Error detection in data transmission
3. **File Formats:** ZIP, PNG, and other formats use CRC-32
4. **Data Validation:** Verify data integrity in databases, storage systems

---

## Future Enhancements

- [ ] Additional CRC variants (CRC-8, CRC-64)
- [ ] Other checksums (Adler-32, Fletcher)
- [ ] Hash functions (if needed for security)
- [ ] Message authentication codes (if needed)

---

## References

- **Source Material:** Mikro Documentation Archive
  - `docs/mikro_docs_archive/Coding/1/CRC.TXT` - CRC algorithm reference
  - Motorola Atari ST Sources - CRC implementation reference
- **Standards:**
  - CRC-16: ISO/IEC 13239, ITU-T V.41
  - CRC-32: ISO 3309, ITU-T V.42, IEEE 802.3
- **Algorithms:** `languageSpecification/algorithms/09_UtilityAlgorithms.md`

---

**Last Updated:** 2025-01-XX  
**Status:** Complete (CRC-16 and CRC-32 implemented)

