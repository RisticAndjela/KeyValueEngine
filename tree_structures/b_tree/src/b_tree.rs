use std::ops::Deref;
use entry_element::entry_element::{EntryElement, extract};
use crate::node::Node;

pub struct BTree{
    pub root:Node,
    pub height:i32 //leaves are in last row->height; starts by 1
}
impl BTree{
    pub fn new()->Self{BTree{root:Node::initialize_new(2*1-1),height:1}}
    pub fn add(&mut self, element: EntryElement){
        self.root.add(element,true);
        self.height= self.root.get_max_height();
    }
    pub fn search(&mut self, key_attribute:String)->bool{
        let key_option= extract(key_attribute.clone().as_str());
        if key_option.is_none(){panic!("wrong key base")}
        let key= key_option.clone().unwrap();
        let mut current= self.root.clone();
        'find: loop{
            for (i,e) in self.root.elements.clone().iter().enumerate(){
                let e_key= e.extract_number_from_key().unwrap().clone();
                if e_key==key{return true}
                if key<e_key{
                    if current.is_leaf(){
                        break 'find;
                    }current=current.children[i].clone().unwrap().deref().clone();
                    break;
                }
            }


        }

        false
    }
}