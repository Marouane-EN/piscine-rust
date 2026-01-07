pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut skip_index = Vec::new();
    for (i, c) in s1.char_indices() {
        for h in s2.chars() {
            if c == h && !skip_index.contains(&i) {
                skip_index.push(i);
            }
        }
    }
    skip_index.len() == s1.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thotugh";
        let word1 = "thougth";
        assert_eq!(is_permutation(word, word1), true);
    }
}
