# Turbo51 Command Line Syntax | Documentation

**URL:** https://turbo51.com/documentation/8051-compiler-syntax

---

## Table of Contents

- Turbo51 Command Line Syntax
  - How To Use the Compiler
    - Command line syntax
    - Command line options
    - Example of Turbo51 console output:
    - Syntax
    - Reserved words
    - Directives
    - Microcontroller Books

---

## Content

How To Use the Compiler
Turbo51 is a command line console application. This means that it has no graphical user interface, menus or windows. It must be run from a console with parameters: pascal source file to compile and optional switches. Optionally you can run Turbo51 from some Integrated Development Environment or editor which supports external applications - see
IDE
page for details. It is always a good idea to use the -M option to recompile modified units to u51 file. The -M switch recompiles only units that need to be recompiled, either their source was modified or one of their used units was modified in interface section or one of the used include files was changed. This is the fastest way of compilation. Optionally you can force to recompile all used units with the -B switch. The command line syntax is the following:
Command line syntax
Turbo51 [options] filename [options]
Command line options
Options: -<letter><optional parameters>
Option
Description
-A
Generate assembler file
-B
Build
-C
Show error column number
-D
Define conditionals
-E
BIN/HEX/U51/OMF output directory
-F
Find source line at address
-G
Generate map file
-H
Generate Intel HEX file
-I
Include file directories
-J
Object file directories
-LA
Use library Turbo51A.l51 (compiled with $A+, ACALL/AJMP instructions)
-M
Make modified units
-MG
Set default memory type for global variables (memory type = D, I or X)
-ML
Set default memory type for local variables (memory type = D, I or X)
-MP
Set default memory type for parameter variables (memory type = D, I or X)
-MT
Set default memory type for temporary variables (memory type = D, I or X)
-O
Generate OMF-51 file
-OX
Generate extended OMF-51 file (needed for source-level debugging)
-Q
Quiet compile
-S
Syntax check
-T
L51/CFG directory
-U
Unit file directories
-$
Command line compiler switch
It is a good idea to use command line option /M to compile used units only when they or files they depend on were changed. This can speed up the compilation. You can force Turbo51 to rebuild all used units with command line option /B. Each time the unit is compiled Turbo51 generates a file 'UnitName.u51'. This file is used next time when the main file is compiled. Without using command line options /M or /B Turbo51 will each time compile all used units without making the compiled unit files (u51).
Command line compiler switches: -$(defaults are shown below)
Compiler switch
Description
A-
Generate absolute instructions (ACALL/AJMP)
B-
Full boolean evaluation
C+
Show source lines in assembler file
I+
IDATA variables can start below $80 (as indirectly addressed DATA variables)
O+
Optimizations
P-
Open string parameters
R-
Reentrant procedures
T-
Typed pointers
U-
Unique local variable names
V+
Strict var-strings
X+
Extended syntax
Example of Turbo51 console output:
Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa
D:\Projects\Turbo51\Demo\Demo.pas (36)
D:\Projects\Turbo51\Demo\Constants.inc (1362)
D:\Projects\Turbo51\Demo\Demo.pas (4436)
5798 lines, 0.79 seconds, 7275 lines/s

CODE  size: $50CA  (20682 bytes)
DATA  size:   $4E  (78 bytes, $20 to $6D, including 19 bits)
IDATA size:   $61  (97 bytes, $6E to $CE)
STACK size:   $31  (49 bytes)
XDATA size: $0F79  (3961 bytes)
Syntax
Turbo51 uses most of the Borland Turbo Pascal 7 syntax including OOP and some additional directives and constructs to support specific features of 8051 family (MCS-51). Check
Turbo Pascal 7.0 Documentation
page for Turbo Pascal User's Guide, Turbo Pascal Language Guide and Turbo Pascal Programmers Reference.
Reserved words
AND, ARRAY, ASM, BEGIN, CASE, CONST, CONSTRUCTOR, DESTRUCTOR, DIV, DO, DOWNTO, ELSE, END, FILE, FOR, FUNCTION, GOTO, IF, IMPLEMENTATION, IN, INHERITED, INTERFACE, LABEL, MOD, NIL, NOT, OBJECT, OF, OR, PACKED, PROCEDURE, PROGRAM, RECORD, REPEAT, SET, SHL, SHR, STRING, THEN, TO, TYPE, UNIT, UNTIL, USES, VAR, WHILE, WITH, XOR
Directives
ABSOLUTE, ASSEMBLER, BITADDRESSABLE, CODE, DATA, EXTERNAL, FORWARD, IDATA, INLINE, INTERRUPT, PRIVATE, PUBLIC, REENTRANT, USING, USINGANY, VIRTUAL, VOLATILE, XDATA

---

## Code Examples

### Code Block 1

```pascal
Turbo51 version 0.1.3.10, Copyright 2000 - 2011 Igor Funa
D:\Projects\Turbo51\Demo\Demo.pas (36)
D:\Projects\Turbo51\Demo\Constants.inc (1362)
D:\Projects\Turbo51\Demo\Demo.pas (4436)
5798 lines, 0.79 seconds, 7275 lines/s

CODE  size: $50CA  (20682 bytes)
DATA  size:   $4E  (78 bytes, $20 to $6D, including 19 bits)
IDATA size:   $61  (97 bytes, $6E to $CE)
STACK size:   $31  (49 bytes)
XDATA size: $0F79  (3961 bytes)

```

## Tables

### Table 1

| Option | Description |
| -A | Generate assembler file |
| -B | Build |
| -C | Show error column number |
| -D | Define conditionals |
| -E | BIN/HEX/U51/OMF output directory |
| -F | Find source line at address |
| -G | Generate map file |
| -H | Generate Intel HEX file |
| -I | Include file directories |
| -J | Object file directories |
| -LA | Use library Turbo51A.l51 (compiled with $A+, ACALL/AJMP instructions) |
| -M | Make modified units |
| -MG | Set default memory type for global variables (memory type = D, I or X) |
| -ML | Set default memory type for local variables (memory type = D, I or X) |
| -MP | Set default memory type for parameter variables (memory type = D, I or X) |
| -MT | Set default memory type for temporary variables (memory type = D, I or X) |
| -O | Generate OMF-51 file |
| -OX | Generate extended OMF-51 file (needed for source-level debugging) |
| -Q | Quiet compile |
| -S | Syntax check |
| -T | L51/CFG directory |
| -U | Unit file directories |
| -$ | Command line compiler switch |

### Table 2

| Compiler switch | Description |
| A- | Generate absolute instructions (ACALL/AJMP) |
| B- | Full boolean evaluation |
| C+ | Show source lines in assembler file |
| I+ | IDATA variables can start below $80 (as indirectly addressed DATA variables) |
| O+ | Optimizations |
| P- | Open string parameters |
| R- | Reentrant procedures |
| T- | Typed pointers |
| U- | Unique local variable names |
| V+ | Strict var-strings |
| X+ | Extended syntax |

