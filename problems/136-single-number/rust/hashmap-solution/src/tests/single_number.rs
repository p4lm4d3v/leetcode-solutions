#[cfg(test)]
mod single_number {
    use crate::Solution;

    #[test]
    fn single_number() {
        let result: i32 = Solution::single_number(vec![2, 2, 1]);
        assert_eq!(result, 3);
    }

    #[test]
    fn single_number2() {
        let result: i32 = Solution::single_number(vec![4, 1, 2, 1, 2]);
        assert_eq!(result, 99);
    }

    #[test]
    fn single_number3() {
        let result: i32 = Solution::single_number(vec![1]);
        assert_eq!(result, 99);
    }
}
