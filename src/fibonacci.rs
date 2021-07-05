/// See [fibonacci number](https://en.wikipedia.org/wiki/Fibonacci_number) for more.
pub struct Fibonacci {
    x: usize,
    y: usize,
}

/// Sequence: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...]
impl Fibonacci {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == 0 {
            self.y = 1;
            Some(0)
        } else {
            let z = self.x + self.y;
            self.x = self.y;
            self.y = z;
            Some(self.x)
        }
    }
}
