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

// implementation for adding new level
impl Node{
    // STEP 1:first I need to see do I have more elements than supposed levels if so than I can add empty children to the child,
    // if not I will only add and rearrange existing elements
    // CASE 1: I only need to add empty spots for new elements-> loop from first to last element(so that the last child stays) I will take last leaf
    // and leave him with 3 elements because all the other will be ends of those who need more elements , except from last which would be left with one element because
    // I need two more for original node
    // CASE 2: only add Node::none() to the end of each leaf

    //tested
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
        let mut old_children=self.children.clone();
        let mut updated_children=vec![];
        for child in old_children{
            let mut child_with_new_level=child.unwrap().clone();
            child_with_new_level.add_empty_children_leaves();
            updated_children.push(Option::from(child_with_new_level.clone()));
        }
        self.children=updated_children;
    }
}
