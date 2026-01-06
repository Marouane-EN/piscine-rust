pub fn capitalize_first(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars() {
        if s.len() == 0 {
            s.push(c.to_ascii_uppercase());
            continue;
        }
        s.push(c);
    }
    s
}

pub fn title_case(input: &str) -> String {
    let mut words = input.split_ascii_whitespace().map(String::from).collect::<Vec<String>>();
    for word in words.iter_mut() {
        *word = word.chars().nth(0).unwrap().to_ascii_uppercase().to_string() + &word[1..];
    }
    words.join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut words = input.split_ascii_whitespace().map(String::from).collect::<Vec<String>>();
    for word in words.iter_mut() {
        *word = word
            .chars()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect();
    }
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(capitalize_first("1\ts"), "Joe is missing");
    }
}
