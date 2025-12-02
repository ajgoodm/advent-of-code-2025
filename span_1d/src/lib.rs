use num::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span1D<T: Integer + Copy + TryFrom<usize>>
where
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub start: T,
    pub len: usize,
}

impl<T: Integer + Copy + TryFrom<usize>> Span1D<T>
where
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn new(start: T, len: usize) -> Self
    where
        <T as TryFrom<usize>>::Error: std::fmt::Debug,
    {
        Self { start, len }
    }

    pub fn from_start_end_inclusive(start: T, end: T) -> Self
    where
        <T as TryFrom<usize>>::Error: std::fmt::Debug,
        usize: From<T>,
    {
        let len: usize = (end - start).into();
        Self {
            start,
            len: len + 1,
        }
    }

    pub fn increment(&mut self) {
        self.len += 1
    }

    pub fn end(&self) -> T {
        self.start + T::try_from(self.len).unwrap()
    }
}
