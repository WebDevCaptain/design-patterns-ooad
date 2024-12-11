interface Shape {
  draw(): void;
}

/**
 * Concrete Flyweight class representing a circle.
 */
class Circle implements Shape {
  private radius: number;

  constructor(radius: number) {
    this.radius = radius;
  }

  draw(): void {
    console.log(`Drawing a circle with a radius of ${this.radius}`);
  }
}

/**
 * Flyweight Factory to manage and reuse shapes.
 */
class ShapeFactory {
  private shapes = {};

  getCircle(radius: number): Circle {
    if (!this.shapes[radius]) {
      this.shapes[radius] = new Circle(radius);
      console.log(`Creating a new circle with radius ${radius}`);
    }

    return this.shapes[radius];
  }
}

// Usage
const shapeFactory = new ShapeFactory();

const c1 = shapeFactory.getCircle(12);
c1.draw();

const c2 = shapeFactory.getCircle(12);
c2.draw();

const c3 = shapeFactory.getCircle(32);
c3.draw();
