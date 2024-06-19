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
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(elem, _)| elem == &ele) {
            self.items.push((ele, product.1));
        }
    }
    pub fn calculate_discounted_price(price: &f32, discount_per_item: &f32) -> f32 {
        ((*price * discount_per_item * 100.0).round()) / 100.0
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.receipt = self.items.iter().map(|item| item.1).collect::<Vec<f32>>();
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let discounted_items = self.receipt.len() / 3;
        let total: f32 = self.receipt.iter().sum::<f32>();
        let discount = self.receipt.iter().take(discounted_items).sum::<f32>();
        let total_discount = total - discount;
        let discount_per_item = total_discount / total;

        let calculate = |price: &f32| -> f32 { Self::calculate_discounted_price(price, &discount_per_item) };

        self.receipt.iter_mut().for_each(|price| *price = calculate(price));
        self.receipt.clone()
    }
}

fn main() {
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product C"), 3.12),
    ]);

    println!("{:?}", store);

    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));

    println!("{:?}", cart.generate_receipt());

    println!("{:?}", cart);
}
