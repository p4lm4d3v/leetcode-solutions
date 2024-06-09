#[cfg(test)]
mod is_only_alphanumeric {
    use crate::Solution;

    #[test]
    fn is_only_alphanumeric1() {
        let result: bool = Solution::is_only_alphanumeric(&"234Adas".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn is_only_alphanumeric2() {
        let result: bool = Solution::is_only_alphanumeric(&"b3".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn is_only_alphanumeric3() {
        let result: bool = Solution::is_only_alphanumeric(&"a3$e".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn is_only_alphanumeric4() {
        let result: bool = Solution::is_only_alphanumeric(&"Ii4".to_string());
        assert_eq!(result, true);
    }
}
