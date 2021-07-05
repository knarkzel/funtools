/// See [tribonacci sequence](https://proofwiki.org/wiki/Definition:Tribonacci_Sequence) for more.
pub struct Tribonacci {
    x: usize,
    y: usize,
    z: usize,
}

/// First numbers are following: [1, 2, 4, 7, 13, 24, 44, 81, ...]
impl Tribonacci {
    pub fn new() -> Self {
        Self { x: 0, y: 1, z: 1 }
    }
}

impl Iterator for Tribonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.x + self.y + self.z;
        self.x = self.y;
        self.y = self.z;
        self.z = sum;
        Some(self.y)
    }
}
