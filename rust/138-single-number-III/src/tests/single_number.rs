#[cfg(test)]
mod single_number {
    use crate::Solution;

    #[test]
    fn single_number1() {
        let result: Vec<i32> = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        assert_eq!(result, vec![3, 5]);
    }

    #[test]
    fn single_number2() {
        let result: Vec<i32> = Solution::single_number(vec![-1, 0]);
        assert_eq!(result, vec![0, -1]);
    }

    #[test]
    fn single_number3() {
        let result: Vec<i32> = Solution::single_number(vec![0, 1]);
        assert_eq!(result, vec![1, 0]);
    }
}
