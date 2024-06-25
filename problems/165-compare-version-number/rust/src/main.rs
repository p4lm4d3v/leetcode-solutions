use std::cmp::max;

mod tests;

fn main() {}

struct Version {
    version: String,
}

impl Version {
    fn to_string(&self) -> &str {
        self.version.as_str()
    }
    fn revisions(&self) -> Vec<&str> {
        self.version.split('.').collect()
    }
    fn len(&self) -> usize {
        if self.version == "" {
            return 0;
        }
        self.revisions().len()
    }
    fn expand(&mut self, new_len: usize) {
        if new_len == self.len() {
            return;
        }
        let to_add = new_len - self.len();
        for _ in 0..to_add {
            self.version += ".0";
        }
    }
    fn parse(version: &str) -> Self {
        let mut split: Vec<&str> = version.split('.').collect();
        split = split
            .iter()
            .map(|f| Self::remove_leading_zeroes(*f))
            .collect();
        Self {
            version: split.join("."),
        }
    }
    fn remove_leading_zeroes(with0: &str) -> &str {
        if with0 == "0" {
            return with0;
        }
        let mut without0 = with0;
        while without0.starts_with('0') {
            if without0 == "0" {
                return without0;
            }
            if let Some(x) = without0.strip_prefix('0') {
                without0 = x;
            } else {
                break;
            }
        }
        without0
    }
    fn compare(&mut self, other: &mut Version) -> i32 {
        let max_len = max(self.len(), other.len());
        self.expand(max_len);
        other.expand(max_len);
        if self.version != other.version {
            for i in 0..max_len {
                if let Some(v1_str) = self.revisions().get(i) {
                    if let Some(v2_str) = other.revisions().get(i) {
                        let v1: i32 = v1_str.parse().unwrap();
                        let v2: i32 = v2_str.parse().unwrap();

                        // Compare parsed integers
                        if v1 > v2 {
                            return 1;
                        } else if v2 > v1 {
                            return -1;
                        }
                    }
                }
            }
        }
        0
    }
}

struct Solution {}
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1: Version = Version::parse(version1.as_str());
        let mut v2: Version = Version::parse(version2.as_str());
        v1.compare(&mut v2)
    }
}
