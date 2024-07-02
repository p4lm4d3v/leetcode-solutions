#[cfg(test)]
mod loop_idx {
    use crate::Solution;

    #[test]
    fn loop_idx1() {
        let result: usize = Solution::loop_idx(2, 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn loop_idx2() {
        let result: usize = Solution::loop_idx(4, 5);
        assert_eq!(result, 4);
    }

    #[test]
    fn loop_idx3() {
        let result: usize = Solution::loop_idx(0, 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn loop_idx4() {
        let result: usize = Solution::loop_idx(7, 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn loop_idx5() {
        let result: usize = Solution::loop_idx(5, 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn loop_idx6() {
        let result: usize = Solution::loop_idx(10, 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn loop_idx7() {
        let result: usize = Solution::loop_idx(123456, 5);
        assert_eq!(result, 1);
    }
}
