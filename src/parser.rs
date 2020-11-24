use shlex;

pub fn parse(s: &str) -> Option<Vec<String>> {
    shlex::split(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        assert_eq!(parse("echo"), Some(vec!["echo"]));
        assert_eq!(parse("echo a b"), Some(vec!["echo", "a", "b"]));
        assert_eq!(parse("echo \"a b\""), Some(vec!["echo", "a b"]));
        assert_eq!(
            parse("echo \"a b\" \"c\""),
            Some(vec!["echo", "a b", "c"])
        );
        assert_eq!(
            parse("echo \"a \\\" b\" \\\"x\\\""),
            Some(vec!["echo", "a \" b", "\"x\""])
        );
        assert_eq!(parse("echo a\\ b"), Some(vec!["echo", "a b"]));
        assert_eq!(
            parse("NAME=\"<Your name>\" echo Hello $NAME"),
            Some(vec!["NAME=<Your name>", "echo", "Hello", "$NAME"])
        );
    }
}
