use std::ops::Deref;
use entry_element::entry_element::{EntryElement as record, EntryElement};
use skip_list::skip_list::{SkipList};
#[derive(Clone,Debug)]
pub struct MemtableSkipList{
    pub data: SkipList,
    pub current_count:u32,
    pub max_size: u32,
    pub read_only:bool
}

impl MemtableSkipList{
    pub fn new(max_size:u32,read_only:bool,entry_min:record,entry_max:record)->Self{
        MemtableSkipList{data:SkipList::make_new(entry_min,entry_max),current_count:2,max_size,read_only}
    }
    pub fn kill(&mut self){
        if self.read_only{println!("already inactive");}
        else{self.read_only=true;}
    }
    pub fn add(&mut self,key:String,value:Vec<u8>,timestamp:i64){
        if self.current_count+1>self.max_size{println!("cannot add already full");return;}
        if !self.read_only{
            let element = EntryElement::new(key.clone(), value, timestamp);
            self.data.add(element);
            self.current_count+=1;
        }
        else{println!("memtable is read only")}
        if self.current_count+1>self.max_size{self.kill();return;}

    }
    pub fn add_element(&mut self, element:EntryElement){
        if self.current_count+1>self.max_size{println!("cannot add already full");return;}
        if !self.read_only{ self.data.add(element);self.current_count+=1;}
        else{println!("memtable is read only")}
        if self.current_count+1>self.max_size{self.kill();return;}
    }
    pub fn delete(&mut self,key:String){
        if !self.read_only{
            let (exist,_) = self.data.search(key.clone());
            if exist {
                self.data.remove(key.clone());
                self.current_count-=1;
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
        let mut elements: Vec<record> = vec![];
        let last_node=self.data.get_all_levels().last().unwrap().clone();
        let mut current= last_node.clone();
        loop{
            elements.push(current.value.clone());
            if current.next.is_none(){break;}
            current=current.next.clone().unwrap().deref().clone();
        }
        self.kill();
        elements
    }
    pub fn get_value(&mut self,key:String)->Vec<u8>{
        let (exist,element)=self.data.search(key);
        if exist{
            return element.value;
        }
        return vec![]
    }
}