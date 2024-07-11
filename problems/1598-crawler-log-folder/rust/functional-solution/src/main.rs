mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let vals: Vec<i32> = logs
            .iter()
            .map(|log| {
                if log == "../" {
                    -1
                } else if log != "./" {
                    1
                } else {
                    0
                }
            })
            .collect();

        0.max(vals.iter().sum())
    }
}
