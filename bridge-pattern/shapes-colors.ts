// Implementor
interface Color {
  fillColor(): string;
}

// Concrete Implementor
class RedColor implements Color {
  fillColor(): string {
    return "red";
  }
}

class GreenColor implements Color {
  fillColor(): string {
    return "green";
  }
}

// Abstraction ----
abstract class Shape {
  protected color: Color;

  constructor(color: Color) {
    this.color = color;
  }

  abstract draw(): string;
}

// Concrete Abstraction
class Circle extends Shape {
  draw(): string {
    return `Circle filled with ${this.color.fillColor()} color`;
  }
}

class Square extends Shape {
  draw(): string {
    return `Square filled with ${this.color.fillColor()} color`;
  }
}

// Client usage
const red = new RedColor();
const green = new GreenColor();

const circle = new Circle(red);
console.log(circle.draw()); // Circle filled with Red color

const square = new Square(green);
console.log(square.draw()); // Square filled with Green color
