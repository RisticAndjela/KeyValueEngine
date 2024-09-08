pub mod cache;
#[cfg(test)]
mod tests {
    use entry_element::entry_element::EntryElement;
    use crate::cache::Cache;

    #[test]
    fn test_cache_insertion() {
        let mut cache = Cache::new(3);
        let record1 = EntryElement::new("key1".to_string(),vec![1,2,3],123456789);
        let record2 = EntryElement::new("key2".to_string(),vec![4,5,6],123456789);
        let record3 = EntryElement::new("key3".to_string(),vec![7,8,9],123456789);

        cache.put(record1.clone());
        cache.put(record2);
        cache.put(record3);
        cache.put(record1);

        assert_eq!(cache.elements.len(), 3);
        assert!(cache.key_position.contains_key("key1"));
        assert_eq!(cache.key_position["key1"],2);
        assert!(cache.key_position.contains_key("key2"));
        assert_eq!(cache.key_position["key2"],0);
        assert!(cache.key_position.contains_key("key3"));
        assert_eq!(cache.key_position["key3"],1);

    }
    #[test]
    fn test_cache_replaces_least_recently_used() {
        let mut cache = Cache::new(2);

        let record1 = EntryElement::new("key1".to_string(),vec![1,2,3],123456789);
        let record2 = EntryElement::new("key2".to_string(),vec![4,5,6],123456789);
        let record3 = EntryElement::new("key3".to_string(),vec![7,8,9],123456789);

        cache.put(record1);
        cache.put(record2);

        cache.put(record3); //replacing 1

        assert_eq!(cache.elements.len(), 2);
        assert!(!cache.key_position.contains_key("key1"));
        assert!(cache.key_position.contains_key("key2"));
        assert_eq!(cache.key_position["key2"],0);
        assert!(cache.key_position.contains_key("key3"));
        assert_eq!(cache.key_position["key3"],1);
    }
    #[test]
    fn test_cache_updates_existing_element() {
        let mut cache = Cache::new(2);

        let mut record1 = EntryElement::new("key1".to_string(), vec![1, 2, 3], 123456789);
        let record2 = EntryElement::new("key2".to_string(),vec![4,5,6],123456789);

        // Insert 2 elements
        cache.put(record1.clone());
        cache.put(record2);

        // Update element1
        record1.tombstone=true;
        cache.put(record1);

        assert_eq!(cache.elements.len(), 2);
        let position=cache.key_position.get("key1").unwrap().clone();
        assert_eq!(cache.elements[position].1.tombstone, true);
    }
    #[test]
    fn test_cache_get() {
        let mut cache = Cache::new(3);

        let record1 = EntryElement::new("key1".to_string(), vec![1, 2, 3], 123456789);
        let record2 = EntryElement::new("key2".to_string(),vec![4,5,6],123456789);
        let record3 = EntryElement::new("key3".to_string(),vec![7,8,9],123456789);

        cache.put(record1.clone());
        cache.put(record2);
        cache.put(record3);

        assert_eq!(cache.get(&"key1".to_string()).unwrap(),record1.clone());
        assert_eq!(cache.get(&"key7".to_string()),None);
        let position=cache.key_position.get("key1").unwrap().clone();
        assert_eq!(position,2);
    }

}
