pub fn arrange_phrase(phrase: &str) -> String {
    let mut pharses: Vec<String> = phrase.split_whitespace().map(String::from).collect();
    pharses.sort_by_key(|p|
        p
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
    );
    for word in pharses.iter_mut() {
        *word = word
            .chars()
            .filter(|c| !c.is_numeric())
            .collect();
    }
    pharses.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(arrange_phrase("is2 Thi1s T4est 3a"), "This is a Test");
    }
}
