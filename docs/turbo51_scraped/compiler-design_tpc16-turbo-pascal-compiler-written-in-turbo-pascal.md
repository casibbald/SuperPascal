# Turbo Pascal Compiler Written in Turbo Pascal

**URL:** https://turbo51.com/compiler-design/tpc16-turbo-pascal-compiler-written-in-turbo-pascal

---

## Table of Contents

  - TPC16
- Turbo Pascal Compiler Written in Turbo Pascal
  - Pascal Compiler Source Code
  - Make your own Turbo Pascal Compiler
  - How can you get TPC16 source files?
  - What can you do with Turbo Pascal Compiler source files?
  - The difference between TPC16 and other compilers
  - What do you get with TPC16
  - Turbo Pascal Download
    - Microcontroller Books

---

## Content

TPC16
Turbo Pascal Compiler Written in Turbo Pascal
Pascal Compiler Source Code
Have you ever wondered how can you start writing your own compiler? What lies beneath
Turbo Pascal
, the most popular Pascal compiler? Would it be possible to get (equivalent) source code of some popular commercial compiler? If the answer is yes then you came to the right place. Here you can get a
Turbo Pascal 7
command line compiler written in Turbo Pascal. This is not the "original" Borland source code of the Turbo Pascal since it was probably not written in Pascal. This is TPC16, a Turbo Pascal compiler written from scratch. It is completely source-code compatible with the Borland Turbo Pascal 7.0 command line compiler TPC.EXE. This means that it uses the same syntax and generates exactly the same compiled units and 16-bit code for x86.
Make your own Turbo Pascal Compiler
This source code shows all the beauty of the Pascal programming language and reveals all the tricks needed to build a fast and compact compiler for any language, not just Pascal. With this Turbo Pascal single pass compiler source code you will get:
Architecture of
symbol tables
Understanding of Turbo Pascal unit structure
Ultra fast
scanner
Examples of
hash tables
for fast keyword search
Plethora of data structures needed in any compiler
Algorithms for dealing with identifiers, scopes and units
Parser
to analyze a sequence of
tokens
and to check Pascal syntax
Example of compiler error reporting
Understanding of
expressions and calculations
Examples of arbitrary expression processing and code generation
Algorithms for generating complex code with limited set of registers
Example of in-line assembler compiler
Example of
intermediate code structure
Optimizing
code generator
Understanding how object files are imported
Optimizing linker
with excellent approach to resolve references
A working compiler compatible with Borland Turbo Pascal 7
Deep understanding of
Turbo Pascal internals
With TPC16 source code you will be able to fully understand how Turbo Pascal compiler works. The compiler is written in Turbo Pascal 7 which means that it can compile itself. At least in theory. Turbo Pascal consists of many units, some of which are quite big. Since the Turbo Pascal compiler is a DOS application it has limited memory. Therefore, it is not possible to compile TPC16 compiler with TPC.EXE directly. However, there is a simple workaround. You can compile it with BPC.EXE which is the same compiler for protected mode and has less memory limitations. The result of compilation is an executable file TPC16.exe which can be used to compile Turbo Pascal source files. It will create a Borland Turbo Pascal 7.0 binary compatible TPU or EXE files. The only exception to this is map file and debug info in exe file which are not implemented.
How can you get TPC16 source files?
Most of the source code is published with comments on
Turbo Pascal
website. Here you will be able to get all the internals of the compiler. However, if you would like to use this compiler for your own project and you need the complete TPC16 package with all source files then please continue reading. You can get TPC16 Turbo Pascal compiler source code for some small compensation. If you have ever tried to write a compiler or you have studied some compiler construction books then you know how hard it is to make a compiler that is fast, compact and has all the features needed. For the price you will pay for the TPC16 source code you will get all the data structures and algorithms needed to write any compiler. You get all the building blocks for compiler construction for only a fraction of the cost for any compiler development. Do not reinvent the wheel, analyze proven and tested code and learn tips and tricks in compiler design. All types, constants, variables, procedures and functions in the source code have meaningful names. Therefore, from the source code itself it is easy to understand what the code does. And since Pascal programming language is also easy to read, this source code can be read as an algorithm for every compiler building block.
What can you do with Turbo Pascal Compiler source files?
You can make your own compiler. You can extend features of Turbo Pascal or make a totally new compiler. The most important feature of this package is practical implementation of various algorithms together with proven data structures. A successful compiler implementation depends mainly on data structures and algorithms used. Therefore TPC16 source can be used as an inspiration on how to implement complex algorithms for your compiler. However, if you would like to develop your own compiler it would be better if you start with
TPC32
. This is the same compiler that compiles with Delphi 7. TPC32 is no longer limited with segment:offset data structures. It features a new symbol table management and supports also long file names.
The difference between TPC16 and other compilers
There are also some open-source compilers available. What is the difference between TPC16 and other compilers and why TPC16 is not free? TPC16 is 100% compatible with Turbo Pascal 7 on the source level (the files it compiles) and on the binary level (the files it produces). TPC16 shows compiler internals not found in other compilers and is an excellent companion to many popular books about compiler construction. If you are serious about making your own compiler then you should have TPC16. TPC16 exe file is available for free and can be found below in the demo version. TPC16 source files are not available for free because I put a lot of effort into writing the code and it took me some time to make it working and compatible with Borland Turbo Pascal on source and binary level. The price I ask for it is only a fraction of what these source files are worth. If you have ever tried to write a compiler then you probably know what I'm talking about.
TPC16 is the cheapest e-book on compiler design and implementation taking into account that it reveals the complete, working compiler
. There is no detailed text book included, all you have to do is examine the source code and follow the control flow of the compiler. You will be amazed how simple can complex tasks be. TPC16 is the best book on practical compiler construction.
What do you get with TPC16
You get
Pascal source code
for main program and all units. You will be able to compile TPC16 command line compiler and make modifications according to your needs.
Program TPC16;

Uses
  Strings,
  Parser,
  Scanner,
  IOUtils,
  ResWords,
  CommVars;

begin
  Writeln (GreetingString);
  CmdPtr := Ptr (PrefixSeg, $80);
  CmdPtr [Byte (CmdPtr [0]) + 1] := #0;
  CmdPtr [0] := ' ';
  CfgPtr := FileBuffer;
  Get_TPL_CFG_Directory;
  Read_TPC_CFG;
  CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk];
  LinkerOptions := [];
  CommandLineConditionalDefines := InitialConditionalDefines;
  StartOfSourceFileStructures := @FileStructure;
  ProcessCompilerParameters;
  StrCopy (TempStr, GlobalModuleData^.LibraryName);
  CurrentFileName := TempStr;
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  LoadLibrary;
  Case LastError of
    FileNotFound: Writeln ('Warning: ', TempStr, ' not found.');
    NoError:
    else WriteCompilationErrorAndHalt;
  end;
  With GlobalModuleData^ do
    begin
      InitialEnvironmentFlags := EnvironmentFlags;
      InitialCompilerSwitches := CompilerSwitches;
      ModuleStack   := Stack;
      ModuleHeapMin := HeapMin;
      ModuleHeapMax := HeapMax;
    end;
  CurrentFileName := MainFileName;
  FindFilePath (CurrentFileName, Dir_None or Ext_PAS);
  StartCounter := InterruptCounter;
  CompilationInProgress := True;
  Compile;
  CompilationInProgress := False;
  StopCounter := InterruptCounter;
  If LastError <> NoError then WriteCompilationErrorAndHalt;
  If cmoFindError in CompilerModeOptions then
    begin
      FindAndWriteSourceLine;
      WriteCompilationErrorAndHalt;
    end;
  WriteCompilationStatus;
end.
Turbo Pascal Download
Below you can download a demo version of the TPC16 compiler which contains all the files except source files for compiler units. This way you can check what you will get in the full version. However, most source files are published and explained on the
Turbo Pascal
website.
TPC16 is a DOS application and relies on segment:offset addressing. For some algorithms it is an advantage, on the other hand it is a drawback and limiting factor. I am offering also the
TPC32,
a 32-bit version of the same Turbo Pascal compiler which compiles with Borland Delphi console compiler and has removed almost all limitations of the 16-bit version. TPC32 still generates 16-bit x86 code, but the compiler is more flexible due to the new approach to symbol table management. TPC32 was also used as a starting point for Turbo51,
Pascal compiler for 8051 microcontrollers
.
The source code of TPC32 costs a little more than TPC16 since there is a lot of additional work that was put into it. However, if you have already purchased the TPC16 source code you can get the TPC32 for some additional contribution (
contact
me to get instructions for this upgrade). TPC32 has all the features of TPC16 with addition of some algorithms dealing with 32-bit offsets instead of segment:offset addresses plus additional improvements. If you would like to understand the internal data representation and the architecture of the compiler or you are only interested in various algorithms and data structures used for internal compiler processing then TPC16 is all you need. If you are interested in developing you own compiler in Pascal language then it would make sense to go for
TPC32
. If you purchase TPC32 you will also get TPC16 source code for free.
Before you decide to pay for the TPC16 source code you can check it at
Turbo Pascal
website and download a demo version of the TPC16 compiler which contains all the files except source files for compiler units. This way you can check the TPC16 compiler and compare it to the original Borland compiler. If you need more information about the TPC16 compiler or have any additional questions regarding the compiler construction please feel free to
contact
me. I will be happy to help you.
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
Program TPC16;

Uses
  Strings,
  Parser,
  Scanner,
  IOUtils,
  ResWords,
  CommVars;

begin
  Writeln (GreetingString);
  CmdPtr := Ptr (PrefixSeg, $80);
  CmdPtr [Byte (CmdPtr [0]) + 1] := #0;
  CmdPtr [0] := ' ';
  CfgPtr := FileBuffer;
  Get_TPL_CFG_Directory;
  Read_TPC_CFG;
  CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk];
  LinkerOptions := [];
  CommandLineConditionalDefines := InitialConditionalDefines;
  StartOfSourceFileStructures := @FileStructure;
  ProcessCompilerParameters;
  StrCopy (TempStr, GlobalModuleData^.LibraryName);
  CurrentFileName := TempStr;
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  LoadLibrary;
  Case LastError of
    FileNotFound: Writeln ('Warning: ', TempStr, ' not found.');
    NoError:
    else WriteCompilationErrorAndHalt;
  end;
  With GlobalModuleData^ do
    begin
      InitialEnvironmentFlags := EnvironmentFlags;
      InitialCompilerSwitches := CompilerSwitches;
      ModuleStack   := Stack;
      ModuleHeapMin := HeapMin;
      ModuleHeapMax := HeapMax;
    end;
  CurrentFileName := MainFileName;
  FindFilePath (CurrentFileName, Dir_None or Ext_PAS);
  StartCounter := InterruptCounter;
  CompilationInProgress := True;
  Compile;
  CompilationInProgress := False;
  StopCounter := InterruptCounter;
  If LastError <> NoError then WriteCompilationErrorAndHalt;
  If cmoFindError in CompilerModeOptions then
    begin
      FindAndWriteSourceLine;
      WriteCompilationErrorAndHalt;
    end;
  WriteCompilationStatus;
end.

```

