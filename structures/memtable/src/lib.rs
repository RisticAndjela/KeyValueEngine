pub mod memtable_hash_map;
pub mod memtable_btree;
pub mod memtable_skip_list;

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use entry_element::entry_element::EntryElement;
    use crate::memtable_btree::MemtableBTree;
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
    #[test]
    fn test_new_memtable() {
        let memtable = MemtableBTree::new(10, false);
        assert_eq!(memtable.max_size, 10);
        assert_eq!(memtable.current_count, 0);
        assert_eq!(memtable.read_only, false);
    }

    #[test]
    fn test_add_element_to_memtable() {
        let mut memtable = MemtableBTree::new(10, false);
        let key = "key1".to_string();
        let value = vec![1, 2, 3];
        let timestamp = 123456;

        memtable.add(key.clone(), value.clone(), timestamp);
        assert_eq!(memtable.current_count, 1);
        assert_eq!(memtable.get_value(key), value);
    }

    #[test]
    fn test_add_exceeding_max_size() {
        let mut memtable = MemtableBTree::new(1, false);  // max size is 1
        let key1 = "key1".to_string();
        let value1 = vec![1, 2, 3];
        let timestamp1 = 123456;

        let key2 = "key2".to_string();
        let value2 = vec![4, 5, 6];
        let timestamp2 = 123457;

        // Add first element
        memtable.add(key1.clone(), value1.clone(), timestamp1);
        assert_eq!(memtable.current_count, 1);

        // Try to add second element (should fail since max size is 1)
        memtable.add(key2.clone(), value2.clone(), timestamp2);
        assert_eq!(memtable.current_count, 1);  // count should remain the same
        assert!(memtable.get_value(key2).is_empty());
    }

    #[test]
    fn test_add_element_in_read_only() {
        let mut memtable = MemtableBTree::new(10, true);  // memtable is read-only
        let key = "key1".to_string();
        let value = vec![1, 2, 3];
        let timestamp = 123456;

        memtable.add(key.clone(), value.clone(), timestamp);
        assert_eq!(memtable.current_count, 0);  // Should not add since it's read-only
        assert!(memtable.get_value(key).is_empty());
    }

    #[test]
    fn test_delete_element() {
        let mut memtable = MemtableBTree::new(10, false);
        let key = "key1".to_string();
        let value = vec![1, 2, 3];
        let timestamp = 123456;

        // Add and then delete the element
        memtable.add(key.clone(), value.clone(), timestamp);
        assert_eq!(memtable.current_count, 1);
        assert_eq!(memtable.get_value(key.clone()), value);

        memtable.delete(key.clone());
        assert_eq!(memtable.get_value(key), vec![]);  // Should be empty after deletion
    }

    #[test]
    fn test_flush_memtable() {
        let mut memtable = MemtableBTree::new(10, false);
        let key1 = "key1".to_string();
        let value1 = vec![1, 2, 3];
        let timestamp1 = 123456;

        let key2 = "key2".to_string();
        let value2 = vec![4, 5, 6];
        let timestamp2 = 123457;

        // Add two elements
        memtable.add(key1.clone(), value1.clone(), timestamp1);
        memtable.add(key2.clone(), value2.clone(), timestamp2);

        // Flush the memtable (it should now be read-only)
        let flushed_elements = memtable.flush();

        assert_eq!(flushed_elements.len(), 2);
        assert_eq!(flushed_elements[0].key, key1);
        assert_eq!(flushed_elements[0].value, value1);
        assert_eq!(flushed_elements[1].key, key2);
        assert_eq!(flushed_elements[1].value, value2);

        assert!(memtable.read_only);
    }

    #[test]
    fn test_kill_memtable() {
        let mut memtable = MemtableBTree::new(10, false);
        memtable.kill();
        assert!(memtable.read_only);  // After killing, memtable should be read-only
    }

    #[test]
    fn test_memtable_is_full() {
        let mut memtable = MemtableBTree::new(2, false);
        let key1 = "key1".to_string();
        let value1 = vec![1, 2, 3];
        let timestamp1 = 123456;

        let key2 = "key2".to_string();
        let value2 = vec![4, 5, 6];
        let timestamp2 = 123457;

        let key3 = "key3".to_string();
        let value3 = vec![7, 8, 9];
        let timestamp3 = 123458;

        // Add two elements (should work)
        memtable.add(key1.clone(), value1.clone(), timestamp1);
        memtable.add(key2.clone(), value2.clone(), timestamp2);
        assert_eq!(memtable.current_count, 2);

        // Try to add a third (should fail, max size is 2)
        memtable.add(key3.clone(), value3.clone(), timestamp3);
        assert_eq!(memtable.current_count, 2);  // count should remain the same
        assert!(memtable.get_value(key3).is_empty());
    }

}
