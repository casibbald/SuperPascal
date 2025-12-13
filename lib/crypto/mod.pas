unit Crypto;

interface

uses
  Crypto_Types,
  Crypto_CRC;

// Re-export types
type
  TCRCParams = Crypto_Types.TCRCParams;
  TCRCTable = Crypto_Types.TCRCTable;
  TCRCContext = Crypto_Types.TCRCContext;

// Re-export constants
const
  CRC16_STANDARD = Crypto_Types.CRC16_STANDARD;
  CRC16_REVERSED = Crypto_Types.CRC16_REVERSED;
  CRC16_CCITT = Crypto_Types.CRC16_CCITT;
  CRC32_STANDARD = Crypto_Types.CRC32_STANDARD;

// Functions are available through imported units:
// - Crypto_CRC.CRCInitTable
// - Crypto_CRC.CRCInit
// - Crypto_CRC.CRCUpdateByte
// - Crypto_CRC.CRCUpdateBlock
// - Crypto_CRC.CRCFinalize
// - Crypto_CRC.CalculateCRC16
// - Crypto_CRC.CalculateCRC32

implementation

end.

