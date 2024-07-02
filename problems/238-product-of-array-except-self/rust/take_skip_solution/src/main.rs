mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; nums.len()];

        for i in 1..nums.len() + 1 {
            let left_side: Vec<i32> = nums.iter().take(i - 1).cloned().collect();
            let right_side: Vec<i32> = nums.iter().skip(i).cloned().collect();

            let left: i32 = left_side.iter().fold(1, |a, &b| a * b);
            let right: i32 = right_side.iter().fold(1, |a, &b| a * b);

            result[i - 1] = left * right;
        }

        result
    }
}
