#[cfg(test)]
mod len {
    use crate::Version;

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
