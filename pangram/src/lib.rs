use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut map: HashSet<char> = HashSet::new();
    for c in s.chars() {
        match c.to_ascii_lowercase() {
            'a'..='z' => {
                map.insert(c);
            }
            _ => {}
        }
    }
    map.len() == 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_pangram("the quick brown fox jumps over the lazy dog!"), true);
    }
     #[test]
    fn it_works1() {
        assert_eq!(is_pangram(""), false);
    }
}
