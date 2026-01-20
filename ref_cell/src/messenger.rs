pub use std::rc::Rc;
pub use std::cell::RefCell;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: usize,
    pub max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self { messages: RefCell::new(Vec::new()), value: 0, max: max }
    }
    pub fn set_value<T>(&self, v: &Rc<T>) {
        let n = Rc::strong_count(v);
        let p = (n * 100) / self.max;
        if p > 70 {
            self.messages
                .borrow_mut()
                .push(
                    format!(
                        "Warning: You have used up over {}% of your quota!",
                        (n * 100) / self.max
                    )
                );
        } else if n >= self.max {
            self.messages.borrow_mut().push(format!("Error: You can't go over your quota!"));
        }
    }

    pub fn peek<T>(&self, v: &Rc<T>) {
        let n = Rc::strong_count(v);
        self.messages
            .borrow_mut()
            .push(
                format!("Info: This value would use {}% of your quota", (n * 100) / self.max)
            )
    }
}
