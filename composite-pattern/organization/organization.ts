// Component
interface EmployeeComponent {
  showDetails(): void;
}

// Leaf
class Employee implements EmployeeComponent {
  private name: string;
  private position: string;

  constructor(name: string, position: string) {
    this.name = name;
    this.position = position;
  }

  showDetails(): void {
    console.log(`${this.position} :: ${this.name}`);
  }
}

// Composite
class Manager implements EmployeeComponent {
  private name: string;
  private position: string;
  private subordinates: EmployeeComponent[] = [];

  constructor(name: string, position: string) {
    this.name = name;
    this.position = position;
    this.subordinates = [];
  }

  add(employee: EmployeeComponent): void {
    this.subordinates.push(employee);
  }

  remove(employee: EmployeeComponent): void {
    this.subordinates = this.subordinates.filter((e) => e !== employee);
  }

  showDetails(): void {
    console.log(`${this.position} -- ${this.name}`);

    for (const subordinate of this.subordinates) {
      subordinate.showDetails();
    }
  }
}

// Usage
const emp1 = new Employee("Shreyash", "Senior Engineer");
const emp2 = new Employee("Jenny", "SEO expert");

const manager = new Manager("Mahesh", "Tech lead");
manager.add(emp1);
manager.add(emp2);

manager.showDetails();
