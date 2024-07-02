mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; nums.len()];

        for i in 0..nums.len() {
            let mut left: i32 = 1;
            let mut right: i32 = 1;

            for j in 0..nums.len() {
                let n: i32 = nums[j];
                if j < i {
                    left *= n;
                } else if j > i {
                    right *= n;
                }
            }

            result[i] = left * right;
        }

        result
    }
}
