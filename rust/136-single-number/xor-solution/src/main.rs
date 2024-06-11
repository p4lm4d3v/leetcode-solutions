mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for n in nums.iter() {
            result ^= n;
        }
        return result;
    }
}
