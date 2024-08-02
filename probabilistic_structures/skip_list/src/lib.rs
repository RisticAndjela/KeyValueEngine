mod node;
mod skip_list;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::node::Node;
    use crate::skip_list::{print_all,SkipList};

    #[test]
    fn clone_node(){
        let node=Node::new("some value".as_bytes().to_vec(),10);
        let cloned=node.clone();
        assert_eq!(node,cloned);
       }
    #[test]
    fn reconnect_nodes(){
        let mut first_node=Node::new("some value".as_bytes().to_vec(),1);
        let mut node1=Node::new("some value".as_bytes().to_vec(),13);
        let last_node=Node::new("some value".as_bytes().to_vec(),20);
        first_node.next=Option::from(Box::new(last_node));
        let mut simple=first_node.clone();
        first_node.reattach_next(&mut node1);
        simple.reconnect(first_node);
        assert_eq!(simple.clone().down.unwrap().deref().clone().key,1);
        assert_eq!(simple.clone().down.unwrap().deref().clone().next.unwrap().deref().clone().key,13);
        assert_eq!(simple.clone().next.unwrap().deref().clone().key,20);
    }
    #[test]
    fn get_all_firsts(){
        let mut first_node=Node::new("some value".as_bytes().to_vec(),1);
        let last_node=Node::new("some value".as_bytes().to_vec(),20);
        first_node.next=Option::from(Box::new(last_node));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add_blank();
        skip_list.add_blank();
        skip_list.add_blank();
        // print_all(skip_list);

    }
    #[test]
    fn add(){
        let mut first_node=Node::new("some value".as_bytes().to_vec(),1);
        let last_node=Node::new("some value".as_bytes().to_vec(),20);
        first_node.next=Option::from(Box::new(last_node));
        first_node.down= Option::from(Box::new(first_node.clone()));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add("some value".as_bytes().to_vec(),17);
        skip_list.add("some value".as_bytes().to_vec(),13);
        // skip_list.add(21,53); cannot for now
        assert_eq!(skip_list.first_node.next.unwrap().deref().clone().key,20);
        assert_eq!(skip_list.first_node.down.unwrap().deref().clone().key,1);
        //others check with print because skip list is unpredictable structure
    }
    #[test]
    fn search(){
        let mut first_node=Node::new("some value 1".as_bytes().to_vec(),1);
        let last_node=Node::new("some value 50".as_bytes().to_vec(),50);
        first_node.next=Option::from(Box::new(last_node));
        first_node.down= Option::from(Box::new(first_node.clone()));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add("some value 17".as_bytes().to_vec(),17);
        skip_list.add("some value 13".as_bytes().to_vec(),13);
        skip_list.add("some value 14".as_bytes().to_vec(),14);
        skip_list.add("some value 21".as_bytes().to_vec(),21);
        skip_list.add("some value 31".as_bytes().to_vec(),31);
        skip_list.add("some value 22".as_bytes().to_vec(),22);
        skip_list.add("some value 42".as_bytes().to_vec(),42);
        skip_list.add("some value 9".as_bytes().to_vec(),9);
        skip_list.add("some value 3".as_bytes().to_vec(),3);
        assert_eq!(skip_list.search(22).1,"some value 22".as_bytes().to_vec());
        assert_eq!(skip_list.search(21).1,"some value 21".as_bytes().to_vec());
        assert_eq!(skip_list.search(23).0,false);
    }
    #[test]
    fn remove(){
        let mut first_node=Node::new("some value 1".as_bytes().to_vec(),1);
        let last_node=Node::new("some value 50".as_bytes().to_vec(),50);
        first_node.next=Option::from(Box::new(last_node));
        first_node.down= Option::from(Box::new(first_node.clone()));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add("some value 17".as_bytes().to_vec(),17);
        skip_list.add("some value 13".as_bytes().to_vec(),13);
        skip_list.add("some value 14".as_bytes().to_vec(),14);
        skip_list.remove(250);
        skip_list.remove(14);
        print_all(skip_list.clone());
        assert_eq!(skip_list.search(13).0,true);
        assert_eq!(skip_list.search(14).0,false);
    }

}
