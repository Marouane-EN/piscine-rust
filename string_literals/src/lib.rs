pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}
pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    match v.find(pat) {
        Some(index) => index,
        _none => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_empty("");
        assert_eq!(result, true);
    }
    #[test]
    fn it_works1() {
        let result = is_ascii("rust");
        assert_eq!(result, true);
    }
    #[test]
    fn it_works2() {
        let result = contains("rust", "ru");
        assert_eq!(result, true);
    }
    #[test]
    fn it_works3() {
        let result = split_at("rust", 2);
        assert_eq!(result, ("ru", "st"));
    }
    #[test]
    fn it_works4() {
        let result = find("rust", 'u');
        assert_eq!(result, 1);
    }
}
