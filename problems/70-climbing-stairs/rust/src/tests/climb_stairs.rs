#[cfg(test)]
mod climb_stairs {
    use crate::Solution;

    #[test]
    fn climb_stairs1() {
        let result = Solution::climb_stairs(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn climb_stairs2() {
        let result = Solution::climb_stairs(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn climb_stairs3() {
        let result = Solution::climb_stairs(3);
        assert_eq!(result, 3);
    }

    #[test]
    fn climb_stairs4() {
        let result = Solution::climb_stairs(4);
        assert_eq!(result, 5);
    }

    #[test]
    fn climb_stairs5() {
        let result = Solution::climb_stairs(5);
        assert_eq!(result, 8);
    }

    #[test]
    fn climb_stairs6() {
        let result = Solution::climb_stairs(6);
        assert_eq!(result, 13);
    }

    #[test]
    fn climb_stairs7() {
        let result = Solution::climb_stairs(7);
        assert_eq!(result, 21);
    }

    #[test]
    fn climb_stairs8() {
        let result = Solution::climb_stairs(8);
        assert_eq!(result, 34);
    }

    #[test]
    fn climb_stairs9() {
        let result = Solution::climb_stairs(9);
        assert_eq!(result, 55);
    }

    #[test]
    fn climb_stairs10() {
        let result = Solution::climb_stairs(10);
        assert_eq!(result, 89);
    }
}
