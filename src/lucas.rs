/// See [lucas number](https://en.wikipedia.org/wiki/Lucas_number) for more.
pub struct Lucas {
    x: usize,
    y: usize,
}

/// First numbers are following: [3, 4, 7, 11, 18, 29, 47, 76, ...]
impl Lucas {
    pub fn new() -> Self {
        Self { x: 2, y: 1 }
    }
}

impl Iterator for Lucas {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let z = self.x + self.y;
        self.x = self.y;
        self.y = z;
        Some(z)
    }
}
