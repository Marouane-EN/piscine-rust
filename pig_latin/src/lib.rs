pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    let mut res = String::new();
    let mut ad = String::new();
    for (i, ch) in text.char_indices() {
        if !ad.is_empty() && ((i == 1 && ch == 'q') || (i == 2 && ch == 'u')) {
            ad.push(ch);
            continue;
        }
        if !vowels.contains(ch) && res.is_empty() {
            ad.push(ch);
            continue;
        }
        res.push(ch);
    }

    res.push_str(&ad);
    res.push_str("ay");

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_igloo() {
        assert_eq!(pig_latin("igloo"), "iglooay");
    }

    #[test]
    fn test_apple() {
        assert_eq!(pig_latin("apple"), "appleay");
    }

    #[test]
    fn test_hello() {
        assert_eq!(pig_latin("hello"), "ellohay");
    }

    #[test]
    fn test_square() {
        assert_eq!(pig_latin("square"), "aresquay");
    }

    #[test]
    fn test_xenon() {
        assert_eq!(pig_latin("xenon"), "enonxay");
    }

    #[test]
    fn test_chair() {
        assert_eq!(pig_latin("chair"), "airchay");
    }

    #[test]
    fn test_queen() {
        assert_eq!(pig_latin("queen"), "ueenqay");
    }
}
