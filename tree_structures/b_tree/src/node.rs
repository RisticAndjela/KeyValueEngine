use entry_element::entry_element::{EntryElement, extract};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub elements: Vec<EntryElement>,
    pub children: Vec<Option<Box<Node>>>,
    pub height: u32,
    pub is_leaf: bool,
}

impl Node {
    pub fn new(height: usize, is_leaf: bool) -> Self {
        Node {
            elements: vec![EntryElement::empty(); 2 * height - 1],
            children: vec![None; 2 * height],
            height: height as u32,
            is_leaf,
        }
    }

    pub fn insert_element(&mut self, new_element: EntryElement) {
        if new_element.extract_number_from_key().is_none() {
            panic!("wrong key structure");
        }
        let mut i = self.num_of_elements() as usize;

        if self.is_leaf {
            self.elements.push(EntryElement::empty()); // Ensure there is space for the new element
            while i > 0 && self.elements[i - 1].extract_number_from_key().unwrap() > new_element.extract_number_from_key().unwrap() {
                self.elements[i] = self.elements[i - 1].clone();
                i -= 1;
            }
            self.elements[i] = new_element;
        } else {
            while i > 0 && self.elements[i - 1].extract_number_from_key().unwrap() > new_element.extract_number_from_key().unwrap() {
                i -= 1;
            }

            let mut child = self.children[i].clone().unwrap();
            if child.clone().num_of_elements() == (2 * self.height - 1) as i32 {
                self.split_child(i,&mut child);
                if self.elements[i].extract_number_from_key().unwrap() < new_element.extract_number_from_key().unwrap() {
                    i += 1;
                }
            }
            self.children[i].as_mut().unwrap().insert_element(new_element);
        }
    }

    pub fn split_child(&mut self, index: usize, y: &mut Node) {
        let mut z = Node::new(y.height as usize, y.is_leaf);
        let t = self.height as usize;

        for j in 0..t - 1 {
            z.elements[j] = y.elements[j + t].clone();
        }

        if !y.is_leaf {
            for j in 0..t {
                z.children[j] = y.children[j + t].take();
            }
        }

        y.elements.truncate(t - 1); // Correctly truncate the elements in y

        for j in (index + 1..=self.num_of_elements() as usize).rev() {
            self.children[j + 1] = self.children[j].take();
        }

        self.children[index + 1] = Some(Box::new(z));

        for j in (index..self.num_of_elements() as usize).rev() {
            self.elements[j + 1] = self.elements[j].clone();
        }

        self.elements[index] = y.elements[t - 1].clone();
    }

    pub fn num_of_elements(&self) -> i32 {
        self.elements.iter().take_while(|e| !e.is_irrelevant()).count() as i32
    }

    pub fn find_element(&self, key: String) -> Option<&Node> {
        if extract(key.as_str()).is_none() {
            panic!("bad key structure");
        }
        let mut i = 0;
        while i < self.num_of_elements() as usize && extract(key.as_str()).unwrap() > self.elements[i].extract_number_from_key().unwrap() {
            i += 1;
        }

        if i < self.num_of_elements() as usize && self.elements[i].extract_number_from_key().unwrap() == extract(key.as_str()).unwrap() {
            return Some(self);
        }

        if self.is_leaf {
            return None;
        }

        self.children[i].as_ref()?.find_element(key)
    }

    pub fn get_relevant_elements(&self) -> Vec<EntryElement> {
        self.elements.iter().filter(|e| !e.is_irrelevant()).cloned().collect()
    }
}