use std::ops::Deref;
use entry_element::entry_element::{EntryElement};

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
// print
impl Node {
    pub fn print_tree(&self) {
        self.print_tree_helper(1);
    }

    fn print_tree_helper(&self, level: usize) {
        // Print the current node's elements with the level
        print!("LEVEL {}: ", level);
        self.print_elements();

        // Recursively print the child nodes
        for child in &self.children {
            if let Some(ref child_node) = child {
                child_node.print_tree_helper(level + 1);
            }
        }
    }

    pub fn print_elements(&self) {
        for element in &self.elements {
            print!("{} ", element.key);
        }
        println!(); // Move to the next line after printing elements
    }
}


