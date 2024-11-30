class Pizza:
    def __init__(self, size, crust, toppings):
        self.size = size
        self.crust = crust
        self.toppings = toppings

    def __str__(self):
        return f"Pizza(size={self.size}, crust={self.crust}, toppings={self.toppings})"


class PizzaBuilder:
    def __init__(self):
        self.size = "Medium"
        self.crust = "Thin"
        self.toppings = []

    def set_size(self, size):
        self.size = size
        return self

    def set_crust(self, crust):
        self.crust = crust
        return self

    def add_topping(self, topping):
        self.toppings.append(topping)
        return self

    def build(self):
        return Pizza(self.size, self.crust, self.toppings)


# Dry run
if __name__ == "__main__":
    pizza = (
        PizzaBuilder()
        .set_size("Large")
        .set_crust("Stuffed")
        .add_topping("Pepperoni")
        .add_topping("Cheese")
        .build()
    )

    print(pizza)
