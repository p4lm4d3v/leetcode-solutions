mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut counter: i32 = 0;
        while num != 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            counter += 1;
        }
        counter
    }
}
