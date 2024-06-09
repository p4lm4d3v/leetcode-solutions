#[cfg(test)]
mod length_valid {
    use crate::Solution;

    #[test]
    fn length_valid1() {
        let result: bool = Solution::length_valid(&"234Adas".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn length_valid2() {
        let result: bool = Solution::length_valid(&"b3".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn length_valid3() {
        let result: bool = Solution::length_valid(&"a3$e".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn length_valid4() {
        let result: bool = Solution::length_valid(&"Ii4".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn length_valid5() {
        let result: bool = Solution::length_valid(&"AhI".to_string());
        assert_eq!(result, true);
    }
}
