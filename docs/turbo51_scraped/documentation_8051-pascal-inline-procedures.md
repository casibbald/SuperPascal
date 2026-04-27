# Turo51 Inline Procedures | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-inline-procedures

---

## Table of Contents

- Turo51 Inline Procedures
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Procedures (and functions) that are declared with the Inline directive are copied to the places where they are called. This has the effect that there is no actual procedure (or function) call, the code of the procedure is just copied to where the procedure is needed, this results in faster execution speed if the procedure or function is used a lot but but usually means also larger code size. You can override this behaviour with the $InlineCode directive. When set to Off (default is On) the compiler will generate normal calls to inline procedures.
Example:
{
    This file is part of the Turbo51 code examples.
    Copyright (C) 2008 by Igor Funa

    http://turbo51.com/

    This file is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
}

Program Example4;

{ Usless program just to demonstrate sets and inline procedures/functions }

Type
  TFlag = (fl0, fl1, fl2, fl3, fl4, fl5, fl6);
  TFlagsSet = Set of TFlag;

  TVariantRecord = Record
                     Case Byte of
                       0: (L: LongInt);
                       1: (W0, W1: Word);
                       2: (B0, B1, B2, B3: Byte);
                       3: (DataWord: Word; LocalFlags: Set of 0..7; Flags: TFlagsSet);
                       4: (IndividualBits: Set of 0..31);
                       5: (Ch0, Ch1, Ch2, Ch3: Char);
                   end;

Const
   InitialFlags = [fl0, fl4, fl5];
   TempFlags    = [fl0, fl1, fl3];

Var
  WatchdogClock: Boolean absolute P0.4;

  GlobalFlags: TFlagsSet;
  DataRecord1,
  DataRecord2: TVariantRecord;
  Character: Char;
  B1, B2: Byte;

Function UpcaseChar (Ch: Char): Char;
{$I InlineChar.inc }

Function InlineUpcaseChar (Ch: Char): Char; Inline;
{$I InlineChar.inc }

Procedure RestartWatchdog; Inline; Assembler;
Asm
  CPL  WatchdogClock;
end;

Procedure Multiply (Var Factor: Byte);
begin
  Factor := Factor * 10;
  If Factor >= 100 then Factor := 0;
end;

Procedure InlineMultiply (Var Factor: Byte); Inline;
begin
  Factor := Factor * 10;
  If Factor >= 100 then Factor := 0;
end;

begin
  GlobalFlags := InitialFlags;

  Include (GlobalFlags, fl4);
  Exclude (GlobalFlags, fl5);
  Change  (GlobalFlags, fl6);

  RestartWatchdog;

  DataRecord1.L := $12345678;
  With DataRecord2 do
    begin
      DataWord   := DataRecord1.W0 + DataRecord1.W1;
      LocalFlags := [3, 5, 6];
      Flags      := GlobalFlags;
    end;

  RestartWatchdog;

  Case fl6 in DataRecord2.Flags of
    True: DataRecord2.Flags := DataRecord2.Flags * TempFlags + [fl2, fl3];
    else  DataRecord2.Flags := TempFlags;
  end;

  RestartWatchdog;

  If 0 in DataRecord2.IndividualBits then With DataRecord2 do
    begin
      Include (IndividualBits, 4);
      Exclude (IndividualBits, 15);
      Change  (IndividualBits, 31);
    end;

{ Call to function UpcaseChar }

  Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
  DataRecord1.Ch0 := UpcaseChar (Character);

{ Inline function InlineUpcaseChar }

  Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
  DataRecord1.Ch1 := InlineUpcaseChar (Character);

{ Call to procedure Multiply }

  Multiply (DataRecord1.B1);

{ Inline procedure InlineMultiply }

  InlineMultiply (DataRecord1.B1);

{$InlineCode Off }

{ Normal call to Inline function InlineUpcaseChar }

  Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
  DataRecord1.Ch1 := InlineUpcaseChar (Character);

{ Normal call to Inline procedure InlineMultiply }

  InlineMultiply (DataRecord1.B1);
end.
InlineChar.inc:
begin
  Result := Char (Byte (Ch) and $DF);
end;
Compiled program above looks like this:
; Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa

$REGISTERBANK (0)

_CODE         SEGMENT  CODE
_DATA         SEGMENT  DATA

              EXTRN    IDATA (StackStart)

              EXTRN    CODE  (sysRandomWord)

; {
;     This file is part of the Turbo51 code examples.
;     Copyright (C) 2008 by Igor Funa
; 
;     http://turbo51.com/
; 
;     This file is distributed in the hope that it will be useful,
;     but WITHOUT ANY WARRANTY; without even the implied warranty of
;     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
; }
; 
; Program Example4;
; 
; { Usless program just to demonstrate sets and inline procedures/functions }
; 
; Type
;   TFlag = (fl0, fl1, fl2, fl3, fl4, fl5, fl6);
;   TFlagsSet = Set of TFlag;
; 
;   TVariantRecord = Record
;                      Case Byte of
;                        0: (L: LongInt);
;                        1: (W0, W1: Word);
;                        2: (B0, B1, B2, B3: Byte);
;                        3: (DataWord: Word; LocalFlags: Set of 0..7; Flags: TFlagsSet);
;                        4: (IndividualBits: Set of 0..31);
;                        5: (Ch0, Ch1, Ch2, Ch3: Char);
;                    end;
; 
; Const

RSEG _CONST

;    InitialFlags = [fl0, fl4, fl5];
;    TempFlags    = [fl0, fl1, fl3];
; 
; Var
;   WatchdogClock: Boolean absolute P0.4;
WatchdogClock                   BIT      P0.4

; 
;   GlobalFlags: TFlagsSet;

RSEG _DATA

GlobalFlags:                    DS       1

;   DataRecord1,
;   DataRecord2: TVariantRecord;
DataRecord1:                    DS       4
DataRecord2:                    DS       4

;   Character: Char;
Character:                      DS       1

;   B1, B2: Byte;
; 
; Function UpcaseChar (Ch: Char): Char;
Ch:                             DS       1


RSEG _CODE

USING 0

UpcaseChar:
; {$I InlineChar.inc }
; begin
;   Result := Char (Byte (Ch) and $DF);
              MOV       A, Ch
              ANL       A, #$DF

; end;
              RET

; 
; Function InlineUpcaseChar (Ch: Char): Char; Inline;

RSEG _DATA

Ch:                             DS       1


RSEG _CODE

InlineUpcaseChar:
; {$I InlineChar.inc }
; begin
;   Result := Char (Byte (Ch) and $DF);
              MOV       A, Ch
              ANL       A, #$DF

; end;
              RET

; 
; Procedure RestartWatchdog; Inline; Assembler;
; Asm
;   CPL  WatchdogClock;
; end;
; 
; Procedure Multiply (Var Factor: Byte);

RSEG _DATA

Factor:                         DS       1


RSEG _CODE

Multiply:
; begin
;   Factor := Factor * 10;
              MOV       R0, Factor
              MOV       A, @R0
              MOV       B, #$0A
              MUL       AB
              MOV       @R0, A

;   If Factor >= 100 then Factor := 0;
              ADD       A, #-100
              JNC       L_00C7
              MOV       @R0, #0
L_00C7:

; end;
              RET

; 
; Procedure InlineMultiply (Var Factor: Byte); Inline;

RSEG _DATA

Factor:                         DS       1


RSEG _CODE

InlineMultiply:
; begin
;   Factor := Factor * 10;
              MOV       R0, Factor
              MOV       A, @R0
              MOV       B, #$0A
              MUL       AB
              MOV       @R0, A

;   If Factor >= 100 then Factor := 0;
              ADD       A, #-100
              JNC       L_00D6
              MOV       @R0, #0
L_00D6:

; end;
              RET


CSEG AT $0000

Example4:
; 
; begin
              MOV       SP, #StackStart-1

;   GlobalFlags := InitialFlags;
              MOV       GlobalFlags, #$31

; 
;   Include (GlobalFlags, fl4);
              ORL       GlobalFlags, #$10

;   Exclude (GlobalFlags, fl5);
              ANL       GlobalFlags, #$DF

;   Change  (GlobalFlags, fl6);
              XRL       GlobalFlags, #$40

; 
;   RestartWatchdog;
;   CPL  WatchdogClock;
              CPL       P0.4

; 
;   DataRecord1.L := $12345678;
              MOV       DataRecord1, #$78
              MOV       DataRecord1+1, #$56
              MOV       DataRecord1+2, #$34
              MOV       DataRecord1+3, #$12

;   With DataRecord2 do
;     begin
;       DataWord   := DataRecord1.W0 + DataRecord1.W1;
              MOV       A, DataRecord1
              ADD       A, DataRecord1+2
              MOV       DataRecord2, A
              MOV       A, DataRecord1+1
              ADDC      A, DataRecord1+3
              MOV       DataRecord2+1, A

;       LocalFlags := [3, 5, 6];
              MOV       DataRecord2+2, #$68

;       Flags      := GlobalFlags;
              MOV       DataRecord2+3, GlobalFlags

;     end;
; 
;   RestartWatchdog;
;   CPL  WatchdogClock;
              CPL       P0.4

; 
;   Case fl6 in DataRecord2.Flags of
              MOV       A, DataRecord2+3

;     True: DataRecord2.Flags := DataRecord2.Flags * TempFlags + [fl2, fl3];
              JNB       ACC.6, L_0040
              MOV       A, DataRecord2+3
              ANL       A, #$0B
              ORL       A, #$0C
              MOV       DataRecord2+3, A
              SJMP      L_0043
L_0040:

;     else  DataRecord2.Flags := TempFlags;
              MOV       DataRecord2+3, #$0B
L_0043:

;   end;
; 
;   RestartWatchdog;
;   CPL  WatchdogClock;
              CPL       P0.4

; 
;   If 0 in DataRecord2.IndividualBits then With DataRecord2 do
              MOV       A, DataRecord2
              JNB       ACC.0, L_0053

;     begin
;       Include (IndividualBits, 4);
              ORL       DataRecord2, #$10

;       Exclude (IndividualBits, 15);
              ANL       DataRecord2+1, #$7F

;       Change  (IndividualBits, 31);
              XRL       DataRecord2+3, #$80

;     end;
L_0053:
; 
; { Call to function UpcaseChar }
; 
;   Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
              MOV       R2, #LOW  ($001A)
              MOV       R3, #HIGH ($001A)
              LCALL     sysRandomWord
              MOV       A, R2
              ADD       A, #LOW  ($0061)
              MOV       Character, A

;   DataRecord1.Ch0 := UpcaseChar (Character);
              MOV       Ch, Character
              LCALL     UpcaseChar
              MOV       DataRecord1, A

; 
; { Inline function InlineUpcaseChar }
; 
;   Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
              MOV       R2, #LOW  ($001A)
              MOV       R3, #HIGH ($001A)
              LCALL     sysRandomWord
              MOV       A, R2
              ADD       A, #LOW  ($0061)
              MOV       Character, A

;   DataRecord1.Ch1 := InlineUpcaseChar (Character);
              MOV       Ch, Character

;   Result := Char (Byte (Ch) and $DF);
              MOV       A, Ch
              ANL       A, #$DF

              MOV       DataRecord1+1, A
; 
; { Call to procedure Multiply }
; 
;   Multiply (DataRecord1.B1);
              MOV       Factor, #DataRecord1+1
              LCALL     Multiply

; 
; { Inline procedure InlineMultiply }
; 
;   InlineMultiply (DataRecord1.B1);
              MOV       Factor, #DataRecord1+1

;   Factor := Factor * 10;
              MOV       R0, Factor
              MOV       A, @R0
              MOV       B, #$0A
              MUL       AB
              MOV       @R0, A

;   If Factor >= 100 then Factor := 0;
              ADD       A, #-100
              JNC       L_0093
              MOV       @R0, #0
L_0093:

; 
; {$InlineCode Off }
; 
; { Normal call to Inline function InlineUpcaseChar }
; 
;   Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
              MOV       R2, #LOW  ($001A)
              MOV       R3, #HIGH ($001A)
              LCALL     sysRandomWord
              MOV       A, R2
              ADD       A, #LOW  ($0061)
              MOV       Character, A

;   DataRecord1.Ch1 := InlineUpcaseChar (Character);
              MOV       Ch, Character
              LCALL     InlineUpcaseChar
              MOV       DataRecord1+1, A

; 
; { Normal call to Inline procedure InlineMultiply }
; 
;   InlineMultiply (DataRecord1.B1);
              MOV       Factor, #DataRecord1+1
              LCALL     InlineMultiply

; end.
L_00AD:
              SJMP      L_00AD

RSEG _CONST


              END

---

## Code Examples

### Code Block 1

```pascal
{
    This file is part of the Turbo51 code examples.
    Copyright (C) 2008 by Igor Funa

    http://turbo51.com/

    This file is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
}

Program Example4;

{ Usless program just to demonstrate sets and inline procedures/functions }

Type
  TFlag = (fl0, fl1, fl2, fl3, fl4, fl5, fl6);
  TFlagsSet = Set of TFlag;

  TVariantRecord = Record
                     Case Byte of
                       0: (L: LongInt);
                       1: (W0, W1: Word);
                       2: (B0, B1, B2, B3: Byte);
                       3: (DataWord: Word; LocalFlags: Set of 0..7; Flags: TFlagsSet);
                       4: (IndividualBits: Set of 0..31);
                       5: (Ch0, Ch1, Ch2, Ch3: Char);
                   end;

Const
   InitialFlags = [fl0, fl4, fl5];
   TempFlags    = [fl0, fl1, fl3];

Var
  WatchdogClock: Boolean absolute P0.4;

  GlobalFlags: TFlagsSet;
  DataRecord1,
  DataRecord2: TVariantRecord;
  Character: Char;
  B1, B2: Byte;

Function UpcaseChar (Ch: Char): Char;
{$I InlineChar.inc }

Function InlineUpcaseChar (Ch: Char): Char; Inline;
{$I InlineChar.inc }

Procedure RestartWatchdog; Inline; Assembler;
Asm
  CPL  WatchdogClock;
end;

Procedure Multiply (Var Factor: Byte);
begin
  Factor := Factor * 10;
  If Factor >= 100 then Factor := 0;
end;

Procedure InlineMultiply (Var Factor: Byte); Inline;
begin
  Factor := Factor * 10;
  If Factor >= 100 then Factor := 0;
end;

begin
  GlobalFlags := InitialFlags;

  Include (GlobalFlags, fl4);
  Exclude (GlobalFlags, fl5);
  Change  (GlobalFlags, fl6);

  RestartWatchdog;

  DataRecord1.L := $12345678;
  With DataRecord2 do
    begin
      DataWord   := DataRecord1.W0 + DataRecord1.W1;
      LocalFlags := [3, 5, 6];
      Flags      := GlobalFlags;
    end;

  RestartWatchdog;

  Case fl6 in DataRecord2.Flags of
    True: DataRecord2.Flags := DataRecord2.Flags * TempFlags + [fl2, fl3];
    else  DataRecord2.Flags := TempFlags;
  end;

  RestartWatchdog;

  If 0 in DataRecord2.IndividualBits then With DataRecord2 do
    begin
      Include (IndividualBits, 4);
      Exclude (IndividualBits, 15);
      Change  (IndividualBits, 31);
    end;

{ Call to function UpcaseChar }

  Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
  DataRecord1.Ch0 := UpcaseChar (Character);

{ Inline function InlineUpcaseChar }

  Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
  DataRecord1.Ch1 := InlineUpcaseChar (Character);

{ Call to procedure Multiply }

  Multiply (DataRecord1.B1);

{ Inline procedure InlineMultiply }

  InlineMultiply (DataRecord1.B1);

{$InlineCode Off }

{ Normal call to Inline function InlineUpcaseChar }

  Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
  DataRecord1.Ch1 := InlineUpcaseChar (Character);

{ Normal call to Inline procedure InlineMultiply }

  InlineMultiply (DataRecord1.B1);
end.

```

### Code Block 2

```pascal
begin
  Result := Char (Byte (Ch) and $DF);
end;

```

### Code Block 3

```pascal
; Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa

$REGISTERBANK (0)

_CODE         SEGMENT  CODE
_DATA         SEGMENT  DATA

              EXTRN    IDATA (StackStart)

              EXTRN    CODE  (sysRandomWord)

; {
;     This file is part of the Turbo51 code examples.
;     Copyright (C) 2008 by Igor Funa
; 
;     http://turbo51.com/
; 
;     This file is distributed in the hope that it will be useful,
;     but WITHOUT ANY WARRANTY; without even the implied warranty of
;     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
; }
; 
; Program Example4;
; 
; { Usless program just to demonstrate sets and inline procedures/functions }
; 
; Type
;   TFlag = (fl0, fl1, fl2, fl3, fl4, fl5, fl6);
;   TFlagsSet = Set of TFlag;
; 
;   TVariantRecord = Record
;                      Case Byte of
;                        0: (L: LongInt);
;                        1: (W0, W1: Word);
;                        2: (B0, B1, B2, B3: Byte);
;                        3: (DataWord: Word; LocalFlags: Set of 0..7; Flags: TFlagsSet);
;                        4: (IndividualBits: Set of 0..31);
;                        5: (Ch0, Ch1, Ch2, Ch3: Char);
;                    end;
; 
; Const

RSEG _CONST

;    InitialFlags = [fl0, fl4, fl5];
;    TempFlags    = [fl0, fl1, fl3];
; 
; Var
;   WatchdogClock: Boolean absolute P0.4;
WatchdogClock                   BIT      P0.4

; 
;   GlobalFlags: TFlagsSet;

RSEG _DATA

GlobalFlags:                    DS       1

;   DataRecord1,
;   DataRecord2: TVariantRecord;
DataRecord1:                    DS       4
DataRecord2:                    DS       4

;   Character: Char;
Character:                      DS       1

;   B1, B2: Byte;
; 
; Function UpcaseChar (Ch: Char): Char;
Ch:                             DS       1


RSEG _CODE

USING 0

UpcaseChar:
; {$I InlineChar.inc }
; begin
;   Result := Char (Byte (Ch) and $DF);
              MOV       A, Ch
              ANL       A, #$DF

; end;
              RET

; 
; Function InlineUpcaseChar (Ch: Char): Char; Inline;

RSEG _DATA

Ch:                             DS       1


RSEG _CODE

InlineUpcaseChar:
; {$I InlineChar.inc }
; begin
;   Result := Char (Byte (Ch) and $DF);
              MOV       A, Ch
              ANL       A, #$DF

; end;
              RET

; 
; Procedure RestartWatchdog; Inline; Assembler;
; Asm
;   CPL  WatchdogClock;
; end;
; 
; Procedure Multiply (Var Factor: Byte);

RSEG _DATA

Factor:                         DS       1


RSEG _CODE

Multiply:
; begin
;   Factor := Factor * 10;
              MOV       R0, Factor
              MOV       A, @R0
              MOV       B, #$0A
              MUL       AB
              MOV       @R0, A

;   If Factor >= 100 then Factor := 0;
              ADD       A, #-100
              JNC       L_00C7
              MOV       @R0, #0
L_00C7:

; end;
              RET

; 
; Procedure InlineMultiply (Var Factor: Byte); Inline;

RSEG _DATA

Factor:                         DS       1


RSEG _CODE

InlineMultiply:
; begin
;   Factor := Factor * 10;
              MOV       R0, Factor
              MOV       A, @R0
              MOV       B, #$0A
              MUL       AB
              MOV       @R0, A

;   If Factor >= 100 then Factor := 0;
              ADD       A, #-100
              JNC       L_00D6
              MOV       @R0, #0
L_00D6:

; end;
              RET


CSEG AT $0000

Example4:
; 
; begin
              MOV       SP, #StackStart-1

;   GlobalFlags := InitialFlags;
              MOV       GlobalFlags, #$31

; 
;   Include (GlobalFlags, fl4);
              ORL       GlobalFlags, #$10

;   Exclude (GlobalFlags, fl5);
              ANL       GlobalFlags, #$DF

;   Change  (GlobalFlags, fl6);
              XRL       GlobalFlags, #$40

; 
;   RestartWatchdog;
;   CPL  WatchdogClock;
              CPL       P0.4

; 
;   DataRecord1.L := $12345678;
              MOV       DataRecord1, #$78
              MOV       DataRecord1+1, #$56
              MOV       DataRecord1+2, #$34
              MOV       DataRecord1+3, #$12

;   With DataRecord2 do
;     begin
;       DataWord   := DataRecord1.W0 + DataRecord1.W1;
              MOV       A, DataRecord1
              ADD       A, DataRecord1+2
              MOV       DataRecord2, A
              MOV       A, DataRecord1+1
              ADDC      A, DataRecord1+3
              MOV       DataRecord2+1, A

;       LocalFlags := [3, 5, 6];
              MOV       DataRecord2+2, #$68

;       Flags      := GlobalFlags;
              MOV       DataRecord2+3, GlobalFlags

;     end;
; 
;   RestartWatchdog;
;   CPL  WatchdogClock;
              CPL       P0.4

; 
;   Case fl6 in DataRecord2.Flags of
              MOV       A, DataRecord2+3

;     True: DataRecord2.Flags := DataRecord2.Flags * TempFlags + [fl2, fl3];
              JNB       ACC.6, L_0040
              MOV       A, DataRecord2+3
              ANL       A, #$0B
              ORL       A, #$0C
              MOV       DataRecord2+3, A
              SJMP      L_0043
L_0040:

;     else  DataRecord2.Flags := TempFlags;
              MOV       DataRecord2+3, #$0B
L_0043:

;   end;
; 
;   RestartWatchdog;
;   CPL  WatchdogClock;
              CPL       P0.4

; 
;   If 0 in DataRecord2.IndividualBits then With DataRecord2 do
              MOV       A, DataRecord2
              JNB       ACC.0, L_0053

;     begin
;       Include (IndividualBits, 4);
              ORL       DataRecord2, #$10

;       Exclude (IndividualBits, 15);
              ANL       DataRecord2+1, #$7F

;       Change  (IndividualBits, 31);
              XRL       DataRecord2+3, #$80

;     end;
L_0053:
; 
; { Call to function UpcaseChar }
; 
;   Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
              MOV       R2, #LOW  ($001A)
              MOV       R3, #HIGH ($001A)
              LCALL     sysRandomWord
              MOV       A, R2
              ADD       A, #LOW  ($0061)
              MOV       Character, A

;   DataRecord1.Ch0 := UpcaseChar (Character);
              MOV       Ch, Character
              LCALL     UpcaseChar
              MOV       DataRecord1, A

; 
; { Inline function InlineUpcaseChar }
; 
;   Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
              MOV       R2, #LOW  ($001A)
              MOV       R3, #HIGH ($001A)
              LCALL     sysRandomWord
              MOV       A, R2
              ADD       A, #LOW  ($0061)
              MOV       Character, A

;   DataRecord1.Ch1 := InlineUpcaseChar (Character);
              MOV       Ch, Character

;   Result := Char (Byte (Ch) and $DF);
              MOV       A, Ch
              ANL       A, #$DF

              MOV       DataRecord1+1, A
; 
; { Call to procedure Multiply }
; 
;   Multiply (DataRecord1.B1);
              MOV       Factor, #DataRecord1+1
              LCALL     Multiply

; 
; { Inline procedure InlineMultiply }
; 
;   InlineMultiply (DataRecord1.B1);
              MOV       Factor, #DataRecord1+1

;   Factor := Factor * 10;
              MOV       R0, Factor
              MOV       A, @R0
              MOV       B, #$0A
              MUL       AB
              MOV       @R0, A

;   If Factor >= 100 then Factor := 0;
              ADD       A, #-100
              JNC       L_0093
              MOV       @R0, #0
L_0093:

; 
; {$InlineCode Off }
; 
; { Normal call to Inline function InlineUpcaseChar }
; 
;   Character := Chr (Ord ('a') + Random (Ord ('z') - Ord ('a') + 1));
              MOV       R2, #LOW  ($001A)
              MOV       R3, #HIGH ($001A)
              LCALL     sysRandomWord
              MOV       A, R2
              ADD       A, #LOW  ($0061)
              MOV       Character, A

;   DataRecord1.Ch1 := InlineUpcaseChar (Character);
              MOV       Ch, Character
              LCALL     InlineUpcaseChar
              MOV       DataRecord1+1, A

; 
; { Normal call to Inline procedure InlineMultiply }
; 
;   InlineMultiply (DataRecord1.B1);
              MOV       Factor, #DataRecord1+1
              LCALL     InlineMultiply

; end.
L_00AD:
              SJMP      L_00AD

RSEG _CONST


              END


```

