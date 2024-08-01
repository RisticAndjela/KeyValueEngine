use std::ops::Deref;
use crate::node;
use crate::node::Node;

pub struct SkipList {
    pub first_node: Node,
    pub base: Node, // first-last only, no other downs or nexts
    pub max_level: i32,
}

impl SkipList {
    pub fn new(first_node: Node) -> Self {
        SkipList {
            first_node: first_node.clone(),
            base: first_node.clone(),
            max_level: 1,
        }
    }
    pub fn add(&mut self, key: i32, value: i32) {
        if self.first_node.value > value {
            return; // some error perhaps
        }
        let mut levels = self.get_all_levels();
        let number_of_shows = self.num_of_flips(levels.len() as i32) as usize;
        levels = self.get_all_levels(); //reload
        levels.reverse(); // so that the first in list is the one with all values
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
                if current.clone().next.unwrap().deref().value > value {
                    // found spot, we had check before to see if it goes before first
                    let mut new_node = Node::new(key, value);
                    current.reattach_next(&mut new_node);
                    all_in_one_row.push(Some(Box::new(current.clone())));
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
        self.add_blank();
    }
    pub fn get_all_levels(&mut self) -> Vec<Node> {
        self.first_node.get_all_levels()
    }
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
        while rand::random::<u8>() % 2 == 0 {
            counter += 1;
        }
        if counter > bound {
            for _ in 0..((counter - bound) as usize) {
                self.add_blank();
            }
        }
        counter
    }
}

pub fn print_all(mut skip_list: SkipList) {
    for (i, node) in skip_list.get_all_levels().iter().enumerate() {
        println!("LEVEL {}", i);
        println!("{:?}", node);
    }
}
