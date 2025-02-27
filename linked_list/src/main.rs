use std::mem;

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
fn reorder_list(head: &mut Option<Box<ListNode>>) {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return;
    }
    // Traverse and printf the linked list
    let mut pointer = head.as_ref();
    while let Some(node) = pointer {
        pointer = node.next.as_ref();
    }

    // find the middle of the list
    let mut slow = head.clone();
    let mut fast = head.clone();
    let mut slow_clone = None;
    let mut mid = 0;
    while let (Some(slow_node), Some(fast_node)) = (slow, fast.unwrap().next.as_ref()) {
        slow = slow_node.next;
        fast = fast_node.next.clone();

        if fast.is_none() {
            mid = slow.as_ref().unwrap().val;
            slow_clone = slow.clone();
            break;
        }
    }
    // reverse the other half start from the mid
    let mut second_half = slow_clone.unwrap().next;
    // use prob 21 reverse linked list.
    let mut reversed = reverse_list(second_half);
    let mut first_half = head;
    merge_two_lists_zig_zag(first_half, &mut reversed);
}

pub fn merge_two_lists_zig_zag(
    mut list1: &mut Option<Box<ListNode>>,
    mut list2: &mut Option<Box<ListNode>>,
) -> () {
    if list1.is_none() {
        mem::swap(list1, list2);
        return;
    }
    if list2.is_some() {
        list1 = &mut list1.as_mut().unwrap().next;
        mem::swap(list1, list2);
    }
}

