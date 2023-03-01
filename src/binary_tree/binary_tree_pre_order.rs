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

    match head {
        Some(existing_head) => {
            walk(existing_head, &mut path);
        }
        None => (),
    }

    path
}

fn walk<T>(node: Box<Node<T>>, path: &mut Vec<T>) {
    path.push(node.value);
    if node.left_node.is_none() && node.right_node.is_none() {
        return;
    }

    match node.left_node {
        Some(left_node) => {
            walk(left_node, path);
        }
        None => (),
    }

    match node.right_node {
        Some(right_node) => {
            walk(right_node, path);
        }
        None => (),
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
        println!("the pre order traversal for tree is: {:?}", path);
    }
}
