use crate::queue::Queue;

use super::types::{BinaryTree, Node};

pub fn breadth_first_search<T: Clone>(tree: &BinaryTree<T>) -> Vec<T> {
    let mut queue = Queue::new();
    let mut path: Vec<T> = vec![];

    push_to_queue(tree.root.clone(), &mut queue);

    while queue.length != 0 {
        if let Some(first_element_in_queue) = queue.dequeue() {
            path.push(first_element_in_queue.value);

            push_to_queue(first_element_in_queue.left_node, &mut queue);
            push_to_queue(first_element_in_queue.right_node, &mut queue);
        }
    }

    path
}

fn push_to_queue<T>(value: Option<T>, queue: &mut Queue<T>) {
    if let Some(x) = value {
        queue.enqueue(x);
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::mock_tree::get_mock_tree;

    use super::breadth_first_search;

    #[test]
    fn test_pre_order() {
        let tree = get_mock_tree();
        let path = breadth_first_search(&tree);
        assert_eq!(path, vec![1, 3, 4, 8, 11, 5, 12, 30, 31, 32, 33, 35]);
    }
}
