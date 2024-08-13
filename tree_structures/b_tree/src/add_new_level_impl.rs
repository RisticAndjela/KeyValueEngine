use entry_element::entry_element::{EntryElement};
use crate::node::Node;
// implementation for adding new level
// PROBLEM: i can add up with this logic until third level but with 216th element. I only make spaces for 2 new children, but they have empty elements which are not levelled properly, also mistake in element space in all rows
impl Node{
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
    fn filled_elements(&self)->bool{if self.elements.iter().any(|a| a.is_irrelevant()) {return false} return true}
    pub fn updated_level_space(&mut self) {
        if !self.is_current_subtree_filled(){panic!("not filled tree but asking for new level")}
        if self.can_add_children(){
            self.add_empty_children_leaves();
        }
        else{
            self.add_only_element_space_in_vectors();
        }

    }
    fn can_add_children(&mut self)->bool{
        let height= self.get_max_height();
        let num_of_allowed_due_height= height*2-1;
        num_of_allowed_due_height<self.elements.clone().len() as i32
    }
    fn new_children_to_one_leaf(&mut self){
        let num_elements=self.elements.len() as i32;
        self.children=vec![Some(Box::new(Node::none(num_elements)));(num_elements+1) as usize];
    }
    fn add_only_element_space_in_vectors(&mut self){
        if self.is_leaf(){self.elements.push(EntryElement::empty());self.elements.push(EntryElement::empty());return;}//adding to second level but not appending children just yet
        for i in 0..self.elements.len(){ // number of elements+1 is number of children and I don't want to reach last
            let mut new_child =self.children[i].clone().unwrap();
            let result= new_child.childs_child(false);
            if result.len()!=0{panic!("didnt pop enough elements or took to much")}
            self.children[i]=Option::from(new_child);
        }
        let mut last_child =self.children[self.elements.clone().len()].clone().unwrap();
        let mut result =last_child.childs_child(true);
        if result.len()!=2{panic!("not left two should be left with two")}
        self.children[self.elements.clone().len()]=Option::from(last_child.clone());
        let element1=result.pop().unwrap(); //larger
        let element2=result.pop().unwrap();
        self.elements.push(element2.clone());
        self.elements.push(element1.clone());
        if result.len()!=0{panic!("not left zero should be left with zero")}
        self.children.push(Option::from(Box::new(Node::none(self.elements.clone().len() as i32))));
        self.children.push(Option::from(Box::new(Node::none(self.elements.clone().len() as i32))));
    }
    fn childs_child(&mut self,is_last:bool)->Vec<EntryElement>{
        let mut until=3; // if we are adding only to childrens of original node
        if is_last{until=1;} // if we are adding also to the elements of original node
        let mut result =vec![];
        if self.is_leaf(){ //take it from here and change for empty
            let elements=self.elements.clone();
            for (i,e) in elements.iter().enumerate(){
                if i<until{continue;}
                result.push(e.clone());
                self.elements[i]=EntryElement::empty();
            }
            self.elements.push(EntryElement::empty());
            self.elements.push(EntryElement::empty());
            return result;
        }
        result=self.children.last().unwrap().clone().unwrap().childs_child(is_last);
        let element1=result.pop().unwrap(); //first to pop is largest
        let element2= result.pop().unwrap(); // with each level I add to new elements and 2 new child(children will be added later)
        self.elements.push(element2);
        self.elements.push(element1);
        result
    }
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
}
