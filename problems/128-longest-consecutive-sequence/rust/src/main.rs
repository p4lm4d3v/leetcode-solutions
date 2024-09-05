use std::collections::HashSet;

mod test;

fn main() {}

struct Solution {}
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        let mut longest: i32 = 0;

        for &n in nums.iter() {
            if !nums.contains(&(n - 1)) {
                let mut length: i32 = 1;

                while nums.contains(&(n + length)) {
                    length += 1;
                }

                longest = longest.max(length);
            }
        }

        longest
    }
}
