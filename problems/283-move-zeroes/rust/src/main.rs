mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut count: i32 = 0;
        nums.retain(|n| {
            if *n == 0 {
                count += 1;
                return false;
            }
            return true;
        });
        for _ in 0..count {
            nums.push(0);
        }
    }
}
