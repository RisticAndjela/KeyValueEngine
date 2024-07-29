use crate::node::Node;

mod node;
mod skip_list;

#[cfg(test)]
mod tests {
    use crate::{ print_all_nexts, print_all_ups};
    use crate::skip_list::SkipList;


    #[test]
    fn skip_list(){
        let mut skip =SkipList::new(1, 10);
        let node=skip.first_node.clone();
        skip.add(2);
        skip.add(2);
        print_all_ups(&node.as_ref());
        print_all_nexts(&node.as_ref(),node.value);

        skip.all_nodes_by_rows();
        print_all_ups(&node.as_ref());
        print_all_nexts(&node.as_ref(),node.value);
       }

}

pub fn print_node_info(node: &Node) {
    let next_value = node.next.as_ref().map_or("None".to_string(), |n| n.value.to_string());
    let up_value = node.up.as_ref().map_or("None".to_string(), |u| u.value.to_string());

    println!("value: {}, next: {}, up: {}", node.value, next_value, up_value);
}

pub fn print_all_ups(node: &Node) {
    let mut current = node;
    let mut count = 0;
    while let Some(up_node) = &current.up {
        println!("Level {}", count);
        print_node_info(&current);
        current = up_node;
        count += 1;
    }
    print_node_info(current);
}

pub fn print_all_nexts(node: &Node, watching_for: i32) {
    println!("\t\t\twatching for row: {}", watching_for);
    let mut current = node;
    while let Some(next_node) = &current.next {
        print_node_info(current);
        current = next_node;
    }
    print_node_info(current);

    if let Some(up_node) = &node.up {
        print_all_nexts(up_node, watching_for + 1);
    }
}
