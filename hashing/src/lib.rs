use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        return 0.0;
    }
    let s: i32 = list.iter().sum();
    (s as f64) / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }

    let mut list = Vec::from(list);
    list.sort();
    let n = list.len();

    if n % 2 != 0 {
        list[n / 2]
    } else {
        (list[n / 2 - 1] + list[n / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for l in list {
        *map.entry(l).or_insert(0) += 1;
    }

    map.iter()
        .max_by_key(|entry| entry.1)
        .map(|(k, _v)| **k)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [i32; 8] = [4, 7, 6, 2, 5, 1, 3, 5];

    #[test]
    fn test_mean() {
        assert_eq!(mean(&DATA), 4.125);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&DATA), 4);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(&DATA), 5);
    }
}
