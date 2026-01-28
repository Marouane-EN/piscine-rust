pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    arr.iter()
        .map(|f| {
            let mut sum = 1;
            arr.iter().for_each(|x| (
                if f != x {
                    sum *= x;
                }
            ));
            sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: Vec<usize> = vec![1, 7, 3, 4];
        let output = get_products(arr);
        println!("{:?}", output);
    }
}
