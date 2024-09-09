use crate::external_configuration::default_constants::{get_bloom_filter_expected_elements, get_bloom_filter_false_positive_rate, get_memtable_max_size, get_memtable_number_of_hash_memtables, get_memtable_number_of_skip_list_memtables, get_memtable_read_write, get_number_of_b_tree_memtables, get_skip_list_max_bounds, get_skip_list_min_bounds, get_sstable_volume_index, get_sstable_volume_summary, MemtableType};
use crate::representation_of_program_as_structure::Program;

impl Program{
    pub fn get(&mut self, key:String) -> Vec<u8> {
        if self.token_bucket.request(){ // 1. look in memtables
            let result_memtable = self.get_from_memtable(key.clone());
            if result_memtable != [0u8; 0] { println!("got from memtable");return result_memtable; }

            // 2. cache
            let result_cache = self.cache.get(&key.clone());
            if result_cache.is_some() && result_cache.clone().unwrap().tombstone == false {println!("got from cache"); return result_cache.clone().unwrap().value; }

            // 3. sstables
            for i in 0..self.sstables.len() {
                let mut sstable = self.sstables[i].clone();
                let result_sstable = sstable.search(key.clone());
                if result_sstable.tombstone==false { println!("got from sstable");return result_sstable.value; }
            }
            vec![0u8; 0]
        }
        else{
            println!("REQUEST DENIED BY OVERLOAD OF TOKEN BUCKET.WAIT FEW SECONDS");
            vec![0u8; 0]
        }
    }
    pub fn get_from_memtable(&mut self,key: String)->Vec<u8> {
    match get_memtable_read_write() {
        MemtableType::Hash => {
            for i in 0..self.memtables_hash_map.len() {
                let table = self.memtables_hash_map[i].clone();
                let result = table.get_value(key.clone());
                if result != [0u8; 0] { return result; }
            }
            vec![0u8; 0]
        },
        MemtableType::SkipList => {
            for i in 0..self.memtables_skip_list.len() {
                let mut table = self.memtables_skip_list[i].clone();
                let result = table.get_value(key.clone());
                if result != [0u8; 0] { return result; }
            }
            vec![0u8; 0]
        },
        MemtableType::BTree => {
            for i in 0..self.memtables_b_tree.len() {
                let mut table = self.memtables_b_tree[i].clone();
                let result = table.get_value(key.clone());
                if result != [0u8; 0] { return result; }
            }
            vec![0u8; 0]
        },
    }
}
}