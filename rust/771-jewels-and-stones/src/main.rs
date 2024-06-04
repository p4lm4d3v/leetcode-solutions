// Explanation in this video
// url: https://www.instagram.com/reel/C7hF_0YqXSA/?igsh=MW9neTVnemRjMDU4dA%3D%3D

use std::collections::HashSet;

fn main() {}

struct Solution {}
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewels_set: HashSet<String> = HashSet::from_iter(jewels.chars().map(|f| f.to_string()));
        let mut num_jewels: i32 = 0;

        for stone in stones.chars() {
            if jewels_set.contains(&stone.to_string()) {
                num_jewels += 1;
            }
        }

        num_jewels
    }
}

#[cfg(test)]
mod num_jewels_in_stones {
    use crate::Solution;

    #[test]
    fn num_jewels_in_stones1() {
        let result: i32 = Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string());
        assert_eq!(result, 3);
    }
    #[test]
    fn num_jewels_in_stones2() {
        let result: i32 = Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string());
        assert_eq!(result, 0);
    }
}
