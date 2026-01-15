pub fn talking(text: &str) -> &str {
    match text {
        t if t == text.to_ascii_uppercase() && text.contains("?") => "Quiet, I am thinking!",
        t if t == text.to_ascii_uppercase() => "There is no need to yell, calm down!",
        t if t == text.to_ascii_lowercase() => "Sure.",
        "" => "Just say something!",
        _ => "Interesting",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(talking("JUST DO IT!"), "There is no need to yell, calm down!");
    }
}
