#[cfg(test)]
mod eval_rpn {
    use crate::Solution;

    fn vec_str_to_string(vec: &[&str]) -> Vec<String> {
        vec.iter().map(|f| f.to_string()).collect()
    }

    #[test]
    fn eval_rpn1() {
        let result: i32 = Solution::eval_rpn(vec_str_to_string(&["2", "1", "+", "3", "*"]));
        assert_eq!(result, 9);
    }

    #[test]
    fn eval_rpn2() {
        let result: i32 = Solution::eval_rpn(vec_str_to_string(&["4", "13", "5", "/", "+"]));
        assert_eq!(result, 6);
    }

    #[test]
    fn eval_rpn3() {
        let result: i32 = Solution::eval_rpn(vec_str_to_string(&[
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]));
        assert_eq!(result, 22);
    }
}
