use std::time::{SystemTime, UNIX_EPOCH};
use entry_element::entry_element::EntryElement;
use crate::node::Node;

pub mod b_tree;
mod node;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::time::{SystemTime, UNIX_EPOCH};
    use entry_element::entry_element::EntryElement;
    use crate::{get_full_node, get_some_node, get_some_shorter_node};
    use crate::b_tree::BTree;
    use crate::node::Node;

    #[test]
    fn test_node_sorter() {
        let node = get_some_node().sort_all_elements_and_children();
        assert_eq!(node.elements[0].key, "key4".to_string());
        assert_eq!(node.elements[1].key, "key9".to_string());
        assert_eq!(node.elements[2].key, "key12".to_string());
        assert_eq!(node.children[0].clone().unwrap().children.len(), 0);
        assert_eq!(node.children[1].clone().unwrap().children.len(), 0);
    }
    #[test]
    fn test_will_overflow() {
        let mut node1 = get_some_node().sort_all_elements_and_children();
        assert!(!node1.will_overflow("key8".to_string())); //should not overflow
        let mut node2 = get_some_shorter_node().sort_all_elements_and_children();
        assert!(node2.will_overflow("key8".to_string())); //should overflow
        assert!(!node2.is_current_subtree_filled());
        let entry1 = EntryElement::new("key1".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let node=Node{elements:vec![entry1],children:vec![]};
        assert!(node.is_current_subtree_filled());
    }
    #[test]
    fn test_dealing_with_k_spots(){
        let mut full_node=get_full_node().sort_all_elements_and_children();
        full_node.updated_level_space();
        assert_eq!(full_node.elements.len(),5);
        assert_eq!(full_node.children.len(),6);
        assert_eq!(full_node.elements.last().unwrap().key,"key15".to_string());
        assert_eq!(full_node.elements[3].key,"key14".to_string());
        assert_eq!(full_node.elements[2].key,"key12".to_string());
        assert!(full_node.children[3].clone().unwrap().elements[1].is_irrelevant());
        assert!(full_node.children[3].clone().unwrap().elements[2].is_irrelevant());
    }
    #[test]
    fn test_add(){
        let mut root=Node::initialize_new(3);//smallest height is 2 so smallest num of elements is 2*2-1
        for i in 1..215{
            let mut str = "key".to_string();
            str.push_str(&i.to_string());
            root.add(EntryElement::new(str, "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64),true);
        }
        root.add(EntryElement::new("key215".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64),true);

        println!("{:?}",root);
    }
    #[test]
    fn test_btree(){
        let mut btree= BTree::new();
        for i in 1..216{
            let mut str = "key".to_string();
            str.push_str(&i.to_string());
            btree.add(EntryElement::new(str, "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64));
        }
        let find=btree.search("key1".to_string());
        assert_eq!("some value".as_bytes().to_vec(),find.value);
        for i in btree.get_all_elements_sorted(){
            print!(" {:?} ",i.key);
        }
        btree.delete("key1".to_string());
        let find=btree.search("key1".to_string());
        assert_eq!(true,find.tombstone);
        let find=btree.search("key2".to_string());
        assert_eq!(false,find.tombstone);

    }

}
fn get_some_node() -> Node {
    let entry1 = EntryElement::new("key1".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry2 = EntryElement::new("key2".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry3 = EntryElement::new("key3".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry4 = EntryElement::new("key4".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry5 = EntryElement::new("key5".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry6 = EntryElement::new("key6".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry7 = EntryElement::new("key7".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry9 = EntryElement::new("key9".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry12 = EntryElement::new("key12".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry14 = EntryElement::new("key14".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let node = Node {
        elements: vec![entry14, entry12,entry9, entry4, EntryElement::empty()], //i have 3 levels
        children: vec![
            Option::from(Box::new(Node {
            elements: vec![entry5, entry6, EntryElement::empty(), entry7, EntryElement::empty()],
            children: vec![],
        })),
            Option::from(Box::new(Node{
            elements: vec![entry2, entry3, entry1, EntryElement::empty(), EntryElement::empty()],
            children: vec![],
        })),
            Option::from(Box::new(Node ::none(5))),
            Option::from(Box::new(Node ::none(5))),
            Option::from(Box::new(Node ::none(5))),
            Option::from(Box::new(Node ::none(5))),
           ], // need 6 new nodes
    };
    node
}
fn get_some_shorter_node() -> Node {
    let entry1 = EntryElement::new("key1".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry2 = EntryElement::new("key2".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry3 = EntryElement::new("key3".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry4 = EntryElement::new("key4".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry5 = EntryElement::new("key5".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry6 = EntryElement::new("key6".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry7 = EntryElement::new("key7".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry9 = EntryElement::new("key9".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry14 = EntryElement::new("key14".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let node = Node {
        elements: vec![entry14,entry9, entry4], //i have 2 levels
        children: vec![Option::from(Box::new(Node {elements: vec![entry5, entry6, entry7],children: vec![], })),
                       Option::from(Box::new(Node { elements: vec![entry2, entry3, entry1], children: vec![], })),
                       Option::from(Box::new(Node::none(3))),
                       Option::from(Box::new(Node::none(3))), ], // need 6 new nodes
    };
    node
}
fn get_full_node()->Node{
    let entry1 = EntryElement::new("key1".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry2 = EntryElement::new("key2".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry3 = EntryElement::new("key3".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry4 = EntryElement::new("key4".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry5 = EntryElement::new("key5".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry6 = EntryElement::new("key6".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry7 = EntryElement::new("key7".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry8 = EntryElement::new("key8".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry9 = EntryElement::new("key9".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry10 = EntryElement::new("key10".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry11 = EntryElement::new("key11".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry12 = EntryElement::new("key12".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry13 = EntryElement::new("key13".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry14 = EntryElement::new("key14".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let entry15 = EntryElement::new("key15".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

    let node=Node{
        elements:vec![entry4,entry8,entry12], //3 elements
        children:vec![
            Option::from(Box::new(Node{
                elements:vec![entry1,entry2,entry3], //3 elements
                children:vec![ ], //4 children none
            })),
            Option::from(Box::new(Node{
                elements:vec![entry5,entry6,entry7], //3 elements
                children:vec![ ], //4 children none
            })),
            Option::from(Box::new(Node{
                elements:vec![entry9,entry10,entry11], //3 elements
                children:vec![ ], //4 children none
            })),
            Option::from(Box::new(Node{
                elements:vec![entry13,entry14,entry15], //3 elements
                children:vec![], //4 children none
            }))
        ], //4 children with no children
    };

    return node;
}