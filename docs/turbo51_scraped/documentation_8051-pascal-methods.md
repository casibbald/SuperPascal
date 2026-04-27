# Turbo51 Methods | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-methods

---

## Table of Contents

- Turbo51 Methods
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Methods are always
reentrant
. Before a call to the method is made the following is pushed on the XDATA stack:
Address for String result (for functions which return String)
All parameters in order in which they were declared
On call to static method the following parameters must be in registers:
R3R2: Self address
On call to virtual method the following parameters must be in registers:
DPTR: Self address (will be placed in R3R2 by system routine for virtual method call)
R2 (or R3R2): Offset of VMT address
R0 (or R1R0): Offset of method address
R5R4: VMT parameter
On call to dynamic method the following parameters must be in registers:
DPTR: Self address (will be placed in R3R2 by system routine for dynamic method call)
R2 (R3R2): Offset of DMT address
R1: Dynamic method index
R5R4: VMT parameter
On call to constructor method the following parameters must be in registers:
R3R2: Self address (If nil, constructor call is via New)
R5R4: VMT parameter (address of VMT - normal call, $0000 means static call -no initialization of VMT address in Self)
Returns Self (address of allocated object) in R3R2
On call to destructor method the following parameters must be in registers:
R3R2: Self address
R4: VMT parameter ($00: normal destructor call, $01: call via Dispose)
Called method on entry:
pushes XBP
sets XBP to point to the top of pushed parameters
pushes Self parameter which was passed in R3R2
pushes VMT parameter which was passed in R5R4
reserves space for local variables (increases XSP accordingly)
On exit called method pops saved XBP and removes all pushed parameters from XDATA stack. XDATA stack during reentrant procedure call looks like this:
XDATA STACK
low memory
Pushed address for String result
(for functions which return String)
Pushed parameters
Outer procedure's XBP
(if there is outer procedure)
XBP - 4
Saved XBP
XBP - 2
Self address
XBP
VMT parameter
XBP + 2
Result variable
(for functions which return simple type
XBP + 4
Local variables and
temporary storage
XDATA STACK
high memory
XSP

---

## Tables

### Table 1

|  |  |
| XDATA STACKlow memory |  |
| Pushed address for String result(for functions which return String) |  |
| Pushed parameters |  |
| Outer procedure's XBP(if there is outer procedure) | XBP - 4 |
| Saved XBP | XBP - 2 |
| Self address | XBP |
| VMT parameter | XBP + 2 |
| Result variable(for functions which return simple type | XBP + 4 |
| Local variables andtemporary storage |  |
| XDATA STACKhigh memory | XSP |

