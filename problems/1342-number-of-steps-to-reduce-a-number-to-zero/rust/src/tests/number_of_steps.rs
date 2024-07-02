#[cfg(test)]
mod number_of_steps {
    use crate::Solution;

    #[test]
    fn number_of_steps1() {
        let result: i32 = Solution::number_of_steps(14);
        assert_eq!(result, 6);
    }
    #[test]
    fn number_of_steps2() {
        let result: i32 = Solution::number_of_steps(8);
        assert_eq!(result, 4);
    }
    #[test]
    fn number_of_steps3() {
        let result: i32 = Solution::number_of_steps(123);
        assert_eq!(result, 12);
    }
}
