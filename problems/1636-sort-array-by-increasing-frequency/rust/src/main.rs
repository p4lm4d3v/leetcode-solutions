use std::collections::HashMap;

pub mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();

        for n in nums.iter() {
            *freq.entry(*n).or_insert(0) += 1;
        }

        let mut freqVec = freq.iter().collect::<Vec<_>>();
        freqVec.sort_by(|a, b| {
            if a.1 == b.1 {
                return b.0.cmp(a.0);
            }
            return a.1.cmp(b.1);
        });

        let mut result: Vec<i32> = Vec::new();
        for (k, v) in freqVec {
            for _ in 0..*v {
                result.push(*k);
            }
        }

        result
    }
}
