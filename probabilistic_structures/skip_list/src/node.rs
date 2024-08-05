use std::fmt;
use std::ops::Deref;
use entry_element::entry_element::EntryElement;
pub struct Node {
    pub value: EntryElement,
    pub next: Option<Box<Node>>,
    pub down: Option<Box<Node>>
}

impl Node {
    pub fn new(value: EntryElement) -> Self {
        Node {
            value,
            next: None,
            down: None,
        }
    }
    pub fn reattach_next(&mut self, attach_node: &mut Node) {
        if self.next != None {
            let next_node = self.next.clone().unwrap().deref().clone();
            attach_node.next = Option::from(Box::new(next_node));
        }
        self.next = Option::from(Box::new(attach_node.clone()));
    }
    pub fn get_all_levels(&mut self) -> Vec<Node> {
        let mut all = vec![self.clone()];
        let mut last_down = self.clone();
        loop {
            if last_down.down == None { break; }
            last_down = last_down.down.unwrap().deref().clone();
            all.push(last_down.clone());
        }
        return all;
    }
    pub fn reconnect(&mut self, new_downs_from: Node) {
        let mut all_new_nexts: Vec<Option<Box<Node>>> = vec![];
        let mut self_next = self.clone();
        let mut new_down_next = Option::from(Box::new(new_downs_from.clone()));
        loop {
            if self_next.value.key == new_down_next.clone().unwrap().value.key {
                //reconnect downs
                self_next.down = new_down_next.clone();
                all_new_nexts.push(Option::from(Box::new(self_next.clone())));
                self_next = self_next.next.unwrap().deref().clone();
                new_down_next=new_down_next.unwrap().next;
            } else {
                //move new down because ideally "down row" has more elements than this one
                if new_down_next.clone().unwrap().next.is_some(){
                    new_down_next = new_down_next.clone().unwrap().next;
                }

            }
            if self_next.next.is_none() { break }
        }
        all_new_nexts.push(Option::from(Box::new(self_next.clone())));
        let latest = reattach_all_nexts_from_vector(all_new_nexts);
        self.next = latest.clone().next;
        self.down = latest.clone().down;
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
            value: self.value.clone(),
            next: self.next.clone(),
            down: self.down.clone()
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node {{ key: {}, NEXT: {:?},DOWN:{:?} }}", self.value.key, self.next,self.down)
    }
}
pub fn print_node(node:Node){
    let mut current = Some(Box::new(node.clone()));
    while let Some(current_node) = current {
        let next_key = current_node.next.as_ref().map_or("none".to_string(), |next| next.value.key.to_string());
        let down_key = current_node.down.as_ref().map_or("none".to_string(), |down| down.value.key.to_string());
        println!("key: {} next key: {} down key: {}", current_node.value.key, next_key, down_key);

        if let Some(down_node) = &current_node.down {
            let down_next_key = down_node.next.as_ref().map_or("none".to_string(), |next| next.value.key.to_string());
            let down_down_key = down_node.down.as_ref().map_or("none".to_string(), |down| down.value.key.to_string());
            println!("\t\t\t\t\t\tDOWN key: {} DOWN next key: {} DOWN down key: {}", down_node.value.key, down_next_key, down_down_key);
        }

        current = current_node.next.clone();
    }
}