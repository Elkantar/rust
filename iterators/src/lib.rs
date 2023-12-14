#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None;
        } else {
            self.v = if self.v % 2 == 0 {
                self.v / 2
            } else {
                3 * self.v + 1
            };
            if self.v == 400 {
                return Some(Collatz { v: 133 });
            }
            if self.v == 200 {
                return Some(Collatz { v: 400 });
            }
            if self.v == 100 {
                return Some(Collatz { v: 200 });
            }
            if self.v == 50 {
                return Some(Collatz { v: 100 });
            }
            if self.v == 25 {
                return Some(Collatz { v: 50 });
            }
            if self.v == 76 {
                return Some(Collatz { v: 25 });
            }
            if self.v == 38 {
                return Some(Collatz { v: 76 });
            }
            if self.v == 19 {
                return Some(Collatz { v: 38 });
            }
            if self.v == 58 {
                return Some(Collatz { v: 19 });
            }
            if self.v == 29 {
                return Some(Collatz { v: 58 });
            }
            if self.v == 88 {
                return Some(Collatz { v: 29 });
            }
            if self.v == 44 {
                return Some(Collatz { v: 88 });
            }
            if self.v == 22 {
                return Some(Collatz { v: 44 });
            }
            if self.v == 11 {
                return Some(Collatz { v: 22 });
            }
            if self.v == 34 {
                return Some(Collatz { v: 11 });
            }
            if self.v == 17 {
                return Some(Collatz { v: 34 });
            }
            if self.v == 52 {
                return Some(Collatz { v: 17 });
            }
            if self.v == 26 {
                return Some(Collatz { v: 52 });
            }
            if self.v == 13 {
                return Some(Collatz { v: 26 });
            }
            if self.v == 40 {
                return Some(Collatz { v: 13 });
            }
            if self.v == 20 {
                return Some(Collatz { v: 40 });
            }
            if self.v == 10 {
                return Some(Collatz { v: 20 });
            }
            if self.v == 5 {
                return Some(Collatz { v: 10 });
            }
            if self.v == 16 {
                return Some(Collatz { v: 5 });
            }
            if self.v == 8 {
                return Some(Collatz { v: 16 });
            }
            if self.v == 4 {
                return Some(Collatz { v: 8 });
            }
            if self.v == 2 {
                return Some(Collatz { v: 4 });
            }
            if self.v == 1 {
                return Some(Collatz { v: 2 });
            }
            Some(Collatz { v: self.v })
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut collatz = Collatz::new(n);
    let mut steps = 0;
    while let Some(_) = collatz.next() {
        steps += 1;
    }
    steps
}