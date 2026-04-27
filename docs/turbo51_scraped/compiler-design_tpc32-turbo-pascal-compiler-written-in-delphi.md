# Turbo Pascal Compiler Written in Delphi

**URL:** https://turbo51.com/compiler-design/tpc32-turbo-pascal-compiler-written-in-delphi

---

## Table of Contents

  - TPC32
- Turbo Pascal Compiler Written in Delphi
  - Pascal Compiler Source Code
  - Make your own Pascal Compiler
  - How can you get TPC32 source files?
  - What can you do with Turbo Pascal Compiler source files?
  - The difference between TPC16 and TPC32
  - The difference between TPC32 and other compilers
  - What do you get with TPC32
    - Microcontroller Books

---

## Content

TPC32
Turbo Pascal Compiler Written in Delphi
Pascal Compiler Source Code
Have you ever wondered how can you start writing your own compiler? What lies beneath Turbo Pascal, the most popular Pascal compiler? Would it be possible to get (equivalent) source code of some popular commercial compiler? If the answer is yes then you came to the right place. Here you can get a Turbo Pascal 7 command line compiler written in
Delphi
. This is not the "original" Borland source code of the Turbo Pascal compilers since it was not written in Delphi. This is TPC32, a Pascal compiler written from scratch. It is completely source-code compatible with the Borland Turbo Pascal 7.0 command line compiler TPC.EXE. This means that it uses the same syntax and generates exactly the same compiled units and 16-bit executable code for x86. It features also some improvements in symbol table management and long file name support.
Make your own Pascal Compiler
This source code shows all the beauty of the Pascal programming language and reveals all the tricks needed to build a fast and compact compiler for any language, not just Pascal. With this Pascal compiler source code you will get:
Architecture of symbol tables
Understanding of Turbo Pascal unit structure
Ultra fast scanner
Examples of hash tables for fast keyword search
Plethora of data structures needed in any compiler
Algorithms for dealing with identifiers, scopes and units
Parser to analyze a sequence of tokens and to check Pascal syntax
Example of compiler error reporting
Understanding of expressions and calculations
Examples of arbitrary expression processing and code generation
Algorithms for generating complex code with limited set of registers
Example of in-line assembler compiler
Example of intermediate code structure
Optimizing code generator
Understanding of importing object files
Optimizing linker with excellent approach to resolve references
A working compiler compatible with Borland Turbo Pascal 7
Deep understanding of Turbo Pascal internals
With TPC32 source code you will be able to make your own compiler. The TPC32 compiler is written in Delphi 7 which means that you will be able to use all the available memory without any segment limitations. The result of compilation is an executable file TPC32.exe which can be used to compile any Turbo Pascal 7 source file. It will create a Borland Turbo Pascal 7.0 binary compatible TPU or EXE files. The only exception to this is the debug information in exe file which is not implemented.
How can you get TPC32 source files?
You can get TPC32 Turbo Pascal compiler source code for the price written below. You pay online with PayPal and then you will get a download link by email. If you have ever tried to write a compiler or you have studied some compiler construction books then you know how hard it is to make a compiler that is fast, compact and has all the features needed. For the price you will pay for the TPC32 source code you will get all the data structures and algorithms needed to write any compiler. You get all the building blocks for compiler construction for only a fraction of the cost for any compiler development. Do not reinvent the wheel, analyze proven and tested code and learn tips and tricks in compiler design. All types, constants, variables, procedures and functions in the source code have meaningful names. Therefore, from the source code itself it is easy to understand what the code does. And since Pascal programming language is also easy to read, this source code can be read as an algorithm for every compiler building block.
What can you do with Turbo Pascal Compiler source files?
You can make your own compiler. You can extend features of Turbo Pascal or make a totally new compiler. The most important feature of this package is practical implementation of various algorithms together with proven data structures. A successful compiler implementation depends mainly on data structures and algorithms used. Therefore TPC32 source can be used as an inspiration on how to implement complex algorithms for your compiler. TPC32 is still a Turbo Pascal compiler and still generates 16-bit code. TPC32 is a WIN32 application, the source code is compatible with Borland Delphi 7 syntax. You will be able to add features, modify code and make a totally new compiler.
The difference between TPC16 and TPC32
TPC16
is a direct representation of the Turbo Pascal 7 command line compiler. The compiler source code, input source files and output unit and executable files are compatible with the Borland Turbo Pascal 7 command line compiler. This compiler is a DOS application and internal data structures use segment:offset addressing. TPC32 is the same compiler modified to compile with Delphi 7. This means that it no longer uses segments and offsets but a memory model with almost no limitations. TPC32 also supports long file names.
If you would like to understand the internal data representation and the architecture of the compiler or you are only interested in various algorithms and data structures used for internal compiler processing then TPC16 is all you need. If you are interested in developing you own compiler in Pascal language then it would make sense to go for TPC32. With TPC32 you will be able to develop a modern Win32 application. With TPC32 you will also get the original
TPC16 source code
for free.
The difference between TPC32 and other compilers
There are also some open-source compilers available. What is the difference between TPC32 and other compilers and why TPC32 is not free? TPC32 is 100% compatible with Turbo Pascal 7 on the source level (the files it compiles) and on the binary level (the files it produces). TPC32 shows compiler internals not found in other compilers and is an excellent companion to many popular books about compiler construction. If you are serious about making your own compiler then you should have TPC32. TPC32 exe file is available for free and can be found below in the demo version. TPC32 source files are not available for free because I put a lot of effort into writing the code and it took me some time to make it working and compatible with Borland Turbo Pascal on source and binary level. It also took me some time to make it compile under Delphi. The original TPC16 is a DOS compiler which relies heavily on the segment:offset memory. The price I ask for it is only a fraction of what these source files are worth. If you have ever tried to write a compiler then you probably know what I'm talking about.
TPC32 is an excellent e-book on practical compiler design and implementation taking into account that it reveals the complete, working compiler
. There is no detailed text book included, all you have to do is examine the source code and follow the control flow of the compiler. You will be amazed how simple can complex tasks be. TPC32 is the a great starting point to write your own compiler.
What do you get with TPC32
Delphi source code for all units and main compiler program:
{$APPTYPE CONSOLE}

Program TPC32;

Uses SysUtils,
     Windows,
     Parser,
     Scanner,
     IOUtilities,
     CommonVariables;

begin
  Writeln (GreetingString);
{$WARNINGS OFF}
  CommandLinePtr := CmdLine;
{$WARNINGS ON}
  ReadConfigurationFile;
  CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk];
  LinkerOptions := [];
  StartOfSourceFilesData := @SourceFilesBuffer;
  ProcessCompilerParameters;
  StrCopy (TempString, DefaultModuleData.LibraryName);
  CurrentFileName := TempString;
  FindFilePath (TempString, fd_TPL_CFG, feOriginal, []);
  SaveRegistersAndSetErrorReturnAddress;
  LoadLibrary;
  Case LastError of
    FileNotFound: Writeln ('Warning: ', TempString, ' not found.');
    NoError:
    else WriteCompilationErrorAndHalt;
  end;
  With DefaultModuleData do
    begin
      InitialEnvironmentFlags := EnvironmentFlags;
      InitialCompilerSwitches := CompilerSwitches;
      ModuleStack   := Stack;
      ModuleHeapMin := HeapMin;
      ModuleHeapMax := HeapMax;
    end;
  CurrentFileName := MainFileName;
  FindFilePath (CurrentFileName, fdCurrentDirectory, fe_PAS, []);
  StartTime := Time;
  CompilationInProgress := True;
  SaveRegistersAndSetErrorReturnAddress;
  Compile;
  CompilationInProgress := False;
  StopTime := Time;
  If LastError <> NoError then WriteCompilationErrorAndHalt;
  If cmoFindError in CompilerModeOptions then
    begin
      SaveRegistersAndSetErrorReturnAddress;
      FindAndWriteSourceLine;
      WriteCompilationErrorAndHalt;
    end;
  WriteCompilationStatus;
end.
Below you can download a demo version of the TPC32 compiler which contains all the files except source files for compiler units. This way you can check what you will get in the full version.
TPC32 is a Delphi application. TPC32.exe is a Win32 console Pascal compiler. TPC32 still generates 16-bit x86 code, but the compiler is more flexible than TPC16 due to the new approach to symbol table management. TPC32 also supports long file names and was used as a starting point for Turbo51,
Pascal compiler for 8051 microcontrollers
.
Before you decide to pay for the TPC32 source code you can download a demo version of the TPC32 compiler which contains all the files except source files for compiler units. This way you can check the TPC32 compiler and compare it to the original Borland compiler. If you need more information about the TPC32 compiler or have any additional questions regarding the compiler construction please feel free to
contact
me. I will be happy to help you. If you purchase TPC32 you will also get the original TPC16 source code for free.
If you are really going to make your own compiler and you would also like to change reserved words or initial types found in the system unit, then you need also some tools to make the bootstrap symbol tables found in the system.tps file and symbol tables with hash values found in the source files of TPC32. This package is available for additional fee and requires TPC32 source files for compilation. This is an add-on package for compiler constructors already using TPC32.
When you click
Proceed to checkout
you will be taken to the payment confirmation page. If you don't have PayPal account you can also pay with any supported credit card. After you click the Buy Now button you will be taken to the actual page where you can make the payment. After completing the checkout process, you will receive a unique link to download the source code sent to your email address. In the case of any questions please go to the
Contact
page and send a message. Warning: if for some reason the payment status will show as "pending" you will not receive the download link until the payment clears and the money is actually transferred.
Payments are managed by
PayPal
, industry leader in online money transfers. A lot of people are scared about using their credit card online, but this fear is totally unmotivated. Using your credit card at the restaurant is much more risky! If you have a major credit card (Visa, MasterCard, American Express), paying online is easy and safe.
Turbo Pascal Compiler Source Code
TPC16 Demo
Turbo Pascal Compiler Written in Turbo Pascal
TPC32 Demo
Turbo Pascal Compiler Written in Delphi
TPC16
€50.00 for 2 downloads in the next 24 hours
Turbo Pascal Compiler Written in Turbo Pascal
TPC32 (+ TPC16 free)
€150.00 for 2 downloads in the next 24 hours
Turbo Pascal Compiler Written in Delphi
Turbo Pascal Compiler Written in Turbo Pascal
Bootstrap Symbol Tables
€50.00 for 2 downloads in the next 24 hours
Add-on package for TPC32
Software to create:
- Bootstrap symbol tables file system.tps
- Source files used for the TPC32 compiler: ReservedWords.pas, AssemblerWords.inc and SystemTypeOffsets.inc
Requires TPC32 source files for compilation.
Copyright © 2025
Igor Funa
. All Rights Reserved.
Terms, Conditions and Privacy policy
Microcontroller
Books

---

## Code Examples

### Code Block 1

```pascal
{$APPTYPE CONSOLE}

Program TPC32;

Uses SysUtils,
     Windows,
     Parser,
     Scanner,
     IOUtilities,
     CommonVariables;

begin
  Writeln (GreetingString);
{$WARNINGS OFF}
  CommandLinePtr := CmdLine;
{$WARNINGS ON}
  ReadConfigurationFile;
  CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk];
  LinkerOptions := [];
  StartOfSourceFilesData := @SourceFilesBuffer;
  ProcessCompilerParameters;
  StrCopy (TempString, DefaultModuleData.LibraryName);
  CurrentFileName := TempString;
  FindFilePath (TempString, fd_TPL_CFG, feOriginal, []);
  SaveRegistersAndSetErrorReturnAddress;
  LoadLibrary;
  Case LastError of
    FileNotFound: Writeln ('Warning: ', TempString, ' not found.');
    NoError:
    else WriteCompilationErrorAndHalt;
  end;
  With DefaultModuleData do
    begin
      InitialEnvironmentFlags := EnvironmentFlags;
      InitialCompilerSwitches := CompilerSwitches;
      ModuleStack   := Stack;
      ModuleHeapMin := HeapMin;
      ModuleHeapMax := HeapMax;
    end;
  CurrentFileName := MainFileName;
  FindFilePath (CurrentFileName, fdCurrentDirectory, fe_PAS, []);
  StartTime := Time;
  CompilationInProgress := True;
  SaveRegistersAndSetErrorReturnAddress;
  Compile;
  CompilationInProgress := False;
  StopTime := Time;
  If LastError <> NoError then WriteCompilationErrorAndHalt;
  If cmoFindError in CompilerModeOptions then
    begin
      SaveRegistersAndSetErrorReturnAddress;
      FindAndWriteSourceLine;
      WriteCompilationErrorAndHalt;
    end;
  WriteCompilationStatus;
end.

```

