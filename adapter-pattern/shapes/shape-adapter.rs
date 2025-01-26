// Third-party lib
struct SquareDrawer;
impl SquareDrawer {
    fn draw_square(&self, size: u32) {
        println!("Drawing a square of size {}x{}", size, size);
    }
}

// Trait (interface) that the client is expecting
trait RectangleDrawer {
    fn draw_rectangle(&self, width: u32, height: u32);
}

// Adapter
struct SquareToRectangleAdapter<'a> {
    square_drawer: &'a SquareDrawer,
}

impl<'a> RectangleDrawer for SquareToRectangleAdapter<'a> {
    fn draw_rectangle(&self, width: u32, height: u32) {
        if width == height {
            self.square_drawer.draw_square(width);
        } else {
            println!("Cannot draw a rectangle with a square drawer!");
        }
    }
}

// Usage
fn main() {
    let square_drawer = SquareDrawer;
    let adapter = SquareToRectangleAdapter {
        square_drawer: &square_drawer,
    };

    adapter.draw_rectangle(6, 6); // Works
    adapter.draw_rectangle(10, 20); // Fails gracefully
}
