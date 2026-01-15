pub fn number_logic(num: u32) -> bool {
    let s = num.to_string();
    let mut i = 1;
    loop {
        let mut sum = 0;
        for c in s.chars() {
            sum += c.to_digit(10).unwrap().pow(i);
        }
        if sum > num {
            return false;
        }
        if sum == num {
            return true;
        }
        i += 1;
        if i > 10 {
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_logic() {
        let array = [9, 10, 153, 154];
        for &pat in &array {
            if number_logic(pat) {
                println!(
                    "this number returns {} because the number {} obey the rules of the sequence",
                    number_logic(pat),
                    pat
                );
            } else {
                println!(
                    "this number returns {} because the number {} does not obey the rules of the sequence",
                    number_logic(pat),
                    pat
                );
            }
        }
    }
}
