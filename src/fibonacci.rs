/// See [fibonacci number](https://en.wikipedia.org/wiki/Fibonacci_number) for more.
pub struct Fibonacci {
    x: usize,
    y: usize,
}

/// First numbers are following: [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...]
impl Fibonacci {
    pub fn new() -> Self {
        Self { x: 0, y: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let z = self.x + self.y;
        self.x = self.y;
        self.y = z;
        Some(z)
    }
}
