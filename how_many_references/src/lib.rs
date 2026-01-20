pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self { ref_list: ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut n: Vec<Rc<String>> = vec![];
        for e in &self.ref_list {
            if !Rc::ptr_eq(e, &element) {
                n.push(e.clone());
            }
        }
        self.ref_list = n;
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
