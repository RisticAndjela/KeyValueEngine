use std::ops::Deref;
use entry_element::entry_element::{EntryElement, extract};
use crate::node::Node;

#[derive(Debug, Clone)]
pub struct BTree{
    pub root:Node,
    pub height:i32 //leaves are in last row->height; starts by 1
}
impl BTree{
    pub fn new()->Self{BTree{root:Node::initialize_new(3),height:1}}
    pub fn add(&mut self, element: EntryElement){
        if element.extract_number_from_key().is_none(){return;}
        let mut root =self.root.clone();
        root.add(element.clone(),true);
        self.root=root;
        self.height= self.root.get_max_height();
    }
    pub fn search(&mut self, key_attribute:String)->EntryElement{
        let key_option= extract(key_attribute.clone().as_str());
        if key_option.is_none(){return EntryElement::empty();}
        let key= key_option.clone().unwrap();
        let mut current= self.root.clone();
        'find:loop{
            for i in 0..current.elements.len() {
                if current.elements[i].extract_number_from_key().is_none(){return EntryElement::empty();}
                if current.elements[i].extract_number_from_key().unwrap() == key.clone(){
                    return current.elements[i].clone();
                }
                if current.elements[i].extract_number_from_key().unwrap() > key.clone() {
                    // this subtree
                    if current.children.is_empty(){return EntryElement::empty();}
                    current=current.children[i].clone().unwrap().deref().clone();
                    continue 'find;
                }
            }
            //check last
            if current.elements.last().unwrap().extract_number_from_key().unwrap() == key.clone(){
                return current.elements.last().unwrap().clone();
            }
            if current.elements.last().unwrap().extract_number_from_key().unwrap() < key.clone(){
                if current.children.is_empty(){return EntryElement::empty();}
                current=current.children.last().unwrap().clone().unwrap().deref().clone();
                continue;
            }
        }
    }
    pub fn get_all_elements_sorted(&self)->Vec<EntryElement>{
        self.root.get_all().into_iter().filter(|a| !a.is_irrelevant()).collect::<Vec<_>>()
    }
    pub fn delete(&mut self, key:String){
        if !self.search(key.clone()).is_irrelevant(){
            let mut root=self.root.clone();
            root.delete(key.as_str());
            self.root=root;
        }
    }
}