pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res = Vec::new();
    let i = 2;
    while res.len() <= 50 {
        let j = i * i;
        if j % 2 == 0 {
            res.push(j);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
