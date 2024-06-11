use std::collections::HashMap;

mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums.iter() {
            if let Some(count) = map.get_mut(n) {
                *count += 1;
            } else {
                map.insert(*n, 1);
            }
        }
        for (key, value) in &map {
            if *value == 1 {
                return *key;
            }
        }

        -1
    }
}
