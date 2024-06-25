#[cfg(test)]
mod single_number {
    use crate::Solution;

    #[test]
    fn single_number() {
        let result: i32 = Solution::single_number(vec![2, 2, 3, 2]);
        assert_eq!(result, 3);
    }

    #[test]
    fn single_number2() {
        let result: i32 = Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]);
        assert_eq!(result, 99);
    }
}
