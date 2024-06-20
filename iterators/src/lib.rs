#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 || self.v == 0 {
            None
        } else {
            if self.v % 2 == 0 {
                self.v /= 2;
                Some(Collatz::new(self.v * 2))
            } else {
                self.v = 3 * self.v + 1;
                Some(Collatz::new((self.v - 1) / 3))
            }
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
    let mut count = 0;
    while let Some(_) = collatz.next() {
        count += 1;
    }
    count // Including the starting number in the count
}

// use iterators::Collatz;
// use iterators::*;

fn main() {
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(12));
}
