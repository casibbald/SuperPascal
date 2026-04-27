# Turbo51 FAQ

**URL:** https://turbo51.com/8051-compiler-faq

---

## Table of Contents

- Turbo51 FAQ
    - What is Turbo51?
    - Who is behind this project?
    - Is source code of Turbo51 available?
    - Does Turbo51 support my 8051 microcontroller?
    - In what programming language is Turbo51 written?
    - Is it really free?
    - How else can I support Turbo51?
    - Why are there ads on this website?
    - What is Internal error?
    - Can I use Turbo51 for commercial products?
    - When will the user manual be published?
    - What about IDE?
    - How can I compile to generate only ACALL / AJMP instructions?
    - How can I declare record fields with Boolean type?
    - How can I call a procedure from an interrupt that uses register bank n?
    - How can I use Turbo51 as a 8051 assembly language compiler?
    - How can I use ReadLn without LF (#10) character?
    - Microcontroller Books

---

## Content

What is Turbo51?
Turbo51 is a Pascal compiler for the 8051 family of microcontrollers.
Who is behind this project?
Me,
Igor Funa
. This is my hobby project and I am doing it in my spare time for fun.
Is source code of Turbo51 available?
No. I have no plans to release it. However, if you are interested in compiler design, I am selling a Turbo Pascal compiler written in Turbo Pascal source code. This compiler was the first step in creating the Turbo51 compiler.
Does Turbo51 support my 8051 microcontroller?
Turbo51 compiler supports all 8051 family microcontrollers which are truly machine code compatible with the original 8051.
In what programming language is Turbo51 written?
Turbo51 is written in Borland Delphi.
Is it really free?
Yes, Turbo51 is available totally free. However, if you like it and you would like to support further development (this takes up a considerable amount of my time), you can
donate
some small amount via PayPal.
How else can I support Turbo51?
You can put a link to Turbo51 on your website. Here is the code:
<a href="http://turbo51.com/">Turbo51 - Pascal compiler for 8051 microcontrollers</a>
And some link description:
Free Pascal compiler for 8051 microcontrollers, Borland Turbo Pascal 7 syntax, floating point support, multi-pass optimizer, output formats: bin, hex, omf and asm source.
Why are there ads on this website?
Simply because I have some bills to pay. I'm offering Turbo51 for free, you can download it and use it without having to pay for it. If everybody who downloaded Turbo51 would donate only $1 this would probably be an ad-free website.
What is Internal error?
Internal error means that Turbo51 has encountered a situation where it can no longer continue the compilation. It is very likely a indication of a bug in the Turbo51. The error message means source file and position (source file of Turbo51) where this problem had happened. Please report internal error following the rules for
bug report
.
Can I use Turbo51 for commercial products?
Yes, you can use Turbo51 for any project you like, but don't forget to
donate
some small amount. Donations are a great way to show your appreciation for my work. Besides being an incredible boost to my morale, they are also a great incentive to fix any bug you find in my software and to add new features.
When will the user manual be published?
Until there will be some stable version with all major features implemented I have no plans to work on user manual. But I am getting close to this point (Turbo51 is mature enough to be used for a serious project), so this moment could be quite close. If you are missing some information or have a problem you can always ask for help.
What about IDE?
Turbo51 is only a compiler. There is no IDE. To write code you can use any IDE or editor of your choice. You can find some proposals on the
IDE
page.
How can I compile to generate only ACALL / AJMP instructions?
Compile all your sources with the
$AbsoluteInstructions On
(short version $A+) switch to generate ACALL/AJMP and use command line option /LA to use library Turbo51A.l51 which was compiled with the $AbsoluteInstructions On switch. In one command line this looks like this: Turbo51 MyProgram /$A+ /AL. If you have previously compiled some units without the $A+ switch then you need to rebuild them: Turbo51 MyProgram /$A+ /AL /B.
How can I declare record fields with Boolean type?
Boolean variables use BIT (boolean) memory in 8051. If you would like to use boolean variables in DATA or XDATA memory (or even in CODE memory as typed constant) then you have two possibilities:
Use type
ByteBool
(
WordBool
and
LongBool
are also possible), this will occupy one byte (or 2 for
WordBool
and 4 bytes for
LongBool
)
Use sets. See some examples of sets in records on the
inline procedures
page. Each set element can be treated as Boolean variable and will occupy only one bit in DATA or XDATA memory - you have 8 Boolean variables in on byte.
How can I call a procedure from an interrupt that uses register bank n?
By default all procedures (and main program body) use register bank 0. You can modify register bank for any procedure with "
using n
" directive or use
UsingAny
directive to make the procedure register bank independent. To call a procedure from an interrupt that uses register bank n you can declare that procedure with "
using n
" directive - in this case you will be able to call it only from procedures (including interrupts) declared with "
using n
" directive. If you will call it only from this interrupt then this is the best way. A general solution is to declare this procedure with
UsingAny
directive and then you can call it from any procedure regardless of the register bank the calling procedure is using. This causes a slightly longer code since instructions like MOV Rn, ARm are not possible and the register transfer is done with two instructions via accumulator.
How can I use Turbo51 as a 8051 assembly language compiler?
You can create
assembler procedures
or use
assembler statements
with minimum Pascal code. Remember: program execution starts at the main program body.
How can I use ReadLn without LF (#10) character?
Write your read function to return LF character according to your needs (e.g. replacing CR (#13) character with LF).

---

## Code Examples

### Code Block 1

```pascal
<a href="http://turbo51.com/">Turbo51 - Pascal compiler for 8051 microcontrollers</a>
```

