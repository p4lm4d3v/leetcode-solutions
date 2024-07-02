#[cfg(test)]
mod product_except_self {
    use crate::Solution;

    #[test]
    fn product_except_self1() {
        let result: Vec<i32> = Solution::product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    #[test]
    fn product_except_self2() {
        let result: Vec<i32> = Solution::product_except_self(vec![-1, 1, 0, -3, 3]);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
