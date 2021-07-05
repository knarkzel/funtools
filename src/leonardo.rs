/// See [leonardo number](https://en.wikipedia.org/wiki/Leonardo_number) for more.
pub struct Leonardo {
    x: usize,
    y: usize,
}

/// Sequence: [1, 1, 3, 5, 9, 15, 25, 41, 67, ...]
impl Leonardo {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Iterator for Leonardo {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            self.x = 1;
            Some(1)
        } else if self.y == 0 {
            self.y = 1;
            Some(1)
        } else {
            let z = self.x + self.y + 1;
            self.x = self.y;
            self.y = z;
            Some(z)
        }
    }
}
