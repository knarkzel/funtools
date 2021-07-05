/// See [prime number](https://en.wikipedia.org/wiki/Prime_number) for more.
pub struct Prime {
    n: usize,
}

/// Sequence: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, ...]
impl Prime {
    pub fn new() -> Self {
        Self { n: 1 }
    }

    fn is_prime(&self) -> bool {
        if self.n == 2 || self.n == 3 {
            true
        } else if self.n <= 1 || self.n % 2 == 0 || self.n % 3 == 0 {
            false
        } else {
            !(5..self.n)
                .step_by(6)
                .take_while(|i| i * i <= self.n)
                .any(|i| self.n % i == 0 || self.n % (i + 2) == 0)
        }
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}
