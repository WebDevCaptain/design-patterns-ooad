// Abstract Product
trait Shape {
    fn draw(&self) -> String;
}

// Concrete Product 1
struct Circle;

impl Shape for Circle {
    fn draw(&self) -> String {
        "Drawing a circle".to_string()
    }
}

// Concrete Product 2
struct Square;

impl Shape for Square {
    fn draw(&self) -> String {
        "Drawing a square".to_string()
    }
}

// Abstract Creator
trait ShapeFactory {
    fn create_shape(&self) -> Box<dyn Shape>;
}

// Concrete Creator 1
struct CircleFactory;

impl ShapeFactory for CircleFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Circle)
    }
}

// Concrete Creator 2
struct SquareFactory;

impl ShapeFactory for SquareFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Square)
    }
}

fn draw_shape(fact: &dyn ShapeFactory) {
    let shape = fact.create_shape();
    println!("{}", shape.draw());
}

fn main() {
    let circle_factory = CircleFactory;
    let square_factory = SquareFactory;

    draw_shape(&circle_factory);
    draw_shape(&square_factory);
}
