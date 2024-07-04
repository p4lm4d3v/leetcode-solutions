#[cfg(test)]
mod add_two_numbers {
    use crate::{LinkedList, ListNode, Solution};

    fn create_list_node(x: ListNode) -> LinkedList {
        Some(Box::new(x))
    }

    #[test]
    fn add_two_numbers1() {
        let _243: LinkedList = Solution::linked_list_from_values(vec![2, 4, 3]);
        let _564: LinkedList = Solution::linked_list_from_values(vec![5, 6, 4]);
        let _708: LinkedList = Solution::linked_list_from_values(vec![7, 0, 8]);

        let result: LinkedList = Solution::add_two_numbers(_243, _564);
        assert_eq!(result, _708);
    }

    #[test]
    fn add_two_numbers2() {
        let _9999999: LinkedList = Solution::linked_list_from_values(vec![9, 9, 9, 9, 9, 9, 9]);
        let _9999: LinkedList = Solution::linked_list_from_values(vec![9, 9, 9, 9]);
        let _89990001: LinkedList = Solution::linked_list_from_values(vec![8, 9, 9, 9, 0, 0, 0, 1]);

        let result: LinkedList = Solution::add_two_numbers(_9999999, _9999);
        assert_eq!(result, _89990001);
    }
}
