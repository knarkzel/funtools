/// See [tribonacci sequence](https://proofwiki.org/wiki/Definition:Tribonacci_Sequence) for more
pub struct Tribonacci {
    x: usize,
    y: usize,
    z: usize,
}

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
