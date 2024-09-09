use std::fs;
use serde_json::{json, Value};
use clap::Parser;
use crate::CONFIGURATION_PATH;

// paths
pub const  STORAGE_PATH:&str="startup/startup_app/src/storage";
pub const SSTABLE:&str="/sstables";
pub const WAL:&str="/wal";

// Bloom Filter constants
const BLOOM_FILTER_FALSE_POSITIVE_RATE: f64 = 0.1;
const BLOOM_FILTER_EXPECTED_ELEMENTS: u64 = 1000;

// Cache constants
const CACHE_CAPACITY: u64 = 5;

// Skip List
const ODDS_OF_FLIPS:i32=50;
const MIN_BOUND:u64=1;
const MAX_BOUND:u64=1000;

// Memtable constants
const MEMTABLE_MAX_SIZE: u64 = 20;
const MEMTABLE_NUMBER_OF_HASH_MEMTABLES: u64 = 40;
const MEMTABLE_NUMBER_OF_SKIP_LIST_MEMTABLES: u64 = 40;
const MEMTABLE_NUMBER_OF_B_TREE_MEMTABLES: u64 = 40;
const MEMTABLE_READ_WRITE: MemtableType = MemtableType::Hash;

// SSTable constants
const SSTABLE_VOLUME_INDEX: u64 = 7;
const SSTABLE_VOLUME_SUMMARY: u64 = 7;

// Token Bucket constants
const TOKEN_BUCKET_TOKENS: u64 = 5;
const TOKEN_BUCKET_REFILL_RATE: u64 = 4;

// Write-Ahead Log (WAL) constants
const WAL_SEGMENT_LENGTH: u64 = 300;
const WAL_MAX_SEGMENTS_IN_MEMORY: u64 = 5;
pub(crate) enum MemtableType {
    Hash,
    SkipList,
    BTree,
}
pub fn take_from_json(object: &str, key: &str)-> Option<Value>{
    let data = fs::read_to_string(CONFIGURATION_PATH.to_string()).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    json.get(object).and_then(|obj| obj.get(key)).cloned()
}

pub fn get_bloom_filter_false_positive_rate() -> f64 {
    let json= take_from_json("bloom_filter","false_positive_rate");
    if json.is_none() {return BLOOM_FILTER_FALSE_POSITIVE_RATE}
    json.unwrap().to_string().parse::<f64>().unwrap()
}
pub fn get_bloom_filter_expected_elements() -> u64 {
    let json= take_from_json("bloom_filter","expected_elements");
    if json.is_none() {return BLOOM_FILTER_EXPECTED_ELEMENTS}
    json.unwrap().to_string().parse::<u64>().unwrap()
}

pub fn get_cache_capacity() -> u64 {
    let json= take_from_json("cache","capacity");
    if json.is_none() {return CACHE_CAPACITY}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_skip_list_odds_of_flips() -> i32 {
    let json=take_from_json("skip_list","odds_of_flips");
    if json.is_none() {return ODDS_OF_FLIPS}
    json.unwrap().to_string().parse::<i32>().unwrap()
}
pub fn get_skip_list_min_bounds()->u64{
    let json=take_from_json("skip_list","min_bounds");
    if json.is_none() {return MIN_BOUND}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_skip_list_max_bounds()->u64{
    let json=take_from_json("skip_list","max_bounds");
    if json.is_none(){return MAX_BOUND}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_memtable_max_size() -> u64 {
    let json= take_from_json("memtable","max_size");
    if json.is_none() {return MEMTABLE_MAX_SIZE}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_memtable_number_of_hash_memtables() -> u64 {
    let json= take_from_json("memtable","number_of_hash_memtables");
    if json.is_none(){return MEMTABLE_NUMBER_OF_HASH_MEMTABLES}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_memtable_number_of_skip_list_memtables() -> u64 {
    let json= take_from_json("memtable","number_of_skip_list_memtables");
    if json.is_none(){return MEMTABLE_NUMBER_OF_SKIP_LIST_MEMTABLES}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_number_of_b_tree_memtables() -> u64 {
    let json= take_from_json("memtable","number_of_b_tree_memtables");
    if json.is_none(){return MEMTABLE_NUMBER_OF_B_TREE_MEMTABLES}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_memtable_read_write() -> MemtableType {
    let json=take_from_json("memtable","read_write");
    if json.is_none() {return MEMTABLE_READ_WRITE}
    match json.unwrap().to_string().as_str() {
        "Hash" => MemtableType::Hash,
        "SkipList" => MemtableType::SkipList,
        "BTree" => MemtableType::BTree,
        _ => {MemtableType::Hash}
    }
}
pub fn get_sstable_volume_index() -> u64 {
    let json= take_from_json("sstable","volume_index");
    if json.is_none() {return SSTABLE_VOLUME_INDEX}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_sstable_volume_summary() -> u64 {
    let json= take_from_json("sstable","volume_summary");
    if json.is_none(){return SSTABLE_VOLUME_SUMMARY}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_token_bucket_tokens() -> u64 {
    let json= take_from_json("token_bucket","tokens");
    if json.is_none() {return TOKEN_BUCKET_TOKENS}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_token_bucket_refill_rate() -> u64 {
    let json= take_from_json("token_bucket","refill_rate");
    if json.is_none(){return TOKEN_BUCKET_REFILL_RATE}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_wal_segment_length() -> u64 {
    let json= take_from_json("wal","segment_length");
    if json.is_none() {return WAL_SEGMENT_LENGTH}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_wal_max_segment_size() -> u64 {
    let json= take_from_json("wal","max_segments_in_memory");
    if json.is_none(){return WAL_MAX_SEGMENTS_IN_MEMORY}
    json.unwrap().to_string().parse::<u64>().unwrap()
}
pub fn get_wal_max_offset()->u64{
    let data = fs::read_to_string(format!("{STORAGE_PATH}/metadata.json")).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let max_offset_wal_str = json["max_offset_wal"].as_str().expect("Value should be a string");
    let max_offset_wal = max_offset_wal_str.parse::<u64>().expect("Value should be a valid u64");
    max_offset_wal
}