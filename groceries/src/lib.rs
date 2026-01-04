pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = Vec::new();
        let first = String::from("ssss");
        insert(&mut vec, first);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], "ssss")
    }

    #[test]
    fn alwiski() {
        let slice = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];
        let result = at_index(&slice, 1);
        assert_eq!(result, "panettone")
    }
}
