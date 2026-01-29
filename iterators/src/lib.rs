#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 || self.v == 1 {
            return None;
        }
        if self.v % 2 == 0 {
            self.v = self.v / 2;
        } else {
            self.v = (3 * self.v) + 1;
        }
        Some(Collatz { v: self.v })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let c = Collatz::new(n);
    c.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        println!("{:?}", collatz(4));
        println!("{:?}", collatz(5));
        println!("{:?}", collatz(6));
        println!("{:?}", collatz(7));
        println!("{:?}", collatz(12));
    }
}
