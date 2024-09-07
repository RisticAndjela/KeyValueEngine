use std::ops::Deref;
use crate::merkle_tree::{combine_values};

#[derive(Clone, Debug)]
pub struct Node {
    pub value: Vec<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub height:i64
}

impl Node {
    pub fn new_combined_node(left: Node, right: Node) -> Self {
       Node { value: combine_values(left.value.clone(), right.value.clone()), left: Some(Box::new(left.clone())), right: Some(Box::new(right)),height:left.clone().height+1 }
    }
    pub fn new_empty(height:i64) -> Self {
        Node { value: vec![0u8;0], left: None, right: None ,height}
    }
    pub fn make_empty_chain(height: i64) -> Node {
        if height == 0 {
            return Self::new_empty(0);
        }

        let mut last_left = Self::new_empty(0);
        let mut last_right = Self::new_empty(0);
        let mut current_root = Self::new_empty(1);

        for i in 1..height {
            current_root.left = Some(Box::new(last_left));
            current_root.right = Some(Box::new(last_right));
            last_left = current_root.clone();
            last_right = current_root.clone();
            current_root = Self::new_empty(i + 1);
        }

        current_root.left = Some(Box::new(last_left));
        current_root.right = Some(Box::new(last_right));

        current_root
    }
    pub fn is_empty(&self)->bool{
        self.value==vec![0u8;0]
    }
    pub fn is_leaf(&self)->bool{self.height==0}
    pub fn has_empty_child(&self)->bool{
        if self.left.is_none()||self.right.is_none(){
            return if self.is_empty() { true } else { false }
        }
        if self.clone().left.unwrap().has_empty_child(){
            return true;
        }
        else{
            if self.clone().right.unwrap().has_empty_child(){
                return true;
            }
        }
        false
    }
    pub fn root_needs_to_double(&self)->bool{
        if self.left.is_none() && self.right.is_none(){
            return if self.is_empty() { false } else { true }
        }
        self.clone().right.unwrap().root_needs_to_double()
    }
    // recursively appending in tree
    pub fn add(&mut self,value:Vec<u8>){
        if self.left.is_none()&&self.right.is_none(){ // reached bottom
            self.value=value;
            return;
        }

        if self.clone().left.unwrap().has_empty_child(){
            let mut current=self.clone().left.unwrap();
            current.add(value.clone());
            self.left=Option::from(current.clone());
            self.value=self.clone().left.unwrap().value;

        }
        else{
            let mut current= self.clone().right.unwrap();
            current.add(value.clone());
            self.right=Option::from(current.clone());
            self.value=combine_values(self.clone().left.unwrap().value, self.clone().right.unwrap().value);
        }


    }
}
