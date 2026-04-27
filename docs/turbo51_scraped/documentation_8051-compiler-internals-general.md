# Turbo51 Compiler Internals | Documentation

**URL:** https://turbo51.com/documentation/8051-compiler-internals-general

---

## Table of Contents

- Turbo51 Compiler Internals
  - Pascal Compiler for 8051 Microcontrollers
    - Data storage
    - Boolean variables
    - Global variables
    - Local variables
    - Parameters
    - Register usage
    - XDATA Stack
    - Calls to normal (non-reentrant) procedures and functions
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
If you are interested in Turbo Pascal compiler internals and would like to see the source code of some popular commercial Pascal compiler then check
Turbo Pascal Compiler Written in Turbo Pascal
.
Data storage
All variables and typed constants are stored in little endian format.
Boolean variables
Boolean variables are stored as bits in bit-addressable DATA memory wich is available in all 8051 derivatives. Boolean variables can not be passed by reference (8051 has no instruction to reference bit variable by address) and can not be passed as parameter in re-entrant procedures. In such cases you can use system type
ByteBool
which occupies 1 byte.
Global variables
Global variables are placed in default memory type for global variables which can be set with compiler directive $MG MemoryType (DATA, IDATA or XDATA) and defaults to DATA. This memory type can be overridden for each variable declaration.
Local variables
All local variables in normal (non-reentrant) procedures and functions are static. They are placed like global variables but are accessible only in local scope. Default memory type for local variables can be set with compiler directive $ML MemoryType (DATA, IDATA or XDATA) and defaults to DATA. This memory type can be overridden for each variable declaration.
Parameters
All parameters in normal (non-reentrant) procedures and functions are stored as local variables and are static. Default memory type for parameters can be set with compiler directive $MP MemoryType (DATA, IDATA or XDATA) and defaults to DATA. This memory type can be overridden for each parameter declaration.
Register usage
Turbo51 internally uses two register sets: R5R4R3R2 and R9R8R7R6. R8 and R9 are ordinary DATA variables declared in System unit. 8-bit data is stored in R2 (R6), 16-bit data is stored in R3R2 (R7R6) and 32-bit data uses whole set.
XDATA Stack
When XDATA memory is present Turbo51 creates there a stack. XSP (Pointer declared in System unit) points to to the top of stack.
Calls to normal (non-reentrant) procedures and functions
For normal, non-reentrant procedures all parameters are stored as local variables. Caller passes data by storing it to local memory for parameters and calls procedure. Functions return simple result in either ACC or first register set (R5R4R3R2). String functions return pointer to result string in R0 (if it is in DATA or IDATA memory) or in DPTR otherwise (result is in CODE or XDATA memory). Currently this is the only supported calling convention.
In most cases there is no need for reentrant procedures. This avoids using XDATA stack and greatly simplyfies generated code. Try to avoid reentrant procedures unless they are really needed.

---

