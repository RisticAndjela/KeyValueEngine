// When a user sends a PUT or DELETE request, it is first recorded in the Write-Ahead Log (Commit Log) WAL.
// ● Once the WAL confirms the entry, the data needs to be added to the Memtable, which is strictly stored in memory.
// ● When the predefined size of the Memtable is reached, the values are sorted by key, and a new SSTable is created and written to disk.
// ● After that, we check if the conditions for starting a compaction are met, and initiate them if they are. It is important to note that compactions at one level can trigger compactions at the next level.

use std::fs;
use std::path::Path;
use crate::external_configuration::default_constants::{get_bloom_filter_expected_elements, get_bloom_filter_false_positive_rate, get_memtable_max_size, get_memtable_number_of_hash_memtables, get_memtable_number_of_skip_list_memtables, get_memtable_read_write, get_number_of_b_tree_memtables, get_skip_list_max_bounds, get_skip_list_min_bounds, get_sstable_volume_index, get_sstable_volume_summary, MemtableType};
use crate::representation_of_program_as_structure::Program;
impl Program{
    pub fn delete(mut self: &mut Program, key:String) {
        // 1. talk to token bucket through get, not necessary calling it twice
        if self.get(key.clone()) == vec![0u8;0]{return;}
        // 3. delete element in any memtable
        self.delete_from_memtable(key.clone());
        // 4. delete in cache
        self.cache.delete(key.clone());
        // 5. sstable delete
        for i in 0..self.sstables.len(){
            let sstable = self.sstables[i].clone();
            let mut changed_sstable =sstable.delete(key.clone());
    
            fs::remove_dir_all(&sstable.dir_path).expect("smor");
            fs::rename(&changed_sstable.dir_path, &sstable.dir_path).expect("jbt vise");
    
            self.sstables[i]=changed_sstable.clone();
        }
    
    }
    pub fn delete_from_memtable(mut self:&mut Program,key: String) {
    match get_memtable_read_write() {
        MemtableType::Hash => {
            for i in 0..self.memtables_hash_map.len() {
                let mut table = self.memtables_hash_map[i].clone();
                table.delete(key.clone());
                self.memtables_hash_map[i] = table;
            }
        },
        MemtableType::SkipList => {
            for i in 0..self.memtables_skip_list.len() {
                let mut table = self.memtables_skip_list[i].clone();
                table.delete(key.clone());
                self.memtables_skip_list[i] = table;
            }
        },
        MemtableType::BTree => {
            for i in 0..self.memtables_b_tree.len() {
                let mut table = self.memtables_b_tree[i].clone();
                table.delete(key.clone());
                self.memtables_b_tree[i] = table;            }
        },
    }
}
}