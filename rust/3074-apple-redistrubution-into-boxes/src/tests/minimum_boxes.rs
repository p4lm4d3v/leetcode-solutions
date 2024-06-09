#[cfg(test)]
mod minimum_boxes {
    use crate::Solution;

    #[test]
    fn minimum_boxes1() {
        let result: i32 = Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]);
        assert_eq!(result, 2);
    }
    #[test]
    fn minimum_boxes2() {
        let result: i32 = Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]);
        assert_eq!(result, 4);
    }
    #[test]
    fn minimum_boxes3() {
        let result: i32 = Solution::minimum_boxes(vec![2, 1, 4, 3], vec![5, 1, 8]);
        assert_eq!(result, 2);
    }
}
