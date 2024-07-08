mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut i: i32 = 0;

        while i * i < x {
            if (i + 1) * (i + 1) > x {
                break;
            }
            i += 1;
        }

        i
    }
}
