mod merkle_tree;
mod node;
mod serialization;

#[cfg(test)]
mod tests {
    use crate::merkle_tree::MerkleTree;
    use crate::node;
    use crate::node::Node;
    use crate::serialization::{deserialize_tree, serialize_tree};

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
        assert_eq!(chain.get_path_to_empty().len(),3);
    }
    #[test]
    fn merkle(){
        let mut tree= MerkleTree::new();
        tree.add([1].to_vec());
        tree.add([2].to_vec());
        tree.add([3].to_vec());
        tree.add([4].to_vec());
        tree.add([5].to_vec());
        tree.add([6].to_vec());
        tree.add([7].to_vec());
        tree.add([8].to_vec());
        tree.add([9].to_vec());
        tree.add([10].to_vec());
        tree.add([11].to_vec());
        tree.add([12].to_vec());
        // println!("ROOT {:?}",tree.root.clone().unwrap());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().left.unwrap().left.unwrap().value,[1].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().left.unwrap().right.unwrap().value,[2].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().right.unwrap().left.unwrap().value,[3].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().right.unwrap().right.unwrap().value,[4].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().left.unwrap().left.unwrap().value,[5].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().left.unwrap().right.unwrap().value,[6].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().right.unwrap().left.unwrap().value,[7].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().right.unwrap().right.unwrap().value,[8].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().left.unwrap().left.unwrap().value,[9].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().left.unwrap().right.unwrap().value,[10].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().right.unwrap().left.unwrap().value,[11].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().right.unwrap().right.unwrap().value,[12].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().right.unwrap().right.unwrap().right.unwrap().value,vec![0u8,1]);
    }
    #[test]
    fn serialization(){
        let tree = MerkleTree {
            root: Some(Box::new(Node {
                value: vec![1, 2, 3],
                left: None,
                right: Some(Box::new(Node {
                    value: vec![4, 5, 6],
                    left: None,
                    right: None,
                    some_child_empty: true,
                    height: 2,
                })),
                some_child_empty: false,
                height: 1,
            })),
        };

        let serialized = serialize_tree(&tree);
        let deserialized_tree = deserialize_tree(&serialized);

        assert_eq!(tree.root.unwrap().value, deserialized_tree.root.unwrap().value);
    }
}