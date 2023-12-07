use std::fmt;

struct Product {
    name: String,
    price: f32,
}

struct ShoppingCart<'a> {
    products: Vec<&'a Product>,
    user_id: &'a str,
}

impl<'a> ShoppingCart<'a> {
    fn new(user_id: &'a str) -> Self {
        ShoppingCart {
            products: Vec::new(),
            user_id,
        }
    }

    fn total(&self) -> f32 {
        self.products
            .iter()
            .fold(0.0, |acc, &product| acc + product.price)
    }

    fn add_product(&mut self, product: &'a Product) {
        self.products.push(product);
    }
}

impl fmt::Display for ShoppingCart<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut output = format!("Shopping cart for user {}:", self.user_id);
        for product in &self.products {
            output.push_str(&format!("{}: ${}, ", product.name, product.price));
        }
        output.push_str(&format!("Total: ${}", self.total()));
        write!(f, "{}", output)
    }
}

fn main() {
    let shop_items = vec![
        Product {
            name: "Milk".to_string(),
            price: 1.5,
        },
        Product {
            name: "Bread".to_string(),
            price: 2.5,
        },
        Product {
            name: "Eggs".to_string(),
            price: 3.5,
        },
    ];

    let mut user1_cart = ShoppingCart::new("user1");
    user1_cart.add_product(&shop_items[0]);
    user1_cart.add_product(&shop_items[0]);
    user1_cart.add_product(&shop_items[1]);
    assert_eq!(
        user1_cart.total(),
        [&shop_items[0], &shop_items[0], &shop_items[1]]
            .iter()
            .fold(0.0, |acc, product| acc + product.price)
    );
    println!("{user1_cart}");

    let mut user2_cart = ShoppingCart::new("user2");
    user2_cart.add_product(&shop_items[0]);
    user2_cart.add_product(&shop_items[0]);
    user2_cart.add_product(&shop_items[1]);
    println!("{user2_cart}");
}
