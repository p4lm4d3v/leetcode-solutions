#[cfg(test)]
mod sorted_squares {
    use crate::Solution;

    #[test]
    fn sorted_squares1() {
        let result = Solution::sorted_squares(vec![-4, -1, 0, 3, 10]);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn sorted_squares2() {
        let result = Solution::sorted_squares(vec![-7, -3, 2, 3, 11]);
        assert_eq!(result, vec![4, 9, 9, 49, 121]);
    }
}
