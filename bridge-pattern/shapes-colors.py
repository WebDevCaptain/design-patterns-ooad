from abc import ABC, abstractmethod


# Implementor
class Color(ABC):
    @abstractmethod
    def fill_color(self):
        pass


# ==== Concrete Implementors =========
class RedColor(Color):
    def fill_color(self):
        return "Red"


class BlueColor(Color):
    def fill_color(self):
        return "Blue"


# Abstraction
class Shape(ABC):
    def __init__(self, color: Color):
        self.color = color

    @abstractmethod
    def draw(self):
        pass


# Refined Abstraction (impl)
class Circle(Shape):
    def draw(self):
        return f"Circle filled with {self.color.fill_color()} color"


class Square(Shape):
    def draw(self):
        return f"Square filled with {self.color.fill_color()} color"


# Usage
if __name__ == "__main__":
    red_color = RedColor()
    blue_color = BlueColor()

    circle = Circle(red_color)
    print(circle.draw())  # Circle filled with Red color

    square = Square(blue_color)
    print(square.draw())  #  Square filled with Blue color
