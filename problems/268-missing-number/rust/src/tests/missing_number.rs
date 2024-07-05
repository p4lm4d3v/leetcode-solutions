#[cfg(test)]
mod missing_number {
    use crate::Solution;

    #[test]
    fn missing_number1() {
        let result: i32 = Solution::missing_number(vec![3, 0, 1]);
        assert_eq!(result, 2);
    }
    #[test]
    fn missing_number2() {
        let result: i32 = Solution::missing_number(vec![0, 1]);
        assert_eq!(result, 2);
    }
    #[test]
    fn missing_number3() {
        let result: i32 = Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]);
        assert_eq!(result, 8);
    }
}
