#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let s = self.head.take();
        let n;
        match s {
            Some(j) => {
                n = Node { value, next: Some(Box::new(j)) };
            }
            None => {
                n = Node { value, next: None };
            }
        }
        self.head = Some(n);
    }

    pub fn pop(&mut self) {
        self.head.take().map(|f| {
            self.head = Some(*f.next.unwrap());
        });
    }

    pub fn len(&self) -> usize {
        let mut i = 0;
        let mut c = self.head.as_ref();
        let mut d;
        while let Some(n) = c {
            i += 1;
            d = n;
            match &d.next {
                Some(k) => c = Some(&k),
                None => c = None,
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
