#[cfg(test)]
mod length_of_last_word {
    use crate::Solution;

    #[test]
    fn length_of_last_word1() {
        let result: i32 = Solution::length_of_last_word(String::from("Hello World"));
        assert_eq!(result, 5);
    }
    #[test]
    fn lengh_of_last_word2() {
        let result: i32 =
            Solution::length_of_last_word(String::from("   fly me   to   the moon  "));
        assert_eq!(result, 4);
    }
    #[test]
    fn length_of_last_word3() {
        let result: i32 = Solution::length_of_last_word(String::from("luffy is still joyboy"));
        assert_eq!(result, 6);
    }
}
