// use std::ops::Deref;
// use entry_element::entry_element::{EntryElement, extract};
// use crate::node::Node;
//
//
// // implementation for insertion
// // lpl = last parent-leaf node in the subtree
// impl Node{
//     pub fn add(&mut self, element:EntryElement,is_root:bool){
//         //if I have a filled tree I will need to added it new level
//         if is_root && self.is_current_subtree_filled(){
//             self.updated_level_space(); // it will either add new children if I have enough elements by level equation or add spots for empty elements and rearrange tree
//         }
//         // check if overflow will happen
//         if self.will_do_overflow(extract(element.clone().key.as_str()).unwrap()){
//             if self.is_lpl(){
//                 self.handle_overflow_in_lpl(element.clone(),is_root);
//             }
//             else{
//                 self.handle_overflow_in_deep_tree(element.clone());
//             }
//             return;
//         }
//         // now I need to see if we are working with 2 level tree or more complex one
//         self.add_with_no_overflow(element.clone());
//     }
//     fn add_with_no_overflow(&mut self,element:EntryElement){ // self is root -> TRANSFER TO BTREE
//         let key= element.extract_number_from_key().unwrap(); // previously checked if there is wrong key structure, so I can safely unwrap
//         let height= self.get_max_height();
//         if height<=2 && self.num_of_relative_elements().clone()<3{ // if the node is a leaf, add the element and sort the elements
//             for i in 0..self.elements.clone().len(){
//                 if self.elements[i].clone().is_irrelevant(){
//                     self.elements[i]=element.clone();
//                     self.sort_elements();
//                     return;
//                 }
//             }
//         }
//         if height>=2{
//             self.add_to_deep_tree(element.clone());
//             return;
//         }
//         for child_index in 0..self.children.clone().len(){
//             let referent =self.children[child_index].clone().unwrap().elements[0].extract_number_from_key();
//             if referent.is_none() { //in children are empty elements
//                 let mut index=child_index.clone();
//                 if self.elements[index].extract_number_from_key().unwrap()<key{
//                     index+=1;
//                 }
//                 let last=self.elements.clone().len();
//                 let mut new_child =self.children[index].clone().unwrap().deref().clone();
//                 let mut new_elements=new_child.elements.clone();
//                 new_elements[last-1]=element.clone();
//                 new_child.elements=new_elements;
//                 new_child.sort_elements();
//                 self.children[index]=Option::from(Box::new(new_child));
//                 return
//
//             }
//             if referent.unwrap()>key{
//                 let mut to_replace=self.children[child_index-1].clone().unwrap();
//                 to_replace.add_with_no_overflow(element.clone());
//                 self.children[child_index-1]=Option::from(to_replace.clone());
//                 return;
//             }
//         }
//         // if it never returned it means I need to check the last one as well which held all biggest
//         let last_index=self.children.clone().len()-1;
//         let mut to_replace=self.children[last_index].clone().unwrap();
//         to_replace.add_to_deep_tree(element.clone());
//         self.children[last_index]=Option::from(to_replace.clone());
//         return;
//     }
//     fn add_to_deep_tree(&mut self,element: EntryElement){
//         if self.is_lpl(){
//             self.add_with_no_overflow_to_lpl(element.clone());
//             return;
//         }
//         let key = element.extract_number_from_key().unwrap();
//         for i in 0..self.elements.clone().len(){
//             let see=self.elements[i].clone().extract_number_from_key().unwrap();
//             if see>key{
//                 let mut child=self.children[i].clone().unwrap().deref().clone();
//                 child.add_to_deep_tree(element.clone());
//                 self.children[i]=Option::from(Box::new(child.clone()));
//                 return; //done
//             }
//         }
//         // it is in the biggest chunk
//         let length =self.elements.clone().len();
//         let mut last=self.children[length].clone().unwrap().deref().clone();
//         last.add_to_deep_tree(element.clone());
//         self.children[length]=Option::from(Box::new(last.clone()));
//     }
//     fn add_with_no_overflow_to_lpl(&mut self,element:EntryElement){
//         let key = element.extract_number_from_key().unwrap();
//
//         let mut position = 0;
//         let num_of_elements=self.elements.clone().len();
//         'find_position: for i in 0..=num_of_elements {
//             if i==num_of_elements{position=i;break 'find_position;} // it larger than last number in elements
//             let to_compare = self.elements[i].extract_number_from_key();
//             if to_compare.is_none(){ self.elements[i]=element.clone();return;}//elements are not full add here but take from someone to fulfill
//             if to_compare.unwrap() > key { position = i;break 'find_position; }
//         }
//
//         let mut child = self.children[position].clone().unwrap().deref().clone();
//         child.elements[num_of_elements-1]=element.clone();
//         child.sort_elements();
//
//         self.children[position] = Option::from(Box::new(child));
//     }
//
//     fn handle_overflow_in_deep_tree(&mut self, element:EntryElement){
//         let key= extract(element.clone().key.as_str()).unwrap();
//         let mut position=0;
//         for i in 0..self.children.clone().len(){
//             let represent= self.children[i].clone().unwrap().elements[0].extract_number_from_key();
//             if represent.unwrap()>key{
//                 break;
//             }
//             position+=1;
//         }
//
//         let left=self.direction_left_in_deep_tree(position as i32);
//         if left{
//             let mut current=self.children[position-1].clone().unwrap().deref().clone();
//             let add_next=current.pop_smallest_element_of_overflow();
//             current.add_with_no_overflow(element.clone());
//             self.children[position-1]=Option::from(Box::new(current.clone()));
//
//         }else {
//             let mut current=self.children[position+1].clone().unwrap().deref().clone();
//             let add_next=current.pop_smallest_element_of_overflow(); // pop largest here
//             self.children[position+1]=Option::from(Box::new(current.clone()));
//             self.add_with_no_overflow(element.clone());
//         }
//
//     }
//     fn pop_smallest_element_of_overflow(&mut self) ->EntryElement{ // self is one level bellow
//         if self.is_leaf(){
//             let result=self.elements[0].clone();
//             self.elements[0]=EntryElement::empty();
//             self.sort_elements();
//             return result;
//         }
//         for i in 0..self.children.clone().len(){
//             let mut child=self.children[i].clone().unwrap().deref().clone();
//             if child.is_current_subtree_filled(){// overflow is here
//                 let result= child.pop_smallest_element_of_overflow();
//                 self.children[i]=Option::from(Box::new(child.clone()));
//                 return result;//recursively
//             }
//         }
//         return EntryElement::empty();
//     }
//     fn direction_left_in_deep_tree(&self, position:i32)->bool{
//         let children = self.children.clone();
//         let mut next=position;
//         let mut before=position;
//         loop{
//             next+=1;
//             before-=1;
//
//             if next > (children.clone().len()-1) as i32 {return true;}
//             if before < 0 {return false;}
//
//             if !children[next as usize].clone().unwrap().is_current_subtree_filled(){return false} // have space
//             if !children[before as usize].clone().unwrap().is_current_subtree_filled(){return true} // have space
//         }
//     }
//     //this means I will have overflow I cannot handle only with lpl I will need to borrow values from other parents
//     fn handle_overflow_on_second_level(&mut self,element:EntryElement){
//         // need to find which element i would be dropping and on which side
//         // if I go left ... I would take from my child THE smallest element and immediately call on drop element
//         // if I go right ... I would take from my child THE largest element and immediately call on drop element
//         // rearrange because of missing spot
//         // then add this element to the right spot
//         let key=element.extract_number_from_key().unwrap();
//         let mut lpl=self.find_good_lpl(key.clone());
//         if lpl.is_current_subtree_filled(){
//             let index_of_node=self.get_all_lpl_nodes(vec![]).iter().enumerate().find(|&r| r.1.eq(&lpl.clone()) ).unwrap().0;
//             let go_left =self.choose_direction_left_on_overflow(index_of_node as i32);
//
//             if go_left{ self.take_smallest_out(index_of_node);self.add(element,false); }
//             else{self.take_largest_out(index_of_node); self.add(element,false); }
//         }
//         else{
//             if lpl.will_overflow(element.clone().key.to_string()){
//                 lpl.handle_overflow_in_lpl(element.clone(),false);
//             }
//             else{
//                 lpl.add_with_no_overflow(element.clone());
//             }
//             self.put_updated_lpl_back(lpl.clone());
//         }
//     }
//     fn will_do_overflow(&mut self,key:i64)->bool{
//         if self.is_leaf() || self.elements[0].is_irrelevant(){
//             // if there is only one empty in elements it will not overflow
//             return !self.elements.iter().any(|a| a.is_irrelevant());
//         }
//         for i in 0..self.elements.clone().len(){
//             let relative=self.elements[i].clone().extract_number_from_key();
//             if relative.is_none(){return false;}
//             if relative.unwrap()>key{return self.children[i].clone().unwrap().will_do_overflow(key.clone())}
//         }
//         let relative=self.elements.last().unwrap().extract_number_from_key();
//         if relative.is_none(){return false;}
//         return self.children.last().clone().unwrap().clone().unwrap().deref().clone().will_do_overflow(key.clone());
//
//     }
//     fn take_smallest_out(&mut self,mut child_index:usize) {
//         //meaning going left
//         let mut current_child =self.children[child_index].clone().unwrap().deref().clone();
//         child_index-=1;
//         let mut before_current =self.children[child_index].clone().unwrap().deref().clone();
//         if before_current.is_current_subtree_filled(){
//             self.take_smallest_out(child_index);// if this is full I need to empty one from here and put it in one before to make room
//             before_current =self.children[child_index].clone().unwrap().deref().clone();
//         }
//         let mut returned_element=EntryElement::empty();
//         if current_child.children.clone().len()==0{
//             returned_element=current_child.elements[0].clone();
//             current_child.elements[0]=EntryElement::empty();
//             current_child.sort_elements();
//             if returned_element.is_irrelevant(){return;}
//             // self.add(returned_element,false);
//         }
//         else {
//             returned_element=current_child.pop_smallest();
//             let ex =self.elements[child_index].clone();
//             self.elements[child_index]=returned_element.clone();
//             before_current.drop(ex, false);
//         }
//
//         self.children[child_index]=Option::from(Box::new(before_current));
//         self.children[child_index+1]=Option::from(Box::new(current_child));
//
//     }
//     fn drop(&mut self, element:EntryElement, take_smaller:bool){//here take smaller is opposite than one from real take
//         if self.is_lpl() && !self.is_current_subtree_filled(){//if its filled I did something wrong
//             self.add(element,false);
//             return;
//         }
//         let last_ind= self.elements.clone().len()-1;
//         let new_next=self.elements[last_ind].clone();
//         self.elements[last_ind]=element.clone();
//
//         if take_smaller{
//             let mut new_child=self.children[0].clone().unwrap().deref().clone();
//             new_child.drop(new_next.clone(),take_smaller);
//             self.children[0]=Option::from(Box::new(new_child.clone()));
//             return;
//         }
//         else {
//             let last_ind= self.children.clone().len()-1;
//             let mut new_child=self.children[last_ind].clone().unwrap().deref().clone();
//             new_child.drop(new_next.clone(),take_smaller);
//             self.children[last_ind]=Option::from(Box::new(new_child.clone()));
//             return;
//         }
//
//     }
//     fn take_largest_out(&self,mut child_index:usize){
//
//     }
//     fn pop_smallest(&mut self)->EntryElement{
//         //self is correct child of root
//         if self.is_lpl(){return self.pop_element(true);}
//         //from each left I will take his left and recursively change it to the one that held lpl with popped element
//         let mut left_child =self.children[0].clone().unwrap();
//         let return_element=left_child.pop_smallest();
//         self.children[0]=Option::from(left_child.clone());
//         return return_element
//     }
//     fn pop_element(&mut self, take_smallest:bool) -> EntryElement { //self must be lpl
//         if !self.is_lpl(){panic!("not lpl")}
//         if take_smallest{
//             let mut new_child =self.children[0].clone().unwrap();
//             let mut new_elements=new_child.elements.clone();
//             let took=new_elements[0].clone();
//             new_elements[0]=EntryElement::empty();
//             new_child.elements=new_elements.clone();
//             new_child.sort_elements();
//             self.children[0]=Option::from(new_child.clone());
//             return took;
//         }
//         //else:
//         let last_ind=self.elements.clone().len();
//         let mut new_child =self.children[last_ind].clone().unwrap();
//         let mut new_elements=new_child.elements.clone();
//         let took=new_elements[last_ind-1].clone();
//         new_elements[last_ind-1]=EntryElement::empty();
//         new_child.elements=new_elements.clone();
//         self.children[last_ind]=Option::from(new_child.clone());
//         return took;
//
//     }
//     fn put_updated_lpl_back(&mut self,lpl:Node){ //self is root
//         if self.get_max_height()==lpl.get_max_height(){
//             self.elements=lpl.elements.clone();
//             self.children=lpl.children.clone();
//             return;
//         } // find lpl
//         let key=lpl.elements[0].clone().extract_number_from_key().unwrap();
//         for i in 0..self.children.clone().len(){
//             // problem here is if I have changed first in elements and if I changed any other but first stayed same so need to check both
//             if self.children[i].clone().unwrap().elements[0].extract_number_from_key().unwrap()>key{
//                 let mut updated_part =self.children[i-1].clone().unwrap();
//                 updated_part.put_updated_lpl_back(lpl.clone());
//                 self.children[i-1]=Option::from(updated_part.clone());
//                 return;
//             }
//             else if self.children[i].clone().unwrap().elements[0].extract_number_from_key().unwrap()==key{
//                 let mut updated_part =self.children[i].clone().unwrap();
//                 updated_part.put_updated_lpl_back(lpl.clone());
//                 self.children[i]=Option::from(updated_part.clone());
//                 return;
//             }
//         }
//         //last
//         let last_ind=self.children.clone().len()-1;
//         if self.children[last_ind.clone()].clone().unwrap().elements[0].extract_number_from_key().unwrap()<key{
//             let mut updated_part =self.children[last_ind.clone()].clone().unwrap();
//             updated_part.put_updated_lpl_back(lpl.clone());
//             self.children[last_ind.clone()]=Option::from(updated_part.clone());
//             return;
//         }
//
//     }
//     fn find_good_lpl(&self,key:i64)->Node{
//         let all =self.get_all_lpl_nodes(vec![]);
//         let last=all.clone().len();
//         for current in 0..last{
//             let referent=all[current].elements[0].clone().extract_number_from_key();
//             if referent.is_none(){return self.clone()}
//             if referent.unwrap()>key{
//                 return all[current-1].clone();
//             }
//         }
//         all[last-1].clone() //not returned before its larger than all
//     }
//     fn choose_direction_left_on_overflow(&self, position_in_all:i32) -> bool {
//         let all_lpl_nodes=self.get_available_lpl_nodes_by_index();
//         let last_ind=all_lpl_nodes.len()-1;
//         if position_in_all==0{return false}
//         if position_in_all==last_ind as i32{return true;}
//
//         let mut next=(position_in_all.clone()+1) as usize;
//         let mut before=(position_in_all.clone()-1) as usize;
//
//         for _ in 0..all_lpl_nodes.clone().len()/2{ // in worst case scenario I am in the middle and last positions is one with empty spot
//             if before==0{return false}
//             if next==last_ind{return true}
//
//             if all_lpl_nodes[next]{return true} //which one out of this two gives me true first
//             if all_lpl_nodes[before]{return false} //it will be the one determine which direction we are going in
//
//             before-=1;
//             next+=1;
//         }
//         panic!("neither?")
//     }
//     pub fn is_lpl(&self)->bool{self.children[0].clone().unwrap().is_leaf()} //self = parent, child = leaf
//     fn handle_overflow_in_lpl(&mut self, element: EntryElement,is_root:bool) {
//         let last_index_in_children =self.elements.clone().iter().len();
//         let key = element.clone().extract_number_from_key().unwrap();
//         let mut element_to_change = EntryElement::empty(); //for now is none
//         for i in 0..= last_index_in_children {
//             let mut child_owner = self.children[i].clone().unwrap().deref().clone();
//             //current element and child by i
//             if i == last_index_in_children {
//                 if child_owner.is_leaf() { //insert in here
//                     let element_in_question = self.elements[i-1].clone().extract_number_from_key().unwrap();
//                     if element_in_question > key {
//                         (_,element_to_change)=self.chose_new_parent(element.clone(),i-1,true);
//                     }
//                     else{
//                         (_,element_to_change)=self.chose_new_parent(element.clone(),i,true);
//                     }
//                 }
//                 else{
//                     child_owner.add(element.clone(),false);
//                     self.children[i] = Option::from(Box::new(child_owner.clone()));
//                     return;
//                 }
//                 break;
//             }
//             let element_in_question = self.elements[i].clone().extract_number_from_key().unwrap();
//             if element_in_question > key { //found position
//                 //either change parent or insert here if its leaf
//                 if child_owner.is_leaf() { //insert in here
//                     //decide on new parent
//                     let direction_left= self.choose_direction_left_in_lpl(i as i32); //choosing to go towards the one we will arrive earlier
//                     (_,element_to_change)=self.chose_new_parent(element.clone(),i,direction_left);
//                     break;
//                 } else { //make it new self
//                     child_owner.add(element.clone(),false);
//                     self.children[i] = Option::from(Box::new(child_owner.clone()));
//                     return; //or break?
//                 }
//             }
//         }
//
//         let sorted=self.sort_all_elements_and_children();
//         self.elements=sorted.elements;
//         self.children=sorted.children;
//         self.add(element_to_change.clone(),is_root);
//     }
//     pub fn chose_new_parent(&mut self,element:EntryElement,mut current_position:usize,go_left:bool )->(EntryElement,EntryElement){
//         let key=element.extract_number_from_key().unwrap();
//         let last_index_in_children=self.elements.clone().len();
//         let mut look_at_element_at_position=last_index_in_children-1;
//         if go_left{
//             //current_position-=1; //if going left I need children[i-1]
//             look_at_element_at_position=0; // if going right I need to compare with last one in elements, if left - first
//         }
//         let mut new_parent =EntryElement::empty();
//         let mut element_to_add_next =EntryElement::empty();
//         //current childs first/last element is greater than key
//         let unwrapped =self.children[current_position].clone().unwrap().elements[look_at_element_at_position].extract_number_from_key().unwrap();
//         if unwrapped > key {
//             if go_left{
//                 new_parent = element.clone();
//             }
//             else{
//                 new_parent = self.children[current_position-1].clone().unwrap().elements[look_at_element_at_position].clone();
//                 let mut child=self.children[current_position-1].clone().unwrap().deref().clone();
//                 element_to_add_next=child.elements[last_index_in_children-1].clone();
//                 child.elements[last_index_in_children-1]=element.clone();
//                 let sorted_child=child.clone().sort_all_elements_and_children();
//                 child.elements=sorted_child.elements;
//                 child.children=sorted_child.children;
//                 self.children[current_position-1]=Option::from(Box::new(child.clone()));
//             }
//         }
//         // key is greater than the current childs first element
//         else {
//             if go_left {
//                 new_parent = self.children[current_position].clone().unwrap().elements[look_at_element_at_position].clone();
//                 let mut child = self.children[current_position].clone().unwrap().deref().clone();
//                 element_to_add_next=self.elements[current_position-1].clone();
//                 self.elements[current_position-1] = new_parent.clone();
//                 child.elements[look_at_element_at_position] = element.clone();
//                 let sorted_child=child.clone().sort_all_elements_and_children();
//                 child.elements=sorted_child.elements;
//                 child.children=sorted_child.children;
//                 self.children[current_position] = Option::from(Box::new(child.clone()));
//                 return (new_parent, element_to_add_next)
//             }
//             else{
//                 new_parent = element.clone();
//             }
//         }
//         if current_position==last_index_in_children{
//             current_position-=1;
//             element_to_add_next = self.elements[current_position].clone();
//         }
//         self.elements[current_position] = new_parent.clone();
//         (new_parent, element_to_add_next)
//
//     }
//     pub fn will_overflow(&mut self, key_attribute:String)->bool{
//         if self.get_max_height()==2{
//             if self.num_of_relative_elements()<self.elements.clone().len() as i32{return false;}
//             if self.is_leaf(){return true; } // reached leaf need new level
//             let key= extract(key_attribute.as_str()).unwrap();
//             for i in 0..self.elements.clone().len(){
//                 if self.elements.clone()[i].extract_number_from_key().unwrap()>key{
//                     if self.children[i].clone().unwrap().is_current_subtree_filled() {return true; }
//                     return false;
//                 }
//             }
//             let last_child = *self.children.clone().last().unwrap().clone().unwrap();
//             if last_child.clone().is_current_subtree_filled() { return true; }
//             else{false;}
//             return false
//         }
//         if self.num_of_relative_elements()<self.elements.clone().len() as i32{return false;}
//         if self.is_leaf(){return true; } // reached leaf need new level
//         let key= extract(key_attribute.as_str()).unwrap();
//         for i in 0..self.elements.clone().len(){
//             if self.elements.clone()[i].extract_number_from_key().unwrap()>key{
//                 if self.children[i].clone().unwrap().will_overflow(key_attribute) {return true; }
//                 return false;
//             }
//         }
//         let last_child = *self.children.clone().last().unwrap().clone().unwrap();
//         if last_child.clone().will_overflow(key_attribute) { return true; }
//         else{false;}
//
//         return false
//     }
//     //function that chooses in overflow weather is closer to go left or right, since it's called on 100% leaves there is never going to be situation where I will reach end and not get at least one empty spot
//     pub fn choose_direction_left_in_lpl(&self, position:i32)->bool{
//         let children=self.children.clone();
//
//         if position==0{return false;}
//         if position==(children.clone().len()-1 )as i32{return true;}
//
//         let mut next = position.clone();
//         let mut before = position.clone();
//
//         loop{
//             next=next+1;
//             before=before-1;
//
//             if next>(children.clone().len()-1 )as i32{return true;} //on this side I will get empty spot
//             if before<0{return false;} //on this side I will get empty spot
//
//             let num_in_next=children.clone()[next as usize].clone().unwrap().num_of_relative_elements();
//             let num_in_before=children.clone()[before as usize].clone().unwrap().num_of_relative_elements();
//
//             if num_in_before==num_in_next{continue;}
//             else if num_in_next<num_in_before{return false;}//right
//             else if num_in_next>num_in_before{return true;}//left
//
//
//
//         }
//
//     }
//     pub fn get_available_lpl_nodes_by_index(&self) -> Vec<bool> {
//         let all_lpl_nodes=self.get_all_lpl_nodes(vec![]);
//         let mut empties:Vec<bool>=vec![]; // it will tell me if I can put element here, or I can't
//         for i in all_lpl_nodes{
//             if i.is_current_subtree_filled(){empties.push(false)}
//             else{empties.push(true);}
//         }
//         empties
//     }
//     pub fn get_all_lpl_nodes(&self, mut all_nodes: Vec<Node>) -> Vec<Node> {
//         let height = self.get_max_height();
//
//         if height == 3 {
//             for child in &self.children {
//                 if child.is_some() {
//                     let child_ref = child.clone().unwrap().deref().clone();
//                     if !all_nodes.contains(&child_ref.clone()) {
//                         all_nodes.push(child_ref.clone());
//                     }
//                 }
//             }
//             return all_nodes;
//         }
//
//         if height <= 2 {
//             if !all_nodes.contains(&self.clone()) {
//                 all_nodes.push(self.clone());
//             }
//             return all_nodes;
//         }
//
//         for child in &self.children {
//             if let Some(ref child_node) = child {
//                 all_nodes = child_node.get_all_lpl_nodes(all_nodes);
//             }
//         }
//
//         all_nodes
//     }
//
// }