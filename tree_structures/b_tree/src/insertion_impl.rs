use std::ops::Deref;
use entry_element::entry_element::{EntryElement, extract};
use crate::node::Node;


// implementation for insertion
// lpl = last parent-leaf node in the subtree
impl Node{
    pub fn add(&mut self, element:EntryElement,is_root:bool){
        //if I have a filled tree I will need to added it new level
        if is_root && self.is_current_subtree_filled(){
            self.updated_level_space(); // it will either add new children if I have enough elements by level equation or add spots for empty elements and rearrange tree
        }
        // check if overflow will happen
        if self.will_do_overflow(extract(element.clone().key.as_str()).unwrap()){
            self.handle_overflow_in_deep_tree(element.clone());
            return;
        }
        // now I need to see if we are working with 2 level tree or more complex one
        self.add_with_no_overflow(element.clone());
    }
    fn add_with_no_overflow(&mut self,element:EntryElement){ // self is root -> TRANSFER TO BTREE
        let key= element.extract_number_from_key().unwrap(); // previously checked if there is wrong key structure, so I can safely unwrap
        let height= self.get_max_height();
        if height<=2 && self.num_of_relative_elements().clone()<3{ // if the node is a leaf, add the element and sort the elements
            for i in 0..self.elements.clone().len(){
                if self.elements[i].clone().is_irrelevant(){
                    self.elements[i]=element.clone();
                    self.sort_elements();
                    return;
                }
            }
        }
        if height>=2{
            self.add_to_deep_tree(element.clone());
            return;
        }
        if self.is_leaf(){
            let len=self.elements.clone().len()-1;
            self.elements[len]=element.clone();
            self.sort_elements();
            return;
        }
        for child_index in 0..self.children.clone().len(){
            let referent =self.children[child_index].clone().unwrap().elements[0].extract_number_from_key();
            if referent.is_none() { //in children are empty elements
                let mut index=child_index.clone();
                if self.elements[index].extract_number_from_key().unwrap()<key{
                    index+=1;
                }
                let last=self.elements.clone().len();
                let mut new_child =self.children[index].clone().unwrap().deref().clone();
                let mut new_elements=new_child.elements.clone();
                new_elements[last-1]=element.clone();
                new_child.elements=new_elements;
                new_child.sort_elements();
                self.children[index]=Option::from(Box::new(new_child));
                return

            }
            if referent.unwrap()>key{
                let mut to_replace=self.children[child_index-1].clone().unwrap();
                to_replace.add_with_no_overflow(element.clone());
                self.children[child_index-1]=Option::from(to_replace.clone());
                return;
            }
        }
        // if it never returned it means I need to check the last one as well which held all biggest
        let last_index=self.children.clone().len()-1;
        let mut to_replace=self.children[last_index].clone().unwrap();
        to_replace.add_to_deep_tree(element.clone());
        self.children[last_index]=Option::from(to_replace.clone());
        return;
    }
    fn add_to_deep_tree(&mut self,element: EntryElement){
        if self.is_lpl(){
            self.add_with_no_overflow_to_lpl(element.clone());
            return;
        }
        let key = element.extract_number_from_key().unwrap();
        for i in 0..self.elements.clone().len(){
            let see=self.elements[i].clone().extract_number_from_key().unwrap();
            if see>key{
                let mut child=self.children[i].clone().unwrap().deref().clone();
                child.add_to_deep_tree(element.clone());
                self.children[i]=Option::from(Box::new(child.clone()));
                return; //done
            }
        }
        // it is in the biggest chunk
        let length =self.elements.clone().len();
        let mut last=self.children[length].clone().unwrap().deref().clone();
        last.add_to_deep_tree(element.clone());
        self.children[length]=Option::from(Box::new(last.clone()));
    }
    fn add_with_no_overflow_to_lpl(&mut self,element:EntryElement){
        let key = element.extract_number_from_key().unwrap();

        let mut position = 0;
        let num_of_elements=self.elements.clone().len();
        'find_position: for i in 0..=num_of_elements {
            if i==num_of_elements{position=i;break 'find_position;} // it larger than last number in elements
            let to_compare = self.elements[i].extract_number_from_key();
            if to_compare.is_none(){ self.elements[i]=element.clone();return;}//elements are not full add here but take from someone to fulfill
            if to_compare.unwrap() > key { position = i;break 'find_position; }
        }

        let mut child = self.children[position].clone().unwrap().deref().clone();
        child.elements[num_of_elements-1]=element.clone();
        child.sort_elements();

        self.children[position] = Option::from(Box::new(child));
    }
    fn handle_overflow_in_deep_tree(&mut self, element:EntryElement){
        let key= extract(element.clone().key.as_str()).unwrap();
        let position=self.get_position(key.clone());
        let mut current=self.children[position].clone().unwrap().deref().clone();

        let left=self.direction_left_in_deep_tree(position as i32);
        if left{
            // let mut behind=self.children[position-1].clone().unwrap().deref().clone();
            let add_next=current.pop_smallest_element_of_overflow(key.clone());
            println!("to_add {} and to pop {}",element.key,add_next.key);
            current.add_with_no_overflow(element.clone());
            self.children[position]=Option::from(Box::new(current.clone()));
            println!("tree before new parent switch");
            self.print_tree();
            self.make_new_parent_and_drop(add_next.clone(),true);
            println!("tree after new parent switch");
            self.print_tree();

        }else {
            let next=self.children[position+1].clone().unwrap().deref().clone();
            let add_next=current.pop_smallest_element_of_overflow(key.clone()); // pop largest here
            self.children[position+1]=Option::from(Box::new(current.clone()));
            self.add_with_no_overflow(element.clone());
        }

    }
    fn get_position(&self,key:i64)->usize{
        for i in 0..self.elements.clone().len(){
            let represent=self.elements[i].clone().extract_number_from_key();
            if represent.is_none(){return i}
            if represent.unwrap()>key{return i}
        }
        return self.elements.clone().len()
    }
    fn pop_smallest_element_of_overflow(&mut self,key:i64) ->EntryElement{ // self is one level bellow
        if self.is_leaf(){
            let result=self.elements[0].clone();
            self.elements[0]=EntryElement::empty();
            self.sort_elements();
            return result;
        }

        let position= self.get_position(key);
        let mut child=self.children[position].clone().unwrap().deref().clone();
        if child.is_current_subtree_filled(){
            let result= child.pop_smallest_element_of_overflow(key);
            self.children[position]=Option::from(Box::new(child.clone()));
            return result;
        }
        else {
            let new_position=child.get_position(key);
            let new_direction_left =child.direction_left_in_deep_tree(new_position as i32);
            if new_direction_left {
            }
            else{
                // pop largest
            }
            return EntryElement::empty();
        }

    }
    fn direction_left_in_deep_tree(&self, position:i32)->bool{
        let children = self.children.clone();
        let mut next=position;
        let mut before=position;
        loop{
            next+=1;
            before-=1;

            if next > (children.clone().len()-1) as i32 {return true;}
            if before < 0 {return false;}

            if !children[next as usize].clone().unwrap().is_current_subtree_filled(){return false} // have space
            if !children[before as usize].clone().unwrap().is_current_subtree_filled(){return true} // have space
        }
    }
    //this means I will have overflow I cannot handle only with lpl I will need to borrow values from other parents
    pub fn will_do_overflow(&mut self, key:i64) ->bool{
        if self.is_leaf() || self.elements[0].is_irrelevant(){
            // if there is only one empty in elements it will not overflow
            return !self.elements.iter().any(|a| a.is_irrelevant());
        }
        for i in 0..self.elements.clone().len(){
            let relative=self.elements[i].clone().extract_number_from_key();
            if relative.is_none(){return false;}
            if relative.unwrap()>key{return self.children[i].clone().unwrap().will_do_overflow(key.clone())}
        }
        let relative=self.elements.last().unwrap().extract_number_from_key();
        if relative.is_none(){return false;}
        return self.children.last().clone().unwrap().clone().unwrap().deref().clone().will_do_overflow(key.clone());

    }
    pub fn is_lpl(&self)->bool{self.children[0].clone().unwrap().is_leaf()} //self = parent, child = leaf

    //in this function I will find which parent will be replaced based on possible overflow and I will drop all the elements
    pub fn make_new_parent_and_drop(&mut self,new_parent:EntryElement,going_left:bool){
        let key=new_parent.extract_number_from_key().unwrap();
        let mut position=self.get_position(key.clone());
        // parent is self his kids are current and left/right neighbor this is sourly based on the fact that parent is last node to change before drop on a side
        let mut current=self.children[position].clone().unwrap().deref().clone();
        if !current.is_current_subtree_filled() && !current.is_leaf(){
            let new_position=current.get_position(key.clone());
            let new_direction_left = current.direction_left_in_deep_tree(new_position as i32);
            current.make_new_parent_and_drop(new_parent.clone(), new_direction_left);
            self.children[position]=Option::from(Box::new(current.clone()));
            return;
        }
        if going_left{
            let mut left_neighbor=self.children[position-1].clone().unwrap().deref().clone();
            if left_neighbor.is_current_subtree_filled(){
                let to_pull_down=left_neighbor.pop_smallest_element_of_overflow(key.clone());
                self.children[position-1]=Option::from(Box::new(left_neighbor.clone()));
                self.make_new_parent_and_drop(to_pull_down,true);
                left_neighbor=self.children[position-1].clone().unwrap().deref().clone();
            }
            let popped=self.elements[position-1].clone();
            self.elements[position-1]=new_parent.clone();
            left_neighbor.add(popped,false);

            self.children[position-1]=Option::from(Box::new(left_neighbor.clone()));
        }
        else{

        }
    }
}