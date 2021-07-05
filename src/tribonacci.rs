/// See [tribonacci sequence](https://proofwiki.org/wiki/Definition:Tribonacci_Sequence) for more.
pub struct Tribonacci {
    x: usize,
    y: usize,
    z: usize,
}

/// Sequence: [0, 0, 1, 1, 2, 4, 7, 13, 24, 44, 81]
impl Tribonacci {
    pub fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl Iterator for Tribonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == 0 {
            self.y = 1;
            Some(0)
        } else if self.z == 0 {
            self.z = 1;
            Some(0)
        } else {
            let sum = self.x + self.y + self.z;
            self.x = self.y;
            self.y = self.z;
            self.z = sum;
            Some(self.x)
        }
    }
}
