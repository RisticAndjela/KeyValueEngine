use crc32fast::Hasher;
use std::convert::TryInto;

pub const CRC_LEN: usize = 4;
pub const TIMESTAMP_LEN: usize = 8;
pub const TOMBSTONE_LEN: usize = 1;
pub const KEY_SIZE_LEN: usize = 8;
pub const VALUE_SIZE_LEN: usize = 8;

pub const CRC_START: usize = 0;
pub const TIMESTAMP_START: usize = CRC_START + CRC_LEN;
pub const TOMBSTONE_START: usize = TIMESTAMP_START + TIMESTAMP_LEN;
pub const KEY_SIZE_START: usize = TOMBSTONE_START + TOMBSTONE_LEN;
pub const VALUE_SIZE_START: usize = KEY_SIZE_START + KEY_SIZE_LEN;
pub const KEY_START: usize = VALUE_SIZE_START + VALUE_SIZE_LEN;

#[derive(Debug)]
pub struct ElementMemtable {
    pub key: String,
    pub value: Vec<u8>,
    pub tombstone: bool,
    pub timestamp: i64,
}

impl ElementMemtable {
    pub fn crc32(data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let key_size = self.key.len() as u64;
        let value_size = self.value.len() as u64;

        let timestamp_bytes = self.timestamp.to_be_bytes();
        let tombstone_byte = if self.tombstone { 255 } else { 0 };
        let key_size_bytes = key_size.to_be_bytes();
        let value_size_bytes = value_size.to_be_bytes();
        let key_bytes = self.key.as_bytes();
        let value_bytes = &self.value;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&timestamp_bytes);
        bytes.push(tombstone_byte);
        bytes.extend_from_slice(&key_size_bytes);
        bytes.extend_from_slice(&value_size_bytes);
        bytes.extend_from_slice(key_bytes);
        bytes.extend_from_slice(value_bytes);

        let crc_value = Self::crc32(&bytes);
        let crc_bytes = crc_value.to_be_bytes();

        let mut serialized = Vec::new();
        serialized.extend_from_slice(&crc_bytes);
        serialized.extend_from_slice(&bytes);

        serialized
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        let crc_bytes = &bytes[..CRC_LEN];
        let rest_bytes = &bytes[CRC_LEN..];

        let timestamp_bytes = &bytes[TIMESTAMP_START..TOMBSTONE_START];
        let tombstone_byte = bytes[TOMBSTONE_START];
        let tombstone = tombstone_byte != 0;

        let key_size = u64::from_be_bytes(bytes[KEY_SIZE_START..VALUE_SIZE_START].try_into().unwrap());
        let value_size = u64::from_be_bytes(bytes[VALUE_SIZE_START..KEY_START].try_into().unwrap());

        let key_start = KEY_START;
        let key_end = key_start + key_size as usize;
        let value_start = key_end;
        let value_end = value_start + value_size as usize;

        let key_bytes = &bytes[key_start..key_end];
        let value_bytes = &bytes[value_start..value_end];

        let crc_current_value = Self::crc32(rest_bytes);
        let crc_previous_value = u32::from_be_bytes(crc_bytes.try_into().unwrap());

        if crc_previous_value != crc_current_value {
            println!("WARNING! THE VALUE MAY NOT BE VALID!");
        }

        ElementMemtable {
            key: String::from_utf8(key_bytes.to_vec()).unwrap(),
            value: value_bytes.to_vec(),
            tombstone,
            timestamp: i64::from_be_bytes(timestamp_bytes.try_into().unwrap()),
        }
    }
    pub fn size(&self) -> u64 {
        // Calculate the size of each field. The size of a string can be approximated as the length of its bytes.
        // Add extra bytes for overhead if needed.
        let key_size = self.key.len();
        let value_size = self.value.len();
        let tombstone_size = std::mem::size_of_val(&self.tombstone);
        let timestamp_size = std::mem::size_of_val(&self.timestamp);

        // Adjust the calculation as needed based on your specific use case
        (key_size + value_size + tombstone_size + timestamp_size )as u64
    }
}
impl PartialEq for ElementMemtable {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key &&
            self.value == other.value &&
            self.tombstone == other.tombstone &&
            self.timestamp == other.timestamp
    }
}