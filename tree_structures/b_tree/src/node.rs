use std::ops::Deref;
use entry_element::entry_element::{EntryElement, extract};

#[derive(Clone, PartialEq, Debug)]
pub struct Node{
    pub elements:Vec<EntryElement>,
    pub children:Vec<Option<Box<Node>>>
}
impl Node{
    //need none node with zero children
    pub fn none(num_elements:i32)->Self{Node{elements:vec![EntryElement::empty(); num_elements as usize],children:vec![]}}
    pub fn initialize_new(num_elements:i32)->Self{
        let mut node=Node{elements:vec![],children:vec![]};
        for _ in 0..num_elements{
            node.elements.push(EntryElement::empty());
            node.children.push(Option::from(Box::new(Node::none(num_elements))));
        }
        node.children.push(Option::from(Box::new(Node::none(num_elements)))); //one more because I need to have elements.len()+1
        node
    }
    pub fn sort_all_elements_and_children(&self) -> Node {
        let mut new_node = self.clone();
        new_node.sort_elements_and_children();
        let mut sorted_children = vec![];

        for child in new_node.children.iter() {// Recursively sort the child node and push to the sorted children
            if child.clone().unwrap().children.len()!=0 { // if it's not leaf
                let sorted_child = child.clone().unwrap().sort_all_elements_and_children();
                sorted_children.push(Some(Box::new(sorted_child)));
            }else{//if it is the leaf it will stay leaf
                let mut sorted_child=child.clone().unwrap();
                sorted_child.sort_elements();
                sorted_children.push(Option::from(sorted_child.clone()));
            }
        }
        new_node.children = sorted_children;

        new_node
    }
    pub fn sort_elements_and_children(&mut self){
        self.sort_elements();
        let mut ordered_children=vec![Option::from(Box::new(Node::none(self.elements.len() as i32)));self.children.clone().len()];
        let mut unordered_children=self.children.clone();
        unordered_children.retain(|x| x.is_some());
        if unordered_children.len()==0{return;}
        for i in 0..self.elements.clone().len(){// index of right one in elements will be
            if self.elements[i].is_irrelevant(){break;}
            let element_in_question=self.elements[i].clone().extract_number_from_key().unwrap();
            for child in unordered_children.clone(){
                let mut look=child.clone().unwrap().clone();
                look.sort_elements();
                if !look.clone().elements[0].is_irrelevant()&& element_in_question>look.elements[0].extract_number_from_key().unwrap(){
                    if ordered_children.contains(&child.clone()){continue;}
                   ordered_children[i]=child.clone();
                }
            }
        }
        //and last one that is bigger than all elements
        let last_index=ordered_children.clone().len() - 1;
        let mut not_used_child=Option::from(Box::new(Node::none(self.elements.clone().len() as i32)));
        for i in unordered_children{
            if !ordered_children.contains(&i){not_used_child=i;break;}
        }
        ordered_children[last_index] = not_used_child;
        self.children=ordered_children.clone();
    }
    pub fn sort_elements(&mut self){
        let original_size=self.elements.clone().len();
        self.elements.retain(|x| x.key != "".to_string());
        self.elements.sort_by(|entry1,entry2|  entry1.extract_number_from_key().unwrap().cmp(&entry2.extract_number_from_key().unwrap() ));
        for _ in self.elements.clone().len()..original_size { self.elements.push(EntryElement::empty()); }
    }
    pub fn num_of_relative_children(&mut self)->i32{
        let mut count=0;
        for i in self.clone().children{
            if i.is_some(){count+=1;}
        }
        count
    }
    pub fn num_of_relative_elements(&mut self)->i32{
        let mut count=0;
        for i in self.clone().elements{
            if !i.is_irrelevant(){count+=1;}
        }
        count
    }
    pub fn is_leaf(&self)->bool{
        if self.children.clone().len()==0{return true; } // reached leaf
        false
    }
    pub fn get_max_height(&self)->i32{
        let mut current=self.clone();
        let mut height=1;
        while current.children.len()!=0{
            current=current.children[0].clone().unwrap().deref().clone();
            height+=1;
        }
        height
    }
}


// implementation for insertion
// lpl = last parent-leaf node in the subtree
impl Node{
    pub fn add(&mut self, element:EntryElement,is_root:bool){
        //if I have a filled tree I will need to added it new level
        if is_root && self.is_current_subtree_filled(){
            self.updated_level_space(); // it will either add new children if I have enough elements by level equation or add spots for empty elements and rearrange tree
        }
        // check if overflow will happen
        if self.will_overflow(element.clone().key.to_string()){
            if self.is_lpl(){
                self.handle_overflow_in_lpl(element.clone(),is_root);
            }
            else{
                self.handle_overflow_on_second_level(element.clone(),is_root);
            }
            return;
        }
        // now I need to see if we are working with 2 level tree or more complex one
        self.add_with_no_overflow(element.clone());
    }
    fn add_with_no_overflow(&mut self,element:EntryElement){ // self is root -> TRANSFER TO BTREE
        let key= element.extract_number_from_key().unwrap(); // previously checked if there is wrong key structure, so I can safely unwrap
        let height= self.get_max_height();
        if height==2 && self.num_of_relative_elements().clone()<3{ // if the node is a leaf, add the element and sort the elements
            for i in 0..self.elements.clone().len(){
                if self.elements[i].clone().is_irrelevant(){
                    self.elements[i]=element.clone();
                    self.sort_elements();
                    return;
                }
            }
        }
        if height==2 { // recursively got to the lpl
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
        //if it never returned it means I need to check the last one as well which held all biggest
        let last_index=self.children.clone().len()-1;
        let mut to_replace=self.children[last_index].clone().unwrap();
        to_replace.add_with_no_overflow(element.clone());
        self.children[last_index]=Option::from(to_replace.clone());
        return;
    }
    fn add_with_no_overflow_to_lpl(&mut self,element:EntryElement){
        let key = element.extract_number_from_key().unwrap();

        let mut position = 0;
        let num_of_elements=self.elements.clone().len();
        'find_position: for i in 0..=num_of_elements {
            if i==num_of_elements{position=i;break 'find_position;} // it larger than last number in elements
            let to_compare = self.elements[i].extract_number_from_key();
            if to_compare.is_none(){ panic!("something is wrong line 51"); }//elements are not full add here but take from someone to fulfill
            if to_compare.unwrap() > key { position = i;break 'find_position; }
        }

        let mut child = self.children[position].clone().unwrap().deref().clone();
        child.elements[num_of_elements-1]=element.clone();
        child.sort_elements();

        self.children[position] = Option::from(Box::new(child));
    }
    //this means I will have overflow I cannot handle only with lpl I will need to borrow values from other parents
    fn handle_overflow_on_second_level(&mut self,element:EntryElement,is_root:bool){
        // need to find which element i would be dropping and on which side
        // if I go left ... I would take from my child THE smallest element and immediately call on drop element
        // if I go right ... I would take from my child THE largest element and immediately call on drop element
        // rearrange because of missing spot
        // then add this element to the right spot
        let key=element.extract_number_from_key().unwrap();
        let mut lpl=self.find_good_lpl(key.clone());
        if lpl.is_current_subtree_filled(){
            let index_of_node=self.get_all_lpl_nodes(vec![]).iter().enumerate().find(|&r| r.1.eq(&lpl.clone()) ).unwrap().0;
            let go_left =self.choose_direction_left_on_overflow(index_of_node as i32);

            if go_left{ self.take_smallest_out(index_of_node);self.add(element,is_root); }
            else{self.take_largest_out(index_of_node); self.add(element,is_root); }
        }
        else{
            if lpl.will_overflow(element.clone().key.to_string()){
                lpl.handle_overflow_in_lpl(element.clone(),is_root);
            }
            else{
                lpl.add_with_no_overflow(element.clone());
            }
            self.put_updated_lpl_back(lpl.clone());
        }
    }
    fn take_smallest_out(&mut self,mut child_index:usize) {
        //meaning going left
        let mut current_child =self.children[child_index].clone().unwrap().deref().clone();
        child_index-=1;
        let mut before_current =self.children[child_index].clone().unwrap().deref().clone();
        if before_current.is_current_subtree_filled(){
            self.take_smallest_out(child_index);// if this is full I need to empty one from here and put it in one before to make room
            before_current =self.children[child_index].clone().unwrap().deref().clone();
        }
        let returned_element=current_child.pop_smallest();
        let ex =self.elements[child_index].clone();
        self.elements[child_index]=returned_element.clone();
        before_current.drop(ex, false);
        self.children[child_index]=Option::from(Box::new(before_current));
        self.children[child_index+1]=Option::from(Box::new(current_child));
    }
    fn drop(&mut self, element:EntryElement, take_smaller:bool){//here take smaller is opposite than one from real take
        if self.is_lpl() && !self.is_current_subtree_filled(){//if its filled i did something wrong
            self.add(element,false);
            return;
        }
        let last_ind= self.elements.clone().len()-1;
        let new_next=self.elements[last_ind].clone();
        self.elements[last_ind]=element.clone();

        if take_smaller{
            let mut new_child=self.children[0].clone().unwrap().deref().clone();
            new_child.drop(new_next.clone(),take_smaller);
            self.children[0]=Option::from(Box::new(new_child.clone()));
            return;
        }
        else {
            let last_ind= self.children.clone().len()-1;
            let mut new_child=self.children[last_ind].clone().unwrap().deref().clone();
            new_child.drop(new_next.clone(),take_smaller);
            self.children[last_ind]=Option::from(Box::new(new_child.clone()));
            return;
        }

    }
    fn take_largest_out(&self,mut child_index:usize){

    }
    fn pop_smallest(&mut self)->EntryElement{
        //self is correct child of root
        if self.is_lpl(){return self.pop_element(true);}
        //from each left I will take his left and recursively change it to the one that held lpl with popped element
        let mut left_child =self.children[0].clone().unwrap();
        let return_element=left_child.pop_smallest();
        self.children[0]=Option::from(left_child.clone());
        return return_element
    }
    fn pop_element(&mut self, take_smallest:bool) -> EntryElement { //self must be lpl
        if !self.is_lpl(){panic!("not lpl")}
        if take_smallest{
            let mut new_child =self.children[0].clone().unwrap();
            let mut new_elements=new_child.elements.clone();
            let took=new_elements[0].clone();
            new_elements[0]=EntryElement::empty();
            new_child.elements=new_elements.clone();
            new_child.sort_elements();
            self.children[0]=Option::from(new_child.clone());
            return took;
        }
        //else:
        let last_ind=self.elements.clone().len();
        let mut new_child =self.children[last_ind].clone().unwrap();
        let mut new_elements=new_child.elements.clone();
        let took=new_elements[last_ind-1].clone();
        new_elements[last_ind-1]=EntryElement::empty();
        new_child.elements=new_elements.clone();
        self.children[last_ind]=Option::from(new_child.clone());
        return took;

    }
    fn put_updated_lpl_back(&mut self,lpl:Node){ //self is root
        if self.get_max_height()==lpl.get_max_height(){
            self.elements=lpl.elements.clone();
            self.children=lpl.children.clone();
            return;
        } // find lpl
        let key=lpl.elements[0].clone().extract_number_from_key().unwrap();
        for i in 0..self.children.clone().len(){
            // problem here is if I have changed first in elements and if I changed any other but first stayed same so need to check both
            if self.children[i].clone().unwrap().elements[0].extract_number_from_key().unwrap()>key{
                let mut updated_part =self.children[i-1].clone().unwrap();
                updated_part.put_updated_lpl_back(lpl.clone());
                self.children[i-1]=Option::from(updated_part.clone());
                return;
            }
            else if self.children[i].clone().unwrap().elements[0].extract_number_from_key().unwrap()==key{
                let mut updated_part =self.children[i].clone().unwrap();
                updated_part.put_updated_lpl_back(lpl.clone());
                self.children[i]=Option::from(updated_part.clone());
                return;
            }
        }
        //last
        let last_ind=self.children.clone().len()-1;
        if self.children[last_ind.clone()].clone().unwrap().elements[0].extract_number_from_key().unwrap()<key{
            let mut updated_part =self.children[last_ind.clone()].clone().unwrap();
            updated_part.put_updated_lpl_back(lpl.clone());
            self.children[last_ind.clone()]=Option::from(updated_part.clone());
            return;
        }

    }
    fn find_good_lpl(&self,key:i64)->Node{
        let all =self.get_all_lpl_nodes(vec![]);
        let last=all.clone().len();
        for current in 0..last{
            if all[current].elements[0].clone().extract_number_from_key().unwrap()>key{
                return all[current-1].clone();
            }
        }
        all[last-1].clone() //not returned before its larger than all
    }
    fn choose_direction_left_on_overflow(&self, position_in_all:i32) -> bool {
        let all_lpl_nodes=self.get_available_lpl_nodes_by_index();
        let last_ind=all_lpl_nodes.len()-1;
        if position_in_all==0{return false}
        if position_in_all==last_ind as i32{return true;}

        let mut next=(position_in_all.clone()+1) as usize;
        let mut before=(position_in_all.clone()-1) as usize;

        for _ in 0..all_lpl_nodes.clone().len()/2{ // in worst case scenario I am in the middle and last positions is one with empty spot
            if before==0{return false}
            if next==last_ind{return true}

            if all_lpl_nodes[next]{return true} //which one out of this two gives me true first
            if all_lpl_nodes[before]{return false} //it will be the one determine which direction we are going in

            before-=1;
            next+=1;
        }
        panic!("neither?")
    }
    fn is_lpl(&self)->bool{self.children[0].clone().unwrap().is_leaf()} //self = parent, child = leaf
    fn handle_overflow_in_lpl(&mut self, element: EntryElement,is_root:bool) {
        let last_index_in_children =self.elements.clone().iter().len();
        let key = element.clone().extract_number_from_key().unwrap();
        let mut element_to_change = EntryElement::empty(); //for now is none
        for i in 0..= last_index_in_children {
            let mut child_owner = self.children[i].clone().unwrap().deref().clone();
            //current element and child by i
            if i == last_index_in_children {
                if child_owner.is_leaf() { //insert in here
                    let element_in_question = self.elements[i-1].clone().extract_number_from_key().unwrap();
                    if element_in_question > key {
                        (_,element_to_change)=self.chose_new_parent(element.clone(),i-1,true);
                    }
                    else{
                        (_,element_to_change)=self.chose_new_parent(element.clone(),i,true);
                    }
                }
                else{
                    child_owner.add(element.clone(),false);
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
                    let direction_left = self.choose_direction_left_in_lpl(i as i32); //choosing to go towards the one we will arrive earlier
                    (_,element_to_change)=self.chose_new_parent(element.clone(),i,direction_left);
                    break;
                } else {//make it new self
                    child_owner.add(element.clone(),false);
                    self.children[i] = Option::from(Box::new(child_owner.clone()));
                    return; //or break?
                }
            }
        }

        let sorted=self.sort_all_elements_and_children();
        self.elements=sorted.elements;
        self.children=sorted.children;
        self.add(element_to_change.clone(),is_root);
    }
    pub fn chose_new_parent(&mut self,element:EntryElement,mut current_position:usize,go_left:bool )->(EntryElement,EntryElement){
        let key=element.extract_number_from_key().unwrap();
        let last_index_in_children=self.elements.clone().len();
        let mut look_at_element_at_position=last_index_in_children-1;
        if go_left{
            //current_position-=1; //if going left I need children[i-1]
            look_at_element_at_position=0; // if going right I need to compare with last one in elements, if left - first
        }
        let mut new_parent =EntryElement::empty();
        let mut element_to_add_next =EntryElement::empty();
        //current childs first/last element is greater than key
        let unwrapped =self.children[current_position].clone().unwrap().elements[look_at_element_at_position].extract_number_from_key().unwrap();
        if unwrapped > key {
            if go_left{
                new_parent = element.clone();
            }
            else{
                new_parent = self.children[current_position-1].clone().unwrap().elements[look_at_element_at_position].clone();
                let mut child=self.children[current_position-1].clone().unwrap().deref().clone();
                element_to_add_next=child.elements[last_index_in_children-1].clone();
                child.elements[last_index_in_children-1]=element.clone();
                let sorted_child=child.clone().sort_all_elements_and_children();
                child.elements=sorted_child.elements;
                child.children=sorted_child.children;
                self.children[current_position-1]=Option::from(Box::new(child.clone()));
            }
        }
        // key is greater than the current childs first element
        else {
            if go_left {
                new_parent = self.children[current_position].clone().unwrap().elements[look_at_element_at_position].clone();
                let mut child = self.children[current_position].clone().unwrap().deref().clone();
                element_to_add_next=self.elements[current_position-1].clone();
                self.elements[current_position-1] = new_parent.clone();
                child.elements[look_at_element_at_position] = element.clone();
                let sorted_child=child.clone().sort_all_elements_and_children();
                child.elements=sorted_child.elements;
                child.children=sorted_child.children;
                self.children[current_position] = Option::from(Box::new(child.clone()));
                return (new_parent, element_to_add_next)
            }
            else{
                new_parent = element.clone();
            }
        }
        if current_position==last_index_in_children{
            current_position-=1;
            element_to_add_next = self.elements[current_position].clone();
        }
        self.elements[current_position] = new_parent.clone();
        (new_parent, element_to_add_next)

    }
    pub fn will_overflow(&mut self, key_attribute:String)->bool{
        if self.get_max_height()==2{
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
        if self.num_of_relative_elements()<self.elements.clone().len() as i32{return false;}
        if self.is_leaf(){return true; } // reached leaf need new level
        let key= extract(key_attribute.as_str()).unwrap();
        for i in 0..self.elements.clone().len(){
            if self.elements.clone()[i].extract_number_from_key().unwrap()>key{
                if self.children[i].clone().unwrap().will_overflow(key_attribute) {return true; }
                return false;
            }
        }
        let last_child = *self.children.clone().last().unwrap().clone().unwrap();
        if last_child.clone().will_overflow(key_attribute) { return true; }
        else{false;}

        return false
    }
    //function that chooses in overflow weather is closer to go left or right, since it's called on 100% leaves there is never going to be situation where I will reach end and not get at least one empty spot
    pub fn choose_direction_left_in_lpl(&self, position:i32)->bool{
        let children=self.children.clone();

        if position==0{return false;}
        if position==(children.clone().len()-1 )as i32{return true;}

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
    pub fn get_available_lpl_nodes_by_index(&self) -> Vec<bool> {
        let all_lpl_nodes=self.get_all_lpl_nodes(vec![]);
        let mut empties:Vec<bool>=vec![]; // it will tell me if I can put element here, or I can't
        for i in all_lpl_nodes{
            if i.is_current_subtree_filled(){empties.push(false)}
            else{empties.push(true);}
        }
        empties
    }
    pub fn get_all_lpl_nodes(&self, mut all_nodes: Vec<Node>) -> Vec<Node> {
        let height = self.get_max_height();

        if height == 3 {
            for child in &self.children {
                if child.is_some() {
                    let child_ref = child.clone().unwrap().deref().clone();
                    if !all_nodes.contains(&child_ref.clone()) {
                        all_nodes.push(child_ref.clone());
                    }
                }
            }
            return all_nodes;
        }

        if height <= 2 {
            if !all_nodes.contains(&self.clone()) {
                all_nodes.push(self.clone());
            }
            return all_nodes;
        }

        for child in &self.children {
            if let Some(ref child_node) = child {
                all_nodes = child_node.get_all_lpl_nodes(all_nodes);
            }
        }

        all_nodes
    }

}