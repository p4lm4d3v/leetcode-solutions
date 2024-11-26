mod test;

fn main() {}

struct Solution {}
impl Solution {
    pub fn find_max_consecutive_ones(mut nums: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut counter: i32 = 0;

        nums.push(0);

        for &n in nums.iter() {
            if n == 1 {
                counter += 1;
            } else if n == 0 {
                if counter > max {
                    max = counter;
                }
                counter = 0;
            }
        }

        max
    }
}
