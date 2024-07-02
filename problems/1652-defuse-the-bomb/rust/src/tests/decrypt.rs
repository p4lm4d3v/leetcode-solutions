#[cfg(test)]
mod decrypt {
    use crate::Solution;

    #[test]
    fn decrypt1() {
        let result: Vec<i32> = Solution::decrypt(vec![5, 7, 1, 4], 3);
        assert_eq!(result, vec![12, 10, 16, 13]);
    }
    #[test]
    fn decrypt2() {
        let result: Vec<i32> = Solution::decrypt(vec![1, 2, 3, 4], 0);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }
    #[test]
    fn decrypt3() {
        let result: Vec<i32> = Solution::decrypt(vec![2, 4, 9, 3], -2);
        assert_eq!(result, vec![12, 5, 6, 13]);
    }
}
