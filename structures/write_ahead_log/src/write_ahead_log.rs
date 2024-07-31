use std::fs::{File};
use std::{fs, io};
use std::ops::Add;
use memtable_element::entry_element::{EntryElement};
use memtable_element::constants::{CONST_LEN_OF_ENTRY, KEY_SIZE_START, VALUE_SIZE_START};
use std::io::{Read, Seek, SeekFrom, Write};

pub struct WriteAheadLog {
    pub storage_path: String, // path to all wal files
    pub segment_length: u64, // we can predefine this but all segments will be same length
    pub offset:u64, // we need to be able to read elements one by one and start from the one we stopped at, this offset is global meaning                   all files as well will be included not just from the last one
}

impl WriteAheadLog {
    pub fn new(directory_path:String, segment_length:u64)->Self{
        let wal=WriteAheadLog{storage_path:directory_path,segment_length,offset:0};
        let _file = File::create(get_name(wal.storage_path.clone(),1));
        wal
    }
    fn get_file_and_offset_for_file(&self) -> (String, u64) {
        let file_index = (( self.offset / self.segment_length)+1) as usize;
        let offset_within_file = self.offset % self.segment_length;
        (get_name(self.storage_path.clone(),file_index as i32), offset_within_file)
    }
    pub fn get_file_current_size(&self,filename:String)->u64{
        let metadata = std::fs::metadata(filename);
        metadata.unwrap().len()
    }
    pub fn get_current_index(&self,filename:String)->i32{
        filename.as_str().split('_').last().and_then(|s| s.split('.').next()).and_then(|s| s.parse::<i32>().ok()).unwrap()
    }
    pub fn add_new_file(&mut self){
        // filename is wal_0001 or like wal_0213 probably not that long but needs to be taken in consideration
        let binding = self.get_all_files().clone();
        let last_file=binding.last().unwrap();
        let mut index =self.get_current_index(last_file.clone());
        index+=1;
        let new_filename=get_name(self.storage_path.clone(),index);
        File::create(&new_filename).expect("cannot create :(");

        self.offset= (self.get_all_files().len() - 1) as u64 * self.segment_length;
    }
    pub fn add_element(&mut self, element: &EntryElement){
        let files = self.get_all_files();
        let mut last_file=files.last().unwrap().clone();
        let mut current_size =self.get_file_current_size(last_file.clone());
        let left_space=&self.segment_length-&current_size;
        if element.size()>left_space{
            self.add_new_file();
            let binding2 = self.get_all_files();
            last_file= binding2.last().unwrap().clone();
            current_size=self.get_file_current_size(last_file.clone());
        }
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(&last_file).unwrap();

        let last_index=self.get_current_index(last_file) as u64;
        let offset_to_write=(last_index - 1 )* self.segment_length + current_size;
        self.offset=offset_to_write;

        file.seek(SeekFrom::Start(self.get_file_and_offset_for_file().1)).expect("cannot seek :(");
        file.write_all(element.serialize().as_slice()).expect("cannot write :(");

        self.offset+=  element.size();
    }
    pub fn remove_file(&mut self, index: i32){
        let mut files=self.get_all_files();
        if files.len() < index as usize {panic!("out of collection of files :(")}
        //remove those who don't need changes
         for _ in 0..index{
             files.remove(0);
         }
        //remove actual file
        let removed_file=get_name(self.storage_path.clone(),index);
        fs::remove_file(&removed_file).expect("cannot remove :(");

        // Rename left files
        let mut working_on_index = index+1;
        for file in files {
            working_on_index -= 1;
            let one_before_file = get_name(self.storage_path.clone(),working_on_index);
            fs::rename(&file, &one_before_file).expect("cannot rename :(");
            working_on_index +=2;
        }
        let offset_on_index=self.get_current_index(self.get_file_and_offset_for_file().0);
        if offset_on_index==index{
            self.offset=0;
        }
        else if offset_on_index>index{
            self.offset-=self.segment_length;
        }
    }
    pub fn get_all_files(&mut self) -> Vec<String> {
        let mut all = vec![];

        for entry in fs::read_dir("src/storage").unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let ind = self.get_current_index(path.file_name().unwrap().to_string_lossy().to_string());
                    all.push(get_name(self.storage_path.clone(),ind));
                }
            }
        }
        all
    }
    pub fn read_element_at(&mut self,filename: &str, position: u64) -> io::Result<EntryElement> {
        let mut file = File::open(filename)?;
        file.seek(SeekFrom::Start(position))?;
        let mut constant_part = vec![0u8; CONST_LEN_OF_ENTRY];
        file.read_exact(&mut constant_part)?;
        let key_size = u64::from_be_bytes(constant_part[KEY_SIZE_START..VALUE_SIZE_START].try_into().unwrap());
        let value_size = u64::from_be_bytes(constant_part[VALUE_SIZE_START..CONST_LEN_OF_ENTRY].try_into().unwrap());
        let mut element = vec![0u8; CONST_LEN_OF_ENTRY + key_size as usize + value_size as usize];
        file.seek(SeekFrom::Start(position))?;
        file.read_exact(&mut element)?;
        let entry=EntryElement::deserialize(&element);
        self.offset+=entry.size();
        Ok(entry)
    }
    pub fn read_where_stopped(&mut self)-> io::Result<EntryElement> {
        let binding = self.get_file_and_offset_for_file();
        let file=binding.0.as_str();
        let offset_file=binding.1;
        let entry=self.read_element_at(file,offset_file);
        if entry.is_err(){
            let wrong_index=self.get_current_index(file.to_string());//first I will try to go in next file
            self.offset=wrong_index as u64*self.segment_length ;//beginning of new file

            let last=self.get_all_files().last().unwrap().to_string(); //I need to check if they are actually more files
            if self.get_current_index(last)+1==wrong_index{
                 self.offset=0;//should change to error of some kind but for now start from beginning
                return self.read_where_stopped();
            }

            return self.read_where_stopped();
        }
        entry
    }
}

pub fn get_name(storage_path:String, index:i32) -> String {
    let mut new_filename=String::new().add(storage_path.as_str());
    new_filename.push_str("/wal_");
    match index.to_string().len() {
        4 => new_filename.push_str(&index.to_string()),
        3 => { new_filename.push_str("0");
            new_filename.push_str(&index.to_string()); },
        2 => { new_filename.push_str("00");
            new_filename.push_str(&index.to_string()); },
        1 => { new_filename.push_str("000");
            new_filename.push_str(&index.to_string()); },
        _ => {panic!("no more space")}
    }
    new_filename.push_str(".txt");
    return new_filename;
}
