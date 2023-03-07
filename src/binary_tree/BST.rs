use super::types::{BinaryTree, Link, Node};

#[derive(Debug)]
struct BinarySearchTree {
    tree: BinaryTree<i32>,
}

impl BinarySearchTree {
    pub fn new(tree: BinaryTree<i32>) -> Self {
        Self { tree }
    }

    pub fn insert(&mut self, item: i32) {
        if let Some(root) = self.tree.root.as_mut() {
            walk_and_insert(root, item);
        } else {
            self.tree.root = Some(Box::new(Node::new(item)))
        }
    }

    pub fn delete(mut self, item: i32) -> Self {
        self.tree.root = delete_node(self.tree.root, item);
        self
    }
}

fn min_value_node(node: &Node<i32>) -> &Node<i32> {
    let mut current = &node.left_node;
    while let Some(left_node) = current {
        current = &left_node.left_node;
    }

    match current {
        Some(current) => current,
        None => node
    }
}

fn delete_node(node: Link<i32>, item: i32) -> Link<i32> {
    match node {
        Some(mut node) => {
            if item < node.value {
                node.left_node = delete_node(node.left_node, item);
            } else if item > node.value {
                node.right_node = delete_node(node.right_node, item);
            } else {
                // if node has only one or no Child
                if node.left_node.is_none() {
                    let temp = node.right_node;
                    return temp;
                }

                // If the node has 2 children
                match node.right_node {
                    Some(right_node) => {
                        let temp = min_value_node(&right_node);

                        node.value = temp.value;
                        node.right_node = delete_node(Some(right_node), node.value);
                    }
                    None => {
                        let temp = node.left_node;
                        return temp;
                    }
                }
            }
            Some(node)
        }
        None => node,
    }
}

fn walk_and_insert(node: &mut Node<i32>, item: i32) {
    if item < node.value {
        match node.left_node.as_mut() {
            Some(left_node) => {
                walk_and_insert(left_node, item);
            }
            None => {
                node.left_node = Some(Box::new(Node::new(item)));
            }
        }
    } else {
        match node.right_node.as_mut() {
            Some(right_node) => {
                walk_and_insert(right_node, item);
            }
            None => {
                node.right_node = Some(Box::new(Node::new(item)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::{binary_tree_pre_order::pre_order_traversal, types::BinaryTree};

    use super::BinarySearchTree;

    #[test]
    fn bst() {
        let mut tree = BinarySearchTree::new(BinaryTree::new());
        tree.insert(5);
        let mut pre_order = pre_order_traversal(&tree.tree.root);

        assert_eq!(pre_order, vec![5]);

        tree.insert(2);
        tree.insert(6);
        tree.insert(4);
        tree.insert(1);
        tree.insert(10);
        tree.insert(8);
        tree.insert(11);

        pre_order = pre_order_traversal(&tree.tree.root);
        assert_eq!(pre_order, vec![5, 2, 1, 4, 6, 10, 8, 11]);

        tree = tree.delete(1);
        pre_order = pre_order_traversal(&tree.tree.root);
        assert_eq!(pre_order, vec![5, 2, 1, 4, 6, 10, 8, 11]);
    }
}
