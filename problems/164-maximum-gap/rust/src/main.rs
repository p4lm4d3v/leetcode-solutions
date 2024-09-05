mod test;

fn main() {}

struct Solution {}
impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        nums.sort();

        let mut max: i32 = 0;
        for i in (1..nums.len()).rev() {
            max = max.max(nums[i] - nums[i - 1]);
        }

        return max;
    }
}
