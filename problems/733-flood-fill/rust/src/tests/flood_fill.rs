#[cfg(test)]
mod flood_fill {
    use crate::Solution;

    #[test]
    fn flood_fill1() {
        let result: Vec<Vec<i32>> =
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2);
        assert_eq!(result, vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]);
    }
    #[test]
    fn flood_fill2() {
        let result: Vec<Vec<i32>> =
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0);
        assert_eq!(result, vec![vec![0, 0, 0], vec![0, 0, 0]]);
    }
}
