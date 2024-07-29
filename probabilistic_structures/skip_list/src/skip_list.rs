use std::ops::Deref;
use crate::node::{Node};

pub struct SkipList{
    pub first_node:Box<Node>,
    top_domain:i32,
    bottom_domain:i32,
    max_level:usize
}

impl SkipList {
    pub fn new(top_domain: i32, bottom_domain: i32) -> Self {
        let mut first_node = Node::new(top_domain);
        let last_node = Node::new(bottom_domain);
        first_node.attach_next(last_node.clone());
        SkipList {
            first_node: Box::new(first_node),
            top_domain,
            bottom_domain,
            max_level: 1
        }
    }
    pub fn add(&mut self, value: i32) {
        let num_of_appearances = num_of_appearances();
        let new_rows = self.max_level as i32 - num_of_appearances;
        if value <= self.bottom_domain || value >= self.top_domain { return; }
        self.all_nodes_by_rows();
    }
    pub fn all_nodes_by_rows(&mut self) ->Vec<Vec<Node>>{
        let mut rows:Vec<Vec<Node>>=Vec::new();
        let mut first = self.first_node.deref().clone();
        loop {
            let mut all_of_one:Vec<Node>=Vec::new();
            let mut next=first.clone();
            loop{
                all_of_one.push(next.clone());
                if next.next.is_none(){break;}
                next=next.get_next().clone();
            }
            rows.push(all_of_one);
            if first.up.is_none(){break;}
            first=first.get_up().deref().clone();
        }
        for i in rows.iter(){

        }
        return rows;
    }

}
pub fn num_of_appearances()->i32{
    let mut count=1;//at least once
    while coin(){
        count+=1;
    }
    return count;
}
pub fn coin()->bool{
    let r=rand::random::<u8>();
    r%2==0
}