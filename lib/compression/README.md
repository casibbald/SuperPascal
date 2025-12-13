# Compression Library

**Location:** `SuperPascal/lib/compression/`

---

## Overview

The Compression library provides lossless data compression algorithms for SuperPascal. All algorithms are implemented in pure Pascal for maximum portability across all target platforms.

**Status:** ✅ Complete (3 modules)

---

## Module Structure

```
lib/compression/
├── mod.pas          # Main entry point
├── types.pas        # Core types and constants
├── rle.pas          # Run-Length Encoding
└── lz77.pas         # LZ77 (LZSS) compression
```

---

## Algorithms

### 1. Run-Length Encoding (RLE)

**Module:** `Compression_RLE`

Simple compression algorithm that replaces sequences of repeated bytes with a count and the byte value.

**Best For:**
- Data with long runs of identical bytes (e.g., simple graphics, sparse data)
- Fast compression/decompression
- Low memory usage

**Performance:**
- Compression: O(n) time, O(n) space
- Decompression: O(n) time, O(n) space
- Compression ratio: 1:1 to 1:255 (depends on data)

**API:**

```pascal
uses Compression_RLE;

var
  input, output: PByte;
  inputSize, outputSize: LongInt;
  params: TRLEParams;
  success: Boolean;
begin
  // Set up input data
  inputSize := 1000;
  GetMem(input, inputSize);
  // ... fill input data ...
  
  // Allocate output buffer (at least inputSize bytes)
  outputSize := inputSize;
  GetMem(output, outputSize);
  
  // Compress with default parameters
  success := RLECompress(input, inputSize, output, outputSize, nil);
  
  if success then
  begin
    // outputSize now contains actual compressed size
    WriteLn('Compressed from ', inputSize, ' to ', outputSize, ' bytes');
  end;
  
  // Decompress
  inputSize := outputSize;  // Use compressed size
  outputSize := 1000;  // Original size
  success := RLEDecompress(output, inputSize, input, outputSize);
  
  // Calculate compression ratio
  var ratio: Fixed16;
  ratio := RLECalculateRatio(1000, outputSize);
end.
```

**Parameters:**

```pascal
type
  TRLEParams = record
    MaxRunLength: Byte;  // Maximum run length (default 255)
    MinRunLength: Byte;  // Minimum run length to encode (default 3)
  end;
```

**Encoding Format:**
- Run: `0x80 | (length - 3)` followed by byte value
- Literal sequence: `0x01-0x7E` (count-1) followed by bytes
- Escape: `0x7F` followed by high byte value
- Single zero: `0x00`

---

### 2. LZ77 (LZSS Variant)

**Module:** `Compression_LZ77`

Sliding window compression that replaces repeated strings with references to previous occurrences in a sliding window.

**Best For:**
- General-purpose data compression
- Text files, binary data
- Better compression ratios than RLE for most data

**Performance:**
- Compression: O(n * w) time (w = window size), O(w) space
- Decompression: O(n) time, O(w) space
- Compression ratio: Typically 2:1 to 10:1 (depends on data)

**API:**

```pascal
uses Compression_LZ77;

var
  input, output: PByte;
  inputSize, outputSize: LongInt;
  params: TLZ77Params;
  success: Boolean;
begin
  // Set up input data
  inputSize := 10000;
  GetMem(input, inputSize);
  // ... fill input data ...
  
  // Allocate output buffer (at least inputSize bytes)
  outputSize := inputSize;
  GetMem(output, outputSize);
  
  // Compress with custom parameters
  params.WindowSize := 4096;
  params.LookAheadSize := 18;
  params.MinMatchLength := 2;
  success := LZ77Compress(input, inputSize, output, outputSize, @params);
  
  if success then
  begin
    WriteLn('Compressed from ', inputSize, ' to ', outputSize, ' bytes');
  end;
  
  // Decompress
  inputSize := outputSize;  // Use compressed size
  outputSize := 10000;  // Original size
  success := LZ77Decompress(output, inputSize, input, outputSize);
  
  // Calculate compression ratio
  var ratio: Fixed16;
  ratio := LZ77CalculateRatio(10000, outputSize);
end.
```

**Parameters:**

```pascal
type
  TLZ77Params = record
    WindowSize: Word;     // Sliding window size (default 4096)
    LookAheadSize: Word; // Look-ahead buffer size (default 18)
    MinMatchLength: Byte; // Minimum match length (default 2)
  end;
```

**File Format:**
- Header: 4 bytes (original file size, little-endian)
- Compression tags: 1 byte per 8 units (bit flags: 0=literal, 1=encoded)
- Literal bytes: 1 byte each
- Encoded strings: 2 bytes (position + length)

**Encoding Format:**
- Tag byte: 8 bits, each bit flags literal (0) or encoded (1)
- Literal: 1 byte
- Encoded: 2 bytes
  - Byte 1: upper nibble = bits 11-8 of position, lower nibble = length - 2
  - Byte 2: bits 7-0 of position

---

## Quick Start

### Example 1: Simple RLE Compression

```pascal
program ExampleRLE;

uses Compression;

var
  data: array[0..99] of Byte;
  compressed: PByte;
  dataSize, compSize: LongInt;
  i: Integer;
begin
  // Create test data (repeating pattern)
  for i := 0 to 99 do
    data[i] := (i mod 10);
  
  dataSize := 100;
  compSize := dataSize;
  GetMem(compressed, compSize);
  
  if RLECompress(@data[0], dataSize, compressed, compSize, nil) then
  begin
    WriteLn('Compressed: ', dataSize, ' -> ', compSize, ' bytes');
    WriteLn('Ratio: ', Fixed16ToInt(RLECalculateRatio(dataSize, compSize) * 100), '%');
  end;
  
  FreeMem(compressed);
end.
```

### Example 2: LZ77 Compression

```pascal
program ExampleLZ77;

uses Compression;

var
  text: String;
  input, output: PByte;
  inputSize, outputSize: LongInt;
begin
  text := 'The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.';
  inputSize := Length(text);
  GetMem(input, inputSize);
  Move(text[1], input^, inputSize);
  
  outputSize := inputSize;
  GetMem(output, outputSize);
  
  if LZ77Compress(input, inputSize, output, outputSize, nil) then
  begin
    WriteLn('Compressed: ', inputSize, ' -> ', outputSize, ' bytes');
  end;
  
  FreeMem(input);
  FreeMem(output);
end.
```

---

## Dependencies

- `Math_Types` - For `Fixed16` type
- `Math_Fixed` - For fixed-point arithmetic

---

## Platform Considerations

### All Platforms

- **Pure Pascal implementation** - No assembly required
- **Memory efficient** - Uses sliding windows for LZ77
- **Portable** - Works on all target platforms

### Performance Notes

- **RLE:** Very fast, suitable for real-time compression
- **LZ77:** Moderate speed, good compression ratios
- **Memory usage:** LZ77 uses O(window_size) memory

### Optimization Opportunities

If profiling shows compression is a bottleneck:

1. **LZ77 search optimization:** Replace linear search with hash table
2. **RLE:** Already optimal for its use case
3. **Platform-specific:** Add inline assembly for inner loops (if needed)

---

## Algorithm Comparison

| Algorithm | Speed | Ratio | Memory | Best For |
|-----------|-------|-------|--------|----------|
| RLE | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ | Repeated bytes |
| LZ77 | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | General purpose |

---

## Future Enhancements

- [ ] Huffman encoding (if needed)
- [ ] LZ78/LZW (if needed)
- [ ] Dictionary-based compression
- [ ] Adaptive compression parameters
- [ ] Streaming compression API

---

## References

- **Source Material:** Mikro Documentation Archive
  - `docs/mikro_docs_archive/Coding/2/COMPRFAQ.TXT` - Compression FAQ
  - Motorola Atari ST Sources - LZ77 implementation reference
- **Algorithms:** `languageSpecification/algorithms/09_UtilityAlgorithms.md`

---

**Last Updated:** 2025-01-XX  
**Status:** Complete (RLE and LZ77 implemented)

