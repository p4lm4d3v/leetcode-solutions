fn main() {}

struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut count: i32 = 0;
        nums.retain(|n| {
            if *n == 0 {
                count += 1;
                return false;
            }
            return true;
        });
        for _ in 0..count {
            nums.push(0);
        }
    }
}

#[cfg(test)]
mod move_zeroes {
    use super::*;

    #[test]
    fn move_zeroes1() {
        let mut vec = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 3, 12, 0, 0]);
    }
    #[test]
    fn move_zeroes2() {
        let mut vec = vec![0];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![0]);
    }
    #[test]
    fn move_zeroes3() {
        let mut vec = vec![0, 0, 1];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 0, 0]);
    }
}
