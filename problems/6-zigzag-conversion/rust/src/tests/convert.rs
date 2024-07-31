#[cfg(test)]
pub mod convert {
    use crate::Solution;

    #[test]
    fn convert() {
        let result: String = Solution::convert(String::from("PAYPALISHIRING"), 3);
        assert_eq!(result, String::from("PAHNAPLSIIGYIR"));
    }
    #[test]
    fn convert1() {
        let result: String = Solution::convert(String::from("PAYPALISHIRING"), 4);
        assert_eq!(result, String::from("PINALSIGYAHRPI"));
    }
    #[test]
    fn convert2() {
        let result: String = Solution::convert(String::from("A"), 1);
        assert_eq!(result, String::from("A"));
    }
    #[test]
    fn convert3() {
        let result: String = Solution::convert(String::from("ABC"), 2);
        assert_eq!(result, String::from("ACB"));
    }
    #[test]
    fn convert4() {
        let result: String = Solution::convert(String::from("AB"), 2);
        assert_eq!(result, String::from("AB"));
    }
}
