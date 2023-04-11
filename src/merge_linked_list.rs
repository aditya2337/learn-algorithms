#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // if list 1 is None
    // return list 2
    // if list 2 is None
    // return list 1
    match list1 {
        Some(list) => match list2 {
            Some(other_list) => {
                let head = if list.val < other_list.val {
                    list.val
                } else {
                    other_list.val
                };
                let mut new_list = ListNode::new(head);
                let mut list_next = list.next.as_ref();
                let mut other_list_next = other_list.next.as_ref();

                while let (Some(list), Some(other_list)) = (list_next, other_list_next) {
                    if list.val < other_list.val {
                        new_list.next = Some(Box::new(ListNode::new(list.val)));
                        list_next = list.next.as_ref();
                    } else {
                        new_list.next = Some(Box::new(ListNode::new(other_list.val)));
                        other_list_next = other_list.next.as_ref();
                    }
                }

                Some(Box::new(new_list))
            }
            None => Some(list),
        },
        None => list2,
    }
}

fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &num in nums.iter().rev() {
        let node = ListNode {
            val: num,
            next: head,
        };
        head = Some(Box::new(node));
    }
    head
}

#[cfg(test)]
mod tests {
    use crate::merge_linked_list::create_linked_list;
    use super::merge_two_lists;


    #[test]
    fn test_merge() {
        let list_a = create_linked_list(vec![1, 2, 3]);
        let list_b = create_linked_list(vec![4, 5, 6]);

        let meged_linked_list = merge_two_lists(list_a, list_b);

        print!("{:?}", meged_linked_list);
    }
}
