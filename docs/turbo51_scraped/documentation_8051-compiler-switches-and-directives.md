# Compiler Switches And Directives | Documentation

**URL:** https://turbo51.com/documentation/8051-compiler-switches-and-directives

---

## Table of Contents

- Compiler Switches And Directives
  - Turbo Pascal Compiler for 8051 microcontrollers
    - Microcontroller Books

---

## Content

Turbo Pascal Compiler for 8051 microcontrollers
Compiler switches: {$(letter/switchname)(state)[, (letter/switchname)(state)] } (defaults are shown below)
Compiler switch
Description
A-
AbsoluteInstructions Off
Generate absolute instructions (ACALL/AJMP)
B-
Full boolean evaluation
C+
Show source lines in assembler file
DefaultFile Off
Assume CurrentIO system file variable is assigned with the actual IO procedures
I+
IDATA variables can start below $80 (as indirectly addressed DATA variables)
InlineCode On
If set to Off compiler generates normal call to inline procedure
NoReturn
Inside assembler procedure prevents generation of RET instruction
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
Note: there is no space between switch letter and + or - and there is space between long switch name and On or Off.
Compiler directives: {$ (directive) (value) }
Compiler directive
Description
DEFINE [symbol]
Defines symbol
ELSE
Conditional compilation with IFDEF and IFNDEF
ENDIF
End conditional compilation
IFDEF [symbol]
Conditional compilation if symbol is defined
IFNDEF [symbol]
Conditional compilation if symbol is not defined
IFOPT [switch+/ -]
Conditional compilation if compiler switch is set/not set
M
[CODE Start],
[CODE Size],
[XDATA Start],
[XDATA Size],
[Heap Size]
Memory sizes (only in program), default values:
$0000
$10000
$0000
$0000
$0000
IDATA
IDATA memory available
XDATA
XDATA memory available (only in unit)
HEAP
Heap available (only in unit)
MG [memory type]
Set default memory type for global variables (memory type = DATA, IDATA or XDATA)
ML [memory type]
Set default memory type for local variables (memory type = DATA, IDATA or XDATA)
MP [memory type]
Set default mem. type for parameter variables (memory type = DATA, IDATA or XDATA)
MT [memory type]
Set default mem. type for temporary variables (memory type = DATA, IDATA or XDATA)

---

## Tables

### Table 1

| Compiler switch | Description |
| A- AbsoluteInstructions Off | Generate absolute instructions (ACALL/AJMP) |
| B- | Full boolean evaluation |
| C+ | Show source lines in assembler file |
| DefaultFile Off | Assume CurrentIO system file variable is assigned with the actual IO procedures |
| I+ | IDATA variables can start below $80 (as indirectly addressed DATA variables) |
| InlineCode On | If set to Off compiler generates normal call to inline procedure |
| NoReturn | Inside assembler procedure prevents generation of RET instruction |
| O+ | Optimizations |
| P- | Open string parameters |
| R- | Reentrant procedures |
| T- | Typed pointers |
| U- | Unique local variable names |
| V+ | Strict var-strings |
| X+ | Extended syntax |

### Table 2

| Compiler directive | Description |
| DEFINE [symbol] | Defines symbol |
| ELSE | Conditional compilation with IFDEF and IFNDEF |
| ENDIF | End conditional compilation |
| IFDEF [symbol] | Conditional compilation if symbol is defined |
| IFNDEF [symbol] | Conditional compilation if symbol is not defined |
| IFOPT [switch+/ -] | Conditional compilation if compiler switch is set/not set |
| M[CODE Start],[CODE Size],[XDATA Start],[XDATA Size],[Heap Size] | Memory sizes (only in program), default values:$0000$10000$0000$0000$0000 |
| IDATA | IDATA memory available |
| XDATA | XDATA memory available (only in unit) |
| HEAP | Heap available (only in unit) |
| MG [memory type] | Set default memory type for global variables (memory type = DATA, IDATA or XDATA) |
| ML [memory type] | Set default memory type for local variables (memory type = DATA, IDATA or XDATA) |
| MP [memory type] | Set default mem. type for parameter variables (memory type = DATA, IDATA or XDATA) |
| MT [memory type] | Set default mem. type for temporary variables (memory type = DATA, IDATA or XDATA) |

