pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    arr.iter()
        .enumerate()
        .map(|(i, _)| {
            arr.iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, x)| x)
                .product()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: Vec<usize> = vec![];
        let output = get_products(arr);
        println!("{:?}--", output);
    }
}
