mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn length_of_last_word(mut s: String) -> i32 {
        let mut s: String = s.trim().to_string();
        let chars: Vec<char> = s.chars().collect();
        let mut parsed: String = String::from("");
        for i in (0..s.len()).rev() {
            if chars[i] == ' ' {
                break;
            }
            parsed += chars[i].to_string().as_str();
        }
        parsed.len() as i32
    }
}
