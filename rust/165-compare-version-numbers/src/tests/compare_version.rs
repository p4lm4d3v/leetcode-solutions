#[cfg(test)]
mod compare_version {
    use crate::Solution;

    #[test]
    fn compare_version1() {
        let res = Solution::compare_version("1.01".to_string(), "1.001".to_string());
        assert_eq!(res, 0);
    }
    #[test]
    fn compare_version2() {
        let res = Solution::compare_version("1.0".to_string(), "1.0.0".to_string());
        assert_eq!(res, 0);
    }
    #[test]
    fn compare_version3() {
        let res = Solution::compare_version("0.1".to_string(), "1.1".to_string());
        assert_eq!(res, -1);
    }
    #[test]
    fn compare_version4() {
        let res = Solution::compare_version("5.1".to_string(), "3.2".to_string());
        assert_eq!(res, 1);
    }
}
