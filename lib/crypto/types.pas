unit Crypto_Types;

interface

type
  // CRC parameters
  TCRCParams = record
    Width: Byte;      // CRC width in bits (16 or 32)
    Poly: LongWord;   // Polynomial (reflected form)
    Init: LongWord;   // Initial value
    RefIn: Boolean;   // Reflect input bytes
    RefOut: Boolean;  // Reflect output
    XorOut: LongWord; // Final XOR value
  end;
  
  // CRC lookup table (256 entries)
  TCRCTable = array[0..255] of LongWord;
  
  // CRC context for incremental calculation
  TCRCContext = record
    Params: TCRCParams;
    Table: TCRCTable;
    Value: LongWord;  // Current CRC value
  end;

const
  // Standard CRC-16 parameters
  CRC16_STANDARD: TCRCParams = (
    Width: 16;
    Poly: $8005;      // CRC-16-IBM polynomial (reflected)
    Init: $0000;
    RefIn: True;
    RefOut: True;
    XorOut: $0000
  );
  
  // CRC-16 (reversed)
  CRC16_REVERSED: TCRCParams = (
    Width: 16;
    Poly: $A001;      // CRC-16-IBM polynomial (non-reflected)
    Init: $FFFF;
    RefIn: False;
    RefOut: False;
    XorOut: $0000
  );
  
  // CRC-16-CCITT
  CRC16_CCITT: TCRCParams = (
    Width: 16;
    Poly: $1021;      // CRC-16-CCITT polynomial
    Init: $FFFF;
    RefIn: False;
    RefOut: False;
    XorOut: $0000
  );
  
  // Standard CRC-32 parameters (used in ZIP, PNG, etc.)
  CRC32_STANDARD: TCRCParams = (
    Width: 32;
    Poly: $EDB88320;  // CRC-32 polynomial (reflected)
    Init: $FFFFFFFF;
    RefIn: True;
    RefOut: True;
    XorOut: $FFFFFFFF
  );

implementation

end.

