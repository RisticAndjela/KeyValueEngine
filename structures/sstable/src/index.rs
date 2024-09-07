use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use entry_element::entry_element::extract;
use crate::global_sstable_functions::get_key_and_position;

pub struct Index {
    pub file_path:String,
    pub volume:i32
}

impl Index {
    pub fn new(path_to_dir:String,volume:i32)->Self{
        let file_path = format!("{}/index.bin", path_to_dir);
        File::create(file_path.clone()).expect("Failed to create file");
        Index{file_path,volume}
    }
    pub fn read_index_file(&self) -> std::io::Result<()> {
        let index_file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut index_reader = BufReader::new(index_file);

        loop {
            let mut size_buffer = [0u8; 8];
            // Try to read the size of the index entry (8 bytes)
            if index_reader.read_exact(&mut size_buffer).is_err() {
                break; // Reached the end of file
            }
            let index_size = u64::from_be_bytes(size_buffer);

            // Read the key size
            let mut key_size_buffer = [0u8; 8];
            index_reader.read_exact(&mut key_size_buffer)?;
            let key_size = u64::from_be_bytes(key_size_buffer) as usize;

            // Read the key itself
            let mut key_buffer = vec![0u8; key_size];
            index_reader.read_exact(&mut key_buffer)?;

            // Read the position in the data file (8 bytes)
            let mut position_buffer = [0u8; 8];
            index_reader.read_exact(&mut position_buffer)?;
            let position = u64::from_be_bytes(position_buffer);

            let key = String::from_utf8(key_buffer).expect("Invalid UTF-8 sequence");

            println!("Index Entry - Key: {}, Data Position: {}", key, position);
        }

        Ok(())
    }

    pub fn search_for_segment_in_data(&self, start_of_segment:u64, end_of_segment:u64, key_attribute:String) -> (u64, u64) {
        let index_file = OpenOptions::new().read(true).open(&self.file_path).expect("Failed to open data file for appending");
        let mut reader = BufReader::new(index_file);
        reader.seek(SeekFrom::Start(start_of_segment)).expect("Seek failed");
        let key= extract(key_attribute.as_str()).unwrap();
        let mut last_good_position=start_of_segment;
        loop {
            let mut size_buffer = [0u8; 8];
            if reader.read_exact(&mut size_buffer).is_err() { panic!(); }
            let size_to_read = u64::from_be_bytes(size_buffer);

            let mut read_data = vec![0u8; size_to_read as usize];
            if reader.read_exact(&mut read_data).is_err(){panic!();};
            let(founded_key,position) = get_key_and_position(read_data);
            let numbered_value_of_key=extract(founded_key.as_str()).unwrap();
            if numbered_value_of_key>key{ // found bigger, return to one smallest and seek from there in index
                return (last_good_position,position); //since I have first and last from index in summary, I will never get to search the key bigger than last key in summary
            }
            if numbered_value_of_key==key{
                return (position,position); // I have exact match, so don't need to get whole segment
            }
            last_good_position=position;
        }
    }

}