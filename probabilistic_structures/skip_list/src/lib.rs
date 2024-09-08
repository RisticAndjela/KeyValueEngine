pub mod node;
pub mod skip_list;
#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::time::{SystemTime, UNIX_EPOCH};
    use entry_element::entry_element::EntryElement;
    use crate::node::Node;
    use crate::skip_list::{print_all,SkipList};

    #[test]
    fn clone_node(){
        let entry=EntryElement::new("key10".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let node=Node::new(entry);
        let cloned=node.clone();
        assert_eq!(node,cloned);
       }
    #[test]
    fn reconnect_nodes(){
        let entry1=EntryElement::new("key1".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry13=EntryElement::new("key13".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry20=EntryElement::new("key20".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let mut first_node=Node::new(entry1.clone());
        let mut node1=Node::new(entry13.clone());
        let last_node=Node::new(entry20.clone());
        first_node.next=Option::from(Box::new(last_node));
        let mut simple=first_node.clone();
        first_node.reattach_next(&mut node1);
        simple.reconnect(first_node);
        assert_eq!(simple.clone().down.unwrap().deref().clone().value.key,"key1");
        assert_eq!(simple.clone().down.unwrap().deref().clone().next.unwrap().deref().clone().value.key,"key13");
        assert_eq!(simple.clone().next.unwrap().deref().clone().value.key,"key20");
    }
    #[test]
    fn get_all_firsts(){
        let entry1=EntryElement::new("key1".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry20=EntryElement::new("key20".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let mut first_node=Node::new(entry1.clone());
        let last_node=Node::new(entry20.clone());
        first_node.next=Option::from(Box::new(last_node));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add_blank();
        skip_list.add_blank();
        skip_list.add_blank();
        // print_all(skip_list);

    }
    #[test]
    fn add(){
        let entry1=EntryElement::new("key1".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry20=EntryElement::new("key20".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry17=EntryElement::new("key17".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry13=EntryElement::new("key13".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

        let mut skip_list=SkipList::make_new(entry1,entry20);
        skip_list.add(entry17);
        skip_list.add(entry13);
        assert_eq!(skip_list.first_node.next.unwrap().deref().clone().value.key,"key20".to_string());
        assert_eq!(skip_list.first_node.down.unwrap().deref().clone().value.key,"key1".to_string());
        //others check with print because skip list is unpredictable structure
    }
    #[test]
    fn search(){
        let entry1=EntryElement::new("key1".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry21=EntryElement::new("key21".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry13=EntryElement::new("key13".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry42=EntryElement::new("key42".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry35=EntryElement::new("key35".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry6=EntryElement::new("key6".to_string(),"some value 6".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry27=EntryElement::new("key27".to_string(),"some value 27".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry28=EntryElement::new("key28".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry19=EntryElement::new("key19".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry10=EntryElement::new("key10".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry50=EntryElement::new("key50".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

        let mut first_node=Node::new(entry1);
        let last_node=Node::new(entry50);
        first_node.next=Option::from(Box::new(last_node));
        first_node.down= Option::from(Box::new(first_node.clone()));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add(entry21);
        skip_list.add(entry13);
        skip_list.add(entry42);
        skip_list.add(entry35);
        skip_list.add(entry6);
        skip_list.add(entry27);
        skip_list.add(entry28);
        skip_list.add(entry19);
        skip_list.add(entry10);
        let all=skip_list.get_all_levels();
        println!("{:?}",all);
        print_all(skip_list.clone());
        assert_eq!(skip_list.search("key27".to_string()).1.value,"some value 27".as_bytes().to_vec());
        assert_eq!(skip_list.search("key6".to_string()).1.value,"some value 6".as_bytes().to_vec());
        assert_eq!(skip_list.search("key122".to_string()).0,false);
    }
    #[test]
    fn remove(){
        let entry1=EntryElement::new("key1".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry50=EntryElement::new("key50".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry17=EntryElement::new("key17".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry13=EntryElement::new("key13".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let entry14=EntryElement::new("key14".to_string(),"some value".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
        let mut first_node=Node::new(entry1);
        let last_node=Node::new(entry50);
        first_node.next=Option::from(Box::new(last_node));
        first_node.down= Option::from(Box::new(first_node.clone()));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add(entry17);
        skip_list.add(entry13);
        skip_list.add(entry14);
        skip_list.remove("key250".to_string());
        skip_list.remove("key14".to_string());
        // print_all(skip_list.clone());
        assert_eq!(skip_list.search("key13".to_string()).0,true);
        assert_eq!(skip_list.search("key14".to_string()).0,false);
    }

}
