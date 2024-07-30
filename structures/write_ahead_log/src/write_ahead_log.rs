use std::fs::{File};
use std::fs;
use std::ops::Add;
use memtable_element::entry_element::{EntryElement};
use std::io::{ Seek, SeekFrom, Write};

pub struct WriteAheadLog {
    pub filename: String, // latest used wal file
    pub segment_length: u64, // we can predefine this but all segments will be same length
    pub current_size: u64, // we need to know how many elements or parts of the elements? can fit more
    pub current_offset:u64, // we need to be able to read elements one by one
}

impl WriteAheadLog {
    pub fn new( segment_length:u64)->Self{
        let wal=WriteAheadLog{filename:get_name(1),segment_length,current_size:0,current_offset:0};
        let _file = File::create(&wal.filename);
        wal
    }
    pub fn get_current_index(&self,filename:String)->i32{
        filename.as_str().split('_').last().and_then(|s| s.split('.').next()).and_then(|s| s.parse::<i32>().ok()).unwrap()
    }
    pub fn add_new_file(&mut self){
        // filename is wal_0001 or like wal_0213 probably not that long but needs to be taken in consideration
        let mut index =self.get_current_index(self.filename.clone());
        index+=1;
        let new_filename=get_name(index);
        self.filename=new_filename;
        self.current_size=0;
        self.current_offset=0;
        File::create(&self.filename).expect("cannot create");
    }
    pub fn add_element(&mut self, element: &EntryElement){
        let binding = self.get_all_files();
        let last_file=binding.last().unwrap();
        self.filename=last_file.clone();
        self.current_size=fs::metadata(last_file.clone()).unwrap().len();
        let left_space=&self.segment_length-&self.current_size;
        if element.size()>left_space{
            self.add_new_file();
        }
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(&self.filename).unwrap();

        file.seek(SeekFrom::Start(self.current_size)).expect("cannot go to till the end");
        file.write_all(element.serialize().as_slice()).expect("cannot write");

    }
    pub fn remove_file(&mut self, index: i32){
        self.current_offset=0;
        let mut files=self.get_all_files();
        if files.len() < index as usize {panic!("out of collection of files")}
        //remove those who don't need changes
         for _ in 0..index{
             files.remove(0);
         }
        let removed_file=get_name(index);
        fs::remove_file(&removed_file).expect("cannot remove");

        // Rename subsequent files
        let mut current_index = index+1;

        for file in files {
            current_index -= 1;
            let one_before_file = get_name(current_index);
            fs::rename(&file, &one_before_file).expect("TODO: panic message");
            current_index+=2;
        }

        let mut self_index = self.get_current_index(self.filename.clone());
        self_index -= 1;
        self.filename = get_name(self_index);

    }
    pub fn get_all_files(&mut self) -> Vec<String> {
        let mut all = vec![];
        let current_index = self.get_current_index(self.filename.clone());
        if current_index == 1 {
            all.push(get_name(current_index));
            return all;
        }

        for entry in fs::read_dir("src/storage").unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let ind = self.get_current_index(path.file_name().unwrap().to_string_lossy().to_string());
                    all.push(get_name(ind));
                }
            }
        }
        all
    }


}

pub fn get_name(index:i32) -> String {
    let mut new_filename=String::new().add("src/storage/wal_");
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
