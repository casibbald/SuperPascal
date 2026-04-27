# Turbo51 System Procedures | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-system-procedures

---

## Table of Contents

- Turbo51 System Procedures
  - Pascal Compiler for 8051 Microcontrollers
  - Assign
  - BlockRead
  - BlockWrite
  - Break
  - Change
  - Continue
  - Dec
  - Delete
  - Dispose
  - Exclude
  - Exit
  - ExitBlock
  - Fail
  - FillChar
  - FreeMem
  - GetMem
  - Halt
  - Inc
  - Include
  - Insert
  - Mark
  - Move
  - New
  - Randomize
  - Read
  - Readln
  - Release
  - Str
  - Val
  - Write
  - Writeln
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Assign
Procedure Assign (Var F: File; ReadFunction: Function; WriteProc: Procedure);
Procedure Assign assigns read function and write procedure to file variable F. Either ReadFunction or WriteProc can be omitted. Read function must be a non-reentrant function with no parameters which returns Char or Byte result (result must be returned in register A - default for Turbo51 pascal functions) and MUST preserve registers R2, R3, R4, R5, R8, R9. WriteProc must be a non-reentrant procedure with no parameters and MUST preserve registers R2, R3, R6, R7. Character to write is passed to procedure in register A. If the WriteProc is written in pascal then it must first save character to some local storage (short asm statement at the beginning of procedure).
BlockRead
Procedure BlockRead (Var F: File; Var Buffer; Count: Word);
Procedure BlockRead reads Count bytes from file F to Buffer. Files are read by the ReadFunction that is assigned to file F.
BlockWrite
Procedure BlockWrite (Var F: File; Var Buffer; Count: Word);
Procedure BlockWrite writes Count bytes from Buffer to file F. Bytes are written by the WriteProcedure that is assigned to file F.
Break
Procedure Break;
Break jumps to the statement following the end of the current loop statement. The code between the Break call and the end of the loop statement is skipped. This can be used with For, Repeat and While statements.
Change
Procedure Change (S: TSetOfElement; Element: TOrdinalType);
Change changes inclusion of Element in the set S (If element is included in the set the procedure performs Exclude and Include otherwise).
Continue
Procedure Continue;
Continue jumps to the end of the current loop statement. The code between the Continue call and the end of the loop statement is skipped. This can be used with For, Repeat and While statements.
Dec
Procedure Dec (Var X: OrdinalType);
Procedure Dec (Var X: OrdinalType; Decrement: Longint);
Dec decrements the value of X with Decrement. If Decrement isn't specified, then 1 is taken as a default.
Delete
Procedure Delete (Var S: String; Index: Byte; Count: Byte);
Delete deletes Count characters from string S, starting at position Index. All characters after the delected characters are shifted Count positions to the left, and the length of the string is adjusted.
Dispose
Procedure Dispose (P: Pointer);
Procedure Dispose (P: TypedPointer; Destuct: Procedure);
The first form Dispose releases the memory allocated with a call to New. The released memory is returned to the heap. The second form of Dispose accepts as a first parameter a pointer to an object type, and as a second parameter the name of a destructor of this object. The destructor will be called, and the memory allocated for the object will be freed.
Exclude
Procedure Exclude (S: TSetOfElement; Element: TOrdinalType);
Exclude excludes Element from the set S.
Exit
Procedure Exit;
Exit exits the current procedure or function and returns control to the calling routine.
ExitBlock
Procedure ExitBlock;
ExitBlock exits the current begin-end block and returns control to the statement after this begin-end block.
Fail
Procedure Fail;
Fail exits the constructor with nil value.
FillChar
Procedure Fillchar (Var Mem; Count: Word; Value: Char);
Fillchar fills the memory starting at Mem with Count characters with value equal to Value.
FreeMem
Procedure FreeMem (Var Ptr: Pointer; Count: Word);
FreeMem releases the memory occupied by the pointer Ptr, of size Count (in bytes), and returns it to the heap. Ptr should point to the memory allocated to a dynamic variable with procedure GetMem.
GetMem
Procedure GetMem (Var Ptr: Pointer; Size: Word);
Getmem reserves Size bytes memory on the heap, and returns a pointer to this memory in Ptr. If no more memory is available, nil is returned.
Halt
Procedure Halt;
Halt generates code for endless loop (i.e. jump to itself).
Inc
Procedure Inc (Var X: OrdinalType);
Procedure Inc (Var X: OrdinalType; Increment: Longint);
Inc increments the value of X with Increment. If Increment isn't specified, then 1 is taken as a default.
Include
Procedure Include (S: TSetOfElement; Element: TOrdinalType);
Include includes Element to the set S.
Insert
Procedure Insert (Const Source: String; Var DestStr: String; Index: Byte);
Insert inserts string Source in string DestStr, at position Index, shifting all characters after Index to the right. The resulting string is truncated at 255 characters, if needed.
Mark
Procedure Mark (Var Ptr: Pointer);
Mark copies the current heap-pointer HeapPtr to Ptr.
Move
Procedure Move (Var Source, Dest; Count: Word);
Move moves Count bytes from Source to Dest.
New
Procedure New (Var Ptr: Pointer);
Procedure New (Var Ptr: PObject; Constructor);
New allocates a new instance of the type pointed to by Ptr, and puts the address in Ptr. If Ptr is a pointer to a object, then it is possible to specify the name of the constructor with which the instance will be created.
Randomize
Procedure Randomize;
Randomize initializes the random number generator of Turbo51, by giving a value to RandSeed, calculated with the system clock.
Read
Procedure Read ([Var F: File, ] V1 [, V2, ... , Vn]);
Read reads one or more values from a file F, and stores the result in V1, V2, etc. If no file F is specified, then standard input is read. If F is a typed file, then each of the variables must be of the type specified in the declaration of F.
Readln
Procedure Readln ([Var F: File, ] V1 [, V2, ... , Vn]);
Read reads one or more values from a file F, and stores the result in V1, V2, etc. After that it goes to the next line in the file (defined by the LineFeed (#10) character). If no file F is specified, then standard input is read. If F is a typed file, then each of the variables must be of the type specified in the declaration of F. Untyped files are not allowed as an argument.
Release
Procedure Release (Ptr: Pointer);
Release sets the top of the heap to the location pointed to by Ptr. All memory at a location higher than Ptr is marked empty.
Str
Procedure Str (Var X[: NumPlaces[:Decimals]]; Var Str: String);
Str returns a string which represents the value of X. X can be any numerical type. The optional NumPlaces and Decimals specifiers control the formatting of the string.
Val
Procedure Val (Const Str: String; Var V; Var ErrorCode: Integer);
Val converts the value represented in the string Str to a numerical value, and stores this value in the variable V, which can be of type Longint or Real. If the conversion isn't succesfull, then the parameter ErrorCode contains the index of the character in S which prevented the conversion. The string S isn't allowed to contain spaces.
Write
Procedure Write ([Var F: File, ] V1 [, V2, ... , Vn]);
Write writes the contents of the variables V1, V2 etc. to the file F. F can be a typed file, or a Text file. If F is a typed file, then the variables V1, V2 etc. must be of the same type as the type in the declaration of F. Untyped files are not allowed. If the parameter F is omitted, standard output is assumed (system file variable Output which is alias of text file SystemIO).
If F is of type Text, then the necessary conversions are done such that the output of the variables is in character format. This conversion is done for all numerical types. Strings are printed exactly as they are in memory, as well as PChar types. The format of the numerical conversions can be influenced through the following modifiers: OutputVariable: NumChars [: Decimals ] This will print the value of OutputVariable with a minimum of NumChars characters, from which Decimals are reserved for the decimals. If the number cannot be represented with NumChars characters, NumChars will be increased, until the representation fits. If the representation requires less than NumChars characters then the output is filled up with spaces, to the left of the generated string, thus resulting in a right-aligned representation. If no formatting is specified, then the number is written using its natural length, with nothing in front of it if it's positive, and a minus sign if it's negative. Real numbers are, by default, written in scientific notation.
Writeln
Procedure Writeln ([Var F: File, ] V1 [, V2, ... , Vn]);
Writeln does the same as Write for text files, and writes a Carriage Return - LineFeed character pair (#13#10) after that. If the parameter F is omitted, standard output is assumed (system text variable Output which is alias of text file SystemIO). If no variables are specified, a Carriage Return - LineFeed character pair is written.

---

## Code Examples

### Code Block 1

```pascal
Procedure Assign (Var F: File; ReadFunction: Function; WriteProc: Procedure);
```

### Code Block 2

```pascal
Procedure BlockRead (Var F: File; Var Buffer; Count: Word);
```

### Code Block 3

```pascal
Procedure BlockWrite (Var F: File; Var Buffer; Count: Word);
```

### Code Block 4

```pascal
Procedure Break;
```

### Code Block 5

```pascal
Procedure Change (S: TSetOfElement; Element: TOrdinalType);
```

### Code Block 6

```pascal
Procedure Continue;
```

### Code Block 7

```pascal
Procedure Dec (Var X: OrdinalType);
Procedure Dec (Var X: OrdinalType; Decrement: Longint);

```

### Code Block 8

```pascal
Procedure Delete (Var S: String; Index: Byte; Count: Byte);
```

### Code Block 9

```pascal
Procedure Dispose (P: Pointer);
Procedure Dispose (P: TypedPointer; Destuct: Procedure);

```

### Code Block 10

```pascal
Procedure Exclude (S: TSetOfElement; Element: TOrdinalType);
```

### Code Block 11

```pascal
Procedure Exit;
```

### Code Block 12

```pascal
Procedure ExitBlock;
```

### Code Block 13

```pascal
Procedure Fail;
```

### Code Block 14

```pascal
Procedure Fillchar (Var Mem; Count: Word; Value: Char);
```

### Code Block 15

```pascal
Procedure FreeMem (Var Ptr: Pointer; Count: Word);
```

### Code Block 16

```pascal
Procedure GetMem (Var Ptr: Pointer; Size: Word);
```

### Code Block 17

```pascal
Procedure Halt;
```

### Code Block 18

```pascal
Procedure Inc (Var X: OrdinalType);
Procedure Inc (Var X: OrdinalType; Increment: Longint);

```

### Code Block 19

```pascal
Procedure Include (S: TSetOfElement; Element: TOrdinalType);
```

### Code Block 20

```pascal
Procedure Insert (Const Source: String; Var DestStr: String; Index: Byte);
```

### Code Block 21

```pascal
Procedure Mark (Var Ptr: Pointer);
```

### Code Block 22

```pascal
Procedure Move (Var Source, Dest; Count: Word);
```

### Code Block 23

```pascal
Procedure New (Var Ptr: Pointer);
Procedure New (Var Ptr: PObject; Constructor);

```

### Code Block 24

```pascal
Procedure Randomize;
```

### Code Block 25

```pascal
Procedure Read ([Var F: File, ] V1 [, V2, ... , Vn]);
```

### Code Block 26

```pascal
Procedure Readln ([Var F: File, ] V1 [, V2, ... , Vn]);
```

### Code Block 27

```pascal
Procedure Release (Ptr: Pointer);
```

### Code Block 28

```pascal
Procedure Str (Var X[: NumPlaces[:Decimals]]; Var Str: String);
```

### Code Block 29

```pascal
Procedure Val (Const Str: String; Var V; Var ErrorCode: Integer);
```

### Code Block 30

```pascal
Procedure Write ([Var F: File, ] V1 [, V2, ... , Vn]);
```

### Code Block 31

```pascal
Procedure Writeln ([Var F: File, ] V1 [, V2, ... , Vn]);
```

