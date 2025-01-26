// Implementor
trait Color {
    fn fill_color(&self) -> &'static str;
}

// Concrete Implementor
struct RedColor;
impl Color for RedColor {
    fn fill_color(&self) -> &'static str {
        "red"
    }
}

struct GreenColor;
impl Color for GreenColor {
    fn fill_color(&self) -> &'static str {
        "green"
    }
}

// Abstraction
trait Shape {
    fn draw(&self) -> String;
}

// Refined Abstraction
struct Circle<'a> {
    color: &'a dyn Color,
}

impl<'a> Shape for Circle<'a> {
    fn draw(&self) -> String {
        format!("Circle filled with {} color", self.color.fill_color())
    }
}

struct Square<'a> {
    color: &'a dyn Color,
}

impl<'a> Shape for Square<'a> {
    fn draw(&self) -> String {
        format!("Square filled with {} color", self.color.fill_color())
    }
}

// Usage
fn main() {
    let red = RedColor;
    let green = GreenColor;

    let circle = Circle { color: &red };
    println!("{}", circle.draw()); // Circle filled with Red color

    let square = Square { color: &green };
    println!("{}", square.draw()); // Square filled with Green color
}
