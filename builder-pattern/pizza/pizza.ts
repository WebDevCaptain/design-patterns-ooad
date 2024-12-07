class Pizza {
  private size: string;
  private crust: string;
  private toppings: string[];

  constructor(size: string, crust: string, toppings: string[]) {
    this.size = size;
    this.crust = crust;
    this.toppings = toppings;
  }

  toString(): string {
    return `Pizza(size=${this.size}, crust=${this.crust}, toppings=${this.toppings})`;
  }
}

// Builder allows us to construct the Pizza (object) incrementally with diff configurations
class PizzaBuilder {
  private size = "Medium";
  private crust = "Thin";
  private toppings: string[] = [];

  setSize(size: string): this {
    this.size = size;
    return this;
  }

  setCrust(crust: string): this {
    this.crust = crust;
    return this;
  }

  addTopping(topping: string): this {
    this.toppings.push(topping);
    return this;
  }

  build(): Pizza {
    return new Pizza(this.size, this.crust, this.toppings);
  }
}

// Usage
const pizza = new PizzaBuilder()
  .setSize("Extra Large")
  .setCrust("Cheese Burst")
  .addTopping("Mushroom")
  .addTopping("Chicken")
  .addTopping("Caps")
  .build();

console.log("\n", pizza.toString(), "\n");
