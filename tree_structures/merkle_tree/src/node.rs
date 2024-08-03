use std::ops::Deref;
use crate::merkle_tree::{combine_values};

#[derive(Clone,Debug)]
pub enum Step {
    Left,
    Right
}

#[derive(Clone, Debug)]
pub struct Node {
    pub value: Vec<u8>, //for debug used i64
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub some_child_empty:bool,
    pub height:i64
}

impl Node {
    pub fn new_combined_node(left: Node, right: Node) -> Self {
       let mut has_empty= false;
        if left.some_child_empty || right.some_child_empty{has_empty=true;}
        Node { value: combine_values(left.value.clone().as_slice(), right.value.clone().as_slice()), left: Some(Box::new(left.clone())), right: Some(Box::new(right)),some_child_empty:has_empty,height:left.clone().height+1 }
    }
    pub fn new_empty(height:i64) -> Self {
        Node { value: vec![0u8,1], left: None, right: None,some_child_empty:true ,height}
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
    pub fn get_all_leaves(&self) -> Vec<Node> {
        let mut leaves = Vec::new();
        let mut stack = Vec::new();
        let binding = self.clone();
        stack.push(&binding);
        while let Some(node) = stack.pop() {
            if node.left.is_none() && node.right.is_none() {// if the node is a leaf add it to the leaves vector
                leaves.push(node.clone());
            } else {
                if let Some(left) = &node.left {// or push the children onto the stack
                    stack.push(left.as_ref());
                }
                if let Some(right) = &node.right {
                    stack.push(right.as_ref());
                }
            }
        }

        leaves
    }
    pub fn is_empty(&self)->bool{
        self.value==vec![0u8,1]
    }
    pub fn is_leaf(&self)->bool{self.height==0}
    pub fn get_path_to_empty(&self) -> Vec<Step> {
        let mut current= self.clone();
        let mut steps:Vec<Step>=vec![];
        loop{
            if current.is_leaf() && current.is_empty(){
                break;
            }
            if current.left.clone().unwrap().some_child_empty{
                current=current.left.unwrap().deref().clone();
                steps.push(Step::Left);
            }
            else{
                current=current.right.unwrap().deref().clone();
                steps.push(Step::Right);
            }
        }
        steps
    }

}
