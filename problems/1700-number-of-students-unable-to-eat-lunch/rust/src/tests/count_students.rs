#[cfg(test)]
mod count_students {
    use crate::Solution;

    #[test]
    fn count_students1() {
        let result: i32 = Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]);
        assert_eq!(result, 0);
    }
    #[test]
    fn count_students2() {
        let result: i32 = Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]);
        assert_eq!(result, 3);
    }
}
