use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::ops::Deref;
use std::ptr::read;
use entry_element::entry_element::{extract, EntryElement as record};
pub struct Data{
    pub file_path:String, // path to directory in which is held all the data, index, summary and filter files
}
impl Data{
    pub fn new(path_to_dir:String)->Self{
        let file_path = format!("{}/data.bin", path_to_dir);
        File::create(file_path.clone()).expect("Failed to create file");
        Data{file_path:file_path.clone()}
    }
    pub fn read_data_file(&self,from_position:u64) -> std::io::Result<()> {
        let data_file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut data_reader = BufReader::new(data_file);
        data_reader.seek(SeekFrom::Start(from_position)).expect("Failed to seek");
        let mut byte_position = 0u64;
        let mut last_bp=0;
        loop {
            let mut size_buffer = [0u8; 8];
            if data_reader.read_exact(&mut size_buffer).is_err() {
                break;
            }
            byte_position += 8;
            let record_size = u64::from_be_bytes(size_buffer) as usize;

            let mut record_buffer = vec![0u8; record_size];
            data_reader.read_exact(&mut record_buffer)?;
            byte_position += record_size as u64;
            let record = record::deserialize(&record_buffer);
            println!("From byte position: {} To the byte position: {}, Record: {:?}",last_bp, byte_position, record);
            last_bp=byte_position+1;
        }

        Ok(())
    }
    pub fn search_from_segment(&self, start_position: u64, key_attribute:String) -> record {
        let data_file = OpenOptions::new().read(true).open(&self.file_path).expect("Failed to open data file for appending");
        let mut reader = BufReader::new(data_file);
       reader.seek(SeekFrom::Start(start_position)).unwrap();
        let key= extract(key_attribute.as_str()).unwrap();
        loop {
            let mut size_buffer = [0u8; 8]; // 8-byte buffer to read u64 size
            if reader.read_exact(&mut size_buffer).is_err() { panic!(); }
            let size_to_read = u64::from_be_bytes(size_buffer);

            let mut read_data = vec![0u8; size_to_read as usize];
            if reader.read_exact(&mut read_data).is_err(){panic!();};
            let record = record::deserialize(read_data.deref());
            let numbered_value_of_key=record.extract_number_from_key().unwrap();
            if numbered_value_of_key==key{
                return record; // I have exact match, so don't need to get whole segment
            }
            if numbered_value_of_key>key{panic!("couldnt find")}
            }
    }
}