use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::convert::TryInto;
use entry_element::entry_element::EntryElement as record;
pub struct Index {
    key: String,
    len_key: u64,
    position_in_data: u64,
    index_size: u64,
    from_index_file: bool,
}

impl Index {
    pub fn from_data(data: &record, position: u64) -> Self {
        let mut index = Index {
            key: data.key.clone(),
            len_key: data.len_key,
            position_in_data: position,
            index_size: 0,
            from_index_file: true,
        };
        index.index_size = index.calculate_size();
        index
    }

    pub fn new(key: String, len_key: u64, position: u64) -> Self {
        let mut index = Index {
            key,
            len_key,
            position_in_data: position,
            index_size: 0,
            from_index_file: true,
        };
        index.index_size = index.calculate_size();
        index
    }

    pub fn calculate_size(&self) -> u64 {
        let mut size = 24;
        size += self.key.len();
        if !self.from_index_file {
            size += 5;
        }
        size.try_into().unwrap()
    }

    pub fn calculate_size_var(&self) -> u64 {
        let mut buf = vec![0u8; 1000];
        let mut size = 0i32;

        size += put_uvarint(&mut buf[size..], self.position_in_data) as u64;
        size += put_uvarint(&mut buf[size..], self.len_key) as u64;
        size += self.len_key;
        size += put_uvarint(&mut buf[size..], self.index_size) as u64;
        size += 2;

        size
    }

    pub fn write_index(&self, file: &mut File, is_index_file: bool) -> io::Result<()> {
        let mut buf_size = 24 + self.key.len();
        let mut i = 0;

        if !is_index_file {
            buf_size += 5;
            i += 5;
        }

        let mut buf = vec![0u8; buf_size];

        if !is_index_file {
            buf[..5].copy_from_slice(b"index");
        }

        buf[i..i + 8].copy_from_slice(&self.position_in_data.to_be_bytes());
        buf[i + 8..i + 16].copy_from_slice(&self.len_key.to_be_bytes());

        let after_key = i + 16 + self.key.len();
        buf[i + 16..after_key].copy_from_slice(self.key.as_bytes());
        buf[after_key..].copy_from_slice(&self.index_size.to_be_bytes());

        println!("{:?}", buf);

        file.write_all(&buf)
    }

    pub fn write_var_index(&self, file: &mut File, is_index_file: bool) -> io::Result<()> {
        let varsize = self.calculate_size_var();
        let mut i = 0;

        if !is_index_file {
            i += 5;
        }

        let mut buf = vec![0u8; varsize as usize];

        if !is_index_file {
            buf[..5].copy_from_slice(b"index");
        }

        i += put_uvarint(&mut buf[i..], varsize);
        i += put_uvarint(&mut buf[i..], self.position_in_data);
        i += put_uvarint(&mut buf[i..], self.len_key);

        let after_key = i + self.key.len();
        buf[i..after_key].copy_from_slice(self.key.as_bytes());
        i += put_uvarint(&mut buf[after_key..], self.index_size);

        println!("{:?}", &buf[..self.key.len() + i]);

        file.write_all(&buf[..self.key.len() + i])
    }

    pub fn read_index(file: &mut File, position: u64, is_index_file: bool) -> io::Result<Index> {
        let mut ind = Index::new(String::new(), 0, 0);
        file.seek(SeekFrom::Start(position))?;

        let mut buf_sized = 16;
        let mut i = 0;

        if !is_index_file {
            buf_sized += 5;
            i = 5;
        }

        let mut buf = vec![0u8; buf_sized];
        file.read_exact(&mut buf)?;

        ind.position_in_data = u64::from_be_bytes(buf[i..i + 8].try_into().unwrap());
        ind.len_key = u64::from_be_bytes(buf[i + 8..i + 16].try_into().unwrap());

        buf_sized = ind.len_key as usize + i;
        buf = vec![0u8; buf_sized];
        file.read_exact(&mut buf)?;

        ind.key = String::from_utf8(buf).unwrap();
        println!("{}", ind.key);

        buf_sized = 8;
        buf = vec![0u8; buf_sized];
        file.read_exact(&mut buf)?;

        ind.index_size = u64::from_be_bytes(buf.try_into().unwrap());
        Ok(ind)
    }

    pub fn read_var_index(file: &mut File, position: u64, is_index_file: bool) -> io::Result<Index> {
        let mut ind = Index::new(String::new(), 0, 0);
        file.seek(SeekFrom::Start(position))?;

        let mut read_first = 2;
        let mut num = 0;

        if !is_index_file {
            read_first += 5;
            num += 5;
        }

        let mut buf1 = vec![0u8; read_first];
        file.read_exact(&mut buf1)?;

        let size_ind = read_uvarint(&buf1[num..num + 2]);

        let mut buf = vec![0u8; size_ind as usize];
        file.read_exact(&mut buf)?;

        let mut seek = 0;
        let (position_in_data, len1) = read_uvarint(&buf[seek..]);
        seek += len1;

        ind.position_in_data = position_in_data;

        let (len_key, len2) = read_uvarint(&buf[seek..]);
        seek += len2;

        ind.len_key = len_key;
        ind.key = String::from_utf8(buf[seek..seek + len_key as usize].to_vec()).unwrap();

        ind.index_size = read_uvarint(&buf[seek + len_key as usize..]).0;

        Ok(ind)
    }
}

// Helper functions for encoding/decoding varint
fn put_uvarint(buf: &mut [u8], mut x: u64) -> usize {
    let mut i = 0;
    while x >= 0x80 {
        buf[i] = (x as u8) | 0x80;
        x >>= 7;
        i += 1;
    }
    buf[i] = x as u8;
    i + 1
}

fn read_uvarint(buf: &[u8]) -> (u64, usize) {
    let mut x = 0;
    let mut s = 0;
    for (i, b) in buf.iter().enumerate() {
        if *b < 0x80 {
            if i > 9 || i == 9 && *b > 1 {
                return (0, 0); // overflow
            }
            return (x | ((*b as u64) << s), i + 1);
        }
        x |= ((*b as u64) & 0x7f) << s;
        s += 7;
    }
    (0, 0)
}