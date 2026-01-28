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
        let mut prices: Vec<f32> = self.items
            .iter()
            .map(|p| p.1)
            .collect();

        prices.sort_by(|a, b| a.total_cmp(b));

        let fm = prices.iter().take(prices.len() / 3);
        let free_sum: f32 = fm.sum();

        let total_sum: f32 = prices.iter().sum();

        self.receipt = prices
            .iter()
            .map(|val| {
                let discounted = (val * (total_sum - free_sum)) / total_sum;
                (discounted * 100.0).round() / 100.0
            })
            .collect();

        self.receipt.clone()
    }
}
