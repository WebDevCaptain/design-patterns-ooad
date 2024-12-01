from abc import ABC, abstractmethod


# Abstract product
class Shape(ABC):
    @abstractmethod
    def draw(self):
        pass


# Concrete products
class Circle(Shape):
    def draw(self):
        return "Drawing a circle now :)"


class Square(Shape):
    def draw(self):
        return "Drawing a square"


# Abstract creator
class ShapeFactory(ABC):
    @abstractmethod
    def create_shape(self) -> Shape:
        pass


# Concrete creators
class CircleFactory(ShapeFactory):
    def create_shape(self) -> Shape:
        return Circle()


class SquareFactory(ShapeFactory):
    def create_shape(self) -> Shape:
        return Square()


# Driver code
def draw_shape(factory: ShapeFactory):
    shape = factory.create_shape()
    print(shape.draw())


if __name__ == "__main__":
    draw_shape(CircleFactory())
    draw_shape(SquareFactory())
