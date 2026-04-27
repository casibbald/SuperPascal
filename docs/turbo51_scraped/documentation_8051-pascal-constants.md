# Turbo51 Constants | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-constants

---

## Table of Contents

- Turbo51 Constants
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Turbo51 constants can be of any ordinal type. Typed constants are stored in CODE memory (in little endian format) and can not be modified. Boolean typed constants are not possible because boolean data can only be stored as bits in bit-addressable DATAmemory (wich is available in all 8051 derivatives), but you can use ByteBool or similar typed constants. Some examples:
Const
  SystemClock              = 22118400;
  ConversionClockValue     = (SystemClock div ConversionClock - 1) shl 3;
  PeriodicTimerValue       = - SystemClock div 12 div TimerInterruptsPerSecond;

  SampleFrequency_10       = SamplesPerBit * 11875;
  RDS_SampleRateTimerValue = - 10 * SystemClock div SampleFrequency_10;
  GroupTime                = 1040000 div 11875;
  SMB0CRValue              = (2 - SystemClock div 2 div SMBusClock) and $FF;
  BaudRateTimerValue1      = - SystemClock div 32 div BaudRateSerial1;

  BaudRateTimerValue_19200 = Word (- SystemClock div 32 div 19200);
  BaudRateTimerValue_38400 = Word (- SystemClock div 32 div 38400);

  BaudRateTimerValue: Array [$01..$02] of Word = (
    BaudRateTimerValue_19200,
    BaudRateTimerValue_38400);

  LedTime                  =  30;

  SampleTable_0_0_0: Array [0..SamplesPerBit - 1] of Word =
    ({$I Table_0_0_0.inc });

  VersionHi = $01;
  VersionLo = $00;

  Signature = $8051;     // Identify 8051 microcontroller

  Greeting = 'PASCAL 8051'#0;
  GreetingString: Array [0..Length (Greeting) - 1] of Char = Greeting;

  ManufacturerKeyData: Array [0..7] of Byte = ($DA, $FE, $02, $40, $03, $3D, $B5, $CA);

  BaudRateTimerValue = Word (- 22118400 div 32 div 9600);

  LedTime         =    30;
  RS485_Time      =     2;
  SetupTime       = 30000;
  NoKeyTime       =   100;

  DafaultRelayLongPulseTime  =   30000 div 32;
  DafaultLightPulseTime      = 1800000 div 32;
  DefaultRelayShortPulseTime =     384 div 32;
  DefaultRelayOffPulseTime   =    1500 div 32;

  Relay_ON  = LowLevel;
  Relay_OFF = HighLevel;

  MotorUp   = 1;
  MotorDown = 0;

  HexChar: Array [0..15] of Char = '0123456789ABCDEF';

  BlockStart    = $AA;
  BlockStartLNG = $CA;

  bpBlockStart         = 0;
  bpDestinationAddress = 1;
  bpcommand            = 2;
  bpParameter1         = 3;
  bpParameter2         = 4;
  bpSourceAddress      = 5;
  bpChecksum           = 6;

  bpParameter3         = 7;
  bpParameter4         = 8;

  LNG_ModuleID         = $A0;

  Cmd_Relay            = $21;

  Segments: Array [' '..'U'] of Byte = (
    %00000000, { ' ' }
    %00000000, { '!' }
    %00100010, { '"' }
    %00000000, { '#' }
    %00000000, { '$' }
    %00000000, { '%' }
    %00000000, { '&' }
    %00000000, { ''' }
    %00111001, { '(' }
    %00001111, { ')' }
    %00000000, { '*' }
    %00000000, { '+' }
    %00000000, { ',' }
    %01000000, { '-' }
    %00000000, { '.' }
    %01001001, { '/' }
    %00111111, { '0' }
    %00000110, { '1' }
    %01011011, { '2' }
    %01001111, { '3' }
    %01100110, { '4' }
    %01101101, { '5' }
    %01111101, { '6' }
    %00100111, { '7' }
    %01111111, { '8' }
    %01101111, { '9' }
    %00000000, { ':' }
    %00000000, { ';' }
    %00000000, { '<' }
    %01001000, { '=' }
    %00000000, { '>' }
    %00000000, { '?' }
    %00000000, { '@' }
    %01110111, { 'A' }
    %01111100, { 'B' }
    %00111001, { 'C' }
    %01011110, { 'D' }
    %01111001, { 'E' }
    %01110001, { 'F' }
    %00000000, { 'G' }
    %01110110, { 'H' }
    %00000000, { 'I' }
    %00000000, { 'J' }
    %00000000, { 'K' }
    %00000000, { 'L' }
    %00000000, { 'M' }
    %01010100, { 'N' }
    %01011100, { 'O' }
    %01110011, { 'P' }
    %01010111, { 'Q' }
    %01010000, { 'R' }
    %01101101, { 'S' }
    %01111000, { 'T' }
    %00011100);{ 'U' }

---

## Code Examples

### Code Block 1

```pascal
Const
  SystemClock              = 22118400;
  ConversionClockValue     = (SystemClock div ConversionClock - 1) shl 3;
  PeriodicTimerValue       = - SystemClock div 12 div TimerInterruptsPerSecond;

  SampleFrequency_10       = SamplesPerBit * 11875;
  RDS_SampleRateTimerValue = - 10 * SystemClock div SampleFrequency_10;
  GroupTime                = 1040000 div 11875;
  SMB0CRValue              = (2 - SystemClock div 2 div SMBusClock) and $FF;
  BaudRateTimerValue1      = - SystemClock div 32 div BaudRateSerial1;

  BaudRateTimerValue_19200 = Word (- SystemClock div 32 div 19200);
  BaudRateTimerValue_38400 = Word (- SystemClock div 32 div 38400);

  BaudRateTimerValue: Array [$01..$02] of Word = (
    BaudRateTimerValue_19200,
    BaudRateTimerValue_38400);

  LedTime                  =  30;

  SampleTable_0_0_0: Array [0..SamplesPerBit - 1] of Word =
    ({$I Table_0_0_0.inc });

  VersionHi = $01;
  VersionLo = $00;

  Signature = $8051;     // Identify 8051 microcontroller

  Greeting = 'PASCAL 8051'#0;
  GreetingString: Array [0..Length (Greeting) - 1] of Char = Greeting;

  ManufacturerKeyData: Array [0..7] of Byte = ($DA, $FE, $02, $40, $03, $3D, $B5, $CA);

  BaudRateTimerValue = Word (- 22118400 div 32 div 9600);

  LedTime         =    30;
  RS485_Time      =     2;
  SetupTime       = 30000;
  NoKeyTime       =   100;

  DafaultRelayLongPulseTime  =   30000 div 32;
  DafaultLightPulseTime      = 1800000 div 32;
  DefaultRelayShortPulseTime =     384 div 32;
  DefaultRelayOffPulseTime   =    1500 div 32;

  Relay_ON  = LowLevel;
  Relay_OFF = HighLevel;

  MotorUp   = 1;
  MotorDown = 0;

  HexChar: Array [0..15] of Char = '0123456789ABCDEF';

  BlockStart    = $AA;
  BlockStartLNG = $CA;

  bpBlockStart         = 0;
  bpDestinationAddress = 1;
  bpcommand            = 2;
  bpParameter1         = 3;
  bpParameter2         = 4;
  bpSourceAddress      = 5;
  bpChecksum           = 6;

  bpParameter3         = 7;
  bpParameter4         = 8;

  LNG_ModuleID         = $A0;

  Cmd_Relay            = $21;

  Segments: Array [' '..'U'] of Byte = (
    %00000000, { ' ' }
    %00000000, { '!' }
    %00100010, { '"' }
    %00000000, { '#' }
    %00000000, { '$' }
    %00000000, { '%' }
    %00000000, { '&' }
    %00000000, { ''' }
    %00111001, { '(' }
    %00001111, { ')' }
    %00000000, { '*' }
    %00000000, { '+' }
    %00000000, { ',' }
    %01000000, { '-' }
    %00000000, { '.' }
    %01001001, { '/' }
    %00111111, { '0' }
    %00000110, { '1' }
    %01011011, { '2' }
    %01001111, { '3' }
    %01100110, { '4' }
    %01101101, { '5' }
    %01111101, { '6' }
    %00100111, { '7' }
    %01111111, { '8' }
    %01101111, { '9' }
    %00000000, { ':' }
    %00000000, { ';' }
    %00000000, { '<' }
    %01001000, { '=' }
    %00000000, { '>' }
    %00000000, { '?' }
    %00000000, { '@' }
    %01110111, { 'A' }
    %01111100, { 'B' }
    %00111001, { 'C' }
    %01011110, { 'D' }
    %01111001, { 'E' }
    %01110001, { 'F' }
    %00000000, { 'G' }
    %01110110, { 'H' }
    %00000000, { 'I' }
    %00000000, { 'J' }
    %00000000, { 'K' }
    %00000000, { 'L' }
    %00000000, { 'M' }
    %01010100, { 'N' }
    %01011100, { 'O' }
    %01110011, { 'P' }
    %01010111, { 'Q' }
    %01010000, { 'R' }
    %01101101, { 'S' }
    %01111000, { 'T' }
    %00011100);{ 'U' }


```

