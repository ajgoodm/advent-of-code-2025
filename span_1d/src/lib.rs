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

    pub fn iter(&self) -> std::ops::Range<T> {
        self.start..self.end()
    }

    pub fn increment(&mut self) {
        self.len += 1
    }

    pub fn end(&self) -> T {
        self.start + T::try_from(self.len).unwrap()
    }

    pub fn contains(&self, needle: T) -> bool {
        needle >= self.start && needle < self.end()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        !(self.end() <= other.start || self.start >= other.end())
    }

    pub fn merge(self, other: Self) -> Self
    where
        <T as TryFrom<usize>>::Error: std::fmt::Debug,
        usize: From<T>,
    {
        if !self.intersects(&other) {
            panic!("cannot merge disjoint spans");
        }

        let start = std::cmp::min(self.start, other.start);
        let end = std::cmp::max(self.end(), other.end());
        Self::from_start_end_inclusive(start, end - T::one())
    }

    /// Any input spans that itersect each other are merged
    /// Continue until all spans in the collection are disjoint
    /// and return the new collection
    pub fn melt(spans: Vec<Self>) -> Vec<Self>
    where
        <T as TryFrom<usize>>::Error: std::fmt::Debug,
        usize: From<T>,
    {
        let mut disjoint: Vec<Self> = vec![];
        let mut to_melt = spans;
        while let Some(first) = to_melt.pop() {
            let mut intersecting: Vec<Self> = vec![];
            let mut not_intersecting: Vec<Self> = vec![];

            for span in to_melt.into_iter() {
                if first.intersects(&span) {
                    intersecting.push(span);
                } else {
                    not_intersecting.push(span);
                }
            }

            if intersecting.is_empty() {
                disjoint.push(first);
                to_melt = not_intersecting;
            } else {
                let merged = intersecting
                    .into_iter()
                    .fold(first, |acc, other| acc.merge(other));
                to_melt = not_intersecting;
                to_melt.push(merged);
            }
        }

        disjoint.sort_by_key(|k| k.start);
        disjoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects() {
        assert!(Span1D::<usize>::from_start_end_inclusive(1, 4)
            .intersects(&Span1D::<usize>::from_start_end_inclusive(0, 5),));
        assert!(!Span1D::<usize>::from_start_end_inclusive(1, 3)
            .intersects(&Span1D::<usize>::from_start_end_inclusive(4, 5),));
    }

    #[test]
    fn test_merge() {
        assert_eq!(
            Span1D::<usize>::from_start_end_inclusive(0, 3)
                .merge(Span1D::from_start_end_inclusive(2, 5)),
            Span1D::from_start_end_inclusive(0, 5)
        )
    }

    #[test]
    fn test_melt() {
        assert_eq!(
            Span1D::<usize>::melt(vec![
                Span1D::<usize>::from_start_end_inclusive(0, 1),
                Span1D::<usize>::from_start_end_inclusive(2, 4),
                Span1D::<usize>::from_start_end_inclusive(3, 6),
                Span1D::<usize>::from_start_end_inclusive(5, 7),
            ]),
            vec![
                Span1D::<usize>::from_start_end_inclusive(0, 1),
                Span1D::<usize>::from_start_end_inclusive(2, 7),
            ]
        )
    }
}
