# 8051 Assembler Statements | Documentation

**URL:** https://turbo51.com/documentation/8051-assembler-statement

---

## Table of Contents

- 8051 Assembler Statements
  - Pascal Compiler for 8051 Microcontrollers
  - Additional notes:
    - DB
    - DW
    - DD
    - Arithmetic functions
    - OR
    - AND
    - XOR
    - NOT
    - MOD
    - SHR
    - SHL
    - CalledProcedure.Parameter
    - RecordType.Field
    - LOW(Word)
    - HIGH(Word)
    - SWAP(Word)
    - VMTADDRESSTObjectType
    - VMTOFFSETTObjectType.VirtualMethod
    - VMTADDRESSOFFSETTObjectType
    - DMTADDRESSTObjectType
    - DMTINDEXTObjectType.DynamicMethod
    - @LOCALS
    - @PARAMS
    - @RESULT
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Turbo51 assembler statement is very similar to 8051 assembler. You can use all instructions from the
8051 instruction set
. Labels starting with @ don't have to be declared. You don't have to preserve any register and don't assume anything about register content before assembler statement. Byte variables AR0 to AR7 are direct locations for registers R0 to R7 (addresses from $00 to $1F, depending on the active register bank). To access identifier which name is also name of a register, place & before identifier name (example: use &R0 to access identifier R0 and not register R0). Procedure's parameters can be accessed as local variables with
Procedure.Parameter
. See also
assembler procedures
.
Example:
Asm
    MOV      R2, UECP_RX_BufferReadPointer
    MOV      R3, UECP_RX_BufferReadPointer + 1
    MOV      R4, FrameCRCAddress
    MOV      R5, FrameCRCAddress + 1
  @1:
    MOV      DPL, R2
    MOV      DPH, R3
    MOVX     A, @DPTR
    INC      DPTR
    MOV      R2, DPL
    MOV      R3, DPH

    XRL      A, RX_CRC + 1
    MOV      B, #2
    MUL      AB
    ADD      A, #LOW  (CrcTable)
    MOV      DPL, A
    MOV      A, #HIGH (CrcTable)
    ADDC     A, B
    MOV      DPH, A

    CLR      A
    MOVC     A, @A+DPTR
    XRL      A, RX_CRC
    MOV      RX_CRC + 1,A
    MOV      A, #1
    MOVC     A, @A + DPTR
    MOV      RX_CRC, A

    MOV      A, R2
    XRL      A, R4
    JNZ      @1
    MOV      A, R3
    XRL      A, R5
    JNZ      @1

    MOV      DPL, R2
    MOV      DPH, R3    { DPTR points to CRC }
    INC      DPTR       { Move to next frame }
    INC      DPTR
    MOV      UECP_RX_BufferReadPointer, DPL
    MOV      UECP_RX_BufferReadPointer + 1, DPH

    MOV      A, RX_CRC
    XRL      A, #$FF
    XCH      A, RX_CRC + 1
    XRL      A, #$FF
    MOV      RX_CRC, A

    MOV      StoreData.CRC, A
    MOV      StoreData.ReadPointer, DPL
    MOV      StoreData.ReadPointer + 1, DPH
    LCALL    StoreData
end;
Additional notes:
DB
Use DB to define byte.
DW
Use DW to define word.
DD
Use DD to define double word (LongInt).
Arithmetic functions
Use +, -, *, / for integer arithmetic operatons.
OR
Use OR for logical or operation.
AND
Use AND for logical and operation.
XOR
Use XOR for logical xor operation.
NOT
Use NOT for logical not operation.
MOD
Use MOD for integer division modulus.
SHR
Use SHR for right shift.
SHL
Use SHL for left shift.
CalledProcedure.Parameter
Use Procedure.Parameter to access called procedure's parameters.
RecordType.Field
Use RecordType.Field to get the offset of field in record.
LOW(Word)
Use LOW to access low byte of word.
HIGH(Word)
Use HIGH to access high byte of word.
SWAP(Word)
Use SWAP to swap low and high byte of word.
VMTADDRESSTObjectType
Use VMTADDRESS to get the address of Virtual Method Table of TObjectType.
VMTOFFSETTObjectType.VirtualMethod
Use VMTOFFSET to get the offset of TObjectType.VirtualMethod in VMT.
VMTADDRESSOFFSETTObjectType
Use VMTADDRESSOFFSET to get the offset in object of the address of Virtual Method Table.
DMTADDRESSTObjectType
Use DMTADDRESS to get the address of Dynamic Method Table of TObjectType.
DMTINDEXTObjectType.DynamicMethod
Use DMTINDEX to get the index of TObjectType.DynamicMethod.
For reentrant procedures and functions the following symbols are defined:
@LOCALS
@LOCALS returns offset of local variables on XDATA stack.
@PARAMS
@PARAMS returns offset of parameters on XDATA stack.
@RESULT
@RESULT returns offset of result variable on XDATA stack.

---

## Code Examples

### Code Block 1

```pascal
Asm
    MOV      R2, UECP_RX_BufferReadPointer
    MOV      R3, UECP_RX_BufferReadPointer + 1
    MOV      R4, FrameCRCAddress
    MOV      R5, FrameCRCAddress + 1
  @1:
    MOV      DPL, R2
    MOV      DPH, R3
    MOVX     A, @DPTR
    INC      DPTR
    MOV      R2, DPL
    MOV      R3, DPH

    XRL      A, RX_CRC + 1
    MOV      B, #2
    MUL      AB
    ADD      A, #LOW  (CrcTable)
    MOV      DPL, A
    MOV      A, #HIGH (CrcTable)
    ADDC     A, B
    MOV      DPH, A

    CLR      A
    MOVC     A, @A+DPTR
    XRL      A, RX_CRC
    MOV      RX_CRC + 1,A
    MOV      A, #1
    MOVC     A, @A + DPTR
    MOV      RX_CRC, A

    MOV      A, R2
    XRL      A, R4
    JNZ      @1
    MOV      A, R3
    XRL      A, R5
    JNZ      @1

    MOV      DPL, R2
    MOV      DPH, R3    { DPTR points to CRC }
    INC      DPTR       { Move to next frame }
    INC      DPTR
    MOV      UECP_RX_BufferReadPointer, DPL
    MOV      UECP_RX_BufferReadPointer + 1, DPH

    MOV      A, RX_CRC
    XRL      A, #$FF
    XCH      A, RX_CRC + 1
    XRL      A, #$FF
    MOV      RX_CRC, A

    MOV      StoreData.CRC, A
    MOV      StoreData.ReadPointer, DPL
    MOV      StoreData.ReadPointer + 1, DPH
    LCALL    StoreData
end;

```

