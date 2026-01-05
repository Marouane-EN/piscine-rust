pub fn first_subword(mut s: String) -> String {
    let mut result = "".to_string();
    for (index, char) in s.chars().enumerate() {
        if char.is_ascii_uppercase() && index != 0 {
            break;
        }
        if char == '_' {
            break;
        }
        result.push(char);
    }
    s = result;
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s1 = first_subword("helloWorld".to_owned());
        assert_eq!(s1, "hello");
    }
    #[test]
    fn it_works1() {
        let s2 = first_subword("snake_case".to_owned());
        assert_eq!(s2, "snake");
    }
    #[test]
    fn it_works2() {
        let s3 = first_subword("CamelCase".to_owned());
        assert_eq!(s3, "Camel");
    }
    #[test]
    fn it_works3() {
        let s4 = first_subword("just".to_owned());
        assert_eq!(s4, "just");
    }
}
