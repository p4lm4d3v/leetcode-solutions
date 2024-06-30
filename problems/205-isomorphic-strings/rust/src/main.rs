use std::collections::HashMap;

mod tests;

fn main() {}

struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        if s == t {
            return true;
        }
        Self::to_numeric(s) == Self::to_numeric(t)
    }
    pub fn to_numeric(str: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut n: i32 = 1;
        let mut numeric: String = String::new();
        for c in str.chars() {
            if map.contains_key(&c) {
                numeric += map[&c].to_string().as_str();
            } else {
                map.insert(c, n);
                numeric += n.to_string().as_str();
                n += 1;
            }
        }

        numeric
    }
}
