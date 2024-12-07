// Third party lib
class SquareDrawer {
  drawSquare(size: number): void {
    console.log(`Drawing a square with size of ${size}x${size}`);
  }
}

// Target interface (for end user consumption, client is expecting this)
interface RectangleDrawer {
  drawRectangle(width: number, height: number): void;
}

// Adapter that adapts SquareDrawer to the RectangleDrawer interface
class SquareToRectAdapter implements RectangleDrawer {
  private drawer: SquareDrawer;

  constructor(drawer: SquareDrawer) {
    this.drawer = drawer;
  }

  drawRectangle(width: number, height: number): void {
    if (width === height) {
      this.drawer.drawSquare(width);
    } else {
      console.log("Cannot draw rectangle with a square drawer :( \n");
    }
  }
}

// Usage
const squareDrawer = new SquareDrawer();
const adapter = new SquareToRectAdapter(squareDrawer);

adapter.drawRectangle(6, 6); // Works
adapter.drawRectangle(10, 20); // Fails
