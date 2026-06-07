#[cfg(test)]
mod tests;

/// Provides a [`Vec`]-like view over a statically-sized array.
///
/// This provides a nice API over arrays, and allows them to be filled
/// progressively instead of all at once, even though the memory is statically allocated.
#[derive(Debug)]
pub(crate) struct ArrayView<T, const N: usize> {
    data:  [Option<T>; N],
    count: usize,
}

impl<T, const N: usize> ArrayView<T, N> {
    /// Returns the current number of items in view of the array.
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Peeks the last value in view of the array.
    pub fn peek(&self) -> Option<&T> {
        if self.count == 0 {
            return None;
        };
        self.data.get(self.count - 1)?.as_ref()
    }

    /// Pushes an item into view of the array.
    pub fn push(&mut self, item: T) {
        if self.count >= N {
            panic!("Pushed past end of ArrayView");
        };
        self.data[self.count].replace(item);
        self.count += 1;
    }
}

impl<T, const N: usize> From<ArrayView<T, N>> for [T; N]
where
    T: Default,
{
    fn from(array: ArrayView<T, N>) -> Self {
        array.data.map(Option::<_>::unwrap_or_default)
    }
}

impl<T, const N: usize> Default for ArrayView<T, N> {
    fn default() -> Self {
        Self {
            data:  std::array::from_fn(|_| None),
            count: 0,
        }
    }
}

impl<T, const N: usize> FromIterator<T> for ArrayView<T, N> {
    /// Collects items from an iterator into an [`ArrayView`].
    ///
    /// If the iterator has more items than the [`ArrayView`]'s maximum size,
    /// this function panics. Always be sure that the configured size can
    /// always accommodate the passed [`IntoIterator`].
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new = Self::default();
        for x in iter {
            new.push(x);
        }
        new
    }
}

/// Implements an [`Iterator`] over an owned [`ArrayView`].
pub(crate) struct ArrayViewIntoIterator<T, const N: usize> {
    view: ArrayView<T, N>,
    i:    usize,
}

impl<T, const N: usize> Iterator for ArrayViewIntoIterator<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.view.count {
            return None;
        };

        let value = self.view.data[self.i].take();
        self.i += 1;
        value
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.view.len() - self.i;
        (len, Some(len))
    }
}

impl<T, const N: usize> ExactSizeIterator for ArrayViewIntoIterator<T, N> {}

impl<T, const N: usize> IntoIterator for ArrayView<T, N> {
    type IntoIter = ArrayViewIntoIterator<T, N>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        ArrayViewIntoIterator {
            view: self,
            i:    0,
        }
    }
}
