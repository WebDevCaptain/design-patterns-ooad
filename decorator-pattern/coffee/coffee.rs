// Component trait
trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

// Concrete component
struct BasicCoffee;

impl Coffee for BasicCoffee {
    fn cost(&self) -> f64 {
        10.0
    }

    fn description(&self) -> String {
        "A basic coffee".to_string()
    }
}

// Decorator trait
// struct CoffeeDecorator<T: Coffee> {
//     coffee: T,
// }

// impl<T: Coffee> CoffeeDecorator<T> {
//     fn new(coffee: T) -> Self {
//         Self { coffee }
//     }
// }

// Concrete decorators
struct MilkDecorator<T: Coffee> {
    coffee: T,
}

impl<T: Coffee> MilkDecorator<T> {
    fn new(coffee: T) -> Self {
        Self { coffee }
    }
}

impl<T: Coffee> Coffee for MilkDecorator<T> {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 5.0
    }

    fn description(&self) -> String {
        format!("{}, with milk", self.coffee.description())
    }
}

struct SugarDecorator<T: Coffee> {
    coffee: T,
}

impl<T: Coffee> SugarDecorator<T> {
    fn new(coffee: T) -> Self {
        Self { coffee }
    }
}

impl<T: Coffee> Coffee for SugarDecorator<T> {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 1.0
    }

    fn description(&self) -> String {
        format!("{}, with sugar", self.coffee.description())
    }
}

fn main() {
    let basic_coffee = BasicCoffee;
    println!("{} -> ${}", basic_coffee.description(), basic_coffee.cost());

    let milk_coffee = MilkDecorator::new(basic_coffee);
    println!("{} -> ${}", milk_coffee.description(), milk_coffee.cost());

    let milk_sugar_coffee = SugarDecorator::new(milk_coffee);
    println!(
        "{} -> ${}",
        milk_sugar_coffee.description(),
        milk_sugar_coffee.cost()
    );
}
