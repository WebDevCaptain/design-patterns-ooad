use std::rc::Rc;

// Component
trait EmployeeComponent {
    fn show_details(&self);
}

// Leaf
struct Employee {
    name: String,
    position: String,
}

impl EmployeeComponent for Employee {
    fn show_details(&self) {
        println!("Position: {}, Name: {}", self.position, self.name);
    }
}

impl Employee {
    fn new(name: &str, position: &str) -> Self {
        Self {
            name: name.to_string(),
            position: position.to_string(),
        }
    }
}

// Composite
struct Manager {
    name: String,
    position: String,
    subordinates: Vec<Rc<dyn EmployeeComponent>>,
}

impl EmployeeComponent for Manager {
    fn show_details(&self) {
        println!("Position: {}, Name: {}", self.position, self.name);

        for subord in &self.subordinates {
            subord.show_details();
        }
    }
}

impl Manager {
    fn new(name: &str, position: &str) -> Self {
        Self {
            name: name.to_string(),
            position: position.to_string(),
            subordinates: vec![],
        }
    }

    fn add(&mut self, employee: Rc<dyn EmployeeComponent>) {
        self.subordinates.push(employee);
    }
}

fn main() {
    let emp1 = Rc::new(Employee::new("Shreyash", "ML Engineer"));
    let emp2 = Rc::new(Employee::new("Khushboo", "Software Engineer"));

    let mut manager = Manager::new("Gaurav", "Staff Engineer");
    manager.add(emp1.clone());
    manager.add(emp2.clone());

    manager.show_details();
}
