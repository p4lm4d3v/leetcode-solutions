#[cfg(test)]
mod fib {
    use crate::Solution;

    #[test]
    fn fib0() {
        let result = Solution::fib(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn fib1() {
        let result = Solution::fib(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn fib2() {
        let result = Solution::fib(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn fib3() {
        let result = Solution::fib(3);
        assert_eq!(result, 3);
    }

    #[test]
    fn fib4() {
        let result = Solution::fib(4);
        assert_eq!(result, 5);
    }

    #[test]
    fn fib5() {
        let result = Solution::fib(5);
        assert_eq!(result, 8);
    }

    #[test]
    fn fib6() {
        let result = Solution::fib(6);
        assert_eq!(result, 13);
    }

    #[test]
    fn fib7() {
        let result = Solution::fib(7);
        assert_eq!(result, 21);
    }

    #[test]
    fn fib8() {
        let result = Solution::fib(8);
        assert_eq!(result, 34);
    }

    #[test]
    fn fib9() {
        let result = Solution::fib(9);
        assert_eq!(result, 55);
    }

    #[test]
    fn fib10() {
        let result = Solution::fib(10);
        assert_eq!(result, 89);
    }
}
