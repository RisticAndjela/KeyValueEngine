pub mod merkle_tree;
pub mod node;
pub mod serialization;

#[cfg(test)]
mod tests {
    use crate::merkle_tree::MerkleTree;
    use crate::node;
    use crate::node::Node;
    use crate::serialization::{deserialize_tree, serialize_tree};

    #[test]
    fn empty_merkle() {
        let chain= node::Node::make_empty_chain(10);
        assert_eq!(chain.has_empty_child(),true);
        assert_eq!(chain.clone().left.unwrap().height,9);
        assert_eq!(chain.clone().left.unwrap().left.unwrap().height,8);
        assert_eq!(chain.clone().left.unwrap().left.unwrap().right.unwrap().height,7);
    }

    #[test]
    fn merkle(){
        let mut tree = MerkleTree::new();
        tree.add([1].to_vec());
        let first=tree.clone();
        for i in 2..=11 {
            tree.add([i].to_vec());
        }
        let before = tree.clone();
        tree.add([12].to_vec());
        println!("Hamming distance tree with only 1 and tree with 1 2 3 4 5 6 7 8 9 10 11 12: {}", tree.compare(first));
        println!("Hamming distance tree with only 1 2 3 4 5 6 7 8 9 10 11 and tree with 1 2 3 4 5 6 7 8 9 10 11 12: {}", tree.compare(before.clone()));
        println!("Hamming distance tree with only 1 2 3 4 5 6 7 8 9 10 11 12 and tree with 1 2 3 4 5 6 7 8 9 10 11 12: {}", tree.compare(tree.clone()));
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().left.unwrap().left.unwrap().value, [1].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().left.unwrap().right.unwrap().value, [2].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().right.unwrap().left.unwrap().value, [3].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().left.unwrap().right.unwrap().right.unwrap().value,[4].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().left.unwrap().left.unwrap().value,[5].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().left.unwrap().right.unwrap().value,[6].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().right.unwrap().left.unwrap().value,[7].to_vec());
        assert_eq!(tree.root.clone().unwrap().left.unwrap().right.unwrap().right.unwrap().right.unwrap().value,[8].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().left.unwrap().left.unwrap().value,[9].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().left.unwrap().right.unwrap().value,[10].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().right.unwrap().left.unwrap().value,[11].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().left.unwrap().right.unwrap().right.unwrap().value,[12].to_vec());
        assert_eq!(tree.root.clone().unwrap().right.unwrap().right.unwrap().right.unwrap().right.unwrap().value,vec![]);
     }
    #[test]
    fn serialization(){
        let tree = MerkleTree {
            root: Some(Box::new(Node {
                value: [1,2,3].to_vec(),
                left: None,
                right: Some(Box::new(Node {
                    value: [4,5,6].to_vec(),
                    left: None,
                    right: None,
                    height: 2,
                })),
                height: 1,
            })),
        };

        let serialized = serialize_tree(&tree);
        let deserialized_tree = deserialize_tree(&serialized);

        assert_eq!(tree.root.clone().unwrap().value, deserialized_tree.root.unwrap().value);
        assert_eq!(tree.root.clone().unwrap().right.unwrap().value,[4,5,6].to_vec());
    }
}