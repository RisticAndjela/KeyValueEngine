use std::ops::Deref;
use entry_element::entry_element::{EntryElement, extract};
use crate::node::Node;
// implementation for insertion
// lpl = last parent-leaf node in the subtree
 impl Node{
     // IDEA: find the leaf and parent where element should go, this implementation of tree(nodes completion) works based on a fact that only leaves have spots with empty elements
     // when I am adding new level I instantly flush all elements in certain positions so that the leaves have enough space
     // problem here is that I will eventually come to the parts where I cannot borrow anything from anywhere in last parent-leaves tree,
     // so I will be forced to take it from another last parent-leaf, the values can be taken all the way to the roof which makes this complicated while working with large amount of data
     pub fn add(&mut self, element:EntryElement){
         //if I have a filled tree I will need to added it new level
         if self.is_current_subtree_filled(){
            self.updated_level_space(); // it will either add new children if I have enough elements by level equation or add spots for empty elements and rearrange tree
         }
         // now I need to see if we are working with 2 level tree or more complex one
        self.add_with_no_overflow(element.clone());
     }

    fn add_with_no_overflow(&mut self,element:EntryElement){ // self is root -> TRANSFER TO BTREE
        let key= element.extract_number_from_key().unwrap(); // previously checked if there is wrong key structure, so I can safely unwrap
        let height= self.get_max_height();
        if height==1 { // if the node is a leaf, add the element and sort the elements
            for i in 0..self.elements.clone().len(){
                if self.elements[i].clone().is_irrelevant(){
                    self.elements[i]=element.clone();
                    self.sort_elements();
                    return;
                }
            }
        }
        if height==2{ // recursively got to the lpl
            self.add_with_no_overflow_to_lpl(element.clone());
            return;
        }
        for child_index in 0..self.children.clone().len(){
            if self.children[child_index].clone().unwrap().elements[0].extract_number_from_key().unwrap()>key{
                let mut to_replace=self.children[child_index-1].clone().unwrap();
                to_replace.add_with_no_overflow(element.clone());
                self.children[child_index-1]=Option::from(to_replace.clone());
                return;
            }
        }
    }
    fn add_with_no_overflow_to_lpl(&mut self,element:EntryElement){
        let key = element.extract_number_from_key().unwrap();

        let mut position = 0;
        let num_of_elements=self.elements.clone().len();
        'find_position: for i in 0..=num_of_elements {
            if i==num_of_elements{position=i;} // it larger than last number in elements
            let to_compare = self.elements[i].extract_number_from_key();
            if to_compare.is_none(){ panic!("something is wrong line 51"); }//elements are not full add here but take from someone to fulfill
            if to_compare.unwrap() > key { position = i; }
            else{break 'find_position;}
        }

        let mut child = self.children[position].clone().unwrap().deref().clone();
        child.elements[num_of_elements]=element.clone();
        child.sort_elements();

        self.children[position] = Option::from(Box::new(child));
    }
    //this means I will have overflow I cannot handle only with lpl I will need to borrow values from other parents
    fn handle_overflow_on_second_level(&mut self,element:EntryElement){
        // need to find which element i would be dropping and on which side
        // if I go left ... I would take from my child THE smallest element and immediately call on drop element
        // if I go right ... I would take from my child THE largest element and immediately call on drop element
        // rearrange because of missing spot
        // then add this element to the right spot


    }
    fn drop_element_for_one_level(&mut self,element: EntryElement,going_left:bool){// self has element in its elements but in first call it doesn't
        // find where to move
        let mut key=element.extract_number_from_key().unwrap();
        let mut position= 0;
        let height=self.get_max_height();
        if height==1{panic!("fuck")}//if its 1 I did something bad because I reached leaf
        if height==2{
            // I am in zone for lpl
            self.handle_overflow_in_lpl(element);
            return
        }
        for e in self.elements{
            position+=1;
            if e.extract_number_from_key().unwrap()>key{break}
        }
        if key>self.elements.last().unwrap().extract_number_from_key().unwrap(){position+1;}
        let mut child_position=position.clone()+1; // if I go right when I increased position in last line something is wrong
        if going_left{child_position=position.clone()}
        let mut his_correct_child =self.children[child_position].clone().unwrap().deref().clone();
        let element_to_keep_on_dropping=self.elements[position].clone();
        self.elements[position]=element.clone();
        his_correct_child.drop_element_for_one_level(element_to_keep_on_dropping,going_left);
        self.children[child_position]=Some(Box::new(his_correct_child.clone()));
    }
    fn is_lpl(&self)->bool{self.children[0].clone().unwrap().is_leaf()} //self = parent, child = leaf


     // idea is to find the node and element to put, next I need to see which element from all old ones plus new one is the best new parent,
     // when I find new parent I need to delete old one, and add them again looking the direction that the insertion will be fastest,
     // the node I leave as self need to have new value in itself and be sorted
     fn handle_overflow_in_lpl(&mut self, element: EntryElement) {
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
                 if self.children[i].clone().unwrap().is_current_subtree_filled() {return true; }
                 return false;
             }
         }
         let last_child = *self.children.clone().last().unwrap().clone().unwrap();
         if last_child.clone().is_current_subtree_filled() { return true; }
         else{false;}

         return false
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