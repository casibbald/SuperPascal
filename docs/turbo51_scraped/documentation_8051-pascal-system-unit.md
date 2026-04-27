# Turbo51 System Unit | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-system-unit

---

## Table of Contents

- Turbo51 System Unit
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
System unit implements Turbo51 runtime library and defines some special function registers (SFR), bits and interrupt addresses that are present in all microcontrollers based on 8051 core. It is implicitly used by the compiler. Usually it is loaded from the library (Turbo51.l51) which is a binary concatenation of units (currently only system unit is included).
There is also the Turbo51A.l51 library which contains the system unit compiled with the $A+ switch and has no LCALL/LJMP instructions (to use it use the /LA command line option). Warning: some declarations in this this unit might change in future versions.
Unit System;

Interface

Const

  BELL = $07;
  BS   = $08;
  TAB  = $09;
  LF   = $0A;
  CR   = $0D;
  EOF  = $1A;
  ESC  = $1B;
  DEL  = $7F;

  External0 = $0003;
  Timer0    = $000B;
  External1 = $0013;
  Timer1    = $001B;
  Serial    = $0023;

Type TDeviceWriteProcedure = Procedure;

     TDeviceReadFunction = Function: Char;

     TFileRecord = Record
                     WriteProcedure: TDeviceWriteProcedure;
                     ReadFunction:   TDeviceReadFunction;
                   end;


Var

// Rn registers at absolute address (bank dependent) - Must be declared before 8051 SFR
// Registers ARn SHOULD NOT be used in system unit since system procedures could be called
// from any procedure using any register bank

  AR0:      Byte absolute 0;
  AR1:      Byte absolute 1;
  AR2:      Byte absolute 2;
  AR3:      Byte absolute 3;
  AR4:      Byte absolute 4;
  AR5:      Byte absolute 5;
  AR6:      Byte absolute 6;
  AR7:      Byte absolute 7;

 { 8051 SFR }

  P0:       Byte absolute $80; Volatile;
  SP:       Byte absolute $81; Volatile;
  DPL:      Byte absolute $82;
  DPH:      Byte absolute $83;
  PCON:     Byte absolute $87; Volatile;
  TCON:     Byte absolute $88; Volatile;
  TMOD:     Byte absolute $89; Volatile;
  TL0:      Byte absolute $8A; Volatile;
  TL1:      Byte absolute $8B; Volatile;
  TH0:      Byte absolute $8C; Volatile;
  TH1:      Byte absolute $8D; Volatile;
  P1:       Byte absolute $90; Volatile;
  SCON:     Byte absolute $98; Volatile;
  SBUF:     Byte absolute $99; Volatile;
  P2:       Byte absolute $A0; Volatile;
  IE:       Byte absolute $A8; Volatile;
  P3:       Byte absolute $B0; Volatile;
  IP:       Byte absolute $B8; Volatile;
  PSW:      Byte absolute $D0; Volatile;
  ACC:      Byte absolute $E0;
  B:        Byte absolute $F0;

  DPTR:     Pointer absolute $82;

{ TCON }
  TF1:      Boolean absolute TCON.7;
  TR1:      Boolean absolute TCON.6;
  TF0:      Boolean absolute TCON.5;
  TR0:      Boolean absolute TCON.4;
  IE1:      Boolean absolute TCON.3;
  IT1:      Boolean absolute TCON.2;
  IE0:      Boolean absolute TCON.1;
  IT0:      Boolean absolute TCON.0;

{ SCON }
  SM0:      Boolean absolute SCON.7;
  SM1:      Boolean absolute SCON.6;
  SM2:      Boolean absolute SCON.5;
  REN:      Boolean absolute SCON.4;
  TB8:      Boolean absolute SCON.3;
  RB8:      Boolean absolute SCON.2;
  TI:       Boolean absolute SCON.1;
  RI:       Boolean absolute SCON.0;

{ IE }
  EA:       Boolean absolute IE.7;
  ES:       Boolean absolute IE.4;
  ET1:      Boolean absolute IE.3;
  EX1:      Boolean absolute IE.2;
  ET0:      Boolean absolute IE.1;
  EX0:      Boolean absolute IE.0;

{ P3 }
  RD:       Boolean absolute P3.7;
  WR:       Boolean absolute P3.6;
  T1:       Boolean absolute P3.5;
  T0:       Boolean absolute P3.4;
  INT1:     Boolean absolute P3.3;
  INT0:     Boolean absolute P3.2;
  TXD:      Boolean absolute P3.1;
  RXD:      Boolean absolute P3.0;

{ IP}
  PS:       Boolean absolute IP.4;
  PT1:      Boolean absolute IP.3;
  PX1:      Boolean absolute IP.2;
  PT0:      Boolean absolute IP.1;
  PX0:      Boolean absolute IP.0;

{ PSW }
  CY:       Boolean absolute PSW.7;
  AC:       Boolean absolute PSW.6;
  F0:       Boolean absolute PSW.5;
  RS1:      Boolean absolute PSW.4;
  RS0:      Boolean absolute PSW.3;
  OV:       Boolean absolute PSW.2;
  P:        Boolean absolute PSW.0;

{ Vars declared before MemCODE will not be shown in asm file }

  MemCODE:  Array [$0000..$FFFF] of Byte CODE  absolute $0000;

{ Warning: for addresses above $80 MemDATA can only be accessed by constant array index }
{ otherwise the compiler will generate indirect instruction and IDATA memory will be accessed }

  MemDATA:  Array [  $00..  $FF] of Byte DATA  absolute   $00;

  MemIDATA: Array [  $00..  $FF] of Byte IDATA absolute   $00;
  MemXDATA: Array [$0000..$FFFF] of Byte XDATA absolute $0000;


Var StackStart:       Byte IDATA;  // Used for SP initialization, first byte of stack
                                   // linker will set its address as last variable in
                                   // (I)DATA, it might also be in IDATA segment!
    XDATA_StackStart: Word XDATA;  // Used for XSP and XBP initialization, first byte of
                                   // XDATA stack (linker will set its address)
    R8, R9:           Byte DATA;   // Used for LongInt set 1 and as additional registers,
                                   // R8 and R9 must be first variables in DATA segment
    TempRegister: Byte DATA;       // Used for UsingAny procedures as temporary storage
                                   // during PUSH / POP
    XSP, XBP: Pointer DATA;        // Used for recursion stack and local variables in XDATA
    HeapOrg,
    HeapPtr,
    HeapEnd:     Pointer DATA;     // Used for heap management
    FreeList:    Pointer XDATA;    // Used for heap management, must be in XDATA to avoid
                                   // XDATA nil and to be compatible with TFreeRec in XDATA
                                   // memory
    HeapError:   Procedure DATA;
    RandomSeed:  LongInt DATA;     // Used for random numbers
    CurrentIO:   File DATA;        // Used for file I/O
    SystemIO:    Text DATA;        // Used for Read/Readln/Write/Writeln
    Input:       Text absolute SystemIO;
    Output:      Text absolute SystemIO;
    LastCharacterBuffer: Char;         // Used for sysReadCharFromCurrentDevice
    LastCharacterBufferValid: Boolean; // Used for sysReadCharFromCurrentDevice
    Overflow: Boolean;                 // Used for arithmetic functions to indicate
                                       // overflow

    ResultSign: Boolean;           // Used for signed arithmetics
    TempBool0: Boolean;            // Used for artithmetic functions
    TempBool1: Boolean;            // Used for system procedure Val, artithmetic functions
    TempBool2: Boolean;            // Used for system procedure Val
    TempWord:  Word DATA;          // Used for heap management - sysNewMemory
    TempByte0: Byte DATA;          // Used for long multiplication, long division
    TempByte1: Byte DATA;
    TempByte2: Byte DATA;
    TempByte3: Byte DATA;
    TempByte4: Byte DATA;
    TempByte5: Byte DATA;
    TempByte6: Byte DATA;
    TempByte7: Byte DATA;
    TempByte8: Byte DATA;
    TempByte9: Byte DATA;
    TempByte10: Byte DATA;

Var RealSigns: Byte DATA;
    RealResult: LongInt DATA;
    RealResultCarry: Byte DATA;

Const Pi_2:     Real = Pi / 2;
      Pi_24:    Real = Pi / 24;
      _Pi:      Real = Pi;
      _2Pi:     Real = 2 * Pi;
      _2_Pi:    Real = 2 / Pi;
      _0_5:     Real = 0.5;
      _1:       Real = 1;
      Sqrt2:    Real = Sqrt (2);
      _1_Sqrt2: Real = 1 / Sqrt (2);
      Ln2:      Real = Ln (2);
      Ln2_2:    Real = Ln (2) / 2;

---

## Code Examples

### Code Block 1

```pascal
Unit System;

Interface

Const

  BELL = $07;
  BS   = $08;
  TAB  = $09;
  LF   = $0A;
  CR   = $0D;
  EOF  = $1A;
  ESC  = $1B;
  DEL  = $7F;

  External0 = $0003;
  Timer0    = $000B;
  External1 = $0013;
  Timer1    = $001B;
  Serial    = $0023;

Type TDeviceWriteProcedure = Procedure;

     TDeviceReadFunction = Function: Char;

     TFileRecord = Record
                     WriteProcedure: TDeviceWriteProcedure;
                     ReadFunction:   TDeviceReadFunction;
                   end;


Var

// Rn registers at absolute address (bank dependent) - Must be declared before 8051 SFR
// Registers ARn SHOULD NOT be used in system unit since system procedures could be called
// from any procedure using any register bank

  AR0:      Byte absolute 0;
  AR1:      Byte absolute 1;
  AR2:      Byte absolute 2;
  AR3:      Byte absolute 3;
  AR4:      Byte absolute 4;
  AR5:      Byte absolute 5;
  AR6:      Byte absolute 6;
  AR7:      Byte absolute 7;

 { 8051 SFR }

  P0:       Byte absolute $80; Volatile;
  SP:       Byte absolute $81; Volatile;
  DPL:      Byte absolute $82;
  DPH:      Byte absolute $83;
  PCON:     Byte absolute $87; Volatile;
  TCON:     Byte absolute $88; Volatile;
  TMOD:     Byte absolute $89; Volatile;
  TL0:      Byte absolute $8A; Volatile;
  TL1:      Byte absolute $8B; Volatile;
  TH0:      Byte absolute $8C; Volatile;
  TH1:      Byte absolute $8D; Volatile;
  P1:       Byte absolute $90; Volatile;
  SCON:     Byte absolute $98; Volatile;
  SBUF:     Byte absolute $99; Volatile;
  P2:       Byte absolute $A0; Volatile;
  IE:       Byte absolute $A8; Volatile;
  P3:       Byte absolute $B0; Volatile;
  IP:       Byte absolute $B8; Volatile;
  PSW:      Byte absolute $D0; Volatile;
  ACC:      Byte absolute $E0;
  B:        Byte absolute $F0;

  DPTR:     Pointer absolute $82;

{ TCON }
  TF1:      Boolean absolute TCON.7;
  TR1:      Boolean absolute TCON.6;
  TF0:      Boolean absolute TCON.5;
  TR0:      Boolean absolute TCON.4;
  IE1:      Boolean absolute TCON.3;
  IT1:      Boolean absolute TCON.2;
  IE0:      Boolean absolute TCON.1;
  IT0:      Boolean absolute TCON.0;

{ SCON }
  SM0:      Boolean absolute SCON.7;
  SM1:      Boolean absolute SCON.6;
  SM2:      Boolean absolute SCON.5;
  REN:      Boolean absolute SCON.4;
  TB8:      Boolean absolute SCON.3;
  RB8:      Boolean absolute SCON.2;
  TI:       Boolean absolute SCON.1;
  RI:       Boolean absolute SCON.0;

{ IE }
  EA:       Boolean absolute IE.7;
  ES:       Boolean absolute IE.4;
  ET1:      Boolean absolute IE.3;
  EX1:      Boolean absolute IE.2;
  ET0:      Boolean absolute IE.1;
  EX0:      Boolean absolute IE.0;

{ P3 }
  RD:       Boolean absolute P3.7;
  WR:       Boolean absolute P3.6;
  T1:       Boolean absolute P3.5;
  T0:       Boolean absolute P3.4;
  INT1:     Boolean absolute P3.3;
  INT0:     Boolean absolute P3.2;
  TXD:      Boolean absolute P3.1;
  RXD:      Boolean absolute P3.0;

{ IP}
  PS:       Boolean absolute IP.4;
  PT1:      Boolean absolute IP.3;
  PX1:      Boolean absolute IP.2;
  PT0:      Boolean absolute IP.1;
  PX0:      Boolean absolute IP.0;

{ PSW }
  CY:       Boolean absolute PSW.7;
  AC:       Boolean absolute PSW.6;
  F0:       Boolean absolute PSW.5;
  RS1:      Boolean absolute PSW.4;
  RS0:      Boolean absolute PSW.3;
  OV:       Boolean absolute PSW.2;
  P:        Boolean absolute PSW.0;

{ Vars declared before MemCODE will not be shown in asm file }

  MemCODE:  Array [$0000..$FFFF] of Byte CODE  absolute $0000;

{ Warning: for addresses above $80 MemDATA can only be accessed by constant array index }
{ otherwise the compiler will generate indirect instruction and IDATA memory will be accessed }

  MemDATA:  Array [  $00..  $FF] of Byte DATA  absolute   $00;

  MemIDATA: Array [  $00..  $FF] of Byte IDATA absolute   $00;
  MemXDATA: Array [$0000..$FFFF] of Byte XDATA absolute $0000;


Var StackStart:       Byte IDATA;  // Used for SP initialization, first byte of stack
                                   // linker will set its address as last variable in
                                   // (I)DATA, it might also be in IDATA segment!
    XDATA_StackStart: Word XDATA;  // Used for XSP and XBP initialization, first byte of
                                   // XDATA stack (linker will set its address)
    R8, R9:           Byte DATA;   // Used for LongInt set 1 and as additional registers,
                                   // R8 and R9 must be first variables in DATA segment
    TempRegister: Byte DATA;       // Used for UsingAny procedures as temporary storage
                                   // during PUSH / POP
    XSP, XBP: Pointer DATA;        // Used for recursion stack and local variables in XDATA
    HeapOrg,
    HeapPtr,
    HeapEnd:     Pointer DATA;     // Used for heap management
    FreeList:    Pointer XDATA;    // Used for heap management, must be in XDATA to avoid
                                   // XDATA nil and to be compatible with TFreeRec in XDATA
                                   // memory
    HeapError:   Procedure DATA;
    RandomSeed:  LongInt DATA;     // Used for random numbers
    CurrentIO:   File DATA;        // Used for file I/O
    SystemIO:    Text DATA;        // Used for Read/Readln/Write/Writeln
    Input:       Text absolute SystemIO;
    Output:      Text absolute SystemIO;
    LastCharacterBuffer: Char;         // Used for sysReadCharFromCurrentDevice
    LastCharacterBufferValid: Boolean; // Used for sysReadCharFromCurrentDevice
    Overflow: Boolean;                 // Used for arithmetic functions to indicate
                                       // overflow

    ResultSign: Boolean;           // Used for signed arithmetics
    TempBool0: Boolean;            // Used for artithmetic functions
    TempBool1: Boolean;            // Used for system procedure Val, artithmetic functions
    TempBool2: Boolean;            // Used for system procedure Val
    TempWord:  Word DATA;          // Used for heap management - sysNewMemory
    TempByte0: Byte DATA;          // Used for long multiplication, long division
    TempByte1: Byte DATA;
    TempByte2: Byte DATA;
    TempByte3: Byte DATA;
    TempByte4: Byte DATA;
    TempByte5: Byte DATA;
    TempByte6: Byte DATA;
    TempByte7: Byte DATA;
    TempByte8: Byte DATA;
    TempByte9: Byte DATA;
    TempByte10: Byte DATA;

Var RealSigns: Byte DATA;
    RealResult: LongInt DATA;
    RealResultCarry: Byte DATA;

Const Pi_2:     Real = Pi / 2;
      Pi_24:    Real = Pi / 24;
      _Pi:      Real = Pi;
      _2Pi:     Real = 2 * Pi;
      _2_Pi:    Real = 2 / Pi;
      _0_5:     Real = 0.5;
      _1:       Real = 1;
      Sqrt2:    Real = Sqrt (2);
      _1_Sqrt2: Real = 1 / Sqrt (2);
      Ln2:      Real = Ln (2);
      Ln2_2:    Real = Ln (2) / 2;

```

