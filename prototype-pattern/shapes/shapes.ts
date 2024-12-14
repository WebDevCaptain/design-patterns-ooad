abstract class Prototype {
  clone(): this {
    return Object.create(this);
  }
}

class Circle extends Prototype {
  private radius: number;

  constructor(rad: number) {
    super();
    this.radius = rad;
  }

  setRadius(rad: number): void {
    this.radius = rad;
  }

  toString(): string {
    return `Circle with radius ${this.radius}`;
  }
}

class Rectangle extends Prototype {
  private width: number;
  private height: number;

  constructor(width: number, height: number) {
    super();
    this.width = width;
    this.height = height;
  }

  setWidth(w: number): void {
    this.width = w;
  }

  setHeight(h: number): void {
    this.height = h;
  }

  toString(): string {
    return `Rectangle with width ${this.width} and height ${this.height}`;
  }
}

// Dry run
const c1 = new Circle(5);
const c2 = c1.clone();
c2.setRadius(9);

console.log(`Original: ${c1.toString()}, Cloned: ${c2.toString()}`);

const r1 = new Rectangle(10, 20);
const r2 = r1.clone();
r2.setHeight(101);

console.log(`Original rectangle: ${r1.toString()}, Cloned: ${r2.toString()}`);
