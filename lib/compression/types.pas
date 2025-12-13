unit Compression_Types;

interface

uses
  Math_Types;

type
  // Compression algorithm types
  TCompressionType = (ctRLE, ctLZ77);
  
  // Compression result
  TCompressionResult = record
    Success: Boolean;
    CompressedSize: LongInt;
    OriginalSize: LongInt;
    Ratio: Fixed16;  // Compression ratio (compressed/original) in Q8.8
  end;
  
  // RLE compression parameters
  TRLEParams = record
    MaxRunLength: Byte;  // Maximum run length (default 255)
    MinRunLength: Byte;  // Minimum run length to encode (default 3)
  end;
  
  // LZ77 compression parameters
  TLZ77Params = record
    WindowSize: Word;     // Sliding window size (default 4096)
    LookAheadSize: Word; // Look-ahead buffer size (default 18)
    MinMatchLength: Byte; // Minimum match length (default 2)
  end;
  
  // Compression statistics
  TCompressionStats = record
    LiteralBytes: LongInt;
    EncodedBytes: LongInt;
    TotalBytes: LongInt;
  end;

const
  // Default RLE parameters
  RLE_DEFAULT_MAX_RUN: Byte = 255;
  RLE_DEFAULT_MIN_RUN: Byte = 3;
  
  // Default LZ77 parameters
  LZ77_DEFAULT_WINDOW: Word = 4096;
  LZ77_DEFAULT_LOOKAHEAD: Word = 18;
  LZ77_DEFAULT_MIN_MATCH: Byte = 2;
  
  // LZ77 file format constants
  LZ77_FILE_ID: LongWord = $37375A4C;  // 'LZ77' in little-endian
  LZ77_TAG_SIZE: Byte = 8;  // 8 units per compression tag

implementation

end.

