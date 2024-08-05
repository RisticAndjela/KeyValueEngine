use std::cmp::max;
use std::ops::Deref;
use entry_element::entry_element::{EntryElement, extract};
use crate::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct BTree {
    pub root: Option<Box<Node>>,
    pub num_elements: u32,
    pub max_height: u32,
}

impl BTree {
    pub fn new() -> Self {
        BTree {
            root: None,
            num_elements: 0,
            max_height: 1,
        }
    }

    pub fn add_element(&mut self, new_element: EntryElement) {
        if new_element.extract_number_from_key().is_none() {
            panic!("bad key structure");
        }

        if let Some(root) = self.root.as_mut() {
            if root.find_element(new_element.clone().key).is_none() {
                self.num_elements += 1;
            }

            if root.num_of_elements() == (2 * self.max_height - 1) as i32 {
                let mut new_node = Node::new(self.max_height as usize, false);
                new_node.children[0] = Some(root.clone());
                new_node.split_child(0, self.root.as_mut().unwrap());
                let mut i = 0;
                if new_node.elements[0].extract_number_from_key().unwrap() < extract(new_element.clone().key.as_str()).unwrap() {
                    i += 1;
                }
                new_node.children[i].as_mut().unwrap().insert_element(new_element.clone());
                self.root = Some(Box::new(new_node));
            } else {
                root.insert_element(new_element.clone());
            }

            let mut max_height = 0;
            let mut parent = self.root.as_ref().unwrap().deref().clone();
            loop {
                max_height += 1;
                if parent.is_leaf {
                    break;
                }
                parent = parent.children[0].as_ref().unwrap().deref().clone();
            }
            self.max_height = max(self.max_height, max_height);
        } else {
            let mut new_node = Node::new(self.max_height as usize, true);
            new_node.elements[0] = new_element;
            self.root = Some(Box::new(new_node));
            self.num_elements = 1;
        }
    }

    pub fn find_element(&self, key: String) -> Option<&crate::node::Node> {
        self.root.as_ref()?.find_element(key)
    }

    pub fn delete(&mut self, key: String) {
        // Check if the element exists before proceeding
        if self.find_element(key.clone()).is_none() {
            panic!("cannot delete something that doesn't exist");
        }

        // Stack to keep track of nodes and their parent-child relationships
        let mut all_children: Vec<crate::node::Node> = vec![];

        let mut parent = self.root.clone().unwrap();

        // Traverse the tree to find the node where the element should be marked as deleted
        'find_element: loop {
            let key_number = extract(key.clone().as_str()).unwrap();
            let parent_elements = parent.get_relevant_elements();
            // Check if the current node's elements span the key
            if parent_elements.is_empty() {
                break;
            }

            let first_element_number = parent_elements.first().unwrap().extract_number_from_key().unwrap();
            let last_element_number = parent_elements.last().unwrap().extract_number_from_key().unwrap();

            if key_number < first_element_number {
                // Key should be in a previous node or not present
                if let Some(index) = all_children.iter().position(|n| n == parent.deref()) {
                    if index > 0 {
                        parent = Box::new(all_children[index - 1].clone());
                    }
                }
            } else if key_number > last_element_number {
                // Key should be in a next node
                if let Some(index) = all_children.iter().position(|n| n == parent.deref()) {
                    if index + 1 < all_children.len() {
                        parent = Box::new(all_children[index + 1].clone());
                    }
                }
            } else {
                // Key is in this node or one of its children
                for (i, element) in parent.elements.iter().enumerate() {
                    if element.key == key {
                        parent.elements[i].tombstone = true; // Mark the element as deleted
                        all_children.push(parent.deref().clone());
                        break 'find_element;
                    }
                }

                // If the element was not found in the current node, explore children
                if let Some(index) = parent.children.iter().position(|child| child.as_ref().map_or(false, |c| c.deref() == parent.deref())) {
                    if index < parent.children.len() {
                        parent = parent.children[index].as_ref().unwrap().clone();
                        all_children.push(parent.deref().clone());
                    }
                }
            }
        }

        // Traverse through the stack to update the nodes
        all_children.reverse();
        for i in 1..all_children.len() - 1 {
            // i is next i's children i have to find the index and put it in
            let mut parent = all_children[i].clone();
            let child = all_children[i - 1].clone();
            for ind in 0..parent.children.len() - 1 {
                let see = parent.children[ind].clone().unwrap().deref().clone();
                if see.elements[0].key == child.elements[0].key {
                    parent.children[ind] = Some(Box::new(child.clone()));
                    all_children[i] = parent.clone();
                    break;
                }
            }
        }
        self.root = Some(Box::new(all_children.last().unwrap().clone()));
        self.num_elements -= 1;
    }
}


