#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u32,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            age: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _result = Person::new("salam");
        println!("{_result:?}");
        assert_eq!(4, 4);
    }
}
