fn main() {}

struct Solution {}
impl Solution {
    pub fn asteroids_destroyed(mut mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass: i64 = mass as i64;

        asteroids.sort();
        for a in asteroids {
            let a: i64 = a as i64;
            if mass >= a {
                mass += a;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod asteroids_destroyed {
    use crate::Solution;

    #[test]
    fn asteroids_destroyed1() {
        let result: bool = Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]);
        assert!(result);
    }
    #[test]
    fn asteroids_destroyed2() {
        let result: bool = Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]);
        assert!(!result);
    }

    #[test]
    fn asteroids_destroyed3() {
        let result: bool = Solution::asteroids_destroyed(70, vec![100, 90, 100, 10, 15]);
        assert!(result);
    }
}
