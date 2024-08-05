mod b_tree;
mod node;
#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use entry_element::entry_element::EntryElement;
    use crate::b_tree::BTree;
    #[test]
    fn test_new_btree() {
        let btree = BTree::new();
        assert!(btree.root.is_none());
        assert_eq!(btree.num_elements, 0);
        assert_eq!(btree.max_height, 1 as u32);
    }
    #[test]
    fn test_add_element() {
        let entry10=EntryElement::new("key10".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let mut btree = BTree::new();
        btree.add_element(entry10);
        assert_eq!(btree.num_elements, 1);
        assert!(btree.root.is_some());
        let root = btree.root.as_ref().unwrap();
        assert_eq!(root.elements[0].key, "key10".to_string());
        assert!(root.is_leaf);
    }
    #[test]
    fn test_add_multiple_elements() {
        let entry10=EntryElement::new("key10".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry20=EntryElement::new("key20".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry5=EntryElement::new("key5".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

        let mut btree = BTree::new();
        btree.add_element(entry10);
        btree.add_element(entry20);
        btree.add_element(entry5);
        assert_eq!(btree.num_elements, 3);
        let root = btree.root.as_ref().unwrap();
        assert_eq!(root.elements[1].key, "key10".to_string());
        assert_eq!(root.elements[2].key, "key20".to_string());
        assert_eq!(root.elements[0].key, "key5".to_string());
    }
    #[test]
    fn test_find_element() {
        let entry10=EntryElement::new("key10".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry20=EntryElement::new("key20".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry5=EntryElement::new("key5".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

        let mut btree = BTree::new();
        btree.add_element(entry10);
        btree.add_element(entry20);
        btree.add_element(entry5);
        let node = btree.find_element("key10".to_string());
        assert!(node.is_some());
        assert_eq!(node.unwrap().elements[1].key, "key10".to_string());
        let node = btree.find_element("key15".to_string());
        assert!(node.is_none());
    }
    #[test]
    fn test_delete() {
        let entry10=EntryElement::new("key10".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

        let mut btree = BTree::new();
        btree.add_element(entry10);
        assert_eq!(btree.num_elements, 1);
        btree.delete("key10".to_string());
        assert_eq!(btree.num_elements, 0);

    }

    #[test]
    fn test_split_child() {
        let entry10=EntryElement::new("key10".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry20=EntryElement::new("key20".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry15=EntryElement::new("key15".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry13=EntryElement::new("key13".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry25=EntryElement::new("key25".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry5=EntryElement::new("key5".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

        let mut btree = BTree::new();
        btree.add_element(entry10);
        btree.add_element(entry15);
        btree.add_element(entry20);
        btree.add_element(entry13);
        btree.add_element(entry25);
        btree.add_element(entry5);

        assert_eq!(btree.num_elements, 6);
        assert!(btree.root.is_some());
        let root = btree.root.as_ref().unwrap();
    }
}
