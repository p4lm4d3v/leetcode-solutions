#[cfg(test)]
mod is_valid {
    use crate::Solution;

    #[test]
    fn is_valid1() {
        let result: bool = Solution::is_valid("234Adas".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn is_valid2() {
        let result: bool = Solution::is_valid("b3".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn is_valid3() {
        let result: bool = Solution::is_valid("a3$e".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn is_valid4() {
        let result: bool = Solution::is_valid("Ii4".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn is_valid5() {
        let result: bool = Solution::is_valid("AhI".to_string());
        assert_eq!(result, true);
    }
}
