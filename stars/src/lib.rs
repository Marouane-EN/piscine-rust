pub fn stars(n: u32) -> String {
    let vec = vec!["*"; (2_i32).pow(n) as usize];
    vec.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(stars(1), "**".to_string());
    }
    #[test]
    fn it_works1() {
        assert_eq!(stars(4), "****************".to_string());
    }
    #[test]
    fn it_works2() {
        assert_eq!(stars(5), "********************************".to_string());
    }
}
