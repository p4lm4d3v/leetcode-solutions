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
