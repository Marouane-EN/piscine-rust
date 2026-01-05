pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();
    for c in a.chars() {
        if !c.is_numeric() {
            result.push(' ');
            continue;
        }
        result.push_str(&(c.to_digit(10).unwrap() as f64).exp().to_string());
    }
    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result: Vec<f64> = Vec::new();
    b.iter().for_each(|x| { result.push((*x as f64).abs().ln()) });
    (b, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let positive_inf: f64 = std::f64::INFINITY;
        let c = 0;
        assert_eq!(nbr_function(c), (0, 1.0, -positive_inf));
    }
    #[test]
    fn it_works0() {
        let b = vec![1, 2, 4, 5];
        let c: Vec<i32> = vec![1, 2, 4, 5];
        let result: Vec<f64> = vec![
            0.0,
            0.6931471805599453,
            1.3862943611198906,
            1.6094379124341003
        ];

        assert_eq!(vec_function(b), (c, result));
    }
    #[test]
    fn it_works1() {
        let a = "1 2 4 5 6".to_owned();
        assert_eq!(str_function(a), (
            "1 2 4 5 6".to_owned(),
            "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_owned(),
        ));
    }
}
