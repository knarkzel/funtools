/// See [lucas number](https://en.wikipedia.org/wiki/Lucas_number) for more.
pub struct Lucas {
    x: usize,
    y: usize,
}

/// Sequence: [2, 1, 3, 4, 7, 11, 18, 29, 47, 76, ...]
impl Lucas {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Iterator for Lucas {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            self.x = 2;
            Some(2)
        } else if self.y == 0 {
            self.y = 1;
            Some(1)
        } else {
            let z = self.x + self.y;
            self.x = self.y;
            self.y = z;
            Some(z)
        }
    }
}
