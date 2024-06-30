#[cfg(test)]
mod my_sqrt {
    use crate::Solution;

    #[test]
    fn my_sqrt1() {
        let result: i32 = Solution::my_sqrt(4);
        assert_eq!(result, 2);
    }
    #[test]
    fn my_sqrt2() {
        let result: i32 = Solution::my_sqrt(8);
        assert_eq!(result, 2);
    }
    #[test]
    fn my_sqrt3() {
        let result: i32 = Solution::my_sqrt(2147395599);
        assert_eq!(result, 46339);
    }
    #[test]
    fn my_sqrt4() {
        let result: i32 = Solution::my_sqrt(2147395600);
        assert_eq!(result, 46340);
    }
}
