// When a user sends a PUT or DELETE request, it is first recorded in the Write-Ahead Log (Commit Log) WAL.
// ● Once the WAL confirms the entry, the data needs to be added to the Memtable, which is strictly stored in memory.
// ● When the predefined size of the Memtable is reached, the values are sorted by key, and a new SSTable is created and written to disk.
// ● After that, we check if the conditions for starting a compaction are met, and initiate them if they are. It is important to note that compactions at one level can trigger compactions at the next level.

use entry_element::entry_element::EntryElement;
use memtable::memtable_btree::MemtableBTree;
use memtable::memtable_hash_map::MemtableHashMap;
use memtable::memtable_skip_list::MemtableSkipList;
use sstable::sstable::SSTable;
use crate::representation_of_program_as_structure::Program;
use token_bucket::token_bucket::{get_now_in_seconds};
use crate::external_configuration::default_constants::{get_bloom_filter_expected_elements, get_bloom_filter_false_positive_rate, get_memtable_max_size, get_memtable_number_of_hash_memtables, get_memtable_number_of_skip_list_memtables, get_memtable_read_write, get_number_of_b_tree_memtables, get_skip_list_max_bounds, get_skip_list_min_bounds, get_sstable_volume_index, get_sstable_volume_summary, MemtableType, SSTABLE, STORAGE_PATH};

pub fn put(mut this: &mut Program, key:String, value:String) {
    let element=EntryElement::new(key.clone(),value.as_bytes().to_vec(),get_now_in_seconds());
    // 1. put element in wal if no errors occurred continue
    this.wal.add_record(element.clone());

    // 2. put element in memtable that is specifically read write
        match get_memtable_read_write(){
            MemtableType::Hash=>{
                if this.memtables_hash_map.is_empty()||this.memtables_hash_map.last().unwrap().read_only{
                    // time for new one because old was killed
                    if this.memtables_hash_map.len()+1> get_memtable_number_of_hash_memtables() as usize{
                        // flush all n memtables in sstables !!!
                        for mut ind in 0..this.memtables_hash_map.len(){
                            let mut i =this.memtables_hash_map[ind].clone();
                            let data=i.flush();
                            let mut sstable_i=SSTable::new(format!("{STORAGE_PATH}/{SSTABLE}"), get_sstable_volume_index() as i32, get_sstable_volume_summary() as i32, get_bloom_filter_expected_elements() as i64, get_bloom_filter_false_positive_rate());
                            sstable_i.activate_flush(data);
                            this.sstables.push(sstable_i);
                        }
                        this.memtables_hash_map.clear();
                    }
                    let mut new_memtable = MemtableHashMap::new(get_memtable_max_size() as u32, false);
                    new_memtable.add_element(element.clone());
                    this.memtables_hash_map.push(new_memtable);
                }
                else{
                    let mut ind= this.memtables_hash_map.len()-1;
                    let mut last=this.memtables_hash_map[ind].clone();
                    last.add_element(element.clone());
                    this.memtables_hash_map[ind]=last.clone();
                }
                let mut representative=this.memtables_hash_map.last().unwrap().clone();
                if representative.get_value(key.clone())==[0u8;0]{println!("ERROR ADDING TO MEMTABLE");return;}
            },
            MemtableType::SkipList=>{
                if this.memtables_skip_list.is_empty()||this.memtables_skip_list.last().unwrap().read_only{
                    // time for new one because old was killed
                    if this.memtables_skip_list.len()+1> get_memtable_number_of_skip_list_memtables() as usize{
                        // flush all n memtables in sstables !!!
                        for mut ind in 0..this.memtables_skip_list.len(){
                            let mut i =this.memtables_skip_list[ind].clone();
                            let data=i.flush();
                            let mut sstable_i=SSTable::new(format!("{STORAGE_PATH}/{SSTABLE}"), get_sstable_volume_index() as i32, get_sstable_volume_summary() as i32, get_bloom_filter_expected_elements() as i64, get_bloom_filter_false_positive_rate());
                            sstable_i.activate_flush(data);
                            this.sstables.push(sstable_i);
                        }
                        this.memtables_skip_list.clear();
                    }
                    let mut new_memtable=MemtableSkipList::new(get_memtable_max_size() as u32, false,EntryElement{key:format!("key{}",get_skip_list_min_bounds().to_string()),value:"non existent".as_bytes().to_vec(),tombstone:true,timestamp:0},EntryElement{key:format!("key{}",get_skip_list_max_bounds().to_string()),value:"non existent".as_bytes().to_vec(),tombstone:true,timestamp:0});
                    new_memtable.add_element(element.clone());
                    this.memtables_skip_list.push(new_memtable);
                }
                else{
                    let mut ind= this.memtables_skip_list.len()-1;
                    let mut last=this.memtables_skip_list[ind].clone();
                    last.add_element(element.clone());
                    this.memtables_skip_list[ind]=last.clone();
                }
                let mut representative=this.memtables_skip_list.last().unwrap().clone();
                if representative.get_value(key.clone())==[0u8;0]{println!("ERROR ADDING TO MEMTABLE");return;}
            },
            MemtableType::BTree=>{
                if this.memtables_b_tree.is_empty()||this.memtables_b_tree.last().unwrap().read_only{
                    // time for new one because old was killed
                    if this.memtables_b_tree.len()+1> get_number_of_b_tree_memtables() as usize{
                        // flush all n memtables in sstables !!!
                        for mut ind in 0..this.memtables_b_tree.len(){
                            let mut i =this.memtables_b_tree[ind].clone();
                            let data=i.flush();
                            let mut sstable_i=SSTable::new(format!("{STORAGE_PATH}/{SSTABLE}"), get_sstable_volume_index() as i32, get_sstable_volume_summary() as i32, get_bloom_filter_expected_elements() as i64, get_bloom_filter_false_positive_rate());
                            sstable_i.activate_flush(data);
                            this.sstables.push(sstable_i);
                        }
                        this.memtables_b_tree.clear();
                    }
                    let mut new_memtable=MemtableBTree::new(get_memtable_max_size() as u32, false);
                    new_memtable.add_element(element.clone());
                    this.memtables_b_tree.push(new_memtable);
                }
                else{
                    let mut ind= this.memtables_b_tree.len()-1;
                    let mut last=this.memtables_b_tree[ind].clone();
                    last.add_element(element.clone());
                    this.memtables_b_tree[ind]=last.clone();
                }
                let mut representative=this.memtables_b_tree.last().unwrap().clone();
                if representative.get_value(key.clone())==[0u8;0]{println!("ERROR ADDING TO MEMTABLE");return;}

            },
        }


}