# 8051 Instruction Set | Documentation

**URL:** https://turbo51.com/documentation/8051-instruction-set

---

## Table of Contents

- 8051 Instruction Set
  - Description of All Instructions
  - Instructions by opcode
  - 8051 Instruction Set: ACALL
  - 8051 Instruction Set: ADD
  - 8051 Instruction Set: AJMP
  - 8051 Instruction Set: ANL
  - 8051 Instruction Set: CJNE
  - 8051 Instruction Set: CLR
  - 8051 Instruction Set: CPL
  - 8051 Instruction Set: DA
  - 8051 Instruction Set: DEC
  - 8051 Instruction Set: DIV
  - 8051 Instruction Set: DJNZ
  - 8051 Instruction Set: INC
  - 8051 Instruction Set: JB
  - 8051 Instruction Set: JBC
  - 8051 Instruction Set: JC
  - 8051 Instruction Set: JMP
  - 8051 Instruction Set: JNB
  - 8051 Instruction Set: JNC
  - 8051 Instruction Set: JNZ
  - 8051 Instruction Set: JZ
  - 8051 Instruction Set: LCALL
  - 8051 Instruction Set: LJMP
  - 8051 Instruction Set: MOV
  - 8051 Instruction Set: MOVC
  - 8051 Instruction Set: MOVX
  - 8051 Instruction Set: MUL
  - 8051 Instruction Set: NOP
  - 8051 Instruction Set: ORL
  - 8051 Instruction Set: POP
  - 8051 Instruction Set: PUSH
  - 8051 Instruction Set: RET
  - 8051 Instruction Set: RETI
  - 8051 Instruction Set: RL
  - 8051 Instruction Set: RLC
  - 8051 Instruction Set: RR
  - 8051 Instruction Set: RRC
  - 8051 Instruction Set: SETB
  - 8051 Instruction Set: SJMP
  - 8051 Instruction Set: SUBB
  - 8051 Instruction Set: SWAP
  - 8051 Instruction Set: Undefined Instruction
  - 8051 Instruction Set: XCH
  - 8051 Instruction Set: XCHD
  - 8051 Instruction Set: XRL
    - Microcontroller Books

---

## Content

Description of All Instructions
The following table lists the 8051 instructions sorted by
opcode
and the next table lists the 8051 instructions in the alphabetical order. The tables are followed with the complete description of each of the 8051 instructions.
Instructions by opcode
00
01
02
03
04
05
06
07
08
09
0A
0B
0C
0D
0E
0F
00
NOP
AJMP
LJMP
RR
INC
INC
INC
INC
INC
INC
INC
INC
INC
INC
INC
INC
10
JBC
ACALL
LCALL
RRC
DEC
DEC
DEC
DEC
DEC
DEC
DEC
DEC
DEC
DEC
DEC
DEC
20
JB
AJMP
RET
RL
ADD
ADD
ADD
ADD
ADD
ADD
ADD
ADD
ADD
ADD
ADD
ADD
30
JNB
ACALL
RETI
RLC
ADDC
ADDC
ADDC
ADDC
ADDC
ADDC
ADDC
ADDC
ADDC
ADDC
ADDC
ADDC
40
JC
AJMP
ORL
ORL
ORL
ORL
ORL
ORL
ORL
ORL
ORL
ORL
ORL
ORL
ORL
ORL
50
JNC
ACALL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
ANL
60
JZ
AJMP
XRL
XRL
XRL
XRL
XRL
XRL
XRL
XRL
XRL
XRL
XRL
XRL
XRL
XRL
70
JNZ
ACALL
ORL
JMP
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
80
SJMP
AJMP
ANL
MOVC
DIV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
90
MOV
ACALL
MOV
MOVC
SUBB
SUBB
SUBB
SUBB
SUBB
SUBB
SUBB
SUBB
SUBB
SUBB
SUBB
SUBB
A0
ORL
AJMP
MOV
INC
MUL
===
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
B0
ANL
ACALL
CPL
CPL
CJNE
CJNE
CJNE
CJNE
CJNE
CJNE
CJNE
CJNE
CJNE
CJNE
CJNE
CJNE
C0
PUSH
AJMP
CLR
CLR
SWAP
XCH
XCH
XCH
XCH
XCH
XCH
XCH
XCH
XCH
XCH
XCH
D0
POP
ACALL
SETB
SETB
DA
DJNZ
XCHD
XCHD
DJNZ
DJNZ
DJNZ
DJNZ
DJNZ
DJNZ
DJNZ
DJNZ
E0
MOVX
AJMP
MOVX
MOVX
CLR
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
F0
MOVX
ACALL
MOVX
MOVX
CPL
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
MOV
Alphabetical List of Instructions
ACALL
- Absolute Call
ADD, ADDC
- Add Accumulator (With Carry)
AJMP
- Absolute Jump
ANL
- Bitwise AND
CJNE
- Compare and Jump if Not Equal
CLR
- Clear Register
CPL
- Complement Register
DA
- Decimal Adjust
DEC
- Decrement Register
DIV
- Divide Accumulator by B
DJNZ
- Decrement Register and Jump if Not Zero
INC
- Increment Register
JB
- Jump if Bit Set
JBC
- Jump if Bit Set and Clear Bit
JC
- Jump if Carry Set
JMP
- Jump to Address
JNB
- Jump if Bit Not Set
JNC
- Jump if Carry Not Set
JNZ
- Jump if Accumulator Not Zero
JZ
- Jump if Accumulator Zero
LCALL
- Long Call
LJMP
- Long Jump
MOV
- Move Memory
MOVC
- Move Code Memory
MOVX
- Move Extended Memory
MUL
- Multiply Accumulator by B
NOP
- No Operation
ORL
- Bitwise OR
POP
- Pop Value From Stack
PUSH
- Push Value Onto Stack
RET
- Return From Subroutine
RETI
- Return From Interrupt
RL
- Rotate Accumulator Left
RLC
- Rotate Accumulator Left Through Carry
RR
- Rotate Accumulator Right
RRC
- Rotate Accumulator Right Through Carry
SETB
- Set Bit
SJMP
- Short Jump
SUBB
- Subtract From Accumulator With Borrow
SWAP
- Swap Accumulator Nibbles
XCH
- Exchange Bytes
XCHD
- Exchange Digits
XRL
- Bitwise Exclusive OR
Undefined
- Undefined Instruction
8051 Instruction Set: ACALL
Operation:
ACALL
Function:
Absolute Call Within 2K Block
Syntax:
ACALL
code address
Instructions
OpCode
Bytes
Flags
ACALL
page0
11
2
None
ACALL
page1
31
2
None
ACALL
page2
51
2
None
ACALL
page3
71
2
None
ACALL
page4
91
2
None
ACALL
page5
B1
2
None
ACALL
page6
D1
2
None
ACALL
page7
F1
2
None
Description:
ACALL unconditionally calls a subroutine at the indicated
code address
. ACALL pushes the address of the instruction that follows ACALL onto the stack, least-significant-byte first, most-significant-byte second. The Program Counter is then updated so that program execution continues at the indicated address.
The new value for the Program Counter is calculated by replacing the least-significant-byte of the Program Counter with the second byte of the ACALL instruction, and replacing bits 0-2 of the most-significant-byte of the Program Counter with 3 bits that indicate the page. Bits 3-7 of the most-significant-byte of the Program Counter remain unchaged.
Since only 11 bits of the Program Counter are affected by ACALL, calls may only be made to routines located within the same 2k block as the first byte that follows ACALL.
See Also
:
LCALL
,
RET
8051 Instruction Set: ADD
Operation:
ADD, ADDC
Function:
Add Accumulator, Add Accumulator With Carry
Syntax:
ADD A,
operand
ADDC A,
operand
Instructions
OpCode
Bytes
Flags
ADD A,#
data
24
2
C, AC, OV
ADD A,
iram addr
25
2
C, AC, OV
ADD A,@R0
26
1
C, AC, OV
ADD A,@R1
27
1
C, AC, OV
ADD A,R0
28
1
C, AC, OV
ADD A,R1
29
1
C, AC, OV
ADD A,R2
2A
1
C, AC, OV
ADD A,R3
2B
1
C, AC, OV
ADD A,R4
2C
1
C, AC, OV
ADD A,R5
2D
1
C, AC, OV
ADD A,R6
2E
1
C, AC, OV
ADD A,R7
2F
1
C, AC, OV
Instructions
OpCode
Bytes
Flags
ADDC A,#
data
34
2
C, AC, OV
ADDC A,
iram addr
35
2
C, AC, OV
ADDC A,@R0
36
1
C, AC, OV
ADDC A,@R1
37
1
C, AC, OV
ADDC A,R0
38
1
C, AC, OV
ADDC A,R1
39
1
C, AC, OV
ADDC A,R2
3A
1
C, AC, OV
ADDC A,R3
3B
1
C, AC, OV
ADDC A,R4
3C
1
C, AC, OV
ADDC A,R5
3D
1
C, AC, OV
ADDC A,R6
3E
1
C, AC, OV
ADDC A,R7
3F
1
C, AC, OV
Description:
Description: ADD and ADDC both add the value
operand
to the value of the Accumulator, leaving the resulting value in the Accumulator. The value
operand
is not affected. ADD and ADDC function identically except that ADDC adds the value of operand as well as the value of the Carry flag whereas ADD does not add the Carry flag to the result.
The
Carry bit (C)
is set if there is a carry-out of bit 7. In other words, if the unsigned summed value of the Accumulator,
operand
and (in the case of ADDC) the Carry flag exceeds 255 Carry is set. Otherwise, the Carry bit is cleared.
The
Auxillary Carry (AC)
bit is set if there is a carry-out of bit 3. In other words, if the unsigned summed value of the low nibble of the Accumulator,
operand
and (in the case of ADDC) the Carry flag exceeds 15 the Auxillary Carry flag is set. Otherwise, the Auxillary Carry flag is cleared.
The
Overflow (OV)
bit is set if there is a carry-out of bit 6 or out of bit 7, but not both. In other words, if the addition of the Accumulator,
operand
and (in the case of ADDC) the Carry flag treated as signed values results in a value that is out of the range of a signed byte (-128 through +127) the Overflow flag is set. Otherwise, the Overflow flag is cleared.
See Also:
SUBB
,
DA
,
INC
,
DEC
8051 Instruction Set: AJMP
Operation:
AJMP
Function:
Absolute Jump Within 2K Block
Syntax:
AJMP
code address
Instructions
OpCode
Bytes
Flags
AJMP
page0
01
2
None
AJMP
page1
21
2
None
AJMP
page2
41
2
None
AJMP
page3
61
2
None
AJMP
page4
81
2
None
AJMP
page5
A1
2
None
AJMP
page6
C1
2
None
AJMP
page7
E1
2
None
Description:
AJMP unconditionally jumps to the indicated
code address
. The new value for the Program Counter is calculated by replacing the least-significant-byte of the Program Counter with the second byte of the AJMP instruction, and replacing bits 0-2 of the most-significant-byte of the Program Counter with 3 bits that indicate the page
of the byte following the AJMP instruction
. Bits 3-7 of the most-significant-byte of the Program Counter remain unchaged.
Since only 11 bits of the Program Counter are affected by AJMP, jumps may only be made to code located within the same 2k block as the first byte that follows AJMP.
See Also
:
LJMP
,
SJMP
8051 Instruction Set: ANL
Operation:
ANL
Function:
Bitwise AND
Syntax:
ANL
operand1
,
operand2
Instructions
OpCode
Bytes
Flags
ANL
iram addr
,A
52
2
None
ANL
iram addr
,#
data
53
3
None
ANL A,#
data
54
2
None
ANL A,
iram addr
55
2
None
ANL A,@R0
56
1
None
ANL A,@R1
57
1
None
ANL A,R0
58
1
None
ANL A,R1
59
1
None
ANL A,R2
5A
1
None
ANL A,R3
5B
1
None
ANL A,R4
5C
1
None
ANL A,R5
5D
1
None
ANL A,R6
5E
1
None
ANL A,R7
5F
1
None
ANL C,
bit addr
82
2
C
ANL C,/
bit addr
B0
2
C
Description:
ANL does a bitwise "AND" operation between
operand1
and
operand2
, leaving the resulting value in
operand1
. The value of operand2 is not affected. A logical "AND" compares the bits of each operand and sets the corresponding bit in the resulting byte only if the bit was set in both of the original operands, otherwise the resulting bit is cleared.
See Also:
ORL
,
XRL
8051 Instruction Set: CJNE
Operation:
CJNE
Function:
Compare and Jump If Not Equal
Syntax:
CJNE
operand1
,
operand2
,
reladdr
Instructions
OpCode
Bytes
Flags
CJNE A,#
data
,
reladdr
B4
3
C
CJNE A,
iram addr
,
reladdr
B5
3
C
CJNE @R0,#
data
,
reladdr
B6
3
C
CJNE @R1,#
data
,
reladdr
B7
3
C
CJNE R0,#
data
,
reladdr
B8
3
C
CJNE R1,#
data
,
reladdr
B9
3
C
CJNE R2,#
data
,
reladdr
BA
3
C
CJNE R3,#
data
,
reladdr
BB
3
C
CJNE R4,#
data
,
reladdr
BC
3
C
CJNE R5,#
data
,
reladdr
BD
3
C
CJNE R6,#
data
,
reladdr
BE
3
C
CJNE R7,#
data
,
reladdr
BF
3
C
Description:
CJNE compares the value of
operand1
and
operand2
and branches to the indicated relative address if
operand1
and
operand2
are not equal. If the two operands are equal program flow continues with the instruction following the CJNE instruction.
The
Carry bit (C)
is set if
operand1
is less than
operand2
, otherwise it is cleared.
See Also:
DJNZ
8051 Instruction Set: CLR
Operation:
CLR
Function:
Clear Register
Syntax:
CLR
register
Instructions
OpCode
Bytes
Flags
CLR
bit addr
C2
2
None
CLR C
C3
1
C
CLR A
E4
1
None
Description:
CLR clears (sets to 0) all the bit(s) of the indicated register. If the register is a bit (including the carry bit), only the specified bit is affected. Clearing the Accumulator sets the Accumulator's value to 0.
See Also:
SETB
8051 Instruction Set: CPL
Operation:
CPL
Function:
Complement Register
Syntax:
CPL
operand
Instructions
OpCode
Bytes
Flags
CPL A
F4
1
None
CPL C
B3
1
C
CPL
bit addr
B2
2
None
Description:
CPL complements
operand
, leaving the result in
operand
. If
operand
is a single bit then the state of the bit will be reversed. If
operand
is the Accumulator then all the bits in the Accumulator will be reversed. This can be thought of as "Accumulator Logical Exclusive OR 255" or as "255-Accumulator." If the
operand
refers to a bit of an output Port, the value that will be complemented is based on the last value written to that bit, not the last value read from it.
See Also
:
CLR
,
SETB
8051 Instruction Set: DA
Operation:
DA
Function:
Decimal Adjust Accumulator
Syntax:
DA A
Instructions
OpCode
Bytes
Flags
DA
D4
1
C
Description:
DA adjusts the contents of the Accumulator to correspond to a BCD (Binary Coded Decimal) number after two BCD numbers have been added by the ADD or ADDC instruction. If the carry bit is set or if the value of bits 0-3 exceed 9, 06 is added to the accumulator. If the carry bit was set when the instruction began, or if 06 was added to the accumulator in the first step, 60 is added to the accumulator.
The
Carry bit (C)
is set if the resulting value is greater than 99, otherwise it is cleared.
See Also:
ADD
,
ADDC
8051 Instruction Set: DEC
Operation:
DEC
Function:
Decrement Register
Syntax:
DEC
register
Instructions
OpCode
Bytes
Flags
DEC A
14
1
None
DEC
iram addr
15
2
None
DEC @R0
16
1
None
DEC @R1
17
1
None
DEC R0
18
1
None
DEC R1
19
1
None
DEC R2
1A
1
None
DEC R3
1B
1
None
DEC R4
1C
1
None
DEC R5
1D
1
None
DEC R6
1E
1
None
DEC R7
1F
1
None
Description:
DEC decrements the value of
register
by 1. If the initial value of
register
is 0, decrementing the value will cause it to reset to 255 (FF hex). Note: The Carry Flag is NOT set when the value "rolls over" from 0 to 255.
See Also:
INC
,
SUBB
8051 Instruction Set: DIV
Operation:
DIV
Function:
Divide Accumulator by B
Syntax:
DIV AB
Instructions
OpCode
Bytes
Flags
DIV AB
84
1
C, OV
Description:
Divides the unsigned value of the Accumulator by the unsigned value of the "B" register. The resulting quotient is placed in the Accumulator and the remainder is placed in the "B" register.
The
Carry flag (C)
is always cleared.
The
Overflow flag (OV)
is set if division by 0 was attempted, otherwise it is cleared.
See Also:
MUL AB
8051 Instruction Set: DJNZ
Operation:
DJNZ
Function:
Decrement and Jump if Not Zero
Syntax:
DJNZ
register
,
reladdr
Instructions
OpCode
Bytes
Flags
DJNZ
iram addr
,
reladdr
D5
3
None
DJNZ R0,
reladdr
D8
2
None
DJNZ R1,
reladdr
D9
2
None
DJNZ R2,
reladdr
DA
2
None
DJNZ R3,
reladdr
DB
2
None
DJNZ R4,
reladdr
DC
2
None
DJNZ R5,
reladdr
DD
2
None
DJNZ R6,
reladdr
DE
2
None
DJNZ R7,
reladdr
DF
2
None
Description:
DJNZ decrements the value of
register
by 1. If the initial value of
register
is 0, decrementing the value will cause it to reset to 255 (FF hex). If the new value of
register
is not 0 the program will branch to the address indicated by
relative addr
. If the new value of
register
is 0 program flow continues with the instruction following the DJNZ instruction.
See Also:
DEC
,
JZ
,
JNZ
8051 Instruction Set: INC
Operation:
INC
Function:
Increment Register
Syntax:
INC
register
Instructions
OpCode
Bytes
Flags
INC A
04
1
None
INC
iram addr
05
2
None
INC @R0
06
1
None
INC @R1
07
1
None
INC R0
08
1
None
INC R1
09
1
None
INC R2
0A
1
None
INC R3
0B
1
None
INC R4
0C
1
None
INC R5
0D
1
None
INC R6
0E
1
None
INC R7
0F
1
None
INC DPTR
A3
1
None
Description:
INC increments the value of
register
by 1. If the initial value of
register
is 255 (FF hex), incrementing the value will cause it to reset to 0. Note: The Carry Flag is NOT set when the value "rolls over" from 255 to 0.
In the case of "INC DPTR", the value two-byte unsigned integer value of DPTR is incremented. If the initial value of DPTR is 65535 (FFFF hex), incrementing the value will cause it to reset to 0. Again, the Carry Flag is NOT set when the value of DPTR "rolls over" from 65535 to 0.
See Also:
ADD
,
ADDC
,
DEC
8051 Instruction Set: JB
Operation:
JB
Function:
Jump if Bit Set
Syntax:
JB
bit addr
,
reladdr
Instructions
OpCode
Bytes
Flags
JB
bit addr
,
reladdr
20
3
None
Description:
JB branches to the address indicated by
reladdr
if the bit indicated by
bit addr
is set. If the bit is not set program execution continues with the instruction following the JB instruction.
See Also:
JBC
,
JNB
8051 Instruction Set: JBC
Operation:
JBC
Function:
Jump if Bit Set and Clear Bit
Syntax:
JB
bit addr
,
reladdr
Instructions
OpCode
Bytes
Flags
JBC
bit addr
,
reladdr
10
3
None
Description:
JBC will branch to the address indicated by
reladdr
if the bit indicated by
bit addr
is set. Before branching to
reladdr
the instruction will clear the indicated bit. If the bit is not set program execution continues with the instruction following the JBC instruction.
See Also:
JB
,
JNB
8051 Instruction Set: JC
Operation:
JC
Function:
Jump if Carry Set
Syntax:
JC
reladdr
Instructions
OpCode
Bytes
Flags
JC
reladdr
40
2
None
Description:
JC will branch to the address indicated by
reladdr
if the Carry Bit is set. If the Carry Bit is not set program execution continues with the instruction following the JC instruction.
See Also:
JNC
8051 Instruction Set: JMP
Operation:
JMP
Function:
Jump to Data Pointer + Accumulator
Syntax:
JMP @A+DPTR
Instructions
OpCode
Bytes
Flags
JMP @A+DPTR
73
1
None
Description:
JMP jumps unconditionally to the address represented by the sum of the value of DPTR and the value of the Accumulator.
See Also:
LJMP
,
AJMP
,
SJMP
8051 Instruction Set: JNB
Operation:
JNB
Function:
Jump if Bit Not Set
Syntax:
JNB
bit addr
,
reladdr
Instructions
OpCode
Bytes
Flags
JNB
bit addr
,
reladdr
30
3
None
Description:
JNB will branch to the address indicated by
reladdress
if the indicated bit is not set. If the bit is set program execution continues with the instruction following the JNB instruction.
See Also:
JB
,
JBC
8051 Instruction Set: JNC
Operation:
JNC
Function:
Jump if Carry Not Set
Syntax:
JNC
reladdr
Instructions
OpCode
Bytes
Flags
JNC
reladdr
50
2
None
Description:
JNC branches to the address indicated by
reladdr
if the carry bit is not set. If the carry bit is set program execution continues with the instruction following the JNB instruction.
See Also:
JC
8051 Instruction Set: JNZ
Operation:
JNZ
Function:
Jump if Accumulator Not Zero
Syntax:
JNZ
reladdr
Instructions
OpCode
Bytes
Flags
JNZ
reladdr
70
2
None
Description:
JNZ will branch to the address indicated by
reladdr
if the Accumulator contains any value except 0. If the value of the Accumulator is zero program execution continues with the instruction following the JNZ instruction.
See Also:
JZ
8051 Instruction Set: JZ
Operation:
JZ
Function:
Jump if Accumulator Zero
Syntax:
JNZ
reladdr
Instructions
OpCode
Bytes
Flags
JZ
reladdr
60
2
None
Description:
JZ branches to the address indicated by
reladdr
if the Accumulator contains the value 0. If the value of the Accumulator is non-zero program execution continues with the instruction following the JNZ instruction.
See Also:
JNZ
8051 Instruction Set: LCALL
Operation:
LCALL
Function:
Long Call
Syntax:
LCALL
code addr
Instructions
OpCode
Bytes
Flags
LCALL
code addr
12
3
None
Description:
LCALL calls a program subroutine. LCALL increments the program counter by 3 (to point to the instruction following LCALL) and pushes that value onto the stack (low byte first, high byte second). The Program Counter is then set to the 16-bit value which follows the LCALL opcode, causing program execution to continue at that address.
See Also:
ACALL
,
RET
8051 Instruction Set: LJMP
Operation:
LJMP
Function:
Long Jump
Syntax:
LJMP
code addr
Instructions
OpCode
Bytes
Flags
LJMP
code addr
02
3
None
Description:
LJMP jumps unconditionally to the specified
code addr
.
See Also:
AJMP
,
SJMP
,
JMP
8051 Instruction Set: MOV
Operation:
MOV
Function:
Move Memory
Syntax:
MOV
operand1
,
operand2
Instructions
OpCode
Bytes
Flags
MOV @R0,#
data
76
2
None
MOV @R1,#
data
77
2
None
MOV @R0,A
F6
1
None
MOV @R1,A
F7
1
None
MOV @R0,
iram addr
A6
2
None
MOV @R1,
iram addr
A7
2
None
MOV A,#
data
74
2
None
MOV A,@R0
E6
1
None
MOV A,@R1
E7
1
None
MOV A,R0
E8
1
None
MOV A,R1
E9
1
None
MOV A,R2
EA
1
None
MOV A,R3
EB
1
None
MOV A,R4
EC
1
None
MOV A,R5
ED
1
None
MOV A,R6
EE
1
None
MOV A,R7
EF
1
None
MOV A,
iram addr
E5
2
None
MOV C,
bit addr
A2
2
C
MOV DPTR,#
data16
90
3
None
MOV R0,#
data
78
2
None
MOV R1,#
data
79
2
None
MOV R2,#
data
7A
2
None
MOV R3,#
data
7B
2
None
MOV R4,#
data
7C
2
None
MOV R5,#
data
7D
2
None
MOV R6,#
data
7E
2
None
MOV R7,#
data
7F
2
None
MOV R0,A
F8
1
None
MOV R1,A
F9
1
None
MOV R2,A
FA
1
None
MOV R3,A
FB
1
None
MOV R4,A
FC
1
None
MOV R5,A
FD
1
None
MOV R6,A
FE
1
None
MOV R7,A
FF
1
None
MOV R0,
iram addr
A8
2
None
MOV R1,
iram addr
A9
2
None
MOV R2,
iram addr
AA
2
None
MOV R3,
iram addr
AB
2
None
MOV R4,
iram addr
AC
2
None
MOV R5,
iram addr
AD
2
None
MOV R6,
iram addr
AE
2
None
MOV R7,
iram addr
AF
2
None
MOV
bit addr
,C
92
2
None
MOV
iram addr
,#
data
75
3
None
MOV
iram addr
,@R0
86
2
None
MOV
iram addr
,@R1
87
2
None
MOV
iram addr
,R0
88
2
None
MOV
iram addr
,R1
89
2
None
MOV
iram addr
,R2
8A
2
None
MOV
iram addr
,R3
8B
2
None
MOV
iram addr
,R4
8C
2
None
MOV
iram addr
,R5
8D
2
None
MOV
iram addr
,R6
8E
2
None
MOV
iram addr
,R7
8F
2
None
MOV
iram addr
,A
F5
2
None
MOV
iram addr
,
iram addr
85
3
None
Description:
MOV copies the value of
operand2
into
operand1
. The value of
operand2
is not affected. Both
operand1
and
operand2
must be in Internal RAM. No flags are affected unless the instruction is moving the value of a bit into the carry bit in which case the carry bit is affected or unless the instruction is moving a value into the PSW register (which contains all the program flags).
** Note: In the case of "MOV iram addr,iram addr", the operand bytes of the instruction are stored in reverse order. That is, the instruction consisting of the bytes 85, 20, 50 means "Move the contents of Internal RAM location 20 to Internal RAM location 50" whereas the opposite would be generally presumed.
See Also:
MOVC
,
MOVX
,
XCH
,
XCHD
,
PUSH
,
POP
8051 Instruction Set: MOVC
Operation:
MOVC
Function:
Move Code Byte to Accumulator
Syntax:
MOVC A,@A+
register
Instructions
OpCode
Bytes
Flags
MOVC A,@A+DPTR
93
1
None
MOVC A,@A+PC
83
1
None
Description:
MOVC moves a byte from Code Memory into the Accumulator. The Code Memory address from which the byte will be moved is calculated by summing the value of the Accumulator with either DPTR or the Program Counter (PC). In the case of the Program Counter, PC is first incremented by 1 before being summed with the Accumulator.
See Also:
MOV
,
MOVX
8051 Instruction Set: MOVX
Operation:
MOVX
Function:
Move Data To/From External Memory (XRAM)
Syntax:
MOVX
operand1
,
operand2
Instructions
OpCode
Bytes
Flags
MOVX @DPTR,A
F0
1
None
MOVX @R0,A
F2
1
None
MOVX @R1,A
F3
1
None
MOVX A,@DPTR
E0
1
None
MOVX A,@R0
E2
1
None
MOVX A,@R1
E3
1
None
Description:
MOVX moves a byte to or from External Memory into or from the Accumulator.
If
operand1
is @DPTR, the Accumulator is moved to the 16-bit External Memory address indicated by DPTR. This instruction uses both P0 (port 0) and P2 (port 2) to output the 16-bit address and data. If
operand2
is DPTR then the byte is moved from External Memory into the Accumulator.
If
operand1
is @R0 or @R1, the Accumulator is moved to the 8-bit External Memory address indicated by the specified Register. This instruction uses only P0 (port 0) to output the 8-bit address and data. P2 (port 2) is not affected. If
operand2
is @R0 or @R1 then the byte is moved from External Memory into the Accumulator.
See Also:
MOV
,
MOVC
8051 Instruction Set: MUL
Operation:
MUL
Function:
Multiply Accumulator by B
Syntax:
MUL AB
Instructions
OpCode
Bytes
Flags
MUL AB
A4
1
C, OV
Description:
Multiples the unsigned value of the Accumulator by the unsigned value of the "B" register. The least significant byte of the result is placed in the Accumulator and the most-significant-byte is placed in the "B" register.
The
Carry Flag (C)
is always cleared.
The
Overflow Flag (OV)
is set if the result is greater than 255 (if the most-significant byte is not zero), otherwise it is cleared.
See Also:
DIV
8051 Instruction Set: NOP
Operation:
NOP
Function:
None, waste time
Syntax:
No Operation
Instructions
OpCode
Bytes
Flags
NOP
00
1
None
Description:
NOP, as it's name suggests, causes No Operation to take place for one machine cycle. NOP is generally used only for timing purposes. Absolutely no flags or registers are affected.
8051 Instruction Set: ORL
Operation:
ORL
Function:
Bitwise OR
Syntax:
ORL
operand1
,
operand2
Instructions
OpCode
Bytes
Flags
ORL
iram addr
,A
42
2
None
ORL
iram addr
,#
data
43
3
None
ORL A,#
data
44
2
None
ORL A,
iram addr
45
2
None
ORL A,@R0
46
1
None
ORL A,@R1
47
1
None
ORL A,R0
48
1
None
ORL A,R1
49
1
None
ORL A,R2
4A
1
None
ORL A,R3
4B
1
None
ORL A,R4
4C
1
None
ORL A,R5
4D
1
None
ORL A,R6
4E
1
None
ORL A,R7
4F
1
None
ORL C,
bit addr
72
2
C
ORL C,/
bit addr
A0
2
C
Description:
ORL does a bitwise "OR" operation between
operand1
and
operand2
, leaving the resulting value in
operand1.
The value of
operand2
is not affected. A logical "OR" compares the bits of each operand and sets the corresponding bit in the resulting byte if the bit was set in either of the original operands, otherwise the resulting bit is cleared.
See Also:
ANL
,
XRL
8051 Instruction Set: POP
Operation:
POP
Function:
Pop Value From Stack
Syntax:
POP
Instructions
OpCode
Bytes
Flags
POP
iram addr
D0
2
None
Description:
POP "pops" the last value placed on the stack into the
iram addr
specified. In other words, POP will load
iram addr
with the value of the Internal RAM address pointed to by the current Stack Pointer. The stack pointer is then decremented by 1.
See Also:
PUSH
8051 Instruction Set: PUSH
Operation:
PUSH
Function:
Push Value Onto Stack
Syntax:
PUSH
Instructions
OpCode
Bytes
Flags
PUSH
iram addr
C0
2
None
Description:
PUSH "pushes" the value of the specified
iram addr
onto the stack. PUSH first increments the value of the Stack Pointer by 1, then takes the value stored in
iram addr
and stores it in Internal RAM at the location pointed to by the incremented Stack Pointer.
See Also:
POP
8051 Instruction Set: RET
Operation:
RET
Function:
Return From Subroutine
Syntax:
RET
Instructions
OpCode
Bytes
Flags
RET
22
1
None
Description:
RET is used to return from a subroutine previously called by LCALL or ACALL. Program execution continues at the address that is calculated by popping the topmost 2 bytes off the stack. The most-significant-byte is popped off the stack first, followed by the least-significant-byte.
See Also:
LCALL
,
ACALL
,
RETI
8051 Instruction Set: RETI
Operation:
RETI
Function:
Return From Interrupt
Syntax:
RETI
Instructions
OpCode
Bytes
Flags
RETI
32
1
None
Description:
RETI is used to return from an interrupt service routine. RETI first enables interrupts of equal and lower priorities to the interrupt that is terminating. Program execution continues at the address that is calculated by popping the topmost 2 bytes off the stack. The most-significant-byte is popped off the stack first, followed by the least-significant-byte.
RETI functions identically to RET if it is executed outside of an interrupt service routine.
See Also:
RET
8051 Instruction Set: RL
Operation:
RL
Function:
Rotate Accumulator Left
Syntax:
RL A
Instructions
OpCode
Bytes
Flags
RL A
23
1
C
Description:
Shifts the bits of the Accumulator to the left. The left-most bit (bit 7) of the Accumulator is loaded into bit 0.
See Also:
RLC
,
RR
,
RRC
8051 Instruction Set: RLC
Operation:
RLC
Function:
Rotate Accumulator Left Through Carry
Syntax:
RLC A
Instructions
OpCode
Bytes
Flags
RLC A
33
1
C
Description:
Shifts the bits of the Accumulator to the left. The left-most bit (bit 7) of the Accumulator is loaded into the Carry Flag, and the original Carry Flag is loaded into bit 0 of the Accumulator. This function can be used to quickly multiply a byte by 2.
See Also:
RL
,
RR
,
RRC
8051 Instruction Set: RR
Operation:
RR
Function:
Rotate Accumulator Right
Syntax:
RR A
Instructions
OpCode
Bytes
Flags
RR A
03
1
None
Description:
Shifts the bits of the Accumulator to the right. The right-most bit (bit 0) of the Accumulator is loaded into bit 7.
See Also:
RL
,
RLC
,
RRC
8051 Instruction Set: RRC
Operation:
RRC
Function:
Rotate Accumulator Right Through Carry
Syntax:
RRC A
Instructions
OpCode
Bytes
Flags
RRC A
13
1
C
Description:
Shifts the bits of the Accumulator to the right. The right-most bit (bit 0) of the Accumulator is loaded into the Carry Flag, and the original Carry Flag is loaded into bit 7. This function can be used to quickly divide a byte by 2.
See Also:
RL
,
RLC
,
RR
8051 Instruction Set: SETB
Operation:
SETB
Function:
Set Bit
Syntax:
SETB
bit addr
Instructions
OpCode
Bytes
Flags
SETB C
D3
1
C
SETB
bit addr
D2
2
None
Description:
Sets the specified bit.
See Also:
CLR
8051 Instruction Set: SJMP
Operation:
SJMP
Function:
Short Jump
Syntax:
SJMP
reladdr
Instructions
OpCode
Bytes
Flags
SJMP
reladdr
80
2
None
Description:
SJMP jumps unconditionally to the address specified
reladdr
.
Reladdr
must be within -128 or +127 bytes of the instruction that follows the SJMP instruction.
See Also:
LJMP
,
AJMP
8051 Instruction Set: SUBB
Operation:
SUBB
Function:
Subtract from Accumulator With Borrow
Syntax:
SUBB A,
operand
Instructions
OpCode
Bytes
Flags
SUBB A,#
data
94
2
C, AC, OV
SUBB A,
iram addr
95
2
C, AC, OV
SUBB A,@R0
96
1
C, AC, OV
SUBB A,@R1
97
1
C, AC, OV
SUBB A,R0
98
1
C, AC, OV
SUBB A,R1
99
1
C, AC, OV
SUBB A,R2
9A
1
C, AC, OV
SUBB A,R3
9B
1
C, AC, OV
SUBB A,R4
9C
1
C, AC, OV
SUBB A,R5
9D
1
C, AC, OV
SUBB A,R6
9E
1
C, AC, OV
SUBB A,R7
9F
1
C, AC, OV
Description:
SUBB subtract the value of
operand
from the value of the Accumulator, leaving the resulting value in the Accumulator. The value
operand
is not affected.
The
Carry Bit (C)
is set if a borrow was required for bit 7, otherwise it is cleared. In other words, if the unsigned value being subtracted is greater than the Accumulator the Carry Flag is set.
The
Auxillary Carry (AC)
bit is set if a borrow was required for bit 3, otherwise it is cleared. In other words, the bit is set if the low nibble of the value being subtracted was greater than the low nibble of the Accumulator.
The
Overflow (OV)
bit is set if a borrow was required for bit 6 or for bit 7, but not both. In other words, the subtraction of two signed bytes resulted in a value outside the range of a signed byte (-128 to 127). Otherwise it is cleared.
See Also:
ADD
,
ADDC
,
DEC
8051 Instruction Set: SWAP
Operation:
SWAP
Function:
Swap Accumulator Nibbles
Syntax:
SWAP A
Instructions
OpCode
Bytes
Flags
SWAP A
C4
1
None
Description:
SWAP swaps bits 0-3 of the Accumulator with bits 4-7 of the Accumulator. This instruction is identical to executing "RR A" or "RL A" four times.
See Also:
RL
,
RLC
,
RR
,
RRC
8051 Instruction Set: Undefined Instruction
Operation:
Undefined Instruction
Function:
Undefined
Syntax:
Instructions
OpCode
Bytes
Flags
A5
1
C
Description:
This opcode is not a valid instruction.
Note:
Philips 8051 model P89C669 uses instruction prefix A5 to let the user access a different (extended) SFR area.
8051 Instruction Set: XCH
Operation:
XCH
Function:
Exchange Bytes
Syntax:
XCH A,
register
Instructions
OpCode
Bytes
Flags
XCH A,@R0
C6
1
None
XCH A,@R1
C7
1
None
XCH A,R0
C8
1
None
XCH A,R1
C9
1
None
XCH A,R2
CA
1
None
XCH A,R3
CB
1
None
XCH A,R4
CC
1
None
XCH A,R5
CD
1
None
XCH A,R6
CE
1
None
XCH A,R7
CF
1
None
XCH A,
iram addr
C5
2
None
Description:
Exchanges the value of the Accumulator with the value contained in
register
.
See Also:
MOV
8051 Instruction Set: XCHD
Operation:
XCHD
Function:
Exchange Digit
Syntax:
XCHD A,
[@R0/@R1]
Instructions
OpCode
Bytes
Flags
XCHD A,@R0
D6
1
None
XCHD A,@R1
D7
1
None
Description:
Exchanges bits 0-3 of the Accumulator with bits 0-3 of the Internal RAM address pointed to indirectly by R0 or R1. Bits 4-7 of each register are unaffected.
See Also:
DA
8051 Instruction Set: XRL
Operation:
XRL
Function:
Bitwise Exclusive OR
Syntax:
XRL
operand1
,
operand2
Instructions
OpCode
Bytes
Flags
XRL
iram addr
,A
62
2
None
XRL
iram addr
,#
data
63
3
None
XRL A,#
data
64
2
None
XRL A,
iram addr
65
2
None
XRL A,@R0
66
1
None
XRL A,@R1
67
1
None
XRL A,R0
68
1
None
XRL A,R1
69
1
None
XRL A,R2
6A
1
None
XRL A,R3
6B
1
None
XRL A,R4
6C
1
None
XRL A,R5
6D
1
None
XRL A,R6
6E
1
None
XRL A,R7
6F
1
None
Description:
XRL does a bitwise "EXCLUSIVE OR" operation between
operand1
and
operand2
, leaving the resulting value in
operand1
. The value of operand2 is not affected. A logical "EXCLUSIVE OR" compares the bits of each operand and sets the corresponding bit in the resulting byte if the bit was set in either (but not both) of the original operands, otherwise the bit is cleared.
See Also:
ANL
,
ORL

---

## Tables

### Table 1

|  | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 | 0A | 0B | 0C | 0D | 0E | 0F |
| 00 | NOP | AJMP | LJMP | RR | INC | INC | INC | INC | INC | INC | INC | INC | INC | INC | INC | INC |
| 10 | JBC | ACALL | LCALL | RRC | DEC | DEC | DEC | DEC | DEC | DEC | DEC | DEC | DEC | DEC | DEC | DEC |
| 20 | JB | AJMP | RET | RL | ADD | ADD | ADD | ADD | ADD | ADD | ADD | ADD | ADD | ADD | ADD | ADD |
| 30 | JNB | ACALL | RETI | RLC | ADDC | ADDC | ADDC | ADDC | ADDC | ADDC | ADDC | ADDC | ADDC | ADDC | ADDC | ADDC |
| 40 | JC | AJMP | ORL | ORL | ORL | ORL | ORL | ORL | ORL | ORL | ORL | ORL | ORL | ORL | ORL | ORL |
| 50 | JNC | ACALL | ANL | ANL | ANL | ANL | ANL | ANL | ANL | ANL | ANL | ANL | ANL | ANL | ANL | ANL |
| 60 | JZ | AJMP | XRL | XRL | XRL | XRL | XRL | XRL | XRL | XRL | XRL | XRL | XRL | XRL | XRL | XRL |
| 70 | JNZ | ACALL | ORL | JMP | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV |
| 80 | SJMP | AJMP | ANL | MOVC | DIV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV |
| 90 | MOV | ACALL | MOV | MOVC | SUBB | SUBB | SUBB | SUBB | SUBB | SUBB | SUBB | SUBB | SUBB | SUBB | SUBB | SUBB |
| A0 | ORL | AJMP | MOV | INC | MUL | === | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV |
| B0 | ANL | ACALL | CPL | CPL | CJNE | CJNE | CJNE | CJNE | CJNE | CJNE | CJNE | CJNE | CJNE | CJNE | CJNE | CJNE |
| C0 | PUSH | AJMP | CLR | CLR | SWAP | XCH | XCH | XCH | XCH | XCH | XCH | XCH | XCH | XCH | XCH | XCH |
| D0 | POP | ACALL | SETB | SETB | DA | DJNZ | XCHD | XCHD | DJNZ | DJNZ | DJNZ | DJNZ | DJNZ | DJNZ | DJNZ | DJNZ |
| E0 | MOVX | AJMP | MOVX | MOVX | CLR | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV |
| F0 | MOVX | ACALL | MOVX | MOVX | CPL | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV | MOV |

### Table 2

| Operation: | ACALL |
| Function: | Absolute Call Within 2K Block |
| Syntax: | ACALL code address |

### Table 3

| Instructions | OpCode | Bytes | Flags |
| ACALL page0 | 11 | 2 | None |
| ACALL page1 | 31 | 2 | None |
| ACALL page2 | 51 | 2 | None |
| ACALL page3 | 71 | 2 | None |
| ACALL page4 | 91 | 2 | None |
| ACALL page5 | B1 | 2 | None |
| ACALL page6 | D1 | 2 | None |
| ACALL page7 | F1 | 2 | None |

### Table 4

| Operation: | ADD, ADDC |
| Function: | Add Accumulator, Add Accumulator With Carry |
| Syntax: | ADD A,operand |
|  | ADDC A,operand |

### Table 5

| Instructions
OpCode
Bytes
Flags


ADD A,#data
24
2
C, AC, OV


ADD A,iram addr
25
2
C, AC, OV


ADD A,@R0
26
1
C, AC, OV


ADD A,@R1
27
1
C, AC, OV


ADD A,R0
28
1
C, AC, OV


ADD A,R1
29
1
C, AC, OV


ADD A,R2
2A
1
C, AC, OV


ADD A,R3
2B
1
C, AC, OV


ADD A,R4
2C
1
C, AC, OV


ADD A,R5
2D
1
C, AC, OV


ADD A,R6
2E
1
C, AC, OV


ADD A,R7
2F
1
C, AC, OV | Instructions | OpCode | Bytes | Flags | ADD A,#data | 24 | 2 | C, AC, OV | ADD A,iram addr | 25 | 2 | C, AC, OV | ADD A,@R0 | 26 | 1 | C, AC, OV | ADD A,@R1 | 27 | 1 | C, AC, OV | ADD A,R0 | 28 | 1 | C, AC, OV | ADD A,R1 | 29 | 1 | C, AC, OV | ADD A,R2 | 2A | 1 | C, AC, OV | ADD A,R3 | 2B | 1 | C, AC, OV | ADD A,R4 | 2C | 1 | C, AC, OV | ADD A,R5 | 2D | 1 | C, AC, OV | ADD A,R6 | 2E | 1 | C, AC, OV | ADD A,R7 | 2F | 1 | C, AC, OV | Instructions
OpCode
Bytes
Flags


ADDC A,#data
34
2
C, AC, OV


ADDC A,iram addr
35
2
C, AC, OV


ADDC A,@R0
36
1
C, AC, OV


ADDC A,@R1
37
1
C, AC, OV


ADDC A,R0
38
1
C, AC, OV


ADDC A,R1
39
1
C, AC, OV


ADDC A,R2
3A
1
C, AC, OV


ADDC A,R3
3B
1
C, AC, OV


ADDC A,R4
3C
1
C, AC, OV


ADDC A,R5
3D
1
C, AC, OV


ADDC A,R6
3E
1
C, AC, OV


ADDC A,R7
3F
1
C, AC, OV | Instructions | OpCode | Bytes | Flags | ADDC A,#data | 34 | 2 | C, AC, OV | ADDC A,iram addr | 35 | 2 | C, AC, OV | ADDC A,@R0 | 36 | 1 | C, AC, OV | ADDC A,@R1 | 37 | 1 | C, AC, OV | ADDC A,R0 | 38 | 1 | C, AC, OV | ADDC A,R1 | 39 | 1 | C, AC, OV | ADDC A,R2 | 3A | 1 | C, AC, OV | ADDC A,R3 | 3B | 1 | C, AC, OV | ADDC A,R4 | 3C | 1 | C, AC, OV | ADDC A,R5 | 3D | 1 | C, AC, OV | ADDC A,R6 | 3E | 1 | C, AC, OV | ADDC A,R7 | 3F | 1 | C, AC, OV |
| Instructions | OpCode | Bytes | Flags |
| ADD A,#data | 24 | 2 | C, AC, OV |
| ADD A,iram addr | 25 | 2 | C, AC, OV |
| ADD A,@R0 | 26 | 1 | C, AC, OV |
| ADD A,@R1 | 27 | 1 | C, AC, OV |
| ADD A,R0 | 28 | 1 | C, AC, OV |
| ADD A,R1 | 29 | 1 | C, AC, OV |
| ADD A,R2 | 2A | 1 | C, AC, OV |
| ADD A,R3 | 2B | 1 | C, AC, OV |
| ADD A,R4 | 2C | 1 | C, AC, OV |
| ADD A,R5 | 2D | 1 | C, AC, OV |
| ADD A,R6 | 2E | 1 | C, AC, OV |
| ADD A,R7 | 2F | 1 | C, AC, OV |
| Instructions | OpCode | Bytes | Flags |
| ADDC A,#data | 34 | 2 | C, AC, OV |
| ADDC A,iram addr | 35 | 2 | C, AC, OV |
| ADDC A,@R0 | 36 | 1 | C, AC, OV |
| ADDC A,@R1 | 37 | 1 | C, AC, OV |
| ADDC A,R0 | 38 | 1 | C, AC, OV |
| ADDC A,R1 | 39 | 1 | C, AC, OV |
| ADDC A,R2 | 3A | 1 | C, AC, OV |
| ADDC A,R3 | 3B | 1 | C, AC, OV |
| ADDC A,R4 | 3C | 1 | C, AC, OV |
| ADDC A,R5 | 3D | 1 | C, AC, OV |
| ADDC A,R6 | 3E | 1 | C, AC, OV |
| ADDC A,R7 | 3F | 1 | C, AC, OV |

### Table 6

| Instructions | OpCode | Bytes | Flags |
| ADD A,#data | 24 | 2 | C, AC, OV |
| ADD A,iram addr | 25 | 2 | C, AC, OV |
| ADD A,@R0 | 26 | 1 | C, AC, OV |
| ADD A,@R1 | 27 | 1 | C, AC, OV |
| ADD A,R0 | 28 | 1 | C, AC, OV |
| ADD A,R1 | 29 | 1 | C, AC, OV |
| ADD A,R2 | 2A | 1 | C, AC, OV |
| ADD A,R3 | 2B | 1 | C, AC, OV |
| ADD A,R4 | 2C | 1 | C, AC, OV |
| ADD A,R5 | 2D | 1 | C, AC, OV |
| ADD A,R6 | 2E | 1 | C, AC, OV |
| ADD A,R7 | 2F | 1 | C, AC, OV |

### Table 7

| Instructions | OpCode | Bytes | Flags |
| ADDC A,#data | 34 | 2 | C, AC, OV |
| ADDC A,iram addr | 35 | 2 | C, AC, OV |
| ADDC A,@R0 | 36 | 1 | C, AC, OV |
| ADDC A,@R1 | 37 | 1 | C, AC, OV |
| ADDC A,R0 | 38 | 1 | C, AC, OV |
| ADDC A,R1 | 39 | 1 | C, AC, OV |
| ADDC A,R2 | 3A | 1 | C, AC, OV |
| ADDC A,R3 | 3B | 1 | C, AC, OV |
| ADDC A,R4 | 3C | 1 | C, AC, OV |
| ADDC A,R5 | 3D | 1 | C, AC, OV |
| ADDC A,R6 | 3E | 1 | C, AC, OV |
| ADDC A,R7 | 3F | 1 | C, AC, OV |

### Table 8

| Operation: | AJMP |
| Function: | Absolute Jump Within 2K Block |
| Syntax: | AJMP code address |

### Table 9

| Instructions | OpCode | Bytes | Flags |
| AJMP page0 | 01 | 2 | None |
| AJMP page1 | 21 | 2 | None |
| AJMP page2 | 41 | 2 | None |
| AJMP page3 | 61 | 2 | None |
| AJMP page4 | 81 | 2 | None |
| AJMP page5 | A1 | 2 | None |
| AJMP page6 | C1 | 2 | None |
| AJMP page7 | E1 | 2 | None |

### Table 10

| Operation: | ANL |
| Function: | Bitwise AND |
| Syntax: | ANL operand1, operand2 |

### Table 11

| Instructions | OpCode | Bytes | Flags |
| ANL iram addr,A | 52 | 2 | None |
| ANL iram addr,#data | 53 | 3 | None |
| ANL A,#data | 54 | 2 | None |
| ANL A,iram addr | 55 | 2 | None |
| ANL A,@R0 | 56 | 1 | None |
| ANL A,@R1 | 57 | 1 | None |
| ANL A,R0 | 58 | 1 | None |
| ANL A,R1 | 59 | 1 | None |
| ANL A,R2 | 5A | 1 | None |
| ANL A,R3 | 5B | 1 | None |
| ANL A,R4 | 5C | 1 | None |
| ANL A,R5 | 5D | 1 | None |
| ANL A,R6 | 5E | 1 | None |
| ANL A,R7 | 5F | 1 | None |
| ANL C,bit addr | 82 | 2 | C |
| ANL C,/bit addr | B0 | 2 | C |

### Table 12

| Operation: | CJNE |
| Function: | Compare and Jump If Not Equal |
| Syntax: | CJNE operand1,operand2,reladdr |

### Table 13

| Instructions | OpCode | Bytes | Flags |
| CJNE A,#data,reladdr | B4 | 3 | C |
| CJNE A,iram addr,reladdr | B5 | 3 | C |
| CJNE @R0,#data,reladdr | B6 | 3 | C |
| CJNE @R1,#data,reladdr | B7 | 3 | C |
| CJNE R0,#data,reladdr | B8 | 3 | C |
| CJNE R1,#data,reladdr | B9 | 3 | C |
| CJNE R2,#data,reladdr | BA | 3 | C |
| CJNE R3,#data,reladdr | BB | 3 | C |
| CJNE R4,#data,reladdr | BC | 3 | C |
| CJNE R5,#data,reladdr | BD | 3 | C |
| CJNE R6,#data,reladdr | BE | 3 | C |
| CJNE R7,#data,reladdr | BF | 3 | C |

### Table 14

| Operation: | CLR |
| Function: | Clear Register |
| Syntax: | CLR register |

### Table 15

| Instructions | OpCode | Bytes | Flags |
| CLR bit addr | C2 | 2 | None |
| CLR C | C3 | 1 | C |
| CLR A | E4 | 1 | None |

### Table 16

| Operation: | CPL |
| Function: | Complement Register |
| Syntax: | CPL operand |

### Table 17

| Instructions | OpCode | Bytes | Flags |
| CPL A | F4 | 1 | None |
| CPL C | B3 | 1 | C |
| CPL bit addr | B2 | 2 | None |

### Table 18

| Operation: | DA |
| Function: | Decimal Adjust Accumulator |
| Syntax: | DA A |

### Table 19

| Instructions | OpCode | Bytes | Flags |
| DA | D4 | 1 | C |

### Table 20

| Operation: | DEC |
| Function: | Decrement Register |
| Syntax: | DEC register |

### Table 21

| Instructions | OpCode | Bytes | Flags |
| DEC A | 14 | 1 | None |
| DEC iram addr | 15 | 2 | None |
| DEC @R0 | 16 | 1 | None |
| DEC @R1 | 17 | 1 | None |
| DEC R0 | 18 | 1 | None |
| DEC R1 | 19 | 1 | None |
| DEC R2 | 1A | 1 | None |
| DEC R3 | 1B | 1 | None |
| DEC R4 | 1C | 1 | None |
| DEC R5 | 1D | 1 | None |
| DEC R6 | 1E | 1 | None |
| DEC R7 | 1F | 1 | None |

### Table 22

| Operation: | DIV |
| Function: | Divide Accumulator by B |
| Syntax: | DIV AB |

### Table 23

| Instructions | OpCode | Bytes | Flags |
| DIV AB | 84 | 1 | C, OV |

### Table 24

| Operation: | DJNZ |
| Function: | Decrement and Jump if Not Zero |
| Syntax: | DJNZ register,reladdr |

### Table 25

| Instructions | OpCode | Bytes | Flags |
| DJNZ iram addr,reladdr | D5 | 3 | None |
| DJNZ R0,reladdr | D8 | 2 | None |
| DJNZ R1,reladdr | D9 | 2 | None |
| DJNZ R2,reladdr | DA | 2 | None |
| DJNZ R3,reladdr | DB | 2 | None |
| DJNZ R4,reladdr | DC | 2 | None |
| DJNZ R5,reladdr | DD | 2 | None |
| DJNZ R6,reladdr | DE | 2 | None |
| DJNZ R7,reladdr | DF | 2 | None |

### Table 26

| Operation: | INC |
| Function: | Increment Register |
| Syntax: | INC register |

### Table 27

| Instructions | OpCode | Bytes | Flags |
| INC A | 04 | 1 | None |
| INC iram addr | 05 | 2 | None |
| INC @R0 | 06 | 1 | None |
| INC @R1 | 07 | 1 | None |
| INC R0 | 08 | 1 | None |
| INC R1 | 09 | 1 | None |
| INC R2 | 0A | 1 | None |
| INC R3 | 0B | 1 | None |
| INC R4 | 0C | 1 | None |
| INC R5 | 0D | 1 | None |
| INC R6 | 0E | 1 | None |
| INC R7 | 0F | 1 | None |
| INC DPTR | A3 | 1 | None |

### Table 28

| Operation: | JB |
| Function: | Jump if Bit Set |
| Syntax: | JB bit addr, reladdr |

### Table 29

| Instructions | OpCode | Bytes | Flags |
| JB bit addr,reladdr | 20 | 3 | None |

### Table 30

| Operation: | JBC |
| Function: | Jump if Bit Set and Clear Bit |
| Syntax: | JB bit addr, reladdr |

### Table 31

| Instructions | OpCode | Bytes | Flags |
| JBC bit addr,reladdr | 10 | 3 | None |

### Table 32

| Operation: | JC |
| Function: | Jump if Carry Set |
| Syntax: | JC reladdr |

### Table 33

| Instructions | OpCode | Bytes | Flags |
| JC reladdr | 40 | 2 | None |

### Table 34

| Operation: | JMP |
| Function: | Jump to Data Pointer + Accumulator |
| Syntax: | JMP @A+DPTR |

### Table 35

| Instructions | OpCode | Bytes | Flags |
| JMP @A+DPTR | 73 | 1 | None |

### Table 36

| Operation: | JNB |
| Function: | Jump if Bit Not Set |
| Syntax: | JNB bit addr,reladdr |

### Table 37

| Instructions | OpCode | Bytes | Flags |
| JNB bit addr,reladdr | 30 | 3 | None |

### Table 38

| Operation: | JNC |
| Function: | Jump if Carry Not Set |
| Syntax: | JNC reladdr |

### Table 39

| Instructions | OpCode | Bytes | Flags |
| JNC reladdr | 50 | 2 | None |

### Table 40

| Operation: | JNZ |
| Function: | Jump if Accumulator Not Zero |
| Syntax: | JNZ reladdr |

### Table 41

| Instructions | OpCode | Bytes | Flags |
| JNZ reladdr | 70 | 2 | None |

### Table 42

| Operation: | JZ |
| Function: | Jump if Accumulator Zero |
| Syntax: | JNZ reladdr |

### Table 43

| Instructions | OpCode | Bytes | Flags |
| JZ reladdr | 60 | 2 | None |

### Table 44

| Operation: | LCALL |
| Function: | Long Call |
| Syntax: | LCALL code addr |

### Table 45

| Instructions | OpCode | Bytes | Flags |
| LCALL code addr | 12 | 3 | None |

### Table 46

| Operation: | LJMP |
| Function: | Long Jump |
| Syntax: | LJMP code addr |

### Table 47

| Instructions | OpCode | Bytes | Flags |
| LJMP code addr | 02 | 3 | None |

### Table 48

| Operation: | MOV |
| Function: | Move Memory |
| Syntax: | MOV operand1,operand2 |

### Table 49

| Instructions | OpCode | Bytes | Flags |
| MOV @R0,#data | 76 | 2 | None |
| MOV @R1,#data | 77 | 2 | None |
| MOV @R0,A | F6 | 1 | None |
| MOV @R1,A | F7 | 1 | None |
| MOV @R0,iram addr | A6 | 2 | None |
| MOV @R1,iram addr | A7 | 2 | None |
| MOV A,#data | 74 | 2 | None |
| MOV A,@R0 | E6 | 1 | None |
| MOV A,@R1 | E7 | 1 | None |
| MOV A,R0 | E8 | 1 | None |
| MOV A,R1 | E9 | 1 | None |
| MOV A,R2 | EA | 1 | None |
| MOV A,R3 | EB | 1 | None |
| MOV A,R4 | EC | 1 | None |
| MOV A,R5 | ED | 1 | None |
| MOV A,R6 | EE | 1 | None |
| MOV A,R7 | EF | 1 | None |
| MOV A,iram addr | E5 | 2 | None |
| MOV C,bit addr | A2 | 2 | C |
| MOV DPTR,#data16 | 90 | 3 | None |
| MOV R0,#data | 78 | 2 | None |
| MOV R1,#data | 79 | 2 | None |
| MOV R2,#data | 7A | 2 | None |
| MOV R3,#data | 7B | 2 | None |
| MOV R4,#data | 7C | 2 | None |
| MOV R5,#data | 7D | 2 | None |
| MOV R6,#data | 7E | 2 | None |
| MOV R7,#data | 7F | 2 | None |
| MOV R0,A | F8 | 1 | None |
| MOV R1,A | F9 | 1 | None |
| MOV R2,A | FA | 1 | None |
| MOV R3,A | FB | 1 | None |
| MOV R4,A | FC | 1 | None |
| MOV R5,A | FD | 1 | None |
| MOV R6,A | FE | 1 | None |
| MOV R7,A | FF | 1 | None |
| MOV R0,iram addr | A8 | 2 | None |
| MOV R1,iram addr | A9 | 2 | None |
| MOV R2,iram addr | AA | 2 | None |
| MOV R3,iram addr | AB | 2 | None |
| MOV R4,iram addr | AC | 2 | None |
| MOV R5,iram addr | AD | 2 | None |
| MOV R6,iram addr | AE | 2 | None |
| MOV R7,iram addr | AF | 2 | None |
| MOV bit addr,C | 92 | 2 | None |
| MOV iram addr,#data | 75 | 3 | None |
| MOV iram addr,@R0 | 86 | 2 | None |
| MOV iram addr,@R1 | 87 | 2 | None |
| MOV iram addr,R0 | 88 | 2 | None |
| MOV iram addr,R1 | 89 | 2 | None |
| MOV iram addr,R2 | 8A | 2 | None |
| MOV iram addr,R3 | 8B | 2 | None |
| MOV iram addr,R4 | 8C | 2 | None |
| MOV iram addr,R5 | 8D | 2 | None |
| MOV iram addr,R6 | 8E | 2 | None |
| MOV iram addr,R7 | 8F | 2 | None |
| MOV iram addr,A | F5 | 2 | None |
| MOV iram addr,iram addr | 85 | 3 | None |

### Table 50

| Operation: | MOVC |
| Function: | Move Code Byte to Accumulator |
| Syntax: | MOVC A,@A+register |

### Table 51

| Instructions | OpCode | Bytes | Flags |
| MOVC A,@A+DPTR | 93 | 1 | None |
| MOVC A,@A+PC | 83 | 1 | None |

### Table 52

| Operation: | MOVX |
| Function: | Move Data To/From External Memory (XRAM) |
| Syntax: | MOVX operand1,operand2 |

### Table 53

| Instructions | OpCode | Bytes | Flags |
| MOVX @DPTR,A | F0 | 1 | None |
| MOVX @R0,A | F2 | 1 | None |
| MOVX @R1,A | F3 | 1 | None |
| MOVX A,@DPTR | E0 | 1 | None |
| MOVX A,@R0 | E2 | 1 | None |
| MOVX A,@R1 | E3 | 1 | None |

### Table 54

| Operation: | MUL |
| Function: | Multiply Accumulator by B |
| Syntax: | MUL AB |

### Table 55

| Instructions | OpCode | Bytes | Flags |
| MUL AB | A4 | 1 | C, OV |

### Table 56

| Operation: | NOP |
| Function: | None, waste time |
| Syntax: | No Operation |

### Table 57

| Instructions | OpCode | Bytes | Flags |
| NOP | 00 | 1 | None |

### Table 58

| Operation: | ORL |
| Function: | Bitwise OR |
| Syntax: | ORL operand1,operand2 |

### Table 59

| Instructions | OpCode | Bytes | Flags |
| ORL iram addr,A | 42 | 2 | None |
| ORL iram addr,#data | 43 | 3 | None |
| ORL A,#data | 44 | 2 | None |
| ORL A,iram addr | 45 | 2 | None |
| ORL A,@R0 | 46 | 1 | None |
| ORL A,@R1 | 47 | 1 | None |
| ORL A,R0 | 48 | 1 | None |
| ORL A,R1 | 49 | 1 | None |
| ORL A,R2 | 4A | 1 | None |
| ORL A,R3 | 4B | 1 | None |
| ORL A,R4 | 4C | 1 | None |
| ORL A,R5 | 4D | 1 | None |
| ORL A,R6 | 4E | 1 | None |
| ORL A,R7 | 4F | 1 | None |
| ORL C,bit addr | 72 | 2 | C |
| ORL C,/bit addr | A0 | 2 | C |

### Table 60

| Operation: | POP |
| Function: | Pop Value From Stack |
| Syntax: | POP |

### Table 61

| Instructions | OpCode | Bytes | Flags |
| POP iram addr | D0 | 2 | None |

### Table 62

| Operation: | PUSH |
| Function: | Push Value Onto Stack |
| Syntax: | PUSH |

### Table 63

| Instructions | OpCode | Bytes | Flags |
| PUSH iram addr | C0 | 2 | None |

### Table 64

| Operation: | RET |
| Function: | Return From Subroutine |
| Syntax: | RET |

### Table 65

| Instructions | OpCode | Bytes | Flags |
| RET | 22 | 1 | None |

### Table 66

| Operation: | RETI |
| Function: | Return From Interrupt |
| Syntax: | RETI |

### Table 67

| Instructions | OpCode | Bytes | Flags |
| RETI | 32 | 1 | None |

### Table 68

| Operation: | RL |
| Function: | Rotate Accumulator Left |
| Syntax: | RL A |

### Table 69

| Instructions | OpCode | Bytes | Flags |
| RL A | 23 | 1 | C |

### Table 70

| Operation: | RLC |
| Function: | Rotate Accumulator Left Through Carry |
| Syntax: | RLC A |

### Table 71

| Instructions | OpCode | Bytes | Flags |
| RLC A | 33 | 1 | C |

### Table 72

| Operation: | RR |
| Function: | Rotate Accumulator Right |
| Syntax: | RR A |

### Table 73

| Instructions | OpCode | Bytes | Flags |
| RR A | 03 | 1 | None |

### Table 74

| Operation: | RRC |
| Function: | Rotate Accumulator Right Through Carry |
| Syntax: | RRC A |

### Table 75

| Instructions | OpCode | Bytes | Flags |
| RRC A | 13 | 1 | C |

### Table 76

| Operation: | SETB |
| Function: | Set Bit |
| Syntax: | SETB bit addr |

### Table 77

| Instructions | OpCode | Bytes | Flags |
| SETB C | D3 | 1 | C |
| SETB bit addr | D2 | 2 | None |

### Table 78

| Operation: | SJMP |
| Function: | Short Jump |
| Syntax: | SJMP reladdr |

### Table 79

| Instructions | OpCode | Bytes | Flags |
| SJMP reladdr | 80 | 2 | None |

### Table 80

| Operation: | SUBB |
| Function: | Subtract from Accumulator With Borrow |
| Syntax: | SUBB A,operand |

### Table 81

| Instructions | OpCode | Bytes | Flags |
| SUBB A,#data | 94 | 2 | C, AC, OV |
| SUBB A,iram addr | 95 | 2 | C, AC, OV |
| SUBB A,@R0 | 96 | 1 | C, AC, OV |
| SUBB A,@R1 | 97 | 1 | C, AC, OV |
| SUBB A,R0 | 98 | 1 | C, AC, OV |
| SUBB A,R1 | 99 | 1 | C, AC, OV |
| SUBB A,R2 | 9A | 1 | C, AC, OV |
| SUBB A,R3 | 9B | 1 | C, AC, OV |
| SUBB A,R4 | 9C | 1 | C, AC, OV |
| SUBB A,R5 | 9D | 1 | C, AC, OV |
| SUBB A,R6 | 9E | 1 | C, AC, OV |
| SUBB A,R7 | 9F | 1 | C, AC, OV |

### Table 82

| Operation: | SWAP |
| Function: | Swap Accumulator Nibbles |
| Syntax: | SWAP A |

### Table 83

| Instructions | OpCode | Bytes | Flags |
| SWAP A | C4 | 1 | None |

### Table 84

| Operation: | Undefined Instruction |
| Function: | Undefined |
| Syntax: |  |

### Table 85

| Instructions | OpCode | Bytes | Flags |
|  | A5 | 1 | C |

### Table 86

| Operation: | XCH |
| Function: | Exchange Bytes |
| Syntax: | XCH A,register |

### Table 87

| Instructions | OpCode | Bytes | Flags |
| XCH A,@R0 | C6 | 1 | None |
| XCH A,@R1 | C7 | 1 | None |
| XCH A,R0 | C8 | 1 | None |
| XCH A,R1 | C9 | 1 | None |
| XCH A,R2 | CA | 1 | None |
| XCH A,R3 | CB | 1 | None |
| XCH A,R4 | CC | 1 | None |
| XCH A,R5 | CD | 1 | None |
| XCH A,R6 | CE | 1 | None |
| XCH A,R7 | CF | 1 | None |
| XCH A,iram addr | C5 | 2 | None |

### Table 88

| Operation: | XCHD |
| Function: | Exchange Digit |
| Syntax: | XCHD A,[@R0/@R1] |

### Table 89

| Instructions | OpCode | Bytes | Flags |
| XCHD A,@R0 | D6 | 1 | None |
| XCHD A,@R1 | D7 | 1 | None |

### Table 90

| Operation: | XRL |
| Function: | Bitwise Exclusive OR |
| Syntax: | XRL operand1,operand2 |

### Table 91

| Instructions | OpCode | Bytes | Flags |
| XRL iram addr,A | 62 | 2 | None |
| XRL iram addr,#data | 63 | 3 | None |
| XRL A,#data | 64 | 2 | None |
| XRL A,iram addr | 65 | 2 | None |
| XRL A,@R0 | 66 | 1 | None |
| XRL A,@R1 | 67 | 1 | None |
| XRL A,R0 | 68 | 1 | None |
| XRL A,R1 | 69 | 1 | None |
| XRL A,R2 | 6A | 1 | None |
| XRL A,R3 | 6B | 1 | None |
| XRL A,R4 | 6C | 1 | None |
| XRL A,R5 | 6D | 1 | None |
| XRL A,R6 | 6E | 1 | None |
| XRL A,R7 | 6F | 1 | None |

