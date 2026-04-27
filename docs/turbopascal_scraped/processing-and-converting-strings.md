# Processing and Converting Strings - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-and-converting-strings

---

## Table of Contents

- Processing and Converting Strings

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing and Converting Strings
Turbo Pascal uses few functions and procedures to process, check and convert strings –
Char
type and zero-based character arrays can also be considered as strings.
This function checks if string (or
Char
) is constant and sets expression to typed constant.
Function TExpression.IsConstantString (Size: Word; Var StringLen: Byte): Boolean;
Var StringPtr: PString;
begin
  IsConstantString := False;
  If Location <> elConstant then Exit;
  Case TypeDefPtr^.BaseType of
    btString: begin
                StringPtr := Ptr (DSeg, Value.Word);
                StringLen := Length (StringPtr^);
                If StringLen = 0 then StringLen := 1 else Inc (PChar (StringPtr));
              end;
    btChar: begin
            StringPtr := @Value;
            StringLen := 1;
          end;
    else Exit;
  end;
  If Size <> 0 then If StringLen <> Size then Exit;
  SetExpressionToStringTypedConstant (StringPtr, StringLen);
  IsConstantString := True;
end;
This procedure stores string data to typed constants table and sets expression accordingly.
Procedure TExpression.SetExpressionToStringTypedConstant (Str: Pointer; Size: Word);
begin
  LocationData.Flags := [efTypedConstant, segDS];
  Location := elMemory;
  Value.Offset := StoreConstant (stTypedConstants, Str, TypedConstantsOffsetBeforeConstantValueProcessing, Size) -
                    LastTypedConstantsSize;
  Value.BlockRecord := SymbolTable [stTypedConstantsBlocks].NextRecordOffset;
  Value.Segment := SymbolTable [stMain].Segment;
end;
This procedure stores constant string to code block.
Procedure TExpression.StoreStringConstantToCodeBlock;
Var StringConstant: PString;
begin
  If Location = elConstant then
    begin
      StringConstant := PString (Ptr (DSeg, Value.Word));
      StoreExpressionConstantToCodeBlock (StringConstant, Length (StringConstant^) + 1);
    end;
end;
This procedure checks and converts zero-based character array to string and puts it to stack frame.
Procedure TExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;
Var Len: Byte;
begin
  If not PArrayTypeDefinition (TypeDefPtr)^.IsCharacterArrayCompatibleWithString (Len) then Exit;
  ConvertToPointerAndPush;
  CreateSpaceInStackFrameAndPushAddressToIt ($0100);
  Calculate;
  GenerateCodeToPushWord (Len);
  GenerateInstruction_CALL_FAR (SysProc_SPacked);
  EndIntermediateCodeSubroutine;
  UsedRegisters := [urBX, urDX, urCX, urAX];
  TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
end;
This procedure positions expression to string length byte.
Procedure TExpression.PositionToStringLength;
begin
  Case Location of
    elMemory: begin
         DataType := [itUnsigned];
         TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
       end;
    elConstant: begin
         CheckOrdinalOverflowAndStore (Mem [DSeg : Value.Word], False);
         TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
       end;
    else begin
           PopPointerAndConvertToMemory (Self);
           DataType := [itUnsigned];
           TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         end;
  end;
end;
This procedure converts
Char
type to
String
with one character.
Procedure TExpression.ConvertCharToString;
Var TempString: PString;
begin
  If TypeDefPtr^.BaseType = btChar then
    Case Location of
      elConstant: begin
                    TempString := Ptr (DSeg, GetTempBufferOffset (2));
                    TempString^ := Value.Char;
                    Value.Word := Ofs (TempString^);
                    DataType := [];
                    TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
                  end;
      else begin
             PushExpression;
             CreateSpaceInStackFrameAndPushAddressToIt ($100);
             Calculate;
             GenerateInstruction_CALL_FAR (SysProc_SChar);
             EndIntermediateCodeSubroutine;
             UsedRegisters := [urSP, urDX, urAX];
             TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
           end;
    end;
end;
This function checks if it is possible to convert zero-based character array to
PChar
.
Procedure TExpression.ConvertZeroBasedCharacterArrayToPChar;
begin
  If PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
    begin
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      TypeDefPtr := Ptr (SystemUnitSegment, PChar_TypeOffset);
    end;
end;
This procedure converts zero-based character array to
PChar
.
Procedure TExpression.ConvertZeroBasedCharacterArrayToPChar;
begin
  If PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
    begin
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      TypeDefPtr := Ptr (SystemUnitSegment, PChar_TypeOffset);
    end;
end;
Search
Search
Home
Internals
Download
Compiler
Scanner
Symbol Tables
Parser
Expressions
Calculator
Statements
Assembler
System Functions
System Procedures
Type Definitions
Object Files
Code Generator
Linker
I/O Utilities
About
assembler
block
code
const
declarations
entry
exit
expressions
file
function
identifiers
instructions
integer
intermediate code
label
mathematical operations
new
offset
omf
pointer
procedure
program
read
source
statements
string
symbol table
symbol tables
type
unit

---

## Code Examples

### Code Block 1

```pascal
Function TExpression.IsConstantString (Size: Word; Var StringLen: Byte): Boolean;
Var StringPtr: PString;
begin
  IsConstantString := False;
  If Location <> elConstant then Exit;
  Case TypeDefPtr^.BaseType of
    btString: begin
                StringPtr := Ptr (DSeg, Value.Word);
                StringLen := Length (StringPtr^);
                If StringLen = 0 then StringLen := 1 else Inc (PChar (StringPtr));
              end;
    btChar: begin
            StringPtr := @Value;
            StringLen := 1;
          end;
    else Exit;
  end;
  If Size <> 0 then If StringLen <> Size then Exit;
  SetExpressionToStringTypedConstant (StringPtr, StringLen);
  IsConstantString := True;
end;
```

### Code Block 2

```pascal
Procedure TExpression.SetExpressionToStringTypedConstant (Str: Pointer; Size: Word);
begin
  LocationData.Flags := [efTypedConstant, segDS];
  Location := elMemory;
  Value.Offset := StoreConstant (stTypedConstants, Str, TypedConstantsOffsetBeforeConstantValueProcessing, Size) -
                    LastTypedConstantsSize;
  Value.BlockRecord := SymbolTable [stTypedConstantsBlocks].NextRecordOffset;
  Value.Segment := SymbolTable [stMain].Segment;
end;
```

### Code Block 3

```pascal
Procedure TExpression.StoreStringConstantToCodeBlock;
Var StringConstant: PString;
begin
  If Location = elConstant then
    begin
      StringConstant := PString (Ptr (DSeg, Value.Word));
      StoreExpressionConstantToCodeBlock (StringConstant, Length (StringConstant^) + 1);
    end;
end;
```

### Code Block 4

```pascal
Procedure TExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;
Var Len: Byte;
begin
  If not PArrayTypeDefinition (TypeDefPtr)^.IsCharacterArrayCompatibleWithString (Len) then Exit;
  ConvertToPointerAndPush;
  CreateSpaceInStackFrameAndPushAddressToIt ($0100);
  Calculate;
  GenerateCodeToPushWord (Len);
  GenerateInstruction_CALL_FAR (SysProc_SPacked);
  EndIntermediateCodeSubroutine;
  UsedRegisters := [urBX, urDX, urCX, urAX];
  TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
end;
```

### Code Block 5

```pascal
Procedure TExpression.PositionToStringLength;
begin
  Case Location of
    elMemory: begin
         DataType := [itUnsigned];
         TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
       end;
    elConstant: begin
         CheckOrdinalOverflowAndStore (Mem [DSeg : Value.Word], False);
         TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
       end;
    else begin
           PopPointerAndConvertToMemory (Self);
           DataType := [itUnsigned];
           TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         end;
  end;
end;
```

### Code Block 6

```pascal
Procedure TExpression.ConvertCharToString;
Var TempString: PString;
begin
  If TypeDefPtr^.BaseType = btChar then
    Case Location of
      elConstant: begin
                    TempString := Ptr (DSeg, GetTempBufferOffset (2));
                    TempString^ := Value.Char;
                    Value.Word := Ofs (TempString^);
                    DataType := [];
                    TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
                  end;
      else begin
             PushExpression;
             CreateSpaceInStackFrameAndPushAddressToIt ($100);
             Calculate;
             GenerateInstruction_CALL_FAR (SysProc_SChar);
             EndIntermediateCodeSubroutine;
             UsedRegisters := [urSP, urDX, urAX];
             TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
           end;
    end;
end;
```

### Code Block 7

```pascal
Procedure TExpression.ConvertZeroBasedCharacterArrayToPChar;
begin
  If PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
    begin
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      TypeDefPtr := Ptr (SystemUnitSegment, PChar_TypeOffset);
    end;
end;
```

### Code Block 8

```pascal
Procedure TExpression.ConvertZeroBasedCharacterArrayToPChar;
begin
  If PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
    begin
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      TypeDefPtr := Ptr (SystemUnitSegment, PChar_TypeOffset);
    end;
end;
```

### Code Block 9

```pascal
Function TExpression.IsConstantString (Size: Word; Var StringLen: Byte): Boolean;
Var StringPtr: PString;
begin
  IsConstantString := False;
  If Location <> elConstant then Exit;
  Case TypeDefPtr^.BaseType of
    btString: begin
                StringPtr := Ptr (DSeg, Value.Word);
                StringLen := Length (StringPtr^);
                If StringLen = 0 then StringLen := 1 else Inc (PChar (StringPtr));
              end;
    btChar: begin
            StringPtr := @Value;
            StringLen := 1;
          end;
    else Exit;
  end;
  If Size <> 0 then If StringLen <> Size then Exit;
  SetExpressionToStringTypedConstant (StringPtr, StringLen);
  IsConstantString := True;
end;
```

### Code Block 10

```pascal
Procedure TExpression.SetExpressionToStringTypedConstant (Str: Pointer; Size: Word);
begin
  LocationData.Flags := [efTypedConstant, segDS];
  Location := elMemory;
  Value.Offset := StoreConstant (stTypedConstants, Str, TypedConstantsOffsetBeforeConstantValueProcessing, Size) -
                    LastTypedConstantsSize;
  Value.BlockRecord := SymbolTable [stTypedConstantsBlocks].NextRecordOffset;
  Value.Segment := SymbolTable [stMain].Segment;
end;
```

### Code Block 11

```pascal
Procedure TExpression.StoreStringConstantToCodeBlock;
Var StringConstant: PString;
begin
  If Location = elConstant then
    begin
      StringConstant := PString (Ptr (DSeg, Value.Word));
      StoreExpressionConstantToCodeBlock (StringConstant, Length (StringConstant^) + 1);
    end;
end;
```

### Code Block 12

```pascal
Procedure TExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;
Var Len: Byte;
begin
  If not PArrayTypeDefinition (TypeDefPtr)^.IsCharacterArrayCompatibleWithString (Len) then Exit;
  ConvertToPointerAndPush;
  CreateSpaceInStackFrameAndPushAddressToIt ($0100);
  Calculate;
  GenerateCodeToPushWord (Len);
  GenerateInstruction_CALL_FAR (SysProc_SPacked);
  EndIntermediateCodeSubroutine;
  UsedRegisters := [urBX, urDX, urCX, urAX];
  TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
end;
```

### Code Block 13

```pascal
Procedure TExpression.PositionToStringLength;
begin
  Case Location of
    elMemory: begin
         DataType := [itUnsigned];
         TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
       end;
    elConstant: begin
         CheckOrdinalOverflowAndStore (Mem [DSeg : Value.Word], False);
         TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
       end;
    else begin
           PopPointerAndConvertToMemory (Self);
           DataType := [itUnsigned];
           TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         end;
  end;
end;
```

### Code Block 14

```pascal
Procedure TExpression.ConvertCharToString;
Var TempString: PString;
begin
  If TypeDefPtr^.BaseType = btChar then
    Case Location of
      elConstant: begin
                    TempString := Ptr (DSeg, GetTempBufferOffset (2));
                    TempString^ := Value.Char;
                    Value.Word := Ofs (TempString^);
                    DataType := [];
                    TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
                  end;
      else begin
             PushExpression;
             CreateSpaceInStackFrameAndPushAddressToIt ($100);
             Calculate;
             GenerateInstruction_CALL_FAR (SysProc_SChar);
             EndIntermediateCodeSubroutine;
             UsedRegisters := [urSP, urDX, urAX];
             TypeDefPtr := Ptr (SystemUnitSegment, String_TypeOffset);
           end;
    end;
end;
```

### Code Block 15

```pascal
Procedure TExpression.ConvertZeroBasedCharacterArrayToPChar;
begin
  If PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
    begin
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      TypeDefPtr := Ptr (SystemUnitSegment, PChar_TypeOffset);
    end;
end;
```

### Code Block 16

```pascal
Procedure TExpression.ConvertZeroBasedCharacterArrayToPChar;
begin
  If PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
    begin
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      TypeDefPtr := Ptr (SystemUnitSegment, PChar_TypeOffset);
    end;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-and-converting-strings
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/tag/assembler
- https://turbopascal.org/tag/block
- https://turbopascal.org/tag/code
- https://turbopascal.org/tag/const
- https://turbopascal.org/tag/declarations
- https://turbopascal.org/tag/entry
- https://turbopascal.org/tag/exit
- https://turbopascal.org/tag/expressions
- https://turbopascal.org/tag/file
- https://turbopascal.org/tag/function
- https://turbopascal.org/tag/identifiers
- https://turbopascal.org/tag/instructions
- https://turbopascal.org/tag/integer
- https://turbopascal.org/tag/intermediate-code
- https://turbopascal.org/tag/label
- https://turbopascal.org/tag/mathematical-operations
- https://turbopascal.org/tag/new
- https://turbopascal.org/tag/offset
- https://turbopascal.org/tag/omf
- https://turbopascal.org/tag/pointer
- https://turbopascal.org/tag/procedure
- https://turbopascal.org/tag/program
- https://turbopascal.org/tag/read
- https://turbopascal.org/tag/source
- https://turbopascal.org/tag/statements
- https://turbopascal.org/tag/string
- https://turbopascal.org/tag/symbol-table
- https://turbopascal.org/tag/symbol-tables
- https://turbopascal.org/tag/type
- https://turbopascal.org/tag/unit
- https://turbopascal.org/turbo-pascal-download
- https://turbopascal.org/turbo-pascal-internals
- https://turbopascal.org/type-definitions

