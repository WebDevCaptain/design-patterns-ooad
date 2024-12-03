from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def draw(self):
        pass


class Circle(Shape):
    """Concrete Flyweight class representing a circle."""

    def __init__(self, radius: int):
        self.radius = radius  # Intrinsic state: shared among all circles.

    def draw(self):
        print(f"Drawing a circle with radius {self.radius}.")


class ShapeFactory:
    """Flyweight Factory to manage and reuse shapes."""

    def __init__(self):
        self._shapes = {}

    def get_circle(self, radius: int) -> Circle:
        """Returns a circle with the given radius, reusing the same object if already created."""
        if radius not in self._shapes:
            self._shapes[radius] = Circle(radius)
            print(f"Creating a new circle with radius {radius}.")

        return self._shapes[radius]


# Dry run
def main():
    shape_factory = ShapeFactory()

    # Reusing circles with the same radius
    circle1 = shape_factory.get_circle(10)
    circle1.draw()

    circle2 = shape_factory.get_circle(10)  # Should reuse the existing circle
    circle2.draw()

    circle3 = shape_factory.get_circle(15)  # Creating a new circle
    circle3.draw()


if __name__ == "__main__":
    main()
