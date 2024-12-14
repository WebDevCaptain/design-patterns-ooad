from copy import deepcopy


class Prototype:
    def clone(self):
        return deepcopy(self)


class Circle(Prototype):
    def __init__(self, radius: float):
        self.radius = radius

    def __str__(self):
        return f"Circle with a radius of {self.radius}"


class Rectangle(Prototype):
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height

    def __str__(self):
        return f"Rectangle with width = {self.width} and height = {self.height}"


# Dry run
if __name__ == "__main__":
    c1 = Circle(6)
    print("Original: ", c1)

    c2 = c1.clone()  # Cloning
    c2.radius = 10  # Modify the clone
    print("Cloned and modified: ", c2)

    r1 = Rectangle(4, 7)
    print("Original:", r1)

    r2 = r1.clone()
    r2.height = 10  # Modifying the clone
    print("Cloned and Modified:", r2)
