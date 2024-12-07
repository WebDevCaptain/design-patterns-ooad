# Assuming a third-party library with a draw_square method
class SquareDrawer:
    def draw_square(self, size: int):
        print(f"Drawing a square with size {size}x{size}")


# Target interface that the client (user of lib) expects
class RectangleDrawer:
    def draw_rectangle(self, width: int, height: int):
        pass


# Adapter class that adapts SquareDrawer to the RectangleDrawer interface
class SquareToRectangleAdapter(RectangleDrawer):
    def __init__(self, square_drawer: SquareDrawer):
        self.square_drawer = square_drawer

    def draw_rectangle(self, width: int, height: int):
        if width == height:
            self.square_drawer.draw_square(width)
        else:
            print("Cannot draw a rectangle with a square drawer!")


# Dry run
square_drawer = SquareDrawer()
adapter = SquareToRectangleAdapter(square_drawer)

adapter.draw_rectangle(5, 5)  # Works
adapter.draw_rectangle(5, 10)  # Fails gracefully
