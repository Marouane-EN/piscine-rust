#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new(), receipt: Vec::new() }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(n) = s.products.iter().find(|f| f.0 == ele) {
            self.items.push(n.clone());
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.items.sort_by(|a, b| a.1.total_cmp(&b.1));
        let sum: f32 = self.items
            .iter()
            .map(|f| f.1)
            .sum();

        let mut sub = 0.0;

        if self.items.len() >= 3 {
            sub = self.items[0].1 / sum;
        }

        if self.items.len() >= 9 {
            sub = self.items[0].1 + self.items[1].1 + self.items[2].1 / sum;
        }

        for item in self.items.iter() {
            let mut i = item.1 * (1.0 - sub);
            i = (i * 100.0).round() / 100.0;
            self.receipt.push(i);
        }

        self.receipt.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let store = Store::new(
            vec![
                (String::from("product A"), 1.23),
                (String::from("product B"), 23.1),
                (String::from("product C"), 3.12)
            ]
        );

        println!("{:?}", store);

        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));
        cart.insert_item(&store, String::from("product C"));

        println!("{:?}", cart.generate_receipt());

        println!("{:?}", cart);
    }
}
