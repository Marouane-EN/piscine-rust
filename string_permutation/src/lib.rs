pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut skip_index = Vec::new();
    for (i, c) in s1.char_indices() {
        for h in s2.chars() {
            if c == h && !skip_index.contains(&i) {
                skip_index.push(i);
            }
        }
    }
    skip_index.len() == s1.chars().count() && skip_index.len() == s2.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(!is_permutation("cde", "edbca"), true);
    }
    #[test]
    fn it_works1() {
        assert_eq!(is_permutation("hello♥", "♥oelhl"), true);
    }
    #[test]
    fn it_works2() {
        assert_eq!(is_permutation("thought", "thougth"), true);
    }
}
