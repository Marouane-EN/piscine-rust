pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = str_len("hello");

        assert_eq!(s, 5);
    }
    #[test]
    fn test2() {
        let s1 = str_len(&"camelCase".to_string());
        assert_eq!(s1, 9)
    }
    #[test]
    fn test3() {
        let s2 = str_len("olá!");

        assert_eq!(s2, 4)
    }
}
