mod tests;

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
