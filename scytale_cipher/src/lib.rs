pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut res = String::new();
    let mut new_message = String::from(message);
    let n = message.len() % i;
    if n != 0 {
        new_message.push_str(&" ".repeat(n));
    }
    for j in 0..i {
        let first = new_message.chars().nth(j).unwrap();
        let second = new_message
            .chars()
            .nth(j + i)
            .unwrap();

        res.push(first);
        res.push(second);
    }
    res.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(scytale_cipher("scytale Code", 6), "sec yCtoadle");
    }
    #[test]
    fn it_works1() {
        assert_eq!(scytale_cipher("scytale Code", 8), "sCcoydtea l e");
    }
}
