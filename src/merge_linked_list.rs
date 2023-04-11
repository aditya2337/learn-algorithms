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
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut new_list: Option<Box<ListNode>> = None;
    let mut tail = &mut new_list;
    while let (Some(list), Some(other_list)) = (list1.as_ref(), list2.as_ref()) {
        println!("List 1: {:?}", list);
        println!("List 2: {:?}", other_list);
        if list.val < other_list.val {
            let node = ListNode::new(list.val);
            *tail = Some(Box::new(node));
            list1 = list.next.clone();
        } else {
            *tail = Some(Box::new(ListNode::new(other_list.val)));
            list2 = other_list.next.clone();
        }
        tail = &mut tail.as_mut().unwrap().next;
    }

    *tail = if list1.is_some() { list1 } else { list2 };
    new_list
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
    use super::merge_two_lists;
    use crate::merge_linked_list::create_linked_list;

    #[test]
    fn test_merge() {
        let list_a = create_linked_list(vec![1, 2, 4]);
        let list_b = create_linked_list(vec![1, 3, 4]);

        let meged_linked_list = merge_two_lists(list_a, list_b);

        print!("{:?}", meged_linked_list);
    }
}
