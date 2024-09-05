#[cfg(test)]
mod longest_consecutive {
    use crate::Solution;

    #[test]
    fn longest_consecutive1() {
        let result: i32 = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(result, 4);
    }

    #[test]
    fn longest_consecutive2() {
        let result: i32 = Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(result, 9);
    }
}
