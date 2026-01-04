pub fn factorial(num: u64) -> u64 {
    let mut result = 1;
    for i in 1..=num {
        result *= i;
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(5);
        assert_eq!(result, 120);
    }
}
