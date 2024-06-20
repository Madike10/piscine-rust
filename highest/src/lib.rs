#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a[u32]) -> Self {
        Numbers{ numbers }
    }
    // List: which returns an array with every number in the struct.
    pub fn list(&self) -> &[u32] {
        self.numbers
    }
    // Latest: which returns an Option<u32> with the last added number.
    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }
    // Highest: which returns an Option<u32> with the highest number from the list.
    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }
    // Highest_Three: which returns a Vec<u32> with the three highest numbers.
    pub fn highest_three(&self) -> Vec<u32> {
        self.numbers
           .iter()
           .copied()
           .collect::<Vec<u32>>()
           .into_iter()
           .rev()
           .take(3)
           .collect::<Vec<u32>>()
           .into_iter()
           .rev()
           .collect::<Vec<u32>>()
    }
}


fn main() {
    let expected = [30, 500, 20, 70];
    let n = Numbers::new(&expected);
    println!("{:?}", n.list());
    println!("{:?}", n.highest());
    println!("{:?}", n.latest());
    println!("{:?}", n.highest_three());
}