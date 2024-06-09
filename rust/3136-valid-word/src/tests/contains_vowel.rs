#[cfg(test)]
mod contains_vowel {
    use crate::Solution;

    #[test]
    fn contains_vowel1() {
        let result: bool = Solution::contains_vowel(&"234Adas".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn contains_vowel2() {
        let result: bool = Solution::contains_vowel(&"b3".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn contains_vowel3() {
        let result: bool = Solution::contains_vowel(&"a3$e".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn contains_vowel4() {
        let result: bool = Solution::contains_vowel(&"Ii4".to_string());
        assert_eq!(result, true);
    }
}
