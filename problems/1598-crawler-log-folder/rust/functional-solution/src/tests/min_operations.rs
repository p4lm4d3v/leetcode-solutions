#[cfg(test)]
mod min_operations {
    use crate::Solution;

    #[test]
    fn min_operations1() {
        let result: i32 = Solution::min_operations(
            vec!["d1/", "d2/", "../", "d21/", "./"]
                .iter()
                .map(|f| f.to_string())
                .collect(),
        );
        assert_eq!(result, 2);
    }
    #[test]
    fn min_operations2() {
        let result: i32 = Solution::min_operations(
            vec!["d1/", "d2/", "./", "d3/", "../", "d31/"]
                .iter()
                .map(|f| f.to_string())
                .collect(),
        );
        assert_eq!(result, 3);
    }
    #[test]
    fn min_operations3() {
        let result: i32 = Solution::min_operations(
            vec!["d1/", "../", "../", "../"]
                .iter()
                .map(|f| f.to_string())
                .collect(),
        );
        assert_eq!(result, 0);
    }
}
