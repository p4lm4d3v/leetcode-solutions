mod tests;

pub type LinkedList = Option<Box<ListNode>>;

fn main() {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: LinkedList,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    pub fn add_two_numbers(mut l1: LinkedList, mut l2: LinkedList) -> LinkedList {
        let mut current1: &mut LinkedList = &mut l1;
        let mut current2: &mut LinkedList = &mut l2;

        let mut carry: i32 = 0;
        let mut values: Vec<i32> = Vec::new();

        while current1.is_some() || current2.is_some() {
            let val1: i32 = if let Some(c1) = current1 { c1.val } else { 0 };
            let val2: i32 = if let Some(c2) = current2 { c2.val } else { 0 };
            let mut value: i32 = val1 + val2 + carry;

            carry = value / 10;
            value = value % 10;
            values.push(value);

            if let Some(c1) = current1 {
                current1 = &mut c1.next;
            }
            if let Some(c2) = current2 {
                current2 = &mut c2.next;
            }

            if current1.is_none() && current2.is_none() && carry != 0 {
                values.push(carry);
            }
        }

        Solution::linked_list_from_values(values)
    }
    pub fn linked_list_from_values(values: Vec<i32>) -> LinkedList {
        if values.is_empty() {
            return None;
        }

        let mut result: LinkedList = Some(Box::new(ListNode::new(values[0])));
        let mut current: &mut LinkedList = &mut result;

        for &value in values.iter().skip(1) {
            if let Some(ref mut node) = current {
                node.next = Some(Box::new(ListNode::new(value)));
                current = &mut node.next;
            }
        }

        return result;
    }
}
