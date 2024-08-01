use std::fmt;
use std::ops::Deref;

pub struct Node {
    pub key: i32,
    pub value: i32,
    pub next: Option<Box<Node>>,
    pub down: Option<Box<Node>>
}

impl Node {
    pub fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            next: None,
            down: None,
        }
    }
    pub fn reattach_next(&mut self, attach_node: &mut Node){
        if self.next!=None{
            let next_node=self.next.clone().unwrap().deref().clone();
            attach_node.next=Option::from(Box::new(next_node));
        }
        self.next=Option::from(Box::new(attach_node.clone()));
    }
    pub fn get_all_levels(&mut self)->Vec<Node>{
        let mut all=vec![self.clone()];
        let mut last_down=self.clone();
        loop{
            if last_down.down==None{break;}
            last_down=last_down.down.unwrap().deref().clone();
            all.push(last_down.clone());
        }
        return all;
    }
    pub fn reconnect(&mut self, new_downs_from:Node){
        let mut all_new_nexts:Vec<Option<Box<Node>>>=vec![];
        let mut self_next =self.clone();
        let mut new_down_next = new_downs_from.clone();
        loop {
            if self_next.value== new_down_next.value{
                //reconnect downs
                self_next.down= Option::from(Box::new(new_down_next.clone()));
                all_new_nexts.push(Option::from(Box::new(self_next.clone())));
                self_next=self_next.next.unwrap().deref().clone();
            }
            else {
                //move new down because ideally "down row" has more elements than this one
                new_down_next= new_down_next.next.clone().unwrap().deref().clone();
            }
            if self_next.next.is_none(){break}
        }
        all_new_nexts.push(Option::from(Box::new(self_next.clone())));
        let latest= reattach_all_nexts_from_vector(all_new_nexts);
        self.next= latest.clone().next;
        self.down= latest.clone().down;
    }
}

pub fn reattach_all_nexts_from_vector(mut all_new_nexts: Vec<Option<Box<Node>>>) -> Node{
    if all_new_nexts.len()==1{return all_new_nexts.first().unwrap().clone().unwrap().deref().clone()}
    all_new_nexts.reverse();
    let mut latest=None;
    for before in all_new_nexts{
        let mut working_with =before.unwrap().deref().clone();
        working_with.next=latest;
        latest=Option::from(Box::new(working_with.clone()));
    }
    latest.unwrap().deref().clone()
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            key: self.key,
            value: self.value,
            next: self.next.clone(),
            down: self.down.clone()
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "Node {{ key: {}, value: {}, NEXT: {:?} }}", self.key, self.value, self.next)
        write!(f, "Node {{ key: {}, value: {}, NEXT: {:?},DOWN:{:?} }}", self.key, self.value, self.next,self.down)
    }
}

// pub fn print_all(mut node: Node){
//     for (i,node) in node.get_all_levels().iter().enumerate(){
//         println!("LEVEL {}",i);
//         println!("{:?}",node);
//     }
// }