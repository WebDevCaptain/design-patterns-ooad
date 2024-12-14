// Abstract visitor
interface ShapeVisitor {
  visitCircle(circle: Shape): number;
  visitRectangle(rectangle: Shape): number;
}

// Concrete visitor
class AreaCalculator implements ShapeVisitor {
  visitCircle(circle: Circle): number {
    return Math.PI * Math.pow(circle.radius, 2);
  }

  visitRectangle(rectangle: Rectangle): number {
    return rectangle.width * rectangle.height;
  }
}

// Abstract shape
interface Shape {
  accept(visitor: ShapeVisitor): number;
}

// <------- Concrete shapes -------->
class Circle implements Shape {
  private _radius: number;

  constructor(rad: number) {
    this._radius = rad;
  }

  get radius() {
    return this._radius;
  }

  accept(visitor: ShapeVisitor): number {
    return visitor.visitCircle(this);
  }
}

class Rectangle implements Shape {
  private _width: number;
  private _height: number;

  constructor(height: number, width: number) {
    this._width = width;
    this._height = height;
  }

  get width() {
    return this._width;
  }

  get height() {
    return this._height;
  }

  accept(visitor: ShapeVisitor): number {
    return visitor.visitRectangle(this);
  }
}

// Dry run
const c1 = new Circle(10);
const r1 = new Rectangle(5, 7);

const visitor = new AreaCalculator();
console.log(`Area of circle: ${c1.accept(visitor)}`);
console.log(`Area of rectangle: ${r1.accept(visitor)}`);
