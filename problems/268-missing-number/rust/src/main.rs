mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let expected: i32 = (nums.len() * (nums.len() + 1) / 2) as i32;
        expected - sum
    }
}
