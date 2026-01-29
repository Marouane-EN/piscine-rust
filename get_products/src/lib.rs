pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut n = Vec::new();
    for i in arr.iter() {
        let mut sum = 1;
        for j in arr.iter() {
            if j == i {
                continue;
            }
            sum *= j;
        }
        n.push(sum);
    }
    n
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
