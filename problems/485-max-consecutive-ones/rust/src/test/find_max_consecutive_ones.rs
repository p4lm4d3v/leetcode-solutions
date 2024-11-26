#[cfg(test)]
mod find_max_consecutive_ones {
    use crate::Solution;

    #[test]
    fn find_max_consecutive_ones1() {
        let result = Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]);
        assert_eq!(result, 3);
    }

    #[test]
    fn find_max_consecutive_ones2() {
        let result = Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]);
        assert_eq!(result, 2);
    }
}
