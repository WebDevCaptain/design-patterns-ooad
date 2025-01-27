// An interface (trait) for shape
trait Shape {
    fn draw(&self);
}

// Concrete implementation of Circle
struct Circle {
    radius: u32,
}

impl Circle {
    fn new(radius: u32) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

// Concrete implementation of Rectangle
struct Square {
    side: u32,
}

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}", self.side);
    }
}

impl Square {
    fn new(side: u32) -> Self {
        Self { side }
    }
}

// Flyweight factory
struct ShapeFactory {
    circles: std::collections::HashMap<u32, Circle>,
    squares: std::collections::HashMap<u32, Square>,
}

impl ShapeFactory {
    fn new() -> Self {
        ShapeFactory {
            circles: std::collections::HashMap::new(),
            squares: std::collections::HashMap::new(),
        }
    }

    fn get_circle(&mut self, radius: u32) -> &Circle {
        self.circles.entry(radius).or_insert_with(|| {
            println!("Creating a new circle with radius {}", radius);
            Circle::new(radius)
        })
    }

    fn get_square(&mut self, side: u32) -> &Square {
        self.squares.entry(side).or_insert_with(|| {
            println!("Creating a new square with side {}", side);
            Square::new(side)
        })
    }
}

fn main() {
    let mut shape_factory = ShapeFactory::new();

    // Reuse circles with the same radius
    let circle1 = shape_factory.get_circle(10);
    circle1.draw();

    let circle2 = shape_factory.get_circle(20);
    circle2.draw();

    let circle3 = shape_factory.get_circle(10); // Should reuse circle1
    circle3.draw();

    println!();

    let square1 = shape_factory.get_square(10);
    square1.draw();

    let square2 = shape_factory.get_square(36);
    square2.draw();

    let square3 = shape_factory.get_square(36); // Should reuse square2
    square3.draw();
}
