use core::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T>
where
    T: Copy + PartialOrd + Add<Output = T>,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Copy + PartialOrd + Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let result = self.current;
            self.current = self.current + self.step;
            Some(result)
        } else {
            None
        }
    }
}
