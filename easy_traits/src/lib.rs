pub trait AppendStrExt {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self;

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self;

    fn remove_punctuation_marks(&mut self) -> &mut Self;
}

impl AppendStrExt for String {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self {
        str_to_append.chars().for_each(|f| self.push(f));
        self
    }

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self {
        nb_to_append
            .to_string()
            .chars()
            .for_each(|c| self.push(c));
        self
    }

    fn remove_punctuation_marks(&mut self) -> &mut Self {
        *self = self
            .chars()
            .filter(|c| !".,?!".contains(*c))
            .collect();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "hello".to_owned();
        assert_eq!(s.append_str(" there!"), "hello there!");
    }
    #[test]
    fn it_works1() {
        let mut s = "hello there!".to_owned();
        assert_eq!(s.remove_punctuation_marks(), "hello there");
    }
}
