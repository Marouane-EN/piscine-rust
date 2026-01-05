pub fn to_url(s: &str) -> String {
    let mut result = String::from(s);
    result = result.replace(' ', "%20");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = to_url("Hello, world!");
        assert_eq!(s, "Hello,%20world!");
    }
}
