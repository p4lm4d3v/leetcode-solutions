#[cfg(test)]
mod maximum_gap {
    use crate::Solution;

    #[test]
    fn maximum_gap1() {
        let result: i32 = Solution::maximum_gap(vec![3, 6, 9, 1]);
        assert_eq!(result, 3);
    }

    #[test]
    fn maximum_gap2() {
        let result: i32 = Solution::maximum_gap(vec![10]);
        assert_eq!(result, 0);
    }
}
