# Turbo51 - Pascal compiler for 8051 microcontrollers

**URL:** https://turbo51.com/

---

## Table of Contents

- Turbo Pascal for 8051 microcontrollers
  - Pascal is easy to write and easy to read
  - Main features:
  - Used optimizations:
    - Microcontroller Books

---

## Content

Turbo Pascal for 8051 microcontrollers
Pascal is easy to write and easy to read
Turbo51 is a free Pascal compiler for the 8051 family of microcontrollers. If you are programming for the 8051 family of microcontrollers and you like Pascal programming language then you will love Turbo51.
Main features:
Win32 console application
Fast single-pass optimizing compiler
Borland Turbo Pascal 7 syntax
Full floating point support
Mixed Pascal and assembler code
Full use of register banks
Advanced multi-pass optimizer
Smart linker
Generates compact high quality code
Output formats: BIN, HEX, OMF
Assembler source code generation
Source-level debugging with absolute
OMF-51 extended object file
Used optimizations:
Constant folding
Integer arithmetic optimizations
Dead code elimination
Branch elimination
Code-block reordering
Loop-invariant code motion
Loop inversion
Induction variable elimination
Instruction selection
Instruction combining
Register allocation
Common subexpression elimination
Peephole optimization
Turbo51 is released as a freeware. You can use Turbo51 for hobby projects and for serious work. Check documentation pages and code examples that show the syntax, features and generated files. This should be enough to start a 8051 project development with Turbo51. And if you are still missing something or have a problem you can always ask for help.
If you are already familiar with 8051 assembly language programming you can start with Turbo51 as 8051 assembly language compiler and then add some Pascal statements until you become familiar with Turbo51 and Pascal syntax. A good approach is also to compile some Pascal code and then check generated code (ASM file). This way you can learn assembly language, get some ideas on how to write effective code and become familiar with the compiler. Turbo51, like many popular C compilers for 8051, generates optimized code and supports source-level debugging with OMF object file.
Program Turbo51;

Uses FastCompiler, AdvancedOptimizations, SmartLinker, AssemblerFileGenerator;

//  Turbo51 is released as freeware. You can download it and use it for FREE.
//  However, if you like Turbo51 you can donate some small amount via PayPal.
//  Donations are a great way to show your appreciation for my software.

 begin
  DownloadFrom ('http://turbo51.com/download-free-pascal-compiler-8051');
  InstallAndConfigure;
  Repeat
    CreateProject;
    CompileProject;
    TestProject;
    While ThereIsAProblem do
      begin
        CheckCode;
        CheckDocumentation;
        TryAgain;
        Case ProblemSolved of
          True: Break;
          else  AskForHelp;
        end;
      end;
    If InstalledVersion < '0.1.3.17' then Update;
    If Satisfied then Donate ($20);
  until NoMoreProjects;
end.

---

## Code Examples

### Code Block 1

```pascal
Program Turbo51;

Uses FastCompiler, AdvancedOptimizations, SmartLinker, AssemblerFileGenerator;

//  Turbo51 is released as freeware. You can download it and use it for FREE.
//  However, if you like Turbo51 you can donate some small amount via PayPal.
//  Donations are a great way to show your appreciation for my software.

 begin
  DownloadFrom ('http://turbo51.com/download-free-pascal-compiler-8051');
  InstallAndConfigure;
  Repeat
    CreateProject;
    CompileProject;
    TestProject;
    While ThereIsAProblem do
      begin
        CheckCode;
        CheckDocumentation;
        TryAgain;
        Case ProblemSolved of
          True: Break;
          else  AskForHelp;
        end;
      end;
    If InstalledVersion < '0.1.3.17' then Update;
    If Satisfied then Donate ($20);
  until NoMoreProjects;
end.

```

## Tables

### Table 1

| Main features:

Win32 console application
Fast single-pass optimizing compiler
Borland Turbo Pascal 7 syntax
Full floating point support
Mixed Pascal and assembler code
Full use of register banks
Advanced multi-pass optimizer
Smart linker
Generates compact high quality code
Output formats: BIN, HEX, OMF
Assembler source code generation
Source-level debugging with absoluteOMF-51 extended object file |  | Used optimizations:

Constant folding
Integer arithmetic optimizations
Dead code elimination
Branch elimination
Code-block reordering
Loop-invariant code motion
Loop inversion
Induction variable elimination
Instruction selection
Instruction combining
Register allocation
Common subexpression elimination
Peephole optimization |

