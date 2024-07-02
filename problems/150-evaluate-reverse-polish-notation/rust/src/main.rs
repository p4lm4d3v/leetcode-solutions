use std::collections::HashSet;

mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let operators: HashSet<String> = HashSet::from(["+", "-", "*", "/"].map(|f| f.to_string()));

        for token in tokens {
            if !operators.contains(&token) {
                stack.push(token.parse().unwrap())
            } else {
                let b: i32 = stack.pop().unwrap();
                let a: i32 = stack.pop().unwrap();
                match token.as_str() {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "*" => stack.push(a * b),
                    "/" => stack.push(a / b),
                    _ => (),
                }
            }
        }

        stack.pop().unwrap()
    }
}
