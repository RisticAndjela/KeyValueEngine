mod merkle_tree;
mod node;

#[cfg(test)]
mod tests {
    use crate::merkle_tree::MerkleTree;
    use crate::node;

    #[test]
    fn empty_merkle() {
        let chain= node::Node::make_empty_chain(10);
        assert_eq!(chain.some_child_empty,true);
        assert_eq!(chain.get_all_leaves().len(),1024);
        assert_eq!(chain.clone().left.unwrap().height,9);
        assert_eq!(chain.clone().left.unwrap().left.unwrap().height,8);
        assert_eq!(chain.clone().left.unwrap().left.unwrap().right.unwrap().height,7);
    }
    #[test]
    fn add_to_empty_merkle() {
        let chain= node::Node::make_empty_chain(3);
        println!("{:?}",chain.get_path_to_empty());
        println!("{:?}",chain.get_all_leaves());
    }
    #[test]
    fn merkle(){
        let mut tree= MerkleTree::new();
        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);
        tree.add(6);
        tree.add(7);
        tree.add(8);
        tree.add(9);
        tree.add(10);
        tree.add(11);
        tree.add(12);
        println!("ROOT {:?}",tree.root.clone().unwrap());
        println!("Leaves: {:?}",tree.root.clone().unwrap().get_all_leaves().reverse());
    }

}