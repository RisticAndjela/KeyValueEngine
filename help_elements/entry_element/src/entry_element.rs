use crc32fast::Hasher;
use std::convert::TryInto;
use crate::constants::{CRC_LEN, KEY_SIZE_START, KEY_START, TIMESTAMP_START, TOMBSTONE_START, VALUE_SIZE_START};
#[derive(Debug,Clone,PartialEq)]
pub struct EntryElement {
    pub key: String,
    pub value: Vec<u8>,
    pub tombstone: bool, //true is deleted
    pub timestamp: i64,
}

impl EntryElement {
    pub fn new(key:String,value:Vec<u8>,timestamp:i64)->Self{EntryElement{key,value,tombstone:false,timestamp}}
    pub fn empty()->Self{EntryElement{key:"".to_string(),value:vec![],tombstone:true,timestamp:0}}
    pub fn delete(&mut self){ self.tombstone=true; }
    pub fn is_irrelevant(&self) ->bool{self.key=="".to_string() || self.tombstone==true}
    fn crc32(data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }
    pub fn serialize(&self) -> Vec<u8> {
        let key_size = self.key.len() as u64;
        let value_size = self.value.len() as u64;

        let timestamp_bytes = self.timestamp.to_be_bytes();
        let tombstone_byte = if self.tombstone { 1 } else { 0 };
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

        EntryElement {
            key: String::from_utf8(key_bytes.to_vec()).unwrap(),
            value: value_bytes.to_vec(),
            tombstone,
            timestamp: i64::from_be_bytes(timestamp_bytes.try_into().unwrap()),
        }
    }
    pub fn size(&self) -> u64 {
        self.serialize().len() as u64
    }
    pub fn extract_number_from_key(&self) -> Option<i64> {
        let key= self.key.as_str();
        extract(key)
    }
}

pub fn extract(key:&str) -> Option<i64> {
    let prefix = "key";
    if key.starts_with(prefix) {
        let number_part = &key[prefix.len()..];
        match number_part.parse::<i64>() {
            Ok(number) => Some(number),
            Err(_) => None,
        }
    } else {
        None
    }
}