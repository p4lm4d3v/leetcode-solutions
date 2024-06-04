use std::cmp::max;

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

#[cfg(test)]
mod compare_version {
    use super::*;

    #[test]
    fn compare_version1() {
        let res = Solution::compare_version("1.01".to_string(), "1.001".to_string());
        assert_eq!(res, 0);
    }
    #[test]
    fn compare_version2() {
        let res = Solution::compare_version("1.0".to_string(), "1.0.0".to_string());
        assert_eq!(res, 0);
    }
    #[test]
    fn compare_version3() {
        let res = Solution::compare_version("0.1".to_string(), "1.1".to_string());
        assert_eq!(res, -1);
    }
    #[test]
    fn compare_version4() {
        let res = Solution::compare_version("5.1".to_string(), "3.2".to_string());
        assert_eq!(res, 1);
    }
}

#[cfg(test)]
mod parse {
    use crate::Version;

    #[test]
    fn parse1() {
        assert_eq!(
            Version::parse("00000.1").version,
            Version {
                version: "0.1".to_string()
            }
            .version
        )
    }
    #[test]
    fn parse2() {
        assert_eq!(
            Version::parse("1.00000000000000000000000000001").version,
            Version {
                version: "1.1".to_string()
            }
            .version
        )
    }
    #[test]
    fn parse3() {
        assert_eq!(
            Version::parse("01.002.0003.00004.000005.0000006").version,
            Version {
                version: "1.2.3.4.5.6".to_string()
            }
            .version
        )
    }
    #[test]
    fn parse4() {
        assert_eq!(
            Version::parse("1").version,
            Version {
                version: "1".to_string()
            }
            .version
        )
    }
    #[test]
    fn parse5() {
        assert_eq!(
            Version::parse("0").version,
            Version {
                version: "0".to_string()
            }
            .version
        )
    }
}

#[cfg(test)]
mod remove_leading_zeroes {
    use super::*;
    #[test]
    fn remove_leading_zeroes1() {
        assert_eq!(Version::remove_leading_zeroes("000001"), "1");
    }
    #[test]
    fn remove_leading_zeroes2() {
        assert_eq!(Version::remove_leading_zeroes("000123"), "123");
    }
    #[test]
    fn remove_leading_zeroes3() {
        assert_eq!(Version::remove_leading_zeroes("456"), "456");
    }
    #[test]
    fn remove_leading_zeroes4() {
        assert_eq!(Version::remove_leading_zeroes("00000"), "0");
    }
    #[test]
    fn remove_leading_zeroes5() {
        assert_eq!(Version::remove_leading_zeroes("0"), "0");
    }
    #[test]
    fn remove_leading_zeroes6() {
        assert_eq!(Version::remove_leading_zeroes(""), "");
    }
    #[test]
    fn remove_leading_zeroes7() {
        assert_eq!(Version::remove_leading_zeroes("abc123"), "abc123");
    }
    #[test]
    fn remove_leading_zeroes8() {
        assert_eq!(Version::remove_leading_zeroes("1"), "1");
    }
}

#[cfg(test)]
mod len {
    use super::*;
    #[test]
    fn len1() {
        let v = Version::parse("1.0.0.0");
        assert_eq!(v.len(), 4);
    }
    #[test]
    fn len2() {
        let v = Version::parse("0.1");
        assert_eq!(v.len(), 2)
    }
    #[test]
    fn len3() {
        let v = Version::parse("1");
        assert_eq!(v.len(), 1)
    }
    #[test]
    fn len4() {
        let v = Version::parse("3");
        assert_eq!(v.len(), 1)
    }
    #[test]
    fn len5() {
        let v = Version::parse("0");
        assert_eq!(v.len(), 1)
    }
    #[test]
    fn len6() {
        let v = Version::parse("");
        assert_eq!(v.len(), 0)
    }
}
