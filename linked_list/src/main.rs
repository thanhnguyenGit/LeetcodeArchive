fn main() {
    let mut head1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: Some(Box::new(ListNode {
                        val: 12,
                        next: Some(Box::new(ListNode {
                            val: 15,
                            next: None,
                        })),
                    })),
                })),
            })),
        })),
    }));
    let head2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 11,
                        next: None,
                    })),
                })),
            })),
        })),
    }));

    reorder_list(&mut head1);
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

//206. Reverse Linked List - Easy
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pointer = None;
    let mut current_pointer = head;
    while let Some(mut node) = current_pointer {
        println!("the value is {}", node.val);
        let next = node.next.take();
        node.next = pointer.take();
        pointer = Some(node);
        current_pointer = next;
    }
    //DEBUG
    if let Some(result) = &pointer {
        println!("result: {}", result.val)
    }
    pointer
}

//21. Merge two sorted lists - Easy
fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut prehead = ListNode::new(-1);
    let mut curr = &mut prehead;

    let mut head1 = list1;
    let mut head2 = list2;

    while let (Some(node1), Some(node2)) = (&head1, &head2) {
        println!("the node1 value is {}", node1.val);
        println!("the node2 value is {}", node2.val);
        if (node1.val <= node2.val) {
            curr.next = head1.take();
            curr = curr.next.as_mut().unwrap();
            head1 = curr.next.take();
        } else {
            curr.next = head2.take();
            curr = curr.next.as_mut().unwrap();
            head2 = curr.next.take();
        }
    }
    curr.next = head1.or(head2);
    prehead.next
}

//143. Reorder List
fn reorder_list(head : &mut Option<Box<ListNode>>) {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return;
    }
    // Traverse and printf the linked list
    let mut pointer = head.as_ref();
    while let Some(node) = pointer {
        println!("Node val: {}", node.val);
        pointer  = node.next.as_ref();
    }

    // find the middle of the list
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();
    let mut mid = 0;
    while let (Some(slow_node), Some(fast_node)) = (slow,fast.as_ref().unwrap().next.as_ref()) {
        println!("Slow: {}, Fast: {}", slow_node.val, fast_node.val);
        slow = slow_node.next.as_ref();
        fast = fast_node.next.as_ref();

        if fast.is_none() {
            mid = slow.as_ref().unwrap().val;
            println!("Middle {}", mid);
            break
        }
    }
    let second = &slow.unwrap().next;
    println!("second {}", second.as_ref().unwrap().val);
}