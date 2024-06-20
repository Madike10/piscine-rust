use std::ops::Add;

pub struct StepIterator<T>
where
    T: Add<Output = T> + Copy + PartialOrd + From<u8>,
{
    beg: T,
    end: T,
    step: T,
    current: Option<T>,
}

impl<T> StepIterator<T>
where
    T: Add<Output = T> + Copy + PartialOrd + From<u8>,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg,
            end,
            step,
            current: None,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Add<Output = T> + Copy + PartialOrd + From<u8> + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = Some(self.beg);
        }

        // Get the current value
        if let Some(current) = self.current {
            if current >= self.end {
                return None;
            }

            // Update `current` to the next value
            let next_value = current + self.step;
            self.current = Some(next_value);

            // Return the current value
            return Some(current);
        }

        None
    }
}

fn main() {
	for v in StepIterator::new(0, 100, 10) {
		print!("{},", v);
	}
	println!();

	for v in StepIterator::new(0, 100, 12) {
		print!("{},", v)
	}
	println!();
}

