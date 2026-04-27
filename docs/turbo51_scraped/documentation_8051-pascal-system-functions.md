# Turbo51 System Functions | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-system-functions

---

## Table of Contents

- Turbo51 System Functions
  - Pascal Compiler for 8051 Microcontrollers
  - Abs
  - Addr
  - ArcTan
  - Assigned
  - Bcd
  - Chr
  - Concat
  - Copy
  - Cos
  - Exp
  - Frac
  - Hi
  - High
  - Int
  - Length
  - Ln
  - Lo
  - Low
  - MaxAvail
  - MemAvail
  - New
  - Odd
  - Ofs
  - Ord
  - Pi
  - Pos
  - Pred
  - Random
  - Round
  - Sin
  - SizeOf
  - Sqr
  - Sqrt
  - Succ
  - Swap
  - SwapWord
  - Trunc
  - TypeOf
  - UpCase
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Abs
Function Abs (X: Integer): Integer;
Function Abs (X: Real): Real;
Abs returns the absolute value of a variable. The result of the function has the same type as its argument, which can be Integer or Real.
Addr
Function Addr (X: T_DATA_Variable): ShortPtr;
Function Addr (X: T_XDATA_Variable): Pointer;
Function Addr (X: TProcedure): CodePointer;
Addr returns a pointer to its argument, which can be any type including procedure or function. If argument is in DATA segment the result is of type ShortPtr, If argument is in XDATA segment the result is of type Pointer and if argument is in CODE segment (typed constant, function, procedure, static method) the result is of type CodePointer. The returned pointer isn't typed. Similar result can be obtained by the @ operator which returns a typed pointer.
ArcTan
Function Arctan (X: Real): Real;
Arctan returns the Arctangent of X. The resulting angle is in radians.
Assigned
Function Assigned (P: Pointer): Boolean;
Assigned returns True if P is non-nil and retuns False otherwise. P can be any pointer or procedural variable.
Bcd
Function Bcd (D: Byte): Byte;
Bcd returns binary coded decimal representation of D.
Chr
Function Chr (X: Byte): Char;
Chr returns the character which has ASCII value X.
Concat
Function Concat (S1, S2 [,S3, ... ,Sn]): String;
Concat concatenates the strings S1, S2 etc. to one long string. The resulting string is truncated at a length of 255 bytes. The same operation can be performed with the + operation. Function Concat needs XDATA memory.
Copy
Function Copy (Const S: String; Index: Byte; Count: Byte): String;
Copy returns a string which is a copy if the Count characters in S, starting at position Index. If Count is larger than the length of the string S, the result is truncated. If Index is larger than the length of the string S, then an empty string is returned. Function Copy needs XDATA memory.
Cos
Function Cos (X: Real): Real;
Cos returns the cosine of X, where X is an angle in radians.
Exp
Function Exp (Var X: Real): Real;
Exp returns the exponent of X, i.e. the number e to the power X.
Frac
Function Frac (X: Real): Real;
Frac returns the fractional part of a floating point number in X.
Hi
Function Hi (X: Word): Byte;
Function Hi (X: Pointer): Byte;
Hi returns the high byte of word or pointer in X.
High
Function High (TOrdinalType): TOrdinalTypeElement;
Function High (X: TOrdinalType): TOrdinalTypeElement;
Function High (X: TArray): TArrayIndex;
Function High (X: TOpenArray): Integer;
The return value of High depends on it's argument: 1. If the argument is an ordinal type, High returns the highest value in the range of the given ordinal type. 2. If the argument is an array type or an array type variable then High returns the highest possible value of it's index. 3. If the argument is an open array identifier in a function or procedure, then High returns the highest index of the array, as if the array has a zero-based index. The return type is always the same type as the type of the argument (or type of index in arrays).
Int
Function Int (X: Real): Real;
Int returns the integer part of a floating point number in X. The result is Real, i.e. a floating point number.
Length
Function Length (S: String): Byte;
Length returns the length of the string S. If the strings S is empty, 0 is returned. Note: The length of the string S is stored in S [0].
Ln
Function Ln (X: Real): Real;
Ln returns the natural logarithm of the Real parameter X. X must be positive.
Lo
Function Lo (X: Word): Byte;
Function Lo (X: Pointer): Byte;
Lo returns the low byte of word or pointer in X.
Low
Function Low (TOrdinalType): TOrdinalTypeElement;
Function Low (X: TOrdinalType): TOrdinalTypeElement;
Function Low (X: TArray): TArrayIndex;
Function Low (X: TOpenArray): Integer;
The return value of Low depends on it's argument: 1. If the argument is an ordinal type, Low returns the lowest value in the range of the given ordinal type. 2. If the argument is an array type or an array type variable then Low returns the lowest possible value of it's index. 3. If the argument is an open array identifier in a function or procedure, then Low returns the lowest index of the array which is 0. The return type is always the same type as the type of the argument (or type of index in arrays).
MaxAvail
Function MaxAvail: Word;
MaxAvail returns the size in bytes of the biggest free memory block in the heap.
MemAvail
Function MemAvail: Word;
MemAvail returns the size in bytes of all free memory in the heap.
New
Function New (PType);
Function New (PObjectType; Constructor);
New returns address of allocated memory for a new instance of the type PType. If PType is a pointer to a object type, then it is possible to specify the name of the constructor with which the instance will be created.
Odd
Function Odd (X: Longint): Boolean;
Odd returns True if X is odd, or False otherwise.
Ofs
Function Ofs (TRecord.Field): Longint;
Ofs returns offset of Field in record type TRecord.
Ord
Function Ord (X: TOrdinalType): Longint;
Ord returns the ordinal value of a ordinal-type variable X.
Pi
Function Pi: Real;
Pi returns the value of Pi (3.1415926535897932385).
Pos
Function Pos (Const Substr, Str: String): Byte;
Pos returns the index of Substr in Str, if Str contains Substr. In case Substr isn't found, 0 is returned. The search is case-sensitive.
Pred
Function Pred (X: TOrdinalType): TOrdinalType;
Pred returns the element that precedes the element that was passed to it.
Random
Function Random (L: Longint): Longint;
Function Random: Real;
Random returns a random number larger or equal to 0 and strictly less than L. If the argument L is omitted, a Real number between 0 and 1 is returned. (0 included, 1 excluded).
Round
Function Round (X: Real): Longint;
Round rounds X to the closest integer, which may be bigger or smaller than X.
Sin
Function Sin (X: Real): Real;
Sin returns the sine of its argument X, where X is an angle in radians.
SizeOf
Function SizeOf (TAnyType): Longint;
Function SizeOf (X: TAnyType): Longint;
Function SizeOf (TRecord.Field): Longint;
SizeOf returns the size in bytes of any variable, type or record field.
Sqr
Function Sqr (X: Real): Real;
Sqr returns the square of its argument X.
Sqrt
Function Sqrt (X: Real): Real;
Sqrt returns the square root of its argument X, which must be positive.
Succ
Function Succ (X: TOrdinalType): TOrdinalType;
Succ returns the element that succeeds the element that was passed to it.
Swap
Function Swap (X: Word): Word;
Swap swaps the high and low order bytes of X.
SwapWord
Function SwapWord (X: LongInt): LongInt;
SwapWord swaps the high and low order words of X.
Trunc
Function Trunc (X: Real): Longint;
Trunc returns the integer part of X, which is always smaller than (or equal to) X in absolute value.
TypeOf
Function TypeOf (TObjectType): Pointer;
TypeOf returns the address of the VMT of the TObjectType.
UpCase
Function Upcase (C: Char): Char;
UpCase returns the uppercase version of its argument C.

---

## Code Examples

### Code Block 1

```pascal
Function Abs (X: Integer): Integer;
Function Abs (X: Real): Real;

```

### Code Block 2

```pascal
Function Addr (X: T_DATA_Variable): ShortPtr;
Function Addr (X: T_XDATA_Variable): Pointer;
Function Addr (X: TProcedure): CodePointer;

```

### Code Block 3

```pascal
Function Arctan (X: Real): Real;
```

### Code Block 4

```pascal
Function Assigned (P: Pointer): Boolean;
```

### Code Block 5

```pascal
Function Bcd (D: Byte): Byte;
```

### Code Block 6

```pascal
Function Chr (X: Byte): Char;
```

### Code Block 7

```pascal
Function Concat (S1, S2 [,S3, ... ,Sn]): String;
```

### Code Block 8

```pascal
Function Copy (Const S: String; Index: Byte; Count: Byte): String;
```

### Code Block 9

```pascal
Function Cos (X: Real): Real;
```

### Code Block 10

```pascal
Function Exp (Var X: Real): Real;
```

### Code Block 11

```pascal
Function Frac (X: Real): Real;
```

### Code Block 12

```pascal
Function Hi (X: Word): Byte;
Function Hi (X: Pointer): Byte;

```

### Code Block 13

```pascal
Function High (TOrdinalType): TOrdinalTypeElement;
Function High (X: TOrdinalType): TOrdinalTypeElement;
Function High (X: TArray): TArrayIndex;
Function High (X: TOpenArray): Integer;

```

### Code Block 14

```pascal
Function Int (X: Real): Real;
```

### Code Block 15

```pascal
Function Length (S: String): Byte;
```

### Code Block 16

```pascal
Function Ln (X: Real): Real;
```

### Code Block 17

```pascal
Function Lo (X: Word): Byte;
Function Lo (X: Pointer): Byte;

```

### Code Block 18

```pascal
Function Low (TOrdinalType): TOrdinalTypeElement;
Function Low (X: TOrdinalType): TOrdinalTypeElement;
Function Low (X: TArray): TArrayIndex;
Function Low (X: TOpenArray): Integer;

```

### Code Block 19

```pascal
Function MaxAvail: Word;
```

### Code Block 20

```pascal
Function MemAvail: Word;
```

### Code Block 21

```pascal
Function New (PType);
Function New (PObjectType; Constructor);

```

### Code Block 22

```pascal
Function Odd (X: Longint): Boolean;
```

### Code Block 23

```pascal
Function Ofs (TRecord.Field): Longint;
```

### Code Block 24

```pascal
Function Ord (X: TOrdinalType): Longint;
```

### Code Block 25

```pascal
Function Pi: Real;
```

### Code Block 26

```pascal
Function Pos (Const Substr, Str: String): Byte;
```

### Code Block 27

```pascal
Function Pred (X: TOrdinalType): TOrdinalType;
```

### Code Block 28

```pascal
Function Random (L: Longint): Longint;
Function Random: Real;

```

### Code Block 29

```pascal
Function Round (X: Real): Longint;
```

### Code Block 30

```pascal
Function Sin (X: Real): Real;
```

### Code Block 31

```pascal
Function SizeOf (TAnyType): Longint;
Function SizeOf (X: TAnyType): Longint;
Function SizeOf (TRecord.Field): Longint;

```

### Code Block 32

```pascal
Function Sqr (X: Real): Real;
```

### Code Block 33

```pascal
Function Sqrt (X: Real): Real;
```

### Code Block 34

```pascal
Function Succ (X: TOrdinalType): TOrdinalType;
```

### Code Block 35

```pascal
Function Swap (X: Word): Word;
```

### Code Block 36

```pascal
Function SwapWord (X: LongInt): LongInt;
```

### Code Block 37

```pascal
Function Trunc (X: Real): Longint;
```

### Code Block 38

```pascal
Function TypeOf (TObjectType): Pointer;
```

### Code Block 39

```pascal
Function Upcase (C: Char): Char;
```

