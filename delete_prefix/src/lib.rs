pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    s.strip_prefix(prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = "foo:nan";
        assert_eq!(delete_prefix("foo", result), Some(":nan"));
    }
}
