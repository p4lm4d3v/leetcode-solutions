#[cfg(test)]
mod move_zeroes {
    use crate::Solution;

    #[test]
    fn move_zeroes1() {
        let mut vec = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 3, 12, 0, 0]);
    }
    #[test]
    fn move_zeroes2() {
        let mut vec = vec![0];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![0]);
    }
    #[test]
    fn move_zeroes3() {
        let mut vec = vec![0, 0, 1];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 0, 0]);
    }
}
