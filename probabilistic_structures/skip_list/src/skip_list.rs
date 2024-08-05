use std::ops::Deref;
use rand::Rng;
use entry_element::entry_element::{EntryElement,extract};
use crate::node;
use crate::node::{Node, print_node};

pub const ODDS_OF_FLIPS: i32 =50; //right now its 80 percent chance that it will be yes and 20 percent for no
pub struct SkipList {
    pub first_node: Node,
    pub base: Node, // first-last only, no other downs or nexts, just to simplify adding blank levels
}

impl SkipList {
    pub fn new(first_node: Node) -> Self {
        SkipList {
            first_node: first_node.clone(),
            base: first_node.clone(),
        }
    }
    pub fn get_all_levels(&mut self) -> Vec<Node> {self.first_node.get_all_levels()}
    pub fn add_blank(&mut self){
        let mut new_first=self.base.clone();
        let last_first=self.first_node.clone();
        new_first.down=Option::from(Box::new(last_first.clone()));
        let mut last_next_from_last_first=last_first.next.clone();
        loop{
            if last_next_from_last_first.clone().unwrap().next.is_none(){break}
            last_next_from_last_first=last_next_from_last_first.unwrap().next.clone();
        }
        let mut new_next_from_new_first =new_first.clone().next.unwrap().deref().clone();
        new_next_from_new_first.down=last_next_from_last_first;
        new_first.next=Option::from(Box::new(new_next_from_new_first));
        self.first_node=new_first.clone();
    }
    pub fn num_of_flips(&mut self, bound: i32) -> i32 {
        let mut counter = 1;
        let mut rng = rand::thread_rng();
        while rng.gen_range(0..100) < ODDS_OF_FLIPS {
            counter += 1;
        }
        if counter > bound {
            for _ in 0..((counter - bound) as usize) {
                self.add_blank();
            }
        }
        counter
    }
    pub fn add(&mut self, value: EntryElement) {
        let key=extract(value.key.clone().as_str());
        if key.is_none(){panic!("bad key value")}
        if self.first_node.value.clone().extract_number_from_key().unwrap() > key.unwrap().clone() ||
            self.clone().base.next.unwrap().value.clone().extract_number_from_key().unwrap() < key.unwrap().clone(){
            panic!("cannot add out of bounds")        }
        let mut levels = self.get_all_levels();
        let number_of_shows = self.num_of_flips(levels.len() as i32) as usize;
        levels = self.get_all_levels(); //reload
        levels.reverse(); // so that the first in list is the one with all value.keys
        let mut unchanged: Vec<Node>=vec![];
        if number_of_shows+1<levels.len(){
            unchanged= levels[number_of_shows+1..].to_vec();
        }
        let mut final_levels: Vec<Node> = vec![];
        levels.truncate(number_of_shows); // so that I go until the level I need to change and no more

        for first_in_row in levels {
            let mut all_in_one_row: Vec<Option<Box<Node>>> = vec![];
            let mut current = first_in_row.clone();
            'attach_loop: loop {
                if current.next.is_none() {
                    return; // out of bounds of last element
                }
                if current.clone().next.unwrap().deref().value.clone().extract_number_from_key().unwrap() > key.unwrap() {
                    // found spot, we had check before to see if it goes before first
                    let mut new_node = Node::new(value.clone());
                    current.reattach_next(&mut new_node);
                    let mut need_more=current.clone();
                    'rest:loop{
                        all_in_one_row.push(Some(Box::new(need_more.clone())));
                        if need_more.next.is_none(){break 'rest}
                        need_more=need_more.next.unwrap().deref().clone();
                    }
                    break 'attach_loop;
                }
                all_in_one_row.push(Some(Box::new(current.clone())));
                current = current.next.unwrap().deref().clone();
            }
            // now from vector all in one row need to join in new first
            let new_first = node::reattach_all_nexts_from_vector(all_in_one_row);
            final_levels.push(new_first);
        }

        // Add the unchangeable levels back to final_levels
        final_levels.extend(unchanged);
        // final_levels has all first nodes, but they need to be better connected in between themselves
        for i in 0..final_levels.len() - 1 {
            let row_behind=final_levels[i].clone();
            let mut row_next=final_levels[i+1].clone();
            row_next.reconnect(row_behind);
            final_levels[i + 1]=row_next;
        }

        self.first_node = final_levels.last().unwrap().clone();
        if self.first_node.clone().next.unwrap().next.is_some() {
            self.add_blank();
        }

    }
    pub fn search(&mut self, key_attribute:String) ->(bool, EntryElement){
        let key=extract(key_attribute.clone().as_str());
        let not_found=(false,EntryElement::empty());
        let mut current=self.first_node.clone();
        loop{
            let curr=current.value.clone().key;
            if curr.eq(&key_attribute){
                return (true,current.value)
            }
            //case 1: if its smaller -> go next
            //case 2: go down
            if current.clone().next.is_some(){
                if current.clone().next.unwrap().clone().value.extract_number_from_key().unwrap()>key.unwrap(){
                    if current.down.is_none(){return not_found}
                    current=current.clone().down.unwrap().deref().clone();
                }
                else{
                    if current.next.is_none(){return not_found}
                    current=current.clone().next.unwrap().deref().clone();
                }
            }else{return not_found}

        }
    }
    //it also removes copies of same levels because it can happen to delete a node and leave same levels
    pub fn remove(&mut self, key_attribute: String) {
        let key=extract(key_attribute.clone().as_str());
        while self.search(key_attribute.clone()).0 {
            let levels = self.get_all_levels();
            let mut final_levels: Vec<Node> = vec![];

            for first_in_level in levels {
                let mut current = first_in_level.clone();
                let mut all_in_row: Vec<Option<Box<Node>>> = vec![];
                loop {
                    if current.value.clone().extract_number_from_key().unwrap() != key.clone().unwrap() {
                        all_in_row.push(Some(Box::new(current.clone())));
                    }
                    if let Some(next_node) = current.next.clone() {
                        current = next_node.deref().clone();
                    } else {
                        break;
                    }
                }

                if !all_in_row.is_empty() {
                    let new_first = node::reattach_all_nexts_from_vector(all_in_row);
                    final_levels.push(new_first);
                }
            }

            final_levels.reverse();
            for i in 0..final_levels.len()-1{
                let mut up=final_levels[i+1].clone();
                let down=final_levels[i].clone();
                up.reconnect(down);
                final_levels[i+1]=up.clone();
            }
            self.first_node = final_levels[0].clone();
            self.add_blank();
        }
    }
}

pub fn print_all(mut skip_list: SkipList) {
    let levels = skip_list.get_all_levels();
    for (i, node) in levels.iter().enumerate() {
        println!("LEVEL {}", i);
        print_node(node.clone());
    }
}

impl Clone for SkipList{
    fn clone(&self)->Self{
        SkipList{first_node:self.first_node.clone(),base:self.base.clone() }
    }
}
