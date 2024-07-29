use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use byteorder::{BigEndian, ReadBytesExt};
use memtable_element::memtable_element::{CRC_LEN, ElementMemtable, KEY_SIZE_LEN, TIMESTAMP_LEN, TOMBSTONE_LEN, VALUE_SIZE_LEN};

pub struct WriteAheadLog {
    pub filename: String,
    pub segment_length: u64,
    pub current_offset: u64,
}

impl WriteAheadLog {
    pub fn read_overflow(&mut self, data: &mut [u8], index_offset: usize) -> Vec<u8> {
        let mut new_file = OpenOptions::new().read(true).open(&self.filename).unwrap();
        new_file.seek(SeekFrom::Start(self.current_offset)).unwrap();
        let bytes_read = new_file.read(&mut data[index_offset..]).unwrap();
        new_file.sync_all().unwrap();
        self.current_offset += bytes_read as u64;
        data.to_vec()
    }

    pub fn read_next(&mut self, file: &mut File) -> (Vec<u8>, bool) {
        let mut file_changed = false;
        let mut all_bytes = Vec::new();

        let mut crc_bytes = vec![0u8; CRC_LEN];
        let bytes_read = file.read(&mut crc_bytes).unwrap();
        if bytes_read == 0 {
            return (vec![], false);
        }
        self.current_offset += bytes_read as u64;
        if bytes_read != CRC_LEN {
            file_changed = true;
            self.current_offset = 0;
            crc_bytes = self.read_overflow(&mut crc_bytes, bytes_read);
        }
        all_bytes.extend(crc_bytes);

        let mut timestamp_bytes = vec![0u8; TIMESTAMP_LEN];
        let bytes_read = file.read(&mut timestamp_bytes).unwrap();
        self.current_offset += bytes_read as u64;
        if bytes_read != TIMESTAMP_LEN {
            file_changed = true;
            self.current_offset = 0;
            timestamp_bytes = self.read_overflow(&mut timestamp_bytes, bytes_read);
        }
        all_bytes.extend(timestamp_bytes);

        let mut tombstone_bytes = vec![0u8; TOMBSTONE_LEN];
        let bytes_read = file.read(&mut tombstone_bytes).unwrap();
        self.current_offset += bytes_read as u64;
        if bytes_read != TOMBSTONE_LEN {
            file_changed = true;
            self.current_offset = 0;
            tombstone_bytes = self.read_overflow(&mut tombstone_bytes, bytes_read);
        }
        all_bytes.extend(tombstone_bytes);

        let mut key_size_bytes = vec![0u8; KEY_SIZE_LEN];
        let bytes_read = file.read(&mut key_size_bytes).unwrap();
        self.current_offset += bytes_read as u64;
        if bytes_read != KEY_SIZE_LEN {
            file_changed = true;
            self.current_offset = 0;
            key_size_bytes = self.read_overflow(&mut key_size_bytes, bytes_read);
        }
        all_bytes.extend(key_size_bytes.clone());

        let mut value_size_bytes = vec![0u8; VALUE_SIZE_LEN];
        let bytes_read = file.read(&mut value_size_bytes).unwrap();
        self.current_offset += bytes_read as u64;
        if bytes_read != VALUE_SIZE_LEN {
            file_changed = true;
            self.current_offset = 0;
            value_size_bytes = self.read_overflow(&mut value_size_bytes, bytes_read);
        }
        all_bytes.extend(value_size_bytes.clone());

        let key_length = (&key_size_bytes[..]).read_u64::<BigEndian>().unwrap();
        let mut key_bytes = vec![0u8; key_length as usize];
        let bytes_read = file.read(&mut key_bytes).unwrap();
        self.current_offset += bytes_read as u64;
        if bytes_read != key_length as usize {
            file_changed = true;
            self.current_offset = 0;
            key_bytes = self.read_overflow(&mut key_bytes, bytes_read);
        }
        all_bytes.extend(key_bytes);

        let value_length = (&value_size_bytes[..]).read_u64::<BigEndian>().unwrap();
        let mut value_bytes = vec![0u8; value_length as usize];
        let bytes_read = file.read(&mut value_bytes).unwrap();
        self.current_offset += bytes_read as u64;
        if bytes_read != value_length as usize {
            file_changed = true;
            self.current_offset = 0;
            value_bytes = self.read_overflow(&mut value_bytes, bytes_read);
        }
        all_bytes.extend(value_bytes);

        (all_bytes, file_changed)
    }

    pub fn append(&mut self, elem: &ElementMemtable) -> std::io::Result<()> {
        let serialized = elem.serialize();
        let serialized_len = serialized.len() as u64;

        if self.current_offset + serialized_len >= self.segment_length {
            self.current_offset = 0;
        }

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.filename)?;

        file.write_all(&serialized)?;
        file.sync_all()?;

        self.current_offset += serialized_len;
        Ok(())
    }

    pub fn iter(&mut self) -> impl Iterator<Item = ElementMemtable> + '_ {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.filename)
            .unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        std::iter::from_fn(move || {
            let (bytes, file_changed) = self.read_next(&mut file);
            if bytes.is_empty() {
                if file_changed {
                    file = OpenOptions::new()
                        .read(true)
                        .open(&self.filename)
                        .unwrap();
                    file.seek(SeekFrom::Start(self.current_offset)).unwrap();
                } else {
                    return None;
                }
            }
            Some(ElementMemtable::deserialize(&bytes))
        })
    }
}