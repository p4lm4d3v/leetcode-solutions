// Explanation in this video
// url: https://www.instagram.com/reel/C4i1P5gqai6/?igsh=MW91d2NwbXRidnIxeg%3D%3D

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

#[cfg(test)]
mod minimum_boxes {
    use super::*;

    #[test]
    fn minimum_boxes1() {
        let result: i32 = Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]);
        assert_eq!(result, 2);
    }
    #[test]
    fn minimum_boxes2() {
        let result: i32 = Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]);
        assert_eq!(result, 4);
    }
    #[test]
    fn minimum_boxes3() {
        let result: i32 = Solution::minimum_boxes(vec![2, 1, 4, 3], vec![5, 1, 8]);
        assert_eq!(result, 2);
    }
}
