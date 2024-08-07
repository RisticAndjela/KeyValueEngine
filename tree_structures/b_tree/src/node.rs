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
    fn sort_elements_and_children(&mut self){
        self.sort_elements();
        let mut ordered_children=vec![Option::from(Box::new(Node::none(self.elements.len() as i32)));self.children.clone().len()];
        let mut unordered_children=self.children.clone();
        unordered_children.retain(|x| x.is_some());
        if unordered_children.len()==0{return;}
        for i in 0..self.elements.clone().len(){// index of right one in elements will be
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
    fn sort_elements(&mut self){
        let original_size=self.elements.clone().len();
        self.elements.retain(|x| x.key != "".to_string());
        self.elements.sort_by(|entry1,entry2|  entry1.extract_number_from_key().unwrap().cmp(&entry2.extract_number_from_key().unwrap() ));
        for _ in self.elements.clone().len()..original_size { self.elements.push(EntryElement::empty()); }
    }
    pub fn add_to_k_empty_children_and_elements(&mut self,k:i32){
        for _ in self.elements.len() as i32..k{
            self.elements.push(EntryElement::empty());
        }
        //need to do it separately because children might be 0 in some nodes->leaves
        for _ in self.children.len() as i32..=k{
            self.children.push(Option::from(Box::new(Node::none(k))));

        }
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
    pub fn add(&mut self, element:EntryElement){
        //search -> none
        if element.extract_number_from_key().is_none(){ panic!("wrong key structure")}
        if self.will_overflow(element.key.clone()) {
            //handle problem
            if self.need_new_level() {
                let new_self=self.add_new_level(self.get_max_height() + 1);
                self.elements=new_self.clone().unwrap().elements;
                self.children=new_self.clone().unwrap().children;
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
            let mut element_in_question = self.elements[i].clone().extract_number_from_key().unwrap();
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
        let last_child = *self.children.clone().last().unwrap().deref().clone().unwrap();
        if last_child.clone().are_elements_full() { return true; }
        else{false;}

        return false
    }
    pub fn are_elements_full(&self)->bool{
        let used_spots=self.clone().num_of_relative_elements();
        let max_capacity=self.elements.clone().len();
        used_spots==max_capacity as i32
    }
    pub fn is_leaf(&self)->bool{
        if self.children.clone().len()==0{return true; } // reached leaf
        false
    }
    pub fn need_new_level(&self)->bool{
        if self.elements.iter().any(|a| a.is_irrelevant()) {return false}

        for child in self.children.clone(){
            if child.is_none(){return false}
            if !child.unwrap().need_new_level(){return false}
        }
        return true;
    }
    pub fn add_new_level(&mut self,on_height:i32)-> Option<Box<Node>> {
        self.add_to_k_empty_children_and_elements(on_height * 2 - 1);
        //if it was a leaf before adding empty children and elements
        if self.children[0].clone().unwrap().elements[0].key==""{
            self.new_children_to_one_leaf(on_height);
            self.children=vec![Some(Box::new(Node::none(on_height * 2 - 1))); on_height as usize * 2];//same as up but easier to debug
            return Option::from(Box::new(self.clone())); //leaf has new children
        }
        //if not leaf need to update recursively based on childrens new children
        let mut old_children=self.children.clone();
        let mut updated_children=vec![];
        for child in old_children{
            let mut child_with_new_level=child.unwrap().clone().add_new_level(on_height);
            updated_children.push(child_with_new_level.clone());
        }
        self.children=updated_children;
        self.rearrange();
        Option::from(Box::new(self.clone()))

    }


    fn new_children_to_one_leaf(&mut self, on_height:i32){
        self.children=vec![Some(Box::new(Node::none(on_height * 2 - 1))); on_height as usize * 2];
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
    //function that chooses in overflow weather is closer to go left or right, since it's called on 100% leaves there is never going to be situation where I will reach end and not get at least one empty spot
    pub fn choose_direction_left(&self, position:i32)->bool{
        let mut children=self.children.clone();

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