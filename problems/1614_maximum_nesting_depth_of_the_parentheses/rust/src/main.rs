mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max: i32 = 0;
        let mut depth: i32 = 0;
        for c in s.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => (),
            }
            if depth > max {
                max = depth;
            }
        }
        max
    }
}
