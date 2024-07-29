pub struct Node {
    pub value: i32,
    pub next:Option<Box<Node>>,
    pub up:Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
            up: None,
        }
    }

    pub fn attach_next(&mut self, node: Node) {
        self.next = Some(Box::new(node));
    }

    pub fn attach_up(&mut self, node: Node) {
        self.up = Some(Box::new(node));
    }

    pub fn get_next(&self) -> &Node {
        self.next.as_deref().unwrap()
    }

    pub fn get_up(&self) -> &Node {
        self.up.as_deref().unwrap()
    }

    pub fn add_ups(&mut self,value:i32,number_of_ups:i32){
        // let mut node=Node::new(value);
        // for mut i in self.iter(){
        //     let mut last_next=i.get_next();
        //     node.attach_next(last_next.clone());
        //     i.attach_up(node.clone());
        // }
    }

}



impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            value: self.value,
            next: self.next.clone(),
            up: self.up.clone(),
        }
    }

}
