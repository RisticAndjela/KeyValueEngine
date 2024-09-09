use std::collections::HashMap;
use entry_element::entry_element::{EntryElement as record, EntryElement};

#[derive(Clone,Debug)]
pub struct MemtableHashMap{
    pub data: HashMap<String, record>,
    pub max_size: u32,
    pub current_count:u32,
    pub read_only:bool
}
impl MemtableHashMap{
    pub fn new(max_size:u32,read_only:bool)->Self{
        MemtableHashMap{data:HashMap::new(),max_size,current_count:0,read_only}
    }
    pub fn kill(&mut self){
        if self.read_only{println!("already inactive");}
        else{self.read_only=true;}
        // println!("KILLED")
    }
    pub fn add(&mut self,key:String,value:Vec<u8>,timestamp:i64){
        if self.current_count+1>self.max_size{println!("cannot add already full");return;}
        if !self.read_only{
            let element = EntryElement::new(key.clone(), value, timestamp);
            self.data.insert(key.clone(), element);
            self.current_count+=1;
        }
        else{println!("memtable is read only")}
        if self.current_count+1>self.max_size{self.kill();return;}

    }
    pub fn add_element(&mut self, element:EntryElement){
        if self.current_count+1>self.max_size{println!("cannot add already full");return;}
        if !self.read_only{ self.data.insert(element.key.clone(),element);self.current_count+=1;}
        else{println!("memtable is read only")}
        if self.current_count+1>self.max_size{self.kill();return;}

    }
    pub fn delete(&mut self,key:String){
        if !self.read_only{
            let element = self.data.get(&key.clone());
            if element.is_some() {
                let mut e = element.unwrap().clone();
                e.delete();
                self.current_count-=1;
                self.data.insert(key.clone(), e);
            } else {
                // when deleting un-existing element, we will make new element and set its tombstone to true
                let mut new_element = EntryElement::empty(); // key:"",value:[],tombstone:true, timestamp:0
                new_element.key = key.clone();
                self.add_element(new_element);
            }
        }
        else{println!("memtable is read only")}
    }
    pub fn flush(&mut self)->Vec<record>{
        let mut elements: Vec<record> = self.data.values().cloned().collect();
        elements.sort_by(|a, b| a.extract_number_from_key().unwrap().cmp(&b.extract_number_from_key().unwrap()));
        self.kill();
        elements
    }
    pub fn get_value(&self,key:String)->Vec<u8>{
        let element=self.data.get(&key.clone());
        if element.is_some(){
            if element.unwrap().tombstone==false{return element.unwrap().clone().value;}
        }
        return vec![]
    }
}