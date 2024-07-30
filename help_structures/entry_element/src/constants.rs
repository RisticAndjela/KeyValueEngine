// SAVING LIKE THIS:
// (0B)crc(4B) - (4B)timestamp(20B) - (20B)tombstone(21B)
//      4B               16B                   1B
// - (21B)key_size(29B) - (29B)value_size(37B) - (37B)key(nB) - (nB)value(mB)
//           8B                   8B                   xB            yB

pub const CRC_LEN: usize = 4;
pub const TIMESTAMP_LEN: usize = 16;
pub const TOMBSTONE_LEN: usize = 1;
pub const KEY_SIZE_LEN: usize = 8;
pub const VALUE_SIZE_LEN: usize = 8;

pub const CRC_START: usize = 0;
pub const TIMESTAMP_START: usize = CRC_START + CRC_LEN;
pub const TOMBSTONE_START: usize = TIMESTAMP_START + TIMESTAMP_LEN;
pub const KEY_SIZE_START: usize = TOMBSTONE_START + TOMBSTONE_LEN;
pub const VALUE_SIZE_START: usize = KEY_SIZE_START + KEY_SIZE_LEN;
pub const KEY_START: usize = VALUE_SIZE_START + VALUE_SIZE_LEN;

pub const CONST_LEN_OF_ENTRY :usize= KEY_START;