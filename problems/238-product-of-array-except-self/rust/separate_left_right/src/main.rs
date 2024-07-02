mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; nums.len()];

        for i in 0..nums.len() {
            let mut left: i32 = 1;
            for l in 0..i {
                left *= nums[l];
            }

            let mut right: i32 = 1;
            for r in i + 1..nums.len() {
                right *= nums[r];
            }

            result[i] = left * right;
        }

        result
    }
}
