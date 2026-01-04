pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let div = x / y;
    let res = x % y;
    (div, res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
