pub fn num_to_ordinal(x: u32) -> String {
    let suffix = match x % 100 {
        11..=13 => "th",
        _ =>
            match x % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
    };
    format!("{x}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(num_to_ordinal(1), "1st");
    }
    #[test]
    fn it_works1() {
        assert_eq!(num_to_ordinal(22), "22nd");
    }
    #[test]
    fn it_works2() {
        assert_eq!(num_to_ordinal(43), "43rd");
    }
    #[test]
    fn it_works3() {
        assert_eq!(num_to_ordinal(47), "47th");
    }
    #[test]
    fn it_works4() {
        assert_eq!(num_to_ordinal(100000011), "100000011th");
    }
}
