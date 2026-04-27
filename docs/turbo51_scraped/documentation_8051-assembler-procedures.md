# Turbo51 Assembler Procedures | Documentation

**URL:** https://turbo51.com/documentation/8051-assembler-procedures

---

## Table of Contents

- Turbo51 Assembler Procedures
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Here you can see some examples of procedures written entirely in 8051 assembly language. At the end of each procedure Turbo51 adds only RET instruction (or RETI in interrupt procedure). Turbo51 automatically removes RET instruction at the end of procedure if it finds out that it will not be reached. If for some reason RET instruction is not removed and you don't want it you can use the $NoReturn compiler directive inside assembler procedure to prevent generating RET instruction. You can easily pass parameters by value (Turbo51 automatically creates static variables for value storage), by reference (Turbo51 automatically creates static variables for pointer storage) or you can pass values in registers. Procedure's parameters can be accessed as local variables with Procedure.Parameter. See also
assembler statement
.
Program AssemblerProcedures;

{ Useless program just to demostrate assembler procedures }

Const DemoText       = 'Turbo51 assembler procedures demo';
      ZeroTerminated = 'Zero terminated';

      DemoString: String [Length (DemoText)] = DemoText;
      String0:    Array [0 .. Length (ZeroTerminated)] of Char = ZeroTerminated;


Var Number1, Number2, Result: Word;

Procedure Add; Assembler;
Asm
  MOV       A, Number1
  ADD       A, Number2
  MOV       Result, A
  MOV       A, Number1 + 1
  ADDC      A, Number2 + 1
  MOV       Result + 1, A
end;

Procedure Multiply; Assembler;
Asm
  MOV       R2, Number1
  MOV       R3, Number1 + 1
  MOV       R6, Number2
  MOV       R7, Number2 + 1

  MOV       A, R2
  MOV       B, R6
  MUL       AB
  XCH       A, R2
  XCH       A, R7
  XCH       A, B
  XCH       A, R7
  MUL       AB
  ADD       A, R7
  XCH       A, R3
  MOV       B, R6
  MUL       AB
  ADD       A, R3
  MOV       R3, A

  MOV       Result, R2
  MOV       Result + 1, R3
end;


Procedure CalculateXorValue (Num1, Num2: Word); Assembler;
Asm
  MOV       A, Num1
  XRL       A, Num2
  MOV       R2, A
  MOV       A, Num1 + 1
  XRL       A, Num2 + 1
  MOV       R3, A
end;

Procedure SwapWords (Var Num1, Num2: Word); Assembler;
Asm
  MOV       R0, Num1
  MOV       R1, Num2

  MOV       B, @R0
  MOV       A, @R1
  MOV       @R0, A
  MOV       @R1, B

  INC       R0
  INC       R1

  MOV       B, @R0
  MOV       A, @R1
  MOV       @R0, A
  MOV       @R1, B
end;

Procedure WriteString; Assembler;
Asm
//  Code to write string in code with address in DPTR
end;

Procedure WriteZeroTerminatedString; Assembler;
Asm
//  Code to write zero terminated string in code with address in DPTR
end;

Procedure WriteResult; Assembler;
Asm
//  Code to write number in Result variable
end;

begin
  Asm
    MOV       DPTR, #DemoString
    LCALL     WriteString

    MOV       DPTR, #String0
    LCALL     WriteZeroTerminatedString

    MOV       Number1,     #LOW  (200)
    MOV       Number1 + 1, #HIGH (200)
    MOV       Number2,     #LOW  (40)
    MOV       Number2 + 1, #HIGH (40)
    LCALL     Add
    LCALL     WriteResult

    MOV       Number1,     #LOW  (2000)
    MOV       Number1 + 1, #HIGH (2000)
    MOV       Number2,     #LOW  (45)
    MOV       Number2 + 1, #HIGH (45)
    LCALL     Multiply
    LCALL     WriteResult

    MOV       CalculateXorValue.Num1,     #LOW  ($1234)
    MOV       CalculateXorValue.Num1 + 1, #HIGH ($1234)
    MOV       CalculateXorValue.Num2,     #LOW  (10000)
    MOV       CalculateXorValue.Num2 + 1, #HIGH (10000)
    LCALL     CalculateXorValue
    MOV       Result,     R2
    MOV       Result + 1, R3
    LCALL     WriteResult

    MOV       Number1,     #LOW  (2000)
    MOV       Number1 + 1, #HIGH (2000)
    MOV       Number2,     #LOW  (45)
    MOV       Number2 + 1, #HIGH (45)
    MOV       SwapWords.Num1, #Number1
    MOV       SwapWords.Num2, #Number2
    LCALL     SwapWords
    LCALL     WriteResult
  end;
end.
Compiled program looks like this:
; Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa

$REGISTERBANK (0)

_CODE         SEGMENT  CODE
_CONST        SEGMENT  CODE
_DATA         SEGMENT  DATA

              EXTRN    IDATA (StackStart)

; Program AssemblerProcedures;
; 
; { Useless program just to demostrate assembler procedures }
; 
; Const DemoText       = 'Turbo51 assembler procedures demo';

RSEG _CONST

;       ZeroTerminated = 'Zero terminated';
; 
;       DemoString: String [Length (DemoText)] = DemoText;
DemoString:                     DB        33, 'Turbo51 assembler procedures demo'
;       String0:    Array [0 .. Length (ZeroTerminated)] of Char = ZeroTerminated;
String0:                        DB        'Zero terminated', 0
; 
; 
; Var Number1, Number2, Result: Word;

RSEG _DATA

Number1:                        DS       2
Number2:                        DS       2
Result:                         DS       2

; 
; Procedure Add; Assembler;


RSEG _CODE

USING 0

Add:
; Asm
;   MOV       A, Number1
              MOV       A, Number1

;   ADD       A, Number2
              ADD       A, Number2

;   MOV       Result, A
              MOV       Result, A

;   MOV       A, Number1 + 1
              MOV       A, Number1+1

;   ADDC      A, Number2 + 1
              ADDC      A, Number2+1

;   MOV       Result + 1, A
              MOV       Result+1, A

; end;
              RET

; 
; Procedure Multiply; Assembler;

Multiply:
; Asm
;   MOV       R2, Number1
              MOV       R2, Number1

;   MOV       R3, Number1 + 1
              MOV       R3, Number1+1

;   MOV       R6, Number2
              MOV       R6, Number2

;   MOV       R7, Number2 + 1
              MOV       R7, Number2+1

; 
;   MOV       A, R2
              MOV       A, R2

;   MOV       B, R6
              MOV       B, R6

;   MUL       AB
              MUL       AB

;   XCH       A, R2
              XCH       A, R2

;   XCH       A, R7
              XCH       A, R7

;   XCH       A, B
              XCH       A, B

;   XCH       A, R7
              XCH       A, R7

;   MUL       AB
              MUL       AB

;   ADD       A, R7
              ADD       A, R7

;   XCH       A, R3
              XCH       A, R3

;   MOV       B, R6
              MOV       B, R6

;   MUL       AB
              MUL       AB

;   ADD       A, R3
              ADD       A, R3

;   MOV       R3, A
              MOV       R3, A

; 
;   MOV       Result, R2
              MOV       Result, R2

;   MOV       Result + 1, R3
              MOV       Result+1, R3

; end;
              RET

; 
; 
; Procedure CalculateXorValue (Num1, Num2: Word); Assembler;

RSEG _DATA

Num1:                           DS       2
Num2:                           DS       2


RSEG _CODE

CalculateXorValue:
; Asm
;   MOV       A, Num1
              MOV       A, Num1

;   XRL       A, Num2
              XRL       A, Num2

;   MOV       R2, A
              MOV       R2, A

;   MOV       A, Num1 + 1
              MOV       A, Num1+1

;   XRL       A, Num2 + 1
              XRL       A, Num2+1

;   MOV       R3, A
              MOV       R3, A

; end;
              RET

; 
; Procedure SwapWords (Var Num1, Num2: Word); Assembler;

RSEG _DATA

Num1:                           DS       1
Num2:                           DS       1


RSEG _CODE

SwapWords:
; Asm
;   MOV       R0, Num1
              MOV       R0, Num1

;   MOV       R1, Num2
              MOV       R1, Num2

; 
;   MOV       B, @R0
              MOV       B, @R0

;   MOV       A, @R1
              MOV       A, @R1

;   MOV       @R0, A
              MOV       @R0, A

;   MOV       @R1, B
              MOV       @R1, B

; 
;   INC       R0
              INC       R0

;   INC       R1
              INC       R1

; 
;   MOV       B, @R0
              MOV       B, @R0

;   MOV       A, @R1
              MOV       A, @R1

;   MOV       @R0, A
              MOV       @R0, A

;   MOV       @R1, B
              MOV       @R1, B

; end;
              RET

; 
; Procedure WriteString; Assembler;

WriteString:
; Asm
; //  Code to write string in code with address in DPTR
; end;
              RET

; 
; Procedure WriteZeroTerminatedString; Assembler;

WriteZeroTerminatedString:
; Asm
; //  Code to write zero terminated string in code with address in DPTR
; end;
              RET

; 
; Procedure WriteResult; Assembler;

WriteResult:
; Asm
; //  Code to write number in Result variable
; end;
              RET


CSEG AT $0000

AssemblerProcedures:
; 
; begin
              MOV       SP, #StackStart-1

;   Asm
;     MOV       DPTR, #DemoString
              MOV       DPTR, #DemoString

;     LCALL     WriteString
              LCALL     WriteString

; 
;     MOV       DPTR, #String0
              MOV       DPTR, #String0

;     LCALL     WriteZeroTerminatedString
              LCALL     WriteZeroTerminatedString

; 
;     MOV       Number1,     #LOW  (200)
              MOV       Number1, #$C8

;     MOV       Number1 + 1, #HIGH (200)
              MOV       Number1+1, #0

;     MOV       Number2,     #LOW  (40)
              MOV       Number2, #$28

;     MOV       Number2 + 1, #HIGH (40)
              MOV       Number2+1, #0

;     LCALL     Add
              LCALL     Add

;     LCALL     WriteResult
              LCALL     WriteResult

; 
;     MOV       Number1,     #LOW  (2000)
              MOV       Number1, #$D0

;     MOV       Number1 + 1, #HIGH (2000)
              MOV       Number1+1, #7

;     MOV       Number2,     #LOW  (45)
              MOV       Number2, #$2D

;     MOV       Number2 + 1, #HIGH (45)
              MOV       Number2+1, #0

;     LCALL     Multiply
              LCALL     Multiply

;     LCALL     WriteResult
              LCALL     WriteResult

; 
;     MOV       CalculateXorValue.Num1,     #LOW  ($1234)
              MOV       CalculateXorValue.Num1, #$34

;     MOV       CalculateXorValue.Num1 + 1, #HIGH ($1234)
              MOV       CalculateXorValue.Num1+1, #$12

;     MOV       CalculateXorValue.Num2,     #LOW  (10000)
              MOV       CalculateXorValue.Num2, #$10

;     MOV       CalculateXorValue.Num2 + 1, #HIGH (10000)
              MOV       CalculateXorValue.Num2+1, #$27

;     LCALL     CalculateXorValue
              LCALL     CalculateXorValue

;     MOV       Result,     R2
              MOV       Result, R2

;     MOV       Result + 1, R3
              MOV       Result+1, R3

;     LCALL     WriteResult
              LCALL     WriteResult

; 
;     MOV       Number1,     #LOW  (2000)
              MOV       Number1, #$D0

;     MOV       Number1 + 1, #HIGH (2000)
              MOV       Number1+1, #7

;     MOV       Number2,     #LOW  (45)
              MOV       Number2, #$2D

;     MOV       Number2 + 1, #HIGH (45)
              MOV       Number2+1, #0

;     MOV       SwapWords.Num1, #Number1
              MOV       SwapWords.Num1, #Number1

;     MOV       SwapWords.Num2, #Number2
              MOV       SwapWords.Num2, #Number2

;     LCALL     SwapWords
              LCALL     SwapWords

;     LCALL     WriteResult
              LCALL     WriteResult

;   end;
; end.
L_0061:
              SJMP      L_0061

RSEG _CONST


              END

---

## Code Examples

### Code Block 1

```pascal
Program AssemblerProcedures;

{ Useless program just to demostrate assembler procedures }

Const DemoText       = 'Turbo51 assembler procedures demo';
      ZeroTerminated = 'Zero terminated';

      DemoString: String [Length (DemoText)] = DemoText;
      String0:    Array [0 .. Length (ZeroTerminated)] of Char = ZeroTerminated;


Var Number1, Number2, Result: Word;

Procedure Add; Assembler;
Asm
  MOV       A, Number1
  ADD       A, Number2
  MOV       Result, A
  MOV       A, Number1 + 1
  ADDC      A, Number2 + 1
  MOV       Result + 1, A
end;

Procedure Multiply; Assembler;
Asm
  MOV       R2, Number1
  MOV       R3, Number1 + 1
  MOV       R6, Number2
  MOV       R7, Number2 + 1

  MOV       A, R2
  MOV       B, R6
  MUL       AB
  XCH       A, R2
  XCH       A, R7
  XCH       A, B
  XCH       A, R7
  MUL       AB
  ADD       A, R7
  XCH       A, R3
  MOV       B, R6
  MUL       AB
  ADD       A, R3
  MOV       R3, A

  MOV       Result, R2
  MOV       Result + 1, R3
end;


Procedure CalculateXorValue (Num1, Num2: Word); Assembler;
Asm
  MOV       A, Num1
  XRL       A, Num2
  MOV       R2, A
  MOV       A, Num1 + 1
  XRL       A, Num2 + 1
  MOV       R3, A
end;

Procedure SwapWords (Var Num1, Num2: Word); Assembler;
Asm
  MOV       R0, Num1
  MOV       R1, Num2

  MOV       B, @R0
  MOV       A, @R1
  MOV       @R0, A
  MOV       @R1, B

  INC       R0
  INC       R1

  MOV       B, @R0
  MOV       A, @R1
  MOV       @R0, A
  MOV       @R1, B
end;

Procedure WriteString; Assembler;
Asm
//  Code to write string in code with address in DPTR
end;

Procedure WriteZeroTerminatedString; Assembler;
Asm
//  Code to write zero terminated string in code with address in DPTR
end;

Procedure WriteResult; Assembler;
Asm
//  Code to write number in Result variable
end;

begin
  Asm
    MOV       DPTR, #DemoString
    LCALL     WriteString

    MOV       DPTR, #String0
    LCALL     WriteZeroTerminatedString

    MOV       Number1,     #LOW  (200)
    MOV       Number1 + 1, #HIGH (200)
    MOV       Number2,     #LOW  (40)
    MOV       Number2 + 1, #HIGH (40)
    LCALL     Add
    LCALL     WriteResult

    MOV       Number1,     #LOW  (2000)
    MOV       Number1 + 1, #HIGH (2000)
    MOV       Number2,     #LOW  (45)
    MOV       Number2 + 1, #HIGH (45)
    LCALL     Multiply
    LCALL     WriteResult

    MOV       CalculateXorValue.Num1,     #LOW  ($1234)
    MOV       CalculateXorValue.Num1 + 1, #HIGH ($1234)
    MOV       CalculateXorValue.Num2,     #LOW  (10000)
    MOV       CalculateXorValue.Num2 + 1, #HIGH (10000)
    LCALL     CalculateXorValue
    MOV       Result,     R2
    MOV       Result + 1, R3
    LCALL     WriteResult

    MOV       Number1,     #LOW  (2000)
    MOV       Number1 + 1, #HIGH (2000)
    MOV       Number2,     #LOW  (45)
    MOV       Number2 + 1, #HIGH (45)
    MOV       SwapWords.Num1, #Number1
    MOV       SwapWords.Num2, #Number2
    LCALL     SwapWords
    LCALL     WriteResult
  end;
end.

```

### Code Block 2

```pascal
; Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa

$REGISTERBANK (0)

_CODE         SEGMENT  CODE
_CONST        SEGMENT  CODE
_DATA         SEGMENT  DATA

              EXTRN    IDATA (StackStart)

; Program AssemblerProcedures;
; 
; { Useless program just to demostrate assembler procedures }
; 
; Const DemoText       = 'Turbo51 assembler procedures demo';

RSEG _CONST

;       ZeroTerminated = 'Zero terminated';
; 
;       DemoString: String [Length (DemoText)] = DemoText;
DemoString:                     DB        33, 'Turbo51 assembler procedures demo'
;       String0:    Array [0 .. Length (ZeroTerminated)] of Char = ZeroTerminated;
String0:                        DB        'Zero terminated', 0
; 
; 
; Var Number1, Number2, Result: Word;

RSEG _DATA

Number1:                        DS       2
Number2:                        DS       2
Result:                         DS       2

; 
; Procedure Add; Assembler;


RSEG _CODE

USING 0

Add:
; Asm
;   MOV       A, Number1
              MOV       A, Number1

;   ADD       A, Number2
              ADD       A, Number2

;   MOV       Result, A
              MOV       Result, A

;   MOV       A, Number1 + 1
              MOV       A, Number1+1

;   ADDC      A, Number2 + 1
              ADDC      A, Number2+1

;   MOV       Result + 1, A
              MOV       Result+1, A

; end;
              RET

; 
; Procedure Multiply; Assembler;

Multiply:
; Asm
;   MOV       R2, Number1
              MOV       R2, Number1

;   MOV       R3, Number1 + 1
              MOV       R3, Number1+1

;   MOV       R6, Number2
              MOV       R6, Number2

;   MOV       R7, Number2 + 1
              MOV       R7, Number2+1

; 
;   MOV       A, R2
              MOV       A, R2

;   MOV       B, R6
              MOV       B, R6

;   MUL       AB
              MUL       AB

;   XCH       A, R2
              XCH       A, R2

;   XCH       A, R7
              XCH       A, R7

;   XCH       A, B
              XCH       A, B

;   XCH       A, R7
              XCH       A, R7

;   MUL       AB
              MUL       AB

;   ADD       A, R7
              ADD       A, R7

;   XCH       A, R3
              XCH       A, R3

;   MOV       B, R6
              MOV       B, R6

;   MUL       AB
              MUL       AB

;   ADD       A, R3
              ADD       A, R3

;   MOV       R3, A
              MOV       R3, A

; 
;   MOV       Result, R2
              MOV       Result, R2

;   MOV       Result + 1, R3
              MOV       Result+1, R3

; end;
              RET

; 
; 
; Procedure CalculateXorValue (Num1, Num2: Word); Assembler;

RSEG _DATA

Num1:                           DS       2
Num2:                           DS       2


RSEG _CODE

CalculateXorValue:
; Asm
;   MOV       A, Num1
              MOV       A, Num1

;   XRL       A, Num2
              XRL       A, Num2

;   MOV       R2, A
              MOV       R2, A

;   MOV       A, Num1 + 1
              MOV       A, Num1+1

;   XRL       A, Num2 + 1
              XRL       A, Num2+1

;   MOV       R3, A
              MOV       R3, A

; end;
              RET

; 
; Procedure SwapWords (Var Num1, Num2: Word); Assembler;

RSEG _DATA

Num1:                           DS       1
Num2:                           DS       1


RSEG _CODE

SwapWords:
; Asm
;   MOV       R0, Num1
              MOV       R0, Num1

;   MOV       R1, Num2
              MOV       R1, Num2

; 
;   MOV       B, @R0
              MOV       B, @R0

;   MOV       A, @R1
              MOV       A, @R1

;   MOV       @R0, A
              MOV       @R0, A

;   MOV       @R1, B
              MOV       @R1, B

; 
;   INC       R0
              INC       R0

;   INC       R1
              INC       R1

; 
;   MOV       B, @R0
              MOV       B, @R0

;   MOV       A, @R1
              MOV       A, @R1

;   MOV       @R0, A
              MOV       @R0, A

;   MOV       @R1, B
              MOV       @R1, B

; end;
              RET

; 
; Procedure WriteString; Assembler;

WriteString:
; Asm
; //  Code to write string in code with address in DPTR
; end;
              RET

; 
; Procedure WriteZeroTerminatedString; Assembler;

WriteZeroTerminatedString:
; Asm
; //  Code to write zero terminated string in code with address in DPTR
; end;
              RET

; 
; Procedure WriteResult; Assembler;

WriteResult:
; Asm
; //  Code to write number in Result variable
; end;
              RET


CSEG AT $0000

AssemblerProcedures:
; 
; begin
              MOV       SP, #StackStart-1

;   Asm
;     MOV       DPTR, #DemoString
              MOV       DPTR, #DemoString

;     LCALL     WriteString
              LCALL     WriteString

; 
;     MOV       DPTR, #String0
              MOV       DPTR, #String0

;     LCALL     WriteZeroTerminatedString
              LCALL     WriteZeroTerminatedString

; 
;     MOV       Number1,     #LOW  (200)
              MOV       Number1, #$C8

;     MOV       Number1 + 1, #HIGH (200)
              MOV       Number1+1, #0

;     MOV       Number2,     #LOW  (40)
              MOV       Number2, #$28

;     MOV       Number2 + 1, #HIGH (40)
              MOV       Number2+1, #0

;     LCALL     Add
              LCALL     Add

;     LCALL     WriteResult
              LCALL     WriteResult

; 
;     MOV       Number1,     #LOW  (2000)
              MOV       Number1, #$D0

;     MOV       Number1 + 1, #HIGH (2000)
              MOV       Number1+1, #7

;     MOV       Number2,     #LOW  (45)
              MOV       Number2, #$2D

;     MOV       Number2 + 1, #HIGH (45)
              MOV       Number2+1, #0

;     LCALL     Multiply
              LCALL     Multiply

;     LCALL     WriteResult
              LCALL     WriteResult

; 
;     MOV       CalculateXorValue.Num1,     #LOW  ($1234)
              MOV       CalculateXorValue.Num1, #$34

;     MOV       CalculateXorValue.Num1 + 1, #HIGH ($1234)
              MOV       CalculateXorValue.Num1+1, #$12

;     MOV       CalculateXorValue.Num2,     #LOW  (10000)
              MOV       CalculateXorValue.Num2, #$10

;     MOV       CalculateXorValue.Num2 + 1, #HIGH (10000)
              MOV       CalculateXorValue.Num2+1, #$27

;     LCALL     CalculateXorValue
              LCALL     CalculateXorValue

;     MOV       Result,     R2
              MOV       Result, R2

;     MOV       Result + 1, R3
              MOV       Result+1, R3

;     LCALL     WriteResult
              LCALL     WriteResult

; 
;     MOV       Number1,     #LOW  (2000)
              MOV       Number1, #$D0

;     MOV       Number1 + 1, #HIGH (2000)
              MOV       Number1+1, #7

;     MOV       Number2,     #LOW  (45)
              MOV       Number2, #$2D

;     MOV       Number2 + 1, #HIGH (45)
              MOV       Number2+1, #0

;     MOV       SwapWords.Num1, #Number1
              MOV       SwapWords.Num1, #Number1

;     MOV       SwapWords.Num2, #Number2
              MOV       SwapWords.Num2, #Number2

;     LCALL     SwapWords
              LCALL     SwapWords

;     LCALL     WriteResult
              LCALL     WriteResult

;   end;
; end.
L_0061:
              SJMP      L_0061

RSEG _CONST


              END


```

