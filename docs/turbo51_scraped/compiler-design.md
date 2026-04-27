# Compiler Design - Practical Compiler Construction

**URL:** https://turbo51.com/compiler-design

---

## Table of Contents

- Compiler Design
  - Practical Compiler Construction
  - Read more about TPC16
    - Turbo Pascal Compiler Written in Turbo Pascal
  - Examine Turbo Pascal compiler design
    - Turbo Pascal Internals
  - Read more about TPC32
    - Turbo Pascal Compiler Written in Delphi
  - Books about Compiler Design
    - Compilers: Principles, Techniques, and Tools
    - Compiler Construction: Principles and Practice
    - Building an Optimizing Compiler
    - Advanced Compiler Design and Implementation
    - Introduction to Compiler Construction
    - Turbo Pascal Compiler Written in Turbo Pascal
    - Microcontroller Books

---

## Content

Practical Compiler Construction
Designing a compiler is not a simple task. Every compiler is a tight cooperation of data structures and algorithms. You need to understand the whole process of compilation in order to properly design symbol tables, scanner, parser, internal data representation, intermediate code structure, optimization passes, and other compiler building blocks.
There are many excellent books on compiler design and implementation. Some compiler books that I recommend are listed below. However, the best book on compiler construction is the compiler itself. There are many compiler construction tools around, but they don't provide the best approach to design a fast, standalone compiler. In order to learn about practical compiler algorithms, internal data representation and to test new approaches you need a working compiler to work with. To write such a program from scratch is inevitably a time consuming task which includes reinventing the wheel.
However, to have every aspect of a compiler under total control you need to write your own compiler. And before you start writing you should check the source code of some existing compiler. There are some open-source compilers around. An additional benefit in understanding the best compiler approaches would be to have a possibility to take a look into the internal architecture of some successful commercial compiler. The most popular Pascal compiler like Turbo Pascal, for example. Luckily, you are at the right place.
I am a big fan of
Pascal programming language
. Therefore, I was using Turbo Pascal since it was available on the CP/M platform in early 1980s. Turbo Pascal was my primary developing platform for years, until Borland introduced Delphi. My passion for Pascal programming language brought me to the decision to write my own Pascal compiler. Mainly because I got some books about compiler construction (see below) and I wanted to have a good, Turbo Pascal compatible compiler for the 8051 family of microcontrollers. My first goal was to write a Pascal compiler that would be compatible with
Borland Turbo Pascal
command line compiler. Compatible compiler means compatible on the source level and on the binary level - compiling the Turbo Pascal 7 source files and generating the same units and executable code. The goal was accomplished with
TPC16, Turbo Pascal compiler written in Borland Turbo Pascal
.
TPC16 is a
complete Turbo Pascal compiler
written from scratch. It is consisted of many units like Scanner, Symbol Tables, Parser, Expressions, Statements, Inline Assembler, Code Generator, OMF import, Linker, and others. TPC16 source code reveals Turbo Pascal internals showing all the secrets of a successful compiler. With it you can easily get an overview of a practical compiler implementation. TPC16 was the first step in writing the Turbo51 compiler. While the source code of the Turbo51 compiler is not available, you can get the source code of TPC16. Unfortunately, I can not offer it for free. It took me some time to write and debug it so I would like to get some compensation for this. But if you would like to write your own compiler you will definitely need some handbook on compilers. And TPC16 is like a "compiler e-book" that can answer all the questions you will be asking yourself during the compiler design. TPC16 will reveal proven data structures and internal data representation including some clever algorithms to deal with symbol tables, expressions, intermediate code, optimizations and more.
The next step is TPC32. This is the same Turbo Pascal compiler as TPC16. It is only slightly modified to compile with Delphi 7. It still generates 16-bit x86 code but the compiler is a true 32-bit applicaton. TPC32 is a great starting point if you would like to write your own compiler.
Read more about TPC16
Turbo Pascal Compiler Written in Turbo Pascal
Examine Turbo Pascal compiler design
Turbo Pascal Internals
Read more about TPC32
Turbo Pascal Compiler Written in Delphi
Books about Compiler Design
You can find most books on compiler design and compiler construction at the
Book store
page under Compiler Construction. Here I recommend some of them.
Compilers: Principles, Techniques, and Tools
This is the basic book you should start with. It provides the foundation for understanding the theory and practice of compilers. It is known as the Dragon Book because its covers depict a knight and a dragon in battle, a metaphor for conquering complexity. Topics covered include: Compiler structure, Lexical analysis (including regular expressions and finite automata), Syntax analysis (including context-free grammars, LL parsers, bottom-up parsers, and LR parsers), Syntax-directed translation, Type checking (including type conversions and polymorphism), Run-time environment (including parameter passing, symbol tables, and storage allocation), Code generation (including intermediate code generation), Code optimization, directed translation, new data flow analysis, parallel machines, JIT compiling, garbage collection and new case studies. Although two decades have passed since the publication of the first edition, it is widely regarded as the classic definitive compiler technology text.
Compiler Construction: Principles and Practice
This is an excellent basic book on compilers. If you want to learn about compilers you should read this book. It explains the theory of compilers a whole lot better than the dragon book. It tries to keep focus on the clean understanding on the theory of parsers, semantic analysis and also later on code optimization. Compiler Construction: Principles and Practice features a comprehensive, hands-on case study project for constructing an actual, working compiler. This case study involves a relatively simple programming language that will expose readers to the basic concepts used (and potential pitfalls) in constructing larger compilers. Kenneth Louden and his colleagues at San Jose State University have successfully class-tested this approach. Professionals joining or beginning a compiler project will find Compiler Construction valuable, as it provides the basic theory, necessary tools, and practical experience to design and program an authentic compiler.
Building an Optimizing Compiler
This book is for an advanced compiler writer. It provides a high level design for a thorough optimizer, code generator, scheduler and register allocator for a generic modern RISC processor. In the process it addresses the small issues that have a long impact on the implementation. The book provides a complete theory for Static Single Assignment Methods and partial redundancy methods for code optimization including a new register allocation techniques. The book approaches this subject from a practical viewpoint. Theory is introduced where intuitive arguments are insufficient, however the theory is described in practical terms. A single running example is used throughout the book to illustrate the compilation process. The focus of this book is on the concepts of building an optimizing compiler and the theory behind code optimization, not exactly on how to build the compiler from scratch. This is not an introductory book on compilers. If you want one, go for the classic book Compilers: Principles, Techniques, and Tools described above.
Book preview: download sample pages of the book
Building an Optimizing Compiler
Advanced Compiler Design and Implementation
This is a very advanced book focused on optimization algorithms. This comprehensive, up-to-date work examines advanced issues in the design and implementation of compilers for modern processors. Written for professionals and graduate students, the book guides readers in designing and implementing efficient structures for highly optimizing compilers for real-world languages. Covering advanced issues in fundamental areas of compiler design, this book discusses a wide array of possible code optimizations, determining the relative importance of optimizations, and selecting the most effective methods of implementation. Presents numerous clearly defined algorithms based on actual cases. Each optimization is clearly described with the Informal Compiler Algorithm Notation (ICAN), a language devised by the author to communicate algorithms effectively to people. With this book you will be able to understand all code optimizations in a modern compiler and you will be able to implement them with your preferred language. Again, this is a book for advanced users.
Introduction to Compiler Construction
Introduction to Compiler Construction addresses the essential aspects of compiler design at a level that is perfect for those studying compiler design. It is intended to the audience of novices, with the clear target of explaining in great details compilers principles. This book carefully describes how a compiler works, how it is organized and what the major problems are and how they have been solved. The approach is quite theoretical and principles-centered, just as the Dragon book is. But author departs from this in the writing style: it is definitely straightforward. He sacrifices the scope of the book in favor of clarity: he took the core of books like the Dragon, and expanded this core to a well appreaciable extent. It comes over and over again on more awkward concepts with detailed examples. This book presents the material the way it ought to be. Text is clear and concise without leaving out any of the essential material.
Turbo Pascal Compiler Written in Turbo Pascal
The best book on compiler design is the compiler itself. This is a Turbo Pascal 7 compatible compiler written in Turbo Pascal. The source code of this compiler shows all the beauty of the Pascal programming language and reveals all the tricks needed to build a fast and compact compiler for any language, not just Pascal. With this source code you will get: architecture of symbol tables, understanding of Turbo Pascal unit structure, ultra fast scanner, examples of hash tables for fast keyword search, plethora of data structures needed in any compiler, parser to analyze a sequence of tokens, understanding of expressions and calculations, examples of code generation with limited set of registers, example of intermediate code structure, understanding of the Turbo Pascal internals, etc. This is a complete source code of a working compiler compatible with Borland Turbo Pascal 7.

---

