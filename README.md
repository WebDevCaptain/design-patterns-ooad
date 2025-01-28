# Design Patterns (OOP)

Revisiting all the major design patterns (inspired by Gang of Four and open-source material on the WWW).

> This repo should serve as a reference guide (not a tutorial) as I plan to add very simple examples to illustrate the patterns.

---

<br>

## Patterns Covered

1. **Abstract Factory Pattern**

- Creates families of related objects without specifying their concrete classes.
- Provides a layer of abstraction for consistent object creation.
- Merits: Ensures consistency across related objects; improves scalability and flexibility.

---

2. Adapter Pattern

- Converts one interface into another that a client expects.
- Acts as a bridge between incompatible interfaces.
- Merits: Enhances reusability of code and compatibility between systems.

---

3. Bridge Pattern

- Decouples an abstraction from its implementation so they can evolve independently.
- Splits the "what" (abstraction) from the "how" (implementation).
- Merits: Increases flexibility and allows independent development.

---

4. Builder Pattern

- Constructs complex objects step-by-step by separating the construction process.
- Useful for creating objects with many configuration options.
- Merits: Improves code readability and allows for controlled, incremental object creation.

---

5. Chain of Responsibility Pattern

- Passes requests along a chain of handlers until one handles it.
- Each handler decides whether to process or pass the request.
- Merits: Promotes loose coupling and simplifies request processing logic.

---

6. Command Pattern

- Encapsulates a request as an object, allowing it to be executed, delayed, or queued.
- Decouples sender and receiver by introducing a command interface.
- Merits: Enables undo/redo functionality, improves extensibility.

---

7. Composite Pattern

- Treats individual objects and compositions of objects uniformly.
- Creates tree structures to represent part-whole hierarchies.
- Merits: Simplifies working with hierarchical or nested structures.

---

8. Decorator Pattern

- Dynamically adds behavior or functionality to objects without altering their structure.
- Wraps objects with additional responsibilities.
- Merits: Promotes flexibility and avoids subclassing for behavior changes.

---

9. Facade Pattern

- Provides a unified interface to a set of interfaces in a subsystem.
- Simplifies interaction with complex systems by offering a high-level API.
- Merits: Reduces complexity and improves ease of use.

---

10. Factory Method Pattern

- Defines an interface for creating objects but lets subclasses decide which class to instantiate.
- Promotes the use of polymorphism.
- Merits: Simplifies object creation and supports open/closed principle.

---

11. Flyweight Pattern

- Shares common parts of objects to reduce memory usage.
- Stores intrinsic state in shared objects and extrinsic state externally.
- Merits: Improves performance and reduces memory footprint for large object collections.

---

12. Iterator Pattern

- Provides a way to traverse elements of a collection without exposing its internal structure.
- Ensures a consistent interface for iteration.
- Merits: Improves maintainability and supports flexible iteration logic.

---

13. Mediator Pattern

- Centralizes communication between components to avoid direct dependencies.
- Components communicate via a mediator instead of directly with each other.
- Merits: Reduces coupling and improves system scalability.

---

14. Memento Pattern

- Captures and restores an object's state without exposing its internal details.
- Used for undo/redo functionality.
- Merits: Improves maintainability by encapsulating state restoration logic.

---

15. Observer Pattern

- Establishes a one-to-many dependency between objects so that updates are automatically sent to dependents.
- Subscribers (observers) are notified of changes in the subject.
- Merits: Promotes loose coupling and ensures real-time updates.

---

16. Prototype Pattern

- Creates objects by copying an existing object (prototype).
- Useful when object creation is costly or complex.
- Merits: Speeds up object creation and simplifies cloning logic.

---

17. Proxy Pattern

- Provides a placeholder or surrogate to control access to another object.
- Can add extra functionality like caching, logging, or security checks.
- Merits: Enhances performance and security; improves access control.

---

18. Singleton Pattern

- Ensures a class has only one instance and provides a global point of access to it.
- Useful for managing shared resources like configurations or loggers.
- Merits: Simplifies global state management but requires careful handling in multithreaded environments.

---

19. State Pattern

- Allows an object to alter its behavior when its internal state changes.
- Encapsulates state-specific behavior into separate classes.
- Merits: Improves flexibility, readability, and maintainability for state-driven logic.

---

20. Strategy Pattern

- Defines a family of algorithms and makes them interchangeable at runtime.
- Encapsulates algorithm logic into separate classes.
- Merits: Improves code reusability and supports open/closed principle.

---

21. Template Method Pattern

- Defines the skeleton of an algorithm in a base class and lets subclasses override specific steps.
- Promotes reuse of common logic while allowing customization.
- Merits: Encourages code reuse and ensures consistency across implementations.

---

22. Visitor Pattern

- Separates an algorithm from the objects it operates on by using a visitor object.
- Adds new operations to classes without modifying them.
- Merits: Supports open/closed principle and simplifies complex object structures.

---

<br>
<br>
<br>

_Programming languages used:_

- Python ✅
- TypeScript ✅
- Rust ✅
- Go [TODO]
