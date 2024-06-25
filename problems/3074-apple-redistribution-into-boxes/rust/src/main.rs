// Explanation in this video
// url: https://www.instagram.com/reel/C4i1P5gqai6/?igsh=MW91d2NwbXRidnIxeg%3D%3D

mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn minimum_boxes(apples: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_by(|a, b| b.cmp(a));
        let mut total_apples = apples.into_iter().reduce(|a, b| a + b).unwrap();
        let mut used_boxes: i32 = 0;

        while total_apples > 0 {
            let c = *capacity.get(0).unwrap();
            capacity = Solution::remove_first_element(&capacity);
            total_apples -= c;
            used_boxes += 1;
        }
        used_boxes
    }
    fn remove_first_element(vec: &Vec<i32>) -> Vec<i32> {
        vec.iter().skip(1).map(|f| *f).collect()
    }
}
