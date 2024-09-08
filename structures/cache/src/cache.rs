//Least Recently Used Cache
use entry_element::entry_element::EntryElement;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cache{
    pub capacity:u64,
    pub key_position:HashMap<String,usize>,
    pub elements:Vec<(u64,EntryElement)>
}
impl Cache{
    pub fn new(capacity:u64) -> Cache{
        Cache{capacity,key_position:HashMap::new(), elements:Vec::new()}
    }
    pub fn put(&mut self, element:EntryElement){
        if self.key_position.contains_key(&element.key){
            let position=self.key_position[&element.key];
            self.elements.push((self.elements.len().clone() as u64, element.clone()));
            self.rearrange(position);
        }
        else{
            self.elements.push((self.elements.len().clone() as u64, element.clone()));
            if self.elements.len() as u64 > self.capacity{
                self.key_position.remove(self.elements[0].1.key.as_str());
                self.rearrange(0);
            }
        }
        self.key_position.insert(element.key, self.elements.len()-1);
    }
    fn rearrange(&mut self,delete_at_position:usize){
        self.elements.remove(delete_at_position);
        for i in delete_at_position..self.elements.len(){
            let mut tuple=self.elements[delete_at_position+i].clone();
            tuple.0-=1;
            self.elements[delete_at_position+i]=tuple.clone();
            // println!("tuple {:?}",tuple);
            self.key_position.insert(tuple.1.key, tuple.0 as usize);
        }
    }
    pub fn get(&mut self, key:&String) -> Option<EntryElement>{
        if self.key_position.contains_key(key){
            let position=self.key_position[key];
            let element=self.elements.get(position)?.clone().1;
            self.put(element.clone()); //updated because it was used
            return Some(element.clone())
        }
        None
    }
}