#[cfg(test)]
mod contains_consonant {
    use crate::Solution;

    #[test]
    fn contains_consonant1() {
        let result: bool = Solution::contains_consonant(&"234Adas".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn contains_consonant2() {
        let result: bool = Solution::contains_consonant(&"b3".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn contains_consonant3() {
        let result: bool = Solution::contains_consonant(&"a3$e".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn contains_consonant4() {
        let result: bool = Solution::contains_consonant(&"Ii4".to_string());
        assert_eq!(result, false);
    }
}
