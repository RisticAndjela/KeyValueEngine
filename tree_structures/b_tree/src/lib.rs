use std::time::{SystemTime, UNIX_EPOCH};
use entry_element::entry_element::EntryElement;
use crate::node::Node;

mod b_tree;
mod node;
mod file;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::time::{SystemTime, UNIX_EPOCH};
    use entry_element::entry_element::EntryElement;
    use crate::{get_full_node, get_some_node, get_some_shorter_node};
    use crate::node::Node;
    #[test]
    fn test_node_initializer() {
        let mut node = Node::initialize_new( 9);
        node.add_to_k_empty_children_and_elements( 6 * 2 - 1);
        assert_eq!(node.elements.len(), 6 * 2 - 1);
        assert_eq!(node.children.len(), 6 * 2);
        assert_eq!(node.num_of_relative_children(), 12);
        assert_eq!(node.num_of_relative_elements(), 0);
        assert_eq!(node.get_max_height(),2);//by initializer
    }
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
        assert!(!node2.need_new_level());
        let entry1 = EntryElement::new("key1".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let node=Node{elements:vec![entry1],children:vec![]};
        assert!(node.need_new_level());
    }
    #[test]
    fn test_add_new_level(){
        let mut node=get_full_node().sort_all_elements_and_children();
        node=node.clone().add_new_level(3).unwrap().deref().clone();
        assert!(node.children[0].clone().unwrap().deref().clone().children[0].is_some());
        assert_eq!(node.children[0].clone().unwrap().deref().clone().elements[0].key,"key1".to_string());//stayed same
        assert_eq!(node.children[1].clone().unwrap().deref().clone().elements[0].key,"key5".to_string());//stayed same
        assert!(node.children[4].clone().unwrap().deref().clone().elements[0].is_irrelevant());//added empty elements
        assert!(node.children[0].clone().unwrap().deref().clone().children[0].is_some())//new node new leaf

    }
    #[test]
    fn test_add(){
        let entry8 = EntryElement::new("key8".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry22 = EntryElement::new("key22".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry23 = EntryElement::new("key23".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry24 = EntryElement::new("key24".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry25 = EntryElement::new("key25".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry26 = EntryElement::new("key26".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry27 = EntryElement::new("key27".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry28 = EntryElement::new("key28".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry29 = EntryElement::new("key29".to_string(), "some value".as_bytes().to_vec(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let mut node= get_some_shorter_node().sort_all_elements_and_children();
        node.add(entry8);
        node.add(entry22);
        node.add(entry23);
        node.add(entry24);
        node.add(entry25);
        node.add(entry26);
        node.add(entry27);
        node.add(entry28);
        node.add(entry29);
        println!("{:?}",node.clone());
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