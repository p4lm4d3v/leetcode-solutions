#[cfg(test)]
mod linked_list_from_values {
    use crate::{LinkedList, ListNode, Solution};

    #[test]
    fn linked_list_from_values1() {
        let _243: LinkedList = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let result: LinkedList = Solution::linked_list_from_values(vec![2, 4, 3]);
        assert_eq!(result, _243);
    }

    #[test]
    fn linked_list_from_values2() {
        let _564: LinkedList = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let result: LinkedList = Solution::linked_list_from_values(vec![5, 6, 4]);
        assert_eq!(result, _564);
    }
    #[test]
    fn linked_list_from_values3() {
        let _708: LinkedList = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));

        let result: LinkedList = Solution::linked_list_from_values(vec![7, 0, 8]);
        assert_eq!(result, _708);
    }
    #[test]
    fn linked_list_from_values4() {
        let _9999999: LinkedList = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let result: LinkedList = Solution::linked_list_from_values(vec![9, 9, 9, 9, 9, 9, 9]);
        assert_eq!(result, _9999999);
    }
    #[test]
    fn linked_list_from_values5() {
        let _9999: LinkedList = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));

        let result: LinkedList = Solution::linked_list_from_values(vec![9, 9, 9, 9]);
        assert_eq!(result, _9999);
    }
    #[test]
    fn linked_list_from_values6() {
        let _89990001: LinkedList = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let result: LinkedList = Solution::linked_list_from_values(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(result, _89990001);
    }
    #[test]
    fn linked_list_from_values7() {
        let result: LinkedList = Solution::linked_list_from_values(vec![]);
        assert_eq!(result, None);
    }
}
