pub fn doubtful(s: &mut String) {
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello".to_owned();
        assert_eq!(s, "Hello");
    }
    #[test]
    fn it_works1() {
        let mut s = "Hello".to_owned();
        doubtful(&mut s);
        assert_eq!(s, "Hello?");
    }
}
