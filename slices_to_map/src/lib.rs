use std::{collections::HashMap, hash::Hash};
pub fn slices_to_map<'a, 'b, T: Hash + Eq, U>(a: &'a [T], b: &'b [U]) -> HashMap<&'a T, &'b U> {
    let mut map:HashMap<&'a T, &'b U> = HashMap::new();
    let m = a.len().min(b.len());
    for i in 0..m {
        map.insert(&a[i], &b[i]);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
	let values = [1, 3, 23, 5, 2];
	println!("{:?}", slices_to_map(&keys, &values));
    }
}
