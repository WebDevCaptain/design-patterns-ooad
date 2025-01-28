// Visitor trait
trait ShapeVisitor {
    fn visit_circle(&self, circle: &Circle) -> f64;
    fn visit_rectangle(&self, rectangle: &Rectangle) -> f64;
}

// Concrete visitor
struct AreaCalculator;

impl ShapeVisitor for AreaCalculator {
    fn visit_circle(&self, circle: &Circle) -> f64 {
        circle.radius * circle.radius * std::f64::consts::PI
    }

    fn visit_rectangle(&self, rectangle: &Rectangle) -> f64 {
        rectangle.width * rectangle.height
    }
}

struct PerimeterCalculator;

impl ShapeVisitor for PerimeterCalculator {
    fn visit_circle(&self, circle: &Circle) -> f64 {
        2.0 * std::f64::consts::PI * circle.radius
    }

    fn visit_rectangle(&self, rectangle: &Rectangle) -> f64 {
        2.0 * (rectangle.width + rectangle.height)
    }
}

// Shape trait
trait Shape {
    fn accept(&self, visitor: &dyn ShapeVisitor) -> f64;
}

// Concrete shapes
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn accept(&self, visitor: &dyn ShapeVisitor) -> f64 {
        visitor.visit_circle(self)
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn accept(&self, visitor: &dyn ShapeVisitor) -> f64 {
        visitor.visit_rectangle(self)
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    let area_calculator = AreaCalculator;

    println!("Circle area: {}", circle.accept(&area_calculator));
    println!("Rectangle area: {}", rectangle.accept(&area_calculator));

    let perimeter_calculator = PerimeterCalculator;

    println!("Circle perimeter: {}", circle.accept(&perimeter_calculator));
    println!(
        "Rectangle perimeter: {}",
        rectangle.accept(&perimeter_calculator)
    );
}
