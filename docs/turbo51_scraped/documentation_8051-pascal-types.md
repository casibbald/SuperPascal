# Turbo51 Types | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-types

---

## Table of Contents

- Turbo51 Types
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Turbo51 provides the following system types: Byte (unsigned 8-bit), Word (unsigned 16-bit), ShortInt (signed 8-bit), Integer (signed 16-bit), LongInt (signed 32-bit), Real (uses 4 bytes), String, Boolean, ByteBool, WordBool, LongBool and Char. You can also construct any other type according to Pascal syntax. In Turbo51 there are three types of pointer: ShortPtr (points to IDATA), Pointer (points to XDATA) and CodePointer (points to CODE). Similarly there are ShortPChar, PChar and CodePChar. Pointers to ordinal types can have memory type directive DATA, IDATA or XDATA which overrides default memory type for variables and sets memory type to which this pointer will point to.
Examples:
Type
    PDataWordX = ^TDataWord XDATA;
    TDataWord = Record
                  Case Byte of
                   0: (Word: Word);
                   1: (Byte0, Byte1: Byte);
                   2: (Bits: Set of 0..15);
                   3: (Pointer: Pointer);
                end;

    TGroupType = (Group_0A, Group_0B, Group_15A, Group_15B);

    PByte = ^Byte;
    PByteX = ^Byte XDATA;
    PPointerX = ^Pointer XDATA;
    TPS = Array [1..8] of Char;
    TRT_Text = Array [1..64] of Char;
    TRT_Flags = (rtToggleAB);
    TRT_FlagSet = Set of TRT_Flags;
    PRTX = ^TRT XDATA;
    TRT = Record
            Flags: TRT_FlagSet;
            RepeatNumber: LongInt;
            Text: TRT_Text;
          end;

    TAF = Array [0..NumberOf_AF_FrequenciesInDataSet - 1] of Byte;

    PEON_AF_X = ^TEON_AF XDATA;
    TEON_AF = Record
                Variant: LongInt;
                Case Byte of
                  0: (AF_DataWord: Word);
                  1: (AF_Data1: Byte; AF_Data2: Byte);
              end;

    TEEPROM_Data = Record
                     eeSignature:             TSignature;
                     eeDataSet:               Array [1..NumOfDS] of TDataSet;
                     eeEnd:                   Byte;
                   end;

---

## Code Examples

### Code Block 1

```pascal
Type
    PDataWordX = ^TDataWord XDATA;
    TDataWord = Record
                  Case Byte of
                   0: (Word: Word);
                   1: (Byte0, Byte1: Byte);
                   2: (Bits: Set of 0..15);
                   3: (Pointer: Pointer);
                end;

    TGroupType = (Group_0A, Group_0B, Group_15A, Group_15B);

    PByte = ^Byte;
    PByteX = ^Byte XDATA;
    PPointerX = ^Pointer XDATA;
    TPS = Array [1..8] of Char;
    TRT_Text = Array [1..64] of Char;
    TRT_Flags = (rtToggleAB);
    TRT_FlagSet = Set of TRT_Flags;
    PRTX = ^TRT XDATA;
    TRT = Record
            Flags: TRT_FlagSet;
            RepeatNumber: LongInt;
            Text: TRT_Text;
          end;

    TAF = Array [0..NumberOf_AF_FrequenciesInDataSet - 1] of Byte;

    PEON_AF_X = ^TEON_AF XDATA;
    TEON_AF = Record
                Variant: LongInt;
                Case Byte of
                  0: (AF_DataWord: Word);
                  1: (AF_Data1: Byte; AF_Data2: Byte);
              end;

    TEEPROM_Data = Record
                     eeSignature:             TSignature;
                     eeDataSet:               Array [1..NumOfDS] of TDataSet;
                     eeEnd:                   Byte;
                   end;

```

