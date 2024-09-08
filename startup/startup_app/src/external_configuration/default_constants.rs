// paths
const  STORAGE_PATH:&str="startup/startup_app/src/storage";
const SSTABLE:&str="/sstable";
const WAL:&str="/wal";

// Bloom Filter constants
pub const BLOOM_FILTER_FALSE_POSITIVE_RATE: f64 = 0.1;
pub const BLOOM_FILTER_EXPECTED_ELEMENTS: u64 = 1000;

// Cache constants
pub const CACHE_CAPACITY: u64 = 5;

// Skip List
pub const ODDS_OF_FLIPS:i32=50;

// Memtable constants
pub const MEMTABLE_MAX_SIZE: u64 = 20;
pub const MEMTABLE_NUMBER_OF_HASH_MEMTABLES: u64 = 40;
pub const MEMTABLE_NUMBER_OF_SKIP_LIST_MEMTABLES: u64 = 40;
pub const MEMTABLE_NUMBER_OF_B_TREE_MEMTABLES: u64 = 40;
pub const MEMTABLE_READ_WRITE: &str = "hash";

// SSTable constants
pub const SSTABLE_VOLUME_INDEX: u64 = 7;
pub const SSTABLE_VOLUME_SUMMARY: u64 = 7;

// Token Bucket constants
pub const TOKEN_BUCKET_TOKENS: u64 = 5;
pub const TOKEN_BUCKET_REFILL_RATE: u64 = 4;

// Write-Ahead Log (WAL) constants
pub const WAL_SEGMENT_LENGTH: u64 = 300;
pub const WAL_MAX_SEGMENTS_IN_MEMORY: u64 = 5;