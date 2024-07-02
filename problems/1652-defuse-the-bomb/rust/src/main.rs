mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn decrypt(mut code: Vec<i32>, mut k: i32) -> Vec<i32> {
        let len: usize = code.clone().len();
        let mut result: Vec<i32> = vec![0; len];

        if k == 0 {
            return result;
        }

        if k.is_negative() {
            code.reverse();
        }

        for i in 0..len {
            result[i] = Self::sum_k(&code, i, k.abs());
        }

        if k.is_negative() {
            result.reverse();
        }
        result
    }
    pub fn loop_idx(idx: usize, len: usize) -> usize {
        (idx + len) % len
    }
    pub fn sum_k(code: &Vec<i32>, i: usize, k: i32) -> i32 {
        let mut result: i32 = 0;

        for j in 0..k as usize {
            result += code[Self::loop_idx(i + 1 + j, code.len())]
        }

        result
    }
}
