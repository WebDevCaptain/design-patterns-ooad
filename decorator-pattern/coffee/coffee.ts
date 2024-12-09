// Component interface
interface Coffee {
  cost(): number;
  description(): string;
}

// Concrete component
class BasicCoffee implements Coffee {
  cost(): number {
    return 5;
  }

  description(): string {
    return `Basic coffee`;
  }
}

// Decorator (base)
abstract class CoffeeDecorator implements Coffee {
  protected coffee: Coffee;

  constructor(c: Coffee) {
    this.coffee = c;
  }

  abstract cost(): number;

  abstract description(): string;
}

// --------- Concrete Decorators -------------
class MilkDecorator extends CoffeeDecorator {
  cost(): number {
    return this.coffee.cost() + 1.5;
  }

  description(): string {
    return this.coffee.description() + ", Milk";
  }
}

class SugarDecorator extends CoffeeDecorator {
  cost(): number {
    return this.coffee.cost() + 0.5;
  }

  description(): string {
    return this.coffee.description() + ", Sugar";
  }
}

// Usage
const coffee = new BasicCoffee();
console.log(`${coffee.description()} -- ${coffee.cost()}`);

const milkCoffee = new MilkDecorator(coffee);
console.log(`${milkCoffee.description()} -- ${milkCoffee.cost()}`);

const milkAndSugarCoffee = new SugarDecorator(milkCoffee);
console.log(
  `${milkAndSugarCoffee.description()} -- ${milkAndSugarCoffee.cost()}`
);
