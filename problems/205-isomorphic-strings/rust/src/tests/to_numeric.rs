#[cfg(test)]
mod to_numeric {
    use crate::Solution;

    #[test]
    fn to_numeric1() {
        let result: String = Solution::to_numeric(String::from("egg"));
        assert_eq!(result, String::from("122"));
    }
    #[test]
    fn to_numeric2() {
        let result: String = Solution::to_numeric(String::from("add"));
        assert_eq!(result, String::from("122"));
    }
    #[test]
    fn to_numeric3() {
        let result: String = Solution::to_numeric(String::from("foo"));
        assert_eq!(result, String::from("122"));
    }
    #[test]
    fn to_numeric4() {
        let result: String = Solution::to_numeric(String::from("bar"));
        assert_eq!(result, String::from("123"));
    }
    #[test]
    fn to_numeric5() {
        let result: String = Solution::to_numeric(String::from("paper"));
        assert_eq!(result, String::from("12134"));
    }
    #[test]
    fn to_numeric6() {
        let result: String = Solution::to_numeric(String::from("title"));
        assert_eq!(result, String::from("12134"));
    }
}
