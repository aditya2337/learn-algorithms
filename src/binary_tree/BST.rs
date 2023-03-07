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

    pub fn delete(&mut self, item: i32) {}
}

fn walk_and_delete(node: Box<Node<i32>>, item: i32) {
    if item < node.value {
        if let Some(left_node) = node.left_node {
            if left_node.value == item {
                // find the node to replace with
            } else {
                walk_and_delete(left_node, item)
            }
        }
    } else {
        if let Some(right_node) = node.right_node {
            if right_node.value == item {
                // find the node to replace with
            } else {
                walk_and_delete(right_node, item)
            }
        }
    }
}

fn replace_by_node(node: &mut Node<i32>) -> Result<&mut Box<Node<i32>>, &'static str> {
    // if left child
    if let Some(left_node) = node.left_node.as_mut() {
        let mut node_to_replace = left_node;

        // ...if left has a right child, get the right most child
        let mut left_right_node = node_to_replace.right_node.as_mut();
        while let Some(right_node) = left_right_node {
            node_to_replace = right_node;
            left_right_node = node_to_replace.right_node.as_mut();
        }

        return Ok(node_to_replace);
    }
    // else if right child
    else if let Some(right_node) = node.right_node.as_mut() {
        let mut node_to_replace = right_node;

        // ...if right child has a left child, get the left most child
        let mut right_left_node = right_node.left_node.as_mut();
        while let Some(left_node) = right_left_node {
            node_to_replace = left_node;
            right_left_node = left_node.right_node.as_mut();
        }

        return Ok(node_to_replace);
    }
    // else return None
    Err("node not found 😱")
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
    }
}
