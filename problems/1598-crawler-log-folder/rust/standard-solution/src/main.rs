mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut counter: i32 = 0;

        for log in logs {
            if log == "../" {
                counter = 0.max(counter - 1);
            } else if log != "./" {
                counter += 1;
            }
        }

        counter
    }
}
