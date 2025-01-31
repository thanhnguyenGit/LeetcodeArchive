fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    reverse_list(head);
}
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

//206. Reverse Linked List
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pointer = None;
    let mut current_pointer = head;
    while let Some(mut node) = current_pointer {
        let next = node.next.take();
        node.next = pointer.take();
        pointer = Some(node);
        current_pointer = next;
    }
    if let Some(result) = &pointer {
        println!("result: {}", result.val)
    }
    pointer
}
