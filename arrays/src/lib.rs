pub fn sum(a: &[i32; 10]) -> i32 {
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let arr: [i32; 32] = std::array::from_fn(|_| 10);
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: [i32; 10] = (1..=10).collect::<Vec<i32>>().try_into().unwrap();
        let b = [5; 10];
        assert_eq!(sum(&a), 55);
        assert_eq!(sum(&b), 50);
        assert_eq!(thirtytwo_tens().len(), 32);
        assert_eq!(
            thirtytwo_tens(),
            [
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            ]
        )
    }
}
