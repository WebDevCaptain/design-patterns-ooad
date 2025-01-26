// Pizza's structure
struct Pizza {
    size: String,
    crust: String,
    toppings: Vec<String>,
}

impl Pizza {
    fn new(size: String, crust: String, toppings: Vec<String>) -> Self {
        Self {
            size,
            crust,
            toppings,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "Pizza(size={}, crust={}, toppings={:?})",
            self.size, self.crust, self.toppings
        )
    }
}

// PizzaBuilder
struct PizzaBuilder {
    size: String,
    crust: String,
    toppings: Vec<String>,
}

impl PizzaBuilder {
    fn new() -> Self {
        Self {
            size: "Medium".to_string(),
            crust: "Thin".to_string(),
            toppings: Vec::new(),
        }
    }

    fn set_size(mut self, size: &str) -> Self {
        self.size = size.to_string();
        self
    }

    fn set_crust(mut self, crust: &str) -> Self {
        self.crust = crust.to_string();
        self
    }

    fn add_topping(mut self, topping: &str) -> Self {
        self.toppings.push(topping.to_string());
        self
    }

    fn build(self) -> Pizza {
        Pizza::new(self.size, self.crust, self.toppings)
    }
}

// Usage
fn main() {
    let pizza = PizzaBuilder::new()
        .set_size("Large")
        .set_crust("Thick")
        .add_topping("Pepperoni")
        .add_topping("Cheese")
        .build();

    println!("{}", pizza.to_string());
}
