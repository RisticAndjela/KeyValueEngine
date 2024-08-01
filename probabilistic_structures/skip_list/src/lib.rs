mod node;
mod skip_list;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::node::Node;
    use crate::skip_list::{print_all, SkipList};

    #[test]
    fn clone_node(){
        let node=Node::new(32,10);
        let cloned=node.clone();
        assert_eq!(node,cloned);
       }
    #[test]
    fn reconnect_nodes(){
        let mut first_node=Node::new(32,1);
        let mut node1=Node::new(29,13);
        let last_node=Node::new(32,20);
        first_node.next=Option::from(Box::new(last_node));
        let mut simple=first_node.clone();
        first_node.reattach_next(&mut node1);
        simple.reconnect(first_node);
        assert_eq!(simple.clone().down.unwrap().deref().clone().value,1);
        assert_eq!(simple.clone().down.unwrap().deref().clone().next.unwrap().deref().clone().value,13);
        assert_eq!(simple.clone().next.unwrap().deref().clone().value,20);
    }
    #[test]
    fn get_all_firsts(){
        let mut first_node=Node::new(32,1);
        let last_node=Node::new(32,20);
        first_node.next=Option::from(Box::new(last_node));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add_blank();
        skip_list.add_blank();
        skip_list.add_blank();
        print_all(skip_list);

    }
    #[test]
    fn add(){
        let mut first_node=Node::new(32,1);
        let last_node=Node::new(32,20);
        first_node.next=Option::from(Box::new(last_node));
        first_node.down= Option::from(Box::new(first_node.clone()));
        let mut skip_list=SkipList::new(first_node);
        skip_list.add(31,17);
        skip_list.add(21,13);
        // skip_list.add(21,53); cannot for now
        print_all(skip_list);
    }

}
