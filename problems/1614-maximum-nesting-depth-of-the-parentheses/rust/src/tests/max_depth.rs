#[cfg(test)]
mod max_depth {
    use crate::Solution;

    #[test]
    fn max_depth1() {
        let result: i32 = Solution::max_depth(String::from("(1+(2*3)+((8)/4))+1"));
        assert_eq!(result, 3);
    }
    #[test]
    fn max_depth2() {
        let result: i32 = Solution::max_depth(String::from("(1)+((2))+(((3)))"));
        assert_eq!(result, 3);
    }
    #[test]
    fn max_depth3() {
        let result: i32 = Solution::max_depth(String::from("()(())((()()))"));
        assert_eq!(result, 3);
    }
}
