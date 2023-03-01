use crate::binary_tree::Link;
use crate::binary_tree::Node;

struct BinaryNode<T> {
    value: T,
    left_child: Node<T>,
    right_child: Node<T>,
}

pub fn pre_order_search<T>(head: Link<T>) -> Vec<T> {
    // if no left and no right return
    // if left node available move recurse left_child
    // if right node available move recurse on right_child
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
    use crate::binary_tree::{BinaryTree, Node};

    use super::pre_order_search;

    #[test]
    fn test_pre_order() {
        let tree = BinaryTree {
            height: 2,
            root: Some(Box::new(Node {
                value: 1,
                left_node: Some(Box::new(Node {
                    value: 3,
                    left_node: Some(Box::new(Node {
                        value: 8,
                        left_node: None,
                        right_node: None,
                    })),
                    right_node: Some(Box::new(Node {
                        value: 11,
                        left_node: None,
                        right_node: None,
                    })),
                })),
                right_node: Some(Box::new(Node {
                    value: 4,
                    left_node: Some(Box::new(Node {
                        value: 5,
                        left_node: Some(Box::new(Node {
                            value: 30,
                            left_node: Some(Box::new(Node {
                                value: 31,
                                left_node: None,
                                right_node: Some(Box::new(Node {
                                    value: 33,
                                    left_node: Some(Box::new(Node {
                                        value: 35,
                                        left_node: None,
                                        right_node: None,
                                    })),
                                    right_node: None,
                                })),
                            })),
                            right_node: Some(Box::new(Node {
                                value: 32,
                                left_node: None,
                                right_node: None,
                            })),
                        })),
                        right_node: None,
                    })),
                    right_node: Some(Box::new(Node {
                        value: 12,
                        left_node: None,
                        right_node: None,
                    })),
                })),
            })),
        };

        let path = pre_order_search(tree.root);
        assert_eq!(path, vec![1, 3, 8, 11, 4, 5, 30, 31, 33, 35, 32, 12]);
    }
}
