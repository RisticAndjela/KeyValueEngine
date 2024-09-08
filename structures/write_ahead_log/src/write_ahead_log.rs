use std::fs::{File, OpenOptions};
use std::{fs};
use std::fmt::format;
use entry_element::entry_element::{EntryElement};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

#[derive(Clone,Debug)]
pub struct WriteAheadLog {
    pub storage_path: String, // path to all wal files
    pub segment_length: u64, // we can predefine this but all segments will be same length
    pub offset:u64, // we need to be able to read elements one by one and start from the one we stopped at, this offset is global meaning all files as well will be included not just from the last one
    pub max_segments_in_memory:u64, // kinda like cache memory, I don't need old ones
    pub max_offset:u64
}
impl WriteAheadLog {
    pub fn open(storage_path:String,segment_length:u64, max_segments_in_memory: u64)->Self{
        let mut wal =WriteAheadLog{storage_path,segment_length,offset:0,max_segments_in_memory,max_offset:0};
        for one in wal.get_all_files(){
            let file = File::open(&one).expect("Failed to open WAL file");
            let mut reader = BufReader::new(file);
            let total_size = reader.get_ref().metadata().unwrap().len();
            wal.max_offset+=total_size;
        }
        wal
    }
    pub fn new(directory_path: String, segment_length: u64, max_segments_in_memory: u64) -> Self {
        let mut wal = WriteAheadLog {
            storage_path: directory_path,
            segment_length,
            offset: 0,
            max_segments_in_memory,
            max_offset:0
        };
        // Create the first file and write 0 to indicate it starts with a full element
        wal.add_new_file(false);
        wal
    }
    pub fn add_new_file(&mut self, continue_previous_byte: bool) {
        // continue_previous_byte indicates whether this file starts with a continued element
        let mut index = (self.offset.clone() - self.offset.clone() % self.segment_length) / self.segment_length;
        if index >= self.max_segments_in_memory {
            self.remove_oldest_file();
            index -= 1;
        }
        index += 1;

        let new_filename = get_name(self.storage_path.clone(), index as i32);
        let mut file = File::create(&new_filename).expect("Cannot create new WAL file");

        // Write 1 if this file is a continuation of a previous element, otherwise write 0
        let start_byte = if continue_previous_byte { 1 } else { 0 };
        file.write_all(&[start_byte]).expect("Failed to write start byte to new WAL file");

        self.offset = (self.get_all_files().len() - 1) as u64 * self.segment_length;
        self.max_offset = (self.get_all_files().len() - 1) as u64 * self.segment_length;
    }
    pub fn add_record(&mut self,entry: EntryElement){
        // println!("{}vs{}",self.offset,self.max_offset);
        self.offset=self.max_offset;
        let serialized=entry.serialize();
        // println!("Add {}",entry.key);
        self.add(serialized);
    }
    pub fn add(&mut self,data:Vec<u8>){
        let data_vec_size=data.len() as u64;
        let (file_path,file_offset)=self.get_file_and_offset_of_current_file(self.offset);
        let left_space_in_current_segment = self.segment_length - file_offset;
        if left_space_in_current_segment > data_vec_size + 8 {
            // println!("add whole");
            let mut vec = vec![];
            vec.extend(data_vec_size.to_be_bytes());
            vec.extend(data);
            self.write(vec);
            if left_space_in_current_segment-data_vec_size-8<9{
                let file = OpenOptions::new().append(true).open(&file_path).expect("Failed to open data file for appending");
                let mut writer = BufWriter::new(file);
                writer.seek(SeekFrom::Start(self.offset)).expect("Seek is wrong");
                writer.write_all(&"*".as_bytes()).unwrap(); // end of each segment is '*'
                writer.flush().expect("Failed to flush writer");
                self.offset = self.segment_length * (extract_index_from_name(file_path.as_str()) as u64);
                self.max_offset = self.segment_length * (extract_index_from_name(file_path.as_str()) as u64);
                self.add_new_file(false);
            }
        }
        else if left_space_in_current_segment > 9{
            let first_half_size = left_space_in_current_segment - 8;

            let first_half = &data[..first_half_size as usize];
            let mut first_part = vec![];
            // println!("left: {}",left_space_in_current_segment);
            first_part.extend(first_half_size.to_be_bytes());
            first_part.extend(first_half);
            // println!("duzina: {}",first_half.len());
            first_part.extend('*'.to_string().as_bytes());
            self.write(first_part);

            // Write the remaining data in the next segment
            let mut second_half = data_vec_size.to_be_bytes().to_vec(); // to subtract for seek from back later
            second_half.extend(data[first_half_size as usize..].to_vec());
            self.add_new_file(true);  // the next file is a continuation
            self.add(second_half);  // recursively add the remaining data
        }
        else{
            // end this go for new
            let file = OpenOptions::new().append(true).open(&file_path).expect("Failed to open data file for appending");
            let mut writer = BufWriter::new(file);
            writer.seek(SeekFrom::Start(self.offset)).expect("Seek is wrong");
            writer.write_all(&"*".as_bytes()).unwrap(); // end of each segment is '*'
            writer.flush().expect("Failed to flush writer");
            self.offset = self.segment_length * (extract_index_from_name(file_path.as_str()) as u64) ;
            self.max_offset = self.segment_length * (extract_index_from_name(file_path.as_str()) as u64) ;
            self.add_new_file(false);
            self.add(data);
        }
    }
    pub fn write(&mut self, data: Vec<u8>) {
        let (file_path, offset) = self.get_file_and_offset_of_current_file(self.offset);
        let file = OpenOptions::new().append(true).open(&file_path).expect("Failed to open data file for appending");
        let mut writer = BufWriter::new(file);
        writer.seek(SeekFrom::Start(offset)).expect("Seek is wrong");
        writer.write_all(&data).unwrap();
        writer.flush().expect("Failed to flush writer");
        self.offset += data.len() as u64;
        self.max_offset+=data.len() as u64;

        if self.offset % self.segment_length == 0 {
            self.add_new_file(false);  // Start a new file and it's not a continuation
        }
    }
    pub fn get_all_files(&mut self) -> Vec<String> {
        let mut file_names = Vec::new();
        if let Ok(entries) = fs::read_dir(self.storage_path.clone()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name_str) = file_name.to_str() {
                                file_names.push(file_name_str.to_string());
                            }
                        }
                    }
                }
            }
        }

        file_names
    }
    pub fn remove_oldest_file(&mut self){
        //remove oldest file
        let files=self.get_all_files();
        let removed_file=get_name(self.storage_path.clone(),1);
        fs::remove_file(&removed_file).expect("cannot remove :(");
        // Rename left files
        let mut working_on_index = 2;
        for file in files.iter().skip(1) {
            working_on_index -= 1;
            let new_name = get_name(self.storage_path.clone(),working_on_index);
            fs::rename(&format!("{}/{file}",self.storage_path), &new_name).expect("cannot rename :(");
            working_on_index +=2;
        }
        let offset_on_index=(self.offset.clone() - self.offset.clone() % self.segment_length) / self.segment_length + 1;
        if offset_on_index == 1 {
            self.offset=0;
            self.max_offset=0;
        }
        else if offset_on_index> 1 {
            self.offset-=self.segment_length;
            self.max_offset-=self.segment_length;
        }
    }
    // function that will write data in any file that has room, this data is either already split or full, but will never overstock on files capacity
    pub fn get_file_and_offset_of_current_file(&self, offset:u64)->(String, u64){
        let current_offset=offset.clone() % self.segment_length;
        let file_index=make_numer_four_digit((offset.clone() - current_offset) / self.segment_length + 1);
        let filepath=format!("{}/wal_{}.bin",self.storage_path,file_index);
        (filepath,current_offset)
    }


    pub fn get_where_stopped(&mut self)->Option<EntryElement>{
        self.get_by_offset(self.offset)
    }
    pub fn get_by_offset(&mut self, mut offset:u64) ->Option<EntryElement>{
        if offset>=self.max_offset{self.offset=0;offset=0;}
        let mut result :Option<EntryElement>= None;
        let (this_file_path,file_offset)=self.get_file_and_offset_of_current_file(offset);
        let index=extract_index_from_name(this_file_path.as_str()) as u64;
        println!("\nfile: {this_file_path}, offset: {file_offset}");

        let file = File::open(&this_file_path).expect("Failed to open WAL file");
        let mut reader = BufReader::new(file);
        let total_size = reader.get_ref().metadata().unwrap().len();
        // seek to the specified position
        reader.seek(SeekFrom::Start(file_offset)).expect("Failed to seek in file");

        if file_offset == 0 {
            // there is a chance I have previous part of an element
            let mut first_byte =[0u8;1];
            reader.read_exact(&mut first_byte).expect("Failed to read first byte from file");
            if first_byte[0]==1{
                return if index > 1 {
                    println!("THIS IS AREA 1");
                    // read part from before part from now
                    // seek done
                    let previous_file_string = get_name(self.storage_path.clone(), index as i32 - 1);
                    self.take_from_file1_and_file2(previous_file_string, this_file_path,1)
                }
                else {
                    println!("THIS IS AREA 2");
                    // previous wal was deleted still need to seek
                    let mut size = [0u8; 8];
                    reader.read_exact(&mut size).expect("Failed to read size from file");
                    // read non relevant data
                    let mut data = vec![0u8; u64::from_be_bytes(size) as usize];
                    reader.read_exact(&mut data).expect("Failed to read data from file");
                    self.offset = reader.stream_position().unwrap() as u64;
                    None //specific case
                }
            }
            if first_byte[0]==0{
                println!("THIS IS AREA 3");
                // read normally
                // read size
                let mut size=[0u8;8];
                reader.read_exact(&mut size).expect("Failed to read size from file");
                // read data
                let mut data=vec![0u8;u64::from_be_bytes(size) as usize];
                reader.read_exact(&mut data).expect("Failed to read data from file");
                self.offset=((index-1)*self.segment_length)+u64::from_be_bytes(size)+8 +file_offset+1;
                return Some(EntryElement::deserialize(data.as_slice()))
            }
        }
        else{
            // I might reach end and see that I cannot get whole element
            // read size
            let mut one=[0u8;1];
            reader.read_exact(&mut one).expect("Failed to read first byte from file");
            if one[0]=='*'.to_string().as_bytes()[0]{
                return self.get_by_offset(self.segment_length*index+1);
            }
            else{
                reader.seek(SeekFrom::Current(-1)).expect("Failed to seek in file");
                // println!("total:{} left:{} currently on:{}", total_size,total_size- reader.stream_position().unwrap() ,reader.stream_position().unwrap());
                let mut size=[0u8;8];
                reader.read_exact(&mut size).expect("Failed to read size from file");
                // read data
                // println!("here i am: {} size i need:{}",offset,u64::from_be_bytes(size));
                let mut data=vec![0u8;u64::from_be_bytes(size) as usize];
                reader.read_exact(&mut data).expect("Failed to read data from file");
                let mut is_end=[0u8;1];
                let exist=reader.read_exact(&mut is_end);
                if !exist.is_ok() {
                    println!("THIS IS AREA 8");
                    result=Some(EntryElement::deserialize(data.as_slice()))
                }
                if is_end[0]=='*'.to_string().into_bytes()[0]{
                    if index+1<=self.max_segments_in_memory{
                        let next_file_string=get_name(self.storage_path.clone(), index as i32 + 1);
                        let next_file = File::open(&next_file_string).expect("Failed to open WAL file");
                        let mut next_reader = BufReader::new(next_file);
                        let mut first_byte =[0u8;1];
                        next_reader.read_exact(&mut first_byte).expect("Failed to read first byte from file");
                        if first_byte[0]==1{
                            println!("THIS IS AREA 4");
                            // before - after
                            // good seek
                            return self.take_from_file1_and_file2(this_file_path,next_file_string,0)
                        }
                        else {
                            // normal
                            println!("THIS IS AREA 5");
                            result= Some(EntryElement::deserialize(data.as_slice()))
                        }
                    }
                    else{
                        println!("THIS IS AREA 6");
                        self.offset=0;
                        return None;
                    }
                }
                else{
                    // normally
                    println!("THIS IS AREA this 7");
                    result=Some(EntryElement::deserialize(data.as_slice()));
                    self.offset=((index-1)*self.segment_length)+result.clone()?.serialize().len() as u64 + 8 +file_offset;
                    return result

                }
            }
        }
        // println!("I am returning offset:{} =({}-1)*{}+{}+8+{}",((index-1)*self.segment_length)+result.clone()?.serialize().len() as u64 + 8 +file_offset,index,self.segment_length,result.clone()?.serialize().len() as u64 ,file_offset);
        self.offset=((index-1)*self.segment_length)+result.clone()?.serialize().len() as u64 + 8 +file_offset;
        result
    }
    pub fn take_from_file1_and_file2(&mut self, file_before_string:String,file_after_string:String,direction:u64)->Option<EntryElement>{
        // if direction is 1 I will add 1 to offset at the end if its 0 I will add 0
        let file_before = File::open(&file_before_string).expect("Failed to open WAL file");
        let mut reader_before = BufReader::new(file_before);
        let file_after = File::open(&file_after_string).expect("Failed to open WAL file");
        let mut reader_after = BufReader::new(file_after);

        reader_after.seek(SeekFrom::Start(1)).expect("Failed to seek in file");
        let mut second_part_size=[0u8;8];
        reader_after.read_exact(&mut second_part_size).expect("Failed to read first byte from file");
        let mut second_part_data=vec![0u8;u64::from_be_bytes(second_part_size) as usize];
        reader_after.read_exact(&mut second_part_data).expect("Failed to read second byte from file");
        let data_size=u64::from_be_bytes(second_part_data[0..8].try_into().expect("slice with incorrect length"));
        let to_seek_back=(data_size-(u64::from_be_bytes(second_part_size)-8)) as i64;
        let second_data=second_part_data[8..].to_vec();
        self.offset = (self.segment_length*(extract_index_from_name(file_after_string.as_str()) as u64 -1)) + reader_after.stream_position().unwrap()+direction;

        reader_before.seek(SeekFrom::End(-to_seek_back-1)).expect("Failed to seek in file");
        let mut first_part=vec![];
        reader_before.read_to_end(&mut first_part).expect("Failed to read first byte from file");
        let mut data=first_part[..first_part.len()-1].to_vec();
        data.extend(second_data);

        Some(EntryElement::deserialize(data.as_slice()))
    }


    pub fn will_need_to_delete_oldest(&self,data_size:u64)->bool{
        let total_offset = self.max_offset + data_size;
        total_offset / self.segment_length >= self.max_segments_in_memory
    }
    pub fn get_all_records_from_oldest_segment(&mut self)->Vec<EntryElement>{
        let mut result:Vec<EntryElement>=vec![];
        self.offset=0; //need to start from start because I will retire only segment 0001
        while self.offset/self.segment_length<1{
            let e= self.get_where_stopped();
            if e.is_none(){continue;} // only possible if the first read data is part of record that belongs to previous segment
            result.push(e.unwrap());
        }
        result
    }
}


pub fn get_name(storage_path: String, index: i32) -> String {
    format!("{}/wal_{}.bin", storage_path, make_numer_four_digit(index as u64))
}
pub fn extract_index_from_name(file_name: &str) -> i32 {
    let parts: Vec<&str> = file_name.split('/').collect(); // Split by '/'

    // Take the last part (the file name itself), and remove "wal_" prefix and ".bin" suffix
    if let Some(file_part) = parts.last() {
        if file_part.starts_with("wal_") && file_part.ends_with(".bin") {
            let index_str = &file_part[4..file_part.len() - 4]; // "wal_" is 4 chars, ".bin" is 4 chars
            if let Ok(index) = index_str.parse::<i32>() {
                return index; // Return the parsed index
            }
        }
    }

    // If parsing fails, return a default value or handle the error
    -1
}

pub fn make_numer_four_digit(number: u64) -> String {
    format!("{:04}", number)
}
