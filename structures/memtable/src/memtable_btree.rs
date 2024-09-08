use entry_element::entry_element::{EntryElement as record, EntryElement};
use b_tree::b_tree::BTree;
#[derive(Clone,Debug)]
pub struct MemtableBTree{
    pub data: BTree,
    pub max_size: u32,
    pub current_count:u32,
    pub read_only:bool
}
impl MemtableBTree{
    pub fn new(max_size:u32,read_only:bool)->Self{
        MemtableBTree{data:BTree::new(),max_size,current_count:0,read_only}
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
            self.data.delete(key);
        }
        else{println!("memtable is read only")}
    }
    pub fn flush(&mut self)->Vec<record>{
        let elements: Vec<record> = self.data.get_all_elements_sorted();
        self.kill();
        elements
    }
    pub fn get_value(&mut self, key:String) ->Vec<u8>{
        let element=self.data.search(key.clone());
        if !element.is_irrelevant(){
            return element.value;
        }
        return vec![]
    }
}