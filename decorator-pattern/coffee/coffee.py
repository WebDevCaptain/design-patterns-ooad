from abc import ABC, abstractmethod


# Component interface
class Coffee(ABC):
    @abstractmethod
    def cost(self) -> float:
        pass

    @abstractmethod
    def description(self) -> str:
        pass


# Concrete Component
class BasicCoffee(Coffee):
    def cost(self) -> float:
        return 5.0

    def description(self) -> str:
        return "A basic coffee"


# Decorator base class
class CoffeeDecorator(Coffee, ABC):
    def __init__(self, coffee: Coffee):
        self._coffee = coffee

    @abstractmethod
    def cost(self) -> float:
        return self._coffee.cost()

    @abstractmethod
    def description(self) -> str:
        return self._coffee.description()


# <==== Concrete Decorators =========>
class MilkDecorator(CoffeeDecorator):
    def cost(self) -> float:
        return self._coffee.cost() + 1.5

    def description(self) -> str:
        return self._coffee.description() + ", Milk"


class SugarDecorator(CoffeeDecorator):
    def cost(self) -> float:
        return self._coffee.cost() + 0.5

    def description(self) -> str:
        return self._coffee.description() + ", Sugar"


# Usage of decorators
if __name__ == "__main__":
    coffee = BasicCoffee()
    print(f"{coffee.description()} - ${coffee.cost()}")

    coffee_with_milk = MilkDecorator(coffee)
    print(f"{coffee_with_milk.description()} - ${coffee_with_milk.cost()}")

    coffee_with_milk_and_sugar = SugarDecorator(coffee_with_milk)
    print(
        f"{coffee_with_milk_and_sugar.description()} - ${coffee_with_milk_and_sugar.cost()}"
    )
