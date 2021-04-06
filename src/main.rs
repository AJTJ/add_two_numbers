// Definition for singly-linked list.
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

// pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
// }

fn push_front(current_list: ListNode, new_value: i32) -> ListNode {
    let new_node = ListNode {
        val: new_value,
        next: Some(Box::new(current_list.clone())),
    };
    new_node
}

fn main() {
    let list_1 = ListNode::new(1);

    let list_2 = push_front(list_1, 2);

    let list_3 = push_front(list_2, 3);

    println!("{:?}", list_3);

    // add_two_numbers(l1, l2)
}
