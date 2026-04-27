# 8051 Interrupts in Turbo51 | Documentation

**URL:** https://turbo51.com/documentation/8051-interrupts

---

## Table of Contents

- 8051 Interrupts in Turbo51
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Interrupts are procedures declared with the Interrupt directive and interrupt address. In this example Timer0 is a constant defined in the
System unit
. For any procedure we can optionally define register bank to be used in this procedure by
Using
and number of bank (0 to 3) or we can define bank independent procedure with
UsingAny
. Such procedure can be called from any interrupt.
Warning
: make sure that all varables that might be changed in the interrupt procedure are marked with the
Volatile
directive. This will tell the compiler that their value can be modified outside of current program flow so many optimizations on these varables will not be performed since their value is not known.
Do not place in an interrupt routine time consuming operations like floating point operations, file I/O, string manipulations, large memory moves, etc
.
Example of interrupt declaration:
Program InterruptDemo;

Const
  Const1ms = - 22118400 div 12 div 1000;

Var
  RS485_TX: Boolean absolute P0.3;
  RX_Led:   Boolean absolute P0.4;
  TX_Led:   Boolean absolute P0.5;

  BlinkTimer:         Word; Volatile;
  KeyProcessingTimer: Word; Volatile;
  DelayTimer:         Word; Volatile;
  RS485_Timer:        Byte; Volatile;
  RX_LedTimer:        Byte; Volatile;
  TX_LedTimer:        Byte; Volatile;

Procedure TimerProc; Interrupt Timer0; Using 2; { 1 ms interrupt }
 begin
   TL0 :=  Lo (Const1ms);
   TH0 :=  Hi (Const1ms);

   Inc (BlinkTimer);
   Inc (KeyProcessingTimer);

   If DelayTimer <> 0 then Dec (DelayTimer);

   If RS485_Timer <> 0 then Dec (RS485_Timer) else RS485_TX := False;
   If RX_LedTimer <> 0 then Dec (RX_LedTimer) else RX_Led := False;
   If TX_LedTimer <> 0 then Dec (TX_LedTimer) else TX_Led := False;
 end;

begin
 { Some code }
end.
Compiled program above looks like this:
; Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa

$REGISTERBANK (0, 2)

_CODE         SEGMENT  CODE
_DATA         SEGMENT  DATA

              EXTRN    IDATA (StackStart)

; Program InterruptDemo;
; 
; Const

RSEG _CONST

;   Const1ms = - 22118400 div 12 div 1000;
; 
; Var
;   RS485_TX: Boolean absolute P0.3;
RS485_TX                        BIT      P0.3

;   RX_Led:   Boolean absolute P0.4;
RX_Led                          BIT      P0.4

;   TX_Led:   Boolean absolute P0.5;
TX_Led                          BIT      P0.5

; 
;   BlinkTimer:         Word; Volatile;

RSEG _DATA

BlinkTimer:                     DS       2

;   KeyProcessingTimer: Word; Volatile;
KeyProcessingTimer:             DS       2

;   DelayTimer:         Word; Volatile;
DelayTimer:                     DS       2

;   RS485_Timer:        Byte; Volatile;
RS485_Timer:                    DS       1

;   RX_LedTimer:        Byte; Volatile;
RX_LedTimer:                    DS       1

;   TX_LedTimer:        Byte; Volatile;
TX_LedTimer:                    DS       1

; 
; Procedure TimerProc; Interrupt Timer0; Using 2; { 1 ms interrupt }


CSEG AT $000B

USING 2

TimerProc:
;  begin
              PUSH      PSW
              MOV       PSW, #$10
              PUSH      ACC

;    TL0 :=  Lo (Const1ms);
              MOV       TL0, #$CD

;    TH0 :=  Hi (Const1ms);
              MOV       TH0, #$F8

; 
;    Inc (BlinkTimer);
              INC       BlinkTimer
              MOV       A, BlinkTimer
              JNZ       L_0020
              INC       BlinkTimer+1
L_0020:

;    Inc (KeyProcessingTimer);
              INC       KeyProcessingTimer
              MOV       A, KeyProcessingTimer
              JNZ       L_0028
              INC       KeyProcessingTimer+1
L_0028:

; 
;    If DelayTimer <> 0 then Dec (DelayTimer);
              MOV       A, DelayTimer+1
              ORL       A, DelayTimer
              JZ        L_0037
              DEC       DelayTimer
              MOV       A, DelayTimer
              CJNE      A, #-1, L_0037
              DEC       DelayTimer+1
L_0037:

; 
;    If RS485_Timer <> 0 then Dec (RS485_Timer) else RS485_TX := False;
              MOV       A, RS485_Timer
              JZ        L_003F
              DEC       RS485_Timer
              SJMP      L_0041
L_003F:
              CLR       P0.3
L_0041:

;    If RX_LedTimer <> 0 then Dec (RX_LedTimer) else RX_Led := False;
              MOV       A, RX_LedTimer
              JZ        L_0049
              DEC       RX_LedTimer
              SJMP      L_004B
L_0049:
              CLR       P0.4
L_004B:

;    If TX_LedTimer <> 0 then Dec (TX_LedTimer) else TX_Led := False;
              MOV       A, TX_LedTimer
              JZ        L_0053
              DEC       TX_LedTimer
              SJMP      L_0055
L_0053:
              CLR       P0.5
L_0055:

;  end;
              POP       ACC
              POP       PSW
              RETI


CSEG AT $0000

              AJMP     InterruptDemo


RSEG _CODE

InterruptDemo:
; 
; begin
              MOV       SP, #StackStart-1

;  { Some code }
; end.
L_0006:
              SJMP      L_0006

RSEG _CONST


              END

---

## Code Examples

### Code Block 1

```pascal
Program InterruptDemo;

Const
  Const1ms = - 22118400 div 12 div 1000;

Var
  RS485_TX: Boolean absolute P0.3;
  RX_Led:   Boolean absolute P0.4;
  TX_Led:   Boolean absolute P0.5;

  BlinkTimer:         Word; Volatile;
  KeyProcessingTimer: Word; Volatile;
  DelayTimer:         Word; Volatile;
  RS485_Timer:        Byte; Volatile;
  RX_LedTimer:        Byte; Volatile;
  TX_LedTimer:        Byte; Volatile;

Procedure TimerProc; Interrupt Timer0; Using 2; { 1 ms interrupt }
 begin
   TL0 :=  Lo (Const1ms);
   TH0 :=  Hi (Const1ms);

   Inc (BlinkTimer);
   Inc (KeyProcessingTimer);

   If DelayTimer <> 0 then Dec (DelayTimer);

   If RS485_Timer <> 0 then Dec (RS485_Timer) else RS485_TX := False;
   If RX_LedTimer <> 0 then Dec (RX_LedTimer) else RX_Led := False;
   If TX_LedTimer <> 0 then Dec (TX_LedTimer) else TX_Led := False;
 end;

begin
 { Some code }
end.

```

### Code Block 2

```pascal
; Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa

$REGISTERBANK (0, 2)

_CODE         SEGMENT  CODE
_DATA         SEGMENT  DATA

              EXTRN    IDATA (StackStart)

; Program InterruptDemo;
; 
; Const

RSEG _CONST

;   Const1ms = - 22118400 div 12 div 1000;
; 
; Var
;   RS485_TX: Boolean absolute P0.3;
RS485_TX                        BIT      P0.3

;   RX_Led:   Boolean absolute P0.4;
RX_Led                          BIT      P0.4

;   TX_Led:   Boolean absolute P0.5;
TX_Led                          BIT      P0.5

; 
;   BlinkTimer:         Word; Volatile;

RSEG _DATA

BlinkTimer:                     DS       2

;   KeyProcessingTimer: Word; Volatile;
KeyProcessingTimer:             DS       2

;   DelayTimer:         Word; Volatile;
DelayTimer:                     DS       2

;   RS485_Timer:        Byte; Volatile;
RS485_Timer:                    DS       1

;   RX_LedTimer:        Byte; Volatile;
RX_LedTimer:                    DS       1

;   TX_LedTimer:        Byte; Volatile;
TX_LedTimer:                    DS       1

; 
; Procedure TimerProc; Interrupt Timer0; Using 2; { 1 ms interrupt }


CSEG AT $000B

USING 2

TimerProc:
;  begin
              PUSH      PSW
              MOV       PSW, #$10
              PUSH      ACC

;    TL0 :=  Lo (Const1ms);
              MOV       TL0, #$CD

;    TH0 :=  Hi (Const1ms);
              MOV       TH0, #$F8

; 
;    Inc (BlinkTimer);
              INC       BlinkTimer
              MOV       A, BlinkTimer
              JNZ       L_0020
              INC       BlinkTimer+1
L_0020:

;    Inc (KeyProcessingTimer);
              INC       KeyProcessingTimer
              MOV       A, KeyProcessingTimer
              JNZ       L_0028
              INC       KeyProcessingTimer+1
L_0028:

; 
;    If DelayTimer <> 0 then Dec (DelayTimer);
              MOV       A, DelayTimer+1
              ORL       A, DelayTimer
              JZ        L_0037
              DEC       DelayTimer
              MOV       A, DelayTimer
              CJNE      A, #-1, L_0037
              DEC       DelayTimer+1
L_0037:

; 
;    If RS485_Timer <> 0 then Dec (RS485_Timer) else RS485_TX := False;
              MOV       A, RS485_Timer
              JZ        L_003F
              DEC       RS485_Timer
              SJMP      L_0041
L_003F:
              CLR       P0.3
L_0041:

;    If RX_LedTimer <> 0 then Dec (RX_LedTimer) else RX_Led := False;
              MOV       A, RX_LedTimer
              JZ        L_0049
              DEC       RX_LedTimer
              SJMP      L_004B
L_0049:
              CLR       P0.4
L_004B:

;    If TX_LedTimer <> 0 then Dec (TX_LedTimer) else TX_Led := False;
              MOV       A, TX_LedTimer
              JZ        L_0053
              DEC       TX_LedTimer
              SJMP      L_0055
L_0053:
              CLR       P0.5
L_0055:

;  end;
              POP       ACC
              POP       PSW
              RETI


CSEG AT $0000

              AJMP     InterruptDemo


RSEG _CODE

InterruptDemo:
; 
; begin
              MOV       SP, #StackStart-1

;  { Some code }
; end.
L_0006:
              SJMP      L_0006

RSEG _CONST


              END


```

