pub fn pre_order_search<T>(head: Link<T>) -> Vec<T> {
    let mut path = vec![];
    walk(head, &mut path);
    path
}

fn walk<T>(node: Link<T>, path: &mut Vec<T>) {
    match node {
        Some(existing_node) => {
            path.push(existing_node.value);

            walk(existing_node.left_node, path);
            walk(existing_node.right_node, path);
        }
        None => {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::{BinaryTree, mock_tree::get_mock_tree};
    use crate::binary_tree::Node;

    use super::pre_order_search;

    #[test]
    fn test_pre_order() {
        let tree = get_mock_tree();
        let path = pre_order_search(tree.root);
        assert_eq!(path, vec![1, 3, 8, 11, 4, 5, 30, 31, 33, 35, 32, 12]);
    }
}
