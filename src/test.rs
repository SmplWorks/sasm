use crate::parse;

macro_rules! case {
    ($ident:ident, $code:literal, $result:expr) => {
        #[test]
        fn $ident() {
            assert_eq!(parse($code), $result);
        }
    };
}

case!(empty, "", Ok(vec![]));
case!(comment, "// This is a comment", Ok(vec![]));
case!(multi_comment, "/* This is a\n * multiline\n * comment\n */", Ok(vec![]));
