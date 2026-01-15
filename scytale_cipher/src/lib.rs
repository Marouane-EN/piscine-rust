pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty() || i == 0 {
        return "".to_string();
    }
    let mut res = String::new();
    let mut new_message = String::from(message);
    let mut n = message.len();
    while n % i != 0 {
        new_message.push_str(&" ");
        n += 1;
    }
    println!("new_message {new_message}--{i}--{}--{}", message.len(), new_message.len());

    for j in 0..i {
        let mut x = j;
        let first = new_message.chars().nth(j).unwrap();
        res.push(first);
        println!("{res} j {j} ");

        while x + i < new_message.len() {
            println!("{res} x {x}");
            let second = new_message
                .chars()
                .nth(x + i)
                .unwrap();

            res.push(second);
            x += i;
        }
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
        assert_eq!(scytale_cipher("attack morning", 6), "a ntmgto ar cn ki");
    }
}
