mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; nums.len()];

        for i in 0..nums.len() {
            let left_side: &[i32] = &nums[..i];
            let right_side: &[i32] = &nums[i + 1..];

            let left: i32 = left_side.iter().fold(1, |a, &b| a * b);
            let right: i32 = right_side.iter().fold(1, |a, &b| a * b);

            result[i] = left * right;
        }

        result
    }
}
