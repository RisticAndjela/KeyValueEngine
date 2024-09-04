use std::ops::Deref;
use entry_element::entry_element::{EntryElement};
use crate::node::Node;

// implementation for adding new level

impl Node{
    // checking whether subtree/tree is fully filled
    pub fn is_current_subtree_filled(&self) ->bool{
        //only way I need new level is if I have unfinished tree
        if !self.filled_elements(){return false} //if it filled is go next
        if self.children.iter().any(|child|!child.clone().unwrap().filled_elements()){return false;}
        for child in self.children.clone(){
            if child.is_none(){return false} //
            if !child.unwrap().is_current_subtree_filled(){return false}
        }
        return true;
    }
    // help function for checking whether are all elements relevant
    fn filled_elements(&self)->bool{if self.elements.iter().any(|a| a.is_irrelevant()) {return false} return true}
    // with this function we will either add new level(of leaves) or spaces in currently existing nodes +2 spaces for elements and +2 children nodes
    pub fn updated_level_space(&mut self) {
        if !self.is_current_subtree_filled(){panic!("not filled tree but asking for new level")}
        if self.can_add_children(){
            self.add_empty_children_leaves();
        }
        else{
            self.add_necessary_spots();
            self.transfer_last_two_spots();
        }

    }
    // function that checks if we can add leaves
    fn can_add_children(&mut self)->bool{
        let height= self.get_max_height();
        let num_of_allowed_due_height= height*2-1;
        num_of_allowed_due_height<self.elements.clone().len() as i32
    }
    // adding leaves
    fn add_empty_children_leaves(&mut self){
        //if it was a leaf before adding empty children and elements
        if self.is_leaf(){
            self.new_children_to_one_leaf();
            return; //leaf has new children
        }
        //if not leaf need to update recursively based on childrens new children
        let old_children=self.children.clone();
        let mut updated_children=vec![];
        for child in old_children{
            let mut child_with_new_level=child.unwrap().clone();
            child_with_new_level.add_empty_children_leaves();
            updated_children.push(Option::from(child_with_new_level.clone()));
        }
        self.children=updated_children;
    }
    // help function that add leaves to only one lpl
    fn new_children_to_one_leaf(&mut self){
        let num_elements=self.elements.len() as i32;
        self.children=vec![Some(Box::new(Node::none(num_elements)));(num_elements+1) as usize];
    }

    // help function for the relocating elements so tree is easier to fill
    fn get_last(&self) ->(Node, usize){
        let mut last=self.children.last().unwrap().clone().unwrap().deref().clone();
        let length= self.elements.clone().len();
        let mut length_of_child=length.clone();
        while last.elements.clone()[0].is_irrelevant(){
            length_of_child-=1;
            last=self.children[length_of_child].clone().unwrap().deref().clone();
        }
        return (last,length_of_child)
    }
    // filling new added spots
    fn transfer_on_two_consecutive_levels(&mut self){
        // transfer between level a and level a-1
        if self.elements[0].is_irrelevant(){return;}
        let (mut last, length_of_child)=self.get_last();
        let length= self.elements.clone().len();

        let mut index1:i32=-2;
        let mut index2:i32=-1;

        for i in last.elements.clone(){
            if i.is_irrelevant(){
                break;
            }
            index1+=1;
            index2+=1;
        }
        let element1=last.elements.clone()[index1 as usize].clone();
        let element2=last.elements.clone()[index2 as usize].clone();
        last.elements[index1 as usize]=EntryElement::empty();
        last.elements[index2 as usize]=EntryElement::empty();
        self.elements[length-1]=element1;
        self.elements[length-2]=element2;
        self.children[length_of_child]=Option::from(Box::new(last.clone()));
        self.sort_elements();
    }
    // it will recursively relocate last two spots for easier approach
    fn transfer_last_two_spots(&mut self){
        let length=self.elements.clone().len();
        if !self.elements.last().unwrap().is_irrelevant(){return;}
        if self.is_lpl(){
            self.transfer_on_two_consecutive_levels();
            return;
        }


        for i in 0..self.children.clone().len(){
            let mut child=self.children[i].clone().unwrap().deref().clone();

            if !child.elements.clone()[0].is_irrelevant(){
                child.transfer_last_two_spots();
            }
            if self.children[i+1].clone().unwrap().elements.clone()[0].is_irrelevant(){
                child.transfer_last_two_spots();
                self.elements[length-1]= child.elements[length-1].clone();
                self.elements[length-2]= child.elements[length-2].clone();
                child.elements[length-1]=EntryElement::empty();
                child.elements[length-2]=EntryElement::empty();
                self.children[i]=Option::from(Box::new(child.clone()));
                child.transfer_last_two_spots();
                self.children[i]=Option::from(Box::new(child.clone()));
                break;
            }
            self.children[i]=Option::from(Box::new(child.clone()));
        }

    }
    // organize the way of filling new spots
    pub fn fill_new_spots(&mut self){
        if !self.is_leaf(){
            self.add_necessary_spots();
        }
        else{
            self.elements.push(EntryElement::empty());
            self.elements.push(EntryElement::empty());
        }
    }
    // function that adds elements AND new children until max level possible
    fn add_necessary_spots(&mut self) {
        self.append_children(self.clone().get_max_height());
        let mut current_level=self.children.clone();
        for mut i in 0..current_level.clone().len() {
            let mut node= current_level[i].clone().unwrap().deref().clone();
            node.fill_new_spots();
            current_level[i]=Option::from(Box::new(node));
        }
        self.children=current_level.clone();
    }
    // will add new node trees to children - it needs to be good height
    pub fn append_children(&mut self, height:i32){
        self.elements.push(EntryElement::empty());
        self.elements.push(EntryElement::empty());
        if self.is_leaf(){
            return;
        }
        let new_child=Option::from(Box::new(Self::new_child_with_height((self.elements.clone().len()-2) as i32, height-1)));
        self.children.push(new_child.clone());
        self.children.push(new_child.clone());
    }
    // help function for "empty" children with certain height
    fn new_child_with_height( num_elements:i32, height:i32) ->Node{
        let mut result =Node::none(num_elements);
        for _ in 1..height{
            let child=Self::new_child_with_height(num_elements, height-1);
            for _ in 0..num_elements+1{
                result.children.push(Option::from(Box::new(child.clone())));}
        }
        return result
    }
}
