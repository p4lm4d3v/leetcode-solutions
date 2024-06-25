mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            _ => {
                let mut vec: Vec<usize> = vec![1, 2];
                let mut idx: usize = 0;
                for i in 2..n as usize {
                    let two_back: usize = vec[i - 2];
                    let one_back: usize = vec[i - 1];
                    vec.push(two_back + one_back);
                    idx = i;
                }
                vec[idx] as i32
            }
        }
    }
}
