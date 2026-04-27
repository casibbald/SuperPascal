# Error Handling - Turbo Pascal Compiler

**URL:** https://turbopascal.org/error-handling

---

## Table of Contents

- Error Handling

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Error Handling
Compiler should have the possibility to report error from any function. Turbo Pascal solves this problem by defining procedure
SetErrorAddress
which saves the stack position and return address.
When error occurs the
Error
procedure restores stack pointers and jumps to predefined return address.
Procedure
SetErrorAddress
saves registers
BP
and
SP
and sets
ReturnAddress
. This is a routine that is called immediately after the error occured and just before the control flow returns to the
main program
.
Procedure SetErrorAddress (ErrAddr: Pointer);
begin
  ReturnAddress := Ptr (Seg (ErrAddr^), Ofs (ErrAddr^) + 3); { skip PUSH  BP     }
                                                             {      MOV   BP, SP }
                                                             { in ErrorProc      }
  LastError := NoError;
  AdditionalErrorStr := nil;
  ErrorSourceFile := nil;
  SavedHeapPtrSeg := HeapPtrRec.Seg;
  SavedSP := SPtr;
  SavedBP := MemW [SSeg : SavedSP];
  Inc (SavedSP, $0E);
end;
Procedure
Error
restores registers and calls previously defined
ReturnAddress
and returns to the main program.
Procedure Error (Err: TCompilerError);
begin
  CloseOpenFiles;
  LastError := Err;
  HeapPtrRec.Seg := SavedHeapPtrSeg;
  Asm
    MOV   AX, SavedSP
    MOV   SP, AX
    MOV   AX, SavedBP
    MOV   BP, AX
    LEA   DI, ReturnAddress
    MOV   AX, [DI + 2]
    PUSH  AX
    MOV   AX, [DI]
    PUSH  AX
    RETF
  end;
end;
This is error procedure used by the compiler in the main
Compile
procedure:
Procedure CompilerError; Far;
begin
  If CurrentSourceFile <> @EndOfFileStructure then
    begin
      ErrorSourceFile := CurrentSourceFile;
      CurrentSourceFile^.CurrentPosition := Ofs (ErrorSourcePosition^);
    end;
  If OutputFileHandle <> 0 then DeleteFile (@Identifier);
  If ExeFileHandle <> 0 then DeleteFile (ExeFileName);
end;
Error messages used in the Turbo Pascal compiler:
Type TCompilerError = (
       TargetAddressFound,
       OutOfMemory,
       IdentifierExpected,
       UnknownIdentifier,
       DuplicateIdentifier,
       SyntaxError,
       ErrorInRealConstant,
       ErrorInIntegerConstant,
       StringConstantExceedsLine,
       Error_09,
       UnexpectedEndOfFile,
       LineTooLong,
       TypeIdentifierExpected,
       TooManyOpenFiles,
       InvalidFilename,
       FileNotFound,
       DiskFull,
       InvalidCompilerDirective,
       TooManyFiles,
       UndefinedTypeInPointerDefinition,
       VariableIdentifierExpected,
       ErrorInType,
       StructureTooLarge,
       SetBaseTypeOutOfRange,
       FileComponentsMayNotBeFilesOrObjects,
       InvalidStringLength,
       TypeMismatch,
       InvalidSubrangeBaseType,
       LowerBoundGreaterThanUpperBound,
       OrdinalTypeExpected,
       IntegerConstantExpected,
       ConstantExpected,
       IntegerOrRealCconstantExpected,
       PointerTypeIdentifierExpected,
       InvalidFunctionResultType,
       LabelIdentifierExpected,
       BEGIN_expected,
       END_expected,
       IntegerExpressionExpected,
       OrdinalExpressionExpected,
       BooleanExpressionExpected,
       OperandTypesDoNotMatchOperator,
       ErrorInExpression,
       IllegalAssignment,
       FieldIdentifierExpected,
       ObjectFileTooLarge,
       UndefinedExternal,
       InvalidObjectFileRecord,
       CodeSegmentTooLarge,
       DataSegmentTooLarge,
       DO_expected,
       Invalid_PUBLIC_definition,
       Invalid_EXTRN_definition,
       TooMany_EXTRN_definitions,
       OF_expected,
       INTERFACE_expected,
       InvalidRelocatableReference,
       THEN_expected,
       TO_or_DOWNTO_expected,
       UndefinedForward,
       Error_3C,
       InvalidTypeCast,
       DivisionByZero,
       InvalidFileType,
       CannotReadOrWriteVariablesOfThisType,
       PointerVariableExpected,
       StringVariableExpected,
       StringExpressionExpected,
       CircularUnitReference,
       UnitNameMismatch,
       UnitVersionMismatch,
       InternalStackOverflow,
       UnitFileFormatError,
       IMPLEMENTATION_expected,
       ConstantAnd_CASE_typesDoNotMatch,
       RecordOrObjectVariableExpected,
       ConstantOutOfRange,
       FileVariableExpected,
       PointerExpressionExpected,
       IntegerOrRealExpressionExpected,
       LabelNotWithinCurrentBlock,
       LabelAlreadyDefined,
       UndefinedLabelInPrecedingStatementPart,
       Invalid_At_argument,
       UNIT_expected,
       SemicolonExpected,
       ColonExpected,
       CommaExpected,
       LeftParenthesisExpected,
       RightParenthesisExpected,
       EqualExpected,
       AssignmentExpected,
       LeftBracketExpected,
       RightBracketExpected,
       PeriodExpected,
       PeriodPeriodExpected,
       TooManyVariables,
       Invalid_FOR_controlVariable,
       IntegerVariableExpected,
       FileTypesAreNotAllowedHere,
       StringLengthMismatch,
       InvalidOrderingOfFields,
       StringConstantExpected,
       IntegerOrRealVariableExpected,
       OrdinalVariableExpected,
       INLINE_error,
       CharacterExpressionExpected,
       TooManyRelocationItems,
       OverflowInArithmeticOperation,
       NoEnclosing_FOR_WHILE_or_REPEAT_statement,
       DebugInformationTableOverflow,
       CompilationAborted,
       CASE_constantOutOfRange,
       ErrorInStatement,
       CannotCallAnInterruptProcedure,
       Error_73,
       MustBeIn8087ModeToCompileThis,
       TargetAddressNotFound,
       IncludeFilesAreNotAllowedHere,
       NoInheritedMethodsAreAccessibleHere,
       Error_78,
       InvalidQualifier,
       InvalidVariableReference,
       TooManySymbols,
       StatementPartTooLarge,
       Error_7D,
       FilesMustBeVarParameters,
       TooManyConditionalSymbols,
       MisplacedConditionalDirective,
       ENDIF_directiveMissing,
       ErrorInInitialConditionalDdefines,
       HeaderDoesNotMatchPreviousDefinition,
       Error_84,
       CannotEvaluateThisExpression,
       ExpressionIncorrectlyTerminated,
       InvalidFormatSpecifier,
       InvalidIndirectReference,
       StructuredVariablesAreNotAllowedHere,
       CannotEvaluateWwithoutSystemUnit,
       CannotAaccessThisSymbol,
       InvalidFloatingPointOperation,
       CannotCompileOverlaysToMemory,
       PointerOrProceduralVariableExpected,
       InvalidProcedureOrFunctionReference,
       CannotOverlayThisUnit,
       TooManyNestedScopes,
       FileAccessDenied,
       ObjectTypeExpected,
       LocalObjectTypesAreNotAllowed,
       VIRTUAL_expected,
       MethodIdentifierExpected,
       VirtualConstructorsAreNotAllowed,
       ConstructorIdentifierExpected,
       DestructorIdentifierExpected,
       FailOnlyAllowedWithinConstructors,
       InvalidCombinationOfOpcodeAndOperands,
       MemoryReferenceExpected,
       CannotAddOrSubtractRelocatableSymbols,
       InvalidRegisterCombination,
       _286_287_instructionsAreNotEnabled,
       InvalidSymbolReference,
       CodeGenerationError,
       ASM_expected,
       DuplicateDynamicMethodIndex,
       DuplcateResourceIdentifier,
       DuplicateOrInvalidExportIndex,
       ProcedureOrFunctionIdentifierExpected,
       CannotExportThisSymbol,
       DuplicateExportName,
       ExecutableFileHeaderTooLarge,
       TooManySegments);

Const

  NoError = TargetAddressFound;

  SymbolTableError: Array [stMain..stIntermediateCode] of TCompilerError = (
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    CodeSegmentTooLarge,
    DataSegmentTooLarge,
    CodeSegmentTooLarge,
    DataSegmentTooLarge,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    StatementPartTooLarge);

  CompilerErrorString: Array [TCompilerError] of PChar = (
    'Target address found',
    'Out of memory',
    'Identifier expected',
    'Unknown identifier',
    'Duplicate identifier',
    'Syntax error',
    'Error in real constant',
    'Error in integer constant',
    'String constant exceeds line',
    '',
    'Unexpected end of file',
    'Line too long',
    'Type identifier expected',
    'Too many open files',
    'Invalid filename',
    'File not found',
    'Disk full',
    'Invalid compiler directive',
    'Too many files',
    'Undefined type in pointer definition',
    'Variable identifier expected',
    'Error in type',
    'Structure too large',
    'Set base type out of range',
    'File components may not be files or objects',
    'Invalid string length',
    'Type mismatch',
    'Invalid subrange base type',
    'Lower bound greater than upper bound',
    'Ordinal type expected',
    'Integer constant expected',
    'Constant expected',
    'Integer or real constant expected',
    'Pointer type identifier expected',
    'Invalid function result type',
    'Label identifier expected',
    'BEGIN expected',
    'END expected',
    'Integer expression expected',
    'Ordinal expression expected',
    'Boolean expression expected',
    'Operand types do not match operator',
    'Error in expression',
    'Illegal assignment',
    'Field identifier expected',
    'Object file too large',
    'Undefined external',
    'Invalid object file record',
    'Code segment too large',
    'Data segment too large',
    'DO expected',
    'Invalid PUBLIC definition',
    'Invalid EXTRN definition',
    'Too many EXTRN definitions',
    'OF expected',
    'INTERFACE expected',
    'Invalid relocatable reference',
    'THEN expected',
    'TO or DOWNTO expected',
    'Undefined forward',
    '',
    'Invalid type cast',
    'Division by zero',
    'Invalid file type',
    'Cannot Read or Write variables of this type',
    'Pointer variable expected',
    'String variable expected',
    'String expression expected',
    'Circular unit reference',
    'Unit name mismatch',
    'Unit version mismatch',
    'Internal stack overflow',
    'Unit file format error',
    'IMPLEMENTATION expected',
    'Constant and CASE types do not match',
    'Record or object variable expected',
    'Constant out of range',
    'File variable expected',
    'Pointer expression expected',
    'Integer or real expression expected',
    'Label not within current block',
    'Label already defined',
    'Undefined label in preceding statement part',
    'Invalid @ argument',
    'UNIT expected',
    '";" expected',
    '":" expected',
    '"," expected',
    '"(" expected',
    '")" expected',
    '"=" expected',
    '":=" expected',
    '"[" or "(." expected',
    '"]" or ".)" expected',
    '"." expected',
    '".." expected',
    'Too many variables',
    'Invalid FOR control variable',
    'Integer variable expected',
    'File types are not allowed here',
    'String length mismatch',
    'Invalid ordering of fields',
    'String constant expected',
    'Integer or real variable expected',
    'Ordinal variable expected',
    'INLINE error',
    'Character expression expected',
    'Too many relocation items',
    'Overflow in arithmetic operation',
    'No enclosing FOR, WHILE, or REPEAT statement',
    'Debug information table overflow',
    'Compilation aborted',
    'CASE constant out of range',
    'Error in statement',
    'Cannot call an interrupt procedure',
    '',
    'Must be in 8087 mode to compile this',
    'Target address not found',
    'Include files are not allowed here',
    'No inherited methods are accessible here',
    '',
    'Invalid qualifier',
    'Invalid variable reference',
    'Too many symbols',
    'Statement part too large',
    '',
    'Files must be var parameters',
    'Too many conditional symbols',
    'Misplaced conditional directive',
    'ENDIF directive missing',
    'Error in initial conditional defines',
    'Header does not match previous definition',
    '',
    'Cannot evaluate this expression',
    'Expression incorrectly terminated',
    'Invalid format specifier',
    'Invalid indirect reference',
    'Structured variables are not allowed here',
    'Cannot evaluate without System unit',
    'Cannot access this symbol',
    'Invalid floating point operation',
    'Cannot compile overlays to memory',
    'Pointer or procedural variable expected',
    'Invalid procedure or function reference',
    'Cannot overlay this unit',
    'Too many nested scopes',
    'File access denied',
    'Object type expected',
    'Local object types are not allowed',
    'VIRTUAL expected',
    'Method identifier expected',
    'Virtual constructors are not allowed',
    'Constructor identifier expected',
    'Destructor identifier expected',
    'Fail only allowed within constructors',
    'Invalid combination of opcode and operands',
    'Memory reference expected',
    'Cannot add or subtract relocatable symbols',
    'Invalid register combination',
    '286/287 instructions are not enabled',
    'Invalid symbol reference',
    'Code generation error',
    'ASM expected',
    'Duplicate dynamic method index',
    'Duplcate resource identifier',
    'Duplicate or invalid export index',
    'Procedure or function identifier expected',
    'Cannot export this symbol',
    'Duplicate export name',
    'Executable file header too large',
    'Too many segments');
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
Procedure SetErrorAddress (ErrAddr: Pointer);
begin
  ReturnAddress := Ptr (Seg (ErrAddr^), Ofs (ErrAddr^) + 3); { skip PUSH  BP     }
                                                             {      MOV   BP, SP }
                                                             { in ErrorProc      }
  LastError := NoError;
  AdditionalErrorStr := nil;
  ErrorSourceFile := nil;
  SavedHeapPtrSeg := HeapPtrRec.Seg;
  SavedSP := SPtr;
  SavedBP := MemW [SSeg : SavedSP];
  Inc (SavedSP, $0E);
end;
```

### Code Block 2

```pascal
Procedure Error (Err: TCompilerError);
begin
  CloseOpenFiles;
  LastError := Err;
  HeapPtrRec.Seg := SavedHeapPtrSeg;
  Asm
    MOV   AX, SavedSP
    MOV   SP, AX
    MOV   AX, SavedBP
    MOV   BP, AX
    LEA   DI, ReturnAddress
    MOV   AX, [DI + 2]
    PUSH  AX
    MOV   AX, [DI]
    PUSH  AX
    RETF
  end;
end;
```

### Code Block 3

```pascal
Procedure CompilerError; Far;
begin
  If CurrentSourceFile <> @EndOfFileStructure then
    begin
      ErrorSourceFile := CurrentSourceFile;
      CurrentSourceFile^.CurrentPosition := Ofs (ErrorSourcePosition^);
    end;
  If OutputFileHandle <> 0 then DeleteFile (@Identifier);
  If ExeFileHandle <> 0 then DeleteFile (ExeFileName);
end;
```

### Code Block 4

```pascal
Type TCompilerError = (
       TargetAddressFound,
       OutOfMemory,
       IdentifierExpected,
       UnknownIdentifier,
       DuplicateIdentifier,
       SyntaxError,
       ErrorInRealConstant,
       ErrorInIntegerConstant,
       StringConstantExceedsLine,
       Error_09,
       UnexpectedEndOfFile,
       LineTooLong,
       TypeIdentifierExpected,
       TooManyOpenFiles,
       InvalidFilename,
       FileNotFound,
       DiskFull,
       InvalidCompilerDirective,
       TooManyFiles,
       UndefinedTypeInPointerDefinition,
       VariableIdentifierExpected,
       ErrorInType,
       StructureTooLarge,
       SetBaseTypeOutOfRange,
       FileComponentsMayNotBeFilesOrObjects,
       InvalidStringLength,
       TypeMismatch,
       InvalidSubrangeBaseType,
       LowerBoundGreaterThanUpperBound,
       OrdinalTypeExpected,
       IntegerConstantExpected,
       ConstantExpected,
       IntegerOrRealCconstantExpected,
       PointerTypeIdentifierExpected,
       InvalidFunctionResultType,
       LabelIdentifierExpected,
       BEGIN_expected,
       END_expected,
       IntegerExpressionExpected,
       OrdinalExpressionExpected,
       BooleanExpressionExpected,
       OperandTypesDoNotMatchOperator,
       ErrorInExpression,
       IllegalAssignment,
       FieldIdentifierExpected,
       ObjectFileTooLarge,
       UndefinedExternal,
       InvalidObjectFileRecord,
       CodeSegmentTooLarge,
       DataSegmentTooLarge,
       DO_expected,
       Invalid_PUBLIC_definition,
       Invalid_EXTRN_definition,
       TooMany_EXTRN_definitions,
       OF_expected,
       INTERFACE_expected,
       InvalidRelocatableReference,
       THEN_expected,
       TO_or_DOWNTO_expected,
       UndefinedForward,
       Error_3C,
       InvalidTypeCast,
       DivisionByZero,
       InvalidFileType,
       CannotReadOrWriteVariablesOfThisType,
       PointerVariableExpected,
       StringVariableExpected,
       StringExpressionExpected,
       CircularUnitReference,
       UnitNameMismatch,
       UnitVersionMismatch,
       InternalStackOverflow,
       UnitFileFormatError,
       IMPLEMENTATION_expected,
       ConstantAnd_CASE_typesDoNotMatch,
       RecordOrObjectVariableExpected,
       ConstantOutOfRange,
       FileVariableExpected,
       PointerExpressionExpected,
       IntegerOrRealExpressionExpected,
       LabelNotWithinCurrentBlock,
       LabelAlreadyDefined,
       UndefinedLabelInPrecedingStatementPart,
       Invalid_At_argument,
       UNIT_expected,
       SemicolonExpected,
       ColonExpected,
       CommaExpected,
       LeftParenthesisExpected,
       RightParenthesisExpected,
       EqualExpected,
       AssignmentExpected,
       LeftBracketExpected,
       RightBracketExpected,
       PeriodExpected,
       PeriodPeriodExpected,
       TooManyVariables,
       Invalid_FOR_controlVariable,
       IntegerVariableExpected,
       FileTypesAreNotAllowedHere,
       StringLengthMismatch,
       InvalidOrderingOfFields,
       StringConstantExpected,
       IntegerOrRealVariableExpected,
       OrdinalVariableExpected,
       INLINE_error,
       CharacterExpressionExpected,
       TooManyRelocationItems,
       OverflowInArithmeticOperation,
       NoEnclosing_FOR_WHILE_or_REPEAT_statement,
       DebugInformationTableOverflow,
       CompilationAborted,
       CASE_constantOutOfRange,
       ErrorInStatement,
       CannotCallAnInterruptProcedure,
       Error_73,
       MustBeIn8087ModeToCompileThis,
       TargetAddressNotFound,
       IncludeFilesAreNotAllowedHere,
       NoInheritedMethodsAreAccessibleHere,
       Error_78,
       InvalidQualifier,
       InvalidVariableReference,
       TooManySymbols,
       StatementPartTooLarge,
       Error_7D,
       FilesMustBeVarParameters,
       TooManyConditionalSymbols,
       MisplacedConditionalDirective,
       ENDIF_directiveMissing,
       ErrorInInitialConditionalDdefines,
       HeaderDoesNotMatchPreviousDefinition,
       Error_84,
       CannotEvaluateThisExpression,
       ExpressionIncorrectlyTerminated,
       InvalidFormatSpecifier,
       InvalidIndirectReference,
       StructuredVariablesAreNotAllowedHere,
       CannotEvaluateWwithoutSystemUnit,
       CannotAaccessThisSymbol,
       InvalidFloatingPointOperation,
       CannotCompileOverlaysToMemory,
       PointerOrProceduralVariableExpected,
       InvalidProcedureOrFunctionReference,
       CannotOverlayThisUnit,
       TooManyNestedScopes,
       FileAccessDenied,
       ObjectTypeExpected,
       LocalObjectTypesAreNotAllowed,
       VIRTUAL_expected,
       MethodIdentifierExpected,
       VirtualConstructorsAreNotAllowed,
       ConstructorIdentifierExpected,
       DestructorIdentifierExpected,
       FailOnlyAllowedWithinConstructors,
       InvalidCombinationOfOpcodeAndOperands,
       MemoryReferenceExpected,
       CannotAddOrSubtractRelocatableSymbols,
       InvalidRegisterCombination,
       _286_287_instructionsAreNotEnabled,
       InvalidSymbolReference,
       CodeGenerationError,
       ASM_expected,
       DuplicateDynamicMethodIndex,
       DuplcateResourceIdentifier,
       DuplicateOrInvalidExportIndex,
       ProcedureOrFunctionIdentifierExpected,
       CannotExportThisSymbol,
       DuplicateExportName,
       ExecutableFileHeaderTooLarge,
       TooManySegments);

Const

  NoError = TargetAddressFound;

  SymbolTableError: Array [stMain..stIntermediateCode] of TCompilerError = (
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    CodeSegmentTooLarge,
    DataSegmentTooLarge,
    CodeSegmentTooLarge,
    DataSegmentTooLarge,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    StatementPartTooLarge);

  CompilerErrorString: Array [TCompilerError] of PChar = (
    'Target address found',
    'Out of memory',
    'Identifier expected',
    'Unknown identifier',
    'Duplicate identifier',
    'Syntax error',
    'Error in real constant',
    'Error in integer constant',
    'String constant exceeds line',
    '',
    'Unexpected end of file',
    'Line too long',
    'Type identifier expected',
    'Too many open files',
    'Invalid filename',
    'File not found',
    'Disk full',
    'Invalid compiler directive',
    'Too many files',
    'Undefined type in pointer definition',
    'Variable identifier expected',
    'Error in type',
    'Structure too large',
    'Set base type out of range',
    'File components may not be files or objects',
    'Invalid string length',
    'Type mismatch',
    'Invalid subrange base type',
    'Lower bound greater than upper bound',
    'Ordinal type expected',
    'Integer constant expected',
    'Constant expected',
    'Integer or real constant expected',
    'Pointer type identifier expected',
    'Invalid function result type',
    'Label identifier expected',
    'BEGIN expected',
    'END expected',
    'Integer expression expected',
    'Ordinal expression expected',
    'Boolean expression expected',
    'Operand types do not match operator',
    'Error in expression',
    'Illegal assignment',
    'Field identifier expected',
    'Object file too large',
    'Undefined external',
    'Invalid object file record',
    'Code segment too large',
    'Data segment too large',
    'DO expected',
    'Invalid PUBLIC definition',
    'Invalid EXTRN definition',
    'Too many EXTRN definitions',
    'OF expected',
    'INTERFACE expected',
    'Invalid relocatable reference',
    'THEN expected',
    'TO or DOWNTO expected',
    'Undefined forward',
    '',
    'Invalid type cast',
    'Division by zero',
    'Invalid file type',
    'Cannot Read or Write variables of this type',
    'Pointer variable expected',
    'String variable expected',
    'String expression expected',
    'Circular unit reference',
    'Unit name mismatch',
    'Unit version mismatch',
    'Internal stack overflow',
    'Unit file format error',
    'IMPLEMENTATION expected',
    'Constant and CASE types do not match',
    'Record or object variable expected',
    'Constant out of range',
    'File variable expected',
    'Pointer expression expected',
    'Integer or real expression expected',
    'Label not within current block',
    'Label already defined',
    'Undefined label in preceding statement part',
    'Invalid @ argument',
    'UNIT expected',
    '";" expected',
    '":" expected',
    '"," expected',
    '"(" expected',
    '")" expected',
    '"=" expected',
    '":=" expected',
    '"[" or "(." expected',
    '"]" or ".)" expected',
    '"." expected',
    '".." expected',
    'Too many variables',
    'Invalid FOR control variable',
    'Integer variable expected',
    'File types are not allowed here',
    'String length mismatch',
    'Invalid ordering of fields',
    'String constant expected',
    'Integer or real variable expected',
    'Ordinal variable expected',
    'INLINE error',
    'Character expression expected',
    'Too many relocation items',
    'Overflow in arithmetic operation',
    'No enclosing FOR, WHILE, or REPEAT statement',
    'Debug information table overflow',
    'Compilation aborted',
    'CASE constant out of range',
    'Error in statement',
    'Cannot call an interrupt procedure',
    '',
    'Must be in 8087 mode to compile this',
    'Target address not found',
    'Include files are not allowed here',
    'No inherited methods are accessible here',
    '',
    'Invalid qualifier',
    'Invalid variable reference',
    'Too many symbols',
    'Statement part too large',
    '',
    'Files must be var parameters',
    'Too many conditional symbols',
    'Misplaced conditional directive',
    'ENDIF directive missing',
    'Error in initial conditional defines',
    'Header does not match previous definition',
    '',
    'Cannot evaluate this expression',
    'Expression incorrectly terminated',
    'Invalid format specifier',
    'Invalid indirect reference',
    'Structured variables are not allowed here',
    'Cannot evaluate without System unit',
    'Cannot access this symbol',
    'Invalid floating point operation',
    'Cannot compile overlays to memory',
    'Pointer or procedural variable expected',
    'Invalid procedure or function reference',
    'Cannot overlay this unit',
    'Too many nested scopes',
    'File access denied',
    'Object type expected',
    'Local object types are not allowed',
    'VIRTUAL expected',
    'Method identifier expected',
    'Virtual constructors are not allowed',
    'Constructor identifier expected',
    'Destructor identifier expected',
    'Fail only allowed within constructors',
    'Invalid combination of opcode and operands',
    'Memory reference expected',
    'Cannot add or subtract relocatable symbols',
    'Invalid register combination',
    '286/287 instructions are not enabled',
    'Invalid symbol reference',
    'Code generation error',
    'ASM expected',
    'Duplicate dynamic method index',
    'Duplcate resource identifier',
    'Duplicate or invalid export index',
    'Procedure or function identifier expected',
    'Cannot export this symbol',
    'Duplicate export name',
    'Executable file header too large',
    'Too many segments');
```

### Code Block 5

```pascal
Procedure SetErrorAddress (ErrAddr: Pointer);
begin
  ReturnAddress := Ptr (Seg (ErrAddr^), Ofs (ErrAddr^) + 3); { skip PUSH  BP     }
                                                             {      MOV   BP, SP }
                                                             { in ErrorProc      }
  LastError := NoError;
  AdditionalErrorStr := nil;
  ErrorSourceFile := nil;
  SavedHeapPtrSeg := HeapPtrRec.Seg;
  SavedSP := SPtr;
  SavedBP := MemW [SSeg : SavedSP];
  Inc (SavedSP, $0E);
end;
```

### Code Block 6

```pascal
Procedure Error (Err: TCompilerError);
begin
  CloseOpenFiles;
  LastError := Err;
  HeapPtrRec.Seg := SavedHeapPtrSeg;
  Asm
    MOV   AX, SavedSP
    MOV   SP, AX
    MOV   AX, SavedBP
    MOV   BP, AX
    LEA   DI, ReturnAddress
    MOV   AX, [DI + 2]
    PUSH  AX
    MOV   AX, [DI]
    PUSH  AX
    RETF
  end;
end;
```

### Code Block 7

```pascal
Procedure CompilerError; Far;
begin
  If CurrentSourceFile <> @EndOfFileStructure then
    begin
      ErrorSourceFile := CurrentSourceFile;
      CurrentSourceFile^.CurrentPosition := Ofs (ErrorSourcePosition^);
    end;
  If OutputFileHandle <> 0 then DeleteFile (@Identifier);
  If ExeFileHandle <> 0 then DeleteFile (ExeFileName);
end;
```

### Code Block 8

```pascal
Type TCompilerError = (
       TargetAddressFound,
       OutOfMemory,
       IdentifierExpected,
       UnknownIdentifier,
       DuplicateIdentifier,
       SyntaxError,
       ErrorInRealConstant,
       ErrorInIntegerConstant,
       StringConstantExceedsLine,
       Error_09,
       UnexpectedEndOfFile,
       LineTooLong,
       TypeIdentifierExpected,
       TooManyOpenFiles,
       InvalidFilename,
       FileNotFound,
       DiskFull,
       InvalidCompilerDirective,
       TooManyFiles,
       UndefinedTypeInPointerDefinition,
       VariableIdentifierExpected,
       ErrorInType,
       StructureTooLarge,
       SetBaseTypeOutOfRange,
       FileComponentsMayNotBeFilesOrObjects,
       InvalidStringLength,
       TypeMismatch,
       InvalidSubrangeBaseType,
       LowerBoundGreaterThanUpperBound,
       OrdinalTypeExpected,
       IntegerConstantExpected,
       ConstantExpected,
       IntegerOrRealCconstantExpected,
       PointerTypeIdentifierExpected,
       InvalidFunctionResultType,
       LabelIdentifierExpected,
       BEGIN_expected,
       END_expected,
       IntegerExpressionExpected,
       OrdinalExpressionExpected,
       BooleanExpressionExpected,
       OperandTypesDoNotMatchOperator,
       ErrorInExpression,
       IllegalAssignment,
       FieldIdentifierExpected,
       ObjectFileTooLarge,
       UndefinedExternal,
       InvalidObjectFileRecord,
       CodeSegmentTooLarge,
       DataSegmentTooLarge,
       DO_expected,
       Invalid_PUBLIC_definition,
       Invalid_EXTRN_definition,
       TooMany_EXTRN_definitions,
       OF_expected,
       INTERFACE_expected,
       InvalidRelocatableReference,
       THEN_expected,
       TO_or_DOWNTO_expected,
       UndefinedForward,
       Error_3C,
       InvalidTypeCast,
       DivisionByZero,
       InvalidFileType,
       CannotReadOrWriteVariablesOfThisType,
       PointerVariableExpected,
       StringVariableExpected,
       StringExpressionExpected,
       CircularUnitReference,
       UnitNameMismatch,
       UnitVersionMismatch,
       InternalStackOverflow,
       UnitFileFormatError,
       IMPLEMENTATION_expected,
       ConstantAnd_CASE_typesDoNotMatch,
       RecordOrObjectVariableExpected,
       ConstantOutOfRange,
       FileVariableExpected,
       PointerExpressionExpected,
       IntegerOrRealExpressionExpected,
       LabelNotWithinCurrentBlock,
       LabelAlreadyDefined,
       UndefinedLabelInPrecedingStatementPart,
       Invalid_At_argument,
       UNIT_expected,
       SemicolonExpected,
       ColonExpected,
       CommaExpected,
       LeftParenthesisExpected,
       RightParenthesisExpected,
       EqualExpected,
       AssignmentExpected,
       LeftBracketExpected,
       RightBracketExpected,
       PeriodExpected,
       PeriodPeriodExpected,
       TooManyVariables,
       Invalid_FOR_controlVariable,
       IntegerVariableExpected,
       FileTypesAreNotAllowedHere,
       StringLengthMismatch,
       InvalidOrderingOfFields,
       StringConstantExpected,
       IntegerOrRealVariableExpected,
       OrdinalVariableExpected,
       INLINE_error,
       CharacterExpressionExpected,
       TooManyRelocationItems,
       OverflowInArithmeticOperation,
       NoEnclosing_FOR_WHILE_or_REPEAT_statement,
       DebugInformationTableOverflow,
       CompilationAborted,
       CASE_constantOutOfRange,
       ErrorInStatement,
       CannotCallAnInterruptProcedure,
       Error_73,
       MustBeIn8087ModeToCompileThis,
       TargetAddressNotFound,
       IncludeFilesAreNotAllowedHere,
       NoInheritedMethodsAreAccessibleHere,
       Error_78,
       InvalidQualifier,
       InvalidVariableReference,
       TooManySymbols,
       StatementPartTooLarge,
       Error_7D,
       FilesMustBeVarParameters,
       TooManyConditionalSymbols,
       MisplacedConditionalDirective,
       ENDIF_directiveMissing,
       ErrorInInitialConditionalDdefines,
       HeaderDoesNotMatchPreviousDefinition,
       Error_84,
       CannotEvaluateThisExpression,
       ExpressionIncorrectlyTerminated,
       InvalidFormatSpecifier,
       InvalidIndirectReference,
       StructuredVariablesAreNotAllowedHere,
       CannotEvaluateWwithoutSystemUnit,
       CannotAaccessThisSymbol,
       InvalidFloatingPointOperation,
       CannotCompileOverlaysToMemory,
       PointerOrProceduralVariableExpected,
       InvalidProcedureOrFunctionReference,
       CannotOverlayThisUnit,
       TooManyNestedScopes,
       FileAccessDenied,
       ObjectTypeExpected,
       LocalObjectTypesAreNotAllowed,
       VIRTUAL_expected,
       MethodIdentifierExpected,
       VirtualConstructorsAreNotAllowed,
       ConstructorIdentifierExpected,
       DestructorIdentifierExpected,
       FailOnlyAllowedWithinConstructors,
       InvalidCombinationOfOpcodeAndOperands,
       MemoryReferenceExpected,
       CannotAddOrSubtractRelocatableSymbols,
       InvalidRegisterCombination,
       _286_287_instructionsAreNotEnabled,
       InvalidSymbolReference,
       CodeGenerationError,
       ASM_expected,
       DuplicateDynamicMethodIndex,
       DuplcateResourceIdentifier,
       DuplicateOrInvalidExportIndex,
       ProcedureOrFunctionIdentifierExpected,
       CannotExportThisSymbol,
       DuplicateExportName,
       ExecutableFileHeaderTooLarge,
       TooManySegments);

Const

  NoError = TargetAddressFound;

  SymbolTableError: Array [stMain..stIntermediateCode] of TCompilerError = (
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    CodeSegmentTooLarge,
    DataSegmentTooLarge,
    CodeSegmentTooLarge,
    DataSegmentTooLarge,
    TooManySymbols,
    TooManySymbols,
    TooManySymbols,
    StatementPartTooLarge);

  CompilerErrorString: Array [TCompilerError] of PChar = (
    'Target address found',
    'Out of memory',
    'Identifier expected',
    'Unknown identifier',
    'Duplicate identifier',
    'Syntax error',
    'Error in real constant',
    'Error in integer constant',
    'String constant exceeds line',
    '',
    'Unexpected end of file',
    'Line too long',
    'Type identifier expected',
    'Too many open files',
    'Invalid filename',
    'File not found',
    'Disk full',
    'Invalid compiler directive',
    'Too many files',
    'Undefined type in pointer definition',
    'Variable identifier expected',
    'Error in type',
    'Structure too large',
    'Set base type out of range',
    'File components may not be files or objects',
    'Invalid string length',
    'Type mismatch',
    'Invalid subrange base type',
    'Lower bound greater than upper bound',
    'Ordinal type expected',
    'Integer constant expected',
    'Constant expected',
    'Integer or real constant expected',
    'Pointer type identifier expected',
    'Invalid function result type',
    'Label identifier expected',
    'BEGIN expected',
    'END expected',
    'Integer expression expected',
    'Ordinal expression expected',
    'Boolean expression expected',
    'Operand types do not match operator',
    'Error in expression',
    'Illegal assignment',
    'Field identifier expected',
    'Object file too large',
    'Undefined external',
    'Invalid object file record',
    'Code segment too large',
    'Data segment too large',
    'DO expected',
    'Invalid PUBLIC definition',
    'Invalid EXTRN definition',
    'Too many EXTRN definitions',
    'OF expected',
    'INTERFACE expected',
    'Invalid relocatable reference',
    'THEN expected',
    'TO or DOWNTO expected',
    'Undefined forward',
    '',
    'Invalid type cast',
    'Division by zero',
    'Invalid file type',
    'Cannot Read or Write variables of this type',
    'Pointer variable expected',
    'String variable expected',
    'String expression expected',
    'Circular unit reference',
    'Unit name mismatch',
    'Unit version mismatch',
    'Internal stack overflow',
    'Unit file format error',
    'IMPLEMENTATION expected',
    'Constant and CASE types do not match',
    'Record or object variable expected',
    'Constant out of range',
    'File variable expected',
    'Pointer expression expected',
    'Integer or real expression expected',
    'Label not within current block',
    'Label already defined',
    'Undefined label in preceding statement part',
    'Invalid @ argument',
    'UNIT expected',
    '";" expected',
    '":" expected',
    '"," expected',
    '"(" expected',
    '")" expected',
    '"=" expected',
    '":=" expected',
    '"[" or "(." expected',
    '"]" or ".)" expected',
    '"." expected',
    '".." expected',
    'Too many variables',
    'Invalid FOR control variable',
    'Integer variable expected',
    'File types are not allowed here',
    'String length mismatch',
    'Invalid ordering of fields',
    'String constant expected',
    'Integer or real variable expected',
    'Ordinal variable expected',
    'INLINE error',
    'Character expression expected',
    'Too many relocation items',
    'Overflow in arithmetic operation',
    'No enclosing FOR, WHILE, or REPEAT statement',
    'Debug information table overflow',
    'Compilation aborted',
    'CASE constant out of range',
    'Error in statement',
    'Cannot call an interrupt procedure',
    '',
    'Must be in 8087 mode to compile this',
    'Target address not found',
    'Include files are not allowed here',
    'No inherited methods are accessible here',
    '',
    'Invalid qualifier',
    'Invalid variable reference',
    'Too many symbols',
    'Statement part too large',
    '',
    'Files must be var parameters',
    'Too many conditional symbols',
    'Misplaced conditional directive',
    'ENDIF directive missing',
    'Error in initial conditional defines',
    'Header does not match previous definition',
    '',
    'Cannot evaluate this expression',
    'Expression incorrectly terminated',
    'Invalid format specifier',
    'Invalid indirect reference',
    'Structured variables are not allowed here',
    'Cannot evaluate without System unit',
    'Cannot access this symbol',
    'Invalid floating point operation',
    'Cannot compile overlays to memory',
    'Pointer or procedural variable expected',
    'Invalid procedure or function reference',
    'Cannot overlay this unit',
    'Too many nested scopes',
    'File access denied',
    'Object type expected',
    'Local object types are not allowed',
    'VIRTUAL expected',
    'Method identifier expected',
    'Virtual constructors are not allowed',
    'Constructor identifier expected',
    'Destructor identifier expected',
    'Fail only allowed within constructors',
    'Invalid combination of opcode and operands',
    'Memory reference expected',
    'Cannot add or subtract relocatable symbols',
    'Invalid register combination',
    '286/287 instructions are not enabled',
    'Invalid symbol reference',
    'Code generation error',
    'ASM expected',
    'Duplicate dynamic method index',
    'Duplcate resource identifier',
    'Duplicate or invalid export index',
    'Procedure or function identifier expected',
    'Cannot export this symbol',
    'Duplicate export name',
    'Executable file header too large',
    'Too many segments');
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compilation-process
- https://turbopascal.org/compiler
- https://turbopascal.org/error-handling
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/main-program
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
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

