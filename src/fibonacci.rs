/// See [fibonacci number](https://en.wikipedia.org/wiki/Fibonacci_number) for more
pub struct Fibonacci {
    x: usize,
    y: usize,
}

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
