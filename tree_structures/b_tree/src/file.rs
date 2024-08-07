// use std::ops::Deref;
// use entry_element::entry_element::{EntryElement, extract};
// use crate::node::Node;
//
// impl Node {
//     // idea is to find the node and element to put, next I need to see which element from all old ones plus new one is the best new parent,
//     // when I find new parent I need to delete old one, and add them again looking the direction that the insertion will be fastest,
//     // the node I leave as self need to have new value in itself and be sorted
//     fn handle_overflow(&mut self, element: EntryElement) {
//         let final_capacity = self.elements.clone().iter().len();
//         let key = element.clone().extract_number_from_key().unwrap();
//         let mut element_to_change = EntryElement::empty(); //for now is none
//         let mut new_parent = EntryElement::empty();
//         for i in 0..self.elements.clone().len() - 1 {
//             //current element and child by i
//             let mut element_in_question = self.elements[i].clone().extract_number_from_key().unwrap();
//             let mut child_owner = self.children[i].clone().unwrap().deref().clone();
//             if element_in_question > key { //found position
//                 //either change parent or insert here if its leaf
//                 if child_owner.is_leaf() { //insert in here
//                     //decide on new parent
//                     if !self.children[i - 1].clone().unwrap().elements[0].is_irrelevant() { //if there is irrelevant should add normally and mistake is made before it came here
//                         //chose new parent
//                         let direction_left = self.choose_direction_left((i - 1) as i32); //choosing to go towards the one we will arrive earlier
//                         //going left
//
//
//                         if direction_left {
//                             if self.children[i - 1].clone().unwrap().elements[0].extract_number_from_key().unwrap() > key {
//                                 new_parent = element.clone();
//                             } else {
//                                 new_parent = self.children[i - 1].clone().unwrap().elements[0].clone();
//                                 let mut child = self.children[i - 1].clone().unwrap().deref().clone();
//                                 child.elements[0] = element.clone();
//                                 child.sort_all_elements_and_children();
//                                 self.children[final_capacity] = Option::from(Box::new(child.clone()));
//                             }
//                             element_to_change = self.elements[i - 1].clone();
//                             self.elements[i - 1] = new_parent.clone();
//                         }
//                         //going right
//                         else {
//                             if self.children[i - 1].clone().unwrap().elements[0].extract_number_from_key().unwrap() < key {
//                                 new_parent = element.clone();
//                             } else {
//                                 new_parent = self.children[i - 1].clone().unwrap().elements[0].clone();
//                                 let mut child = self.children[i - 1].clone().unwrap().deref().clone();
//                                 child.elements[final_capacity - 1] = element.clone();
//                                 child.sort_all_elements_and_children();
//                                 self.children[final_capacity] = Option::from(Box::new(child.clone()));
//                             }
//                             element_to_change = self.elements[i].clone();
//                             self.elements[i] = new_parent.clone();
//                         }
//                     }
//                     println!("here");
//                     break;
//                 } else {
//                     //make it new self
//                     child_owner.add(element.clone());
//                     self.children[i] = Option::from(Box::new(child_owner.clone()));
//                     return; //or break?
//                 }
//             }
//         }
//         //check for last child as well
//         if element_to_change.is_irrelevant() {
//             let mut last_child = self.children[final_capacity].clone().unwrap().deref().clone();
//             //either change parent or insert here if its leaf
//             if last_child.is_leaf() { //insert in here
//                 //decide on new parent
//                 if !last_child.clone().elements[0].is_irrelevant() { //if there is irrelevant should add normally and mistake is made before it came here
//                     //going left
//                     if last_child.clone().elements[0].extract_number_from_key().unwrap() > key {
//                         new_parent = element.clone();
//                     } else {
//                         new_parent = self.children[final_capacity].clone().unwrap().elements[0].clone();
//                         last_child.elements[final_capacity - 1] = element.clone();
//                         last_child.sort_all_elements_and_children();
//                         self.children[final_capacity] = Option::from(Box::new(last_child.clone()));
//                     }
//                     element_to_change = self.elements[final_capacity - 1].clone();
//                     self.elements[final_capacity - 1] = new_parent.clone();
//                 }
//                 println!("here now");
//             } else {
//                 panic!("need to check")
//             }
//         }
//
//         self.sort_all_elements_and_children();
//         self.add(element_to_change.clone());
//     }
// }