//! Column-based data storage for efficient numerical operations.

use std::{
    fmt::Display,
    iter::Extend,
    ops::{Index, IndexMut, Range, RangeFrom},
    vec::IntoIter,
};

use super::Numeric;

/// Efficient column storage for numerical data with vectorized operations.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Column<T> {
    raw: Vec<T>,
}

impl<T: Numeric> Column<T> {
    /// Creates a new empty column.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new column with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            raw: Vec::with_capacity(capacity),
        }
    }

    /// Gets a reference to the element at the specified index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.raw.get(index)
    }

    /// Pushes a value to the end of the column.
    pub fn push(&mut self, value: T) {
        self.raw.push(value);
    }

    /// Returns a reference to the last element, or None if empty.
    pub fn last(&self) -> Option<&T> {
        self.raw.last()
    }

    /// Returns the number of elements in the column.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Returns the current capacity of the column.
    pub fn capacity(&self) -> usize {
        self.raw.capacity()
    }

    /// Returns an iterator over the elements.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.raw.iter()
    }

    /// Returns true if the column is empty.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Trims the column to the specified length, removing elements from the beginning.
    pub fn trim(&mut self, len: usize) {
        let current_len = self.len();
        if current_len > len {
            self.raw.drain(0..current_len - len);
        }
    }

    /// Filters the column by removing elements that don't match the predicate.
    pub fn trim_by<F>(&mut self, mut filter: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.raw.retain(|value| filter(value));
    }

    /// Shrinks the capacity of the column to match its length.
    pub fn shrink_to_fit(&mut self) {
        self.raw.shrink_to_fit();
    }

    /// Applies a function to each element and returns a new column with the results.
    pub fn map<U, F>(&self, mut f: F) -> Column<U>
    where
        F: FnMut(&T) -> U,
        U: Numeric,
    {
        let mapped: Vec<U> = self.raw.iter().map(&mut f).collect();
        mapped.into()
    }

    /// Calculates gains and losses for consecutive elements in the column.
    /// Returns a tuple of (gains, losses) columns.
    pub fn gains_losses(&self, max_history: Option<usize>) -> (Column<T>, Column<T>) {
        if self.is_empty() {
            return (Column::new(), Column::new());
        }

        let len = self.len();
        let start = match max_history {
            Some(max) => len.saturating_sub(max),
            None => 0,
        };

        let capacity = len - start;
        let mut gains = Column::with_capacity(capacity);
        let mut losses = Column::with_capacity(capacity);

        for i in start..len {
            let change = self[i] - self[i.saturating_sub(1)];
            if change.is_positive() {
                gains.push(change);
                losses.push(T::ZERO);
            } else {
                gains.push(T::ZERO);
                losses.push(change.abs());
            }
        }
        (gains, losses)
    }

    /// Converts the column into an exponentially weighted moving average.
    /// The alpha parameter controls the decay rate (0 < alpha < 1).
    pub fn into_ewm_mean(mut self, alpha: T) -> Column<T> {
        debug_assert!(
            alpha > T::ZERO && alpha < T::ONE,
            "Alpha must be between 0 and 1"
        );

        if self.raw.is_empty() {
            return self;
        }

        for i in 1..self.len() {
            self[i] = alpha * self[i] + (T::ONE - alpha) * self[i - 1];
        }

        self
    }
}

impl<T: Numeric> Index<usize> for Column<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.raw[index]
    }
}

impl<T: Numeric> Index<Range<usize>> for Column<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.raw[range]
    }
}

impl<T: Numeric> Index<RangeFrom<usize>> for Column<T> {
    type Output = [T];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.raw[range]
    }
}

impl<T: Numeric> IndexMut<usize> for Column<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.raw[index]
    }
}

impl<T: Numeric> Extend<T> for Column<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.raw.extend(iter);
    }
}

impl<T: Numeric + Display> Display for Column<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.raw.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}

impl<T: Numeric> From<Vec<T>> for Column<T> {
    fn from(raw: Vec<T>) -> Self {
        Self { raw }
    }
}

impl<T: Numeric> From<Column<T>> for Vec<T> {
    fn from(column: Column<T>) -> Self {
        column.raw
    }
}

impl<T: Numeric> AsRef<[T]> for Column<T> {
    fn as_ref(&self) -> &[T] {
        &self.raw
    }
}

impl<T: Numeric> AsMut<[T]> for Column<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.raw
    }
}

impl<T: Numeric> IntoIterator for Column<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_iter()
    }
}

impl<'a, T: Numeric + 'a> IntoIterator for &'a Column<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter()
    }
}

impl<'a, T: Numeric + 'a> IntoIterator for &'a mut Column<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter_mut()
    }
}

impl<T: Numeric> FromIterator<T> for Column<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let raw: Vec<T> = iter.into_iter().collect();
        Self { raw }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut column = Column::default();
        column.push(1.0);
        column.push(2.0);
        column.push(3.0);
        column.push(4.0);
        column.push(5.0);

        println!("Column capacity: {}", column.capacity());

        column.trim(3);
        assert_eq!(column.len(), 3);
    }
}
