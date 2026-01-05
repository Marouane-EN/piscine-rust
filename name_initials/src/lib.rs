pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let result: Vec<String> = name
                .split_whitespace()
                .map(|s| {
                    let ch = s.chars().next().unwrap();
                    format!("{}.", ch)
                })
                .collect();
            result.join(" ")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];

        assert_eq!(initials(names), ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
