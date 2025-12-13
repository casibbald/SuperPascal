unit Compression;

interface

uses
  Compression_Types,
  Compression_RLE,
  Compression_LZ77;

// Re-export types
type
  TCompressionType = Compression_Types.TCompressionType;
  TCompressionResult = Compression_Types.TCompressionResult;
  TRLEParams = Compression_Types.TRLEParams;
  TLZ77Params = Compression_Types.TLZ77Params;
  TCompressionStats = Compression_Types.TCompressionStats;

// Re-export constants
const
  RLE_DEFAULT_MAX_RUN = Compression_Types.RLE_DEFAULT_MAX_RUN;
  RLE_DEFAULT_MIN_RUN = Compression_Types.RLE_DEFAULT_MIN_RUN;
  LZ77_DEFAULT_WINDOW = Compression_Types.LZ77_DEFAULT_WINDOW;
  LZ77_DEFAULT_LOOKAHEAD = Compression_Types.LZ77_DEFAULT_LOOKAHEAD;
  LZ77_DEFAULT_MIN_MATCH = Compression_Types.LZ77_DEFAULT_MIN_MATCH;
  LZ77_FILE_ID = Compression_Types.LZ77_FILE_ID;
  LZ77_TAG_SIZE = Compression_Types.LZ77_TAG_SIZE;

// Functions are available through imported units:
// - Compression_RLE.RLECompress
// - Compression_RLE.RLEDecompress
// - Compression_RLE.RLECalculateRatio
// - Compression_LZ77.LZ77Compress
// - Compression_LZ77.LZ77Decompress
// - Compression_LZ77.LZ77CalculateRatio

implementation

end.

