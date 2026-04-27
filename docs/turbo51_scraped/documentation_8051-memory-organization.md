# Turbo51 Memory Organization | Documentation

**URL:** https://turbo51.com/documentation/8051-memory-organization

---

## Table of Contents

- Turbo51 Memory Organization
  - Pascal Compiler for 8051 microcontrollers
    - CODE memory
    - IDATA memory
    - XDATA memory
    - DATA / IDATA Memory organization
    - XDATA Memory organization
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 microcontrollers
CODE memory
By default the maximum code size is $10000 bytes (64 KB). This can be changed in the main program with the
$M
directive.
IDATA memory
By default there is no IDATA memory. This can be changed either in the main program or in one of the directly or indirectly used units with the
$IDATA
directive. Usualy the main program uses a unit which declares features of the microcontroller including the IDATA memory with the
$IDATA
directive.
XDATA memory
By default there is no XDATA memory. This can be changed either in the main program with the
$M
directive or in one of the used units with the
$XDATA
directive.
DATA / IDATA Memory organization
If compiler switch $I is set then IDATA variables immediately follow DATA varables. If there is no IDATA memory or IDATA variables then stack immediately follows DATA variables. IDATA variables can start below $80 if there is free DATA memory and the $I directive is on.
Register bank 0
$00..$07
Register bank 1
$08..$0F
Register bank 2
$10..$17
Register bank 3
$18..$1F
Boolean variables
$20
Bit-addressable DATA variables
DATA variables
Free DATA memory
$80 - First IDATA address
IDATA variables
StackStart
Last variable (dummy)
Stack
End of DATA / IDATA memory
XDATA Memory organization
Start address and size of XDATA memory and heap size can be set with compiler directive M ($M<CODE Start>, <CODE Size>, <XDATA Start>, <XDATA Size>, <Heap Size>).
XDATA memory start (first byte)
XDATA Variables
XDATA_StackStart
Last XDATA variable (dummy)
XDATA Stack
Free XDATA Stack (first byte)
XSP
Free XDATA Stack
HeapOrg
- first byte of heap
HEAP
Free HEAP (first byte)
HeapPtr
points to the address of unused heap block
Free HEAP
HeapEnd
- XDATA memory end (last byte)

---

## Tables

### Table 1

| Register bank 0 | $00..$07 |
| Register bank 1 | $08..$0F |
| Register bank 2 | $10..$17 |
| Register bank 3 | $18..$1F |
| Boolean variables | $20 |
| Bit-addressable DATA variables |  |
| DATA variables |  |
| Free DATA memory |  |
|  | $80 - First IDATA address |
| IDATA variables |  |
| StackStart | Last variable (dummy) |
| Stack |  |
|  | End of DATA / IDATA memory |

### Table 2

|  | XDATA memory start (first byte) |
| XDATA Variables |  |
| XDATA_StackStart | Last XDATA variable (dummy) |
| XDATA Stack |  |
| Free XDATA Stack (first byte) | XSP |
| Free XDATA Stack |  |
|  | HeapOrg - first byte of heap |
| HEAP |  |
| Free HEAP (first byte) | HeapPtr points to the address of unused heap block |
| Free HEAP |  |
|  | HeapEnd - XDATA memory end (last byte) |

