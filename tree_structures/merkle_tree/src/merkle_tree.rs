use crate::node::Step;
use std::ops::Deref;
use md5::{Digest, Md5};
use crate::merkle_tree;
use crate::node::Node;

#[derive(Clone, Debug)]
pub struct MerkleTree {
    pub root: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree { root: Option::from(Box::new(Node::new_empty(0)))}
    }
    pub fn add(&mut self,value: Vec<u8>){
        let mut path = self.root.clone().unwrap().get_path_to_empty();
        let mut all_nodes_on_path=vec![]; //I need this to go recursive through nodes
        let mut current_node=self.root.clone().unwrap().deref().clone();
        //loop through path
        for step in path.clone(){
            match step {
                Step::Left => {
                   current_node = current_node.left.unwrap().deref().clone();
                }
                Step::Right => {
                   current_node = current_node.right.unwrap().deref().clone();
                }
            }
            all_nodes_on_path.push(current_node.clone());
        }
        all_nodes_on_path.reverse();
        path.reverse();

        if all_nodes_on_path.len()==1{
            let mut root=self.root.clone().unwrap().deref().clone();
            all_nodes_on_path[0].value=value.clone();
            if root.left.clone().unwrap().value==vec![0u8,1]{
                all_nodes_on_path[0].some_child_empty=false;
                root.left=Option::from(Box::new(all_nodes_on_path[0].clone()));
                root.value= merkle_tree::combine_values(root.clone().left.unwrap().value.as_slice(), root.clone().right.unwrap().value.as_slice());
                self.root=Option::from(Box::new(root.clone()));
                return;
            }
            all_nodes_on_path[0].some_child_empty=false;
            root.right=Option::from(Box::new(all_nodes_on_path[0].clone()));
            root.some_child_empty=false;
            root.value= merkle_tree::combine_values(root.clone().left.unwrap().value.as_slice(), root.clone().right.unwrap().value.as_slice());
            self.root=Option::from(Box::new(root.clone()));
            self.new_root();
            return;
        }
        all_nodes_on_path.push(self.root.clone().unwrap().deref().clone());
        all_nodes_on_path[0].value=value.clone();
        all_nodes_on_path[0].some_child_empty=false;
        for i in 1..all_nodes_on_path.len(){
            match path[i-1] {
                Step::Left => {
                    all_nodes_on_path[i].left = Option::from(Box::new(all_nodes_on_path[i-1].clone()));
                    all_nodes_on_path[i].value= merkle_tree::combine_values(all_nodes_on_path[i].clone().left.unwrap().value.as_slice(), all_nodes_on_path[i].clone().right.unwrap().value.as_slice());
                    //value_to_put is same because according to sane logic right child is none and some child empty for node i+1 is true
                }
                Step::Right => {
                    all_nodes_on_path[i].right = Option::from(Box::new(all_nodes_on_path[i-1].clone()));
                    if all_nodes_on_path[i-1].clone().right.is_some(){
                        all_nodes_on_path[i].some_child_empty=all_nodes_on_path[i-1].clone().right.unwrap().some_child_empty;}
                    else{
                        all_nodes_on_path[i].some_child_empty=false;
                    };
                    all_nodes_on_path[i].value= merkle_tree::combine_values(all_nodes_on_path[i].clone().left.unwrap().value.as_slice(), all_nodes_on_path[i].clone().right.unwrap().value.as_slice());
                }
            }
        }
        let final_root=all_nodes_on_path[all_nodes_on_path.clone().len()-1].clone();
        self.root=Option::from(Box::new(final_root.clone()));

        if !final_root.some_child_empty{
            self.new_root();
        }
        self.refresh_value();
    }
    pub fn new_root(&mut self){
        if self.root.is_none(){self.root=Option::from(Box::new(Node::make_empty_chain(1)));return;}
        let right_node=Node::make_empty_chain(self.root.clone().unwrap().height);
        let new_root=Node::new_combined_node(self.root.clone().unwrap().deref().clone(),right_node.clone());
        self.root=Option::from(Box::new(new_root));

    }
    pub fn refresh_value(&mut self){
        let mut root=self.root.clone().unwrap().deref().clone();
        if root.left.is_some(){
            root.value=combine_values(root.clone().left.unwrap().value.as_slice(), root.clone().right.unwrap().value.as_slice());
        }
        self.root=Option::from(Box::new(root.clone()));
    }
}


pub fn combine_values(value1: &[u8], value2: &[u8]) -> Vec<u8> {
    let hash1 = Md5::digest(value1);
    let hash2 = Md5::digest(value2);

    let mut combined = Vec::with_capacity(hash1.len() + hash2.len());
    combined.extend_from_slice(&hash1);
    combined.extend_from_slice(&hash2);

    let final_hash = Md5::digest(&combined);

    final_hash.to_vec()
}