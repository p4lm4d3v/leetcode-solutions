mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => Solution::fib(n - 2) + Solution::fib(n - 1),
        }
    }
}
