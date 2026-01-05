pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut index = 0;
    for name in names {
        let mut sum = String::new();
        sum.push(name.chars().next().unwrap());
        sum.push('.');
        sum.push(' ');
        index = name
            .chars()
            .position(|c| c == ' ')
            .unwrap();
        sum.push(
            name
                .chars()
                .nth(index + 1)
                .unwrap()
        );
        sum.push('.');
        result.push(sum.clone());
        sum.clear();
    }

    result
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
