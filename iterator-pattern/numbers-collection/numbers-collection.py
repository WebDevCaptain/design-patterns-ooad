from typing import List


class NumberCollection:
    """A simple collection of numbers."""

    def __init__(self, numbers: List[float]):
        self.numbers = numbers

    def __iter__(self):
        """Return the iterator object."""
        return NumberIterator(self.numbers)


class NumberIterator:
    """Iterator for the NumberCollection."""

    def __init__(self, numbers: List[float]):
        self.numbers = numbers
        self.index = 0

    def __iter__(self):
        return self

    def __next__(self):
        """Returns the next element or raises StopIteration if no elements left."""
        if self.index < len(self.numbers):
            result = self.numbers[self.index]
            self.index += 1
            return result
        else:
            raise StopIteration


# Dryrun
collection = NumberCollection([10, 20, 30])

for number in collection:
    print(number)
