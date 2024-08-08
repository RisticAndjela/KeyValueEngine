use std::ops::Deref;
use entry_element::entry_element::{EntryElement, extract};
use crate::node::Node;
// implementation for insertion
 impl Node{
     // IDEA: find the leaf and parent where element should go, this implementation of tree(nodes completion) works based on a fact that only leaves have spots with empty elements
     // when I am adding new level I instantly flush all elements in certain positions so that the leaves have enough space
     // problem here is that I will eventually come to the parts where I cannot borrow anything from anywhere in last parent-leaves tree,
     // so I will be forced to take it from another last parent-tree, the values can be taken all the way to the roof which makes this complicated while working with large amount of data
     pub fn add(&mut self, element:EntryElement){
         //search -> none
         if element.extract_number_from_key().is_none(){ panic!("wrong key structure")}
         if self.will_overflow(element.key.clone()) {
             //handle problem
             if self.is_current_subtree_filled() {
                 self.updated_level_space();
                 self.add(element);
                 return;
             } else {
                 // child split
                 self.handle_overflow(element.clone());
                 return;
             }
         }
         self.add_normally(element.clone());
         let sorted=self.sort_all_elements_and_children();
         self.elements=sorted.elements;
         self.children=sorted.children;
     }
     fn add_normally(&mut self, element: EntryElement) -> Node {
         // If the node is a leaf, add the element and sort the elements
         if self.is_leaf() {
             self.elements.push(element);
             self.sort_elements();
             for (i,e) in self.elements.clone().iter().enumerate(){
                 if e.is_irrelevant(){
                     self.elements.remove(i);
                     break;
                 }
             }
             return self.clone();
         }
         let key = element.extract_number_from_key().unwrap();
         let mut position = 0;
         for i in 0..self.elements.len() {
             let to_compare = self.elements[i].extract_number_from_key();
             if to_compare.is_none(){
                 //elements are not full add here but take from someone to full fill
                 panic!();
             }
             if to_compare.unwrap() < key {
                 position = i + 1;
             }else{break;}
         }
         let child = self.children[position].clone().unwrap();
         let new_child = child.clone().add_normally(element);

         self.children[position] = Some(Box::new(new_child));
         self.clone() // return the updated node
     }

     // idea is to find the node and element to put, next I need to see which element from all old ones plus new one is the best new parent,
     // when I find new parent I need to delete old one, and add them again looking the direction that the insertion will be fastest,
     // the node I leave as self need to have new value in itself and be sorted
     fn handle_overflow(&mut self, element: EntryElement) {
         let last_index_in_children =self.elements.clone().iter().len();
         let key = element.clone().extract_number_from_key().unwrap();
         let mut element_to_change = EntryElement::empty(); //for now is none
         let mut new_parent = EntryElement::empty();
         for i in 0..= last_index_in_children {
             let mut child_owner = self.children[i].clone().unwrap().deref().clone();
             //current element and child by i
             if i == last_index_in_children {
                 if child_owner.is_leaf() { //insert in here
                     (new_parent,element_to_change)=self.chose_new_parent(element.clone(),i,true);
                 }
                 else{
                     child_owner.add(element.clone());
                     self.children[i] = Option::from(Box::new(child_owner.clone()));
                     return;
                 }
                 break;
             }
             let element_in_question = self.elements[i].clone().extract_number_from_key().unwrap();
             if element_in_question > key { //found position
                 //either change parent or insert here if its leaf
                 if child_owner.is_leaf() { //insert in here
                     //decide on new parent
                     let direction_left = self.choose_direction_left((i - 1) as i32); //choosing to go towards the one we will arrive earlier
                     (new_parent,element_to_change)=self.chose_new_parent(element.clone(),i,direction_left);
                     break;
                 } else {//make it new self
                     child_owner.add(element.clone());
                     self.children[i] = Option::from(Box::new(child_owner.clone()));
                     return; //or break?
                 }
             }
         }

         let sorted=self.sort_all_elements_and_children();
         self.elements=sorted.elements;
         self.children=sorted.children;
         self.add(element_to_change.clone());
     }
     pub fn chose_new_parent(&mut self,element:EntryElement,mut current_position:usize,go_left:bool )->(EntryElement,EntryElement){
         let key=element.extract_number_from_key().unwrap();
         let last_index_in_children=self.elements.clone().len();
         let mut look_at_element_at_position=last_index_in_children-1;
         if go_left{
             // current_position-=1; //if going left I need children[i-1]
             look_at_element_at_position=0; // if going right I need to compare with last one in elements, if left - first
         }
         let mut new_parent =EntryElement::empty();
         let mut element_to_add_next =EntryElement::empty();
         //current childs first/last element is greater than key
         if self.children[current_position].clone().unwrap().elements[look_at_element_at_position].extract_number_from_key().unwrap() > key {
             if go_left{
                 new_parent = element.clone();
             }
             else{
                 new_parent = self.children[current_position-1].clone().unwrap().elements[look_at_element_at_position].clone();
                 let mut child=self.children[current_position-1].clone().unwrap().deref().clone();
                 child.elements[last_index_in_children-1]=element.clone();
                 let sorted_child=child.clone().sort_all_elements_and_children();
                 child.elements=sorted_child.elements;
                 child.children=sorted_child.children;
                 self.children[last_index_in_children]=Option::from(Box::new(child.clone()));
             }
         }
         // key is greater than the current childs first element
         else {
             if go_left {
                 new_parent = self.children[current_position].clone().unwrap().elements[look_at_element_at_position].clone();
                 let mut child = self.children[current_position].clone().unwrap().deref().clone();
                 child.elements[look_at_element_at_position] = element.clone();
                 let sorted_child=child.clone().sort_all_elements_and_children();
                 child.elements=sorted_child.elements;
                 child.children=sorted_child.children;
                 self.children[last_index_in_children] = Option::from(Box::new(child.clone()));
             }
             else{
                 new_parent = element.clone();
             }
         }
         if current_position==last_index_in_children{
             current_position-=1;
         }
         element_to_add_next = self.elements[current_position].clone();
         self.elements[current_position] = new_parent.clone();
         (new_parent, element_to_add_next)

     }
     pub fn will_overflow(&mut self, key_attribute:String)->bool{
         if self.num_of_relative_elements()<self.elements.clone().len() as i32{return false;}
         if self.is_leaf(){return true; } // reached leaf need new level
         let key= extract(key_attribute.as_str()).unwrap();
         for i in 0..self.elements.clone().len(){
             if self.elements.clone()[i].extract_number_from_key().unwrap()>key{
                 if self.children[i].clone().unwrap().are_elements_full() {return true; }
                 return false;
             }
         }
         let last_child = *self.children.clone().last().unwrap().clone().unwrap();
         if last_child.clone().are_elements_full() { return true; }
         else{false;}

         return false
     }
     pub fn are_elements_full(&self)->bool{
         let used_spots=self.clone().num_of_relative_elements();
         let max_capacity=self.elements.clone().len();
         used_spots==max_capacity as i32
     }
     //function that chooses in overflow weather is closer to go left or right, since it's called on 100% leaves there is never going to be situation where I will reach end and not get at least one empty spot
     pub fn choose_direction_left(&self, position:i32)->bool{
         let children=self.children.clone();

         if position==0{return false;}
         if position==(children.clone().len()-1) as i32{return true;}

         let mut next = position.clone();
         let mut before = position.clone();

         loop{
             next=next+1;
             before=before-1;

             if next>(children.clone().len()-1 )as i32{return true;} //on this side I will get empty spot
             if before<0{return false;} //on this side I will get empty spot

             let num_in_next=children.clone()[next as usize].clone().unwrap().num_of_relative_elements();
             let num_in_before=children.clone()[before as usize].clone().unwrap().num_of_relative_elements();

             if num_in_before==num_in_next{continue;}
             else if num_in_next<num_in_before{return false;}//right
             else if num_in_next>num_in_before{return true;}//left
         }
     }

 }