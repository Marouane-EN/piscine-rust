pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let nums: Vec<u32> = s
        .split_whitespace()
        .map(|f| {
            if f.ends_with('k') {
                let n: Vec<u32> = f[..f.len() - 1]
                    .split(".")
                    .map(|t| { t.parse::<u32>().unwrap() })
                    .collect();

                return n[0] * 1000 + n[1] * 100;
            }
            f.parse::<u32>().unwrap()
        })
        .collect();
    let mut box_vec: Vec<Box<u32>> = Vec::new();
    for num in nums {
        box_vec.push(Box::new(num));
    }
    box_vec
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    for n in a {
        nums.push(*n);
    }

    nums
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn it_works() {
        let s = "5.5k 8.9k 32".to_owned();

        let boxed = parse_into_boxed(s);
        assert_eq!(boxed[0], Box::new(5500));
        assert_eq!(mem::size_of_val(&boxed[0]), 8);
    }
}
