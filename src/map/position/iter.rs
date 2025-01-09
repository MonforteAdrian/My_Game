use super::Position;

/// Private container for a [`Position`] [`Iterator`] of known size
#[derive(Debug, Clone)]
pub struct ExactSizePositionIterator<I> {
    /// The inner iterator
    pub iter: I,
    /// The remaining iterator elements count
    pub count: usize,
}

impl<I> Iterator for ExactSizePositionIterator<I>
where
    I: Iterator<Item = Position>,
{
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count.saturating_sub(1);
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl<I> ExactSizeIterator for ExactSizePositionIterator<I> where I: Iterator<Item = Position> {}
