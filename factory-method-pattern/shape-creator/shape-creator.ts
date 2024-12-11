interface Shape {
  draw(): string;
}

// <=========  Concrete products  ===========>
class Circle implements Shape {
  draw(): string {
    return "Drawing a circle";
  }
}

class Square implements Shape {
  draw(): string {
    return "Drawing a square :||";
  }
}

// <---------- Abstract creator ------------>
interface ShapeFactory {
  createShape(): Shape;
}

// --- Concrete creators ----
class CircleFactory implements ShapeFactory {
  createShape(): Shape {
    return new Circle();
  }
}

class SquareFactory implements ShapeFactory {
  createShape(): Shape {
    return new Square();
  }
}

function drawShape(factory: ShapeFactory) {
  const shape = factory.createShape();
  console.log(shape.draw());
}

drawShape(new CircleFactory());
drawShape(new SquareFactory());
