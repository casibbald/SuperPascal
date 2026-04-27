# Reentrant Procedures in Turbo51 | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-reentrant-procedures

---

## Table of Contents

- Reentrant Procedures in Turbo51
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
For reentrant procedures all parameters are pushed on XDATA stack. Functions return simple result in first register set (R5R4R3R2). For functions which return String result calle must reserve space on XDATA stack and push its address (see picture below). Before a call to the reentrant procedure is made the following is pushed on the XDATA stack:
Address for String result (for functions which return String)
All parameters in order in which they were declared
XBP if calling local procedure
Called procedure on entry:
pushes XBP
sets XBP to point to the top of pushed parameters
reserves space for local variables (increases XSP accordingly)
On exit called procedure pops saved XBP and removes all pushed parameters from XDATA stack. XDATA stack during reentrant procedure call looks like this:
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
Result variable
(for functions which return simple type)
XBP
Local variables and
temporary storage
XDATA STACK
high memory
XSP

---

## Tables

### Table 1

| XDATA STACKlow memory |  |
| Pushed address for String result(for functions which return String) |  |
| Pushed parameters |  |
| Outer procedure's XBP(if there is outer procedure) | XBP - 4 |
| Saved XBP | XBP - 2 |
| Result variable(for functions which return simple type) | XBP |
| Local variables andtemporary storage |  |
| XDATA STACKhigh memory | XSP |

