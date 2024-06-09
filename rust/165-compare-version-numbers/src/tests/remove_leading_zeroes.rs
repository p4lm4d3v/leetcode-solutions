#[cfg(test)]
mod remove_leading_zeroes {
    use crate::Version;

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
