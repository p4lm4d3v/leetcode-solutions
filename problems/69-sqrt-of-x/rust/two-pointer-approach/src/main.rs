mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn my_sqrt(mut x: i32) -> i32 {
        let mut n: usize = 0;
        let mut check: bool = false;
        while let (check, _) = Self::has_natural_root(x as usize) {
            if check { break; }
            x -= 1;
        }
        n as i32
    }
    pub fn has_natural_root(n: usize) -> (bool, usize) {
        if n == 0 || n == 1 {
            return (true, n);
        }

        let mut low: usize = 1;
        let mut high: usize = n;

        while low <= high {
            let mid: usize = low + (high - low) / 2;
            let square: usize = mid * mid;

            if square == n {
                return (true, mid);
            }
            else if square < n {
                low = mid + 1;
            }
            else {
                high = mid - 1;
            }
        }
        
        (false, 0)
    }
}
