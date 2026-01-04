pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sum = sum(2, 2);
        let diff = diff(100, 2);
        let pro = pro(2, 2);
        let quo = quo(22.0, 2.0);
        let rem = rem(-128.23, 2.0);
        assert_eq!(sum, 4);
        assert_eq!(diff, 98);
        assert_eq!(pro, 4);
        assert_eq!(quo, 11.0);
        assert_eq!(rem, -0.22999573);
    }
}
