from abc import ABC, abstractmethod
from typing import List


# Component
class EmployeeComponent(ABC):
    @abstractmethod
    def show_details(self):
        raise NotImplementedError("This method should be implemented by subclasses")


# Leaf
class Employee(EmployeeComponent):
    def __init__(self, name: str, position: str):
        self.name = name
        self.position = position

    def show_details(self):
        print(f"{self.position}: {self.name}")


# Composite
class Manager(EmployeeComponent):
    def __init__(self, name: str, position: str):
        self.name = name
        self.position = position
        self.subordinates: List[EmployeeComponent] = []

    def add(self, employee: EmployeeComponent):
        self.subordinates.append(employee)

    def remove(self, employee: EmployeeComponent):
        self.subordinates.remove(employee)

    def show_details(self):
        print(f"{self.position}: {self.name}")
        for subordinate in self.subordinates:
            subordinate.show_details()


# Dryrun
if __name__ == "__main__":
    # Employees
    emp1 = Employee("Shreyash", "Engineer")
    emp2 = Employee("Khushi", "Designer")

    # Manager
    manager = Manager("Hrithik", "Team Lead")
    manager.add(emp1)
    manager.add(emp2)

    # Show details
    manager.show_details()
