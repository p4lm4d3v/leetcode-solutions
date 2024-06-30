#[cfg(test)]
mod is_isomorphic {
    use crate::Solution;

    #[test]
    fn is_isomorphic1() {
        let result: bool = Solution::is_isomorphic(String::from("egg"), String::from("add"));
        assert!(result);
    }
    #[test]
    fn is_isomorphic2() {
        let result: bool = Solution::is_isomorphic(String::from("foo"), String::from("bar"));
        assert!(!result);
    }
    #[test]
    fn is_isomorphic3() {
        let result: bool = Solution::is_isomorphic(String::from("paper"), String::from("title"));
        assert!(result);
    }
}
