use std::string::ToString;

mod index;
mod summary;
mod data;
mod sstable;
pub mod global_sstable_functions;

use entry_element::entry_element::{EntryElement as record, EntryElement};
const  SSTABLE_PATH:&str="src/storage";
#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::path::Path;
    use std::fs;
    use crate::sstable::SSTable;
    use entry_element::entry_element::EntryElement as record;
    use crate::{get_memtable_flushed, SSTABLE_PATH};

    #[test]
    #[serial]
    fn test_sstable_new() {
        fs::remove_dir_all(SSTABLE_PATH).unwrap();
        let sstable = SSTable::new(SSTABLE_PATH.to_string(), 5, 2, 1000, 0.01);

        assert_eq!(sstable.data.file_path, "src/storage/sstable_0001/data.bin");
        assert_eq!(sstable.index.file_path, "src/storage/sstable_0001/index.bin");
        assert_eq!(sstable.summary.file_path, "src/storage/sstable_0001/summary.bin");
        assert_eq!(sstable.index.volume, 5);
        assert_eq!(sstable.summary.volume, 2);
    }

    #[test]
    #[serial]
    fn test_sstable_activate_flush() {
        fs::remove_dir_all(SSTABLE_PATH).unwrap();
        let mut sstable = SSTable::new(SSTABLE_PATH.to_string(), 5, 3, 1000, 0.01);

        let records = get_memtable_flushed();

        sstable.activate_flush(records);
        let data_file_path = format!("{}/sstable_0001", SSTABLE_PATH);
        assert!(Path::new(&data_file_path).exists());
        let searched1=sstable.search("key12".to_string());
        assert_ne!(searched1,record::empty());
        let searched2=sstable.search("key112212".to_string());
        assert_eq!(searched2,record::empty());

    }
    #[test]
    #[serial]
    fn test_sstable_delete(){
        fs::remove_dir_all(SSTABLE_PATH).unwrap();
        let mut sstable = SSTable::new(SSTABLE_PATH.to_string(), 5, 3, 1000, 0.01);
        let records = get_memtable_flushed();
        sstable.activate_flush(records);
        assert_ne!(sstable.search("key12".to_string()), record::empty());
        sstable.delete("key12".to_string());
        let mut loaded=SSTable::open(format!("{}/sstable_0002",SSTABLE_PATH.to_string()),5,3);
        assert_eq!(loaded.search("key12".to_string()), record::empty());
        assert_eq!(true,sstable.compare_similarity(loaded));
    }


}

pub fn get_memtable_flushed() -> Vec<EntryElement> {
    let record1 = record::new("key1".to_string(), vec![1, 2, 3], 1234567890);
    let record2 = record::new("key2".to_string(), vec![4, 5, 6], 1234567891);
    let record3 = record::new("key3".to_string(), vec![7, 8, 9], 1234567891);
    let record4 = record::new("key4".to_string(), vec![10, 11, 12], 1234567891);
    let record5 = record::new("key5".to_string(), vec![13, 14, 15], 1234567891);
    let record6 = record::new("key6".to_string(), vec![16,17, 18], 1234567891);
    let record7 = record::new("key7".to_string(), vec![19, 20, 21], 1234567891);
    let record8 = record::new("key8".to_string(), vec![22, 23, 24], 1234567891);
    let record9 = record::new("key9".to_string(), vec![25, 26, 27], 1234567891);
    let record10 = record::new("key10".to_string(), vec![28, 29, 30], 1234567891);
    let record11 = record::new("key11".to_string(), vec![31, 32, 33], 1234567891);
    let record12 = record::new("key12".to_string(), vec![34, 35, 36], 1234567891);
    let record13 = record::new("key13".to_string(), vec![37, 38, 39], 1234567891);
    let record14 = record::new("key14".to_string(), vec![40, 41, 42], 1234567891);
    let record15 = record::new("key15".to_string(), vec![43, 44, 45], 1234567891);
    let record16 = record::new("key16".to_string(), vec![46, 47, 48], 1234567891);
    let record17 = record::new("key17".to_string(), vec![49, 50, 51], 1234567891);
    let record18 = record::new("key18".to_string(), vec![52, 53, 54], 1234567891);
    let record19 = record::new("key19".to_string(), vec![55, 56, 57], 1234567891);
    let record20 = record::new("key20".to_string(), vec![58, 59, 60], 1234567891);
    let record21 = record::new("key21".to_string(), vec![61, 62, 63], 1234567891);
    let record22 = record::new("key22".to_string(), vec![64, 65, 66], 1234567891);
    let record23 = record::new("key23".to_string(), vec![67, 68, 69], 1234567891);
    let record24 = record::new("key24".to_string(), vec![70, 71, 72], 1234567891);
    let record25 = record::new("key25".to_string(), vec![73, 74, 75], 1234567891);
    let record26 = record::new("key26".to_string(), vec![76, 77, 78], 1234567891);
    let record27 = record::new("key27".to_string(), vec![79, 80, 81], 1234567891);
    let records = vec![record1, record2, record3, record4, record5, record6, record7, record8, record9, record10, record11, record12, record13, record14, record15, record16, record17, record18, record19, record20, record21, record22, record23, record24, record25, record26, record27];
    records
}