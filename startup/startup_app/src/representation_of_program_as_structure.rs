use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use cache::cache::Cache;
use sstable::sstable::SSTable;
use memtable::{memtable_btree::MemtableBTree,memtable_hash_map::MemtableHashMap,memtable_skip_list::MemtableSkipList};
use token_bucket::token_bucket::TokenBucket;
use write_ahead_log::write_ahead_log::WriteAheadLog;
use crate::external_configuration::default_constants::{STORAGE_PATH, SSTABLE, WAL, get_cache_capacity, get_wal_segment_length, get_wal_max_segment_size, get_sstable_volume_index, get_sstable_volume_summary, get_token_bucket_tokens, get_token_bucket_refill_rate, get_memtable_read_write, MemtableType, get_memtable_number_of_hash_memtables, get_memtable_number_of_skip_list_memtables, get_number_of_b_tree_memtables, get_memtable_max_size};
pub struct Program{
    pub path_to_whole_storage:String,

    pub cache: Cache,
    pub token_bucket: TokenBucket,

    pub sstables:Vec<SSTable>,
    pub wal:WriteAheadLog,

    pub memtables_hash_map:Vec<MemtableHashMap>,
    pub memtables_skip_list:Vec<MemtableSkipList>,
    pub memtables_b_tree:Vec<MemtableBTree>,

}
impl Program{
    pub fn new() -> Program{
        Program{path_to_whole_storage:STORAGE_PATH.to_string(),cache:Cache::new(get_cache_capacity()),token_bucket:TokenBucket::new(get_token_bucket_tokens(),get_token_bucket_refill_rate()),
            sstables:vec![],wal:WriteAheadLog::new(format!("{STORAGE_PATH}/{WAL}"),get_wal_segment_length(),get_wal_max_segment_size()),
            memtables_hash_map:vec![],memtables_skip_list:vec![],memtables_b_tree:vec![]
        }
    }
    pub fn open_recent()->Program{
        let p=Program{path_to_whole_storage:STORAGE_PATH.to_string(),cache:Cache::new(get_cache_capacity()),token_bucket:get_token_bucket(),
            sstables:get_sstables(format!("{STORAGE_PATH}/{SSTABLE}")),wal:WriteAheadLog::open(format!("{STORAGE_PATH}/{WAL}"),get_wal_segment_length(),get_wal_max_segment_size()),
            memtables_hash_map:vec![],memtables_skip_list:vec![],memtables_b_tree:vec![]};
        p.load_memtables_from_wal();
        p
    }
    pub fn stop(&self){}
    pub fn provide_with_status(&mut self){
        let mut current_num_of_memtables=0;
        let mut num_of_memtables=0;
        let mut currently_filled=0;
        match get_memtable_read_write(){
            MemtableType::Hash=>{
                current_num_of_memtables=self.memtables_hash_map.len();
                num_of_memtables=get_memtable_number_of_hash_memtables();
                if self.memtables_hash_map.last().is_some(){currently_filled=self.memtables_hash_map.last().unwrap().current_count; }
            }
            MemtableType::SkipList=>{
                current_num_of_memtables=self.memtables_skip_list.len();
                num_of_memtables=get_memtable_number_of_skip_list_memtables();
                if self.memtables_skip_list.last().is_some(){currently_filled=self.memtables_skip_list.last().unwrap().current_count; }
            }
            MemtableType::BTree=>{
                current_num_of_memtables=self.memtables_b_tree.len();
                num_of_memtables=get_number_of_b_tree_memtables();
                if self.memtables_b_tree.last().is_some(){currently_filled=self.memtables_b_tree.last().unwrap().current_count; }
            }
        }
        println!("CURRENT STATUS: \nCache:{}/{} \nToken bucket:{}/{} \nSSTables:{} \nWAL:{}/{} \nMemtables:{}/{} read-write status:{}/{}",
            self.cache.elements.len(),self.cache.capacity,
            self.token_bucket.tokens,self.token_bucket.capacity,
            self.sstables.len(),
            self.wal.get_all_files().len(),self.wal.max_segments_in_memory,
            current_num_of_memtables,num_of_memtables,currently_filled,get_memtable_max_size()
        )
    }
    fn load_memtables_from_wal(&self) {
        todo!()
    }
}

pub fn get_sstables(dir_path:String)->Vec<SSTable>{
    let mut sstables=vec![];
    for i in list_subdirectories(&dir_path){
        sstables.push(SSTable::open(i, get_sstable_volume_index() as i32, get_sstable_volume_summary() as i32));
    }
    sstables
}

pub fn get_token_bucket() -> TokenBucket{
    let filepath=format!("{}/token_bucket.bin",STORAGE_PATH.to_string());
    let file = File::open(&filepath).expect("Failed to open WAL file");
    let mut reader = BufReader::new(file);
    if reader.get_ref().metadata().unwrap().len()<32{return TokenBucket::new(get_token_bucket_tokens(),get_token_bucket_refill_rate());}
    let mut data=vec![];
    reader.read_to_end(&mut data).expect("Failed to read token bucket file");
    TokenBucket::deserialize(&data)
}

fn list_subdirectories(base_dir: &str) -> Vec<String> {
    let mut directories = Vec::new();
    let base_path = Path::new(base_dir);
    if base_path.is_dir() {
        let mut stack = vec![base_path.to_path_buf()];
        while let Some(path) = stack.pop() {
            if path.is_dir() {
                for entry in fs::read_dir(&path).expect("Error reading directory") {
                    let entry = entry.expect("Error reading directory entry");
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        directories.push(entry_path.clone().as_path().to_str().unwrap().to_string());
                    }
                }
            }
        }
    } else {
        println!("The provided path is not a directory");
    }

    directories
}