# Turbo51 Variables | Documentation

**URL:** https://turbo51.com/documentation/8051-pascal-variables

---

## Table of Contents

- Turbo51 Variables
  - Pascal Compiler for 8051 Microcontrollers
    - Microcontroller Books

---

## Content

Pascal Compiler for 8051 Microcontrollers
Turbo51 variables can have memory type directives DATA, IDATA, or XDATA which overrides default memory type for variables (IDATA memory with addresses starting from $80 is not available on all 8051 derivatives, some 8051 derivatives have also internal XDATA memory). Boolean variables are stored as bits in bit-addressable DATA memory wich is available in all 8051 derivatives. Volatile directive declares volatile variable - variable which is modified by some interrupt or hardware. Absolute directive declares variable on top of another variable (AbsVar absolute RecordVariable.Field is also possible) or at some absolute address.
Boolean variables can not be passed by reference (8051 has no instruction to reference bit variable by address) and can not be passed as parameter in re-entrant procedures. In such cases you can use system type ByteBool which occupies 1 byte. BitAddressable directive declares variable which will be placed in DATA address space from $20 to $2F - you can access individual bits of such (8-bit) bit-addressable variable with BitAddressableVar.n where n is 0 to 7. Data is always stored in little endian format. Examples:
Var
   EthernetReset:               Boolean absolute P0.7;
   EthernetMode:                Boolean absolute P0.6;

   TempString:                  String [24] XDATA;
   TempChecksum:                Byte;
   TempByte2:                   Byte absolute TempChecksum;
   DelayTimer:                  Word XDATA; Volatile;
   SamplePulse:                 Boolean;
   InputSync:                   Byte; BitAddressable;
   VideoSync1:                  Boolean absolute InputSync.0;
   VideoSync2:                  Boolean absolute InputSync.1;
   LastPCPort:                  TActiveBuffer;

   RemoteTemperatureReadState:  TRemoteTemperatureReadState XDATA;
   TempRemoteTemperature,
   RemoteTemperature:           Array [1..4] of Word XDATA;
   RemoteTemperatureThreshold:  Word XDATA;

 {$IFDEF TEST }
   TempTimer:                   Word; Volatile;
   TxBuffer1:                   Array [0..15] of Byte IDATA;
 {$ENDIF }

   BroadcastReplyTimer_Serial0: Word IDATA; Volatile;

   UART:                        Array [1..4] of TUART XDATA;

   TX_Buffer_Serial0:           TExtendedGeneralPacket XDATA;
   TX_BufferArray_Serial0:      TBufferArray absolute TX_Buffer_Serial0;

   PRX_Buffer_Cmd_Message:      ^TCmd_Message XDATA absolute PRX_Buffer;

   UartData:                    Array [1..4] of TUartData IDATA;
   RX_Buffer_UART:              Array [1..4] of TExtendedGeneralPacket XDATA;

   EEPROM_Data:                 TEEPROM_Data XDATA absolute 0;
You can also declare variables in unit with directive
absolute Forward
which means some (from the unit) unknown memory address. Example:
Unit I2C; Interface Var Ack: Boolean;    SDA: Boolean absolute Forward;    SCL: Boolean absolute Forward;
Main program which uses this unit declares these absolute Forward variables at correct address.
Program Test; Uses I2C; Var  I2C.SCL: Boolean absolute P3.4;     I2C.SDA: Boolean absolute P3.5;

---

## Code Examples

### Code Block 1

```pascal
Var
   EthernetReset:               Boolean absolute P0.7;
   EthernetMode:                Boolean absolute P0.6;

   TempString:                  String [24] XDATA;
   TempChecksum:                Byte;
   TempByte2:                   Byte absolute TempChecksum;
   DelayTimer:                  Word XDATA; Volatile;
   SamplePulse:                 Boolean;
   InputSync:                   Byte; BitAddressable;
   VideoSync1:                  Boolean absolute InputSync.0;
   VideoSync2:                  Boolean absolute InputSync.1;
   LastPCPort:                  TActiveBuffer;

   RemoteTemperatureReadState:  TRemoteTemperatureReadState XDATA;
   TempRemoteTemperature,
   RemoteTemperature:           Array [1..4] of Word XDATA;
   RemoteTemperatureThreshold:  Word XDATA;

 {$IFDEF TEST }
   TempTimer:                   Word; Volatile;
   TxBuffer1:                   Array [0..15] of Byte IDATA;
 {$ENDIF }

   BroadcastReplyTimer_Serial0: Word IDATA; Volatile;

   UART:                        Array [1..4] of TUART XDATA;

   TX_Buffer_Serial0:           TExtendedGeneralPacket XDATA;
   TX_BufferArray_Serial0:      TBufferArray absolute TX_Buffer_Serial0;

   PRX_Buffer_Cmd_Message:      ^TCmd_Message XDATA absolute PRX_Buffer;

   UartData:                    Array [1..4] of TUartData IDATA;
   RX_Buffer_UART:              Array [1..4] of TExtendedGeneralPacket XDATA;

   EEPROM_Data:                 TEEPROM_Data XDATA absolute 0;


```

### Code Block 2

```pascal
Unit I2C; Interface Var Ack: Boolean;    SDA: Boolean absolute Forward;    SCL: Boolean absolute Forward;

```

### Code Block 3

```pascal
Program Test; Uses I2C; Var  I2C.SCL: Boolean absolute P3.4;     I2C.SDA: Boolean absolute P3.5;

```

