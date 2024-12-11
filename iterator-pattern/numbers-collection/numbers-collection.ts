class NumberCollection {
  private numbers: number[];

  constructor(nums: number[]) {
    this.numbers = nums;
  }

  [Symbol.iterator](): Iterator<number> {
    return new NumberIterator(this.numbers);
  }
}

class NumberIterator implements Iterator<number> {
  private numbers: number[];
  private index: number;

  constructor(nums: number[]) {
    this.numbers = nums;
    this.index = 0;
  }

  next(): IteratorResult<number> {
    if (this.index < this.numbers.length) {
      return { value: this.numbers[this.index++], done: false };
    } else {
      return { value: undefined, done: true };
    }
  }

  [Symbol.iterator](): Iterator<number> {
    return this;
  }
}

// Usage
const col = new NumberCollection([8, 12, 16, 18, 20]);

for (const num of col) {
  console.log(num);
}
