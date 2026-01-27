pub fn score(text: &str) -> u64 {
    let mut result = 0;
    for c in text.chars() {
        match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => {
                result += 1;
            }
            'D' | 'G' => {
                result += 2;
            }
            'B' | 'C' | 'M' | 'P' => {
                result += 3;
            }
            'F' | 'H' | 'V' | 'W' | 'Y' => {
                result += 4;
            }
            'K' => {
                result += 5;
            }
            'J' | 'X' => {
                result += 8;
            }
            'Q' | 'Z' => {
                result += 10;
            }
            _ => {}
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(score("a"), 1);
    }
    #[test]
    fn it_works0() {
        assert_eq!(score("ã ê Á?"), 0);
    }
    #[test]
    fn it_works1() {
        assert_eq!(score("ThiS is A Test"), 14);
    }
}
