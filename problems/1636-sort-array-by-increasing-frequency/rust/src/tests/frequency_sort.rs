#[cfg(test)]
pub mod frequency_sort {
    use crate::Solution;

    #[test]
    fn frequency_sort1() {
        let result: Vec<i32> = Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]);
        assert_eq!(result, vec![3, 1, 1, 2, 2, 2]);
    }
    #[test]
    fn frequency_sort2() {
        let result: Vec<i32> = Solution::frequency_sort(vec![2, 3, 1, 3, 2]);
        assert_eq!(result, vec![1, 3, 3, 2, 2]);
    }
    #[test]
    fn frequency_sort3() {
        let result: Vec<i32> = Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]);
        assert_eq!(result, vec![5, -1, 4, 4, -6, -6, 1, 1, 1]);
    }
}
