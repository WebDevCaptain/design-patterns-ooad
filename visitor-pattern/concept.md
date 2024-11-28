The *Visitor Design Pattern* allows you to add further operations to objects without modifying their structure. It is achieved by separating the algorithm from the object structure.


* Example-1: Shape hierarchy
  ---
  Imagine a `Shape` hierarchy with two classes: Circle and Rectangle. We want to apply operations like calculating the area or printing the shape's name. Instead of embedding these operations inside the Shape classes, we use a Visitor class to encapsulate the operations.
