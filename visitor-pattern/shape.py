from abc import ABC, abstractmethod
import math


# Abstract visitor
class ShapeVisitor(ABC):
    @abstractmethod
    def visit_circle(self, circle):
        pass

    def visit_rectangle(self, rectangle):
        pass


# Concrete Visitor
class AreaCalculator(ShapeVisitor):
    def visit_circle(self, circle):
        return math.pi * (circle.radius**2)

    def visit_rectangle(self, rectangle):
        return rectangle.width * rectangle.height


# Abstract shape
class Shape(ABC):
    @abstractmethod
    def accept(self, visitor: ShapeVisitor):
        pass


class Circle(Shape):
    def __init__(self, radius: int):
        self.radius = radius

    def accept(self, visitor: ShapeVisitor):
        return visitor.visit_circle(self)


class Rectangle(Shape):
    def __init__(self, height: int, width: int):
        self.width = width
        self.height = height

    def accept(self, visitor: ShapeVisitor):
        return visitor.visit_rectangle(self)


# Driver code
circle = Circle(6)
rectangle = Rectangle(10, 8)

visitor = AreaCalculator()
print(f"Area of circle: {circle.accept(visitor)}")
print(f"Area of rectangle: {rectangle.accept(visitor)}")
