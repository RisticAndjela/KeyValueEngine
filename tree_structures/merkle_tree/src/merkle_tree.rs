use std::ops::Deref;
use md5::{Digest, Md5};
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
        let mut root=self.clone().root.unwrap().deref().clone();
        if root.root_needs_to_double(){
            self.new_root();
            root=self.clone().root.unwrap().deref().clone();
        }
        root.add(value.clone());
        self.root=Option::from(Box::new(root.clone()));
    }
    pub fn new_root(&mut self){
        if self.root.is_none(){self.root=Option::from(Box::new(Node::make_empty_chain(1)));return;}
        let right_node=Node::make_empty_chain(self.root.clone().unwrap().height);
        let new_root=Node::new_combined_node(self.root.clone().unwrap().deref().clone(),right_node.clone());
        self.root=Option::from(Box::new(new_root));

    }
    pub fn compare(&self, other:MerkleTree)->f64{
        hamming_distance(&self.clone().root.unwrap().value, &other.root.unwrap().value) as f64 / 100.0
    }
}

fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    let len = a.len().min(b.len());
    let mut distance = 0;
    for i in 0..len {
        let diff = a[i] ^ b[i];
        distance += diff.count_ones() as usize;
    }
    distance += (a.len() as isize - b.len() as isize).abs() as usize * 8;

    distance}


pub fn combine_values(value1: Vec<u8>, value2: Vec<u8>) -> Vec<u8> {
    let hash1 = Md5::digest(value1);
    let hash2 = Md5::digest(value2);

    let mut combined = Vec::with_capacity(hash1.len() + hash2.len());
    combined.extend_from_slice(&hash1);
    combined.extend_from_slice(&hash2);

    let final_hash = Md5::digest(&combined);

    final_hash.to_vec()
}