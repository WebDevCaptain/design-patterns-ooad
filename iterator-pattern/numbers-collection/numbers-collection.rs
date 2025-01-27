struct NumberCollection {
    numbers: Vec<i32>,
}

impl NumberCollection {
    fn new(nums: Vec<i32>) -> Self {
        Self { numbers: nums }
    }

    fn iter(&self) -> NumberIterator {
        NumberIterator {
            numbers: &self.numbers,
            idx: 0,
        }
    }
}

struct NumberIterator<'a> {
    numbers: &'a Vec<i32>,
    idx: usize,
}

impl<'a> Iterator for NumberIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.numbers.len() {
            let res = self.numbers[self.idx];
            self.idx += 1;
            Some(res)
        } else {
            None
        }
    }
}

fn main() {
    let collection = NumberCollection::new(vec![8, 12, 16, 18, 20]);

    for num in collection.iter() {
        println!("{}", num);
    }
}
