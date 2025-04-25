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

    pub fn insert_item(&mut self, store: &Store, ele: String) {
        // Find the product in the store and add it to the cart
        if let Some(product) = store.products.iter().find(|(name, _)| name == &ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // Clear previous receipt
        self.receipt.clear();
        
        // If no items, return empty receipt
        if self.items.is_empty() {
            return self.receipt.clone();
        }

        // Get prices and sort them
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Calculate total discount from "buy three, get one free"
        let mut total_discount = 0.0;
        for chunk in prices.chunks(3) {
            if chunk.len() == 3 {
                // Cheapest item in each group of 3 is free
                total_discount += chunk[0];
            }
        }

        // Calculate total price before discount
        let total_price: f32 = prices.iter().sum();
        
        // If no discount, return original prices sorted
        if total_discount == 0.0 {
            self.receipt = prices.iter().map(|x| (x * 100.0).round() / 100.0).collect();
            self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return self.receipt.clone();
        }

        // Calculate the scaling factor to distribute discount
        let target_total = total_price - total_discount;
        let scale = target_total / total_price;

        // Apply discount proportionally and round to 2 decimal places
        self.receipt = prices
            .iter()
            .map(|price| ((price * scale * 100.0).round() / 100.0))
            .collect();

        // Sort receipt prices
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        self.receipt.clone()
    }
}