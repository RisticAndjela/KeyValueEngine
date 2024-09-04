mod memtable_hash_map;
mod memtable_btree;
mod memtable_skip_list;

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use entry_element::entry_element::EntryElement;
    use crate::memtable_hash_map::MemtableHashMap;
    use crate::memtable_skip_list::MemtableSkipList;

    #[test]
    fn test_memtable_hash_map_new() {
        let memtable = MemtableHashMap::new(10, false);
        assert_eq!(memtable.max_size, 10);
        assert_eq!(memtable.current_count, 0);
        assert_eq!(memtable.read_only, false);
        assert!(memtable.data.is_empty());
    }
    #[test]
    fn test_memtable_hash_map_add_element() {
        let mut memtable = MemtableHashMap::new(10, false);
        memtable.add("key1".to_string(), vec![1, 2, 3], 1234567890);

        assert_eq!(memtable.data.len(), 1);
        let element = memtable.data.get("key1").unwrap();
        assert_eq!(element.value, vec![1, 2, 3]);
        assert_eq!(element.timestamp, 1234567890);
    }
    #[test]
    fn test_memtable_hash_map_add_element_read_only() {
        let mut memtable = MemtableHashMap::new(10, true);
        memtable.add("key1".to_string(), vec![1, 2, 3], 1234567890);

        assert!(memtable.data.is_empty());
    }
    #[test]
    fn test_memtable_hash_map_delete_element() {
        let mut memtable = MemtableHashMap::new(10, false);
        memtable.add("key1".to_string(), vec![1, 2, 3], 1234567890);
        memtable.delete("key1".to_string());

        let element = memtable.data.get("key1").unwrap();
        assert!(element.is_irrelevant());
    }
    #[test]
    fn test_memtable_hash_map_delete_nonexistent_element() {
        let mut memtable = MemtableHashMap::new(10, false);
        memtable.delete("key1".to_string());

        let element = memtable.data.get("key1").unwrap();
        assert!(element.is_irrelevant());
    }
    #[test]
    fn test_memtable_hash_map_flush_memtable() {
        let mut memtable = MemtableHashMap::new(10, false);
        memtable.add("key2".to_string(), vec![4, 5, 6], 1234567891);
        memtable.add("key1".to_string(), vec![1, 2, 3], 1234567890);
        let flushed = memtable.flush();

        assert_eq!(flushed.len(), 2);
        assert_eq!(flushed[0].key, "key1");
        assert_eq!(flushed[1].key, "key2");
        assert!(memtable.read_only);
    }
    #[test]
    fn test_memtable_hash_map_get_value() {
        let mut memtable = MemtableHashMap::new(10, false);
        memtable.add("key1".to_string(), vec![1, 2, 3], 1234567890);

        let value = memtable.get_value("key1".to_string());
        assert_eq!(value, vec![1, 2, 3]);

        let nonexistent_value = memtable.get_value("nonexistent_key".to_string());
        assert!(nonexistent_value.is_empty());
    }
    #[test]
    fn test_memtable_skip_list(){
        let entry1=EntryElement::new("key1".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry50=EntryElement::new("key50".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let memtable = MemtableSkipList::new(10, false,entry1,entry50);
        assert_eq!(memtable.max_size, 10);
        assert_eq!(memtable.current_count, 2);
        assert_eq!(memtable.read_only, false);
    }
    #[test]
    fn test_memtable_skip_list_add_element() {
        let mut memtable = MemtableSkipList::new(10, false, EntryElement::new("key1".to_string(), vec![1, 2, 3], 1234567890), EntryElement::new("key50".to_string(), vec![4, 5, 6], 1234567891));
        memtable.add("key25".to_string(), vec![7, 8, 9], 1234567892);
        let e = memtable.get_value("key25".to_string());
        assert!(e== vec![7, 8, 9] && memtable.current_count == 3);
    }
    #[test]
    fn test_memtable_skip_list_add_element_read_only() {
        let mut memtable = MemtableSkipList::new(10, true, EntryElement::new("key1".to_string(), vec![1, 2, 3], 1234567890), EntryElement::new("key50".to_string(), vec![4, 5, 6], 1234567891));
        memtable.add("key25".to_string(), vec![7, 8, 9], 1234567892);
        let e = memtable.get_value("key25".to_string());
        assert!(e==vec![] && memtable.current_count == 2);
    }
    #[test]
    fn test_memtable_skip_list_delete_element() {
        let mut memtable = MemtableSkipList::new(10, false, EntryElement::new("key1".to_string(), vec![1, 2, 3], 1234567890), EntryElement::new("key50".to_string(), vec![4, 5, 6], 1234567891));
        memtable.add("key25".to_string(), vec![7, 8, 9], 1234567892);
        memtable.delete("key25".to_string());
        let e = memtable.get_value("key25".to_string());
        assert!(e==vec![] && memtable.current_count == 2);
    }
    #[test]
    fn test_memtable_skip_list_delete_nonexistent_element() {
        let mut memtable = MemtableSkipList::new(10, false, EntryElement::new("key1".to_string(), vec![1, 2, 3], 1234567890), EntryElement::new("key50".to_string(), vec![4, 5, 6], 1234567891));
        assert!(memtable.current_count == 2);
        memtable.delete("key25".to_string());
        let e = memtable.get_value("key25".to_string());
        assert!(e==vec![]);
        let e_again = memtable.get_value("key25".to_string());
        assert!(e_again==vec![]);
        assert!(memtable.current_count == 3);
    }
    #[test]
    fn test_memtable_skip_list_flush() {
        let mut memtable = MemtableSkipList::new(10, false, EntryElement::new("key1".to_string(), vec![1, 2, 3], 1234567890), EntryElement::new("key50".to_string(), vec![4, 5, 6], 1234567891));
        memtable.add("key25".to_string(), vec![7, 8, 9], 1234567892);
        let flushed = memtable.flush();
        assert!(flushed.len() == 3 && flushed[0].key == "key1" && flushed[1].key == "key25" && flushed[2].key == "key50" && memtable.read_only);
    }
    #[test]
    fn test_memtable_skip_list_get_value() {
        let mut memtable = MemtableSkipList::new(10, false, EntryElement::new("key1".to_string(), vec![1, 2, 3], 1234567890), EntryElement::new("key50".to_string(), vec![4, 5, 6], 1234567891));
        memtable.add("key25".to_string(), vec![7, 8, 9], 1234567892);
        let value = memtable.get_value("key25".to_string());
        assert_eq!(value, vec![7, 8, 9]);
    }
    #[test]
    fn test_memtable_skip_list_get_value_nonexistent() {
        let mut memtable = MemtableSkipList::new(10, false, EntryElement::new("key1".to_string(), vec![1, 2, 3], 1234567890), EntryElement::new("key50".to_string(), vec![4, 5, 6], 1234567891));
        let value = memtable.get_value("key25".to_string());
        assert!(value.is_empty());
    }


}
