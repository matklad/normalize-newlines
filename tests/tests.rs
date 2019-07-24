use proptest::prelude::*;

use normalize_newlines::normalize_newlines;

#[test]
fn test_normalize_newlines() {
    fn check(before: &str, after: &str, ok: bool) {
        let mut actual = before.to_string();
        assert_eq!(normalize_newlines(&mut actual).is_ok(), ok);
        assert_eq!(actual.as_str(), after);
    }
    check("", "", true);
    check("\n", "\n", true);
    check("\r", "\r", false);
    check("\r\r", "\r\r", false);
    check("\r\n", "\n", true);
    check("hello world", "hello world", true);
    check("hello\nworld", "hello\nworld", true);
    check("hello\r\nworld", "hello\nworld", true);
    check("\r\nhello\r\nworld\r\n", "\nhello\nworld\n", true);
    check("\r\r\n", "\r\n", false);
    check("hello\rworld", "hello\rworld", false);
}

proptest! {
    #[test]
    fn doesnt_crash(s in "(\r\nabЫ)*") {
        let mut actual = s.to_string();
        let _ = normalize_newlines(&mut actual);
        assert_eq!(
            s.replace("\r", "").replace("\r", "\n"),
            actual.replace("\r", "").replace("\r", "\n"),
        );
    }
}
