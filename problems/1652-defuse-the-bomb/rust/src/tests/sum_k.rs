#[cfg(test)]
mod sum_k {
    use crate::Solution;

    #[test]
    fn sum_k1() {
        let result: i32 = Solution::sum_k(&mut vec![5, 7, 1, 4], 0, 3);
        assert_eq!(result, 12);
    }
    #[test]
    fn sum_k2() {
        let result: i32 = Solution::sum_k(&mut vec![5, 7, 1, 4], 1, 3);
        assert_eq!(result, 10);
    }
    #[test]
    fn sum_k3() {
        let result: i32 = Solution::sum_k(&mut vec![5, 7, 1, 4], 2, 3);
        assert_eq!(result, 16);
    }
    #[test]
    fn sum_k4() {
        let result: i32 = Solution::sum_k(&mut vec![5, 7, 1, 4], 3, 3);
        assert_eq!(result, 13);
    }
    #[test]
    fn sum_k5() {
        let result: i32 = Solution::sum_k(&mut vec![1, 2, 3, 4], 0, 0);
        assert_eq!(result, 0);
    }
    #[test]
    fn sum_k6() {
        let result: i32 = Solution::sum_k(&mut vec![1, 2, 3, 4], 1, 0);
        assert_eq!(result, 0);
    }
    #[test]
    fn sum_k7() {
        let result: i32 = Solution::sum_k(&mut vec![1, 2, 3, 4], 2, 0);
        assert_eq!(result, 0);
    }
    #[test]
    fn sum_k8() {
        let result: i32 = Solution::sum_k(&mut vec![1, 2, 3, 4], 3, 0);
        assert_eq!(result, 0);
    }
    #[test]
    fn sum_k9() {
        let result: i32 = Solution::sum_k(&mut vec![3, 9, 4, 2], 0, 2);
        assert_eq!(result, 13);
    }
    #[test]
    fn sum_k10() {
        let result: i32 = Solution::sum_k(&mut vec![3, 9, 4, 2], 1, 2);
        assert_eq!(result, 6);
    }
    #[test]
    fn sum_k11() {
        let result: i32 = Solution::sum_k(&mut vec![3, 9, 4, 2], 2, 2);
        assert_eq!(result, 5);
    }
    #[test]
    fn sum_k12() {
        let result: i32 = Solution::sum_k(&mut vec![3, 9, 4, 2], 3, 2);
        assert_eq!(result, 12);
    }
}
