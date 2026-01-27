#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut s = String::new();
    for c in original.chars() {
        let ascii_num: u32 = u32::from(c);
        if ascii_num > 64 && ascii_num < 91 {
            let d = ascii_num - 64;
            let char = (91 - (d as u8)) as char;
            s.push(char);
        } else if ascii_num > 96 && ascii_num < 123 {
            let d = ascii_num - 96;
            let char = (123 - (d as u8)) as char;
            s.push(char);
        } else {
            s.push(c);
        }
    }
    if s != ciphered.to_string() {
        return Err(CipherError { expected: s });
    } else {
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Ok(()));
    }
    #[test]
    fn it_works1() {
        assert_eq!(
            cipher("1Hello 2world!", "svool"),
            Err(CipherError { expected: "1Svool 2dliow!".to_string() })
        );
    }
}
