mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<i32> = nums.iter().map(|n| n * n).collect();
        nums.sort();
        nums
    }
}
