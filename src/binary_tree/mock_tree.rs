use super::{BinaryTree, Node};

pub fn get_mock_tree() -> BinaryTree<i32> {
    BinaryTree {
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
    }
}
