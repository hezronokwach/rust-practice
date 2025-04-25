use std::cmp::Ordering;

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
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, product_name: String) {
        if let Some(product) = store.products.iter().find(|(name, _)| name == &product_name) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.receipt.clear();

        if self.items.is_empty() {
            return Vec::new();
        }

        // Extract and sort prices
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        // Calculate discount (cheapest item free for each group of 3)
        let total_discount: f32 = prices
            .chunks(3)
            .filter(|chunk| chunk.len() == 3)
            .map(|chunk| chunk[0])
            .sum();

        // Calculate total price and scale factor
        let total_price: f32 = prices.iter().sum();
        let scale = if total_discount > 0.0 {
            (total_price - total_discount) / total_price
        } else {
            1.0
        };

        // Generate adjusted prices, round to 2 decimal places, and sort
        self.receipt = prices
            .into_iter()
            .map(|price| round_to_two_decimals(price * scale))
            .collect();
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        self.receipt.clone()
    }
}

fn round_to_two_decimals(num: f32) -> f32 {
    (num * 100.0).round() / 100.0
}