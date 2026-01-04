pub fn rev_str(input: &str) -> String {
    let mut rev = String::new();
    for c in input.chars().rev() {
        rev.push(c);
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rev_str("Hello, world!");
        assert_eq!(result, "!dlrow ,olleH");
    }
}
